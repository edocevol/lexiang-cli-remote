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

### 进入场景

- 用户说"创建页面"/"上传文件"/"查看某个文档"
- 用户说"导入 markdown 创建文档"/"管理草稿"
- 用户说"浏览目录树"/"重命名条目"/"移动条目"

### 禁止在本 skill 中执行

- **不要修改页面内部内容**：用户说"改一段内容"/"替换某个章节"/"改表格" → **立即切换到 lx-block skill**
- **不要进行可回滚的批量修改**：多步高风险修改 → **立即切换到 lx-git skill**，先用 `lx git clone` 建立本地工作区
- **不要在知识库中搜索**：用户说"搜索" → **立即切换到 lx-search skill**

## ⚡ 怎么选命令？（决策树）

```text
识别场景 →
├── 创建新页面/文件夹? → lx entry create-entry（需先获取 root_entry_id）
├── 查看/读取文档内容? → lx entry describe-ai-parse-content
├── 浏览目录树? → lx entry list-children（需先拿到 parent_id）
├── 导入 Markdown/HTML?
│   ├── 创建新文档 → lx entry import-content
│   └── 追加到已有页面 → lx entry import-content-to-entry（优先用 lx-block）
├── 上传文件? → 3步流程：apply-upload → HTTP PUT → commit-upload
├── 下载文件? → lx file download-file
├── 管理 Markdown 草稿? → lx draft describe/save/publish-markdown-draft
├── 管理条目标签? → lx knowledge-tag list-entry-tags / set-entry-tags
└── 移动/重命名条目? → lx entry move-entry / rename-entry
```

## ⚠️ 高风险操作与默认路径

**多步修改必须建立 checkpoint：**

- 用户要进行多步编辑、批量修改、或需要可回退的变更管理时
- **必须引导用户使用 lx-git skill**
- 纯在线编辑没有版本记录，一旦覆盖无法回滚

**默认优先路径：**

1. 已有页面内容改动 → 先切到 lx-block skill，**禁止** `import-content-to-entry --force-write`
2. 新建页面后整段导入 → 再使用 `lx entry import-content` / `lx entry import-content-to-entry`
3. 内容导入必须使用 base64 编码 → `markdown_base64` / `html_base64`
4. 文件上传是 3 步流程 → `apply-upload` → HTTP PUT → `commit-upload`

## 可用工具（场景分组）

### 创建与浏览

<!-- TODO: tools entry [] -->

### 内容导入

<!-- TODO: tools entry [] -->

### 文件管理

<!-- TODO: tools file [] -->

### 草稿与标签

<!-- TODO: tools draft [] -->

<!-- TODO: tools knowledge-tag [] -->

## 🎯 执行规则

1. **创建一级条目**：必须先通过 `lx space describe-space` 获取 `root_entry_id`，再将其作为 `--parent-entry-id` 传入。
2. **内容编码**：导入内容时 **Agent 必须使用 base64 编码格式**（`markdown_base64` / `html_base64`），避免转义问题。
3. **已有页面优先局部编辑**：若目标页面已存在，**禁止**用 `lx entry import-content-to-entry --force-write` 覆盖，应优先使用 lx-block skill 的高级命令进行局部更新。
4. **文件上传是 3 步流程**：`lx file apply-upload` → HTTP PUT 到 `upload_url` → `lx file commit-upload`，缺一不可。
5. **条目访问链接**：`{domain}/pages/{entry_id}`
6. **`--after-block-id` 限制**：`lx entry import-content-to-entry` 的 `--after-block-id` 只能是页面第一层（根级别）的 block ID，不能是嵌套的子 block。

## 典型组合流程

### 创建页面并导入内容

1. 获取 root_entry_id - 调用 `lx-space-describe-space` 工具
   - space_id: `sp_xxx`

2. 创建空白页面 - 调用 `lx-entry-create-entry` 工具
   - parent_entry_id: `root_xxx`
   - name: `新文档`
   - entry_type: `page`

3. 导入内容 - 调用 `lx-entry-import-content-to-entry` 工具
   - entry_id: `entry_xxx`
   - content: `<base64 内容>`
   - content_type: `markdown_base64`

### 上传文件到知识库

1. Step 1: 获取上传凭证 - 调用 `lx-file-apply-upload` 工具
   - parent_entry_id: `folder_xxx`
   - name: `report.pdf`
   - upload_type: `PRE_SIGNED_URL`

2. Step 3: 确认上传 - 调用 `lx-file-commit-upload` 工具
   - session_id: `sess_xxx`

### 浏览文档目录

1. 获取 root_entry_id - 调用 `lx-space-describe-space` 工具
   - space_id: `sp_xxx`

2. 获取一级目录 - 调用 `lx-entry-list-children` 工具
   - parent_id: `root_xxx`

3. 逐级展开子目录 - 调用 `lx-entry-list-children` 工具
   - parent_id: `folder_xxx`

### 草稿编辑流程

1. 检查是否有未发布草稿 - 调用 `lx-draft-describe-markdown-draft` 工具
   - entry_id: `entry_xxx`

2. 保存草稿 - 调用 `lx-draft-save-markdown-draft` 工具
   - entry_id: `entry_xxx`
   - revision_id: `rev_xxx`
   - content: `...`
   - seq: `0`

3. 发布为正式版本 - 调用 `lx-draft-publish-markdown-draft` 工具
   - entry_id: `entry_xxx`
   - revision_id: `rev_xxx`

### 管理条目标签

```bash
# 查看现有标签
lx knowledge-tag list-entry-tags --entry-id entry_xxx

# 增删标签
lx knowledge-tag set-entry-tags --entry-id entry_xxx --add-tags "重要" --del-tags "过时"
```
