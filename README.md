# lx - 乐享知识库命令行工具

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](Cargo.toml)

`lx` 是一个强大的命令行工具，让你可以在终端中高效管理乐享知识库。支持在线操作和本地工作区两种模式，提供离线编辑、批量同步、版本控制等能力。

## ✨ 核心特性

- 🚀 **即装即用** - 内置 50+ 个命令，覆盖团队、知识库、文档、搜索等全部功能
- 📁 **本地知识库管理** - 克隆知识库到本地，离线编辑、批量操作、版本回退
- 🔧 **动态命令系统** - 自动从 MCP Schema 生成命令，新功能一键同步
- 📦 **多格式输出** - 支持 JSON、YAML、CSV、Markdown、表格等 6 种格式
- 💡 **灵活参数输入** - 支持命令行参数和 JSON 两种方式
- 🎨 **Shell 补全** - 支持 Bash、Zsh、Fish、PowerShell 自动补全

## 📦 安装

### 从源码安装

确保已安装 Rust 1.70 或更高版本：

```bash
# 克隆仓库
git clone https://github.com/your-org/lexiang-cli.git
cd lexiang-cli

# 安装
cargo install --force --path .

# 验证安装
lx version
```

### 前置要求

- Rust 1.70+
- 支持 macOS、Linux、Windows

## 🚀 快速开始

### 1. 登录授权

```bash
lx login
```

浏览器会自动打开 OAuth 登录页面，授权后 token 将安全保存在本地。

### 2. 探索命令

```bash
# 查看所有命令
lx --help

# 更新 tool 
lx tools sync

# 查看团队命令
lx team --help

# 查看具体命令参数
lx search kb --help
```

### 3. 开始使用

```bash
# 列出我的团队
lx team list

# 列出团队下的知识库
lx space list --team-id <TEAM_ID>

# 搜索知识
lx search kb --keyword "项目文档"

# 以表格格式输出
lx team list -o table
```

## 💻 使用示例

### 本地知识库管理

`lx git` 提供类似 Git 的本地工作流，让知识管理更加高效：

**核心场景：**
- 📝 **离线编辑** - 在本地编辑知识文档，无需实时联网
- 🔄 **批量同步** - 批量导入文件、批量修改文档后一次性推送
- ⏪ **版本回退** - 将知识库回退到历史版本
- 📂 **多知识库管理** - 本地同时管理多个知识库工作区

**工作原理：**
本地知识库会创建 `.lxworktree` 目录，存储文档与远端的映射关系和本地提交历史。所有变更先提交到本地仓库，再推送到乐享知识库。

```bash
# 克隆知识库到本地
lx git clone <space_id> ./my-kb
cd my-kb

# 此时本地是完整的文档树
# - 文件夹对应知识库目录
# - .md 文件对应知识库页面
# - PDF/DOCX 等文件对应知识库文件

# 离线编辑文档
vim 产品文档/需求说明.md
vim 技术方案/架构设计.md

# 批量导入本地文件
cp ~/documents/*.pdf ./参考资料/
cp ~/reports/*.docx ./项目报告/

# 查看变更状态
lx git status

# 提交到本地仓库
lx git add .
lx git commit -m "批量导入项目文档"

# 推送到乐享知识库
lx git push
# ✓ 自动创建新页面
# ✓ 自动上传文件
# ✓ 自动更新已有内容
```

**高级用法：**

```bash
# 查看变更详情
lx git diff                # 本地变更
lx git diff --remote       # 对比远端（检查是否有人修改）

# 拉取最新内容
lx git pull                # 同步远端最新变更

# 版本回退
lx git log                 # 查看历史版本
lx git revert <commit>     # 回退远端到指定版本（推送后生效）
lx git reset --hard HEAD~1 # 本地回退（不影响远端）

# 管理多个工作区
lx worktree list           # 列出所有本地知识库
lx worktree remove <path>  # 删除工作区
```

**支持的文件类型：**

| 类型 | 本地文件 | 拉取 | 推送 | 版本回退 |
|------|---------|------|------|---------|
| 页面 | `.md` | ✅ 转为 Markdown | ✅ 覆盖内容 | ✅ |
| 文件 | PDF/DOCX/XLSX 等 | ✅ 下载原文件 | ✅ 预签名上传 | ✅ |
| 文件夹 | 目录 | ✅ 创建目录结构 | ✅ 自动创建 | - |

**典型工作流：**

1. **知识库初始化**：`lx git clone` 克隆知识库到本地
2. **日常编辑**：用熟悉的编辑器（Vim/VS Code）编辑 `.md` 文件
3. **批量导入**：复制文件到工作区，一次推送完成所有上传
4. **版本管理**：查看历史、回退误操作
5. **多知识库协作**：在不同目录管理多个知识库

### 多格式输出

```bash
# JSON（紧凑）
lx team list -o json

# JSON（格式化，默认）
lx team list -o json-pretty

# 表格
lx team list -o table

# YAML
lx team list -o yaml

# CSV（方便 Excel 处理）
lx team list -o csv > teams.csv

# Markdown 表格
lx team list -o markdown
```

### 文档操作

```bash
# 获取文档详情
lx entry describe --entry-id <ENTRY_ID>

# 列出子条目
lx entry list-children --parent-id <PARENT_ID>

# 创建新文档
lx entry create --parent-entry-id <PARENT_ID> --name "新文档" --entry-type page

# 获取 AI 可解析内容（返回 markdown）
lx entry describe-ai-parse-content --entry-id <ENTRY_ID>

# 导入内容
lx entry import-content \
  --parent-id <PARENT_ID> \
  --name "导入的文档" \
  --content "# 标题\n\n正文内容"
```

### 文件上传

```bash
# 申请上传凭证
lx file apply-upload --parent-entry-id <PARENT_ID>

# 使用返回的 upload_url 上传文件
curl -X PUT "<upload_url>" --data-binary @file.pdf

# 确认上传
lx file commit-upload --session-id <SESSION_ID>
```

### 与其他工具配合

```bash
# 搜索并用 fzf 交互选择
lx search kb --keyword "文档" -o json | \
  jq -r '.data.docs[] | "\(.id)\t\(.title)"' | fzf

# 批量获取文档内容
for id in $(cat entry_ids.txt); do
  lx entry describe-ai-parse-content --entry-id "$id" -o json >> all_docs.json
done

# 导出为 CSV，用 Excel 打开
lx entry list-children --parent-id <ID> -o csv > entries.csv
```

## 🔧 动态命令系统

CLI 自动从 MCP Schema 生成命令，新功能上线后只需：

```bash
# 同步最新工具定义
lx tools sync

# 查看工具分类
lx tools categories

# 查看某分类下的工具
lx tools list --category team
```

所有命令都有完整的帮助信息：

```bash
lx --help              # 查看所有命令
lx team --help         # 查看团队相关命令
lx team list --help    # 查看具体命令的参数
```

## 🎨 Shell 补全

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

## ⚙️ 配置

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

Token 支持自动刷新。

### Schema 文件

位置：`~/.lexiang/tools/override.json`

通过 `lx tools sync` 生成，优先级高于内置 Schema。

## 📚 支持的命令

### 动态命令（从 MCP Schema 生成）

- `team` - 团队接口（3 个命令）
- `space` - 知识库接口（3 个命令）
- `entry` - 知识增删改查接口（10 个命令）
- `block` - 在线文档接口（10 个命令）
- `file` - 知识文件接口（5 个命令）
- `search` - 搜索接口（2 个命令）
- `ppt` - PPT 服务相关接口（6 个命令）
- `meeting` - 腾讯会议（5 个命令）
- `comment` - 知识评论查询接口（2 个命令）
- `contact` - 联系人接口（2 个命令）
- `iwiki` - iWiki 接口（1 个命令）

### 本地知识库管理

- `lx git clone` - 克隆知识库到本地工作区
- `lx git status` - 查看本地变更状态
- `lx git add` - 暂存文件变更
- `lx git commit` - 提交到本地仓库
- `lx git push` - 推送本地变更到乐享知识库
- `lx git pull` - 拉取乐享知识库最新内容
- `lx git log` - 查看本地提交历史
- `lx git diff` - 查看本地变更详情
- `lx git diff --remote` - 对比本地与远端差异
- `lx git reset` - 本地版本回退
- `lx git revert` - 回退乐享知识库到历史版本
- `lx git remote` - 查看远端信息

### 工作区管理

- `lx worktree list` - 列出所有工作区
- `lx worktree remove` - 删除工作区

### 其他命令

- `lx login` - OAuth 登录
- `lx logout` - 登出
- `lx tools sync` - 同步工具定义
- `lx tools categories` - 查看工具分类
- `lx tools list` - 列出工具
- `lx tools skill` - 生成 AI Agent Skill 文件
- `lx completion` - 生成 Shell 补全脚本
- `lx version` - 显示版本

## 🐛 故障排查

### Token 过期

```bash
lx login  # 重新登录
```

### 命令未找到

```bash
lx tools sync  # 同步最新工具定义
```

### 调试模式

```bash
# 查看详细日志
RUST_LOG=debug lx team list

# 查看请求参数
RUST_LOG=trace lx search kb --keyword "test"
```

## 📖 文档

详细文档请参阅：

- [使用指南](docs/USAGE.md) - 完整的使用文档
- [API 文档](schemas/) - MCP Schema 定义

## 🤝 贡献

欢迎贡献代码、报告问题或提出建议！

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/your-org/lexiang-cli.git
cd lexiang-cli

# 构建项目
cargo build

# 运行测试
cargo test

# 代码检查
cargo clippy

# 格式化代码
cargo fmt
```

### 提交代码

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📝 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- [clap](https://github.com/clap-rs/clap) - 强大的命令行参数解析库
- [tokio](https://github.com/tokio-rs/tokio) - 异步运行时
- [gix](https://github.com/Byron/gitoxide) - Git 实现
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP 客户端

---

**[⬆ 返回顶部](#lx---乐享知识库命令行工具)**
