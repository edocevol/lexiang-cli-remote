//! `lx serve` — stdio JSON-RPC 2.0 server
//!
//! Provides a programmatic API for editors (VS Code, Neovim, etc.) to access
//! Lexiang knowledge base data via JSON-RPC 2.0 over stdin/stdout.
//!
//! # Architecture
//!
//! Method handlers use `inventory::submit!` for compile-time auto-registration.
//! No central match statement — each handler is an independent unit.
//!
//! Unknown methods automatically fall through to MCP tool calls (dynamic proxy),
//! so new MCP tools are available without code changes.
//!
//! # Adding a new handler
//!
//! ```ignore
//! // In any file under src/serve/methods/
//! use crate::serve::{JsonRpcResult, ServeContext, rpc_method};
//!
//! async fn handle_my_method(ctx: &ServeContext, params: Value) -> JsonRpcResult {
//!     let client = ctx.mcp_client().await?;
//!     Ok(serde_json::json!({ "result": "..." }))
//! }
//!
//! inventory::submit! {
//!     rpc_method!("my/domain/method", handle_my_method)
//! }
//! ```
//!
//! # Protocol
//! - Reads JSON-RPC requests from stdin (one per line, `\n`-delimited)
//! - Writes JSON-RPC responses/notifications to stdout
//! - Logs to stderr only (never stdout — would corrupt JSON-RPC stream)

mod handler;
mod methods;
mod protocol;
mod transport;

pub use protocol::{
    JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, JsonRpcResult,
};
pub use transport::ServeTransport;

use crate::config::Config;
use anyhow::Result;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════
//  Shared State & Context
// ═══════════════════════════════════════════════════════════

/// Shared server state (mutable, guarded by `RwLock`)
pub struct ServeState {
    pub config: Config,
    pub access_token: Option<String>,
    /// 进行中的 OAuth 流程（auth/startOAuth 创建，auth/completeOAuth 消费）
    pub pending_oauth: Option<Arc<crate::auth::PendingOAuth>>,
    /// 缓存的 MCP 客户端（避免每次请求都重建）
    pub cached_mcp_client: Option<crate::mcp::McpClient>,
}

impl ServeState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            access_token: None,
            pending_oauth: None,
            cached_mcp_client: None,
        }
    }
}

/// Request-scoped context passed to every handler
///
/// Provides convenience methods for common operations (MCP client, auth, etc.)
pub struct ServeContext {
    pub(crate) state: Arc<RwLock<ServeState>>,
}

impl ServeContext {
    pub fn new(state: Arc<RwLock<ServeState>>) -> Self {
        Self { state }
    }

    /// Get a ready-to-use MCP client (resolves auth automatically)
    ///
    /// 缓存 `McpClient` 实例：token 不变时复用，token 变化时重建。
    pub async fn mcp_client(&self) -> Result<crate::mcp::McpClient, JsonRpcError> {
        // 1. 获取当前 token（优先内存，否则从文件读取并可能触发 refresh）
        let token = {
            let state = self.state.read().await;
            match &state.access_token {
                Some(t) => t.clone(),
                None => {
                    drop(state); // 释放读锁
                                 // get_access_token 可能触发 token refresh
                    let config = {
                        let state = self.state.read().await;
                        state.config.clone()
                    };
                    let token = crate::auth::get_access_token(&config)
                        .await
                        .map_err(|e| JsonRpcError::new(error_codes::AUTH_EXPIRED, e.to_string()))?;
                    // 同步回内存
                    {
                        let mut state = self.state.write().await;
                        state.access_token = Some(token.clone());
                    }
                    token
                }
            }
        };

        // 2. 检查缓存的 client 是否可用（token 相同）
        {
            let state = self.state.read().await;
            if let Some(ref cached) = state.cached_mcp_client {
                if cached.access_token() == Some(&token) {
                    return Ok(cached.clone());
                }
            }
        }

        // 3. 创建新的 client 并缓存
        let url = {
            let state = self.state.read().await;
            state.config.mcp.url.clone()
        };
        let client = crate::mcp::McpClient::new(&url, Some(token.clone()))
            .map_err(|e| JsonRpcError::new(error_codes::INTERNAL_ERROR, e.to_string()))?;

        {
            let mut state = self.state.write().await;
            state.cached_mcp_client = Some(client.clone());
        }

        tracing::debug!(
            token_len = token.len(),
            "mcp_client: created new client (cached)"
        );
        Ok(client)
    }

    /// Call an MCP tool directly
    pub async fn mcp_call(&self, tool_name: &str, args: Value) -> JsonRpcResult {
        tracing::info!(tool = tool_name, "call_tool");
        let client = self.mcp_client().await?;
        self.mcp_call_with(&client, tool_name, args).await
    }

    /// Call an MCP tool with a pre-created client (avoids redundant auth resolution)
    ///
    /// 自动处理 MCP 的 { code, message, data } 响应格式：
    /// - 如果 code == 0，返回 data 部分
    /// - 如果 code != 0，返回错误
    /// - 如果不是标准格式，原样返回
    pub async fn mcp_call_with(
        &self,
        client: &crate::mcp::McpClient,
        tool_name: &str,
        args: Value,
    ) -> JsonRpcResult {
        let result = client
            .call_tool(tool_name, args)
            .await
            .map_err(|e| JsonRpcError::new(error_codes::NETWORK_ERROR, e.to_string()))?;

        // 处理 MCP 的标准响应格式 { code, message, data }
        if let Some(code) = result.get("code").and_then(serde_json::Value::as_i64) {
            if code == 0 {
                // 成功，返回 data 部分（如果没有 data 则返回整个结果）
                return Ok(result.get("data").cloned().unwrap_or(result));
            } else {
                // 失败，返回错误
                let message = result
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("MCP call failed");
                return Err(JsonRpcError::new(
                    error_codes::NETWORK_ERROR,
                    format!("MCP error {}: {}", code, message),
                ));
            }
        }

        // 不是标准格式，原样返回
        Ok(result)
    }

    /// Extract a required string param
    pub fn require_str<'a>(&self, params: &'a Value, key: &str) -> Result<&'a str, JsonRpcError> {
        params
            .get(key)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError::invalid_params(format!("Missing {key}")))
    }
}

// ═══════════════════════════════════════════════════════════
//  Method Registry (inventory-based)
// ═══════════════════════════════════════════════════════════

/// Type-erased async handler function
type HandlerFn =
    fn(&ServeContext, Value) -> Pin<Box<dyn Future<Output = JsonRpcResult> + Send + '_>>;

/// A registered JSON-RPC method descriptor
pub struct RpcMethod {
    /// Method name (e.g., "space/list", "entry/tree")
    pub name: &'static str,
    /// Handler function
    pub handler: HandlerFn,
}

inventory::collect!(RpcMethod);

/// Macro to submit a method to the registry at compile time
///
/// # Usage
/// ```ignore
/// inventory::submit! { rpc_method!("space/list", handle_space_list) }
/// ```
#[macro_export]
macro_rules! rpc_method {
    ($name:literal, $handler:expr) => {
        $crate::serve::RpcMethod {
            name: $name,
            handler: |ctx, params| Box::pin($handler(ctx, params)),
        }
    };
}

// ═══════════════════════════════════════════════════════════
//  Error Codes
// ═══════════════════════════════════════════════════════════

pub mod error_codes {
    pub const AUTH_EXPIRED: i32 = -32001;
    pub const AUTH_REQUIRED: i32 = -32001;
    pub const NOT_FOUND: i32 = -32002;
    pub const QUOTA_EXCEEDED: i32 = -32003;
    pub const NETWORK_ERROR: i32 = -32004;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INTERNAL_ERROR: i32 = -32603;
}

// ═══════════════════════════════════════════════════════════
//  Entry Point
// ═══════════════════════════════════════════════════════════

/// Run the JSON-RPC server on stdio
pub async fn run_serve(config: Config, verbose: bool) -> Result<()> {
    if verbose {
        let methods: Vec<&str> = inventory::iter::<RpcMethod>
            .into_iter()
            .map(|m| m.name)
            .collect();
        eprintln!("[lx serve] registered methods: {:?}", methods);
        eprintln!("[lx serve] starting JSON-RPC server on stdio (verbose mode)");
    }

    // 预加载 token 到内存：即使过期也加载，让 mcp_client() 统一处理 refresh
    let token_data = crate::auth::load_token().ok().flatten();
    let access_token = token_data.as_ref().map(|t| t.access_token.clone());

    if let Some(ref token) = access_token {
        tracing::info!(
            token_len = token.len(),
            "mcp_client: token loaded from file"
        );
    } else {
        tracing::info!("mcp_client: no valid token found");
    }

    let mut state = ServeState::new(config);
    state.access_token = access_token;
    let state = Arc::new(RwLock::new(state));
    let transport = ServeTransport::new(state, verbose);

    transport.run().await
}
