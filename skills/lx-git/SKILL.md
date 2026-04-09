---
name: lx-git
version: 1.0.0
description: "Git 风格的知识库本地工作流。支持克隆知识库到本地目录，用 git-like 命令管理版本（add/commit/push/pull/diff/log/reset/revert），以及多 worktree 管理。触发词：克隆、clone、push、pull、同步、推送、拉取、版本、提交、commit、worktree、工作区"
metadata:
  requires:
    bins: ["lx"]
---

# Git 风格知识库工作流

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**用户说"克隆知识库到本地"/"推送本地修改"/"回退版本"/"查看本地改了什么"** → 用本 skill

**用户说"编辑某个页面内容"** → 用 lx-entry 或 lx-block skill
**用户说"浏览知识库内容（只读）"** → 用 lx-sh skill（虚拟 Shell）

## ⚡ 怎么选命令？（快速决策树）

```
用户想 →
├── 把远程知识库拉到本地编辑? → lx git clone
├── 查看本地改了什么? → lx git status / lx git diff
├── 暂存 + 提交本地修改? → lx git add + lx git commit
├── 推送本地修改到远程? → lx git push
├── 拉取远程最新内容? → lx git pull
├── 查看提交历史? → lx git log
├── 回退本地版本? → lx git reset
├── 回退远程版本? → lx git revert
├── 查看远程信息? → lx git remote
├── 管理多个本地工作区? → lx worktree add/list/remove
└── 用 Shell 浏览知识库? → 切换到 lx-sh skill
```

## 可用工具

### lx git — 核心版本控制

| 命令 | 说明 |
|------|------|
| `lx git clone` | 克隆远程知识库到本地 |
| `lx git add` | 暂存文件 |
| `lx git commit` | 提交本地变更 |
| `lx git status` | 查看工作树状态 |
| `lx git diff` | 查看变更差异 |
| `lx git log` | 查看提交历史 |
| `lx git pull` | 从远程拉取最新内容 |
| `lx git push` | 推送本地变更到远程 |
| `lx git reset` | 重置本地 HEAD |
| `lx git revert` | 回退远程到指定提交 |
| `lx git remote` | 查看远程仓库信息 |

详细参数：`lx git --help`

### lx worktree — 多工作区管理

| 命令 | 说明 |
|------|------|
| `lx worktree add` | 创建新 worktree |
| `lx worktree list` | 列出所有 worktree |
| `lx worktree remove` | 删除 worktree |
| `lx worktree status` | 查看 worktree 状态 |
| `lx worktree diff` | 查看 worktree 差异 |
| `lx worktree commit` | 提交 worktree 变更 |
| `lx worktree log` | 查看 worktree 提交历史 |
| `lx worktree push` | 推送 worktree 到远程 |
| `lx worktree pull` | 拉取远程到 worktree |
| `lx worktree reset` | 重置 worktree HEAD |
| `lx worktree revert` | 回退 worktree 远程版本 |

详细参数：`lx worktree --help`

## 🎯 执行规则

1. **必须先 clone**：所有 `lx git` 命令需要在 worktree 目录（包含 `.lxworktree`）内执行。如果用户还没有本地工作区，先引导 `lx git clone`。
2. **space_id 支持 URL**：`lx git clone` 的 `<space_id>` 可以传完整的知识库 URL，CLI 会自动提取 space_id。
3. **push 前先 commit**：有未提交的变更时 `lx git push` 会报错，必须先 `lx git add . && lx git commit -m "..."`。
4. **dry-run 先预览**：推送和回退操作建议先加 `--dry-run` 预览，确认无误后再正式执行。
5. **force push 需确认**：`--force` 会覆盖远程冲突，必须确认用户意图。
6. **revert 回退远程**：`lx git revert` 是将远程内容回退到指定 commit 的状态，会直接修改远程——这是危险操作。
7. **Markdown 文件 = 页面**：`.md` 文件被视为知识库页面，其他文件按附件处理。push 时自动根据文件类型选择创建方式。
8. **worktree vs git**：`lx worktree` 用于管理多个独立的本地工作区（类似 `git worktree`），适合同时操作多个知识库。`lx git` 在当前 worktree 内操作。

## 典型组合流程

### 首次克隆并编辑

```bash
# 克隆知识库到本地
lx git clone sp_xxx ./my-kb

# 进入目录
cd my-kb

# 查看本地文件结构
ls -la

# 编辑 .md 文件...（用编辑器修改）

# 查看改了什么
lx git status
lx git diff

# 暂存 + 提交 + 推送
lx git add .
lx git commit -m "更新了项目计划"
lx git push
```

### 拉取远程更新

```bash
cd my-kb
lx git pull
# → 自动拉取最新内容并创建 commit
```

### 推送前预览

```bash
lx git push --dry-run
# → 显示哪些文件会被创建/更新，不实际执行

# 确认无误后正式推送
lx git push
```

### 回退远程版本

```bash
# 查看历史
lx git log

# 先预览回退效果
lx git revert abc1234 --dry-run

# 确认后执行
lx git revert abc1234
```
