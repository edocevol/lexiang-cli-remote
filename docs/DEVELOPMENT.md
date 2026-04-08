# 开发指引

本文档面向 `lx-cli` 的开发者，介绍项目架构、代码组织和扩展方法。

## 项目结构

```text
├── schemas/                   # Schema 源文件（编译时嵌入）
│   ├── lexiang.json           #   listed tools schema（lx tools sync-embedded 自动更新）
│   └── unlisted.json          #   unlisted tools schema（tool_names 为配置源，lx tools sync-unlisted 更新）
├── src/
│   ├── main.rs                # CLI 入口
│   ├── auth/                  # OAuth 认证
│   │   ├── mod.rs
│   │   ├── oauth.rs           # OAuth 2.0 登录流程
│   │   └── token_store.rs     # Token 存储（JSON 文件）
│   ├── cmd/                   # 命令实现
│   │   ├── mod.rs
│   │   ├── cli.rs             # CLI 定义（clap derive）
│   │   ├── tools/mod.rs       # tools 子命令（sync / sync-embedded / sync-unlisted）
│   │   ├── dynamic_cli.rs     # 动态命令入口
│   │   ├── executor.rs        # 动态命令执行器
│   │   ├── lexiang.rs         # 传统命令（兼容层）
│   │   └── mcp.rs             # MCP 原始命令
│   ├── config.rs              # 配置管理
│   ├── daemon/                # 守护进程（VFS）
│   ├── json_rpc.rs            # JSON-RPC 协议
│   ├── mcp/                   # MCP 客户端
│   │   ├── client.rs          # MCP HTTP 客户端
│   │   ├── protocol.rs        # MCP 协议层类型（ToolSchema, InputSchema）
│   │   ├── mod.rs
│   │   └── schema/            # Schema 管理
│   │       ├── mod.rs         # SchemaManager
│   │       ├── types.rs       # Schema 类型定义 + 类型转换方法
│   │       ├── runtime.rs     # 运行时 Schema 管理
│   │       ├── embedded.rs    # 编译时嵌入 Schema（include_str! schemas/*.json）
│   │       ├── generator.rs   # 动态命令生成器
│   │       └── skill_generator.rs # Skill 文件生成
│   ├── output/                # 输出格式化
│   └── vfs/                   # 虚拟文件系统
├── Cargo.toml
└── build.rs                   # 构建脚本（监听 schemas/*.json 变更）
```

## 架构设计

### 三层命令架构

```text
┌─────────────────────────────────────────┐
│  Layer 3: Dynamic Commands              │
│  - lx team list                         │
│  - lx space describe                    │
│  - 从 MCP Schema 动态生成               │
├─────────────────────────────────────────┤
│  Layer 2: Legacy Commands               │
│  - lx lexiang search                    │
│  - 静态代码实现                         │
├─────────────────────────────────────────┤
│  Layer 1: Raw MCP Commands              │
│  - lx mcp list                          │
│  - lx mcp call                          │
│  - 直接调用 MCP Server                  │
└─────────────────────────────────────────┘
```

### Schema 四层合并策略

优先级：**embedded（含 unlisted）< override < custom < dynamic**

1. **embedded** - 编译时嵌入的 listed tools schema (`schemas/lexiang.json`)
2. **unlisted** - 编译时嵌入的 unlisted tools schema (`schemas/unlisted.json`)，与 embedded 合并后作为基础层
3. **override** - 运行时从 MCP Server 同步的 schema (`~/.lexiang/tools/override.json`)
4. **custom** - 用户自定义覆盖 (`~/.lexiang/tools/custom.json`)

> **Listed vs Unlisted：** Listed tools 通过 `tools/list` API 可发现；Unlisted tools 不在任何 list API 中返回，但可通过 `tools/call` 调用。两者的 schema 分开维护，unlisted 不会被 `lx tools sync` 覆盖。

CLI 启动时优先加载 override，如果不存在则使用 embedded，确保用户安装后即可使用，无需先运行 `lx tools sync`。

## 核心模块

### 1. Schema 管理 (`mcp/schema/`)

#### 类型定义 (`types.rs`)

项目有两套 schema 类型：protocol 层（`mcp/protocol.rs`，properties 为 `Map<String, Value>`）和 schema 层（`mcp/schema/types.rs`，properties 为强类型）。转换方法封装在 schema 层类型上：

```rust
// schema 层类型
pub struct McpToolSchema { name, description, input_schema, namespace, command_name }
pub struct McpInputSchema { type_, properties: HashMap<String, McpPropertySchema>, required }
pub struct McpPropertySchema { type_, description, default, enum_values, items }

// 类型转换方法
McpPropertySchema::from_value_or_fallback(&Value)  // JSON Value → 属性，带 fallback
McpInputSchema::from_protocol(InputSchema)          // protocol 层 → schema 层
McpInputSchema::from_value(&Value)                  // JSON Value → 输入 schema
McpInputSchema::to_protocol() -> InputSchema        // schema 层 → protocol 层
McpToolSchema::from_protocol(&ToolSchema)           // protocol 层 → schema 层
McpToolSchema::from_raw_response(name, &Value)      // get_tool_schema 响应 → schema 层
McpToolSchema::to_protocol() -> ToolSchema          // schema 层 → protocol 层
```

#### 运行时管理 (`runtime.rs`)

```rust
pub struct RuntimeSchemaManager {
    config_dir: PathBuf,  // ~/.lexiang/tools/
}

impl RuntimeSchemaManager {
    pub fn load(&self) -> Result<Option<McpSchemaCollection>>;
    pub async fn sync_from_server(
        &self,
        client: &McpClient
    ) -> Result<McpSchemaCollection>;
    pub fn save(&self, schema: &McpSchemaCollection) -> Result<()>;
}
```

#### 命令生成器 (`generator.rs`)

从 schema 动态生成 clap 命令：

```rust
pub struct CommandGenerator<'a> {
    schema: &'a McpSchemaCollection,
}

impl<'a> CommandGenerator<'a> {
    pub fn generate_namespaces(&self) -> Vec<Command>;
    pub fn generate_tool_command(
        &self,
        tool: &McpCategoryTool,
        namespace: &str
    ) -> Command;
}
```

### 2. 动态命令执行 (`cmd/dynamic_cli.rs`)

```rust
/// 处理动态命令（基于 schema 的 namespace 命令）
async fn handle_dynamic_command(
    args: &[String],
    schema: &McpSchemaCollection,
) -> Result<()> {
    // 1. 构建动态命令树
    // 2. 解析参数
    // 3. 查找对应 tool name
    // 4. 调用 MCP
    // 5. 格式化输出
}
```

### 3. Token 管理 (`auth/token_store.rs`)

使用 JSON 文件存储，支持自动 refresh：

```rust
impl TokenStore {
    // ~/.lexiang/auth/{company}.json
    pub fn save(token: &TokenData) -> Result<()>;
    pub fn load(company_from: &str) -> Result<Option<TokenData>>;
    // 自动 refresh
    pub async fn get_valid_token(
        company_from: &str
    ) -> Result<Option<TokenData>>;
}
```

## 开发流程

### 添加新的输出格式

1. 修改 `generator.rs` 中的 `generate_tool_command`，添加 format 参数
2. 在 `main.rs` 的 `handle_dynamic_command` 中添加格式处理逻辑
3. 实现对应的格式化函数（如 `print_csv`, `print_markdown`）

### 添加新的 Schema 字段支持

1. 在 `types.rs` 中添加字段定义
2. 在 `generator.rs` 的 `create_argument` 中添加参数类型映射
3. 在 `build_tool_args` 中添加参数转换逻辑

### 添加新的工具分类

MCP Server 添加新分类后：

1. 运行 `lx tools sync` 同步 schema
2. 动态命令自动生成，无需代码修改
3. 如需生成 skill 文件：`lx tools skill`

### 更新内置 Schema

#### Listed Tools（`schemas/lexiang.json`）

当 MCP Server 有更新时，使用自举命令直接从 server 拉取最新 schema：

```bash
# 从 MCP Server 拉取并写入 schemas/lexiang.json
lx tools sync-embedded
# 或开发阶段
cargo run -- tools sync-embedded

# 重新构建，嵌入更新后的 schema
cargo build --release
```

`sync-embedded` 流程：调用 `list_tool_categories` + `tools/list` → 构建 `McpSchemaCollection` → 写入 `schemas/lexiang.json`。

#### Unlisted Tools（`schemas/unlisted.json`）

Unlisted tools 不在 `tools/list` 中返回，通过配置即代码管理：

```bash
# 1. 在 schemas/unlisted.json 的 tool_names 数组中添加 tool name
# 2. 运行同步命令，自动调用 get_tool_schema 获取每个 tool 的完整 schema
lx tools sync-unlisted
# 或
cargo run -- tools sync-unlisted

# 3. 重新构建
cargo build --release
```

`schemas/unlisted.json` 结构：

```json
{
  "_comment": "Unlisted-but-callable tools. tool_names is config-as-code.",
  "tool_names": [
    "entry_delete_entry",
    "file_list_revisions"
  ],
  "tools": {
    // ← 由 sync-unlisted 自动填充，勿手动编辑
  }
}
```

> **注意：** `build.rs` 会监听 `schemas/lexiang.json` 和 `schemas/unlisted.json` 的变更，修改后 `cargo build` 会自动重新编译嵌入。

## 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test mcp::schema::types::tests

# 构建并测试
cargo build --release
./target/release/lx --help
```

## 调试

```bash
# 启用日志
RUST_LOG=debug cargo run -- team list

# 查看 token 文件
cat ~/.lexiang/auth/token.json

# 查看运行时 schema 文件
cat ~/.lexiang/tools/override.json | jq

# 查看编译时 schema 源文件
cat schemas/lexiang.json | jq '.tools | keys | length'
cat schemas/unlisted.json | jq '.tool_names'

# 查看生成的 skill 文件
ls ~/.lexiang/skills/
```

## 发布

```bash
# 构建 release
cargo build --release
```

## Shell Completion

Completion 脚本由 `lx completion <shell>` 运行时自动生成，无需手工维护：

```bash
# Bash
lx completion bash > ~/.bash_completion.d/lx

# Zsh
lx completion zsh > ~/.zsh/completions/_lx

# Fish
lx completion fish > ~/.config/fish/completions/lx.fish
```

## 常见问题

### Q: 动态命令如何知道有哪些 namespace？

A: 按以下优先级加载 schema：

1. 优先从 `~/.lexiang/tools/override.json` 加载（如果存在）
2. 否则使用编译时嵌入的 `schemas/lexiang.json`（含 `schemas/unlisted.json` 合并）

解析其中的 categories 获取所有 namespace。

### Q: 如何支持新的参数类型？

A: 在 `generator.rs` 的 `create_argument` 中添加类型映射：

```rust
match type_str {
    "boolean" => arg.action(ArgAction::SetTrue),
    "array" => arg.action(ArgAction::Append),
    "integer" | "number" => {
        arg.value_parser(clap::value_parser!(i64))
    }
    _ => {} // string default
}
```

### Q: Token 过期如何处理？

A: `TokenStore::get_valid_token()` 会自动检查过期时间，如果过期且有 refresh_token，会自动刷新。
