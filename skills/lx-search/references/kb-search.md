# search — 搜索与发现

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解搜索 skill 的整体决策树。

知识库资源的统一发现入口。提供两种互补的搜索模式：**关键词精确匹配** (`lx search kb-search`) 和**语义向量召回** (`lx search kb-embedding-search`)。大多数场景从关键词搜索开始；当用户意图模糊、使用自然语言提问、或关键词搜索无结果时，切换到向量搜索。

## 使用场景

### 用户给出明确关键词

```bash
lx search kb-search --keyword "API 设计规范"
```

### 限定知识库、只搜标题

```bash
lx search kb-search --keyword "周报" --space-id sp_xxx --title-only
```

### 按最近编辑排序

```bash
lx search kb-search --keyword "发版计划" --sort-by "-edited_at"
```

### 用户描述模糊意图，使用语义搜索

```bash
lx search kb-embedding-search --keyword "如何申请服务器资源"
```

### 关键词无结果，回退到语义搜索

```bash
# 先精确搜索
lx search kb-search --keyword "连接池"
# 无结果 → 切换语义搜索
lx search kb-embedding-search --keyword "数据库连接池配置最佳实践"
```

### 查询同事信息

```bash
lx contact search-staff --staff-id "张三" --fuzzy-search
```

### 确认当前登录身份

```bash
lx contact whoami
```

## 关键规则

1. **关键词 vs 向量**：用户给出具体名词/标题 → `lx search kb-search`；用户提问/描述需求 → `lx search kb-embedding-search`。不确定时先关键词，无结果再向量。
2. **缩小范围**：知道目标知识库时传 `--space-id`，知道团队时传 `--team-id`，只搜标题时加 `--title-only`。越精确越好。
3. **翻页策略**：默认只返回第一页。单轮最多拉取 3 页，达到上限后询问用户是否继续。
4. **搜索后续**：拿到 `entry_id` 后，下一步通常是 `lx entry describe-entry`（查详情）或 `lx entry describe-ai-parse-content`（读内容）。不要搜到就停，要完成用户的实际意图。
5. **人员查询**：`lx contact search-staff` 依赖企业通讯录权限，若报错应告知"可能是企业通讯录未开放"。
6. **身份确认**：如果不确定当前用户身份，先 `lx contact whoami`。

## ⚠️ 副作用与风险

- `--type` 参数决定搜索范围，不要遗漏——搜知识库时用 `space`，搜文档时用 `kb_doc`
- 向量搜索和关键词搜索的 `--keyword` 含义不同：前者接受自然语言长句，后者接受精确关键词
- 搜索结果中的 `entry_id` 是后续一切操作的入口，务必保留
- 人员搜索若企业未开启通讯录可见，接口会报错

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx search --help
lx contact --help
```

## 参考

- [lx-search](../SKILL.md) — 搜索 skill 完整决策树
- [lx-entry](../../lx-entry/SKILL.md) — 拿到 entry_id 后的条目操作
- [lx-space](../../lx-space/SKILL.md) — 知识库与团队管理
