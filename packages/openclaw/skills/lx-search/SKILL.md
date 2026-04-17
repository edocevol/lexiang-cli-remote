---
name: lx-search
description: |
  乐享知识库搜索与人员查询。支持关键词搜索、语义向量搜索和企业员工信息查询。触发词：搜索、找一下、有没有关于、查找、search、查人、谁是
---

# lx-search

**当以下情况时使用此 Skill**:

- 需要搜索知识库文档
- 需要查找特定内容或关键词
- 需要查询企业同事信息
- 需要确认当前登录身份

## 工具一：关键词精确搜索

根据关键词搜索知识库文档，支持标题搜索和限定知识库范围。

### 参数（关键词搜索）

- **keyword** (string, required): 搜索关键词
- **space-id** (string): 知识库 ID，限定搜索范围
- **title-only** (boolean): 是否仅搜索标题
- **type** (string): 文档类型，如 `kb_doc`

### 示例（关键词搜索）

```json
lx-search: { "tool": "kb-search", "keyword": "API 设计" }
```

### 示例（限定知识库）

```json
lx-search: { "tool": "kb-search", "keyword": "API 设计", "space-id": "sp_xxx" }
```

### 示例（仅搜索标题）

```json
lx-search: { "tool": "kb-search", "keyword": "API 设计", "title-only": true }
```

## 工具二：语义向量搜索

根据自然语言描述进行语义搜索，适合模糊意图查询。

### 参数（语义搜索）

- **keyword** (string, required): 自然语言描述
- **space-id** (string): 知识库 ID，限定搜索范围

### 示例（语义搜索）

```json
lx-search: { "tool": "kb-embedding-search", "keyword": "如何配置数据库连接池" }
```

## 工具三：企业人员查询

搜索企业员工信息，支持模糊搜索。

### 参数（人员搜索）

- **staff-id** (string, required): 员工姓名或工号
- **fuzzy-search** (boolean): 是否启用模糊搜索

### 示例（人员搜索）

```json
lx-search: { "tool": "search-staff", "staff-id": "张三", "fuzzy-search": true }
```

## 工具四：获取当前用户身份

查看当前登录的用户信息。

### 示例（获取身份）

```json
lx-search: { "tool": "whoami" }
```

## 工具五：获取文档内容

根据 entry_id 获取文档的 AI 可解析内容。

### 参数（获取内容）

- **entry-id** (string, required): 文档条目 ID

### 示例（获取内容）

```json
lx-search: { "tool": "describe-ai-parse-content", "entry-id": "entry_xxx" }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 明确关键词（产品名、文档标题） | 工具一：kb-search |
| 模糊意图（自然语言提问） | 工具二：kb-embedding-search |
| 查找同事信息 | 工具三：search-staff |
| 确认登录身份 | 工具四：whoami |
| 获取文档详细内容 | 工具五：describe-ai-parse-content |

## 执行规则

1. **关键词 vs 语义搜索**：用户给出明确关键词（如产品名、文档标题）时用 `kb-search`；用户描述模糊意图（如"怎么申请资源"）时用 `kb-embedding-search`。
2. **限定搜索范围**：如果用户指定了知识库，先通过 `list-recently-spaces` 或 `describe-space` 拿到 `space_id`，再传入搜索命令。
3. **只搜标题**：用户说"标题是…"时，使用 `kb-search` 并设置 `title-only: true`。
4. **搜索结果后续处理**：搜索返回 `entry_id` 后，如需获取内容，调用 `describe-ai-parse-content`。
5. **人员搜索限制**：`search-staff` 依赖企业通讯录可见设置，接口报错时应告知用户可能是权限限制。
6. **分页**：默认只返回第一页。用户明确要求"更多结果"时才传 `--page-token` 翻页。

## 典型组合流程

### 在指定知识库中搜索

```json
// 先获取最近知识库列表
lx-space: { "tool": "list-recently-spaces" }

// 在指定知识库中搜索
lx-search: { "tool": "kb-search", "keyword": "API 设计", "space-id": "sp_xxx" }
```

### 语义搜索 + 获取文档内容

```json
// 语义搜索，获取匹配的 entry_id
lx-search: { "tool": "kb-embedding-search", "keyword": "如何配置数据库连接池" }

// 获取文档 AI 可解析内容
lx-search: { "tool": "describe-ai-parse-content", "entry-id": "entry_xxx" }
```

### 查找同事的文档

```json
// 查找人员信息
lx-search: { "tool": "search-staff", "staff-id": "张三", "fuzzy-search": true }

// 按创建人搜索文档
lx-search: { "tool": "kb-search", "keyword": "张三", "type": "kb_doc" }
```

## 禁止操作

- **不要在搜索后直接编辑**：搜索返回 `entry_id` 后，如需编辑 → **立即切换到 lx-entry 或 lx-block skill**
- **不要在本 skill 中创建页面**：用户说"创建一个页面" → **立即切换到 lx-entry skill**

搜索只是定位工具，搜到 `entry_id` 后必须继续进入对应 skill 执行后续操作。
