---
name: lx-sh
description: |
  虚拟 Shell 引擎，用 UNIX 命令浏览和探索乐享知识库。支持 ls/cat/grep/find/tree/awk/fzf 等命令、管道、重定向、变量替换。两种模式：Worktree 本地模式和 MCP 远程模式。触发词：shell、sh、浏览知识库、grep、ls、cat、搜索文件、虚拟shell
---

# lx-sh

**当以下情况时使用此 Skill**:

- 需要交互式浏览知识库内容
- 需要执行单条命令后退出
- 需要连接远程知识库（无需本地 clone）
- 需要在已有 worktree 中浏览
- 需要搜索知识库内容
- 需要查看文档结构

## 工具一：启动 Shell

启动 REPL 交互模式或执行单条命令。

### 参数（启动 REPL）

无参数，直接启动交互式 Shell。

### 示例（启动 REPL）

```bash
lx sh
```

### 参数（执行单条命令）

- **exec** (string, required): 要执行的命令

### 示例（执行单条命令）

```json
lx-sh: { "tool": "exec", "exec": "grep -r 'API' /kb | head -20" }
```

### 参数（MCP 远程模式）

- **space** (string, required): 知识库 ID

### 示例（MCP 远程模式）

```json
lx-sh: { "tool": "exec", "space": "sp_xxx", "exec": "tree /kb --depth 2" }
```

### 参数（指定 worktree 路径）

- **path** (string, required): worktree 路径

### 示例（指定 worktree）

```json
lx-sh: { "tool": "exec", "path": "./my-kb", "exec": "ls -la" }
```

## Shell 内置命令（在 `lx sh` 中使用）

### 文件浏览

- `ls` - 列出目录内容
- `cat` - 查看文件内容
- `tree` - 以树形结构显示目录
- `stat` - 显示文件状态

### 搜索

- `grep` - 搜索文件内容
- `find` - 查找文件
- `fzf` - 模糊搜索

### 文本处理

- `head` - 显示文件开头
- `tail` - 显示文件结尾
- `wc` - 统计行数、字数、字节数
- `sort` - 排序
- `uniq` - 去重
- `awk` - 文本处理
- `cut` - 切割字段
- `tr` - 字符转换

### 导航

- `cd` - 切换目录
- `pwd` - 显示当前目录
- `echo` - 输出文本

### 桥接命令

- `search` - 搜索知识库（快捷封装）
- `mcp` - 透传 MCP 调用
- `git` - Git 命令（只读）

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 交互式浏览知识库 | 工具一：启动 REPL（`lx sh`） |
| 执行单条命令后退出 | 工具一：`lx sh --exec "..."` |
| 连接远程知识库（无需本地 clone） | 工具一：`lx sh --space sp_xxx` |
| 在已有 worktree 中浏览 | 工具一：`lx sh --path <dir>` |
| 搜索知识库内容 | 工具一：`lx sh --exec "grep -r <pattern> /kb"` |
| 查看文档结构 | 工具一：`lx sh --exec "tree /kb"` |

## 执行规则

1. **只读文件系统**：Shell 中的 `/kb` 是只读的，`rm`/`mv`/`cp`/`mkdir`/`touch` 等写命令会被拦截。需要修改内容请使用 `lx git` 或 `lx block` 命令。
2. **管道和重定向**：完整支持 `|` 管道和 `>` 重定向。如 `grep -r "API" /kb | head -20`。
3. **别名系统**：`rg` → `grep -rn`、`eza` → `ls`、`fd` → `find`、`bat` → `cat`。用户使用现代工具名时自动转换。
4. **Shell 内 git 只读**：在 Shell 中输入 `git status`/`git log`/`git diff` 可用（只读），但 `git push`/`git commit` 等写操作会提示退出 Shell 用 `lx git` 执行。
5. **`/tmp` 可写**：`/tmp` 目录是内存文件系统，可以写入临时数据。
6. **非交互模式**：Agent 应优先使用 `lx sh --exec "..."` 单次执行，避免启动 REPL。
7. **search 命令**：Shell 内置的 `search <keyword>` 是对 MCP `search_kb_search` 的快捷封装。
8. **mcp 透传**：Shell 内的 `mcp <tool_name> [json_args]` 可以直接调用任意 MCP tool。

## 禁止操作

- **不要修改内容**：用户说"编辑某个页面"/"推送修改" → **立即切换到 lx-git 或 lx-block skill**
- **Shell 是只读的**：`/kb` 是只读文件系统，`rm`/`mv`/`cp`/`mkdir`/`touch` 等写命令会被拦截

## 典型组合流程

### 浏览远程知识库结构

```json
// 直接连接远程，不需要 clone
lx-sh: { "tool": "exec", "space": "sp_xxx", "exec": "tree /kb --depth 2" }

lx-sh: { "tool": "exec", "space": "sp_xxx", "exec": "ls -la /kb/项目文档/" }

lx-sh: { "tool": "exec", "space": "sp_xxx", "exec": "cat /kb/项目文档/README.md" }
```

### 在知识库中搜索内容

```json
// 单次执行
lx-sh: { "tool": "exec", "exec": "grep -r 'OAuth' /kb | head -10" }

// 或使用内置 search
lx-sh: { "tool": "exec", "exec": "search OAuth 认证" }
```

### 分析文档内容

```json
lx-sh: { "tool": "exec", "exec": "wc -l /kb/**/*.md" }

lx-sh: { "tool": "exec", "exec": "grep -rl 'TODO' /kb" }

lx-sh: { "tool": "exec", "exec": "find /kb -name '*.md' | wc -l" }
```

### 编程 API 用法（Agent 集成）

```json
// 执行命令并获取输出
lx-sh: { "tool": "exec", "exec": "grep -r 'API 设计' /kb | head -5" }

// 透传 MCP 调用
lx-sh: { "tool": "exec", "exec": "mcp entry_describe_entry {\"entry_id\": \"entry_xxx\"}" }
```

## 两种运行模式对比

| 模式 | 启动方式 | /kb 映射 | 适用场景 |
|------|----------|----------|----------|
| **Worktree** | `cd <worktree> && lx sh` | 本地磁盘 | 已 clone 的知识库，需要快速浏览 |
| **MCP 远程** | `lx sh --space sp_xxx` | 远程 MCP API | 不想 clone，直接浏览远程内容 |
