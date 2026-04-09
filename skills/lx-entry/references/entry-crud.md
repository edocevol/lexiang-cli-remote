# entry CRUD — 条目增删改查

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解条目管理的整体决策树。

知识库条目（页面、文件夹）的基础操作：创建、查询、浏览、移动、重命名。这是知识库操作中最常用的一组命令。

## 使用场景

### 在知识库中创建新页面

```bash
lx entry create-entry --entry-type page --parent-entry-id root_xxx --name "项目计划书"
# → 返回 entry 对象，entry.id 可构建链接 {domain}/pages/{entry_id}
```

### 在指定文件夹下创建子页面

```bash
lx entry create-entry --entry-type page --parent-entry-id folder_xxx --name "第一周迭代"
```

### 用户给了页面链接，查看详情

```bash
# 从 URL {domain}/pages/{entry_id} 提取 entry_id
lx entry describe-entry --entry-id entry_xxx
# → 返回 entry_type, name, space_id, has_children, target_id
```

### 读取页面内容给 AI 处理

```bash
lx entry describe-ai-parse-content --entry-id entry_xxx
# → 返回 HTML/markdown/OCR 格式内容
```

### 浏览知识库目录结构

```bash
# 先拿 root_entry_id（见 lx-space），再逐级展开
lx entry list-children --parent-id root_xxx

# 展开子目录，按最近编辑排序
lx entry list-children --parent-id folder_xxx --sort-by "-edited_at"
```

### 确认条目位置（面包屑）

```bash
lx entry list-parents --entry-id entry_xxx
# → 返回 [root, folder_a, folder_b, entry_xxx]
```

### 查看最近更新

```bash
lx entry list-latest-entries --space-id sp_xxx
```

### 移动条目

```bash
lx entry move-entry --entry-id entry_xxx --parent-id target_folder_xxx
```

### 重命名条目

```bash
lx entry rename-entry --entry-id entry_xxx --name "新名称"
```

## 关键规则

1. **创建前确认位置**：创建条目必须传 `--parent-entry-id`。如果用户说"在知识库根目录创建"，需要先通过 `lx space describe-space` 拿到 `root_entry_id`。
2. **查看 vs 阅读**：`lx entry describe-entry` 返回元信息（类型、名称、所属库）；`lx entry describe-ai-parse-content` 返回实际内容（HTML/markdown）。查详情用前者，读内容用后者。
3. **URL 解析**：页面链接格式 `{domain}/pages/{entry_id}`，提取 `entry_id` 后直接使用。
4. **浏览目录**：`lx entry list-children` 返回的 `has_children` 字段决定是否可以继续展开。`entry_type` 区分 page/folder/file。
5. **移动/重命名是写入操作**：执行前确认用户意图，避免误操作。
6. **file 类型条目**：`lx entry describe-entry` 返回的 `target_id` 就是 `file_id`，可用于文件下载/更新（见 [entry-file.md](entry-file.md)）。

## ⚠️ 副作用与风险

- 创建条目是**写入操作**。创建前确认用户意图——尤其是 `--parent-entry-id` 是否正确。
- 移动操作会改变条目在知识库中的位置和层级关系，执行前确认目标父级正确。
- `lx entry list-children` 返回的 `entry_type` 决定后续操作方式：page 可读内容，file 可下载/更新，folder 可继续展开。

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx entry --help
lx entry create-entry --help
lx entry describe-entry --help
# ...
```

## 参考

- [lx-entry](../SKILL.md) — 条目 skill 完整决策树
- [entry-import.md](entry-import.md) — 导入内容创建/追加文档
- [entry-file.md](entry-file.md) — 文件上传与管理
- [entry-draft.md](entry-draft.md) — Markdown 草稿管理
- [entry-tag.md](entry-tag.md) — 标签管理
- [lx-space](../../lx-space/SKILL.md) — 获取 root_entry_id
