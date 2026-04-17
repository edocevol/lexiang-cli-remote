---
name: lx-url-route
description: |
  识别乐享相关 URL 并路由到对应的 lx CLI 命令。当用户发送了链接但不明确要做什么时，用此 Skill 判断链接类型并调用合适的命令。触发词：链接、URL、帮我看看这个、lexiangla.com、lexiang.tencent.com、mp.weixin.qq.com
---

# lx-url-route

**当以下情况时使用此 Skill**:

- 用户只发了一个链接，没有明确说要查看、编辑、导入还是评论
- 需要先判断这是页面、知识库、团队还是公众号文章链接

## 工具一：页面链接处理

处理 `/pages/{entry_id}` 类型的链接。

### 参数（获取页面内容）

- **entry-id** (string, required): 条目 ID

### 示例（获取页面内容）

```json
lx-url-route: { "tool": "describe-ai-parse-content", "entry-id": "entry_xxx" }
```

### 参数（获取页面详情）

- **entry-id** (string, required): 条目 ID

### 示例（获取页面详情）

```json
lx-url-route: { "tool": "describe-entry", "entry-id": "entry_xxx" }
```

## 工具二：知识库链接处理

处理 `/spaces/{space_id}` 类型的链接。

### 参数（获取知识库详情）

- **space-id** (string, required): 知识库 ID

### 示例（获取知识库详情）

```json
lx-url-route: { "tool": "describe-space", "space-id": "sp_xxx" }
```

## 工具三：团队链接处理

处理 `/t/{team_id}/spaces` 类型的链接。

### 参数（列出团队知识库）

- **team-id** (string, required): 团队 ID

### 示例（列出团队知识库）

```json
lx-url-route: { "tool": "list-spaces", "team-id": "team_xxx" }
```

## 工具四：公众号文章导入

处理 `mp.weixin.qq.com/*` 类型的链接。

### 参数（创建超链接）

- **url** (string, required): 原始 URL
- **space-id** (string, required): 目标知识库 ID
- **parent-entry-id** (string, required): 目标父条目 ID

### 示例（导入公众号文章）

```json
lx-url-route: { "tool": "create-hyperlink", "url": "https://mp.weixin.qq.com/xxx", "space-id": "sp_xxx", "parent-entry-id": "folder_xxx" }
```

## URL 类型与默认动作

| URL 模式 | 提取字段 | 默认动作 | 用户意图明确时的去向 |
|------|------|------|------|
| `/pages/{entry_id}` | `entry_id` | `describe-ai-parse-content` | 查看详情 → `describe-entry`；编辑内容 → `lx-block`；查看评论 → `lx-connector` |
| `/spaces/{space_id}` | `space_id` | `describe-space` | 浏览目录 / 创建页面 → `lx-space` → `lx-entry` |
| `/t/{team_id}/spaces` | `team_id` | `list-spaces` | 选定知识库后继续走 `lx-space` / `lx-entry` |
| `mp.weixin.qq.com/*` | 原始 URL | 先补齐目标位置，再 `create-hyperlink` | 导入完成后如需编辑页面，再切到 `lx-entry` / `lx-block` |
| `lexiang.tencent.com/ai/claw` | 无 | 提示用户去该页面获取 Token | 不调用命令 |
| `mcp.lexiang-app.com/*` | 无 | 内部服务端点 | 不展示给用户 |

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 用户只发页面链接且没说要做什么 | 工具一：describe-ai-parse-content |
| 用户只发知识库链接 | 工具二：describe-space |
| 用户只发团队链接 | 工具三：list-spaces |
| 用户发送公众号文章链接 | 工具四：create-hyperlink（需先补齐目标位置） |

## 执行规则

1. **先判断要不要路由**：只有"用户给了 URL，但意图不明确"时才用本 skill。
2. **页面链接默认看内容**：默认动作是 `describe-ai-parse-content`，不是 `describe-entry`。
3. **公众号文章导入必须先补齐目标位置**：缺 `space-id` / `parent_entry_id` 时不能直接导入。
4. **不暴露内部域名**：`mcp.lexiang-app.com` 只用于内部服务识别，不能回显给用户。
5. **路由完成就退出**：拿到目标 ID 或确定下游命令后，应切换到对应 skill，不在本 skill 内继续做复杂操作。

## 禁止操作

- **不要路由已明确意图的场景**：用户已经说了要"编辑页面"/"查看评论"/"创建页面" → **直接切到对应 skill**
- **不要把内部 MCP 域名展示给用户**：生成用户可见链接时只使用 `lexiang.tencent.com`

## 典型组合流程

### 处理页面链接

```json
// 用户发送：https://lexiang.tencent.com/pages/entry_xxx

// 默认：查看内容
lx-url-route: { "tool": "describe-ai-parse-content", "entry-id": "entry_xxx" }

// 用户说"我要编辑这个页面" → 切到 lx-block
lx-block: { "tool": "tree", "block-id": "entry_xxx", "recursive": true }

// 用户说"看看评论" → 切到 lx-connector
lx-connector: { "tool": "list-comments", "target-type": "kb_entry", "target-id": "entry_xxx" }
```

### 处理知识库链接

```json
// 用户发送：https://lexiang.tencent.com/spaces/sp_xxx

// 获取详情和 root_entry_id
lx-url-route: { "tool": "describe-space", "space-id": "sp_xxx" }

// 用户说"浏览目录" → 切到 lx-entry
lx-entry: { "tool": "list-children", "parent-id": "root_xxx" }

// 用户说"创建页面" → 切到 lx-entry
lx-entry: { "tool": "create-entry", "parent-entry-id": "root_xxx", "name": "新文档", "entry-type": "page" }
```

### 导入公众号文章

```json
// 用户发送：https://mp.weixin.qq.com/s/xxx

// 先补齐目标位置
lx-space: { "tool": "list-recently-spaces" }

// 用户选择目标知识库和文件夹后
lx-url-route: { "tool": "create-hyperlink", "url": "https://mp.weixin.qq.com/s/xxx", "space-id": "sp_xxx", "parent-entry-id": "folder_xxx" }

// 导入完成后如需编辑 → 切到 lx-entry / lx-block
```
