---
name: lx-url-router
version: 1.0.0
description: "识别乐享相关 URL 并路由到对应的 lx CLI 命令。当用户发送了链接但不明确要做什么时，用此 Skill 判断链接类型并调用合适的命令。触发词：链接、URL、帮我看看这个、lexiangla.com、lexiang.tencent.com、mp.weixin.qq.com"
metadata:
  requires:
    bins: ["lx"]
---

# URL 路由

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**进入场景：**
- 用户发了一个链接，你不确定要做什么
- 需要判断链接类型并路由到正确的命令

**禁止在本 skill 中执行：**
- **不要路由已明确意图的场景**：用户明确说了要做什么（如"帮我编辑这个页面"） → **直接用对应的 skill**，不需要路由

## ⚡ 怎么路由？（决策树）

```
收到 URL →
├── /pages/{id}         → lx entry describe-ai-parse-content（默认查看）
├── /spaces/{id}        → lx space describe-space
├── /t/{id}/spaces      → lx space list-spaces
├── mp.weixin.qq.com/*  → lx file create-hyperlink（需问目标位置）
├── lexiang.tencent.com/ai/claw → Token 配置引导，不调用命令
├── mcp.lexiang-app.com/*       → 内部端点，不展示给用户
└── 其他                → 告知不支持
```

## ⚠️ 高风险操作与默认优先路径

**默认行为：**
- 当用户只发了链接没说做什么时，默认用 `lx entry describe-ai-parse-content` 获取内容

**禁止行为：**
- 不要向用户展示 `mcp.lexiang-app.com` 域名
- 生成用户链接时使用 `lexiang.tencent.com` 域名

**微信公众号文章导入：**
- 必须额外询问用户：导入到哪个知识库/文件夹？

## URL 识别规则

### 1. 文档/页面链接

**匹配模式：**

- `https://lexiangla.com/pages/{entry_id}?company_from=xxx`
- `https://{tenant}.lexiangla.com/pages/{entry_id}`
- `https://lexiang.tencent.com/pages/{entry_id}`

**提取方式：** 从 URL path 中 `/pages/` 后面提取 `entry_id`

**路由动作：**

| 用户意图         | 命令 |
|------------------|------|
| 查看内容（默认） | `lx entry describe-ai-parse-content --entry-id {entry_id}` |
| 查看条目详情     | `lx entry describe-entry --entry-id {entry_id}` |
| 编辑/更新块内容  | 切换到 lx-block skill |
| 查看评论         | `lx comment list-comments --target-type kb_entry --target-id {entry_id}` |

> 当用户只发了链接没说做什么时，默认用 `lx entry describe-ai-parse-content` 获取内容。

### 2. 知识库链接

**匹配模式：**

- `https://lexiangla.com/spaces/{space_id}?company_from=xxx`
- `https://{tenant}.lexiangla.com/spaces/{space_id}`
- `https://lexiang.tencent.com/spaces/{space_id}`

**提取方式：** 从 URL path 中 `/spaces/` 后面提取 `space_id`

**路由动作：**

```bash
# 获取知识库详情
lx space describe-space --space-id {space_id}

# 如需浏览内容，用返回的 root_entry_id 列出目录
lx entry list-children --parent-id {root_entry_id}
```

### 3. 团队链接

**匹配模式：**

- `https://lexiangla.com/t/{team_id}/spaces?company_from=xxx`
- `https://{tenant}.lexiangla.com/t/{team_id}/spaces`
- `https://lexiang.tencent.com/t/{team_id}/spaces`

**提取方式：** 从 URL path 中 `/t/` 后面提取 `team_id`

**路由动作：**

```bash
lx space list-spaces --team-id {team_id}
```

### 4. 微信公众号文章链接

**匹配模式：**

- `https://mp.weixin.qq.com/s/xxx`
- `https://mp.weixin.qq.com/s?__biz=xxx&mid=xxx&idx=xxx`

**路由动作：**

```bash
# 需要先确认用户要导入到哪个知识库/文件夹
lx file create-hyperlink --url "https://mp.weixin.qq.com/s/xxx" --space-id sp_xxx --parent-entry-id folder_xxx
```

> 必须额外询问用户：导入到哪个知识库/文件夹？

### 5. Token 配置链接

**匹配模式：**

- `https://lexiang.tencent.com/ai/claw`

**路由动作：** 这是 Token 获取页面，不需要调用命令。提示用户在该页面获取 Token 后发送给你。

### 6. MCP 服务链接（不是用户链接）

**匹配模式：**

- `https://mcp.lexiang-app.com/*`

**路由动作：** 这是内部 MCP 服务端点，不是用户可访问的网页。不要向用户展示此域名。生成用户链接时使用 `lexiang.tencent.com` 域名。

## 无法识别的链接

如果 URL 不匹配以上任何模式，告诉用户此链接不是乐享相关链接，无法处理。

## 示例对话

```text
用户: https://lexiang.tencent.com/pages/abc123
AI:   [执行 lx entry describe-ai-parse-content --entry-id abc123]
      这是一篇关于 XXX 的文档，内容如下...

用户: 帮我把这个公众号文章存到知识库 https://mp.weixin.qq.com/s/xyz789
AI:   好的，请问要存到哪个知识库？
用户: 存到 XX 团队的 YY 知识库
AI:   [执行 lx file create-hyperlink --url "https://mp.weixin.qq.com/s/xyz789" --space-id sp_xxx --parent-entry-id folder_xxx]
      已导入！

用户: https://lexiang.tencent.com/spaces/sp_123
AI:   [执行 lx space describe-space --space-id sp_123]
      这是「XX」知识库，包含 N 个文档...
```
