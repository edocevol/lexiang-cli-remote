---
name: lx-ppt
description: |
  乐享 AI PPT 生成与编辑。基于服务端 AI 能力，从文字描述直接生成专业 PPT，支持生成、修改页面内容、增删页面、调整顺序。触发词：PPT、幻灯片、演示文稿、slide、deck、制作PPT、生成PPT
---

# lx-ppt

**当以下情况时使用此 Skill**:

- 需要从零生成一套 PPT
- 需要修改已有 PPT 的页面内容
- 需要增加或删除 PPT 页面
- 需要调整页面顺序

## 工具一：生成 PPT

从文字描述异步生成完整 PPT。

### 参数（生成）

- **planning** (string, required): 规划描述（页数、主题、风格）
- **context** (string): 内容上下文
- **deep-research-report-url** (string): 深度研究报告 URL（可选，提升质量）

### 示例（生成）

```json
lx-ppt: { "tool": "generate-ppt", "planning": "10页，主题：Q2业绩汇报，风格：商务简约", "context": "Q2 营收 1.5 亿..." }
```

> 生成返回任务 ID，需轮询 `get-ppt-task` 直到 `status` 为 `completed`，拿到 `title` 和 `preview_url`。

## 工具二：修改页面

用自然语言描述修改意图，修改指定页面内容。

### 参数（修改）

- **title** (string, required): PPT 标题（需用 get-ppt-task 返回的精确标题）
- **pages** (string, required): JSON 数组，含 `page_index` 和 `modification`

### 示例（修改）

```json
lx-ppt: { "tool": "modify-ppt-pages", "title": "Q2业绩汇报", "pages": "[{\"page_index\": 3, \"modification\": \"数据图表换成柱状图\"}]" }
```

## 工具三：增删页面

添加新页面或删除指定页面。

### 参数（添加页面）

- **title** (string, required): PPT 标题
- **pages** (string, required): JSON 数组，含 `insert_after`、`title`、`key_points`、`slide_type`

### 示例（添加页面）

```json
lx-ppt: { "tool": "add-ppt-pages", "title": "Q2业绩汇报", "pages": "[{\"insert_after\": 5, \"title\": \"风险分析\", \"key_points\": \"...\", \"slide_type\": \"content\"}]" }
```

### 示例（删除页面）

```json
lx-ppt: { "tool": "delete-ppt-pages", "title": "Q2业绩汇报", "page-indexes": 2 }
```

## 工具四：调整顺序

调整 PPT 页面顺序。

### 参数（调整顺序）

- **title** (string, required): PPT 标题
- **new-order** (number, repeated): 新的页面顺序（索引从 1 开始）

### 示例（调整顺序）

```json
lx-ppt: { "tool": "reorder-ppt-pages", "title": "Q2业绩汇报", "new-order": [1, 3, 4, 2] }
```

## 注意事项

- 页面索引从 **1** 开始（不是 0）
- `slide_type` 仅支持 `cover`（封面）、`content`（内容页）、`ending`（结束页）
- `modification` 直接用中文描述修改意图，无需指定坐标或样式

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 从零生成 PPT | 工具一：generate-ppt + 轮询 get-ppt-task |
| 修改页面内容 | 工具二：modify-ppt-pages |
| 添加新页面 | 工具三：add-ppt-pages |
| 删除页面 | 工具三：delete-ppt-pages |
| 调整页面顺序 | 工具四：reorder-ppt-pages |
