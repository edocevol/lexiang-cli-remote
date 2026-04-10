# shell — 虚拟 Shell 引擎

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解虚拟 Shell 的整体决策树和运行模式。

纯 Rust 实现的虚拟 Shell 引擎，提供完整的 UNIX 命令行体验来浏览知识库。支持管道、重定向、变量替换、别名系统。

## 启动方式

### REPL 交互模式（默认）

```bash
# 在 worktree 目录中自动检测
cd ~/my-kb && lx sh

# 指定 worktree 路径
lx sh --path ~/my-kb

# 连接远程知识库（无需 clone）
lx sh --space sp_xxx

# 使用完整 URL
lx sh --space "https://lexiangla.com/spaces/sp_xxx"
```

### 非交互模式（单次执行，Agent 推荐）

```bash
lx sh --exec "tree /kb --depth 2"
lx sh --space sp_xxx --exec "ls /kb"
lx sh --exec "grep -r 'API' /kb | sort | uniq"
```

## 启动参数

| 参数 | 说明 |
|------|------|
| `--space <id>` | 知识库 ID 或 URL（MCP 远程模式，无需本地 worktree） |
| `--path <dir>` | 指定 worktree 目录路径（默认从 cwd 自动检测 `.lxworktree`） |
| `--exec <cmd>` | 执行单条命令后退出（非交互模式） |

**模式优先级**：`--space` > `--path` > 自动检测 cwd

## 文件系统布局

```text
/             → InMemoryFs（基础，只读）
/kb           → WorktreeFs（本地磁盘）或 LexiangFs（MCP 远程）
/tmp          → InMemoryFs（临时区，可写）
```

默认 cwd = `/kb`

## 内置命令

### 核心文件命令

```bash
ls [-la] [path]              # 列出目录内容
cat <file>                  # 输出文件内容
tree [path] [--depth N]    # 树形显示目录结构
stat <path>                # 显示文件/目录详细信息
```

### 搜索命令

```bash
grep [-rni] <pattern> [path]   # 正则搜索
find <path> [-name <p>] [-type f|d]  # 按路径/名称/类型查找
fzf                            # 交互式模糊搜索
```

### 文本处理命令

```bash
head [-n N] <file>    # 显示文件前 N 行
tail [-n N] <file>    # 显示文件后 N 行
wc [-lwc] <file>      # 行/词/字符计数
sort [-rn]             # 排序
uniq [-c]             # 去重
awk '{print $1}'       # 文本处理
cut -d',' -f1,2       # 按分隔符切割字段
tr 'a-z' 'A-Z'        # 字符转换
```

### 导航命令

```bash
cd <path>    # 切换目录
pwd          # 显示当前工作目录
echo <text>  # 输出文本（支持变量替换）
xargs        # 将标准输入转为命令参数
```

### 写入守卫命令（只读文件系统中被拦截）

以下命令会返回提示而不会执行：`rm`, `mv`, `cp`, `mkdir`, `touch`, `chmod`

### Shell 内桥接命令

| 命令 | 可用模式 | 说明 |
|------|----------|------|
| `search <keyword>` | 两种模式 | 搜索知识库（封装 MCP `search_kb_search`） |
| `mcp <tool> [json]` | 两种模式 | 透传调用任意 MCP tool |
| `git status/log/diff/remote` | 仅 Worktree | 只读 Git 操作 |
| `git pull/push/add/commit` | 仅 Worktree | **拦截** — 提示用 `lx git <cmd>` 在终端执行 |

## 别名系统

| 别名 | 实际命令 |
|------|----------|
| `rg` | `grep -rn` |
| `eza` | `ls` |
| `fd` | `find` |
| `bat` | `cat` |

## Shell 引擎特性

- **管道**：`grep -r "API" /kb | sort | uniq -c | head`
- **重定向**：`cat /kb/doc.md > /tmp/copy.md`
- **变量替换**：`echo $PWD`
- **完整词法/语法分析器**：支持引号、转义、通配符

## REPL 特殊命令

| 命令 | 说明 |
|------|------|
| `help` | 显示可用命令列表 |
| `exit` / `quit` | 退出 Shell |
| `Ctrl+D` | 退出 Shell |
| `Ctrl+C` | 取消当前输入行 |

## 注意事项

> [!CAUTION]
>
> - `/kb` 是**只读**的，无法在 Shell 中修改知识库内容
> - 需要修改内容请使用 `lx git push`（lx-git skill）或 `lx block` / `lx entry` 命令
> - MCP 远程模式下，文件内容是按需从 API 拉取的，首次访问可能较慢
> - Shell 内的 `git` 命令是桥接版本，仅支持只读子命令
> - Agent 应优先使用 `--exec` 非交互模式，避免卡在 REPL 等待输入

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx sh --help
```

在 Shell 内查看命令帮助：

```bash
help        # 列出所有可用命令
```

## 参考

- [lx-sh](../SKILL.md) — Shell skill 完整决策树
- [lx-git](../../lx-git/SKILL.md) — 需要修改内容时使用 Git 工作流
