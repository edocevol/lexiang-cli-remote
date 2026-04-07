# lx - 乐享知识库命令行工具

`lx` 是一个强大的命令行工具，让你可以在终端中完成几乎所有乐享知识库操作。

## 核心特性

### 🚀 即装即用，无需配置

```bash
cargo install --path .
lx login
lx search kb --keyword "文档"  # 立即可用
```

内置完整的 MCP Schema，安装后直接拥有 50+ 个命令，覆盖团队、知识库、文档、搜索等全部功能。

### 🔄 Git 风格的本地工作区

像使用 Git 一样管理乐享知识库，支持离线编辑、版本控制、批量同步：

```bash
lx git clone <space_id> ./my-kb    # 克隆知识库到本地
cd my-kb
# 编辑文件...
lx git add .                        # 暂存变更
lx git commit -m "更新文档"          # 提交
lx git push                         # 推送到远端
```

### 🔧 动态命令系统

CLI 自动从 MCP Schema 生成命令，新功能上线后只需一条命令即可获取：

```bash
lx tools sync  # 同步最新工具定义
```

所有命令都有完整的帮助信息：

```bash
lx --help              # 查看所有命令
lx team --help         # 查看团队相关命令
lx team list --help    # 查看具体命令的参数
```

### 🐚 虚拟 Shell（知识库探索）

用 `ls`、`cat`、`grep`、`find` 等熟悉的 Linux 命令直接浏览和搜索知识库：

```bash
cd my-kb && lx sh                         # 在 worktree 目录下启动（推荐）
lx sh --path ~/my-kb                      # 指定 worktree 路径
lx sh --space <SPACE_ID>                  # MCP 远程模式（无需本地 worktree）
lx sh -e "grep -r OAuth /kb | head -5"   # 单次执行
```

内置 20+ 命令，支持管道 `|`、逻辑运算 `&&`/`||`、变量 `$VAR`、Glob 通配符。自动识别 `rg`/`eza`/`fd`/`bat` 等现代 CLI 工具的参数格式。

### 📦 多格式输出

所有命令支持 6 种输出格式，轻松集成到各种工作流：

```bash
lx team list -o json        # JSON（紧凑）
lx team list -o json-pretty # JSON（格式化，默认）
lx team list -o table       # 表格
lx team list -o yaml        # YAML
lx team list -o csv         # CSV（方便 Excel 处理）
lx team list -o markdown    # Markdown 表格
```

### 💡 灵活的参数输入

支持两种参数传递方式：

```bash
# 方式1：逐参数传入（推荐日常使用）
lx search kb --keyword "test" --limit 10 --type doc

# 方式2：JSON 一次性传入（适合脚本和复杂参数）
lx search kb -d '{"keyword":"test","limit":10,"type":"doc"}'
```

`-d / --data-raw` 参数风格与 curl 一致，方便从 API 文档直接复制参数。

---

## 安装

### 从源码安装

```bash
git clone <repo-url>
cd lexiang-cli
cargo install --path .
```

### 验证安装

```bash
lx version
# lx-cli v0.1.0
```

---

## 快速开始

### 1. 登录授权

```bash
lx login
```

浏览器自动打开 OAuth 登录页面，授权后 token 保存在 `~/.lexiang/auth/token.json`。

### 2. 探索可用命令

```bash
lx --help
```

输出示例：

```
Lexiang CLI - A command-line tool for Lexiang MCP

Usage: lx [COMMAND]

Commands:
  search         Search in knowledge base
  fetch-doc      Fetch a document
  lexiang        Lexiang namespace commands
  mcp            MCP operations
  tools          Tools schema management
  sh             Virtual shell for knowledge base exploration
  ...

Dynamic Commands (from MCP schema):
  team           团队接口 (3 commands)
  space          知识库接口 (3 commands)
  entry          知识增删改查接口 (10 commands)
  block          在线文档接口 (10 commands)
  file           知识文件接口 (5 commands)
  search         search (2 commands)
  ppt            PPT服务相关接口 (6 commands)
  meeting        腾讯会议 (5 commands)
  comment        知识评论查询接口 (2 commands)
  contact        contact (2 commands)
  iwiki          iwiki (1 commands)
```

### 3. 开始使用

```bash
# 列出我的团队
lx team list

# 列出团队下的知识库
lx space list --team-id <TEAM_ID>

# 搜索知识
lx search kb --keyword "项目文档"
```

---

## 命令参考

### 全局选项

所有动态命令都支持：

| 选项 | 说明 |
|------|------|
| `-o, --format <FORMAT>` | 输出格式：json / json-pretty / table / yaml / csv / markdown |
| `-d, --data-raw <JSON>` | 以 JSON 格式传入所有参数 |
| `-h, --help` | 显示帮助信息 |

### 搜索命令

```bash
# 全局搜索
lx search kb --keyword "关键词"

# 指定知识库搜索
lx search kb --keyword "关键词" --space-id <SPACE_ID>

# 指定搜索类型
lx search kb --keyword "关键词" --type doc        # 仅搜索文档
lx search kb --keyword "关键词" --type kb_doc     # 仅搜索知识库文档
lx search kb --keyword "关键词" --title-only      # 仅搜索标题

# 限制结果数量
lx search kb --keyword "关键词" --limit 5

# 向量语义检索
lx search kb-embedding --keyword "如何部署服务"

# 使用 JSON 参数（适合复杂查询）
lx search kb -d '{"keyword":"test","space_id":"xxx","type":"doc","limit":10}'
```

### 团队命令

```bash
# 列出所有可访问的团队
lx team list
lx team list -o table

# 列出常用团队
lx team list-frequent

# 获取团队详情
lx team describe --team-id <TEAM_ID>
```

### 知识库命令

```bash
# 列出团队下的知识库
lx space list --team-id <TEAM_ID>

# 获取知识库详情（返回 root_entry_id 用于遍历目录）
lx space describe --space-id <SPACE_ID>

# 列出最近访问的知识库
lx space list-recently
```

### 条目命令

```bash
# 获取条目详情
lx entry describe --entry-id <ENTRY_ID>

# 获取子条目列表（目录遍历）
lx entry list-children --parent-id <PARENT_ID>

# 获取最近更新的条目
lx entry list-latest --space-id <SPACE_ID>

# 获取面包屑路径
lx entry list-parents --entry-id <ENTRY_ID>

# 创建文档
lx entry create --parent-entry-id <PARENT_ID> --name "新文档" --entry-type page

# 创建文件夹
lx entry create --parent-entry-id <PARENT_ID> --name "新文件夹" --entry-type folder

# 导入内容到新页面
lx entry import-content \
  --parent-id <PARENT_ID> \
  --name "导入的文档" \
  --content "# 标题\n\n正文内容"

# 导入内容到已有页面
lx entry import-content-to-entry \
  --entry-id <ENTRY_ID> \
  --content "追加的内容"

# 重命名
lx entry rename --entry-id <ENTRY_ID> --name "新名称"

# 移动
lx entry move --entry-id <ENTRY_ID> --parent-entry-id <NEW_PARENT_ID>

# 获取 AI 可解析内容（返回 markdown/html）
lx entry describe-ai-parse-content --entry-id <ENTRY_ID>
```

### 文档块命令

```bash
# 获取块详情
lx block describe --block-id <BLOCK_ID>

# 获取块的子节点
lx block list-children --block-id <BLOCK_ID>
lx block list-children --block-id <BLOCK_ID> --recursive  # 递归获取

# 更新块内容
lx block update -d '{"block_id":"xxx","content":{"text":"新内容"}}'

# 批量更新
lx block update-blocks -d '{"blocks":[...]}'

# 创建子块
lx block create-descendant -d '{"block_id":"xxx","descendant":{...}}'

# 删除块
lx block delete --block-id <BLOCK_ID>

# 批量删除子块
lx block delete-children -d '{"block_id":"xxx","children_ids":["id1","id2"]}'

# 移动块
lx block move -d '{"block_ids":["id1","id2"],"parent_block_id":"xxx"}'

# 内容转块结构
lx block convert-content-to-blocks -d '{"content":"# 标题","content_type":"markdown"}'
```

### 文件命令

```bash
# 获取文件详情
lx file describe --file-id <FILE_ID>

# 获取下载链接
lx file download --file-id <FILE_ID>

# 上传文件（三步流程）
# Step 1: 申请上传凭证
lx file apply-upload --parent-entry-id <PARENT_ID>
# Step 2: 使用返回的 upload_url 上传文件
curl -X PUT "<upload_url>" --data-binary @file.pdf
# Step 3: 确认上传
lx file commit-upload --session-id <SESSION_ID>

# 导入外部链接（如微信公众号文章）
lx file create-hyperlink \
  --url "https://mp.weixin.qq.com/s/xxx" \
  --space-id <SPACE_ID> \
  --parent-entry-id <PARENT_ID>
```

### 评论命令

```bash
# 获取页面评论列表
lx comment list --target-id <ENTRY_ID>

# 获取评论详情
lx comment describe --target-id <ENTRY_ID>
```

### PPT 命令

```bash
# 生成 PPT
lx ppt generate -d '{
  "planning": "10页，标题：产品介绍，风格：商务",
  "context": "产品功能特性说明..."
}'

# 查询任务状态
lx ppt get-task --id <TASK_ID>

# 添加页面
lx ppt add-pages -d '{
  "title": "产品介绍",
  "pages": [{"insert_after": 2, "title": "新页面", "key_points": "要点内容"}]
}'

# 修改页面
lx ppt modify-pages -d '{
  "title": "产品介绍",
  "pages": [{"page_index": 3, "modification": "将标题改为..."}]
}'

# 删除页面
lx ppt delete-pages --title "产品介绍" -d '{"page_indexes": [1, 2]}'

# 调整页面顺序
lx ppt reorder-pages -d '{"title": "产品介绍", "new_order": [3, 1, 2]}'
```

### 腾讯会议命令

```bash
# 搜索会议录制
lx meeting search --meeting-id <MEETING_CODE>

# 获取录制详情
lx meeting describe --record-id <RECORD_ID>

# 导入会议录制到知识库
lx meeting import -d '{
  "record_id": "xxx",
  "parent_entry_id": "yyy",
  "start_time": "2026-01-01T10:00:00",
  "end_time": "2026-01-01T11:00:00"
}'

# 重新导入
lx meeting reload --record-id <RECORD_ID>
```

### 联系人命令

```bash
# 搜索员工
lx contact search-staff --keyword "张三"

# 获取当前用户信息
lx contact whoami
```

### iWiki 命令

```bash
# 导入 iWiki 文档
lx iwiki import -d '{
  "page_id": "12345",
  "space_id": "xxx",
  "parent_entry_id": "yyy"
}'
```

---

## 虚拟 Shell（知识库探索）

`lx sh` 提供一个虚拟 Bash Shell，让你用熟悉的 Linux 命令（`ls`、`cat`、`grep`、`find` 等）直接浏览和检索乐享知识库内容，无需记忆 API 参数。

支持两种运行模式：
- **Worktree 模式**（默认）：在 `lx git clone` 创建的 worktree 目录下运行，`/kb` 映射到本地磁盘文件，内置 `git` 命令
- **MCP 远程模式**：通过 `--space` 直接连接远程知识库，内容实时从 MCP API 加载

### 快速开始

```bash
# ── Worktree 模式（推荐）──
lx git clone <SPACE_ID> ./my-kb    # 先克隆知识库
cd my-kb
lx sh                               # 自动检测 worktree 目录
lx sh --path ~/my-kb                # 或指定 worktree 路径

# ── MCP 远程模式 ──
lx sh --space <SPACE_ID>            # 无需本地 worktree

# ── 单次执行模式 ──
lx sh -e "ls /kb"                            # worktree 模式
lx sh --space <SPACE_ID> -e "tree /kb"      # MCP 模式
```

### 虚拟文件系统布局

```
/             → 基础文件系统
├── kb/       → 知识库挂载点（只读）
│   ├── 产品文档/
│   │   ├── API说明.md
│   │   └── 部署指南.md
│   └── 会议纪要/
│       └── 2026-Q1.md
└── tmp/      → 临时存储区（可写）
```

- `/kb` — 知识库内容。Worktree 模式映射到本地磁盘（`.lxworktree/` 和 `.git/` 目录自动隐藏），MCP 模式从远端实时加载
- `/tmp` — 临时可写区域，供 `sort`/`uniq` 等命令中间结果使用
- 默认工作目录为 `/kb`

### 内置命令

#### 核心命令

| 命令 | 说明 | 示例 |
|------|------|------|
| `ls` | 列出目录 | `ls -la /kb` |
| `cat` | 查看文件内容 | `cat /kb/产品文档/API说明.md` |
| `grep` | 搜索文本 | `grep -rni "OAuth" /kb` |
| `find` | 查找文件 | `find /kb -name "*.md" -type f` |
| `tree` | 目录树 | `tree -L 2 /kb` |

#### 辅助命令

| 命令 | 说明 | 示例 |
|------|------|------|
| `head` / `tail` | 查看头部/尾部 | `head -20 /kb/README.md` |
| `wc` | 统计行/词/字符数 | `wc -l /kb/docs/*.md` |
| `sort` / `uniq` | 排序/去重 | `cat log.md \| sort \| uniq -c` |
| `echo` | 输出文本 | `echo $PWD` |
| `pwd` / `cd` | 路径导航 | `cd /kb/产品文档 && pwd` |
| `stat` | 文件信息 | `stat /kb/README.md` |
| `xargs` | 参数传递 | `find /kb -name "*.md" \| xargs grep "API"` |
| `fzf` | 模糊搜索 | `ls /kb \| fzf` |

#### 桥接命令

| 命令 | 说明 | 示例 |
|------|------|------|
| `git` | Git 操作（Worktree 模式） | `git status`、`git log`、`git diff` |
| `search` | 知识库关键词搜索 | `search OAuth 认证` |
| `mcp` | 透传调用 MCP 工具 | `mcp entry_describe '{"entry_id":"xxx"}'` |

> **注意**：`git` 命令仅在 Worktree 模式下可用，且 `git pull`/`push`/`commit` 等写操作需退出 shell 后用 `lx git` 执行。

#### 只读保护

以下命令会提示「只读文件系统」错误，防止误操作：

`rm`、`mv`、`cp`、`mkdir`、`touch`、`chmod`

### 现代工具别名

AI Agent（Claude Code、Cursor 等）倾向使用 `rg`、`eza`、`fd` 等现代工具。虚拟 Shell 自动识别并翻译参数：

| 输入 | 翻译为 | 说明 |
|------|--------|------|
| `rg "pattern" /kb` | `grep -rn "pattern" /kb` | ripgrep → grep |
| `rg --type md "API"` | `grep -rn --include="*.md" "API"` | 类型过滤翻译 |
| `eza -la /kb` | `ls -la /kb` | eza → ls |
| `eza --tree -L 2` | `tree -L 2` | 自动切换 tree |
| `fd readme /kb` | `find /kb -name '*readme*'` | fd → find |
| `fd -e md` | `find . -name '*.md'` | 扩展名搜索 |
| `bat /kb/README.md` | `cat /kb/README.md` | bat → cat |
| `ll` / `la` / `l` | `ls -la` / `ls -a` / `ls` | 常见别名 |

### Shell 特性

支持标准 Bash 语法：

```bash
# 管道
ls /kb | grep "产品" | head -5

# 逻辑运算
cd /kb/docs && cat README.md

# 分号连接
pwd; ls; echo "done"

# 变量
NAME="test"; echo $NAME

# Glob 通配符
cat /kb/*.md

# 重定向（写入 /tmp）
grep -r "API" /kb > /tmp/results.txt
```

### 工作流示例

#### 快速浏览知识库结构

```bash
lx sh -e "tree -L 2 /kb"              # worktree 模式
lx sh -s <SPACE_ID> -e "tree -L 2 /kb"  # MCP 模式
```

#### 搜索知识库内容

```bash
cd my-kb && lx sh
# 进入 REPL
lx:/kb$ grep -rn "部署" /kb | head -10
lx:/kb$ search 部署流程
lx:/kb$ git status                     # 查看本地修改状态
```

#### 管道组合查询

```bash
lx sh -e "find /kb -name '*.md' | xargs grep 'OAuth' | head -20"
```

#### Agent 编程调用

`build_shell()` 返回 `Bash` 实例，Agent runtime 直接调用 `exec()` 即可，无需中转层：

```rust
use lexiang_cli::cmd::sh;

// Worktree 模式（自动检测 cwd）
let mut bash = sh::build_shell(&config, None, None).await?;

// 指定 worktree 路径
let mut bash = sh::build_shell(&config, None, Some("~/my-kb")).await?;

// MCP 远程模式
let mut bash = sh::build_shell(&config, Some("space_xxx"), None).await?;

let out = bash.exec("grep -r OAuth /kb | head -5").await?;
println!("{}", out.stdout);  // 搜索结果
println!("exit: {}", out.exit_code);
```

---

## Git 命令（本地工作区）

`lx git` 提供 Git 风格的命令来管理本地知识库工作区，支持离线编辑、版本控制、批量同步。

### 克隆知识库

```bash
# 克隆整个知识库
lx git clone <space_id> <path>
lx git clone f146992b7ca54bcaa3964458dfb775a7 ./my-kb

# 克隆后自动切换到工作目录
cd ./my-kb
```

### 查看状态

```bash
# 查看工作区状态（类似 git status）
lx git status

# 查看变更差异
lx git diff
lx git diff --remote    # 对比远端

# 查看提交历史
lx git log
lx git log -n 20        # 显示更多
```

### 暂存和提交

```bash
# 暂存文件
lx git add <file>       # 暂存指定文件
lx git add .            # 暂存所有变更

# 提交
lx git commit -m "更新文档"
lx git commit -a -m "msg"   # 自动暂存并提交
```

### 推送和拉取

```bash
# 拉取远端最新内容
lx git pull

# 推送本地变更到远端
lx git push
lx git push --dry-run   # 预览（不实际执行）
lx git push --force     # 强制推送
```

### 版本回退

```bash
# 本地回退到指定版本
lx git reset <commit>
lx git reset --hard <commit>    # 同时重置工作区文件
lx git reset --hard HEAD~1      # 回退到上一个版本

# 回退远端到指定版本（将变更推送到远端）
lx git revert <commit>
lx git revert <commit> --dry-run    # 预览
```

### 远端信息

```bash
# 查看远端信息
lx git remote -v
```

### 工作区管理

```bash
# 列出所有工作区
lx worktree list

# 删除工作区
lx worktree remove <path>
lx worktree remove ./my-kb --yes    # 跳过确认
```

### 支持的文件类型

| 类型 | 拉取 | 推送 | 回退 |
|------|------|------|------|
| 页面 (.md) | ✅ 转换为 Markdown | ✅ 覆盖内容 | ✅ |
| 文件 (PDF/DOCX/...) | ✅ 下载原文件 | ✅ 预签名 URL 上传 | ✅ |
| 文件夹 | ✅ 创建目录 | ✅ 自动创建 | - |

### 工作流示例

#### 日常编辑流程

```bash
cd my-kb
lx git pull                    # 先拉取最新
vim docs/guide.md              # 编辑文档
lx git add .
lx git commit -m "更新指南"
lx git push
```

#### 批量导入文件

```bash
cd my-kb
cp ~/documents/*.pdf ./        # 复制文件到工作区
lx git add .
lx git commit -m "导入 PDF 文档"
lx git push
```

#### 回退到历史版本

```bash
lx git log                     # 查看历史，找到目标 commit
lx git revert abc123 --dry-run # 预览回退操作
lx git revert abc123           # 执行回退（推送到远端）
```

#### 本地重置后重新推送

```bash
lx git reset --hard HEAD~2     # 本地回退 2 个版本
lx git push --force            # 强制推送到远端
```

---

## 工具管理

### 同步最新工具定义

```bash
lx tools sync
```

从 MCP Server 获取最新的工具定义，保存到 `~/.lexiang/tools/override.json`。

### 查看工具分类

```bash
lx tools categories
```

### 查看某分类下的工具

```bash
lx tools list --category team
lx tools list --category entry
```

### 查看 Schema 版本

```bash
lx tools version
```

### 生成 AI Agent Skill 文件

为 AI Agent（如 Claude、GPT）生成工具说明文件：

```bash
# 生成到默认目录 ~/.lexiang/skills/
lx tools skill

# 生成到指定目录
lx tools skill --output ./my-skills
```

生成的文件包括：
- `README.md` - 所有 namespace 概览
- `{namespace}.md` - 每个 namespace 的详细说明和示例

---

## Shell 补全

### Bash

```bash
# 临时启用
eval "$(lx completion bash)"

# 永久启用
lx completion bash >> ~/.bashrc
```

### Zsh

```bash
# 临时启用
eval "$(lx completion zsh)"

# 永久启用
lx completion zsh >> ~/.zshrc
```

### Fish

```bash
lx completion fish > ~/.config/fish/completions/lx.fish
```

---

## 实用技巧

### 批量导出数据

```bash
# 导出为 CSV，用 Excel 打开
lx entry list-children --parent-id <ID> -o csv > entries.csv

# 导出为 JSON，用 jq 处理
lx team list -o json | jq '.data.teams[].name'
```

### 与其他工具配合

```bash
# 搜索并用 fzf 交互选择
lx search kb --keyword "文档" -o json | jq -r '.data.docs[] | "\(.id)\t\(.title)"' | fzf

# 批量获取文档内容
for id in $(cat entry_ids.txt); do
  lx entry describe-ai-parse-content --entry-id "$id" -o json >> all_docs.json
done
```

### 调试模式

```bash
# 查看详细日志
RUST_LOG=debug lx team list

# 查看请求参数
RUST_LOG=trace lx search kb --keyword "test"
```

---

## 配置文件

### 主配置文件

位置：`~/.lexiang/config.json`

```json
{
  "mcp": {
    "url": "https://mcp.lexiang-app.com/mcp",
    "access_token": null
  }
}
```

### Token 文件

位置：`~/.lexiang/auth/token.json`

```json
{
  "access_token": "xxx",
  "refresh_token": "yyy",
  "expires_at": 1234567890
}
```

### Schema 文件

位置：`~/.lexiang/tools/override.json`

通过 `lx tools sync` 生成，优先级高于内置 Schema。

---

## 故障排查

### Token 过期

```bash
lx login  # 重新登录
```

Token 支持自动刷新（如果有 refresh_token）。

### 命令未找到

```bash
lx tools sync  # 同步最新工具定义
```

### 检查 Token 状态

```bash
cat ~/.lexiang/auth/token.json | jq '.expires_at | todate'
```

### 检查 Schema

```bash
cat ~/.lexiang/tools/override.json | jq '.tools | keys | length'
# 53 (当前工具数量)
```

---

## FAQ

**Q: 如何获取 entry-id？**

从 URL 中提取：`https://lexiangla.com/pages/xxx` → `xxx` 就是 entry-id。

**Q: 动态命令和 lexiang 子命令有什么区别？**

动态命令（如 `lx team list`）从 MCP Schema 自动生成，参数使用 `--` 形式；
lexiang 子命令（如 `lx lexiang search xxx`）是静态实现，参数可以是位置参数。
两者功能等价，推荐使用动态命令。

**Q: 如何在脚本中使用？**

使用 `-o json` 输出 JSON，配合 `jq` 处理：

```bash
TEAM_ID=$(lx team list -o json | jq -r '.data.teams[0].id')
lx space list --team-id "$TEAM_ID" -o csv > spaces.csv
```

**Q: 参数太复杂怎么办？**

使用 `-d` 传入 JSON：

```bash
lx block update -d '{
  "block_id": "xxx",
  "content": {
    "text": "复杂内容",
    "style": {"bold": true}
  }
}'
```
