---
name: lx-connector
version: 1.0.0
description: "乐享外部数据导入与评论管理。支持腾讯会议记录导入、页面评论查看。当用户需要从外部系统导入数据到知识库，或查看文档评论时使用。触发词：腾讯会议、会议记录、导入、评论、comment"
metadata:
  requires:
    bins: ["lx"]
---

# 外部数据导入与评论

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**进入场景：**

- 用户说"导入腾讯会议记录"/"把这个会议存到知识库"
- 用户说"查看这个页面的评论"

**禁止在本 skill 中执行：**

- **不要编辑页面内容**：用户说"编辑某个页面内容" → **立即切换到 lx-block skill**
- **不要创建页面**：用户说"在知识库里创建页面" → **立即切换到 lx-entry skill**

## ⚡ 怎么选命令？（决策树）

```text
识别场景 →
├── 导入腾讯会议记录?
│   └── lx meeting search-tx-meeting-records → lx meeting import-tx-meeting-record
└── 查看/管理页面评论?
    └── lx comment list-comments / lx comment describe-comment
```

## ⚠️ 高风险操作与默认优先路径

**会议导入流程：**

- 必须先搜索（`lx meeting search-tx-meeting-records`）拿到录制信息
- 再导入（`lx meeting import-tx-meeting-record`）
- 导入需要指定目标知识库的 `--parent-entry-id`

**默认优先路径：**

1. 导入目标必须预先确定 → 若用户未指定，需先通过 lx-space skill 定位目标知识库和父节点
2. 评论内容特殊格式 → `lx comment describe-comment` 返回的 `content` 不是普通 HTML，需要特殊解析

## 可用工具

### 腾讯会议导入

<!-- TODO: tools meeting [] -->

### 评论管理

<!-- TODO: tools comment [] -->

## 🎯 执行规则

1. **会议导入流程**：必须先搜索（`lx meeting search-tx-meeting-records`）拿到录制信息，再导入（`lx meeting import-tx-meeting-record`）。导入需要指定目标知识库的 `--parent-entry-id`。
2. **评论内容特殊格式**：`lx comment describe-comment` 返回的 `content` 不是普通 HTML，需要特殊解析。
3. **导入目标必须预先确定**：所有导入操作都需要 `--parent-entry-id`，若用户未指定，需先通过 lx-space skill 定位目标知识库和父节点。

## 典型组合流程

### 导入腾讯会议记录

1. 搜索会议记录 - 调用 `lx-meeting-search-tx-meeting-records` 工具
   - meeting_code: `123456789`

2. 用户确认后导入 - 调用 `lx-meeting-import-tx-meeting-record` 工具
   - record_file_id: `rec_file_xxx`
   - parent_entry_id: `folder_xxx`

### 查看页面评论

1. 获取评论列表 - 调用 `lx-comment-list-comments` 工具
   - target_type: `kb_entry`
   - target_id: `entry_xxx`

2. 查看评论详情 - 调用 `lx-comment-describe-comment` 工具
   - comment_id: `comment_xxx`
