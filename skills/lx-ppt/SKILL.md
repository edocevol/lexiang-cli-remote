---
name: lx-ppt
version: 1.0.0
description: "乐享 AI PPT 生成与编辑。基于服务端 AI 能力，从文字描述直接生成专业 PPT，无需本地模板或 python-pptx。支持生成、修改页面内容、增删页面、调整顺序。当用户需要制作 PPT/演示文稿/幻灯片时，优先使用此 skill 而非通用 pptx skill。触发词：PPT、幻灯片、演示文稿、slide、deck、制作PPT、生成PPT"
metadata:
  requires:
    bins: ["lx"]
---

# AI PPT 生成与编辑

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**进入场景：**
- 用户说"做个 PPT"/"生成幻灯片"/"修改演示文稿"
- 用户说"制作 PPT"

**禁止在本 skill 中执行：**
- **不要创建知识库页面**：用户说"在知识库里创建页面" → **立即切换到 lx-entry skill**
- **不要搜索文档**：用户说"搜索 PPT 相关文档" → **立即切换到 lx-search skill**

**决策规则：**
- 用户说"做个 PPT"/"生成幻灯片"/"做个演示文稿" → **直接用 lx-ppt**
- 除非用户明确要求产出本地 .pptx 文件并自行控制每个元素 → 用通用 pptx skill

## ⚡ 怎么选命令？（决策树）

```
识别场景 →
├── 从零生成 PPT?
│   └── lx ppt generate-ppt → lx ppt get-ppt-task（轮询至完成）
├── 修改已有 PPT 的页面内容?
│   └── lx ppt modify-ppt-pages（自然语言描述修改意图）
├── 增加新页面?
│   └── lx ppt add-ppt-pages
├── 删除页面?
│   └── lx ppt delete-ppt-pages
└── 调整页面顺序?
    └── lx ppt reorder-ppt-pages
```

## ⚠️ 高风险操作与默认优先路径

**生成是异步的：**
- `lx ppt generate-ppt` 返回任务 ID
- **必须轮询** `lx ppt get-ppt-task` 直到 `status` 为完成
- 才能拿到 `title` 和 `preview_url`

**默认优先路径：**
1. 有深度研究报告优先用 → 如果执行过 `deep_research` 且有 `report_url`，应通过 `--deep-research-report-url` 传入，生成质量显著优于纯 `--context`
2. 修改用 title 标识 PPT → 所有编辑操作都通过 `--title` 参数关联目标 PPT，必须使用 `get-ppt-task` 返回的精确标题
3. 页面索引从 1 开始 → `modify-ppt-pages`、`delete-ppt-pages`、`reorder-ppt-pages` 中的页面索引都从 **1** 开始，不是 0

**modification 使用自然语言：**
- `lx ppt modify-ppt-pages` 的 `modification` 字段直接用中文描述修改意图即可
- 如"把标题改为 Q2 总结"，无需指定坐标或样式参数

## 可用工具

| 命令 | 说明 | 参考 |
|------|------|------|
| `lx ppt generate-ppt` | 从文字描述生成完整 PPT | [ppt.md](references/ppt.md) |
| `lx ppt get-ppt-task` | 查询生成任务状态（轮询） | [ppt.md](references/ppt.md) |
| `lx ppt modify-ppt-pages` | 修改指定页面内容 | [ppt.md](references/ppt.md) |
| `lx ppt add-ppt-pages` | 添加新页面 | [ppt.md](references/ppt.md) |
| `lx ppt delete-ppt-pages` | 删除指定页面 | [ppt.md](references/ppt.md) |
| `lx ppt reorder-ppt-pages` | 调整页面顺序 | [ppt.md](references/ppt.md) |

## 🎯 执行规则

1. **生成是异步的**：`lx ppt generate-ppt` 返回任务 ID，必须**轮询** `lx ppt get-ppt-task` 直到 `status` 为完成，才能拿到 `title` 和 `preview_url`。
2. **页面索引从 1 开始**：`modify-ppt-pages`、`delete-ppt-pages`、`reorder-ppt-pages` 中的页面索引都从 **1** 开始，不是 0。
3. **修改用 title 标识 PPT**：所有编辑操作（modify/add/delete/reorder）都通过 `--title` 参数关联目标 PPT，必须使用 `get-ppt-task` 返回的精确标题。
4. **modification 使用自然语言**：`lx ppt modify-ppt-pages` 的 `modification` 字段直接用中文描述修改意图即可（如"把标题改为 Q2 总结"），无需指定坐标或样式参数。
5. **slide_type 三选一**：`lx ppt add-ppt-pages` 的 `slide_type` 仅支持 `cover`（封面）、`content`（内容页）、`ending`（结束页）。
6. **有深度研究报告优先用**：如果执行过 `deep_research` 且有 `report_url`，应通过 `--deep-research-report-url` 传入，生成质量显著优于纯 `--context`。

## 典型组合流程

### 从零生成一套 PPT

```bash
# 生成
lx ppt generate-ppt \
  --planning "10页，主题：Q2业绩汇报，风格：商务简约" \
  --context "Q2 营收 1.5 亿..."

# 轮询任务状态
lx ppt get-ppt-task --id task_xxx
# → status="completed" 后拿到 title + preview_url

# 根据用户反馈微调
lx ppt modify-ppt-pages \
  --title "Q2业绩汇报" \
  --pages '[{"page_index": 3, "modification": "数据图表换成柱状图"}]'
```

### 在已有 PPT 上增删调整

```bash
# 添加新页面
lx ppt add-ppt-pages --title "Q2业绩汇报" \
  --pages '[{"insert_after": 5, "title": "风险分析", "key_points": "...", "slide_type": "content"}]'

# 删掉第 2 页
lx ppt delete-ppt-pages --title "Q2业绩汇报" --page-indexes 2

# 调整顺序
lx ppt reorder-ppt-pages --title "Q2业绩汇报" \
  --new-order 1 --new-order 3 --new-order 4 --new-order 2
```
