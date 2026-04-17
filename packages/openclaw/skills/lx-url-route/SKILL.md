---
name: lx-url-route
description: |
  识别乐享相关 URL 并路由到对应的 lx CLI 命令。当用户发送了链接但不明确要做什么时，用此 Skill 判断链接类型并调用合适的命令。触发词：链接、URL、帮我看看这个、lexiangla.com、lexiang.tencent.com、mp.weixin.qq.com
---

# lx-url-route

**当以下情况时使用此 Skill**:

- 用户只发了一个链接，没有明确意图
- 需要判断链接类型（页面/知识库/团队/公众号文章）
- 需要路由到合适的后续操作

## 工具：URL 路由

根据 URL 类型自动路由到对应命令。

### URL 类型与默认动作

| URL 模式 | 提取字段 | 默认动作 |
|------|------|----------|
| `/pages/{entry_id}` | `entry_id` | `lx entry describe-ai-parse-content --entry-id <ID>` |
| `/spaces/{space_id}` | `space_id` | `lx space describe-space --space-id <ID>` |
| `/t/{team_id}/spaces` | `team_id` | `lx space list-spaces --team-id <ID>` |
| `mp.weixin.qq.com/*` | 原始 URL | 先补齐目标位置，再 `lx file create-hyperlink` |
| `lexiang.tencent.com/ai/claw` | 无 | 提示用户去该页面获取 Token |
| `mcp.lexiang-app.com/*` | 无 | 内部端点，不展示给用户 |

### 示例（页面链接）

用户发：`https://lexiang.tencent.com/pages/entry_xxx`

```json
lx-url-route: { "url": "https://lexiang.tencent.com/pages/entry_xxx" }
```

→ 默认执行：`lx entry describe-ai-parse-content --entry-id entry_xxx`

### 示例（知识库链接）

用户发：`https://lexiang.tencent.com/spaces/sp_xxx`

```json
lx-url-route: { "url": "https://lexiang.tencent.com/spaces/sp_xxx" }
```

→ 默认执行：`lx space describe-space --space-id sp_xxx`

### 示例（团队链接）

用户发：`https://lexiang.tencent.com/t/team_xxx/spaces`

```json
lx-url-route: { "url": "https://lexiang.tencent.com/t/team_xxx/spaces" }
```

→ 默认执行：`lx space list-spaces --team-id team_xxx`

### 示例（公众号文章）

用户发：`https://mp.weixin.qq.com/s/...`

→ 先询问用户要导入到哪个知识库和父条目，再执行导入

## 路由后的后续动作

| 用户明确意图 | 路由去向 |
|------|----------|
| 查看详情 | `lx-entry` skill |
| 编辑内容 | `lx-block` skill |
| 查看评论 | `lx-connector` skill |
| 创建页面 | `lx-entry` skill |
| 浏览目录 | `lx-space` → `lx-entry` skill |

## 注意事项

- 只有"用户给了 URL，但意图不明确"时才用本 skill
- 页面链接默认看内容（`describe-ai-parse-content`），不是 `describe-entry`
- 公众号文章导入必须先补齐 `space_id` 和 `parent_entry_id`
- 不暴露内部域名（`mcp.lexiang-app.com`）
- 路由完成就退出，切换到对应 skill

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 用户只发页面链接 | 默认查看内容 → lx-entry |
| 用户只发知识库链接 | describe-space → lx-space |
| 用户只发团队链接 | list-spaces → lx-space |
| 公众号文章导入 | 先补齐目标位置再导入 |
| 用户意图明确 | 直接切到对应 skill，不用本 skill |
