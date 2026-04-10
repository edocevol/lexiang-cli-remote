# worktree — 多工作区管理

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解 Git 工作流的整体决策树。

管理多个独立的本地工作区（类似 `git worktree`）。每个 worktree 绑定一个知识库，支持选择性拉取特定条目。

## 使用场景

### 创建新 worktree

```bash
# 完整克隆知识库
lx worktree add ./work-kb --space-id sp_xxx

# 只拉取特定条目
lx worktree add ./partial-kb --space-id sp_xxx --entry-ids "entry_001,entry_002"
```

### 列出所有 worktree

```bash
lx worktree list           # 表格格式
lx worktree list -f json  # JSON 格式
```

### 删除 worktree

```bash
lx worktree remove ./work-kb      # 交互式确认
lx worktree remove ./work-kb -y   # 跳过确认
```

### 查看 worktree 状态

```bash
lx worktree status
```

### 查看 worktree 差异

```bash
lx worktree diff                    # diff 格式（默认）
lx worktree diff -f json           # JSON 格式
lx worktree diff -f markdown      # Markdown 格式
lx worktree diff --remote         # 与远端快照比较
```

### 在 worktree 中提交

```bash
lx worktree commit -m "更新文档"
lx worktree commit -a -m "更新文档"  # 自动暂存已修改文件
```

### worktree 提交历史

```bash
lx worktree log           # 默认 10 条
lx worktree log -l 20    # 指定条数
```

### 推送 worktree 到远程

```bash
lx worktree push           # 正式推送
lx worktree push --dry-run  # 预览
lx worktree push --force   # 强制推送
```

### 从远程拉取到 worktree

```bash
lx worktree pull
```

### 重置 / 回退 worktree

```bash
lx worktree reset HEAD~1
lx worktree reset HEAD~1 --hard
lx worktree revert abc1234
lx worktree revert abc1234 --dry-run
```

## worktree vs git 的区别

| 方面 | `lx git` | `lx worktree` |
|------|----------|---------------|
| **初始化** | `lx git clone` | `lx worktree add` |
| **工作目录** | 从 cwd 自动检测 `.lxworktree` | 需指定 `<path>` 参数 |
| **选择性拉取** | 不支持，总是全量克隆 | 支持 `--entry-ids` 只拉取部分条目 |
| **diff 格式** | 仅 diff 文本 | 支持 diff / json / markdown |
| **适用场景** | 单知识库日常操作 | 多知识库并行管理 |

## 注意事项

> [!CAUTION]
>
> - `lx worktree push` 和 `lx worktree revert` 会**修改远程知识库内容**
> - `lx worktree remove` 会删除本地目录，但不影响远程内容
> - 选择性拉取（`--entry-ids`）后 push 时只会推送已拉取的条目

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx worktree --help
lx worktree add --help
lx worktree push --help
# ...
```

## 参考

- [lx-git](../SKILL.md) — Git skill 完整决策树
- [git.md](git.md) — 核心 Git 命令
