# entry import — 内容导入

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解条目管理的整体决策树。

将 Markdown 或 HTML 内容导入知识库，创建新文档或向已有页面追加内容。这是**批量写入**的主要方式——适合一次性导入完整文档，而不是逐块编辑。

## 使用场景

### 从 Markdown 创建新文档

```bash
lx entry import-content \
  --content "<base64 编码的 Markdown 内容>" \
  --content-type markdown_base64 \
  --name "项目计划书" \
  --parent-id root_xxx
```

### 在指定知识库根节点下创建

```bash
lx entry import-content \
  --content "<base64 编码的内容>" \
  --content-type markdown_base64 \
  --name "新文档" \
  --space-id sp_xxx
```

### 向已有页面末尾追加内容

```bash
lx entry import-content-to-entry \
  --entry-id entry_xxx \
  --content "<base64 编码的内容>" \
  --content-type markdown_base64
```

### 向指定位置插入内容

```bash
lx entry import-content-to-entry \
  --entry-id entry_xxx \
  --content "<base64 编码的内容>" \
  --content-type markdown_base64 \
  --after-block-id block_xxx
```

> ⚠️ `--after-block-id` **只能**是页面第一层（根级别）的 block ID，不能是嵌套子 block。

### 完全重写已有页面（危险操作）

```bash
lx entry import-content-to-entry \
  --entry-id entry_xxx \
  --content "<base64 编码的全新内容>" \
  --content-type markdown_base64 \
  --force-write
# ⚠️ 会清空页面所有现有内容！
```

## 关键规则

1. **必须使用 base64 编码**：`--content-type` 应设为 `markdown_base64` 或 `html_base64`。原因：Markdown/HTML 中的特殊字符在传输中会被转义导致内容损坏。**绝对不要用** `markdown` 或 `html` 纯文本模式。
2. **新建 vs 追加**：需要创建新页面 → `lx entry import-content`；向已有页面写入 → `lx entry import-content-to-entry`。
3. **追加 vs 覆盖**：默认追加到末尾。`--force-write` 会**清空页面所有内容**再写入——这是破坏性操作，必须确认用户意图。
4. **局部更新优先**：如果目标页面已有内容且只需修改部分，**不要用 import 覆盖**，应使用 `lx block replace-section` / `lx block append` 等高级命令进行精确编辑。
5. **after-block-id 限制**：只能是页面**第一层**（根级别）的 block ID。如果不确定块的层级，先用 `lx block tree --block-id <PAGE_ID> --recursive` 查看页面块结构。

## ⚠️ 副作用与风险

- `lx entry import-content` 和 `lx entry import-content-to-entry` 是**写入操作**，执行前确认用户意图
- `--force-write` 会**不可逆地清空**页面现有内容，除非用户明确要求"重写"或"覆盖"，否则不要使用
- `--after-block-id` 必须是页面根级别 block，传子级 block 会导致位置异常
- 内容格式支持标准 Markdown 和 HTML，不支持私有标记语法

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx entry --help
lx entry import-content --help
lx entry import-content-to-entry --help
```

## 参考

- [lx-entry](../SKILL.md) — 条目 skill 完整决策树
- [entry-crud.md](entry-crud.md) — 条目基础 CRUD
- [lx-block](../../lx-block/SKILL.md) — 需要精确编辑已有页面时使用 block 操作
