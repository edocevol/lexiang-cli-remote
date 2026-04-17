---
name: lx-entry
description: |
  乐享知识库条目管理。支持创建、查看、编辑、删除页面/文件夹，导入内容，管理文件（上传、下载、版本控制），处理 Markdown 草稿。触发词：页面、文档、条目、文件夹、创建文档、导入、上传文件、草稿、版本
---

# lx-entry

**当以下情况时使用此 Skill**:

- 需要创建新页面或文件夹
- 需要查看或读取文档内容
- 需要浏览目录树
- 需要导入 Markdown/HTML 内容
- 需要上传或下载文件
- 需要管理 Markdown 草稿
- 需要管理条目标签
- 需要移动或重命名条目

## 工具一：创建与浏览

创建页面/文件夹，列出子条目，获取条目详情和内容。

### 参数（创建条目）

- **parent-entry-id** (string, required): 父条目 ID
- **name** (string, required): 条目名称
- **entry-type** (string): 条目类型，`page` 或 `folder`

### 示例（创建页面）

```json
lx-entry: { "tool": "create-entry", "parent-entry-id": "root_xxx", "name": "新文档", "entry-type": "page" }
```

### 参数（列出子条目）

- **parent-id** (string, required): 父条目 ID

### 示例（列出子条目）

```json
lx-entry: { "tool": "list-children", "parent-id": "root_xxx" }
```

### 参数（获取条目详情）

- **entry-id** (string, required): 条目 ID

### 示例（获取详情）

```json
lx-entry: { "tool": "describe-entry", "entry-id": "entry_xxx" }
```

### 参数（获取 AI 可解析内容）

- **entry-id** (string, required): 条目 ID

### 示例（获取内容）

```json
lx-entry: { "tool": "describe-ai-parse-content", "entry-id": "entry_xxx" }
```

## 工具二：内容导入

导入内容创建新文档或追加到已有页面。

### 参数（导入创建新文档）

- **parent-entry-id** (string, required): 父条目 ID
- **name** (string, required): 文档名称
- **content** (string): base64 编码的内容
- **content-type** (string): 内容类型，`markdown_base64` 或 `html_base64`

### 示例（导入创建新文档）

```json
lx-entry: { "tool": "import-content", "parent-entry-id": "root_xxx", "name": "API文档", "content": "base64内容", "content-type": "markdown_base64" }
```

### 参数（导入到已有页面）

- **entry-id** (string, required): 目标条目 ID
- **content** (string): base64 编码的内容
- **content-type** (string): 内容类型，`markdown_base64` 或 `html_base64`
- **after-block-id** (string): 在此块后插入（仅限根级别块）
- **force-write** (boolean): 是否强制覆盖

### 示例（导入到已有页面）

```json
lx-entry: { "tool": "import-content-to-entry", "entry-id": "entry_xxx", "content": "base64内容", "content-type": "markdown_base64" }
```

## 工具三：文件上传（三步流程）

申请上传凭证、HTTP PUT 上传、确认上传完成。

### 参数（申请上传凭证）

- **parent-entry-id** (string, required): 父条目 ID
- **name** (string, required): 文件名
- **upload-type** (string): 上传类型，`PRE_SIGNED_URL`

### 示例（申请上传凭证）

```json
lx-entry: { "tool": "apply-upload", "parent-entry-id": "folder_xxx", "name": "report.pdf", "upload-type": "PRE_SIGNED_URL" }
```

### 参数（确认上传）

- **session-id** (string, required): 会话 ID

### 示例（确认上传）

```json
lx-entry: { "tool": "commit-upload", "session-id": "sess_xxx" }
```

## 工具四：文件下载与版本管理

获取文件下载地址、查看文件详情、历史版本、恢复指定版本。

### 参数（下载文件）

- **file-id** (string, required): 文件 ID

### 示例（下载文件）

```json
lx-entry: { "tool": "download-file", "file-id": "file_xxx" }
```

### 参数（获取文件详情）

- **file-id** (string, required): 文件 ID

### 示例（获取文件详情）

```json
lx-entry: { "tool": "describe-file", "file-id": "file_xxx" }
```

### 参数（列出历史版本）

- **file-id** (string, required): 文件 ID

### 示例（列出历史版本）

```json
lx-entry: { "tool": "list-revisions", "file-id": "file_xxx" }
```

### 参数（恢复版本）

- **file-id** (string, required): 文件 ID
- **revision-id** (string, required): 版本 ID

### 示例（恢复版本）

```json
lx-entry: { "tool": "revert-file", "file-id": "file_xxx", "revision-id": "rev_xxx" }
```

## 工具五：草稿管理

获取、保存、发布 Markdown 草稿。

### 参数（获取草稿）

- **entry-id** (string, required): 条目 ID

### 示例（获取草稿）

```json
lx-entry: { "tool": "describe-markdown-draft", "entry-id": "entry_xxx" }
```

### 参数（保存草稿）

- **entry-id** (string, required): 条目 ID
- **revision-id** (string, required): 版本 ID
- **content** (string, required): 草稿内容
- **seq** (number): 序号

### 示例（保存草稿）

```json
lx-entry: { "tool": "save-markdown-draft", "entry-id": "entry_xxx", "revision-id": "rev_xxx", "content": "草稿内容", "seq": 0 }
```

### 参数（发布草稿）

- **entry-id** (string, required): 条目 ID
- **revision-id** (string, required): 版本 ID

### 示例（发布草稿）

```json
lx-entry: { "tool": "publish-markdown-draft", "entry-id": "entry_xxx", "revision-id": "rev_xxx" }
```

## 工具六：标签管理

获取条目标签、增删标签。

### 参数（获取标签）

- **entry-id** (string, required): 条目 ID

### 示例（获取标签）

```json
lx-entry: { "tool": "list-entry-tags", "entry-id": "entry_xxx" }
```

### 参数（设置标签）

- **entry-id** (string, required): 条目 ID
- **add-tags** (string): 要添加的标签
- **del-tags** (string): 要删除的标签

### 示例（设置标签）

```json
lx-entry: { "tool": "set-entry-tags", "entry-id": "entry_xxx", "add-tags": "重要", "del-tags": "过时" }
```

## 工具七：条目操作

移动、重命名条目。

### 参数（移动条目）

- **entry-id** (string, required): 条目 ID
- **target-parent-id** (string, required): 目标父条目 ID

### 示例（移动条目）

```json
lx-entry: { "tool": "move-entry", "entry-id": "entry_xxx", "target-parent-id": "folder_yyy" }
```

### 参数（重命名条目）

- **entry-id** (string, required): 条目 ID
- **name** (string, required): 新名称

### 示例（重命名条目）

```json
lx-entry: { "tool": "rename-entry", "entry-id": "entry_xxx", "name": "新名称" }
```

## 选择建议

| 场景 | 推荐工具 |
|------|----------|
| 创建页面/文件夹 | 工具一：create-entry |
| 浏览目录树 | 工具一：list-children |
| 查看文档内容 | 工具一：describe-ai-parse-content |
| 导入内容创建新文档 | 工具二：import-content |
| 导入内容到已有页面 | 工具二：import-content-to-entry |
| 上传文件 | 工具三：三步流程 |
| 下载文件 | 工具四：download-file |
| 管理文件版本 | 工具四：list-revisions / revert-file |
| 编辑草稿 | 工具五：草稿管理 |
| 管理标签 | 工具六：标签管理 |
| 移动/重命名条目 | 工具七：move-entry / rename-entry |

## 执行规则

1. **创建一级条目**：必须先通过 `describe-space` 获取 `root_entry_id`，再将其作为 `parent-entry-id` 传入。
2. **内容编码**：导入内容时必须使用 base64 编码格式（`markdown_base64` / `html_base64`），避免转义问题。
3. **已有页面优先局部编辑**：若目标页面已存在，禁止用 `import-content-to-entry --force-write` 覆盖，应优先使用 lx-block skill 进行局部更新。
4. **文件上传是 3 步流程**：`apply-upload` → HTTP PUT 到 `upload_url` → `commit-upload`，缺一不可。
5. **条目访问链接**：`{domain}/pages/{entry_id}`
6. **`--after-block-id` 限制**：`import-content-to-entry` 的 `--after-block-id` 只能是页面第一层（根级别）的 block ID，不能是嵌套的子 block。

## 禁止操作

- **不要修改页面内部内容**：用户说"改一段内容"/"替换某个章节"/"改表格" → **立即切换到 lx-block skill**
- **不要进行可回滚的批量修改**：多步高风险修改 → **立即切换到 lx-git skill**，先用 `lx git clone` 建立本地工作区
- **不要在知识库中搜索**：用户说"搜索" → **立即切换到 lx-search skill**

## 典型组合流程

### 创建页面并导入内容

```json
// 获取 root_entry_id
lx-space: { "tool": "describe-space", "space-id": "sp_xxx" }

// 创建空白页面
lx-entry: { "tool": "create-entry", "parent-entry-id": "root_xxx", "name": "新文档", "entry-type": "page" }

// 导入内容
lx-entry: { "tool": "import-content-to-entry", "entry-id": "entry_xxx", "content": "base64内容", "content-type": "markdown_base64" }
```

### 上传文件到知识库

```json
// Step 1: 获取上传凭证
lx-entry: { "tool": "apply-upload", "parent-entry-id": "folder_xxx", "name": "report.pdf", "upload-type": "PRE_SIGNED_URL" }

// Step 2: HTTP PUT 上传（外部执行）

// Step 3: 确认上传
lx-entry: { "tool": "commit-upload", "session-id": "sess_xxx" }
```

### 浏览文档目录

```json
// 获取 root_entry_id
lx-space: { "tool": "describe-space", "space-id": "sp_xxx" }

// 获取一级目录
lx-entry: { "tool": "list-children", "parent-id": "root_xxx" }

// 逐级展开子目录
lx-entry: { "tool": "list-children", "parent-id": "folder_xxx" }
```
