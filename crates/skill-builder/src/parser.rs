//! Markdown 解析器 - 使用 Pest

use anyhow::{Context, Result};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Parser)]
#[grammar = "skill.pest"]
struct SkillParser;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub metadata: SkillMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillMetadata {
    #[serde(default)]
    pub requires: SkillRequires,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SkillRequires {
    #[serde(default)]
    pub bins: Vec<String>,
    #[serde(default)]
    pub tools: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Placeholder {
    Tools {
        namespace: String,
        tools: Vec<String>,
    },
    Params {
        tool_name: String,
    },
    Schema {
        tool_name: String,
    },
    Include {
        path: String,
    },
    Example, // 示例代码块，内容在 inner_content 中
}

#[derive(Debug, Clone)]
pub struct PlaceholderSpan {
    pub placeholder: Placeholder,
    pub start: usize,
    pub end: usize,
    pub inner_content: Option<String>,
}

#[derive(Debug)]
pub struct ParsedSkill {
    pub frontmatter: SkillFrontmatter,
    pub content: String,
    pub placeholders: Vec<PlaceholderSpan>,
}

pub fn parse_skill(content: &str) -> Result<ParsedSkill> {
    let mut pairs =
        SkillParser::parse(Rule::skill, content).context("Failed to parse skill file")?;

    let skill_pair = pairs.next().unwrap();

    let mut frontmatter_str = String::new();
    let mut body_start = 0;
    let mut body_end = 0;
    let mut placeholders = Vec::new();

    for pair in skill_pair.into_inner() {
        match pair.as_rule() {
            Rule::frontmatter => {
                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::frontmatter_content {
                        frontmatter_str = inner.as_str().to_string();
                    }
                }
            }
            Rule::body => {
                body_start = pair.as_span().start();
                body_end = pair.as_span().end();

                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::placeholder {
                        let span = inner.as_span();
                        let start = span.start() - body_start;
                        let end = span.end() - body_start;

                        let mut ptype = "";
                        let mut pargs = "";
                        let mut inner_content = None;

                        for p in inner.into_inner() {
                            match p.as_rule() {
                                Rule::placeholder_type => ptype = p.as_str(),
                                Rule::placeholder_args => pargs = p.as_str().trim(),
                                Rule::placeholder_inner => {
                                    let c = p.as_str().trim();
                                    if !c.is_empty() {
                                        inner_content = Some(c.to_string());
                                    }
                                }
                                _ => {}
                            }
                        }

                        if let Some(ph) = build_placeholder(ptype, pargs)? {
                            placeholders.push(PlaceholderSpan {
                                placeholder: ph,
                                start,
                                end,
                                inner_content,
                            });
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let frontmatter: SkillFrontmatter =
        serde_yaml::from_str(&frontmatter_str).context("Failed to parse frontmatter YAML")?;

    let body_content = content[body_start..body_end].to_string();

    Ok(ParsedSkill {
        frontmatter,
        content: body_content,
        placeholders,
    })
}

fn build_placeholder(ptype: &str, args: &str) -> Result<Option<Placeholder>> {
    let ph = match ptype {
        "tools" => {
            let parts: Vec<&str> = args.split_whitespace().collect();
            if parts.is_empty() {
                anyhow::bail!("::: tools requires a namespace");
            }
            Placeholder::Tools {
                namespace: parts[0].to_string(),
                tools: parts[1..].iter().map(std::string::ToString::to_string).collect(),
            }
        }
        "params" => {
            if args.is_empty() {
                anyhow::bail!("::: params requires a tool_name");
            }
            Placeholder::Params {
                tool_name: args.split_whitespace().next().unwrap().to_string(),
            }
        }
        "schema" => {
            if args.is_empty() {
                anyhow::bail!("::: schema requires a tool_name");
            }
            Placeholder::Schema {
                tool_name: args.split_whitespace().next().unwrap().to_string(),
            }
        }
        "include" => {
            if args.is_empty() {
                anyhow::bail!("::: include requires a path");
            }
            Placeholder::Include {
                path: args.to_string(),
            }
        }
        "example" => Placeholder::Example,
        _ => return Ok(None),
    };
    Ok(Some(ph))
}

pub fn parse_skill_file(path: &Path) -> Result<ParsedSkill> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read: {}", path.display()))?;
    parse_skill(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = "---\nname: test\nversion: 1.0.0\n---\n\n# Hello\n";
        let parsed = parse_skill(content).unwrap();
        assert_eq!(parsed.frontmatter.name, "test");
        assert_eq!(parsed.frontmatter.version, "1.0.0");
    }

    #[test]
    fn test_parse_tools_placeholder() {
        let content =
            "---\nname: test\n---\n\n::: tools search kb-search kb-embedding-search\n:::\n";
        let parsed = parse_skill(content).unwrap();
        assert_eq!(parsed.placeholders.len(), 1);

        if let Placeholder::Tools { namespace, tools } = &parsed.placeholders[0].placeholder {
            assert_eq!(namespace, "search");
            assert_eq!(tools, &["kb-search", "kb-embedding-search"]);
        } else {
            panic!("Expected Tools");
        }
    }

    #[test]
    fn test_parse_params_placeholder() {
        let content = "---\nname: test\n---\n\n::: params search_kb_search\n| a | b |\n:::\n";
        let parsed = parse_skill(content).unwrap();
        assert_eq!(parsed.placeholders.len(), 1);

        if let Placeholder::Params { tool_name } = &parsed.placeholders[0].placeholder {
            assert_eq!(tool_name, "search_kb_search");
        } else {
            panic!("Expected Params");
        }
        assert!(parsed.placeholders[0].inner_content.is_some());
    }

    #[test]
    fn test_parse_full_skill() {
        let content = "---\nname: test\n---\n\n# Title\n\n::: tools search\n:::\n\n## Params\n\n::: params foo\n:::\n";
        let parsed = parse_skill(content).unwrap();
        assert_eq!(parsed.placeholders.len(), 2);
    }

    #[test]
    fn test_parse_include_placeholder() {
        let content = "---\nname: test\n---\n\n::: include ./ref.md\n:::\n";
        let parsed = parse_skill(content).unwrap();
        assert_eq!(parsed.placeholders.len(), 1);

        if let Placeholder::Include { path } = &parsed.placeholders[0].placeholder {
            assert_eq!(path, "./ref.md");
        } else {
            panic!("Expected Include");
        }
    }
}
