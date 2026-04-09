# comment — 评论管理

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解连接器 skill 的整体决策树。

查看知识库页面的评论。支持列出评论和获取评论详情。目前仅支持页面（`kb_entry`）类型的评论。

## 使用场景

### 查看页面有哪些评论

```bash
lx comment list-comments --target-type kb_entry --target-id entry_xxx
# → 返回评论列表（id, content, created_at, creator）
```

### 获取特定评论的详细内容

```bash
lx comment describe-comment --comment-id comment_xxx
```

### 先获取页面信息，再查看评论

```bash
# 确认是 page 类型
lx entry describe-entry --entry-id entry_xxx

# 获取评论列表
lx comment list-comments --target-type kb_entry --target-id entry_xxx
```

## 关键规则

1. **target_type 固定为 `kb_entry`**：当前只支持页面评论。其他类型暂不支持。
2. **target_id 就是 entry_id**：传页面的 entry_id。
3. **content 格式特殊**：返回的 `content` 不是普通 HTML 或纯文本，是富文本结构，需要解析后再展示给用户。
4. **只读操作**：当前只提供查看评论的能力，不支持创建/删除评论。

## ⚠️ 注意事项

- `describe-comment` 返回的 `content` 是特殊富文本格式，不是普通 HTML，直接展示原始内容可能不可读
- 评论是页面维度的，一个页面下可能有多个评论

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx comment --help
```

## 参考

- [lx-connector](../SKILL.md) — 连接器 skill 完整决策树
- [lx-entry](../../lx-entry/SKILL.md) — 获取页面 entry_id
