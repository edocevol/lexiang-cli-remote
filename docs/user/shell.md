# 虚拟 Shell (`lx sh`)

`lx sh` 提供一个面向知识库的虚拟 Shell。用熟悉的 Bash 风格命令浏览和搜索知识库，无需记 API 参数。

## 运行模式

| 模式 | 说明 | 启动方式 |
|------|------|---------|
| **Worktree** | 本地 worktree 目录映射 | `cd my-kb && lx sh` |
| **MCP 远程** | 直接连远端知识库 | `lx sh --space <SPACE_ID>` |
| **单次执行** | 执行一条命令即退出 | `lx sh -e "ls /kb"` |

## 文件系统布局

```text
/
├── kb/     # 知识库挂载点（只读）
└── tmp/    # 临时可写区域
```

- `/kb`：Worktree 模式映射到本地磁盘；MCP 模式实时加载远端
- `/tmp`：中间结果存储（sort/uniq/重定向用）
- 默认工作目录：`/kb`

## 内置命令

### 核心

| 命令 | 说明 | 示例 |
|------|------|------|
| `ls` | 列目录 | `ls -la /kb` |
| `cat` | 查看内容 | `cat /kb/README.md` |
| `grep` | 搜文本 | `grep -rni "OAuth" /kb` |
| `find` | 查文件 | `find /kb -name "*.md"` |
| `tree` | 目录树 | `tree -L 2 /kb` |

### 辅助

| 命令 | 说明 | 示例 |
|------|------|------|
| `head` / `tail` | 头/尾 | `head -20 /kb/README.md` |
| `wc` | 统计 | `wc -l /kb/docs/*.md` |
| `sort` / `uniq` | 排序/去重 | `cat log \| sort \| uniq -c` |
| `echo` | 输出 | `echo $PWD` |
| `pwd` / `cd` | 导航 | `cd /kb/docs && pwd` |
| `stat` | 文件信息 | `stat /kb/README.md` |
| `xargs` | 参数传递 | `find /kb -name "*.md" \| xargs grep "API"` |
| `fzf` | 模糊筛选 | `ls /kb \| fzf` |

### 桥接

| 命令 | 说明 | 示例 |
|------|------|------|
| `git` | Git 操作（仅 Worktree） | `git status`、`git log` |
| `search` | 知识库搜索 | `search OAuth 认证` |
| `mcp` | 透传 MCP Tool | `mcp entry_describe` |

> `git` 只在 Worktree 模式可用。写操作建议退出 Shell 后用 `lx git` 执行。

## 只读保护

以下命令返回"只读文件系统"错误：`rm`、`mv`、`cp`、`mkdir`、`touch`、`chmod`

## 现代 CLI 兼容

自动翻译常见现代 CLI 工具：

| 输入 | 实际执行 |
|------|---------|
| `rg "p" /kb` | `grep -rn "p" /kb` |
| `eza -la /kb` | `ls -la /kb` |
| `fd readme /kb` | `find /kb -name '*readme*'` |
| `bat /kb/README.md` | `cat /kb/README.md` |
| `ll` / `la` / `l` | `ls -la` / `ls -a` / `ls` |

## 支持的 Bash 语法

```bash
# 管道
ls /kb | grep "产品" | head -5

# 逻辑运算
cd /kb/docs && cat README.md

# 分号
pwd; ls; echo "done"

# 变量
NAME="test"; echo $NAME

# Glob
cat /kb/*.md

# 重定向（写入 /tmp）
grep -r "API" /kb > /tmp/results.txt
```

## 典型工作流

### 快速浏览结构

```bash
lx sh -e "tree -L 2 /kb"
lx sh --space <SPACE_ID> -e "tree -L 2 /kb"
```

### 搜索 + 组合查询

```bash
cd my-kb && lx sh
# 进入后：
grep -rn "部署" /kb | head -10
find /kb -name '*.md' | xargs grep 'OAuth' | head -20
```

## 程序内调用（Rust）

```rust
use lexiang_cli::cmd::sh;

let mut bash = sh::build_shell(&config, None, None).await?;
let out = bash.exec("grep -r OAuth /kb | head -5").await?;
println!("{}", out.stdout);
```
