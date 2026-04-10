//! 输出生成器
//!
//! 从 SKILL.md 模板生成不同格式的输出：
//! - Claude Code: plugin marketplace 格式
//! - `OpenClaw`: skills.json (JSON manifest)

use crate::parser::{ParsedSkill, Placeholder, PlaceholderSpan};
use anyhow::{Context, Result};
use std::path::Path;

/// 生成 Claude Code plugin marketplace 格式的 SKILL.md
pub fn generate_claude_code(skill: &ParsedSkill, base_dir: &Path) -> Result<String> {
    let mut output = String::new();

    // 1. 生成 frontmatter
    output.push_str("---\n");
    output.push_str(&format!("name: {}\n", skill.frontmatter.name));
    if !skill.frontmatter.version.is_empty() {
        output.push_str(&format!("version: {}\n", skill.frontmatter.version));
    }
    if !skill.frontmatter.description.is_empty() {
        output.push_str(&format!(
            "description: \"{}\"\n",
            skill.frontmatter.description.replace('"', "\\\"")
        ));
    }
    if !skill.frontmatter.metadata.requires.bins.is_empty() {
        output.push_str("metadata:\n");
        output.push_str("  requires:\n");
        output.push_str(&format!(
            "    bins: {:?}\n",
            skill.frontmatter.metadata.requires.bins
        ));
    }
    output.push_str("---\n\n");

    // 2. 替换占位符
    let mut last_end = 0;
    for span in &skill.placeholders {
        output.push_str(&skill.content[last_end..span.start]);
        let replacement = generate_placeholder_content(&span.placeholder, base_dir, span)?;
        output.push_str(&replacement);
        last_end = span.end;
    }
    output.push_str(&skill.content[last_end..]);

    Ok(output)
}

/// 生成占位符内容
fn generate_placeholder_content(
    placeholder: &Placeholder,
    base_dir: &Path,
    span: &PlaceholderSpan,
) -> Result<String> {
    match placeholder {
        Placeholder::Include { path } => {
            let include_path = base_dir.join(path);
            if include_path.exists() {
                std::fs::read_to_string(&include_path)
                    .with_context(|| format!("Failed to include: {}", include_path.display()))
            } else {
                Ok(format!("<!-- include not found: {} -->\n", path))
            }
        }
        Placeholder::Tools { namespace, tools } => {
            Ok(format!("<!-- TODO: tools {} {:?} -->\n", namespace, tools))
        }
        Placeholder::Params { tool_name } => Ok(format!("<!-- TODO: params {} -->\n", tool_name)),
        Placeholder::Schema { tool_name } => Ok(format!("<!-- TODO: schema {} -->\n", tool_name)),
        Placeholder::Example => {
            // Claude Code 目标：保持 bash 代码块格式
            if let Some(content) = &span.inner_content {
                Ok(format!("```bash\n{}\n```", content))
            } else {
                Ok("```bash\n```".to_string())
            }
        }
    }
}

/// 生成 `OpenClaw` SKILL.md（替换占位符，转换示例格式）
pub fn generate_openclaw(skill: &ParsedSkill, base_dir: &Path) -> Result<String> {
    let mut output = String::new();

    // 1. 生成 frontmatter
    output.push_str("---\n");
    output.push_str(&format!("name: {}\n", skill.frontmatter.name));
    if !skill.frontmatter.version.is_empty() {
        output.push_str(&format!("version: {}\n", skill.frontmatter.version));
    }
    if !skill.frontmatter.description.is_empty() {
        output.push_str(&format!(
            "description: \"{}\"\n",
            skill.frontmatter.description.replace('"', "\\\"")
        ));
    }
    if !skill.frontmatter.metadata.requires.bins.is_empty() {
        output.push_str("metadata:\n");
        output.push_str("  requires:\n");
        output.push_str(&format!(
            "    bins: {:?}\n",
            skill.frontmatter.metadata.requires.bins
        ));
    }
    output.push_str("---\n\n");

    // 2. 替换占位符
    let mut last_end = 0;
    for span in &skill.placeholders {
        output.push_str(&skill.content[last_end..span.start]);
        let replacement = generate_placeholder_content_openclaw(&span.placeholder, base_dir, span)?;
        output.push_str(&replacement);
        last_end = span.end;
    }
    output.push_str(&skill.content[last_end..]);

    Ok(output)
}

/// 生成 `OpenClaw` 目标的占位符内容
fn generate_placeholder_content_openclaw(
    placeholder: &Placeholder,
    base_dir: &Path,
    span: &PlaceholderSpan,
) -> Result<String> {
    match placeholder {
        Placeholder::Include { path } => {
            let include_path = base_dir.join(path);
            if include_path.exists() {
                std::fs::read_to_string(&include_path)
                    .with_context(|| format!("Failed to include: {}", include_path.display()))
            } else {
                Ok(format!("<!-- include not found: {} -->\n", path))
            }
        }
        Placeholder::Tools { namespace, tools } => {
            Ok(format!("<!-- TODO: tools {} {:?} -->\n", namespace, tools))
        }
        Placeholder::Params { tool_name } => Ok(format!("<!-- TODO: params {} -->\n", tool_name)),
        Placeholder::Schema { tool_name } => Ok(format!("<!-- TODO: schema {} -->\n", tool_name)),
        Placeholder::Example => {
            // OpenClaw 目标：转换为工具调用描述
            if let Some(content) = &span.inner_content {
                Ok(convert_cli_to_tool_calls(content))
            } else {
                Ok(String::new())
            }
        }
    }
}

/// 将 CLI 命令转换为 MCP 工具调用描述
fn convert_cli_to_tool_calls(bash_code: &str) -> String {
    use regex::Regex;

    let mut output = Vec::new();
    let mut step = 1;

    // 逐行处理，跟踪注释和命令
    let mut current_comment: Option<String> = None;
    let mut current_cmd: Option<(String, String, String)> = None; // (ns, cmd, args)

    let cmd_re = Regex::new(r#"^lx\s+(\w+)\s+(\S+)\s*(.*)"#).unwrap();
    let arg_re = Regex::new(r#"--(\w[\w-]*)\s+('[^']*'|"[^"]*"|\S+)"#).unwrap();
    let comment_re = Regex::new(r"^#\s*(.*)").unwrap();

    let mut lines: Vec<&str> = bash_code.lines().collect();
    lines.push(""); // 追加空行确保最后一个命令被处理

    for line in lines {
        let trimmed = line.trim();

        // 处理续行
        if let Some((ns, cmd, ref mut args)) = current_cmd.as_mut() {
            if trimmed.ends_with('\\') {
                args.push(' ');
                args.push_str(trimmed.trim_end_matches('\\').trim());
                continue;
            } else if !trimmed.is_empty()
                && !trimmed.starts_with('#')
                && !trimmed.starts_with("lx ")
            {
                // 续行内容
                args.push(' ');
                args.push_str(trimmed);
                continue;
            }

            // 当前命令结束，生成输出
            let tool_name = format!("lx-{}-{}", ns, cmd);
            let mut call_lines = Vec::new();

            if let Some(desc) = &current_comment {
                if !desc.is_empty() {
                    call_lines.push(format!("{}. {} - 调用 `{}` 工具", step, desc, tool_name));
                } else {
                    call_lines.push(format!("{}. 调用 `{}` 工具", step, tool_name));
                }
            } else {
                call_lines.push(format!("{}. 调用 `{}` 工具", step, tool_name));
            }

            for arg_cap in arg_re.captures_iter(args) {
                let param_name = arg_cap[1].replace('-', "_");
                let mut value = arg_cap[2].to_string();
                if (value.starts_with('\'') && value.ends_with('\''))
                    || (value.starts_with('"') && value.ends_with('"'))
                {
                    value = value[1..value.len() - 1].to_string();
                }
                call_lines.push(format!("   - {}: `{}`", param_name, value));
            }

            output.push(call_lines.join("\n"));
            step += 1;
            current_cmd = None;
            current_comment = None;
        }

        // 检测注释
        if let Some(cap) = comment_re.captures(trimmed) {
            current_comment = Some(cap[1].to_string());
            continue;
        }

        // 检测新命令
        if let Some(cap) = cmd_re.captures(trimmed) {
            let ns = cap[1].to_string();
            let cmd = cap[2].to_string();
            let mut args = cap[3].to_string();

            if args.ends_with('\\') {
                args = args.trim_end_matches('\\').trim().to_string();
                current_cmd = Some((ns, cmd, args));
            } else {
                // 单行命令，直接处理
                current_cmd = Some((ns, cmd, args));
            }
            continue;
        }

        // 空行或其他内容，清空注释
        if trimmed.is_empty() {
            current_comment = None;
        }
    }

    if output.is_empty() {
        format!("```bash\n{}\n```", bash_code)
    } else {
        output.join("\n\n")
    }
}

/// `OpenClaw` skill manifest
#[derive(Debug, serde::Serialize)]
pub struct OpenClawManifest {
    pub skills: Vec<OpenClawSkill>,
}

#[derive(Debug, serde::Serialize)]
pub struct OpenClawSkill {
    pub name: String,
    pub version: String,
    pub description: String,
    pub triggers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<String>,
}

/// 生成 `OpenClaw` manifest
pub fn generate_openclaw_manifest(skills: &[ParsedSkill]) -> Result<String> {
    let manifest = OpenClawManifest {
        skills: skills
            .iter()
            .map(|s| {
                let triggers = extract_triggers(&s.frontmatter.description);
                OpenClawSkill {
                    name: s.frontmatter.name.clone(),
                    version: s.frontmatter.version.clone(),
                    description: s.frontmatter.description.clone(),
                    triggers,
                    tools: s.frontmatter.metadata.requires.tools.clone(),
                }
            })
            .collect(),
    };
    serde_json::to_string_pretty(&manifest).context("Failed to serialize OpenClaw manifest")
}

/// 从描述中提取触发词
fn extract_triggers(description: &str) -> Vec<String> {
    let triggers_pattern = ["触发词：", "触发词:", "triggers:"];
    for pattern in triggers_pattern {
        if let Some(pos) = description.find(pattern) {
            let after = &description[pos + pattern.len()..];
            let end = after.find(['。', '.', '\n']).unwrap_or(after.len());
            let triggers_str = &after[..end];
            return triggers_str
                .split(['、', ',', '，'])
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_triggers() {
        let desc = "搜索知识库。触发词：搜索、查找、有没有关于";
        let triggers = extract_triggers(desc);
        assert_eq!(triggers, vec!["搜索", "查找", "有没有关于"]);
    }

    #[test]
    fn test_extract_triggers_english() {
        let desc = "Search docs. triggers: search, find, lookup";
        let triggers = extract_triggers(desc);
        assert_eq!(triggers, vec!["search", "find", "lookup"]);
    }

    #[test]
    fn test_convert_cli_to_tool_calls() {
        let bash = r#"# 查看当前状态
lx block table-get --block-id tbl_xxx --format table

# 修改
lx block table-set --block-id tbl_xxx --row 2 --col 1 --text "修正值""#;

        let result = convert_cli_to_tool_calls(bash);
        eprintln!("Result:\n{}", result);
        assert!(result.contains("调用 `lx-block-table-get` 工具"));
        assert!(result.contains("block_id: `tbl_xxx`"));
        assert!(result.contains("调用 `lx-block-table-set` 工具"));
        assert!(result.contains("text: `修正值`"));
    }
}
