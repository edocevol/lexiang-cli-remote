/**
 * 认证桥接层。
 *
 * 通过 lx serve JSON-RPC 做认证（auth/status, auth/startOAuth, auth/completeOAuth）。
 *
 * 认证流程：
 * 1. `auth/status` — 检查是否已认证（Rust 端读取 ~/.lexiang/auth/token.json）
 * 2. 如果已认证 → 直接返回
 * 3. 如果未认证 → 触发两阶段 OAuth:
 *    a. `auth/startOAuth` → 获取 authUrl
 *    b. VS Code 用 `vscode.env.openExternal()` 打开浏览器
 *    c. `auth/completeOAuth` → 等待回调完成
 *
 * 两阶段 OAuth 的好处：
 * - `vscode.env.openExternal()` 在 Remote SSH 下也能正常打开本地浏览器
 * - 不依赖 CLI 的 `open_browser()` 函数
 */

import * as vscode from 'vscode';

import type { LxRpcClient, RpcError } from '../rpc/lx-rpc-client.js';

/**
 * 认证桥接层。
 *
 * Rust 端管理 token 文件（~/.lexiang/auth/token.json），
 * VSCode 端只负责触发 OAuth 流程，不缓存认证状态。
 */
export class AuthBridge {
  private readonly changeEmitter = new vscode.EventEmitter<void>();
  readonly onDidChange = this.changeEmitter.event;

  /** 防止并发 OAuth 流程 */
  private oauthPromise: Promise<void> | null = null;

  constructor(
    private readonly rpcClient?: LxRpcClient,
  ) {}

  /**
   * 确保用户已认证（RPC 模式下直接返回，不弹进度条）。
   *
   * RPC 模式下认证由 lx serve 进程管理，token 存储在 Rust 端。
   * 每次调用都直接问 Rust，不缓存，避免状态不一致。
   */
  async ensureAuthenticated(): Promise<void> {
    // 1. 防止并发 OAuth
    if (this.oauthPromise) {
      await this.oauthPromise;
      return;
    }

    try {
      // 2. 检查认证状态（Rust 端读取 token 文件）
      const status = await this.rpcClient!.sendRequest<{
        authenticated: boolean;
      }>('auth/status', {});

      if (status.authenticated) {
        return;
      }

      // 3. 未认证 → 触发 OAuth
      this.oauthPromise = this.performOAuthFlow();
      try {
        await this.oauthPromise;
      } finally {
        this.oauthPromise = null;
      }
    } catch (err) {
      const rpcErr = err as RpcError;
      if (rpcErr?.isAuthError?.()) {
        throw err;
      }
      throw new Error(`认证失败: ${err instanceof Error ? err.message : String(err)}`);
    }
  }

  /**
   * 两阶段 OAuth 流程：startOAuth → 打开浏览器 → completeOAuth
   */
  private async performOAuthFlow(): Promise<void> {
    if (!this.rpcClient?.isReady()) {
      throw new Error('认证服务不可用：lx serve 未运行');
    }

    // 2a. 启动 OAuth，获取授权 URL
    const startResult = await this.rpcClient.sendRequest<{ authUrl: string }>(
      'auth/startOAuth',
      {},
    );

    // 2b. 用 VS Code API 打开浏览器（兼容 Remote SSH）
    const authUrl = startResult.authUrl;
    const opened = await vscode.env.openExternal(vscode.Uri.parse(authUrl));
    if (!opened) {
      // 浏览器打开失败，提示用户手动复制
      const copyAction = '复制链接';
      const result = await vscode.window.showWarningMessage(
        '无法自动打开浏览器，请手动复制链接到浏览器中打开。',
        copyAction,
      );
      if (result === copyAction) {
        await vscode.env.clipboard.writeText(authUrl);
        void vscode.window.showInformationMessage('授权链接已复制到剪贴板');
      }
    } else {
      void vscode.window.showInformationMessage('请在浏览器中完成登录授权...');
    }

    // 2c. 等待 OAuth 回调完成（超时 120 秒）
    try {
      const completeResult = await this.rpcClient.sendRequest<{ success: boolean }>(
        'auth/completeOAuth',
        {},
        120_000, // 120 秒超时
      );

      if (completeResult.success) {
        this.changeEmitter.fire();
        return;
      }

      throw new Error('OAuth 授权未成功完成');
    } catch (err) {
      const rpcErr = err as RpcError;
      if (rpcErr?.isAuthError?.()) {
        throw new Error('OAuth 授权超时或失败，请重试');
      }
      throw err;
    }
  }

  /**
   * 查询当前认证状态（不触发登录）。
   * 直接问 Rust 端，不缓存。
   */
  async getAuthStatus(): Promise<{ authenticated: boolean }> {
    if (!this.rpcClient?.isReady()) {
      return { authenticated: false };
    }

    try {
      return await this.rpcClient.sendRequest<{
        authenticated: boolean;
      }>('auth/status', {});
    } catch {
      return { authenticated: false };
    }
  }

  /**
   * 在 VSCode 的 withProgress 上下文中执行认证，显示进度提示。
   * 先快速检查，已认证直接返回不弹进度。
   */
  async ensureAuthenticatedWithProgress(): Promise<void> {
    // 先快速检查，已认证直接返回
    const status = await this.getAuthStatus();
    if (status.authenticated) {
      return;
    }

    return vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Window,
        title: '正在验证乐享账户',
        cancellable: false,
      },
      async () => {
        await this.ensureAuthenticated();
      },
    );
  }

}
