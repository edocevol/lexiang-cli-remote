# 命令参考

## 全局参数

大多数动态命令支持：

| 参数 | 说明 |
|------|------|
| `-o, --format <FORMAT>` | 输出格式：json/json-pretty/table/yaml/csv/markdown |
| `-d, --data-raw <JSON>` | 用 JSON 一次性传入全部参数 |
| `-h, --help` | 查看帮助 |

两种传参方式：

```bash
# 方式 1：逐参数传入（日常使用）
lx search kb --keyword "test" --limit 10 --type doc

# 方式 2：JSON 传入（脚本/复杂参数）
lx search kb -d '{"keyword":"test","limit":10,"type":"doc"}'
```

## 命令总览

| Namespace | 用途 | 常用命令 |
|-----------|------|---------|
| `search` | 搜索知识 | `lx search kb`, `lx search kb-embedding` |
| `team` | 团队信息 | `lx team list`, `lx team describe` |
| `space` | 知识库信息 | `lx space list`, `lx space describe` |
| `entry` | 条目与页面 | `lx entry describe`, `lx entry create` |
| `block` | 文档块操作 | `lx block ls`, `lx block get`, `lx block create` |
| `file` | 文件上传下载 | `lx file apply-upload`, `lx file download` |
| `comment` | 评论查询 | `lx comment list` |
| `ppt` | PPT 服务 | `lx ppt generate` |
| `meeting` | 会议录制 | `lx meeting import` |
| `contact` | 联系人 | `lx contact whoami` |

## 搜索

```bash
# 全局搜索
lx search kb --keyword "关键词"

# 指定知识库
lx search kb --keyword "关键词" --space-id <SPACE_ID>

# 指定类型
lx search kb --keyword "关键词" --type doc
lx search kb --keyword "关键词" --type kb_doc
lx search kb --keyword "关键词" --title-only

# 向量语义检索
lx search kb-embedding --keyword "如何部署服务"
```

## 团队与知识库

```bash
# 团队
lx team list
lx team list-frequent
lx team describe --team-id <TEAM_ID>

# 知识库
lx space list --team-id <TEAM_ID>
lx space list-recently
lx space describe --space-id <SPACE_ID>
```

`lx space describe` 返回的 `root_entry_id` 用于后续遍历目录。

## 条目

```bash
# 查看条目详情
lx entry describe --entry-id <ENTRY_ID>

# 遍历目录
lx entry list-children --parent-id <PARENT_ID>
lx entry list-latest --space-id <SPACE_ID>
lx entry list-parents --entry-id <ENTRY_ID>

# 创建
lx entry create --parent-entry-id <PARENT_ID> --name "新文档" --entry-type page
lx entry create --parent-entry-id <PARENT_ID> --name "新文件夹" --entry-type folder

# 导入内容
lx entry import-content --parent-id <PARENT_ID> --name "导入的文档" --content "# 标题\n\n正文"
lx entry import-content-to-entry --entry-id <ENTRY_ID> --content "追加的内容"

# 移动重命名
lx entry rename --entry-id <ENTRY_ID> --name "新名称"
lx entry move --entry-id <ENTRY_ID> --parent-entry-id <NEW_PARENT_ID>

# AI 可解析内容（Markdown/HTML）
lx entry describe-ai-parse-content --entry-id <ENTRY_ID>
```

## 文档块（Block）

细粒度修改在线文档内容。

### 动态命令（基础操作）

```bash
lx block describe --block-id <BLOCK_ID>
lx block list-children --block-id <BLOCK_ID>
lx block list-children --block-id <BLOCK_ID> --recursive
lx block update -d '{"block_id":"xxx","content":{"text":"新内容"}}'
lx block create-descendant -d '{"block_id":"xxx","descendant":{...}}'
lx block delete --block-id <BLOCK_ID>
lx block delete-children -d '{"block_id":"xxx","children_ids":["id1","id2"]}'
lx block move -d '{"block_ids":["id1"],"parent_block_id":"xxx"}'
lx block convert-content-to-blocks -d '{"content":"# 标题","content_type":"markdown"}'
```

### 静态命令（增强操作，推荐）

```bash
# 列出子块（树形展示）
lx block ls --block-id <BLOCK_ID>

# 获取块内容（支持 MDX 输出）
lx block get --block-id <BLOCK_ID>
lx block get --block-id <BLOCK_ID> --format mdx

# 创建子块（自动 MDX→blocks 转换）
lx block create --block-id <BLOCK_ID> --content "# 标题\n\n正文"
lx block create --block-id <BLOCK_ID> --file ./doc.mdx

# 更新块
lx block update --block-id <BLOCK_ID> --text "新文本"
lx block update --block-id <BLOCK_ID> --content "$(cat doc.mdx)"

# 删除/移动
lx block delete --block-id <BLOCK_ID>
lx block move --block-ids id1,id2 --parent-block-id <TARGET_ID>

# 转换预览
lx block convert --content "# 标题" --from mdx --to blocks
```

> 静态命令优先于动态命令。当 `lx block <subcmd>` 匹配到静态实现时，不会走动态 MCP 调用。
> 见 [架构文档](../dev/module-boundaries.md) 的「规则 1：静态优先于动态」。

## 文件

```bash
# 查看
lx file describe --file-id <FILE_ID>
lx file download --file-id <FILE_ID>

# 上传（三步走）
lx file apply-upload --parent-entry-id <PARENT_ID>   # 1. 申请凭证
curl -X PUT "<upload_url>" --data-binary @file.pdf     # 2. 上传文件
lx file commit-upload --session-id <SESSION_ID>        # 3. 确认

# 导入链接
lx file create-hyperlink --url "https://..." --space-id <SID> --parent-entry-id <PID>
```

## 其他业务 Tool

```bash
# 评论
lx comment list --target-id <ENTRY_ID>
lx comment describe --target-id <ENTRY_ID>

# PPT
lx ppt generate -d '{"planning":"10页产品介绍","context":"..."}'
lx ppt get-task --id <TASK_ID>
lx ppt add-pages / modify-pages / delete-pages / reorder-pages

# 会议
lx meeting search --meeting-id <CODE>
lx meeting describe --record-id <RECORD_ID>
lx meeting import -d '{"record_id":"xxx", ...}'

# 联系人
lx contact search-staff --keyword "张三"
lx contact whoami

# iWiki
lx iwiki import -d '{"page_id":"123", ...}'
```

## 低层调试：`lx mcp`

绕过动态命令，直接调用 MCP Tool：

```bash
lx mcp list                          # 列出所有可用工具
lx mcp call entry_describe --params '{"entry_id":"xxx"}'  # 直接调用
```
