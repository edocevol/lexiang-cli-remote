# block basic — 原子 Block 操作

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解 block 操作的整体决策树。
> **优先使用 [高级命令](block-advanced.md)**。仅在需要精确控制单个块（如修改特定块的样式、移动块位置、操作非标准块类型）时才使用原子操作。

直接操作知识库页面中的文档块（block）：查询、创建、更新、删除、移动。每个命令对应一个 MCP Tool 原子操作。

## 使用场景

### 查看页面块结构

```bash
lx block list-block-children --entry-id entry_xxx --with-descendants
# → 返回完整块树，包含每个块的 ID、类型、内容
```

### 获取单个块详情

```bash
lx block describe-block --entry-id entry_xxx --block-id blk_xxx
```

### 将 Markdown 转为块结构，再插入页面

这是向页面插入内容的标准两步流程：

**Step 1 — 转换内容**
```bash
lx block convert-content-to-blocks --content "## 新章节\n\n段落内容" --content-type markdown
# → 返回 descendant 结构（JSON）
```

**Step 2 — 插入到页面**
```bash
lx block create-block-descendant \
  --entry-id entry_xxx \
  --parent-block-id parent_xxx \
  --descendant '<上一步返回的 JSON 结构>'
```

> 不要手动构造 `descendant` 结构，始终先通过 `lx block convert-content-to-blocks` 转换。

### 更新块文本

```bash
lx block update-block \
  --entry-id entry_xxx \
  --block-id blk_xxx \
  --update-text '{"elements": [{"text": {"content": "新内容"}}]}'
```

### 批量更新多个块

```bash
lx block update-blocks \
  --entry-id entry_xxx \
  --updates '{"blk_1": {"UpdateText": {"elements": [...]}}, "blk_2": {"SetTaskDone": true}}'
```

> 每个块在单次请求中只能执行一种更新操作。如需同时更新文本和样式，用 `UpdateText`。单次最多更新 20 个块。

### 移动块到新位置

```bash
lx block move-blocks \
  --entry-id entry_xxx \
  --block-ids blk_xxx \
  --parent-block-id new_parent_xxx
```

### 删除块

```bash
lx block delete-block --entry-id entry_xxx --block-id blk_xxx
# ⚠️ 会级联删除所有子孙节点
```

### 批量删除子块

```bash
lx block delete-block-children \
  --entry-id entry_xxx \
  --parent-block-id parent_xxx \
  --ids blk_1 --ids blk_2
```

## 关键规则

1. **高级命令优先**：如果能用 `lx block replace-section` / `table-set` / `append` 等高级命令完成，不要用原子操作。
2. **转换后创建**：要向页面插入 Markdown/HTML 内容时，先 `lx block convert-content-to-blocks` 转为块结构，再 `lx block create-block-descendant` 插入。不要手动构造块结构。
3. **批量更新**：需要同时修改多个块时用 `lx block update-blocks`，比逐个调用 `lx block update-block` 更高效。
4. **大文档分批**：`lx block create-block-descendant` 内容较大时（如长文档或大量块），**必须分批调用**，每批控制在合理大小。
5. **移动限制**：`lx block move-blocks` 的目标父节点**不能是叶子节点类型**，包括：h1-h5（标题）、code（代码）、image（图片）、attachment（附件）、video（视频）、divider（分割线）、mermaid、plantuml。
6. **删除级联**：`lx block delete-block` 会同时删除所有子孙节点，操作不可逆。`lx block delete-block-children` 只删除指定的直接子块。

## ⚠️ 副作用与风险

- 所有写入操作（create、update、delete、move）执行前确认用户意图
- `lx block create-block-descendant` 大量内容时必须分批，单次请求过大会超时
- `lx block move-blocks` 所有块只能移动到同一个目标父节点，不支持分别移到不同父节点
- 手动构造块结构容易出错，优先用 `lx block convert-content-to-blocks` 从 Markdown/HTML 转换
- 暂不支持视频（VOD）上传，video block 请使用 `file_id` 字段

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx block --help
lx block create-block-descendant --help
lx block update-blocks --help
# ...
```

## 参考

- [lx-block](../SKILL.md) — block 操作完整决策树
- [block-advanced.md](block-advanced.md) — 高级命令（优先使用）
