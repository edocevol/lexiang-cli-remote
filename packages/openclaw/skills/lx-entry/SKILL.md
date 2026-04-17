---
name: lx-entry
description: |
  乐享知识库条目管理。支持创建/查看/删除页面和文件夹、内容导入、文件上传下载、草稿管理、标签管理。触发词：页面、文档、条目、文件夹、创建文档、导入、上传文件、草稿、版本
---

# lx-entry

**当以下情况时使用此 Skill**:

- 需要创建新页面或文件夹
- 需要查看或读取文档内容
- 需要导入 Markdown/HTML 内容
- 需要上传或下载文件
- 需要管理草稿或标签

## 工具一：创建与浏览

创建条目，查看条目详情，列出子条目。

### 参数（创建条目）

- **parent-entry-id** (string, required): 父条目 ID
- **name** (string, required): 条目名称
- **entry-type** (string, required): 类型，`page` 或 `folder`

### 示例（创建页面）

```json
lx-entry: { "tool": "create-entry", "parent-entry-id": "root_xxx", "name": "新文档", "entry-type": "page" }
```

### 参数（列出子条目）

- **parent-id** (string, required): 父条目 ID

### 示例（浏览目录）

```json
lx-entry: { "tool": "list-children", "parent-id": "root_xxx" }
```

### 示例（查看内容）

```json
lx-entry: { "tool": "describe-ai-parse-content", "entry-id": "entry_xxx" }
```

## 工具二：内容导入

导入内容创建新文档，或导入到已有页面。

### 参数（导入内容）

- **space-id** (string): 知识库 ID（create-entry + import-content 组合）
- **parent-entry-id** (string): 父条目 ID
- **name** (string): 页面名称
- **content** (string, required): base64 编码的内容
- **content-type** (string, required): `markdown_base64` 或 `html_base64`
- **entry-id** (string): 已有页面 ID（import-content-to-entry）

### 示例（导入创建新文档）

```json
lx-entry: { "tool": "import-content", "space-id": "sp_xxx", "parent-entry-id": "root_xxx", "name": "新文档", "content": "<base64>", "content-type": "markdown_base64" }
```

## 工具三：文件管理

上传文件（3 步流程）、下载文件、管理文件版本。

### 参数（上传文件）

- **parent-entry-id** (string, required): 目标父条目 ID
- **name** (string, required): 文件名
- **upload-type** (string, required): `PRE_SIGNED_URL`
- **session-id** (string, required): apply-upload 返回的会话 ID

### 示例（上传流程）

Step 1: `lx-entry: { "tool": "apply-upload", "parent-entry-id": "folder_xxx", "name": "report.pdf", "upload-type": "PRE_SIGNED_URL" }`
Step 2: HTTP PUT 到返回的 `upload_url`
Step 3: `lx-entry: { "tool": "commit-upload", "session-id": "sess_xxx" }`

### 示例（下载文件）

```json
lx-entry: { "tool": "download-file", "entry-id": "entry_xxx" }
```

## 工具四：草稿与标签

管理 Markdown 草稿，设置条目标签。

### 参数（草稿）

- **entry-id** (string, required): 条目 ID
- **content** (string): 草稿内容
- **revision-id** (string): 版本 ID
- **seq** (number): 序列号

### 示例（保存草稿）

```json
lx-entry: { "tool": "save-markdown-draft", "entry-id": "entry_xxx", "content": "...", "seq": 0 }
```

### 参数（标签）

- **entry-id** (string, required): 条目 ID
- **add-tags** (string): 要添加的标签
- **del-tags** (string): 要删除的标签

### 示例（设置标签）

```json
lx-entry: { "tool": "set-entry-tags", "entry-id": "entry_xxx", "add-tags": "重要", "del-tags": "过时" }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 创建新页面 | 工具一：create-entry |
| 浏览目录树 | 工具一：list-children |
| 查看文档内容 | 工具一：describe-ai-parse-content |
| 导入内容创建文档 | 工具二：import-content |
| 上传文件 | 工具三：3 步上传流程 |
| 下载文件 | 工具三：download-file |
| 保存草稿 | 工具四：save-markdown-draft |
| 管理标签 | 工具四：set-entry-tags |
