---
name: lx-connector
description: |
  乐享外部数据导入与评论管理。支持腾讯会议记录导入、页面评论查看。触发词：腾讯会议、会议记录、导入、评论、comment
---

# lx-connector

**当以下情况时使用此 Skill**:

- 需要导入腾讯会议记录到知识库
- 需要查看页面的评论
- 需要管理评论内容

## 工具一：腾讯会议导入

搜索并导入腾讯会议记录到知识库。

### 参数（会议搜索）

- **meeting-code** (string, required): 会议号

### 示例（搜索会议）

```json
lx-connector: { "tool": "search-tx-meeting-records", "meeting-code": "123456789" }
```

### 参数（会议导入）

- **record-file-id** (string, required): 录制文件 ID
- **parent-entry-id** (string, required): 目标父条目 ID

### 示例（导入会议）

```json
lx-connector: { "tool": "import-tx-meeting-record", "record-file-id": "rec_file_xxx", "parent-entry-id": "folder_xxx" }
```

## 工具二：评论管理

查看页面评论详情。

### 参数（评论列表）

- **target-type** (string, required): 目标类型，如 `kb_entry`
- **target-id** (string, required): 目标 ID

### 示例（评论列表）

```json
lx-connector: { "tool": "list-comments", "target-type": "kb_entry", "target-id": "entry_xxx" }
```

### 示例（评论详情）

```json
lx-connector: { "tool": "describe-comment", "comment-id": "comment_xxx" }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 导入腾讯会议记录 | 工具一：先搜索再导入 |
| 查看页面评论 | 工具二：list-comments |
| 查看评论详情 | 工具二：describe-comment |
