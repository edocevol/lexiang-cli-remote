---
name: lx-block
version: 1.0.0
description: "乐享文档块编辑。当用户需要对知识库页面进行结构化编辑（增删改查块、表格操作、章节替换、内容导入导出）时使用。触发词：block、编辑文档、修改内容、表格、块、插入、追加、替换章节"
metadata:
  requires:
    bins: ["lx"]
---

# 文档块编辑

> **前置条件：** 需要 `lx` CLI 已配置并登录。

## ⚡ 什么时候用这个 skill？

**进入场景：**

- 用户说"编辑某个页面"/"修改某个章节"/"改表格"
- 用户说"在文档里插入内容"/"追加内容"
- 用户说"替换某个标题下的内容"/"导入 markdown 到文档"

**禁止在本 skill 中执行：**

- **不要创建新页面**：用户说"创建一个新页面" → **立即切换到 lx-entry skill**
- **不要推送修改到远程**：用户说"推送"/"commit"/"push" → **立即切换到 lx-git skill**
- **不要浏览目录结构**：用户说"浏览知识库"/"看看目录" → **立即切换到 lx-sh skill**

## ⚡ 怎么选命令？（决策树）

```text
├── 修改表格?
│   ├── 读取表格 → lx block table-get
│   ├── 改单元格 → lx block table-set
│   ├── 加一行 → lx block table-add-row
│   └── 删一行 → lx block table-del-row
├── 替换某个章节? → lx block replace-section
├── 在某处插入内容? → lx block insert-after
├── 在页面末尾追加? → lx block append
├── 导入整个 markdown 文件? → lx block import
├── 导出文档内容? → lx block export
├── 查看文档结构? → lx block tree
└── 精细控制单个块?
    ├── 读取块 → lx block describe-block / list-block-children
    ├── 创建块 → lx block create-block-descendant
    ├── 更新块 → lx block update-block / update-blocks
    ├── 删除块 → lx block delete-block / delete-block-children
    ├── 移动块 → lx block move-blocks
    └── 转换内容 → lx block convert-content-to-blocks
```

## ⚠️ 高风险操作与默认优先路径

**高级命令优先，原子命令兜底：**

- 能用高级命令（table-* / replace-section / import 等）就不要读原子命令
- 原子命令只在高级命令无法表达时使用
- 这是最重要的决策规则

**默认优先路径：**

1. 表格操作 → 用 `table-*` 高级命令
2. 章节替换 → 用 `replace-section`
3. 批量导入 → 用 `import --chunk-size 20` 自动分批
4. 精细控制单个块 → 才回退到原子命令

**大文档必须分批：**

- 大文档导入使用 `lx block import --chunk-size 20` 自动分批，避免单次请求过大

## 可用工具

### 高级命令（默认优先使用）

高级命令封装多步操作，自动处理块树遍历、内容转换等复杂逻辑。**能用高级命令就不要读原子命令。**

| 命令 | 说明 | 参考 |
|------|------|------|
| `lx block table-get` | 读取表格结构 | [block-advanced.md](references/block-advanced.md) |
| `lx block table-set` | 修改单元格 | [block-advanced.md](references/block-advanced.md) |
| `lx block table-add-row` | 追加行 | [block-advanced.md](references/block-advanced.md) |
| `lx block table-del-row` | 删除行 | [block-advanced.md](references/block-advanced.md) |
| `lx block replace-section` | 按标题替换章节 | [block-advanced.md](references/block-advanced.md) |
| `lx block insert-after` | 在指定块后插入 | [block-advanced.md](references/block-advanced.md) |
| `lx block append` | 追加到末尾 | [block-advanced.md](references/block-advanced.md) |
| `lx block export` | 导出为 markdown/json | [block-advanced.md](references/block-advanced.md) |
| `lx block tree` | 显示块树结构 | [block-advanced.md](references/block-advanced.md) |
| `lx block import` | 导入 markdown（自动分批）| [block-advanced.md](references/block-advanced.md) |

### 原子命令（仅在高级命令无法表达时使用）

原子命令对应 MCP 接口，适合需要精确控制单个块的场景。**仅在高级命令无法表达时使用。**

| 命令 | 说明 | 参考 |
|------|------|------|
| `lx block describe-block` | 获取块信息 | [block-basic.md](references/block-basic.md) |
| `lx block list-block-children` | 列出子块 | [block-basic.md](references/block-basic.md) |
| `lx block create-block-descendant` | 创建子块 | [block-basic.md](references/block-basic.md) |
| `lx block update-block` | 更新块 | [block-basic.md](references/block-basic.md) |
| `lx block update-blocks` | 批量更新块 | [block-basic.md](references/block-basic.md) |
| `lx block delete-block` | 删除块 | [block-basic.md](references/block-basic.md) |
| `lx block delete-block-children` | 批量删除子块 | [block-basic.md](references/block-basic.md) |
| `lx block move-blocks` | 移动块 | [block-basic.md](references/block-basic.md) |
| `lx block convert-content-to-blocks` | 内容转换为块结构 | [block-basic.md](references/block-basic.md) |

## 🎯 执行规则

1. **高级命令优先**：表格操作、章节替换、批量导入等场景，优先使用 `lx block` 高级命令。只有需要精确控制单个块时才回退到原子命令。
2. **块移动限制**：`lx block move-blocks` 的目标父节点不能是叶子节点类型（h1-h5、code、image、attachment、video、divider、mermaid、plantuml）。
3. **大文档分批**：大文档导入使用 `lx block import --chunk-size 20` 自动分批，避免单次请求过大。
4. **批量更新**：修改多个块时用 `lx block update-blocks` 而非多次调用 `lx block update-block`。每个块在单次请求中只能执行一种更新操作。
5. **内容转换流程**：需要从 Markdown/HTML 创建块时，先调 `lx block convert-content-to-blocks` 转换，再用 `lx block create-block-descendant` 插入。
6. **命令调度**：高级命令与原子命令共存于 `lx block` 命名空间。高级命令名优先匹配，未匹配时自动回退到动态生成的原子命令。

## 典型组合流程

### 修改表格单元格

```bash
# 查看当前状态
lx block table-get --block-id tbl_xxx --format table

# 修改
lx block table-set --block-id tbl_xxx --row 2 --col 1 --text "修正值"

# 验证结果
lx block table-get --block-id tbl_xxx --format json
```

### 替换文档中的某个章节

```bash
# 查看文档树，定位目标章节
lx block tree --block-id root_xxx --recursive

# 一键替换
lx block replace-section --block-id root_xxx --heading "## API 参考" \
  --file ./updated-api.md
```

### 使用原子命令精细编辑

```bash
# 获取完整块树
lx block list-block-children --entry-id entry_xxx --with-descendants

# 更新目标块内容
lx block update-block --entry-id entry_xxx --block-id blk_xxx \
  --update-text '{"elements": [{"text": {"content": "新内容"}}]}'

# 在指定位置插入新块（先转换 markdown）
lx block convert-content-to-blocks --content "## 新章节" --content-type markdown
lx block create-block-descendant --entry-id entry_xxx --parent-block-id page_xxx \
  --descendant '<转换结果>'
```

### 导入 Markdown 创建文档内容

```bash
# 直接从文件导入（推荐）
lx block import --block-id page_xxx --file ./doc.md --chunk-size 20
```
