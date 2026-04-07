//! Skill 文件自动生成器
//!
//! 根据 MCP schema 自动生成 AI agent 技能文件，用于指导 agent 如何使用 CLI 命令。

use super::types::{extract_command_name, extract_namespace, to_kebab_case, McpSchemaCollection};
use crate::datadir;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Skill 文件生成器
pub struct SkillGenerator<'a> {
    schema: &'a McpSchemaCollection,
    output_dir: PathBuf,
}

impl<'a> SkillGenerator<'a> {
    pub fn new(schema: &'a McpSchemaCollection, output_dir: PathBuf) -> Self {
        Self { schema, output_dir }
    }

    /// 生成所有 skill 文件
    pub fn generate_all(&self) -> Result<Vec<PathBuf>> {
        fs::create_dir_all(&self.output_dir)?;

        let mut generated_files = Vec::new();

        // 1. 生成总览文件
        let readme_path = self.output_dir.join("README.md");
        fs::write(&readme_path, self.generate_readme())?;
        generated_files.push(readme_path);

        // 2. 为每个 namespace 生成 skill 文件
        for category in &self.schema.categories {
            let namespace = extract_namespace(&category.name);
            let filename = format!("{}.md", namespace);
            let filepath = self.output_dir.join(&filename);

            let content = self.generate_namespace_skill(category);
            fs::write(&filepath, content)?;
            generated_files.push(filepath);
        }

        Ok(generated_files)
    }

    /// 生成 README.md 总览文件
    fn generate_readme(&self) -> String {
        let mut content = String::new();

        content.push_str("# lx CLI Skills\n\n");
        content.push_str(
            "本目录包含 lx CLI 的 AI agent 技能文件，用于指导 agent 如何使用各个 namespace 的命令。\n\n",
        );

        // 统计信息
        let total_tools: usize = self.schema.categories.iter().map(|c| c.tools.len()).sum();
        content.push_str(&format!(
            "## 概览\n\n- **Namespaces**: {}\n- **Total Tools**: {}\n\n",
            self.schema.categories.len(),
            total_tools
        ));

        // Namespace 列表
        content.push_str("## 可用 Namespaces\n\n");
        content.push_str("| Namespace | 描述 | 命令数 | 技能文件 |\n");
        content.push_str("|-----------|------|--------|----------|\n");

        for category in &self.schema.categories {
            let namespace = extract_namespace(&category.name);
            let desc = category.description.as_deref().unwrap_or("无描述");
            let tool_count = category.tools.len();
            content.push_str(&format!(
                "| `{}` | {} | {} | [{}.md]({}.md) |\n",
                namespace, desc, tool_count, namespace, namespace
            ));
        }

        // 快速开始
        content.push_str("\n## 快速开始\n\n");
        content.push_str("```bash\n");
        content.push_str("# 登录\n");
        content.push_str("lx login\n\n");
        content.push_str("# 同步最新 schema\n");
        content.push_str("lx tools sync\n\n");
        content.push_str("# 查看可用命令\n");
        content.push_str("lx tools categories\n");
        content.push_str("lx tools list --category team\n");
        content.push_str("```\n\n");

        // 使用方法
        content.push_str("## 如何使用技能文件\n\n");
        content.push_str("AI agent 可以读取这些 skill 文件来学习如何使用 lx CLI。\n\n");
        content.push_str("例如，当用户需要搜索知识库时，agent 可以：\n");
        content.push_str("1. 读取 `search.md` 了解搜索命令的参数和用法\n");
        content.push_str("2. 构建正确的命令执行搜索\n");
        content.push_str("3. 解析输出结果\n");

        content
    }

    /// 生成单个 namespace 的 skill 文件
    fn generate_namespace_skill(&self, category: &super::types::McpCategory) -> String {
        let namespace = extract_namespace(&category.name);
        let mut content = String::new();

        // 标题和描述
        content.push_str(&format!("# lx {} - {}\n\n", namespace, category.name));

        if let Some(desc) = &category.description {
            content.push_str(&format!("{}\n\n", desc));
        }

        // 概览表格
        content.push_str("## 命令概览\n\n");
        content.push_str("| 命令 | 描述 |\n");
        content.push_str("|------|------|\n");

        for tool in &category.tools {
            let cmd_name = extract_command_name(&tool.name, &namespace);
            let desc = tool.description.as_deref().unwrap_or("无描述");
            // 截断过长描述
            let short_desc = if desc.len() > 80 {
                format!("{}...", &desc[..77])
            } else {
                desc.to_string()
            };
            content.push_str(&format!(
                "| `lx {} {}` | {} |\n",
                namespace, cmd_name, short_desc
            ));
        }

        // 详细命令说明
        content.push_str("\n## 命令详情\n\n");

        for tool in &category.tools {
            let cmd_name = extract_command_name(&tool.name, &namespace);
            content.push_str(&format!("### `lx {} {}`\n\n", namespace, cmd_name));

            if let Some(desc) = &tool.description {
                content.push_str(&format!("{}\n\n", desc));
            }

            content.push_str(&format!("**MCP Tool**: `{}`\n\n", tool.name));

            // 获取完整 schema 以显示参数
            if let Some(full_schema) = self.schema.tools.get(&tool.name) {
                if let Some(input_schema) = &full_schema.input_schema {
                    if !input_schema.properties.is_empty() {
                        content.push_str("**参数**:\n\n");
                        content.push_str("| 参数 | 类型 | 必填 | 描述 |\n");
                        content.push_str("|------|------|------|------|\n");

                        for (name, prop) in &input_schema.properties {
                            let arg_name = to_kebab_case(name);
                            let type_str = prop.type_.as_deref().unwrap_or("string");
                            let required = if input_schema.required.contains(name) {
                                "是"
                            } else {
                                "否"
                            };
                            let desc = prop.description.as_deref().unwrap_or("-");
                            // 截断过长描述
                            let short_desc = if desc.len() > 60 {
                                format!("{}...", &desc[..57])
                            } else {
                                desc.to_string()
                            };
                            content.push_str(&format!(
                                "| `--{}` | {} | {} | {} |\n",
                                arg_name, type_str, required, short_desc
                            ));
                        }
                        content.push('\n');
                    }
                }
            }

            // 使用示例
            content.push_str("**示例**:\n\n");
            content.push_str("```bash\n");
            content.push_str(&format!("lx {} {}", namespace, cmd_name));

            // 添加示例参数
            if let Some(full_schema) = self.schema.tools.get(&tool.name) {
                if let Some(input_schema) = &full_schema.input_schema {
                    for name in &input_schema.required {
                        let arg_name = to_kebab_case(name);
                        content.push_str(&format!(" --{} <{}>", arg_name, name.to_uppercase()));
                    }
                }
            }
            content.push_str("\n```\n\n");

            content.push_str("---\n\n");
        }

        // 典型工作流
        content.push_str("## 典型工作流\n\n");
        content.push_str(&self.generate_workflow_examples(&namespace));

        content
    }

    /// 生成典型工作流示例
    fn generate_workflow_examples(&self, namespace: &str) -> String {
        let mut content = String::new();

        match namespace {
            "team" => {
                content.push_str("### 列出所有团队\n\n");
                content.push_str("```bash\n");
                content.push_str("# 获取用户可访问的团队列表\n");
                content.push_str("lx team list\n\n");
                content.push_str("# 获取常用团队\n");
                content.push_str("lx team list-frequent\n");
                content.push_str("```\n");
            }
            "space" => {
                content.push_str("### 操作知识库\n\n");
                content.push_str("```bash\n");
                content.push_str("# 获取团队下的知识库列表\n");
                content.push_str("lx space list --team-id <TEAM_ID>\n\n");
                content.push_str("# 获取知识库详情\n");
                content.push_str("lx space describe --space-id <SPACE_ID>\n\n");
                content.push_str("# 获取最近访问的知识库\n");
                content.push_str("lx space list-recently\n");
                content.push_str("```\n");
            }
            "entry" => {
                content.push_str("### 操作知识条目\n\n");
                content.push_str("```bash\n");
                content.push_str("# 获取条目详情\n");
                content.push_str("lx entry describe --entry-id <ENTRY_ID>\n\n");
                content.push_str("# 创建新条目\n");
                content.push_str("lx entry create --parent-entry-id <PARENT_ID> --name \"新文档\" --entry-type page\n\n");
                content.push_str("# 获取条目的子节点\n");
                content.push_str("lx entry list-children --parent-id <ENTRY_ID>\n");
                content.push_str("```\n");
            }
            "search" => {
                content.push_str("### 搜索知识库\n\n");
                content.push_str("```bash\n");
                content.push_str("# 全局搜索\n");
                content.push_str("lx search kb --keyword \"关键词\"\n\n");
                content.push_str("# 在指定知识库中搜索\n");
                content.push_str("lx search kb --keyword \"关键词\" --space-id <SPACE_ID>\n\n");
                content.push_str("# 向量检索\n");
                content.push_str("lx search embedding --keyword \"语义查询\"\n");
                content.push_str("```\n");
            }
            "block" => {
                content.push_str("### 操作文档块\n\n");
                content.push_str("```bash\n");
                content.push_str("# 获取块详情\n");
                content.push_str("lx block describe --block-id <BLOCK_ID>\n\n");
                content.push_str("# 获取块的子节点\n");
                content.push_str("lx block list-children --block-id <BLOCK_ID>\n\n");
                content.push_str("# 更新块内容\n");
                content.push_str("lx block update --block-id <BLOCK_ID> --text \"新内容\"\n");
                content.push_str("```\n");
            }
            "file" => {
                content.push_str("### 文件操作\n\n");
                content.push_str("```bash\n");
                content.push_str("# 获取文件详情\n");
                content.push_str("lx file describe --file-id <FILE_ID>\n\n");
                content.push_str("# 下载文件\n");
                content.push_str("lx file download --file-id <FILE_ID>\n\n");
                content.push_str("# 上传文件（需要多步骤）\n");
                content.push_str("# 1. 申请上传\n");
                content.push_str("lx file apply-upload --parent-entry-id <PARENT_ID> --file-name \"example.pdf\"\n");
                content.push_str("# 2. 使用返回的 upload_url 上传文件\n");
                content.push_str("# 3. 确认上传\n");
                content.push_str("lx file commit-upload --session-id <SESSION_ID>\n");
                content.push_str("```\n");
            }
            _ => {
                content.push_str(&format!("### 使用 {} namespace\n\n", namespace));
                content.push_str("```bash\n");
                content.push_str(&format!("# 查看 {} 命令帮助\n", namespace));
                content.push_str(&format!("lx {} --help\n", namespace));
                content.push_str("```\n");
            }
        }

        content
    }
}

/// 生成 skill 文件到默认目录
#[allow(dead_code)]
pub fn generate_skills(schema: &McpSchemaCollection) -> Result<Vec<PathBuf>> {
    let skill_dir = datadir::skills_dir();

    let generator = SkillGenerator::new(schema, skill_dir);
    generator.generate_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_skill_generator_readme() {
        let schema = McpSchemaCollection {
            version: "test".to_string(),
            categories: vec![super::super::types::McpCategory {
                name: "teamspace.team".to_string(),
                description: Some("团队管理".to_string()),
                tool_count: 2,
                tools: vec![
                    super::super::types::McpCategoryTool {
                        name: "team_list_teams".to_string(),
                        description: Some("列出团队".to_string()),
                    },
                    super::super::types::McpCategoryTool {
                        name: "team_describe_team".to_string(),
                        description: Some("获取团队详情".to_string()),
                    },
                ],
            }],
            tools: HashMap::new(),
        };

        let temp_dir = std::env::temp_dir().join("lexiang-skill-test");
        let generator = SkillGenerator::new(&schema, temp_dir.clone());

        let readme = generator.generate_readme();
        assert!(readme.contains("# lx CLI Skills"));
        // namespace extracted as "team" from "teamspace.team"
        assert!(readme.contains("team"));
        assert!(readme.contains("团队管理"));

        // Cleanup
        let _ = fs::remove_dir_all(temp_dir);
    }
}
