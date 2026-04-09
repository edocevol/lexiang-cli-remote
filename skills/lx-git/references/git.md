# git — Git 风格知识库版本控制

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解 Git 工作流的整体决策树。

用 Git 风格命令管理知识库的本地工作区：克隆、暂存、提交、推送、拉取、回退。所有命令需要在 worktree 目录（含 `.lxworktree`）内执行。

## 使用场景

### 克隆知识库到本地

```bash
lx git clone sp_xxx ./my-kb

# 使用完整 URL（自动提取 space_id）
lx git clone "https://lexiangla.com/spaces/sp_xxx" ./my-kb
```

### 查看工作树状态

```bash
lx git status
```

### 暂存 + 提交

```bash
lx git add .
lx git commit -m "更新项目计划"

# 一步完成：自动暂存已修改文件后提交
lx git commit -a -m "更新项目计划"
```

### 查看差异

```bash
lx git diff
```

### 查看提交历史

```bash
lx git log          # 默认 10 条
lx git log -n 5     # 指定条数
```

### 拉取远程最新内容

```bash
lx git pull
```

### 推送本地变更到远程

```bash
lx git push           # 正式推送
lx git push --dry-run # 预览，不实际执行
lx git push --force   # 强制推送（覆盖远程冲突）
```

### 重置本地 HEAD

```bash
lx git reset HEAD~1          # 软重置
lx git reset HEAD~1 --hard   # 硬重置（同时重置工作目录）
```

### 回退远程到指定 commit

```bash
lx git revert abc1234          # 正式回退
lx git revert abc1234 --dry-run  # 预览效果
```

### 查看远程信息

```bash
lx git remote
lx git remote -v   # 显示 URL
```

## 注意事项

> [!CAUTION]
> - `lx git push` 和 `lx git revert` 会**修改远程知识库内容**，操作前确认用户意图
> - `--force` 跳过安全检查，仅在用户明确要求时使用
> - `lx git revert` 会直接在远端创建/删除/更新页面，**不可自动撤销**
> - 文件类型决定远程操作方式：`.md` = 页面，其他 = 附件
> - 有未提交变更时 push 会报错，必须先 `lx git add` + `lx git commit`
> - `--remote` diff 功能暂未实现

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx git --help
lx git clone --help
lx git push --help
# ...
```

## 参考

- [lx-git](../SKILL.md) — Git skill 完整决策树
- [worktree.md](worktree.md) — 多工作区管理
- [lx-sh](../../lx-sh/SKILL.md) — 用虚拟 Shell 浏览知识库
