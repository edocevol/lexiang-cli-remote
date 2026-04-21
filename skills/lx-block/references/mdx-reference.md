# MDX Block 数据格式参考

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解 block 操作的整体决策树。
>
> 本文档基于 xiaokeai MCP Server 的真实 `block_create_block_descendant` schema 定义。

## 概述

MDX（Markdown + JSX）是 AI 生成结构化内容的**推荐输入格式**。CLI 本地解析 MDX → 构造与后端完全兼容的 Block JSON → 调用 MCP 插入。

### 核心流程

```text
AI 产出 .mdx 文件 → CLI parse_mdx() → DocIR → ir_to_descendant()
→ 完整 Block JSON（与 MCP schema 一致）→ block_create_block_descendant() 直接插入
```

---

## 支持的 Block 类型（完整清单）

以下所有类型均来自后端 `block_create_block_descendant` 的 `block_type` 枚举，全部支持。

### 文本块（叶子节点，不支持 children）

这些块的直接子块不能包含其他块。

| block_type | MDX 语法 | content 结构 | 说明 |
|-----------|----------|-------------|------|
| `p` | 普通文本 | `{ elements: [TextElement[]], style?: BlockStyle }` | 默认段落 |
| `h1` ~ `h5` | `#` ~ `#####` | `{ elements: [TextElement[]], style?: BlockStyle }` | 1-5级标题 |
| `code` | \`\`\`language\ncode\n\`\`\` | `{ elements: [TextElement[]], style?: { language, wrap? } }` | 代码块 |
| `divider` | `---` | `{}` | 分割线 |
| `image` | `![alt](url)` | `{ file_id?, caption?, width?, height?, align? }` | 图片 |
| `mermaid` | `<Mermaid>` 组件 | `{ content: "mermaid code" }` | Mermaid 图表 |
| `plantuml` | `<PlantUml>` 组件 | `{ content: "plantuml code" }` | PlantUML 图表 |
| `smartsheet` | `<SmartSheet>` 组件 | `{ name?, smartsheet_id? }` | 智能表格 |
| `video` | `<Video>` 组件 | `{ file_id?, width?, height?, align? }` | 视频 |
| `attachment` | `<Attachment>` 组件 | `{ file_id?, session_id?, view_type?, name? }` | 附件 |

### 容器块（支持 children 嵌套子块）

这些块的内容通过 `children` 数组存储子块。

| block_type | MDX 语法 | content 结构 | children 约束 | 说明 |
|-----------|----------|-------------|-------------|------|
| `callout` | `<Callout icon="🚧" color="red">...children...</Callout>` | `{ color?, icon? }` | 任意块（p/h/list/code/table 等） | **高亮提示框，内容在子块中** |
| `toggle` | `<Toggle>标题</Toggle>` | `{ elements: [TextElement[]], style? }` | 无 | **折叠块（可展开/收起）** |
| `task` | `- [x] 任务名` 或 `<Todo checked={true}>任务名</Todo>` | `{ name?, done, assignees?, due_at? }` | 无 | **任务（含负责人、截止时间）** |
| `bulleted_list` | `- item` | `{ elements: [TextElement[]] }` | 无 | 无序列表 |
| `numbered_list` | `1. item` | `{ elements: [TextElement[]] }` | 无 | 有序列表 |
| `column_list` | `<ColumnList><Column>...</Column></ColumnList>` | `{ column_size? }` | 仅 `column` | **分栏容器** |
| `column` | `<Column ratio={0.5}>...</Column>` | `{ width_ratio? }` | 任意块 | **分栏列（宽度比例为 number）** |
| `table` | GFM 表格语法 | `{ column_size?, column_width[]?, header_row?, header_column?, row_size? }` | 仅 `table_cell` | **表格容器** |
| `table_cell` | 表格单元格 | `{ align?, background_color?, col_span?, row_span?, vertical_align? }` | 任意块（通常 p） | **表格单元格** |

---

## 详细组件说明

### Callout（高亮提示框）

**关键：Callout 是容器类型，实际内容存储在 `children` 子块中，不是内联文本。**

```mdx
<Callout icon="🚧" color="red">
## 注意事项

这是 Callout 内部的一个标题段落。

- 可以包含列表
- 也可以包含代码块

\`\`\`rust
fn example() {}
\`\`\`
</Callout>
```

**对应的 Block JSON：**

```json
{
  "block_type": "callout",
  "block_id": "temp_callout",
  "callout": {
    "color": "red",
    "icon": "🚧"
  },
  "children": ["temp_h2", "temp_p", "temp_list", "temp_code"],
  "descendants": [
    {
      "block_id": "temp_h2",
      "block_type": "h2",
      "heading2": {
        "elements": [{ "text_run": { "content": "注意事项", "text_style": {} } }]
      },
      "children": []
    },
    {
      "block_id": "temp_p",
      "block_type": "p",
      "text": {
        "elements": [{ "text_run": { "content": "这是 Callout 内部的一个标题段落。", "text_style": {} } }]
      },
      "children": []
    }
    // ... 更多子块
  ]
}
```

| 属性 | 类型 | 说明 |
|------|------|------|
| `color` | String | 边框/背景颜色名 |
| `icon` | String | Emoji 图标 |

### Task（任务）

Task 比 Todo 更丰富，支持名称、完成状态、负责人、截止时间。

```mdx
- [x] 完成接口开发
- [ ] 编写单元测试
- [ ] 代码审查
```

或 MDX 组件形式：

```mdx
<Todo checked={true} name="完成接口开发" assignees={["staff_001"]} due_at="2025-04-25"/>
```

**Block JSON：**

```json
{
  "block_type": "task",
  "block_id": "temp_task",
  "task": {
    "name": "完成接口开发",
    "done": true,
    "assignees": [{ "staff_id": "staff_001" }],
    "due_at": { "date": "2025-04-25", "time": "18:00" }
  },
  "children": []
}
```

| 属性 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| `name` | String | 否 | 任务名称（也可用 elements 表示） |
| `done` | Boolean | 是 | 是否完成 |
| `assignees` | Array<{ staff_id }> | 否 | 负责人列表（@人） |
| `due_at` | { date, time } | 否 | 截止时间 |

### Toggle（折叠块）

可展开/收起的折叠区域。

```mdx
<Toggle>
## 点击展开查看详情

这里的内容默认折叠，用户点击后展开显示。
</Toggle>
```

**Block JSON：**

```json
{
  "block_type": "toggle",
  "block_id": "temp_toggle",
  "toggle": {
    "elements": [
      { "text_run": { "content": "点击展开查看详情", "text_style": {} } }
    ]
  },
  "children": []
}
```

### ColumnList / Column（分栏布局）

**Column 和 Callout 一样是容器类型**，分栏内容存在 children 子块中。

```mdx
<ColumnList>
<Column ratio={0.5}>
### 左侧

左侧内容。
</Column>
<Column ratio={0.5}>
### 右侧

右侧内容。
</Column>
</ColumnList>
```

**Block JSON：**

```json
{
  "block_type": "column_list",
  "block_id": "temp_cl",
  "column_list": { "column_size": 2 },
  "children": ["temp_col1", "temp_col2"],
  "descendants": [
    {
      "block_id": "temp_col1",
      "block_type": "column",
      "column": { "width_ratio": 0.5 },
      "children": ["temp_h2_l", "temp_p_l"]
    },
    {
      "block_id": "temp_col2",
      "block_type": "column",
      "column": { "width_ratio": 0.5 },
      "children": ["temp_h2_r", "temp_p_r"]
    }
  ]
}
```

| 属性 | 类型 | 说明 |
|------|------|------|
| `width_ratio` | Number | 宽度比例（0~1 或正数），不是百分比字符串 |
| `column_size` | Number | 列数（需与 children 数量一致） |

### Table（表格）

标准 GFM Markdown 表格语法，支持丰富的表格属性。

```mdx
| 姓名 | 部门 | 状态 |
| ---- | ---- | ---- |
| 张三 | 开发 | 进行中 |
| 李四 | 产品 | 已完成 |
```

**Block JSON（简化）：**

```json
{
  "block_type": "table",
  "block_id": "temp_tbl",
  "table": {
    "column_size": 3,
    "header_row": true,
    "row_size": 3
  },
  "children": ["tc11", "tc12", "tc13", "tc21", "tc22", "tc23", "tc31", "tc32", "tc33"],
  "descendants": [
    // 每个 table_cell: { table_cell: { align, background_color, col_span, row_span, vertical_align }, children: [p 块] }
  ]
}
```

### Mermaid / PlantUML 图表

**一等公民类型，有独立的 content 字段存储图表代码。**

```mdx
<Mermaid>
graph LR
    A --> B --> C
</Mermaid>

<Plantuml>
@startuml
Alice -> Bob: Hello
@enduml
</PlantUml>
```

**Block JSON：**

```json
{
  "block_type": "mermaid",
  "mermaid": { "content": "graph LR\n    A --> B --> C" }
}
```

### Image（图片）

使用 `file_id` 引用已上传的图片文件。

```mdx
![图片描述](https://example.com/img.png)
```

> CLI 导入时会将 URL 转换为 file_id。直接创建时建议用 `<Image file_id="xxx">` 组件。

**Block JSON：**

```json
{
  "block_type": "image",
  "image": {
    "file_id": "file_xxx",
    "caption": "图片描述",
    "width": 800,
    "height": 400,
    "align": "center"
  }
}
```

---

## Text / TextElement 结构（行内样式）

这是最核心的数据结构。所有文本块（p, h1-h5, bulleted_list, numbered_list, toggle 等）的内容都遵循此结构：

```typescript
// Block 级别的文本内容
interface Text {
  elements: TextElement[];        // 文本元素数组
  style?: BlockStyle;            // 块级样式（可选）
}

// 文本元素（可以是纯文本、@人、#文档引用、日期）
interface TextElement {
  text_run?: {                  // 纯文本
    content: string;             // 文本内容
    text_style?: TextStyle;     // 行内样式
  };
  mention_staff?: {             // @人
    staff_id: string;
  };
  mention_entry?: {             // #文档
    entry_id: string;
  };
  mention_date?: {              // 日期
    date: string;
    time?: string;
  };
}

// 行内样式
interface TextStyle {
  bold?: boolean;               // 加粗
  italic?: boolean;              // 斜体
  strikethrough?: boolean;       // 删除线
  underline?: boolean;           // 下划线
  link?: string;                // 链接 URL
  text_color?: string;          // 文字颜色
  background_color?: string;    // 背景颜色
  inline_code?: boolean;         // 行内代码
}

// 块级样式
interface BlockStyle {
  align?: "left" | "center" | "right";   // 对齐方式
  background_color?: string;           // 背景色
  language?: string;                   // 代码语言（仅 code 块）
  wrap?: boolean;                      // 自动换行（仅 code 块）
}
```

**MDX → TextElement 映射示例：**

```mdx
这是 **粗体** 和 *斜体* 以及 ~~删除~~ 的组合。
点击[这里](https://example.com)访问链接。
`inline code` 格式。
```

→ 对应 `elements`:

```json
[
  { "text_run": { "content": "这是 ", "text_style": {} } },
  { "text_run": { "content": "粗体", "text_style": { "bold": true } } },
  { "text_run": { "content": " 和 ", "text_style": {} } },
  { "text_run": { "content": "斜体", "text_style": { "italic": true } } },
  { "text_run": { "content": " 以及 ", "text_style": {} } },
  { "text_run": { "content": "删除", "text_style": { "strikethrough": true } } },
  { "text_run": { "content": " 的组合。", "text_style": {} } },
  { "text_run": { "content": "点击", "text_style": {} } },
  { "text_run": { "content": "这里", "text_style": { "link": "https://example.com" } } },
  { "text_run": { "content": "访问链接。", "text_style": {} } },
  { "text_run": { "content": "`inline code`", "text_style": { "inline_code": true } } },
  { "text_run": { "content": " 格式。", "text_style": {} } }
]
```

---

## 嵌套规则总结

### 容器类型及其可包含的子块类型

```text
Document (根)
├── p, h1-h5, divider, image, code, mermaid, plantuml, video, attachment, smartsheet  ← 叶子节点
├── bulleted_list, numbered_list, task, toggle                                      ← 叶子节点（无 children）
├── callout                                                                          ← 容器：任意块
│   ├── p, h1-h5, code, list, task, toggle, callout(嵌套), table...
├── column_list                                                                      ← 容器：仅 column
│   └── column                                                                         ← 容器：任意块
│       ├── p, h1-h5, code, list, task, callout, table...
└── table                                                                           ← 容器：仅 table_cell
    └── table_cell                                                                     ← 容器：任意块（通常 p）
        └── p, h1-h5, code, list...
```

**重要约束（来自 MCP schema）：**

> 以下类型为**叶子节点**，不支持 children：
> h1, h2, h3, h4, h5（标题）、code（代码块）、image（图片）、attachment（附件）、video（视频）、divider（分割线）、mermaid（Mermaid）、plantuml（PlantUML）、smartsheet（智能表格）

---

## 完整示例：产品需求文档

```mdx
---
title: PRD v2.0
---

# 产品概述

本版本重点改进**性能**和*用户体验*。

<Callout icon="💡" color="blue">
## 设计原则

1. 用户优先
2. 向后兼容
3. 渐进式增强
</Callout>

## 功能列表

<Toggle>
### 核心功能

- [x] Callout 组件重构
- [x] 性能优化（首屏 < 1s）
- [ ] 国际化支持
</Toggle>

<ColumnList>
<Column ratio={0.6}>
### 开发计划

\`\`\`typescript
interface User {
  id: string;
  name: string;
}
\`\`\`

### 时间线

| 阶段 | 时间 | 负责人 |
| ---- | ---- | ---- |
| Design | W1-W2 | Alice |
| Dev | W3-W4 | Bob |
| QA | W5 | Carol |
</Column>

<Column ratio={0.4}>
### 验收标准

- [x] 所有 Case 通过
- [ ] 性能达标
- [ ] 安全审计通过

<Mermaid>
gantt
    dateFormat YYYY-MM-DD
    title 开发计划
    section 设计
    UI设计 :a1, 2025-04-01, 7d
    API设计 :a2, after a1, 5d
    section 开发
    前端开发 :a3, after a2, 14d
    后端开发 :a4, after a2, 14d
</Mermaid>
</Column>
</ColumnList>

---

> **注意：** 以上数据为预估值，以实测为准。
```

---

## 组件能力完整对照表

| 组件 / 特性 | 支持 | Phase | 备注 |
|:------------|:----:|:-----:|------|
| **文本块** ||||
| Paragraph (`p`) | ✅ | P1 | 默认块 |
| Heading (`h1`~`h5`) | ✅ | P1 | 1-5 级 |
| Code (`code`) | ✅ | P1 | 带 language/wrap |
| Divider (`divider`) | ✅ | P1 | 空结构体 |
| Image (`image`) | ✅ | P1 | file_id + caption + size + align |
| **结构化块** ||||
| **Callout** | ✅ | P1 | **容器类型，color + icon，内容在 children 中** |
| **Toggle** | ✅ | P1 | **折叠块，elements + 可选 children** |
| **Task** | ✅ | P1 | **name + done + assignees(@人) + due_at(日期)** |
| BulletedList | ✅ | P1 | elements 内联文本 |
| NumberedList | ✅ | P1 | elements 内联文本 |
| **布局块** ||||
| **ColumnList** | ✅ | P1 | **容器，column_size + column[] 子块** |
| **Column** | ✅ | P1 | **容器，width_ratio(number) + 任意子块** |
| **Table** | ✅ | P1 | **容器，column_size + table_cell[] 子块** |
| **TableCell** | ✅ | P1 | **容器，align/span/color + 任意子块** |
| **图表** ||||
| **Mermaid** | ✅ | P1 | **独立 { content } 结构** |
| **PlantUML** | ✅ | P1 | **独立 { content } 结构** |
| **多媒体** ||||
| **Attachment** | ✅ | P1 | file_id + session_id + view_type |
| **Video** | ✅ | P1 | file_id + size + align |
| **SmartSheet** | ✅ | P1 | smartsheet_id，叶子节点 |
| **行内样式** ||||
| Bold (`**text**`) | ✅ | P1 | `text_style.bold` |
| Italic (`*text*`) | ✅ | P1 | `text_style.italic` |
| Strikethrough (`~~text~~`) | ✅ | P1 | `text_style.strikethrough` |
| Underline | ✅ | P1 | `text_style.underline` |
| Link (`[text](url)`) | ✅ | P1 | `text_style.link` |
| Inline Code (`` ` ``) | ✅ | P1 | `text_style.inline_code` |
| Text Color | ✅ | P1 | `text_style.text_color` |
| Background Color | ✅ | P1 | `text_style.background_color` |
| **@mention (@人)** | ✅ | P1 | `mention_staff: { staff_id }` |
| **#mention (#文档)** | ✅ | P1 | `mention_entry: { entry_id }` |
| **日期 Mention** | ✅ | P1 | `mention_date: { date, time }` |
| **块级样式** ||||
| Align (left/center/right) | ✅ | P1 | `style.align` |
| Background Color | ✅ | P1 | `style.background_color` |
| **Frontmatter** | ✅ | P1 | YAML 元数据头 |

---

## 格式规则速查

1. **组件标签大小写不敏感**：`<Callout>` = `<callout>`
2. **属性用双引号**：`<Callout icon="🚧">`
3. **Callout/Column/TableCell 是容器** — 内容写在内部子块中，不是内联文本
4. **Task 用 GFM `- [x]` 语法更简洁**
5. **Column 宽度用 `ratio={0.5}` 数字比例**（不是字符串 "50%"）
6. **表格必须带 header 分隔行**
7. **Mermaid/PlantUML 使用独立 `<Component>` 标签**，内容为图表代码
8. **叶子节点不能有 children**：h1-h5, code, image, attachment, video, divider, mermaid, plantuml, smartsheet

---

## 参考

- [lx-block SKILL.md](../SKILL.md) — block 操作决策树
- [block-basic.md](block-basic.md) — 原子命令
- [block-advanced.md](block-advanced.md) — 高级命令
