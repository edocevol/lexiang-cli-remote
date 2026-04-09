# ppt — PPT 生成与编辑

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解 PPT skill 的整体决策树和与通用 pptx skill 的差异。

服务端 AI 驱动的 PPT 全生命周期管理：生成 → 轮询 → 编辑（修改/添加/删除/重排页面）。用户只需描述意图，AI 自动处理排版、设计、配色。

## 使用场景

### 从自然语言描述生成 PPT

```bash
lx ppt generate-ppt \
  --planning "10 页的产品介绍 PPT，风格简洁商务，涵盖产品背景、核心功能、竞争优势、路线图"
# → 返回任务 id，需要轮询
```

### 带参考内容生成

```bash
lx ppt generate-ppt \
  --planning "8 页的季度总结汇报" \
  --context "Q3 营收 1.2 亿，同比增长 30%..."
```

### 使用深度研究报告生成

```bash
lx ppt generate-ppt \
  --planning "基于报告生成 12 页技术架构演讲" \
  --deep-research-report-url "https://cos.xxx/report.md"
```

### 使用企业模板

```bash
lx ppt generate-ppt \
  --planning "公司介绍" \
  --use-template tencent_cloud_template
```

### 轮询任务状态

```bash
# 生成是异步的，必须轮询直到完成
lx ppt get-ppt-task --id task_xxx
# → status: "running" | "completed" | "failed"
# → 完成后拿到 result.title（后续编辑必须用这个）
# → result.preview_url 给用户预览
```

### 修改指定页面内容（自然语言描述）

```bash
lx ppt modify-ppt-pages \
  --title "产品介绍" \
  --pages '[{"page_index": 3, "modification": "将标题改为核心优势，补充3个优势要点"}]'
```

### 添加新页面

```bash
lx ppt add-ppt-pages \
  --title "产品介绍" \
  --pages '[{"insert_after": 5, "title": "客户案例", "subtitle": "Top 3 客户", "key_points": "案例A...", "slide_type": "content"}]'
```

### 删除页面

```bash
lx ppt delete-ppt-pages --title "产品介绍" --page-indexes 8 --page-indexes 9
```

### 调整页面顺序

```bash
lx ppt reorder-ppt-pages --title "产品介绍" --new-order 1 --new-order 3 --new-order 2 --new-order 4 --new-order 5
# 原来的 1,2,3 变成 1,3,2
```

## 完整工作流

```bash
# Step 1: 生成
lx ppt generate-ppt --planning "..."
# → 拿到 task_id

# Step 2: 轮询直到完成（每 3-5 秒）
lx ppt get-ppt-task --id task_xxx
# → status="completed" → 拿到 title, preview_url

# Step 3: 展示预览给用户，收集反馈

# Step 4: 根据反馈修改
lx ppt modify-ppt-pages --title "产品介绍" --pages '[...]'

# Step 5: 如需补充页面
lx ppt add-ppt-pages --title "产品介绍" --pages '[...]'

# Step 6: 如需删减
lx ppt delete-ppt-pages --title "产品介绍" --page-indexes 8
```

## 关键规则

1. **生成是异步的**：`lx ppt generate-ppt` 返回任务 ID，必须用 `lx ppt get-ppt-task` 轮询。轮询间隔建议 3-5 秒，超过 2 分钟未完成应告知用户等待。
2. **title 是编辑操作的主键**：所有编辑操作（modify/add/delete/reorder）的 `--title` 参数**必须与 `get-ppt-task` 返回的 `result.title` 完全一致**。不要让用户自己输入标题。
3. **页面索引从 1 开始**：`page_index` 和 `--page-indexes` 都从 1 开始计数，不是 0。
4. **修改用自然语言**：`lx ppt modify-ppt-pages` 的 `modification` 字段接受自然语言描述——"把标题改成XX"、"添加一段关于XX的内容"。AI 自动理解并执行。
5. **context vs deep-research-report-url**：如果已有深度研究报告的 COS URL，优先用 `--deep-research-report-url`（效果更好）。否则用 `--context` 传入参考文本。
6. **每次编辑后展示预览**：编辑操作返回 `edit_result.preview_url`，务必展示给用户确认效果。

## ⚠️ 副作用与风险

- 生成和编辑操作都是**写入操作**，确认用户意图后再执行
- `--title` 参数是所有编辑操作的主键标识，**不要修改或自行构造**——必须使用 `get-ppt-task` 返回的原值
- 页面索引从 1 开始，不是 0。传 0 会导致错误
- `--new-order` 数组长度必须等于当前总页数，且包含所有原始页面索引
- 删除不可撤销

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx ppt --help
lx ppt generate-ppt --help
lx ppt get-ppt-task --help
# ...
```

## 参考

- [lx-ppt](../SKILL.md) — PPT skill 完整决策树和与通用 pptx skill 的对比
