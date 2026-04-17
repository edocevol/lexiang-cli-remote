---
name: lx-git
description: |
  Git 风格的知识库本地工作流。支持克隆知识库到本地、版本管理（add/commit/push/pull/diff/log/reset/revert）和多 worktree 管理。触发词：克隆、clone、push、pull、同步、推送、拉取、版本、提交、commit、worktree、工作区
---

# lx-git

**当以下情况时使用此 Skill**:

- 需要将知识库克隆到本地进行编辑
- 需要提交、推送或拉取变更
- 需要查看历史记录或回退版本
- 需要管理多个本地工作区

## 工具一：克隆与状态

克隆远程知识库到本地，查看工作树状态。

### 参数（克隆）

- **space-id** (string, required): 知识库 ID 或完整 URL
- **directory** (string, required): 本地目录路径

### 示例（克隆）

```json
lx-git: { "tool": "clone", "space-id": "sp_xxx", "directory": "./my-kb" }
```

### 示例（状态）

在 worktree 目录中执行：

```bash
lx git status
lx git diff
```

## 工具二：提交与推送

暂存文件、提交本地变更、推送到远程。

### 示例（提交并推送）

在 worktree 目录中执行：

```bash
lx git add .
lx git commit -m "更新了项目计划"
lx git push
```

### 参数（dry-run 预览）

- **--dry-run** (flag): 推送前预览，不实际执行

### 示例（推送预览）

```bash
lx git push --dry-run
```

## 工具三：拉取与历史

从远程拉取最新内容，查看提交历史。

### 示例（拉取）

```bash
lx git pull
```

### 示例（查看历史）

```bash
lx git log
```

## 工具四：回退

重置本地 HEAD，或回退远程到指定提交（危险操作）。

### 参数（回退）

- **commit** (string): 目标提交 ID
- **--dry-run** (flag): 预览回退效果

### 示例（回退远程）

```bash
lx git revert abc1234 --dry-run
lx git revert abc1234
```

## 工具五：多工作区管理

创建、列出、删除多个 worktree。

### 示例（worktree 操作）

```bash
lx worktree add ./wt2 sp_xxx
lx worktree list
lx worktree remove ./wt2
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 首次使用，克隆知识库 | 工具一：clone |
| 查看本地改了什么 | 工具一：status / diff |
| 提交并推送修改 | 工具二：add + commit + push |
| 拉取远程更新 | 工具三：pull |
| 查看历史 | 工具三：log |
| 回退版本 | 工具四：reset / revert |
| 同时操作多个知识库 | 工具五：worktree |
