# block advanced — 高级 Block 命令

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解 block 操作的整体决策树。

高级命令封装了多步 MCP 调用，自动处理块树遍历、内容转换、分块导入等复杂逻辑。**推荐优先使用高级命令**，仅在需要精确控制单个块时回退到[原子命令](block-basic.md)。

## 使用场景

### 表格操作

```bash
# 场景 1：读取表格内容（先确认表格块 ID）
lx block table-get --block-id <TABLE_ID> --format markdown

# 场景 2：修改单元格
lx block table-set --block-id <TABLE_ID> --row 1 --col 2 --text "新值"
# 注意：row 从 0 开始，不含表头

# 场景 3：追加一行数据
lx block table-add-row --block-id <TABLE_ID> --values "张三,开发,进行中"

# 场景 4：删除一行
lx block table-del-row --block-id <TABLE_ID> --row 3
```

### 文档编辑

```bash
# 场景 5：替换文档中某个章节的内容（按标题定位）
lx block replace-section --block-id <ROOT_ID> --heading "## API 参考" \
  --content "更新后的 API 文档内容..."

# 场景 6：从文件替换章节
lx block replace-section --block-id <ROOT_ID> --heading "## 更新日志" \
  --file ./changelog.md

# 场景 7：在指定块后插入内容
lx block insert-after --block-id <BLOCK_ID> --content "## 新章节\n\n内容..."

# 场景 8：追加内容到文档末尾
lx block append --block-id <ROOT_ID> --content "## 附录\n\n补充说明..."
```

### 内容导入导出

```bash
# 场景 9：导出页面为 Markdown
lx block export --block-id <ROOT_ID> --format markdown

# 场景 10：查看页面块结构树
lx block tree --block-id <ROOT_ID> --recursive

# 场景 11：从 Markdown 文件导入到页面（自动分批）
lx block import --block-id <ROOT_ID> --file ./doc.md --chunk-size 20
```

## 关键规则

1. **replace-section 是最常用的编辑命令**：用户说"更新某个章节"时优先用它。它按标题文本匹配定位，替换标题下方所有内容但保留标题本身。
2. **table 操作的行列索引**：`--row` 和 `--col` 都从 0 开始，`--row` 不含表头。修改前建议先 `table-get` 查看当前数据确认索引。
3. **`--content` vs `--file`**：短内容直接用 `--content`，长文档用 `--file` 从文件读取。两者互斥。
4. **export 后再 import**：如果需要大幅重写文档，推荐 `export` 导出 → 本地修改 → `import` 重新导入。
5. **大文档分批**：`import` 的 `--chunk-size` 参数控制分批大小，大文档建议设置（默认 20）。
6. **块 ID 来源**：`tree` 命令可以展示完整的块结构树，包含每个块的 ID、类型、内容摘要。编辑前先 `tree` 定位目标。

## ⚠️ 副作用与风险

- `replace-section`、`table-set`、`table-del-row` 是**写入操作**，执行前确认用户意图
- `replace-section` 只匹配标题文本，如果文档中有多个同名标题，会匹配第一个
- `import` 会在指定块下创建子块，不会清空已有内容。如需替换，先删除再导入
- `table-del-row` 删除行是**不可逆操作**

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx block --help
lx block replace-section --help
lx block table-get --help
# ...
```

## 参考

- [lx-block](../SKILL.md) — block 操作完整决策树
- [block-basic.md](block-basic.md) — 原子 block 操作（需要精确控制单个块时使用）
- [lx-entry](../../lx-entry/SKILL.md) — 条目管理（获取 entry_id / 页面根块 ID）
