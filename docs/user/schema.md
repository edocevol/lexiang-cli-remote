# 动态 Schema 管理

lx 的命令不是全写死在代码里——大部分从 MCP Schema **动态加载**。MCP 新增 Tool 后，同步 Schema 即可获得新命令，无需等发版。

## 什么时候需要同步

通常安装后即可使用内置 Schema。仅在以下情况需要手动同步：

- MCP Server 新增了 Tool
- 某个分类下的命令缺失
- 需要最新参数定义

```bash
lx tools sync
```

Schema 保存位置：`~/.lexiang/tools/override.json`（优先级高于内置）。

## 常用管理命令

```bash
# 同步最新工具定义
lx tools sync

# 查看分类
lx tools categories

# 某分类下的工具
lx tools list --category team
lx tools list --category entry

# Schema 版本
lx tools version
```

## 发现新命令

```bash
lx --help              # 所有顶层命令
lx team --help         # team 下的子命令
lx team list --help    # 具体参数
```

推荐顺序：先 namespace → 再 action → 最后参数。

## 生成 Agent Skill 文件

导出面向 AI Agent 的说明文件：

```bash
# 输出到默认目录 ~/.lexiang/skills/
lx skill generate

# 指定目录
lx skill generate --output ./my-skills
```

结果包含：

- `README.md` — 总览
- `{namespace}.md` — 每个 namespace 的详细说明
