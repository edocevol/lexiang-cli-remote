# Git 版本化知识库管理

`lx git` 提供类似 Git 的本地知识库工作流：克隆到本地 → 编辑 → 提交 → 推送。

## 适用场景

- **离线编辑**：本地修改 Markdown 页面
- **批量导入**：一次性导入大量 PDF/DOCX
- **版本追踪**：查看历史、比较差异、回退误操作
- **推送前检查**：对比远端变更，避免冲突

## 基本流程

```bash
# 克隆
lx git clone <SPACE_ID> ./my-kb
cd my-kb

# 编辑
vim 产品文档/需求说明.md

# 提交并推送
lx git add .
lx git commit -m "更新需求说明"
lx git push
```

## 状态与差异

```bash
lx git status              # 工作区状态
lx git diff                # 本地 vs 当前提交
lx git diff --remote       # 本地 vs 远端快照
lx git log                 # 提交历史
lx git log -n 20           # 最近 N 条
```

## 提交与同步

```bash
# 暂存
lx git docs/guide.md       # 单文件
lx git add .               # 全部

# 提交
lx git commit -m "更新文档"
lx git commit -a -m "批量修正"

# 远程
lx git pull                # 拉取
lx git push                # 推送
lx git push --dry-run      # 试运行
lx git push --force        # 强制推送
```

## 回退

```bash
# 本地回退
lx git reset <COMMIT>
lx git reset --hard <COMMIT>
lx git reset --hard HEAD~1

# 远端回退（生成逆向变更）
lx git revert <COMMIT>
lx git revert <COMMIT> --dry-run
```

建议：撤回本地未同步历史用 `reset`，回退远端用 `revert`。

## Worktree 管理

```bash
lx worktree add ./my-kb --space-id <SPACE_ID>   # 创建
lx worktree list                                 # 列表
lx worktree remove ./my-kb                       # 删除
lx worktree remove ./my-kb --yes                 # 强制删除
```

## 支持的文件类型

| 类型 | 拉取 | 推送 | 回退 |
|------|------|------|------|
| 页面 `.md` | 转 Markdown | 覆盖内容 | ✅ |
| 文件 PDF/DOCX/XLSX | 下载原文件 | 预签名上传 | ✅ |
| 文件夹 | 创建目录结构 | 自动创建 | - |

## 典型工作流

### 日常编辑

```bash
cd my-kb && lx git pull
vim docs/guide.md
lx git add . && lx git commit -m "更新指南" && lx git push
```

### 批量导入

```bash
cd my-kb && cp ~/documents/*.pdf ./
lx git add . && lx git commit -m "导入 PDF" && lx git push
```

### 回退示例

```bash
lx git log
lx git revert abc123 --dry-run   # 先预览
lx git revert abc123             # 确认执行
```
