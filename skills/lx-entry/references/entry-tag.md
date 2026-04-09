# entry tag — 标签管理

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解条目管理的整体决策树。

为知识库条目添加、删除、查看标签。标签是条目级别的分类标记，用于组织和检索。

## 使用场景

### 查看条目现有标签

```bash
lx knowledge-tag list-entry-tags --entry-id entry_xxx
```

### 为条目添加标签

```bash
lx knowledge-tag set-entry-tags --entry-id entry_xxx --add-tags "重要" --add-tags "Q1计划"
```

### 删除标签

```bash
lx knowledge-tag set-entry-tags --entry-id entry_xxx --del-tags "过时"
```

### 同时添加和删除

```bash
lx knowledge-tag set-entry-tags --entry-id entry_xxx \
  --add-tags "Q2计划" \
  --del-tags "Q1计划"
```

## 关键规则

1. **先查后改**：修改标签前先 `lx knowledge-tag list-entry-tags` 查看现有标签，避免重复添加或删除不存在的标签。
2. **原子操作**：`set-entry-tags` 支持同时添加和删除，一次调用完成。
3. **标签名即标识**：标签通过**名称**（而非 ID）进行增删。标签名大小写敏感。
4. **`--add-tags` 和 `--del-tags` 至少传一个**。

## ⚠️ 注意事项

- 标签名大小写敏感，`"重要"` 和 `"重要"` 会被视为不同标签
- 删除不存在的标签不会报错，操作前先 list 确认

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx knowledge-tag --help
```

## 参考

- [lx-entry](../SKILL.md) — 条目 skill 完整决策树
- [entry-crud.md](entry-crud.md) — 获取 entry_id
