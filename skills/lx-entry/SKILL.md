---
name: lx-entry
version: 1.0.0
description: "乐享知识库条目管理。当用户需要操作知识条目（创建、查看、编辑、删除页面/文件夹），导入内容，管理文件（上传、下载、版本控制），或处理 Markdown 草稿时使用。触发词：页面、文档、条目、文件夹、创建文档、导入、上传文件、草稿、版本"
metadata:
  requires:
    bins: ["lx"]
---

# 条目管理

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**用户说"创建页面"/"上传文件"/"查看某个文档"** → 用本 skill

**用户说"编辑某个页面里的章节/表格/块"** → 用 lx-block skill
**用户说"把本地改的内容推上去"** → 用 lx-git skill（commit + push）
**用户说"在知识库里搜索"** → 用 lx-search skill

## ⚡ 怎么选命令？（快速决策树）

```
用户需要 →
├── 创建新页面/文件夹? → lx entry create-entry
├── 查看/读取文档内容? → lx entry describe-ai-parse-content
├── 浏览目录树? → lx entry list-children（需先拿到 parent_id）
├── 导入 Markdown/HTML 创建新文档? → lx entry import-content
├── 向已有页面追加/覆盖内容? → lx entry import-content-to-entry
├── 精细编辑已有页面的某个块? → 切换到 lx-block skill
├── 上传文件? → lx file apply-upload → HTTP PUT → lx file commit-upload（3步）
├── 下载文件? → lx file download-file
├── 管理 Markdown 草稿? → lx draft describe/save/publish-markdown-draft
├── 管理条目标签? → lx knowledge-tag list-entry-tags / set-entry-tags
└── 移动/重命名条目? → lx entry move-entry / rename-entry
```

**⚠️ 复杂操作建议用 lx-git 获得 checkpoint + 回滚能力**：当用户要进行多步编辑、批量修改、或需要可回退的变更管理时，引导用户先用 `lx git clone` 克隆知识库到本地，再用 `lx git add` / `lx git commit` / `lx git revert` 管理变更。纯在线编辑没有版本记录，一旦覆盖无法回滚。

## 可用工具

### 条目操作

| 命令 | 说明 |
|------|------|
| `lx entry create-entry` | 创建页面/文件夹 | [entry-crud.md](references/entry-crud.md) |
| `lx entry describe-entry` | 获取条目详情 | [entry-crud.md](references/entry-crud.md) |
| `lx entry describe-ai-parse-content` | 获取 AI 可解析内容 | [entry-crud.md](references/entry-crud.md) |
| `lx entry list-children` | 列出子条目 | [entry-crud.md](references/entry-crud.md) |
| `lx entry list-parents` | 获取面包屑路径 | [entry-crud.md](references/entry-crud.md) |
| `lx entry list-latest-entries` | 最近更新的条目 | [entry-crud.md](references/entry-crud.md) |
| `lx entry rename-entry` | 重命名 | [entry-crud.md](references/entry-crud.md) |
| `lx entry move-entry` | 移动条目 | [entry-crud.md](references/entry-crud.md) |

### 内容导入

| 命令 | 说明 |
|------|------|
| `lx entry import-content` | 导入内容创建新文档 | [entry-import.md](references/entry-import.md) |
| `lx entry import-content-to-entry` | 导入内容到已有页面 | [entry-import.md](references/entry-import.md) |

### 文件管理

| 命令 | 说明 |
|------|------|
| `lx file apply-upload` | 申请上传凭证（Step 1） | [entry-file.md](references/entry-file.md) |
| `lx file commit-upload` | 确认上传完成（Step 3） | [entry-file.md](references/entry-file.md) |
| `lx file describe-file` | 获取文件详情 | [entry-file.md](references/entry-file.md) |
| `lx file download-file` | 获取文件下载地址 | [entry-file.md](references/entry-file.md) |
| `lx file list-revisions` | 文件历史版本 | [entry-file.md](references/entry-file.md) |
| `lx file revert-file` | 恢复到指定版本 | [entry-file.md](references/entry-file.md) |
| `lx file save-file` | 从 COS URL 注册文件 | [entry-file.md](references/entry-file.md) |
| `lx file create-hyperlink` | 导入外部链接 | [entry-file.md](references/entry-file.md) |

### 草稿管理

| 命令 | 说明 |
|------|------|
| `lx draft describe-markdown-draft` | 获取草稿 | [entry-draft.md](references/entry-draft.md) |
| `lx draft save-markdown-draft` | 保存草稿 | [entry-draft.md](references/entry-draft.md) |
| `lx draft publish-markdown-draft` | 发布草稿 | [entry-draft.md](references/entry-draft.md) |
| `lx draft delete-markdown-draft` | 删除草稿 | [entry-draft.md](references/entry-draft.md) |

### 标签管理

| 命令 | 说明 |
|------|------|
| `lx knowledge-tag list-entry-tags` | 获取条目标签 | [entry-tag.md](references/entry-tag.md) |
| `lx knowledge-tag set-entry-tags` | 设置条目标签（增删） | [entry-tag.md](references/entry-tag.md) |

## 🎯 执行规则

1. **创建一级条目**：必须先通过 `lx space describe-space` 获取 `root_entry_id`，再将其作为 `--parent-entry-id` 传入。
2. **内容编码**：导入内容时 **Agent 必须使用 base64 编码格式**（`markdown_base64` / `html_base64`），避免转义问题。
3. **已有页面优先局部编辑**：若目标页面已存在，**禁止**用 `lx entry import-content-to-entry --force-write` 覆盖，应优先使用 lx-block skill 的高级命令进行局部更新。
4. **文件上传是 3 步流程**：`lx file apply-upload` → HTTP PUT 到 `upload_url` → `lx file commit-upload`，缺一不可。
5. **条目访问链接**：`{domain}/pages/{entry_id}`
6. **`--after-block-id` 限制**：`lx entry import-content-to-entry` 的 `--after-block-id` 只能是页面第一层（根级别）的 block ID，不能是嵌套的子 block。

## 典型组合流程

### 创建页面并导入内容

```bash
# 获取 root_entry_id
lx space describe-space --space-id sp_xxx

# 创建空白页面
lx entry create-entry --parent-entry-id root_xxx --name "新文档" --entry-type page

# 导入内容
lx entry import-content-to-entry \
  --entry-id entry_xxx \
  --content "<base64 内容>" \
  --content-type markdown_base64
```

### 上传文件到知识库

```bash
# Step 1: 获取上传凭证
lx file apply-upload --parent-entry-id folder_xxx --name "report.pdf" --upload-type PRE_SIGNED_URL

# Step 2: HTTP PUT 上传
curl -X PUT "{upload_url}" --data-binary @/path/to/report.pdf

# Step 3: 确认上传
lx file commit-upload --session-id sess_xxx
```

### 浏览文档目录

```bash
# 获取 root_entry_id
lx space describe-space --space-id sp_xxx

# 获取一级目录
lx entry list-children --parent-id root_xxx

# 逐级展开子目录
lx entry list-children --parent-id folder_xxx
```

### 草稿编辑流程

```bash
# 检查是否有未发布草稿
lx draft describe-markdown-draft --entry-id entry_xxx

# 保存草稿
lx draft save-markdown-draft --entry-id entry_xxx --revision-id rev_xxx --content "..." --seq 0

# 发布为正式版本
lx draft publish-markdown-draft --entry-id entry_xxx --revision-id rev_xxx
```

### 管理条目标签

```bash
# 查看现有标签
lx knowledge-tag list-entry-tags --entry-id entry_xxx

# 增删标签
lx knowledge-tag set-entry-tags --entry-id entry_xxx --add-tags "重要" --del-tags "过时"
```
