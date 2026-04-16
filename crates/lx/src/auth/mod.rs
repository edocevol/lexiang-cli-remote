//! 认证模块 —— OAuth 登录、Token 存储与刷新
//!
//! 对外暴露：
//!   - `login()`           — OAuth 2.0 登录（CLI 一次性流程）
//!   - `login_start()`     — OAuth 两阶段：启动回调服务器，返回授权 URL
//!   - `login_wait()`      — OAuth 两阶段：等待回调完成，返回 token
//!   - `logout()`          — 登出（删除本地 token）
//!   - `get_access_token`  — 获取有效 token（自动刷新）

use crate::config::Config;
use crate::datadir;
use anyhow::Result;
use axum::extract::Query;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

// ═══════════════════════════════════════════════════════════
//  常量
// ═══════════════════════════════════════════════════════════

const WELL_KNOWN_URL: &str = "https://mcp.lexiang-app.com/.well-known/oauth-authorization-server";
const CALLBACK_START_PORT: u16 = 18765;
const CALLBACK_MAX_PORT_ATTEMPTS: u16 = 50;
const TOKEN_FILE: &str = "token.json";

// ═══════════════════════════════════════════════════════════
//  公开数据结构
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    /// 动态注册获得的 `client_id`，refresh 时需要
    pub client_id: Option<String>,
}

// ═══════════════════════════════════════════════════════════
//  内部数据结构（OAuth 协议）
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct OAuthServerConfig {
    authorization_endpoint: String,
    token_endpoint: String,
    registration_endpoint: String,
    scopes_supported: Vec<String>,
}

/// 简化版 —— 仅在 refresh 时使用，只需 `token_endpoint`
#[derive(Debug, Deserialize)]
struct OAuthServerConfigMinimal {
    token_endpoint: String,
}

#[derive(Debug, Deserialize)]
struct ClientRegistration {
    client_id: String,
    client_secret: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct CallbackQuery {
    code: String,
    state: String,
}

/// 进行中的 OAuth 状态（两阶段登录）
///
/// `auth/startOAuth` 创建，回调服务器写入完成信号，
/// `auth/completeOAuth` 等待并读取结果。
pub struct PendingOAuth {
    /// 期望的 CSRF state（用于验证回调）
    pub expected_state: String,
    /// 回调服务器端口
    pub callback_port: u16,
    /// OAuth 配置（token exchange 需要）
    oauth_config: OAuthServerConfig,
    /// 动态注册的客户端信息
    client_registration: ClientRegistration,
    /// PKCE verifier（exchange code 需要）
    pkce_verifier: Arc<Mutex<Option<PkceCodeVerifier>>>,
    /// `redirect_uri（exchange` code 需要）
    redirect_uri: String,
    /// 回调完成通知
    completed: Notify,
    /// 回调结果
    result: Arc<Mutex<Option<Result<TokenData>>>>,
    /// 回调服务器任务句柄（OAuth 完成后关闭）
    server_handle: Option<tokio::task::AbortHandle>,
}

impl PendingOAuth {
    /// 处理 OAuth 回调：验证 state，用 code 换 token，然后关闭回调服务器
    pub async fn handle_callback(&self, code: String, state: String) {
        if state != self.expected_state {
            let mut result = self.result.lock().await;
            *result = Some(Err(anyhow::anyhow!("Invalid state parameter")));
            self.abort_server();
            self.completed.notify_one();
            return;
        }

        let pkce_verifier = {
            let mut guard = self.pkce_verifier.lock().await;
            guard.take()
        };
        let Some(verifier) = pkce_verifier else {
            let mut result = self.result.lock().await;
            *result = Some(Err(anyhow::anyhow!("PKCE verifier already used")));
            self.abort_server();
            self.completed.notify_one();
            return;
        };

        let http = Client::new();
        let token_result = exchange_code(
            &http,
            &self.oauth_config,
            &self.client_registration,
            code,
            &self.redirect_uri,
            verifier,
        )
        .await;

        if let Ok(ref token) = token_result {
            tracing::info!(
                token_len = token.access_token.len(),
                "OAuth callback: saving token"
            );
            if let Err(e) = save_token(token) {
                tracing::warn!(error = %e, "OAuth callback: save_token failed");
            } else {
                tracing::info!("OAuth callback: token saved successfully");
            }
        }

        // 关闭回调服务器（无论成功失败）
        self.abort_server();

        let mut result = self.result.lock().await;
        *result = Some(token_result);
        self.completed.notify_one();
    }

    /// 关闭回调 HTTP 服务器
    pub fn abort_server(&self) {
        if let Some(ref handle) = self.server_handle {
            handle.abort();
            tracing::debug!("OAuth callback server aborted");
        }
    }

    /// 等待回调完成并返回结果
    pub async fn wait(&self) -> Result<TokenData> {
        self.completed.notified().await;
        let mut guard = self.result.lock().await;
        guard
            .take()
            .ok_or_else(|| anyhow::anyhow!("OAuth callback not completed"))?
    }
}

// ═══════════════════════════════════════════════════════════
//  公开 API
// ═══════════════════════════════════════════════════════════

/// 统一的 `access_token` 获取入口。
///
/// 1. config 中显式配置的 token（环境变量 / 配置文件 / --token 注入）→ 直接使用
/// 2. 本地 token 未过期 → 直接使用
/// 3. 本地 token 已过期 + 有 `refresh_token` → 自动刷新
/// 4. 刷新失败或无 token → 返回错误提示 `lx login`
pub async fn get_access_token(config: &Config) -> Result<String> {
    // 优先使用 config 中显式配置的 token（--token / LX_ACCESS_TOKEN / 配置文件）
    if let Some(token) = config.mcp.access_token.clone() {
        return Ok(token);
    }

    // 3. 本地 token 文件（自动刷新）
    match get_valid_token().await? {
        Some(td) => Ok(td.access_token),
        None => {
            if load_token()?.is_some() {
                anyhow::bail!("Access token 已过期且刷新失败。请重新运行 'lx login' 登录。")
            } else {
                anyhow::bail!("未找到有效的 access token。请先运行 'lx login' 登录。")
            }
        }
    }
}

/// OAuth 2.0 登录（PKCE + 动态客户端注册）
///
/// CLI 一次性流程：启动回调服务器 → 打印授权链接 → 阻塞等待回调 → 保存 token
pub async fn login() -> Result<TokenData> {
    println!("正在获取 OAuth 配置...");
    let (auth_url, pending) = start_oauth_server().await?;
    println!("\n请在浏览器中完成登录。授权链接:\n{auth_url}\n");
    let token = pending.wait().await?;
    Ok(token)
}

/// 登出 —— 删除本地 token
pub fn logout() -> Result<()> {
    delete_token()
}

/// 两阶段 OAuth 登录 — 第一阶段：启动回调服务器，返回授权 URL
///
/// 适用于 VS Code 等编辑器扩展场景：编辑器用 `vscode.env.openExternal()` 打开浏览器，
/// 然后调用 `pending.wait()` 等待完成。Remote SSH 下也能正常工作。
pub async fn login_start() -> Result<(String, Arc<PendingOAuth>)> {
    start_oauth_server().await
}

/// 底层 OAuth 启动逻辑：获取配置 → 启动回调服务器 → 注册客户端 → 生成授权 URL → 构造 `PendingOAuth`
///
/// `login()` 和 `login_start()` 都调用此函数，统一存储和回调处理逻辑。
async fn start_oauth_server() -> Result<(String, Arc<PendingOAuth>)> {
    let http = Client::new();

    // 1. 获取 OAuth 服务端配置
    let oauth_cfg = fetch_oauth_config(&http).await?;

    // 2. 启动本地回调服务器
    let (tx, rx) = tokio::sync::oneshot::channel();
    let tx = Arc::new(Mutex::new(Some(tx)));

    let app = Router::new().route(
        "/",
        get(move |query: Query<CallbackQuery>| {
            let tx = tx.clone();
            async move {
                if let Some(tx) = tx.lock().await.take() {
                    let _ = tx.send(query.0);
                }
                Html("<html><body><h1>登录成功，请关闭此页面</h1></body></html>")
            }
        }),
    );

    let mut actual_port = CALLBACK_START_PORT;
    let listener = loop {
        let addr = SocketAddr::from(([127, 0, 0, 1], actual_port));
        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                tracing::debug!("OAuth 回调端口: {}", actual_port);
                break listener;
            }
            Err(_) => {
                actual_port += 1;
                if actual_port >= CALLBACK_START_PORT + CALLBACK_MAX_PORT_ATTEMPTS {
                    anyhow::bail!(
                        "无法在 {}-{} 范围内分配 OAuth 回调端口",
                        CALLBACK_START_PORT,
                        CALLBACK_START_PORT + CALLBACK_MAX_PORT_ATTEMPTS - 1
                    );
                }
            }
        }
    };

    let redirect_uri = format!("http://127.0.0.1:{}", actual_port);
    let server_handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    })
    .abort_handle();

    // 3. 动态注册客户端
    let reg = register_client(&http, &oauth_cfg, &redirect_uri).await?;

    // 4. PKCE + CSRF
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let state = CsrfToken::new_random();
    let state_secret = state.secret().to_string();

    // 5. 授权 URL
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        oauth_cfg.authorization_endpoint,
        reg.client_id,
        urlencoding::encode(&redirect_uri),
        oauth_cfg.scopes_supported.join("+"),
        state_secret,
        pkce_challenge.as_str(),
    );

    // 6. 构造 PendingOAuth
    let pending = Arc::new(PendingOAuth {
        expected_state: state_secret,
        callback_port: actual_port,
        oauth_config: oauth_cfg,
        client_registration: reg,
        pkce_verifier: Arc::new(Mutex::new(Some(pkce_verifier))),
        redirect_uri,
        completed: Notify::new(),
        result: Arc::new(Mutex::new(None)),
        server_handle: Some(server_handle),
    });

    // 7. 后台等待回调 → handle_callback 里会 save_token + abort_server
    let pending_clone = pending.clone();
    tokio::spawn(async move {
        match rx.await {
            Ok(callback) => {
                pending_clone
                    .handle_callback(callback.code, callback.state)
                    .await;
            }
            Err(_) => {
                let mut result = pending_clone.result.lock().await;
                *result = Some(Err(anyhow::anyhow!("OAuth callback channel closed")));
                pending_clone.completed.notify_one();
            }
        }
    });

    Ok((auth_url, pending))
}

/// 直接保存 access token（跳过 OAuth 流程）
///
/// 适用于从其他渠道获取 token 后直接配置的场景。
pub fn save_token_direct(access_token: &str) -> Result<()> {
    let token = TokenData {
        access_token: access_token.to_string(),
        refresh_token: None,
        expires_at: None,
        client_id: None,
    };
    save_token(&token)
}

// ═══════════════════════════════════════════════════════════
//  Token 持久化（~/.lexiang/auth/token.json）
// ═══════════════════════════════════════════════════════════

fn token_path() -> Result<PathBuf> {
    Ok(datadir::auth_dir().join(TOKEN_FILE))
}

pub fn save_token(token: &TokenData) -> Result<()> {
    let path = token_path()?;
    let json = serde_json::to_string_pretty(token)?;
    fs::write(&path, json)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&path, perms)?;
    }

    Ok(())
}

pub fn load_token() -> Result<Option<TokenData>> {
    let path = token_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let json = fs::read_to_string(&path)?;
    let token: TokenData = serde_json::from_str(&json)?;
    Ok(Some(token))
}

fn delete_token() -> Result<()> {
    let path = token_path()?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

// ═══════════════════════════════════════════════════════════
//  Token 有效性检查 & 自动刷新
// ═══════════════════════════════════════════════════════════

pub fn is_expired_public(token: &TokenData) -> bool {
    match token.expires_at {
        Some(expires_at) => chrono::Utc::now().timestamp() >= expires_at - 300, // 提前 5 分钟
        None => false,
    }
}

fn is_expired(token: &TokenData) -> bool {
    is_expired_public(token)
}

/// 获取有效 token：未过期直接返回，过期则尝试 refresh，失败返回 None
async fn get_valid_token() -> Result<Option<TokenData>> {
    match load_token()? {
        Some(token) if !is_expired(&token) => Ok(Some(token)),
        Some(token) if token.refresh_token.is_some() => {
            tracing::info!("Access token 已过期，正在使用 refresh_token 刷新...");
            match refresh_token(&token).await {
                Ok(new_token) => {
                    tracing::info!("Token 刷新成功");
                    save_token(&new_token)?;
                    Ok(Some(new_token))
                }
                Err(e) => {
                    tracing::warn!("Token 刷新失败: {e}");
                    Ok(None)
                }
            }
        }
        Some(_) => {
            tracing::warn!("Access token 已过期且无 refresh_token");
            Ok(None)
        }
        None => Ok(None),
    }
}

/// 使用 `refresh_token` 换取新 token
async fn refresh_token(token: &TokenData) -> Result<TokenData> {
    let refresh = token
        .refresh_token
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No refresh token available"))?;

    let http = Client::new();

    // 复用 .well-known 获取 token_endpoint
    let cfg: OAuthServerConfigMinimal = http.get(WELL_KNOWN_URL).send().await?.json().await?;

    let mut form = vec![
        ("grant_type".to_string(), "refresh_token".to_string()),
        ("refresh_token".to_string(), refresh.clone()),
    ];
    // 公共客户端 refresh 时需要携带 client_id
    if let Some(cid) = &token.client_id {
        form.push(("client_id".to_string(), cid.clone()));
    }

    // lgtm[go/cleartext-transmission] endpoint 来自 HTTPS .well-known 响应
    let resp = http.post(&cfg.token_endpoint).form(&form).send().await?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        anyhow::bail!("Token 刷新请求失败 (HTTP {}): {}", status, body);
    }

    let tr: OAuthTokenResponse = resp.json().await?;

    Ok(TokenData {
        access_token: tr.access_token,
        refresh_token: tr.refresh_token.or_else(|| token.refresh_token.clone()),
        expires_at: tr
            .expires_in
            .map(|ei| chrono::Utc::now().timestamp() + ei as i64),
        client_id: token.client_id.clone(),
    })
}

// ═══════════════════════════════════════════════════════════
//  OAuth 协议辅助函数（login 内部使用）
// ═══════════════════════════════════════════════════════════

async fn fetch_oauth_config(http: &Client) -> Result<OAuthServerConfig> {
    let resp = http.get(WELL_KNOWN_URL).send().await?;
    if !resp.status().is_success() {
        let err = resp.text().await?;
        anyhow::bail!("Failed to fetch OAuth config: {}", err);
    }
    Ok(resp.json().await?)
}

async fn register_client(
    http: &Client,
    oauth_cfg: &OAuthServerConfig,
    redirect_uri: &str,
) -> Result<ClientRegistration> {
    let body = serde_json::json!({
        "redirect_uris": [redirect_uri],
        "client_name": "Lexiang CLI",
        "grant_types": ["authorization_code", "refresh_token"],
        "response_types": ["code"],
        "scope": oauth_cfg.scopes_supported.join(" "),
    });

    // lgtm[go/cleartext-transmission] endpoint 来自 HTTPS .well-known 响应
    let resp = http
        .post(&oauth_cfg.registration_endpoint)
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let err = resp.text().await?;
        anyhow::bail!("Failed to register client: {}", err);
    }
    Ok(resp.json().await?)
}

async fn exchange_code(
    http: &Client,
    oauth_cfg: &OAuthServerConfig,
    reg: &ClientRegistration,
    code: String,
    redirect_uri: &str,
    pkce_verifier: PkceCodeVerifier,
) -> Result<TokenData> {
    let mut form = vec![
        ("grant_type", "authorization_code".to_string()),
        ("code", code),
        ("redirect_uri", redirect_uri.to_string()),
        ("client_id", reg.client_id.clone()),
        ("code_verifier", pkce_verifier.secret().clone()),
    ];
    if let Some(secret) = &reg.client_secret {
        form.push(("client_secret", secret.clone()));
    }

    // lgtm[go/cleartext-transmission] endpoint 来自 HTTPS .well-known 响应
    let resp = http
        .post(&oauth_cfg.token_endpoint)
        .form(&form)
        .send()
        .await?;

    if !resp.status().is_success() {
        let err = resp.text().await?;
        anyhow::bail!("Token exchange failed: {}", err);
    }

    let tr: OAuthTokenResponse = resp.json().await?;

    Ok(TokenData {
        access_token: tr.access_token,
        refresh_token: tr.refresh_token,
        expires_at: tr
            .expires_in
            .map(|ei| chrono::Utc::now().timestamp() + ei as i64),
        client_id: Some(reg.client_id.clone()),
    })
}

// open_browser 已移除 — 不允许自动打开浏览器，用户需手动复制链接
