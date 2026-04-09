---
name: lx-sh
version: 1.0.0
description: "虚拟 Shell 引擎，用 UNIX 命令浏览和探索乐享知识库。支持 ls/cat/grep/find/tree/awk/fzf 等命令、管道、重定向、变量替换。两种模式：Worktree 本地模式和 MCP 远程模式。触发词：shell、sh、浏览知识库、grep、ls、cat、搜索文件、虚拟shell"
metadata:
  requires:
    bins: ["lx"]
---

# 虚拟 Shell

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**用户说"浏览知识库"/"搜索文件内容"/"查看文档结构"** → 用本 skill（只读，不修改内容）

**用户说"编辑某个页面"/"推送修改"** → 用 lx-git 或 lx-block skill

## ⚡ 怎么选命令？（快速决策树）

```
用户想 →
├── 交互式浏览知识库内容? → lx sh（REPL 模式）
├── 执行单条命令后退出? → lx sh --exec "<command>"
├── 连接远程知识库（无需本地 clone）? → lx sh --space sp_xxx
├── 在已有 worktree 中浏览? → cd <worktree> && lx sh
├── 搜索知识库内容? → lx sh --exec "grep -r <pattern> /kb"
├── 查看文档结构? → lx sh --exec "tree /kb"
└── 需要修改内容（push/commit）? → 切换到 lx-git skill
```

## 两种运行模式

| 模式 | 启动方式 | /kb 映射 | 适用场景 |
|------|----------|----------|----------|
| **Worktree** | `cd <worktree> && lx sh` | 本地磁盘 | 已 clone 的知识库，需要快速浏览 |
| **MCP 远程** | `lx sh --space sp_xxx` | 远程 MCP API | 不想 clone，直接浏览远程内容 |

## 可用工具

### 启动命令

| 命令 | 说明 |
|------|------|
| `lx sh` | 启动 REPL 交互模式 |
| `lx sh --exec "<cmd>"` | 执行单条命令后退出 |
| `lx sh --space sp_xxx` | MCP 远程模式 |
| `lx sh --path <dir>` | 指定 worktree 路径 |

详细参数：`lx sh --help`

### Shell 内置命令（在 `lx sh` 中使用）

| 分类 | 命令 |
|------|------|
| **文件浏览** | `ls`, `cat`, `tree`, `stat` |
| **搜索** | `grep`, `find`, `fzf` |
| **文本处理** | `head`, `tail`, `wc`, `sort`, `uniq`, `awk`, `cut`, `tr` |
| **导航** | `cd`, `pwd`, `echo` |
| **桥接命令** | `search`, `mcp`, `git` |

详细参数：`lx sh --help`，或在 Shell 内输入 `help`

## 🎯 执行规则

1. **只读文件系统**：Shell 中的 `/kb` 是只读的，`rm`/`mv`/`cp`/`mkdir`/`touch` 等写命令会被拦截。需要修改内容请使用 `lx git` 或 `lx block` 命令。
2. **管道和重定向**：完整支持 `|` 管道和 `>` 重定向。如 `grep -r "API" /kb | head -20`。
3. **别名系统**：`rg` → `grep -rn`、`eza` → `ls`、`fd` → `find`、`bat` → `cat`。用户使用现代工具名时自动转换。
4. **Shell 内 git 只读**：在 Shell 中输入 `git status`/`git log`/`git diff` 可用（只读），但 `git push`/`git commit` 等写操作会提示退出 Shell 用 `lx git` 执行。
5. **`/tmp` 可写**：`/tmp` 目录是内存文件系统，可以写入临时数据。
6. **非交互模式**：Agent 应优先使用 `lx sh --exec "..."` 单次执行，避免启动 REPL。
7. **search 命令**：Shell 内置的 `search <keyword>` 是对 MCP `search_kb_search` 的快捷封装。
8. **mcp 透传**：Shell 内的 `mcp <tool_name> [json_args]` 可以直接调用任意 MCP tool。

## 典型组合流程

### 浏览远程知识库结构

```bash
# 直接连接远程，不需要 clone
lx sh --space sp_xxx

# 在 Shell 中：
tree /kb --depth 2
ls -la /kb/项目文档/
cat /kb/项目文档/README.md
```

### 在知识库中搜索内容

```bash
# 单次执行
lx sh --exec "grep -r 'OAuth' /kb | head -10"

# 或使用内置 search
lx sh --exec "search OAuth 认证"
```

### 分析文档内容

```bash
lx sh --exec "wc -l /kb/**/*.md"
lx sh --exec "grep -rl 'TODO' /kb"
lx sh --exec "find /kb -name '*.md' | wc -l"
```

### 编程 API 用法（Agent 集成）

```bash
# 执行命令并获取输出
lx sh --exec "grep -r 'API 设计' /kb | head -5"

# 透传 MCP 调用
lx sh --exec 'mcp entry_describe_entry {"entry_id": "entry_xxx"}'
```
