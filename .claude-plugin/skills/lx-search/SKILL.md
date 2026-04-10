---
name: lx-search
version: 1.0.0
description: "乐享知识库搜索与人员查询。支持关键词搜索、语义向量搜索和企业员工信息查询。当用户需要搜索文档、查找内容或查询同事信息时使用。触发词：搜索、找一下、有没有关于、查找、search、查人、谁是"
metadata:
  requires:
    bins: ["lx"]
---

# 搜索与人员查询

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**进入场景：**

- 用户说"搜索 XX"/"有没有关于 XX 的文档"/"找一下 XX"
- 用户说"查一下 XX 同事"/"谁是 XX"
- 用户说"搜索知识库"

**禁止在本 skill 中执行：**

- **不要在搜索后直接编辑**：搜索返回 `entry_id` 后，如需编辑 → **立即切换到 lx-entry 或 lx-block skill**
- **不要在本 skill 中创建页面**：用户说"创建一个页面" → **立即切换到 lx-entry skill**

**搜索不是终点：**

- 搜索只是定位工具，搜到 `entry_id` 后必须继续进入对应 skill 执行后续操作

## ⚡ 怎么选命令？（决策树）

```text
识别场景 →
├── 知道精确关键词? → lx search kb-search
├── 模糊描述 / 自然语言提问? → lx search kb-embedding-search
├── 查找同事信息? → lx contact search-staff
└── 确认当前登录身份? → lx contact whoami
```

## ⚠️ 高风险操作与默认优先路径

**关键词 vs 语义搜索：**

- 用户给出明确关键词（如产品名、文档标题）→ 用 `lx search kb-search`
- 用户描述模糊意图（如"怎么申请资源"）→ 用 `lx search kb-embedding-search`

**默认优先路径：**

1. 精确关键词 → `kb-search`
2. 模糊意图 → `kb-embedding-search`
3. 限定搜索范围 → 先获取 `space_id`，再传入搜索命令

**搜索结果后续处理：**

- 搜索返回 `entry_id` 后，如需获取内容，调用 `lx entry describe-ai-parse-content --entry-id <ID>`

## 可用工具

<!-- TODO: tools search [] -->

<!-- TODO: tools contact [] -->

## 🎯 执行规则

1. **关键词 vs 语义搜索**：用户给出明确关键词（如产品名、文档标题）时用 `lx search kb-search`；用户描述模糊意图（如"怎么申请资源"）时用 `lx search kb-embedding-search`。
2. **限定搜索范围**：如果用户指定了知识库，先通过 `lx space list-recently-spaces` 或 `lx space describe-space` 拿到 `space_id`，再传入搜索命令的 `--space-id`。
3. **只搜标题**：用户说"标题是…"时，使用 `lx search kb-search --keyword "..." --title-only`。
4. **搜索结果后续处理**：搜索返回 `entry_id` 后，如需获取内容，调用 `lx entry describe-ai-parse-content --entry-id <ID>`。
5. **人员搜索限制**：`lx contact search-staff` 依赖企业通讯录可见设置，接口报错时应告知用户可能是权限限制。
6. **分页**：默认只返回第一页。用户明确要求"更多结果"时才传 `--page-token` 翻页。

## 典型组合流程

### 在指定知识库中搜索

```bash
# 获取最近知识库列表，确认 space_id
lx space list-recently-spaces

# 在指定知识库中搜索
lx search kb-search --keyword "API 设计" --space-id sp_xxx
```

### 语义搜索 + 获取文档内容

```bash
# 语义搜索，获取匹配的 entry_id
lx search kb-embedding-search --keyword "如何配置数据库连接池"

# 获取文档 AI 可解析内容
lx entry describe-ai-parse-content --entry-id entry_xxx
```

### 查找同事的文档

```bash
# 查找人员信息
lx contact search-staff --staff-id "张三" --fuzzy-search

# 按创建人搜索文档
lx search kb-search --keyword "张三" --type kb_doc
```
