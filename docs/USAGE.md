# lx - 乐享知识库命令行工具

`lx` 是乐享知识库的命令行工具，支持两类典型工作流：

- **在线操作**：直接调用 MCP Tool，完成搜索、查询、创建、导入等操作
- **本地工作区操作**：把知识库克隆到本地，离线编辑、版本化管理、批量同步

本文档按以下 5 个大章节组织：

- **1、基础 TOOL**：最常用的业务 Tool 和通用参数
- **2、动态加载 TOOL**：动态命令的发现、同步、查看与生成说明文件
- **3、JUST-BASH 能力**：`lx sh` 提供的知识库虚拟 Shell
- **4、git 版本化知识库管理**：`lx git` 与 `lx worktree` 的本地工作流
- **5、其他**：补全、配置、实用技巧、故障排查与 FAQ

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
```

---

## 快速开始

### 1. 登录

```bash
lx login
```

浏览器会自动打开 OAuth 登录页面。授权完成后，Token 会保存在 `~/.lexiang/auth/token.json`。

### 2. 看看现在有哪些命令

```bash
lx --help
lx team --help
lx search kb --help
```

### 3. 先跑几个最常见的命令

```bash
# 列出我可访问的团队
lx team list

# 查看某个团队下的知识库
lx space list --team-id <TEAM_ID>

# 全局搜索知识
lx search kb --keyword "项目文档"
```

---

## 1. 基础 TOOL

这一章聚焦“直接干活”的 Tool：搜索、团队、知识库、条目、文档块、文件、评论、PPT、会议、联系人等。

### 全局参数

大多数动态命令都支持以下通用参数：

| 参数                    | 说明                                      |
|-------------------------|-------------------------------------------|
| `-o, --format <FORMAT>` | 输出格式：json/json-pretty/table/yaml/csv |
| `-d, --data-raw <JSON>` | 用 JSON 一次性传入全部参数                |
| `-h, --help`            | 查看帮助                                  |

### 两种传参方式

```bash
# 方式 1：逐参数传入（适合日常使用）
lx search kb --keyword "test" --limit 10 --type doc

# 方式 2：JSON 一次性传入（适合脚本或复杂参数）
lx search kb -d '{"keyword":"test","limit":10,"type":"doc"}'
```

### 输出格式示例

```bash
lx team list -o json
lx team list -o json-pretty
lx team list -o table
lx team list -o yaml
lx team list -o csv
lx team list -o markdown
```

### 常用命令分组总览

| Namespace | 用途         | 常用命令                  |
|-----------|--------------|---------------------------|
| `search`  | 搜索知识     | `lx search kb`            |
| `team`    | 团队信息     | `lx team list`            |
| `space`   | 知识库信息   | `lx space list`           |
| `entry`   | 条目与页面   | `lx entry describe`       |
| `block`   | 在线文档块   | `lx block list-children`  |
| `file`    | 文件上传下载 | `lx file describe`        |
| `comment` | 评论查询     | `lx comment list`         |
| `ppt`     | PPT 服务     | `lx ppt generate`         |
| `meeting` | 会议录制     | `lx meeting search`       |
| `contact` | 联系人       | `lx contact search-staff` |
| `iwiki`   | iWiki 导入   | `lx iwiki import`         |

### 搜索 TOOL

```bash
# 全局搜索
lx search kb --keyword "关键词"

# 指定知识库搜索
lx search kb --keyword "关键词" --space-id <SPACE_ID>

# 指定搜索类型
lx search kb --keyword "关键词" --type doc
lx search kb --keyword "关键词" --type kb_doc
lx search kb --keyword "关键词" --title-only

# 限制结果数量
lx search kb --keyword "关键词" --limit 5

# 向量语义检索
lx search kb-embedding --keyword "如何部署服务"
```

### 团队与知识库 TOOL

```bash
# 团队
lx team list
lx team list-frequent
lx team describe --team-id <TEAM_ID>

# 知识库
lx space list --team-id <TEAM_ID>
lx space list-recently
lx space describe --space-id <SPACE_ID>
```

`lx space describe` 的返回结果里通常包含 `root_entry_id`，后续遍历目录时会用到。

### 条目 TOOL

```bash
# 查看条目详情
lx entry describe --entry-id <ENTRY_ID>

# 查看子条目（遍历目录）
lx entry list-children --parent-id <PARENT_ID>

# 查看最近更新条目
lx entry list-latest --space-id <SPACE_ID>

# 查看面包屑路径
lx entry list-parents --entry-id <ENTRY_ID>

# 创建页面
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

# 重命名与移动
lx entry rename --entry-id <ENTRY_ID> --name "新名称"
lx entry move --entry-id <ENTRY_ID> --parent-entry-id <NEW_PARENT_ID>

# 获取 AI 可解析内容（Markdown / HTML）
lx entry describe-ai-parse-content --entry-id <ENTRY_ID>
```

### 文档块 TOOL

当你需要细粒度修改在线文档内容时，可以直接用 `block` 相关命令。

```bash
# 获取块详情
lx block describe --block-id <BLOCK_ID>

# 获取块的子节点
lx block list-children --block-id <BLOCK_ID>
lx block list-children --block-id <BLOCK_ID> --recursive

# 更新块内容
lx block update -d '{"block_id":"xxx","content":{"text":"新内容"}}'

# 批量更新块
lx block update-blocks -d '{"blocks":[...]}'

# 创建子块
lx block create-descendant -d '{"block_id":"xxx","descendant":{...}}'

# 删除块
lx block delete --block-id <BLOCK_ID>

# 批量删除子块
lx block delete-children -d '{"block_id":"xxx","children_ids":["id1","id2"]}'

# 移动块
lx block move -d '{"block_ids":["id1","id2"],"parent_block_id":"xxx"}'

# 把内容转换为块结构
lx block convert-content-to-blocks -d '{"content":"# 标题","content_type":"markdown"}'
```

### 文件 TOOL

```bash
# 获取文件详情
lx file describe --file-id <FILE_ID>

# 获取下载链接
lx file download --file-id <FILE_ID>
```

上传文件通常分 3 步：

```bash
# 第 1 步：申请上传凭证
lx file apply-upload --parent-entry-id <PARENT_ID>

# 第 2 步：使用返回的 upload_url 上传文件
curl -X PUT "<upload_url>" --data-binary @file.pdf

# 第 3 步：确认上传
lx file commit-upload --session-id <SESSION_ID>
```

导入外部链接：

```bash
lx file create-hyperlink \
  --url "https://mp.weixin.qq.com/s/xxx" \
  --space-id <SPACE_ID> \
  --parent-entry-id <PARENT_ID>
```

### 其他业务 TOOL

```bash
# 评论
lx comment list --target-id <ENTRY_ID>
lx comment describe --target-id <ENTRY_ID>

# PPT
lx ppt generate -d '{
  "planning": "10页，标题：产品介绍，风格：商务",
  "context": "产品功能特性说明..."
}'
lx ppt get-task --id <TASK_ID>
lx ppt add-pages -d '{"title":"产品介绍","pages":[{"insert_after":2,"title":"新页面","key_points":"要点内容"}]}'
lx ppt modify-pages -d '{"title":"产品介绍","pages":[{"page_index":3,"modification":"将标题改为..."}]}'
lx ppt delete-pages --title "产品介绍" -d '{"page_indexes":[1,2]}'
lx ppt reorder-pages -d '{"title":"产品介绍","new_order":[3,1,2]}'

# 会议
lx meeting search --meeting-id <MEETING_CODE>
lx meeting describe --record-id <RECORD_ID>
lx meeting import -d '{
  "record_id":"xxx",
  "parent_entry_id":"yyy",
  "start_time":"2026-01-01T10:00:00",
  "end_time":"2026-01-01T11:00:00"
}'
lx meeting reload --record-id <RECORD_ID>

# 联系人
lx contact search-staff --keyword "张三"
lx contact whoami

# iWiki
lx iwiki import -d '{
  "page_id":"12345",
  "space_id":"xxx",
  "parent_entry_id":"yyy"
}'
```

### 低层调试 TOOL：`lx mcp`

如果你想绕过动态命令，直接查看或调用 MCP Tool，可以使用 `lx mcp`：

```bash
# 列出可用工具
lx mcp list

# 直接调用某个工具
lx mcp call entry_describe --params '{"entry_id":"xxx"}'
```

---

## 2. 动态加载 TOOL

`lx` 的一个核心能力是：**命令不是全部写死在代码里，而是可以从 MCP Schema 动态加载。**

这意味着 MCP 新增 Tool 后，CLI 可以通过同步最新 Schema 直接获得新命令，而不必等下一次发版。

### 动态命令是什么

典型的动态命令如下：

```bash
lx team list
lx space describe --space-id <SPACE_ID>
lx entry create --parent-entry-id <PARENT_ID> --name "新文档" --entry-type page
```

这些命令来自 MCP Schema 中的 namespace 与 Tool 定义。

### 什么时候需要同步

通常情况下，`lx` 已内置一份可用 Schema，**安装后就能直接使用**。

只有在以下情况，建议执行同步：

- MCP Server 新增了 Tool
- 某个分类下的命令没有出现
- 你希望马上拿到最新参数定义

```bash
lx tools sync
```

同步后的 Schema 会保存到 `~/.lexiang/tools/override.json`。

### 常用动态 TOOL 管理命令

```bash
# 同步最新工具定义
lx tools sync

# 查看有哪些分类
lx tools categories

# 查看某个分类下的工具
lx tools list --category team
lx tools list --category entry

# 查看 Schema 版本信息
lx tools version
```

### 如何发现新命令

```bash
# 查看所有顶层命令
lx --help

# 查看某个 namespace 下有哪些子命令
lx team --help
lx entry --help

# 查看具体命令支持哪些参数
lx team list --help
lx search kb --help
```

推荐的顺序是：

1. 先用 `lx --help` 看有哪些 namespace
2. 再用 `<namespace> --help` 看有哪些 action
3. 最后用具体命令的 `--help` 看参数

### 生成 AI Agent Skill 文件

如果你希望把当前 Tool 能力导出成面向 AI Agent 的说明文件，可以用：

```bash
# 输出到默认目录 ~/.lexiang/skills/
lx tools skill

# 输出到指定目录
lx tools skill --output ./my-skills
```

生成结果通常包括：

- `README.md`：所有 namespace 的总览
- `{namespace}.md`：每个 namespace 的详细说明与示例

### 与配置相关的 Schema 文件

| 文件                             | 说明                    |
|----------------------------------|-------------------------|
| `~/.lexiang/tools/override.json` | 运行时同步得到的 Schema |
| 内置 Schema                      | 安装时自带，开箱即用    |

### 常见问题

**Q：明明服务端有新 Tool，为什么本地没有？**

先执行：

```bash
lx tools sync
```

如果同步后仍然没有，再检查当前账号权限和分类名称是否正确。

**Q：命令没记住怎么办？**

直接看帮助，不要硬背：

```bash
lx --help
lx <namespace> --help
lx <namespace> <command> --help
```

---

## 3. JUST-BASH 能力

`lx sh` 提供一个面向知识库的虚拟 Shell。你可以直接用熟悉的 Bash 风格命令浏览和搜索知识库内容，而不需要记各种 API 参数。

### 两种运行模式

- **Worktree 模式**：在本地 worktree 目录下运行，`/kb` 映射到本地知识库文件
- **MCP 远程模式**：通过 `--space` 直接连接远端知识库，不依赖本地 worktree

### 启动方式

```bash
# Worktree 模式（推荐）
lx git clone <SPACE_ID> ./my-kb
cd my-kb
lx sh

# 指定 worktree 路径
lx sh --path ~/my-kb

# MCP 远程模式
lx sh --space <SPACE_ID>

# 单次执行并退出
lx sh -e "ls /kb"
lx sh --space <SPACE_ID> -e "tree -L 2 /kb"
```

### 虚拟文件系统布局

```text
/
├── kb/        # 知识库挂载点（只读）
└── tmp/       # 临时可写区域
```

说明：

- `/kb`：知识库内容
  - Worktree 模式下映射到本地磁盘
  - MCP 模式下实时从远端加载
- `/tmp`：临时可写区域，适合给 `sort`、`uniq`、重定向等命令存中间结果
- 默认工作目录是 `/kb`

### 内置命令

#### 核心命令

| 命令   | 说明         | 示例                    |
|--------|--------------|-------------------------|
| `ls`   | 列出目录     | `ls -la /kb`            |
| `cat`  | 查看文件内容 | `cat /kb/README.md`     |
| `grep` | 搜索文本     | `grep -rni "OAuth" /kb` |
| `find` | 查找文件     | `find /kb -name "*.md"` |
| `tree` | 目录树       | `tree -L 2 /kb`         |

#### 辅助命令

| 命令            | 说明                   | 示例                                        |
|-----------------|------------------------|---------------------------------------------|
| `head` / `tail` | 查看头部/尾部          | `head -20 /kb/README.md`                    |
| `wc`            | 统计行数/单词数/字符数 | `wc -l /kb/docs/*.md`                       |
| `sort` / `uniq` | 排序/去重              | `cat log.md \| sort \| uniq -c`             |
| `echo`          | 输出文本               | `echo $PWD`                                 |
| `pwd` / `cd`    | 路径导航               | `cd /kb/docs && pwd`                        |
| `stat`          | 查看文件信息           | `stat /kb/README.md`                        |
| `xargs`         | 参数传递               | `find /kb -name "*.md" \| xargs grep "API"` |
| `fzf`           | 模糊筛选               | `ls /kb \| fzf`                             |

#### 桥接命令

| 命令     | 说明                    | 示例                    |
|----------|-------------------------|-------------------------|
| `git`    | Git 操作（仅 Worktree） | `git status`、`git log` |
| `search` | 知识库关键词搜索        | `search OAuth 认证`     |
| `mcp`    | 透传调用 MCP Tool       | `mcp entry_describe`    |

> **注意**：`git` 只在 Worktree 模式下可用，`pull` / `push` / `commit` 这类写操作建议退出 Shell 后用 `lx git` 执行。

### 只读保护

以下命令会返回“只读文件系统”错误，防止误操作：

`rm`、`mv`、`cp`、`mkdir`、`touch`、`chmod`

### 现代 CLI 兼容

很多 AI Agent 或现代命令行用户更习惯 `rg`、`eza`、`fd`、`bat` 这类工具。`lx sh` 会自动做一层兼容翻译。

| 输入                 | 实际转换                          | 说明           |
|----------------------|-----------------------------------|----------------|
| `rg "p" /kb`         | `grep -rn "p" /kb`                | `rg` → `grep`  |
| `rg --type md "API"` | `grep -rn --include="*.md" "API"` | 类型过滤       |
| `eza -la /kb`        | `ls -la /kb`                      | `eza` → `ls`   |
| `eza --tree -L 2`    | `tree -L 2`                       | 自动切换目录树 |
| `fd readme /kb`      | `find /kb -name '*readme*'`       | `fd` → `find`  |
| `fd -e md`           | `find . -name '*.md'`             | 扩展名搜索     |
| `bat /kb/README.md`  | `cat /kb/README.md`               | `bat` → `cat`  |
| `ll` / `la` / `l`    | `ls -la` / `ls -a` / `ls`         | 常见别名       |

### 支持的 Bash 风格语法

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

### 典型工作流

#### 快速浏览知识库结构

```bash
lx sh -e "tree -L 2 /kb"
lx sh --space <SPACE_ID> -e "tree -L 2 /kb"
```

#### 搜索知识库内容

```bash
cd my-kb
lx sh

# 进入交互模式后
lx:/kb$ grep -rn "部署" /kb | head -10
lx:/kb$ search 部署流程
lx:/kb$ git status
```

#### 组合查询

```bash
lx sh -e "find /kb -name '*.md' | xargs grep 'OAuth' | head -20"
```

### Agent / 程序内调用

如果你要在 Rust 代码里复用这套 Shell 能力，可以直接构造 `Bash` 实例并调用 `exec()`：

```rust
use lexiang_cli::cmd::sh;

// Worktree 模式（自动检测 cwd）
let mut bash = sh::build_shell(&config, None, None).await?;

// 指定 worktree 路径
let mut bash = sh::build_shell(&config, None, Some("~/my-kb")).await?;

// MCP 远程模式
let mut bash = sh::build_shell(&config, Some("space_xxx"), None).await?;

let out = bash.exec("grep -r OAuth /kb | head -5").await?;
println!("{}", out.stdout);
println!("exit: {}", out.exit_code);
```

---

## 4. git 版本化知识库管理

`lx git` 提供类似 Git 的本地知识库工作流：先把知识库克隆到本地，再用本地提交记录管理变更，最后统一推送到远端。

### 典型场景

- **离线编辑**：本地修改 Markdown 页面
- **批量导入**：一次性导入大量 PDF / DOCX / 其他文件
- **版本追踪**：查看历史、比较差异、回退误操作
- **多人协作前检查**：推送前先对比远端变更

### 基本流程

```bash
# 克隆知识库到本地
lx git clone <SPACE_ID> ./my-kb
cd my-kb

# 编辑文件
vim 产品文档/需求说明.md

# 查看状态
lx git status

# 提交本地变更
lx git add .
lx git commit -m "更新需求说明"

# 推送到远端
lx git push
```

### 查看状态与差异

```bash
lx git status
lx git diff
lx git diff --remote
lx git log
lx git log -n 20
```

说明：

- `lx git diff`：查看本地工作区与当前提交的差异
- `lx git diff --remote`：对比本地与远端快照，适合推送前确认是否有冲突风险
- `lx git log`：查看本地提交历史

### 提交与同步

```bash
# 暂存指定文件或全部变更
lx git add docs/guide.md
lx git add .

# 提交
lx git commit -m "更新文档"
lx git commit -a -m "批量修正"

# 拉取远端最新内容
lx git pull

# 推送本地变更
lx git push
lx git push --dry-run
lx git push --force
```

### 回退版本

```bash
# 本地回退
lx git reset <COMMIT>
lx git reset --hard <COMMIT>
lx git reset --hard HEAD~1

# 远端回退（生成逆向变更并推送）
lx git revert <COMMIT>
lx git revert <COMMIT> --dry-run
```

一般建议：

- 想撤回本地未同步历史，用 `reset`
- 想把远端知识库回退到某个版本，用 `revert`

### 查看远端信息

```bash
lx git remote -v
```

### Worktree 管理

如果你需要显式管理本地工作区，可以使用 `lx worktree`：

```bash
# 低层方式创建工作区
lx worktree add ./my-kb --space-id <SPACE_ID>

# 列出全部工作区
lx worktree list

# 删除工作区
lx worktree remove ./my-kb
lx worktree remove ./my-kb --yes
```

### 支持的文件类型

| 类型                  | 拉取            | 推送         | 回退 |
|-----------------------|-----------------|--------------|------|
| 页面（`.md`）         | ✅ 转为 Markdown | ✅ 覆盖内容   | ✅    |
| 文件（PDF/DOCX/XLSX） | ✅ 下载原文件    | ✅ 预签名上传 | ✅    |
| 文件夹                | ✅ 创建目录结构  | ✅ 自动创建   | -    |

### 典型工作流（Git）

#### 日常编辑流程

```bash
cd my-kb
lx git pull
vim docs/guide.md
lx git add .
lx git commit -m "更新指南"
lx git push
```

#### 批量导入文件

```bash
cd my-kb
cp ~/documents/*.pdf ./
lx git add .
lx git commit -m "导入 PDF 文档"
lx git push
```

#### 回退到历史版本

```bash
lx git log
lx git revert abc123 --dry-run
lx git revert abc123
```

#### 本地重置后重新推送

```bash
lx git reset --hard HEAD~2
lx git push --force
```

---

## 5. 其他

### Shell 补全

#### Bash

```bash
# 临时启用
eval "$(lx completion bash)"

# 永久启用
lx completion bash >> ~/.bashrc
```

#### Zsh

```bash
# 临时启用
eval "$(lx completion zsh)"

# 永久启用
lx completion zsh >> ~/.zshrc
```

#### Fish

```bash
lx completion fish > ~/.config/fish/completions/lx.fish
```

其他 Shell 可通过 `lx completion --help` 查看。

### 版本与更新

```bash
# 查看版本
lx version

# 检查新版本
lx update check
lx update check --prerelease

# 查看最近发布记录
lx update list
lx update list --limit 10
```

### 配置文件

#### 主配置文件

位置：`~/.lexiang/config.json`

```json
{
  "mcp": {
    "url": "https://mcp.lexiang-app.com/mcp",
    "access_token": null
  }
}
```

#### Token 文件

位置：`~/.lexiang/auth/token.json`

```json
{
  "access_token": "xxx",
  "refresh_token": "yyy",
  "expires_at": 1234567890
}
```

如果存在 `refresh_token`，CLI 会自动刷新 Token。

#### Schema 文件

位置：`~/.lexiang/tools/override.json`

这个文件由 `lx tools sync` 生成，优先级高于内置 Schema。

### 实用技巧

#### 批量导出数据

```bash
# 导出为 CSV，方便用 Excel 打开
lx entry list-children --parent-id <ID> -o csv > entries.csv

# 导出为 JSON，再交给 jq 处理
lx team list -o json | jq '.data.teams[].name'
```

#### 与其他命令行工具组合

```bash
# 搜索后交给 fzf 选择
lx search kb --keyword "文档" -o json | jq -r '.data.docs[] | "\(.id)\t\(.title)"' | fzf

# 批量获取文档内容
for id in $(cat entry_ids.txt); do
  lx entry describe-ai-parse-content --entry-id "$id" -o json >> all_docs.json
done
```

#### 调试日志

```bash
# 查看调试日志
RUST_LOG=debug lx team list

# 查看更详细的请求日志
RUST_LOG=trace lx search kb --keyword "test"
```

### 故障排查

#### Token 过期

```bash
lx login
```

#### 命令未出现或分类为空

```bash
lx tools sync
```

#### 检查 Token 状态

```bash
cat ~/.lexiang/auth/token.json | jq '.expires_at | todate'
```

#### 检查当前 Schema 是否已加载

```bash
cat ~/.lexiang/tools/override.json | jq '.tools | keys | length'
```

### FAQ

**Q：如何获取 `entry-id`？**

通常可以直接从页面 URL 中提取，例如：`https://lexiangla.com/pages/xxx` 中的 `xxx`。

**Q：参数太复杂怎么办？**

直接改用 JSON 传参：

```bash
lx block update -d '{
  "block_id": "xxx",
  "content": {
    "text": "复杂内容",
    "style": {"bold": true}
  }
}'
```

**Q：脚本里怎么用最稳妥？**

优先使用 `-o json`，再交给 `jq` 处理：

```bash
TEAM_ID=$(lx team list -o json | jq -r '.data.teams[0].id')
lx space list --team-id "$TEAM_ID" -o csv > spaces.csv
```

**Q：我只想快速浏览知识库，不想记 API 参数怎么办？**

直接用 `lx sh`。

**Q：我想离线编辑、批量导入、做版本回退怎么办？**

直接用 `lx git`。
