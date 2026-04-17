---
name: lx-space
description: |
  乐享知识库与团队管理。支持查看团队列表、知识库列表、知识库详情（含 root_entry_id）。触发词：知识库、空间、团队、space、team、最近的知识库
---

# lx-space

**当以下情况时使用此 Skill**:

- 需要查看用户可访问的团队列表
- 需要查看团队下的知识库列表
- 需要获取知识库详情（含 root_entry_id）
- 需要快速定位最近使用的知识库

## 工具一：团队管理

列出用户可访问的团队，或查看团队详情。

### 参数（列出团队）

无必需参数，支持 `--limit`、`--offset` 分页。

### 示例（列出所有团队）

```json
lx-space: { "tool": "list-teams" }
```

### 示例（常用团队）

```json
lx-space: { "tool": "list-frequent-teams" }
```

### 示例（团队详情）

```json
lx-space: { "tool": "describe-team", "team-id": "team_xxx" }
```

## 工具二：知识库管理

列出团队下的知识库，或查看知识库详情。

### 参数（列出知识库）

- **team-id** (string, required): 团队 ID

### 示例（列出知识库）

```json
lx-space: { "tool": "list-spaces", "team-id": "team_xxx" }
```

### 参数（知识库详情）

- **space-id** (string, required): 知识库 ID

### 示例（知识库详情）

```json
lx-space: { "tool": "describe-space", "space-id": "sp_xxx" }
```

> `describe-space` 返回 `root_entry_id`，是后续操作条目的关键字段。

### 示例（最近使用的知识库）

```json
lx-space: { "tool": "list-recently-spaces" }
```

## 层级关系

```text
团队(Team) → 知识库(Space) → 条目(Entry)
```

- 知识库必须属于某个团队
- `describe-space` 返回的 `root_entry_id` 是知识库的根条目 ID，用于后续创建页面或浏览目录树

## 访问链接

- 团队首页：`{domain}/t/{team_id}/spaces`
- 知识库访问：`{domain}/spaces/{space_id}`

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 不知道在哪个团队 | 工具一：list-teams 或 list-frequent-teams |
| 知道团队，找知识库 | 工具二：list-spaces |
| 知道知识库 ID | 工具二：describe-space |
| 快速定位最近用的知识库 | 工具二：list-recently-spaces |
| 需要获取 root_entry_id | 工具二：describe-space |
