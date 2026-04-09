# entry file — 文件管理与上传

> **前置条件：** 先阅读 [`../SKILL.md`](../SKILL.md) 了解条目管理的整体决策树。

知识库中的文件管理：上传新文件、更新已有文件、下载、版本控制、从 URL 注册文件、导入外部链接。核心操作是**三步文件上传**——这个流程不可跳步。

## 使用场景

### 上传新文件到知识库（完整 3 步）

**Step 1 — 申请上传凭证**
```bash
lx file apply-upload \
  --parent-entry-id folder_xxx \
  --name "report.pdf" \
  --upload-type PRE_SIGNED_URL
# → 返回 session.upload_url 和 session.session_id
```

**Step 2 — HTTP PUT 上传文件（非 lx 命令，直接执行）**
```bash
curl -X PUT "{upload_url}" --data-binary @/path/to/report.pdf
```

**Step 3 — 确认上传完成**
```bash
lx file commit-upload --session-id sess_xxx
# → 文件条目正式创建，返回 entry 对象
```

### 更新已有文件（重新上传）

```bash
# 先获取 file_id（target_id 就是 file_id）
lx entry describe-entry --entry-id file_entry_xxx

# 再申请上传（注意参数差异）
lx file apply-upload \
  --parent-entry-id file_entry_xxx \
  --name "report_v2.pdf" \
  --upload-type PRE_SIGNED_URL \
  --file-id file_xxx
# → 后续同上：PUT + commit
```

> ⚠️ 更新文件时 `--parent-entry-id` 填的是**文件条目自己的 entry_id**，不是父文件夹——这是最常见的错误。

### 文件已在 COS 上，直接注册

```bash
lx file save-file \
  --parent-entry-id folder_xxx \
  --name "数据分析.xlsx" \
  --url "https://cos.xxx/data.xlsx"
```

### 导入外部链接

```bash
lx file create-hyperlink \
  --url "https://mp.weixin.qq.com/s/xxx" \
  --parent-entry-id folder_xxx
```

### 下载文件

```bash
lx file download-file --file-id file_xxx
# → 返回临时下载 URL
```

### 版本回滚

```bash
# 查看历史版本
lx file list-revisions --file-id file_xxx

# 恢复到指定版本
lx file revert-file --file-id file_xxx --revision-id rev_xxx
```

## 关键规则

1. **三步上传不可跳**：`lx file apply-upload` → HTTP PUT → `lx file commit-upload`。跳过任何一步文件都不会创建成功。
2. **新建 vs 更新**的参数差异（这是最容易搞错的）：

   | 场景 | `--file-id` | `--parent-entry-id` |
   |------|-------------|---------------------|
   | **新建文件** | 不传 | 父节点的 `entry_id` |
   | **更新已有文件** | 必填（`lx entry describe-entry` 返回的 `target_id`） | 当前文件条目自己的 `entry_id`（**不是**父节点！） |

3. **获取 file_id**：通过 `lx entry describe-entry` 查询文件条目，返回的 `target_id` 就是 `file_id`。
4. **COS 注册**：如果文件已在 COS 上有完整 URL，用 `lx file save-file` 比三步上传更快。系统自动获取文件大小和 MIME 类型。
5. **版本回滚是破坏性操作**：`lx file revert-file` 会将文件恢复到历史版本，当前版本会丢失，执行前确认用户意图。

## ⚠️ 副作用与风险

- 三步上传流程中，Step 2 的 HTTP PUT 是直接上传到预签名 URL，**不经过 lx CLI**。确保在终端或 HTTP 客户端中执行。
- 更新文件时 `--parent-entry-id` 填的是**文件条目自己的 entry_id**，不是父文件夹——这是最常见的错误。
- `upload_url` 有时效性，申请后应尽快完成上传和 commit。
- 版本恢复是**不可逆操作**，当前版本将被覆盖。

## 详细参数

所有命令的完整参数说明请运行：

```bash
lx file --help
lx file apply-upload --help
lx file commit-upload --help
# ...
```

## 参考

- [lx-entry](../SKILL.md) — 条目 skill 完整决策树
- [entry-crud.md](entry-crud.md) — 获取条目详情（`target_id` → `file_id`）
