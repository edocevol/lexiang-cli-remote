---
name: lx-space
description: |
  乐享知识库与团队管理。支持查看、管理知识库（空间）或团队信息。触发词：知识库、空间、团队、space、team
---

# lx-space

**当以下情况时使用此 Skill**:

- 需要查看可访问的团队列表
- 需要查看团队下的知识库列表
- 需要获取知识库详情
- 需要获取最近访问的知识库
- 需要获取知识库根节点（后续操作条目）

## 工具一：团队管理

列出用户可访问的团队、获取常用团队、获取团队详情。

### 参数（列出团队）

无参数。

### 示例（列出团队）

```json
lx-space: { "tool": "list-teams" }
```

### 参数（列出常用团队）

无参数。

### 示例（列出常用团队）

```json
lx-space: { "tool": "list-frequent-teams" }
```

### 参数（获取团队详情）

- **team-id** (string, required): 团队 ID

### 示例（获取团队详情）

```json
lx-space: { "tool": "describe-team", "team-id": "team_xxx" }
```

## 工具二：知识库管理

列出团队下的知识库、获取知识库详情、获取最近访问的知识库。

### 参数（列出知识库）

- **team-id** (string, required): 团队 ID

### 示例（列出知识库）

```json
lx-space: { "tool": "list-spaces", "team-id": "team_xxx" }
```

### 参数（获取知识库详情）

- **space-id** (string, required): 知识库 ID

### 示例（获取知识库详情）

```json
lx-space: { "tool": "describe-space", "space-id": "sp_xxx" }
```

### 参数（获取最近访问的知识库）

无参数。

### 示例（获取最近访问的知识库）

```json
lx-space: { "tool": "list-recently-spaces" }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 不知道在哪个团队 | 工具一：list-teams 或 list-frequent-teams |
| 知道团队，找知识库 | 工具二：list-spaces |
| 知道知识库 ID | 工具二：describe-space |
| 快速定位最近用的知识库 | 工具二：list-recently-spaces |
| 需要获取知识库根节点（后续操作条目） | 工具二：describe-space → 取 root_entry_id |

## 执行规则

1. **层级关系**：团队(Team) → 知识库(Space) → 条目(Entry)。知识库必须属于某个团队。
2. **获取 root_entry_id 是关键**：后续操作条目（创建页面、浏览目录树）都需要先通过 `describe-space` 拿到 `root_entry_id`。
3. **快速路径优先**：用户说"最近的知识库"时直接用 `list-recently-spaces`，不要走 team → space 全链路。
4. **团队首页链接**：`{domain}/t/{team_id}/spaces`
5. **知识库访问链接**：`{domain}/spaces/{space_id}`

## 禁止操作

- **不要在本 skill 中创建页面**：用户说"创建页面" → **立即切换到 lx-entry skill**
- **不要在本 skill 中编辑页面**：用户说"编辑某个页面" → **立即切换到 lx-block skill**
- **不要在本 skill 中搜索内容**：用户说"搜索知识库内容" → **立即切换到 lx-search skill**

## 典型组合流程

### 从团队到知识库到文档

```json
// 获取团队列表，让用户选择目标团队
lx-space: { "tool": "list-teams" }

// 获取该团队下的知识库列表
lx-space: { "tool": "list-spaces", "team-id": "team_xxx" }

// 获取知识库 root_entry_id
lx-space: { "tool": "describe-space", "space-id": "sp_xxx" }

// 遍历文档目录树（→ lx-entry skill）
lx-entry: { "tool": "list-children", "parent-id": "root_entry_xxx" }
```

### 快速定位最近使用的知识库

```json
// 获取最近访问的知识库
lx-space: { "tool": "list-recently-spaces" }

// 获取详情和 root_entry_id
lx-space: { "tool": "describe-space", "space-id": "sp_xxx" }
```
