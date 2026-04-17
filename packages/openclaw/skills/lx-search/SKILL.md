---
name: lx-search
description: |
  在乐享知识库中搜索内容，支持关键词搜索、语义向量搜索和企业员工信息查询。触发词：搜索、找一下、有没有关于、查找、search、查人、谁是
---

# lx-search

**当以下情况时使用此 Skill**:

- 需要在乐享中搜索文档或内容
- 需要按关键词查找相关文档
- 需要模糊查询或语义匹配
- 需要查询同事信息
- 用户提到"搜索"、"找一下"、"有没有关于..."

## 工具一：lx-search（关键词搜索）

精确匹配关键词，适合已知明确关键词的场景。

### 参数（关键词搜索）

- **keyword** (string, required): 搜索关键词
- **type** (string): 搜索类型：`all` / `doc` / `space` / `folder` / `file` / `page` 等
- **space-id** (string): 限定知识库
- **team-id** (string): 限定团队
- **sort-by** (string): 排序：`created_at` / `-created_at` / `edited_at` / `-edited_at`
- **title-only** (boolean): 只搜索标题
- **limit** (number): 结果数量
- **page-token** (string): 翻页 token

### 示例（关键词搜索）

```json
lx-search: { "keyword": "项目计划" }
```

```json
lx-search: { "keyword": "API 设计", "space-id": "sp_xxx", "title-only": true }
```

## 工具二：lx-embedding-search（语义搜索）

基于向量的语义相似度匹配，适用于模糊查询和自然语言提问。

### 参数（语义搜索）

- **keyword** (string, required): 搜索语句（自然语言）
- **space-id** (string): 限定知识库
- **team-id** (string): 限定团队
- **parent-id** (string): 限定父节点
- **limit** (number): 结果数量

### 示例（语义搜索）

```json
lx-search: { "tool": "embedding-search", "keyword": "如何申请服务器资源" }
```

## 工具三：人员查询

查询企业员工信息，确认当前登录身份。

### 参数（人员搜索）

- **staff-id** (string, required): 员工 ID 或姓名
- **fuzzy-search** (boolean): 模糊搜索

### 示例（查人）

```json
lx-search: { "tool": "search-staff", "staff-id": "张三", "fuzzy-search": true }
```

### 示例（当前身份）

```json
lx-search: { "tool": "whoami" }
```

## 搜索结果后续处理

搜索返回 `entry_id` 后，如需获取内容：

```json
lx-entry: { "tool": "describe-ai-parse-content", "entry-id": "entry_xxx" }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 精确查找已知关键词 | 工具一：lx-search |
| 模糊查询、语义匹配 | 工具二：lx-embedding-search |
| 查找同事信息 | 工具三：search-staff |
| 确认当前登录身份 | 工具三：whoami |
