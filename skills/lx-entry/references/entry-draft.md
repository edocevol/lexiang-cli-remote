# entry draft — Markdown 草稿管理

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解条目管理的整体决策树。

Markdown 格式页面的草稿系统。支持创建草稿、保存修改、发布为正式版本。草稿使用**乐观锁**（`seq` 序列号）防止并发冲突。

## 使用场景

### 查看页面是否有未发布的草稿

```bash
lx draft describe-markdown-draft --entry-id entry_xxx
# → 返回草稿内容和版本信息；空响应表示无草稿
```

### 保存草稿

```bash
lx draft save-markdown-draft \
  --entry-id entry_xxx \
  --revision-id rev_xxx \
  --content "修改后的 Markdown 内容" \
  --seq 0
# seq: 首次创建传 0，更新时传当前 seq
```

### 发布草稿为正式版本

```bash
lx draft publish-markdown-draft --entry-id entry_xxx --revision-id rev_xxx
```

### 发布时覆盖草稿内容

```bash
lx draft publish-markdown-draft \
  --entry-id entry_xxx \
  --revision-id rev_xxx \
  --content "最终内容" \
  --force-publish
```

### 放弃草稿

```bash
lx draft delete-markdown-draft --entry-id entry_xxx
# ⚠️ 未发布的修改不可恢复
```

## 完整草稿编辑流程

```bash
# Step 1: 检查草稿
lx draft describe-markdown-draft --entry-id entry_xxx
# → 有草稿？基于 content 修改
# → 无草稿？先用 lx entry describe-ai-parse-content 获取正式版内容

# Step 2: 保存草稿
lx draft save-markdown-draft \
  --entry-id entry_xxx \
  --revision-id rev_xxx \
  --content "修改后内容" \
  --seq 0

# Step 3: 发布
lx draft publish-markdown-draft --entry-id entry_xxx --revision-id rev_xxx
```

## 关键规则

1. **编辑前先查草稿**：调用 `lx draft describe-markdown-draft` 确认是否已有未发布草稿。如果有，基于草稿内容修改；如果没有，需要先通过 `lx entry describe-ai-parse-content` 获取正式版内容。
2. **乐观锁机制**：`--seq` 是版本序列号——首次创建草稿传 `0`，后续更新传当前 `seq`。如果 `seq` 不匹配说明有并发修改，需要重新获取最新版本。
3. **发布 vs 保存**：`lx draft save-markdown-draft` 只保存草稿（不对外生效）；`lx draft publish-markdown-draft` 将草稿发布为正式版本（用户可见）。
4. **force-publish**：设为 `true` 可跳过版本冲突检测，适用于确定要覆盖的场景。正常情况不建议使用。
5. **草稿是临时的**：`lx draft delete-markdown-draft` 会丢弃所有未发布修改，不可恢复。

## ⚠️ 副作用与风险

- 发布是**写入操作**，会替换页面的正式内容。
- `seq` 不匹配会导致版本冲突，需要重新获取最新版本后再操作。
- 删除草稿后未发布的修改**不可恢复**。

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx draft --help
```

## 参考

- [lx-entry](../SKILL.md) — 条目 skill 完整决策树
- [entry-crud.md](entry-crud.md) — 获取条目详情和 AI 内容
