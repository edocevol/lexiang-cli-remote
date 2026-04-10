//! Skill Builder - 从 Markdown 源文件构建多目标输出
//!
//! 用法:
//!   skill-builder build [TARGET]
//!
//! 目标:
//!   claude-code - 生成 Claude Code plugin marketplace 格式
//!   openclaw   - 生成 `OpenClaw` 插件 manifest
//!   mcp        - 生成 MCP plugin manifest
//!   all        - 生成所有目标（默认）

mod generator;
mod parser;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "skill-builder")]
#[command(about = "从 Markdown 源文件构建多目标 Skill 输出")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 构建 Skill 到指定目标格式
    Build {
        /// 构建目标
        #[arg(value_enum, default_value = "all")]
        target: BuildTarget,

        /// Skills 目录路径
        #[arg(short, long, default_value = "skills")]
        skills_dir: PathBuf,

        /// 输出目录（默认根据目标自动选择）
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// 详细输出
        #[arg(short, long)]
        verbose: bool,
    },

    /// 验证 Skill 源文件格式
    Validate {
        /// Skills 目录路径
        #[arg(short, long, default_value = "skills")]
        skills_dir: PathBuf,
    },

    /// 列出所有 Skill
    List {
        /// Skills 目录路径
        #[arg(short, long, default_value = "skills")]
        skills_dir: PathBuf,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum BuildTarget {
    /// Claude Code plugin marketplace 格式
    ClaudeCode,
    /// `OpenClaw` 插件 manifest
    Openclaw,
    /// MCP plugin manifest
    Mcp,
    /// 所有目标
    All,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            target,
            skills_dir,
            output,
            verbose,
        } => {
            build_skills(target, &skills_dir, output.as_deref(), verbose).await?;
        }
        Commands::Validate { skills_dir } => {
            validate_skills(&skills_dir).await?;
        }
        Commands::List { skills_dir } => {
            list_skills(&skills_dir).await?;
        }
    }

    Ok(())
}

async fn build_skills(
    target: BuildTarget,
    skills_dir: &std::path::Path,
    output: Option<&std::path::Path>,
    verbose: bool,
) -> Result<()> {
    let targets = match target {
        BuildTarget::All => vec![
            BuildTarget::ClaudeCode,
            BuildTarget::Openclaw,
            BuildTarget::Mcp,
        ],
        t => vec![t],
    };

    for t in targets {
        if verbose {
            println!("Building target: {:?}", t);
        }

        match t {
            BuildTarget::ClaudeCode => {
                build_claude_code(skills_dir, output, verbose).await?;
            }
            BuildTarget::Openclaw => {
                build_openclaw(skills_dir, output, verbose).await?;
            }
            BuildTarget::Mcp => {
                build_mcp(skills_dir, output, verbose).await?;
            }
            BuildTarget::All => unreachable!(),
        }
    }

    println!("✓ Build complete");
    Ok(())
}

async fn build_claude_code(
    skills_dir: &std::path::Path,
    output: Option<&std::path::Path>,
    verbose: bool,
) -> Result<()> {
    use walkdir::WalkDir;

    let output_dir = output
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from(".claude-plugin"));
    let skills_output = output_dir.join("skills");
    std::fs::create_dir_all(&skills_output)?;

    let mut skill_paths: Vec<String> = Vec::new();

    for entry in WalkDir::new(skills_dir).min_depth(1).max_depth(2) {
        let entry = entry?;
        let path = entry.path();

        if path.file_name().map(|n| n == "SKILL.md").unwrap_or(false) {
            let skill = parser::parse_skill_file(path)?;
            let generated = generator::generate_claude_code(&skill, path.parent().unwrap())?;

            // 输出到 .claude-plugin/skills/<skill-name>/SKILL.md
            let skill_name = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let skill_dir = skills_output.join(&skill_name);
            std::fs::create_dir_all(&skill_dir)?;

            let out_path = skill_dir.join("SKILL.md");
            std::fs::write(&out_path, &generated)?;

            // 记录相对路径用于 plugin.json
            skill_paths.push(format!("./skills/{}/", skill_name));

            if verbose {
                println!("    {} → {}", path.display(), out_path.display());
            }
        }
    }

    // 生成 plugin.json
    let plugin_json = serde_json::json!({
        "version": "1.0.0",
        "name": "lx-cli-skills",
        "description": "乐享知识库 CLI 工具 skills",
        "skills": ["./skills/"]
    });

    let plugin_path = output_dir.join("plugin.json");
    std::fs::write(&plugin_path, serde_json::to_string_pretty(&plugin_json)?)?;

    if verbose {
        println!("    plugin.json → {}", plugin_path.display());
    }

    println!("  → Claude Code: {}/", output_dir.display());
    println!("    - plugin.json");
    println!("    - skills/*/SKILL.md ({} skills)", skill_paths.len());
    Ok(())
}

async fn build_openclaw(
    skills_dir: &std::path::Path,
    output: Option<&std::path::Path>,
    verbose: bool,
) -> Result<()> {
    use walkdir::WalkDir;

    let output_dir = output
        .map(std::path::Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("openclaw/generated"));
    std::fs::create_dir_all(&output_dir)?;

    for entry in WalkDir::new(skills_dir).min_depth(1).max_depth(2) {
        let entry = entry?;
        let path = entry.path();

        if path.file_name().map(|n| n == "SKILL.md").unwrap_or(false) {
            let skill = parser::parse_skill_file(path)?;
            let generated = generator::generate_openclaw(&skill, path.parent().unwrap())?;

            // 输出到 openclaw/generated/<skill-name>/SKILL.md
            let skill_name = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let out_path = output_dir.join(&skill_name).join("SKILL.md");

            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&out_path, &generated)?;

            if verbose {
                println!("    {} → {}", path.display(), out_path.display());
            }
        }
    }

    println!("  → OpenClaw: {}/*/SKILL.md", output_dir.display());
    Ok(())
}

async fn build_mcp(
    skills_dir: &std::path::Path,
    _output: Option<&std::path::Path>,
    _verbose: bool,
) -> Result<()> {
    println!("  → MCP: dist/mcp-tools.json");
    let _ = skills_dir;
    // TODO: 实现 MCP manifest 生成
    Ok(())
}

async fn validate_skills(skills_dir: &std::path::Path) -> Result<()> {
    println!("Validating skills in: {}", skills_dir.display());
    // TODO: 实现验证逻辑
    Ok(())
}

async fn list_skills(skills_dir: &std::path::Path) -> Result<()> {
    println!("Skills in: {}", skills_dir.display());
    // TODO: 列出所有 skill
    Ok(())
}
