//! Skill Builder - 从 Markdown 源文件构建多目标输出
//!
//! 用法:
//!   skill-builder build [TARGET]
//!
//! 目标:
//!   codebuddy  - 生成 CodeBuddy/Claude Code 的 SKILL.md
//!   openclaw   - 生成 `OpenClaw` 插件 manifest
//!   mcp        - 生成 MCP plugin manifest
//!   all        - 生成所有目标（默认）

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
    /// CodeBuddy/Claude Code SKILL.md
    Codebuddy,
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
            BuildTarget::Codebuddy,
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
            BuildTarget::Codebuddy => {
                build_codebuddy(skills_dir, output, verbose).await?;
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

async fn build_codebuddy(
    skills_dir: &std::path::Path,
    _output: Option<&std::path::Path>,
    _verbose: bool,
) -> Result<()> {
    println!("  → CodeBuddy: {}/*/SKILL.md", skills_dir.display());
    // TODO: 实现 Markdown 解析和占位符替换
    Ok(())
}

async fn build_openclaw(
    skills_dir: &std::path::Path,
    _output: Option<&std::path::Path>,
    _verbose: bool,
) -> Result<()> {
    println!("  → OpenClaw: openclaw/generated/skills.json");
    let _ = skills_dir;
    // TODO: 实现 OpenClaw manifest 生成
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
