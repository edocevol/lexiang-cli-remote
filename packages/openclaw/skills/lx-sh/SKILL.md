---
name: lx-sh
description: |
  虚拟 Shell 引擎，用 UNIX 命令浏览和探索乐享知识库。支持 ls/cat/grep/find/tree/awk/fzf 等命令、管道、重定向、变量替换。两种模式：Worktree 本地模式和 MCP 远程模式。触发词：shell、sh、浏览知识库、grep、ls、cat、搜索文件、虚拟shell
---

# lx-sh

**当以下情况时使用此 Skill**:

- 需要浏览知识库结构
- 需要在知识库中搜索文件内容
- 需要查看文档内容
- 需要用到 UNIX 命令（ls/cat/grep/find/tree 等）

## 工具一：交互式 Shell（REPL）

启动交互式 Shell 会话，支持管道、重定向、别名。

### 示例（启动 REPL）

```bash
lx sh
```

### 内置命令

| 分类 | 命令 |
|------|------|
| 文件浏览 | `ls`, `cat`, `tree`, `stat` |
| 搜索 | `grep`, `find`, `fzf` |
| 文本处理 | `head`, `tail`, `wc`, `sort`, `uniq`, `awk`, `cut`, `tr` |
| 导航 | `cd`, `pwd`, `echo` |
| 桥接 | `search`, `mcp`, `git` |

### 别名

- `rg` → `grep -rn`
- `eza` → `ls`
- `fd` → `find`
- `bat` → `cat`

## 工具二：单次执行模式

执行单条命令后退出，适合 Agent 编程调用。

### 参数（单次执行）

- **exec** (string, required): 要执行的命令

### 示例（单次执行）

```json
lx-sh: { "exec": "grep -r 'OAuth' /kb | head -20" }
```

```json
lx-sh: { "exec": "tree /kb --depth 2" }
```

```json
lx-sh: { "exec": "wc -l /kb/**/*.md" }
```

## 工具三：MCP 远程模式

无需 clone 知识库，直接连接远程浏览。

### 参数（远程模式）

- **space** (string, required): 知识库 ID

### 示例（远程浏览）

```json
lx-sh: { "tool": "remote", "space": "sp_xxx" }
```

等效于：`lx sh --space sp_xxx`

## 两种运行模式对比

| 模式 | 启动方式 | /kb 映射 | 适用场景 |
|------|----------|----------|----------|
| Worktree | `cd <worktree> && lx sh` | 本地磁盘 | 已 clone 的知识库 |
| MCP 远程 | `lx sh --space sp_xxx` | 远程 MCP API | 不想 clone，直接浏览远程 |

## 注意事项

- `/kb` 是只读文件系统，`rm`/`mv`/`cp`/`mkdir`/`touch` 等写命令会被拦截
- `/tmp` 目录可写（内存文件系统）
- Shell 内 `git status`/`git log`/`git diff` 可用（只读）
- Agent 应优先使用 `lx sh --exec "..."` 单次执行

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 交互式浏览 | 工具一：REPL 模式 |
| Agent 编程调用 | 工具二：单次执行 |
| 不想 clone，直接浏览远程 | 工具三：MCP 远程模式 |
| 搜索知识库内容 | 工具二：`grep -r <pattern> /kb` |
| 查看文档结构 | 工具二：`tree /kb` |
