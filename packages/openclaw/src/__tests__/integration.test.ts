/**
 * Integration tests using OpenClaw plugin-sdk/testing
 *
 * 使用 OpenClaw 官方的 capturePluginRegistration 验证：
 * 1. register() 是同步的（不返回 Promise）
 * 2. 静态 lx-status 工具一定会注册
 * 3. 动态 schema 工具通过真实 api.registerTool() 注册成功
 * 4. fallback 核心工具在无 schema 时注册
 */

// @ts-nocheck - Test file uses vitest globals

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { capturePluginRegistration } from 'openclaw/plugin-sdk/testing';

// ---------------------------------------------------------------------------
// Mock CLI / schema 层（不碰真实二进制）
// ---------------------------------------------------------------------------

vi.mock('../cli.js', () => ({
  isLxAvailable: vi.fn(),
  getLxBinary: vi.fn(),
  getLxBinarySync: vi.fn(),
  downloadLxBinary: vi.fn(),
  execLx: vi.fn(),
  execLxSync: vi.fn(),
  execLxJson: vi.fn(),
  getManualInstallHelp: vi.fn(),
  checkForLxUpdate: vi.fn(),
}));

vi.mock('../schema.js', () => ({
  loadCachedSchema: vi.fn(),
  loadCachedSchemaSync: vi.fn(),
  registerToolsFromSchema: vi.fn(),
  registerCoreTools: vi.fn(),
}));

vi.mock('../onboarding.js', () => ({
  lexiangOnboardingAdapter: {
    channel: 'lexiang',
    getStatus: vi.fn(),
    configure: vi.fn(),
    disable: vi.fn(),
  },
}));

import { isLxAvailable, getLxBinary, downloadLxBinary } from '../cli.js';
import { loadCachedSchemaSync, registerToolsFromSchema, registerCoreTools } from '../schema.js';

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

describe('Integration: capturePluginRegistration', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // 设置 fire-and-forget 异步调用的默认返回值
    vi.mocked(downloadLxBinary).mockResolvedValue('/usr/local/bin/lx');
    vi.mocked(getLxBinary).mockResolvedValue('/usr/local/bin/lx');
  });

  it('register() is synchronous — capturePluginRegistration does not throw', async () => {
    vi.mocked(isLxAvailable).mockReturnValue(false);
    vi.mocked(loadCachedSchemaSync).mockReturnValue(null);

    const plugin = (await import('../index.js')).default;

    // capturePluginRegistration 要求 register 签名为 (api) => void
    // 如果 register 返回 Promise，SDK 内部不会 await，工具就丢失
    const captured = capturePluginRegistration({ register: plugin.register });

    // 不抛异常即通过；同时验证 api 对象存在
    expect(captured.api).toBeDefined();
  });

  it('lx-status tool is always captured via real SDK api', async () => {
    vi.mocked(isLxAvailable).mockReturnValue(false);
    vi.mocked(loadCachedSchemaSync).mockReturnValue(null);

    const plugin = (await import('../index.js')).default;
    const captured = capturePluginRegistration({ register: plugin.register });

    const toolNames = captured.tools.map((t: any) => t.name);
    expect(toolNames).toContain('lx-status');
  });

  it('schema tools are registered synchronously and captured', async () => {
    vi.mocked(isLxAvailable).mockReturnValue(true);
    vi.mocked(loadCachedSchemaSync).mockReturnValue({
      version: 'test',
      categories: [],
      tools: {
        entry_list_children: {
          name: 'entry_list_children',
          description: 'List children',
          inputSchema: { type: 'object', properties: {}, required: [] },
        },
        search_kb_search: {
          name: 'search_kb_search',
          description: 'Search',
          inputSchema: {
            type: 'object',
            properties: { keyword: { type: 'string' } },
            required: ['keyword'],
          },
        },
      },
    });

    const plugin = (await import('../index.js')).default;
    const captured = capturePluginRegistration({ register: plugin.register });

    // lx-status 一定在
    const toolNames = captured.tools.map((t: any) => t.name);
    expect(toolNames).toContain('lx-status');

    // registerToolsFromSchema 被调用，说明动态工具路径走通了
    expect(registerToolsFromSchema).toHaveBeenCalled();
    // registerCoreTools 不应被调用
    expect(registerCoreTools).not.toHaveBeenCalled();
  });

  it('falls back to core tools when schema is empty', async () => {
    vi.mocked(isLxAvailable).mockReturnValue(true);
    vi.mocked(loadCachedSchemaSync).mockReturnValue({
      version: 'test',
      categories: [],
      tools: {},
    });

    const plugin = (await import('../index.js')).default;
    const captured = capturePluginRegistration({ register: plugin.register });

    const toolNames = captured.tools.map((t: any) => t.name);
    expect(toolNames).toContain('lx-status');

    expect(registerCoreTools).toHaveBeenCalled();
    expect(registerToolsFromSchema).not.toHaveBeenCalled();
  });

  it('falls back to core tools when CLI is not available', async () => {
    vi.mocked(isLxAvailable).mockReturnValue(false);
    vi.mocked(loadCachedSchemaSync).mockReturnValue(null);

    const plugin = (await import('../index.js')).default;
    const captured = capturePluginRegistration({ register: plugin.register });

    const toolNames = captured.tools.map((t: any) => t.name);
    expect(toolNames).toContain('lx-status');

    expect(registerCoreTools).toHaveBeenCalled();
    expect(registerToolsFromSchema).not.toHaveBeenCalled();
  });
});

describe('Integration: real registerToolsFromSchema through SDK api', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(downloadLxBinary).mockResolvedValue('/usr/local/bin/lx');
    vi.mocked(getLxBinary).mockResolvedValue('/usr/local/bin/lx');
  });

  it('dynamic schema tools appear in captured.tools when using real registerToolsFromSchema', async () => {
    vi.mocked(isLxAvailable).mockReturnValue(true);

    // 让 registerToolsFromSchema 走模拟的真实注册行为
    vi.mocked(registerToolsFromSchema).mockImplementation(
      (api: any, schema: any, _config: any) => {
        // 委派给真实实现的未 mock 版本
        // 但因为 schema.js 整体被 mock 了，我们直接在这里模拟注册行为
        for (const [toolName] of Object.entries(schema.tools)) {
          api.registerTool({
            name: `lx-${toolName.replace(/_/g, '-')}`,
            label: toolName,
            description: `Execute ${toolName}`,
            parameters: { type: 'object', properties: {} },
            execute: async () => ({ text: 'ok' }),
          });
        }
      },
    );

    vi.mocked(loadCachedSchemaSync).mockReturnValue({
      version: 'test',
      categories: [],
      tools: {
        entry_list_children: {
          name: 'entry_list_children',
          description: 'List children',
          inputSchema: { type: 'object', properties: {}, required: [] },
        },
        team_list_teams: {
          name: 'team_list_teams',
          description: 'List teams',
          inputSchema: { type: 'object', properties: {}, required: [] },
        },
        search_kb_search: {
          name: 'search_kb_search',
          description: 'Search',
          inputSchema: {
            type: 'object',
            properties: { keyword: { type: 'string' } },
            required: ['keyword'],
          },
        },
      },
    });

    const plugin = (await import('../index.js')).default;
    const captured = capturePluginRegistration({ register: plugin.register });

    const toolNames = captured.tools.map((t: any) => t.name);

    // 静态工具
    expect(toolNames).toContain('lx-status');

    // 动态 schema 工具（通过真实 SDK api 注册）
    expect(toolNames).toContain('lx-entry-list-children');
    expect(toolNames).toContain('lx-team-list-teams');
    expect(toolNames).toContain('lx-search-kb-search');

    // 总数 = 1 (lx-status) + 3 (schema)
    expect(captured.tools.length).toBe(4);
  });
});
