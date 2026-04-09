---
name: lx-url-route
version: 1.0.0
description: "识别乐享相关 URL 并路由到对应的 lx CLI 命令。当用户发送了链接但不明确要做什么时，用此 Skill 判断链接类型并调用合适的命令。触发词：链接、URL、帮我看看这个、lexiangla.com、lexiang.tencent.com、mp.weixin.qq.com"
metadata:
  requires:
    bins: ["lx"]
---

# URL 路由

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

### 进入场景

- 用户只发了一个链接，没有明确说要查看、编辑、导入还是评论
- 需要先判断这是页面、知识库、团队还是公众号文章链接

### 禁止在本 skill 中执行

- **不要路由已明确意图的场景**：用户已经说了要“编辑页面”/“查看评论”/“创建页面” → **直接切到对应 skill**
- **不要把内部 MCP 域名展示给用户**：生成用户可见链接时只使用 `lexiang.tencent.com`

## ⚡ 怎么选命令？（决策树）

```text
收到 URL →
├── /pages/{entry_id}        → 默认 `lx entry describe-ai-parse-content`
├── /spaces/{space_id}       → `lx space describe-space`
├── /t/{team_id}/spaces      → `lx space list-spaces`
├── mp.weixin.qq.com/*       → 先补齐目标位置，再 `lx file create-hyperlink`
├── lexiang.tencent.com/ai/claw → Token 配置引导，不调用命令
├── mcp.lexiang-app.com/*    → 内部端点，不展示给用户
└── 其他                     → 告知不是可处理的乐享链接
```

## ⚠️ 高风险操作与默认优先路径

**默认行为：**

- 用户只发页面链接且没说要做什么 → 默认查看内容，用 `lx entry describe-ai-parse-content`
- 用户只发知识库链接 → 先用 `lx space describe-space` 拿 `root_entry_id`，再决定是否切到 `lx-entry`
- 用户只发团队链接 → 先列出该团队下的知识库，不直接猜测目标知识库

**必须补齐的信息：**

- 微信公众号文章导入前，必须先确认 `space_id` 和 `parent_entry_id`
- 如果用户要编辑页面内容、查看评论、创建页面，不要停留在本 skill，立刻切换到下游 skill

## URL 类型与默认动作

| URL 模式 | 提取字段 | 默认动作 | 用户意图明确时的去向 |
|------|------|------|------|
| `/pages/{entry_id}` | `entry_id` | `lx entry describe-ai-parse-content --entry-id <ID>` | 查看详情 → `lx entry describe-entry`；编辑内容 → `lx-block`；查看评论 → `lx-connector` |
| `/spaces/{space_id}` | `space_id` | `lx space describe-space --space-id <ID>` | 浏览目录 / 创建页面 → `lx-space` → `lx-entry` |
| `/t/{team_id}/spaces` | `team_id` | `lx space list-spaces --team-id <ID>` | 选定知识库后继续走 `lx-space` / `lx-entry` |
| `mp.weixin.qq.com/*` | 原始 URL | 先补齐目标位置，再 `lx file create-hyperlink --url ... --space-id ... --parent-entry-id ...` | 导入完成后如需编辑页面，再切到 `lx-entry` / `lx-block` |
| `lexiang.tencent.com/ai/claw` | 无 | 提示用户去该页面获取 Token | 不调用命令 |
| `mcp.lexiang-app.com/*` | 无 | 内部服务端点 | 不展示给用户 |

## 🎯 执行规则

1. **先判断要不要路由**：只有“用户给了 URL，但意图不明确”时才用本 skill。
2. **页面链接默认看内容**：默认动作是 `describe-ai-parse-content`，不是 `describe-entry`。
3. **公众号文章导入必须先补齐目标位置**：缺 `space_id` / `parent_entry_id` 时不能直接导入。
4. **不暴露内部域名**：`mcp.lexiang-app.com` 只用于内部服务识别，不能回显给用户。
5. **路由完成就退出**：拿到目标 ID 或确定下游命令后，应切换到对应 skill，不在本 skill 内继续做复杂操作。
