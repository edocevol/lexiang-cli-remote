---
name: lx-block
description: |
  乐享文档块编辑。支持表格操作、章节替换、内容插入追加、导入导出、块树浏览和原子块操作。当用户需要结构化编辑知识库页面时使用。触发词：编辑文档、修改内容、表格、块、插入、追加、替换章节
---

# lx-block

**当以下情况时使用此 Skill**:

- 需要修改页面中的表格内容
- 需要替换某个章节的内容
- 需要在页面中插入或追加内容
- 需要导入 Markdown 到页面
- 需要查看或操作块树结构

## 工具一：表格操作

读取、修改、增删表格行。

### 参数（表格操作）

- **block-id** (string, required): 表格块 ID
- **row** (number): 行号（table-set/table-del-row）
- **col** (number): 列号（table-set）
- **text** (string): 单元格文本（table-set）

### 示例（读取表格）

```json
lx-block: { "tool": "table-get", "block-id": "tbl_xxx" }
```

### 示例（修改单元格）

```json
lx-block: { "tool": "table-set", "block-id": "tbl_xxx", "row": 2, "col": 1, "text": "修正值" }
```

## 工具二：章节替换

按标题查找并替换整个章节内容。

### 参数（章节替换）

- **block-id** (string, required): 页面根块 ID
- **heading** (string, required): 目标章节标题（如 "## API 参考"）
- **file** (string): 替换内容文件路径

### 示例（章节替换）

```json
lx-block: { "tool": "replace-section", "block-id": "page_xxx", "heading": "## API 参考", "file": "./updated-api.md" }
```

## 工具三：内容插入与追加

在指定块后插入内容，或在页面末尾追加内容。

### 参数（插入/追加）

- **block-id** (string, required): 目标页面块 ID
- **after-block-id** (string): 在此块后插入（insert-after）
- **file** (string): 内容文件路径（append/insert-after）
- **content** (string): 直接传入内容（append/insert-after）

### 示例（追加内容）

```json
lx-block: { "tool": "append", "block-id": "page_xxx", "file": "./new-content.md" }
```

## 工具四：导入导出

导入 Markdown 文件到页面，或导出页面内容为 Markdown/JSON。

### 参数（导入）

- **block-id** (string, required): 目标页面块 ID
- **file** (string, required): Markdown 文件路径
- **chunk-size** (number): 分批大小，默认 20

### 示例（导入）

```json
lx-block: { "tool": "import", "block-id": "page_xxx", "file": "./doc.md", "chunk-size": 20 }
```

## 工具五：块树浏览

查看页面的块树结构。

### 参数（块树）

- **block-id** (string, required): 页面根块 ID
- **recursive** (boolean): 是否递归显示子块

### 示例（块树）

```json
lx-block: { "tool": "tree", "block-id": "page_xxx", "recursive": true }
```

## 工具六：原子块操作

精细控制单个块：读取、创建、更新、删除、移动块。

### 参数（原子操作）

- **entry-id** (string): 条目 ID
- **block-id** (string): 块 ID
- **parent-block-id** (string): 父块 ID
- **descendant** (string): 块结构 JSON
- **update-text** (string): 更新文本 JSON

### 示例（更新块）

```json
lx-block: { "tool": "update-block", "entry-id": "entry_xxx", "block-id": "blk_xxx", "update-text": "{\"elements\": [{\"text\": {\"content\": \"新内容\"}}]}" }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 修改表格 | 工具一：表格操作 |
| 替换整个章节 | 工具二：章节替换 |
| 在页面中插入内容 | 工具三：insert-after |
| 在末尾追加内容 | 工具三：append |
| 导入 Markdown | 工具四：import |
| 查看页面结构 | 工具五：tree |
| 精细控制单个块 | 工具六：原子块操作 |
