---
name: lx-ppt
description: |
  乐享 AI PPT 生成与编辑。基于服务端 AI 能力，从文字描述直接生成专业 PPT，无需本地模板或 python-pptx。支持生成、修改页面内容、增删页面、调整顺序。触发词：PPT、幻灯片、演示文稿、slide、deck、制作PPT、生成PPT
---

# lx-ppt

**当以下情况时使用此 Skill**:

- 需要从零生成 PPT
- 需要修改已有 PPT 的页面内容
- 需要在 PPT 中添加新页面
- 需要删除 PPT 页面
- 需要调整 PPT 页面顺序

## 工具一：生成 PPT

从文字描述生成完整 PPT，支持异步生成。

### 参数（生成 PPT）

- **planning** (string, required): PPT 规划，如页数、主题、风格
- **context** (string, required): 上下文内容
- **deep-research-report-url** (string): 深度研究报告 URL（可选，优先使用）

### 示例（生成 PPT）

```json
lx-ppt: { "tool": "generate-ppt", "planning": "10页，主题：Q2业绩汇报，风格：商务简约", "context": "Q2 营收 1.5 亿..." }
```

## 工具二：查询任务状态

查询生成任务状态，轮询直到完成。

### 参数（查询任务）

- **id** (string, required): 任务 ID

### 示例（查询任务）

```json
lx-ppt: { "tool": "get-ppt-task", "id": "task_xxx" }
```

## 工具三：修改页面内容

修改指定页面的内容，使用自然语言描述修改意图。

### 参数（修改页面）

- **title** (string, required): PPT 标题（从 get-ppt-task 获取）
- **pages** (array, required): 页面修改列表

### 示例（修改页面）

```json
lx-ppt: { "tool": "modify-ppt-pages", "title": "Q2业绩汇报", "pages": [{"page_index": 3, "modification": "数据图表换成柱状图"}] }
```

## 工具四：添加新页面

在指定位置添加新页面。

### 参数（添加页面）

- **title** (string, required): PPT 标题
- **pages** (array, required): 页面添加列表

### 示例（添加页面）

```json
lx-ppt: { "tool": "add-ppt-pages", "title": "Q2业绩汇报", "pages": [{"insert_after": 5, "title": "风险分析", "key_points": "...", "slide_type": "content"}] }
```

## 工具五：删除页面

删除指定页面。

### 参数（删除页面）

- **title** (string, required): PPT 标题
- **page-indexes** (array, required): 页面索引列表（从 1 开始）

### 示例（删除页面）

```json
lx-ppt: { "tool": "delete-ppt-pages", "title": "Q2业绩汇报", "page-indexes": [2] }
```

## 工具六：调整页面顺序

调整 PPT 页面的顺序。

### 参数（调整顺序）

- **title** (string, required): PPT 标题
- **new-order** (array, required): 新的页面顺序（从 1 开始）

### 示例（调整顺序）

```json
lx-ppt: { "tool": "reorder-ppt-pages", "title": "Q2业绩汇报", "new-order": [1, 3, 4, 2] }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 从零生成 PPT | 工具一：generate-ppt → 工具二：get-ppt-task（轮询） |
| 修改页面内容 | 工具三：modify-ppt-pages |
| 添加新页面 | 工具四：add-ppt-pages |
| 删除页面 | 工具五：delete-ppt-pages |
| 调整页面顺序 | 工具六：reorder-ppt-pages |

## 执行规则

1. **生成是异步的**：`generate-ppt` 返回任务 ID，必须轮询 `get-ppt-task` 直到 `status` 为完成，才能拿到 `title` 和 `preview_url`。
2. **页面索引从 1 开始**：`modify-ppt-pages`、`delete-ppt-pages`、`reorder-ppt-pages` 中的页面索引都从 **1** 开始，不是 0。
3. **修改用 title 标识 PPT**：所有编辑操作都通过 `--title` 参数关联目标 PPT，必须使用 `get-ppt-task` 返回的精确标题。
4. **modification 使用自然语言**：`modify-ppt-pages` 的 `modification` 字段直接用中文描述修改意图即可（如"把标题改为 Q2 总结"），无需指定坐标或样式参数。
5. **slide_type 三选一**：`add-ppt-pages` 的 `slide_type` 仅支持 `cover`（封面）、`content`（内容页）、`ending`（结束页）。
6. **有深度研究报告优先用**：如果执行过 `deep_research` 且有 `report_url`，应通过 `--deep-research-report-url` 传入，生成质量显著优于纯 `--context`。

## 禁止操作

- **不要创建知识库页面**：用户说"在知识库里创建页面" → **立即切换到 lx-entry skill**
- **不要搜索文档**：用户说"搜索 PPT 相关文档" → **立即切换到 lx-search skill**
- **不要处理本地 `.pptx` 精修**：用户明确要求编辑本地文件、母版、备注、逐元素排版 → **立即切换到通用 `pptx` skill**

## 典型组合流程

### 从零生成一套 PPT

```json
// 生成
lx-ppt: { "tool": "generate-ppt", "planning": "10页，主题：Q2业绩汇报，风格：商务简约", "context": "Q2 营收 1.5 亿..." }

// 轮询任务状态
lx-ppt: { "tool": "get-ppt-task", "id": "task_xxx" }
// → status="completed" 后拿到 title + preview_url

// 根据用户反馈微调
lx-ppt: { "tool": "modify-ppt-pages", "title": "Q2业绩汇报", "pages": [{"page_index": 3, "modification": "数据图表换成柱状图"}] }
```

### 在已有 PPT 上增删调整

```json
// 添加新页面
lx-ppt: { "tool": "add-ppt-pages", "title": "Q2业绩汇报", "pages": [{"insert_after": 5, "title": "风险分析", "key_points": "...", "slide_type": "content"}] }

// 删掉第 2 页
lx-ppt: { "tool": "delete-ppt-pages", "title": "Q2业绩汇报", "page-indexes": [2] }

// 调整顺序
lx-ppt: { "tool": "reorder-ppt-pages", "title": "Q2业绩汇报", "new-order": [1, 3, 4, 2] }
```
