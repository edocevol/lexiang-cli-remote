# meeting — 腾讯会议导入

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解连接器 skill 的整体决策树。

搜索腾讯会议记录并导入到知识库。典型流程：先搜索找到会议记录 → 用户确认 → 导入到指定位置。

## 使用场景

### 搜索最近的会议记录

```bash
lx meeting search-tx-meeting-records --meeting-code "123456789"
```

### 按时间范围搜索

```bash
lx meeting search-tx-meeting-records \
  --start-time "1700000000" \
  --end-time "1700100000"
```

### 列出会议录制记录

```bash
lx meeting list-tx-meeting-records \
  --meeting-code "123456789" \
  --limit 10
```

### 查看录制详情

```bash
lx meeting describe-tx-meeting-record --record-id rec_xxx
```

### 导入会议记录到知识库

```bash
lx meeting import-tx-meeting-record \
  --record-file-id rec_file_xxx \
  --parent-entry-id folder_xxx \
  --start-time "1700000000" \
  --end-time "1700100000"
```

### 重新加载已导入的会议记录

```bash
lx meeting reload-tx-meeting-record \
  --record-id rec_xxx \
  --entry-id entry_xxx
```

## 关键规则

1. **先搜再导**：不要直接导入。先 `lx meeting search-tx-meeting-records` 或 `lx meeting list-tx-meeting-records` 展示结果让用户确认要导入哪条会议记录。
2. **时间格式**：`--start-time` 和 `--end-time` 使用**时间戳**（秒级），不是 ISO 8601 格式。
3. **导入位置**：`--parent-entry-id` 需要传。如果用户没指定位置，先引导用户选择目标知识库和文件夹。
4. **导入结果**：导入后会在目标位置创建新的页面条目，包含会议的完整记录。

## ⚠️ 副作用与风险

- 导入是**写入操作**，会在知识库中创建新条目。执行前确认用户选择的会议记录和目标位置。
- 时间参数用时间戳（秒），不是人类可读格式，注意单位。

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx meeting --help
```

## 参考

- [lx-connector](../SKILL.md) — 连接器 skill 完整决策树
- [lx-entry](../../lx-entry/SKILL.md) — 条目操作（获取 parent_entry_id）
- [lx-space](../../lx-space/SKILL.md) — 知识库操作（获取 space_id）
