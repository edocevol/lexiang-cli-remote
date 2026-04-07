//! Skill 分发安装器
//!
//! 将生成的 skill 文件安装到各 AI Agent 的配置目录中，支持：
//! - Claude Code: ~/.claude/skills/ 或 .claude/skills/ (项目级)
//! - `CodeBuddy`: ~/.codebuddy/skills/ 或 .codebuddy/skills/ (项目级)
//! - Gemini CLI: ~/.gemini/ (GEMINI.md) 或 .gemini/ (项目级)
//! - Codex CLI: ~/.codex/skills/ 或 .codex/skills/ (项目级)

use crate::datadir;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

/// 支持的 AI Agent 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentKind {
    ClaudeCode,
    CodeBuddy,
    GeminiCli,
    CodexCli,
}

impl AgentKind {
    /// 所有支持的 Agent 类型
    pub fn all() -> &'static [AgentKind] {
        &[
            AgentKind::ClaudeCode,
            AgentKind::CodeBuddy,
            AgentKind::GeminiCli,
            AgentKind::CodexCli,
        ]
    }

    /// Agent 名称（用于 CLI 参数）
    pub fn name(&self) -> &'static str {
        match self {
            AgentKind::ClaudeCode => "claude",
            AgentKind::CodeBuddy => "codebuddy",
            AgentKind::GeminiCli => "gemini",
            AgentKind::CodexCli => "codex",
        }
    }

    /// Agent 显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            AgentKind::ClaudeCode => "Claude Code",
            AgentKind::CodeBuddy => "CodeBuddy",
            AgentKind::GeminiCli => "Gemini CLI",
            AgentKind::CodexCli => "Codex CLI",
        }
    }

    /// 从字符串解析 Agent 类型
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "claude" | "claude-code" | "claudecode" => Some(AgentKind::ClaudeCode),
            "codebuddy" | "code-buddy" => Some(AgentKind::CodeBuddy),
            "gemini" | "gemini-cli" | "geminicli" => Some(AgentKind::GeminiCli),
            "codex" | "codex-cli" | "codexcli" => Some(AgentKind::CodexCli),
            _ => None,
        }
    }
}

impl fmt::Display for AgentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// 安装范围
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallScope {
    /// 用户级（全局 home 目录）
    User,
    /// 项目级（当前工作目录）
    Project,
}

impl fmt::Display for InstallScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstallScope::User => write!(f, "user"),
            InstallScope::Project => write!(f, "project"),
        }
    }
}

/// 安装结果
#[derive(Debug)]
pub struct InstallResult {
    pub agent: AgentKind,
    pub scope: InstallScope,
    pub target_dir: PathBuf,
    pub files_installed: Vec<PathBuf>,
    #[allow(dead_code)]
    pub skill_name: String,
}

impl fmt::Display for InstallResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  ✓ {} ({}) → {}",
            self.agent.display_name(),
            self.scope,
            self.target_dir.display()
        )?;
        for file in &self.files_installed {
            write!(
                f,
                "\n    - {}",
                file.file_name().unwrap_or_default().to_string_lossy()
            )?;
        }
        Ok(())
    }
}

/// Skill 安装器
pub struct SkillInstaller {
    /// skill 源文件目录（默认 ~/.lexiang/skills/）
    source_dir: PathBuf,
    /// 项目根目录（用于项目级安装）
    project_dir: Option<PathBuf>,
}

impl SkillInstaller {
    /// 创建安装器
    pub fn new(source_dir: Option<PathBuf>, project_dir: Option<PathBuf>) -> Self {
        Self {
            source_dir: source_dir.unwrap_or_else(datadir::skills_dir),
            project_dir,
        }
    }

    /// 获取 Agent 的用户级 skill 目录
    fn user_skill_dir(agent: AgentKind) -> PathBuf {
        let home = dirs::home_dir().expect("Cannot determine home directory");
        match agent {
            // ~/.claude/skills/lx-cli/
            AgentKind::ClaudeCode => home.join(".claude").join("skills").join("lx-cli"),
            // ~/.codebuddy/skills/lx-cli/
            AgentKind::CodeBuddy => home.join(".codebuddy").join("skills").join("lx-cli"),
            // ~/.gemini/skills/lx-cli/
            AgentKind::GeminiCli => home.join(".gemini").join("skills").join("lx-cli"),
            // ~/.codex/skills/lx-cli/
            AgentKind::CodexCli => home.join(".codex").join("skills").join("lx-cli"),
        }
    }

    /// 获取 Agent 的项目级 skill 目录
    fn project_skill_dir(agent: AgentKind, project_root: &Path) -> PathBuf {
        match agent {
            AgentKind::ClaudeCode => project_root.join(".claude").join("skills").join("lx-cli"),
            AgentKind::CodeBuddy => project_root
                .join(".codebuddy")
                .join("skills")
                .join("lx-cli"),
            AgentKind::GeminiCli => project_root.join(".gemini").join("skills").join("lx-cli"),
            AgentKind::CodexCli => project_root.join(".codex").join("skills").join("lx-cli"),
        }
    }

    /// 安装 skill 到指定 Agent 目录
    pub fn install(&self, agents: &[AgentKind], scope: InstallScope) -> Result<Vec<InstallResult>> {
        // 检查源目录
        if !self.source_dir.exists() {
            anyhow::bail!(
                "Skill 源目录不存在: {:?}\n请先运行 'lx tools skill' 生成 skill 文件",
                self.source_dir
            );
        }

        let source_files = self.collect_source_files()?;
        if source_files.is_empty() {
            anyhow::bail!(
                "Skill 源目录中没有 .md 文件: {:?}\n请先运行 'lx tools skill' 生成 skill 文件",
                self.source_dir
            );
        }

        let mut results = Vec::new();

        for &agent in agents {
            let target_dir = match scope {
                InstallScope::User => Self::user_skill_dir(agent),
                InstallScope::Project => {
                    let project_root = self.project_dir.as_deref().ok_or_else(|| {
                        anyhow::anyhow!(
                            "项目级安装需要指定项目目录（--project-dir）或在项目目录下执行"
                        )
                    })?;
                    Self::project_skill_dir(agent, project_root)
                }
            };

            let installed = self.install_to_dir(agent, &source_files, &target_dir)?;
            results.push(installed);
        }

        Ok(results)
    }

    /// 将 skill 文件安装到目标目录
    fn install_to_dir(
        &self,
        agent: AgentKind,
        source_files: &[PathBuf],
        target_dir: &Path,
    ) -> Result<InstallResult> {
        // 安装前先清理旧文件，确保 update 场景不残留过期文件
        if target_dir.exists() {
            fs::remove_dir_all(target_dir)
                .with_context(|| format!("清理旧 skill 目录失败: {:?}", target_dir))?;
        }
        fs::create_dir_all(target_dir)
            .with_context(|| format!("创建目录失败: {:?}", target_dir))?;

        let mut files_installed = Vec::new();

        // 所有 Agent 统一使用 SKILL.md + references/ 结构
        let skill_content = self.generate_unified_skill_md(source_files)?;
        let skill_path = target_dir.join("SKILL.md");
        fs::write(&skill_path, &skill_content)
            .with_context(|| format!("写入失败: {:?}", skill_path))?;
        files_installed.push(skill_path);

        // 创建 references/ 子目录并复制 namespace 参考文档
        let refs_dir = target_dir.join("references");
        fs::create_dir_all(&refs_dir)
            .with_context(|| format!("创建 references 目录失败: {:?}", refs_dir))?;

        for src_file in source_files {
            let filename = src_file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if filename == "README.md" {
                continue; // SKILL.md 已包含概览信息
            }
            let dest = refs_dir.join(&filename);
            fs::copy(src_file, &dest)
                .with_context(|| format!("复制失败: {:?} -> {:?}", src_file, dest))?;
            files_installed.push(dest);
        }

        Ok(InstallResult {
            agent,
            scope: if target_dir
                .to_string_lossy()
                .contains(&format!("{}/.lexiang", dirs::home_dir().unwrap().display()))
                || target_dir
                    .starts_with(dirs::home_dir().unwrap().join(format!(".{}", agent.name())))
            {
                InstallScope::User
            } else {
                InstallScope::Project
            },
            target_dir: target_dir.to_path_buf(),
            files_installed,
            skill_name: "lx-cli".to_string(),
        })
    }

    /// 收集源目录中的 .md 文件（包括 references/ 子目录）
    fn collect_source_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        // 收集根目录的 README.md
        let readme = self.source_dir.join("README.md");
        if readme.exists() {
            files.push(readme);
        }

        // 收集 references/ 子目录中的 .md 文件
        let refs_dir = self.source_dir.join("references");
        if refs_dir.exists() {
            for entry in
                fs::read_dir(&refs_dir).with_context(|| format!("读取目录失败: {:?}", refs_dir))?
            {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "md") {
                    files.push(path);
                }
            }
        } else {
            // 兼容旧格式：直接收集根目录的 .md 文件
            for entry in fs::read_dir(&self.source_dir)
                .with_context(|| format!("读取目录失败: {:?}", self.source_dir))?
            {
                let entry = entry?;
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "md") {
                    files.push(path);
                }
            }
        }

        // README.md 排第一，其余按字母序
        files.sort_by(|a, b| {
            let a_is_readme = a.file_name().unwrap_or_default() == "README.md";
            let b_is_readme = b.file_name().unwrap_or_default() == "README.md";
            match (a_is_readme, b_is_readme) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });
        Ok(files)
    }

    /// 生成统一的 SKILL.md（Claude Code / `CodeBuddy` / Gemini 格式）
    fn generate_unified_skill_md(&self, source_files: &[PathBuf]) -> Result<String> {
        let mut content = String::new();

        // YAML frontmatter
        content.push_str("---\n");
        content.push_str("name: lx-cli\n");
        content.push_str("description: |\n");
        content.push_str(
            "  Lexiang CLI (lx) 知识库管理工具。当用户需要操作乐享知识库（搜索、创建、\n",
        );
        content.push_str("  编辑文档、管理团队/空间/条目/文件/块等）时使用此 skill。\n");
        content.push_str("  触发词：知识库、乐享、lx、lexiang、文档管理、知识管理\n");
        content.push_str("---\n\n");

        // 读取 README.md 作为概览
        let readme_path = self.source_dir.join("README.md");
        if readme_path.exists() {
            let readme = fs::read_to_string(&readme_path)?;
            content.push_str(&readme);
            content.push_str("\n\n");
        }

        // 引用各 namespace 的详细文档
        content.push_str("## 详细命令参考\n\n");
        content.push_str("以下文件包含各 namespace 的详细命令说明，按需查阅：\n\n");
        for file in source_files {
            let filename = file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if filename == "README.md" {
                continue;
            }
            content.push_str(&format!(
                "- [`{}`](references/{}) - {} namespace 详细参考\n",
                filename,
                filename,
                filename.trim_end_matches(".md")
            ));
        }

        Ok(content)
    }

    /// 卸载 skill
    pub fn uninstall(
        &self,
        agents: &[AgentKind],
        scope: InstallScope,
    ) -> Result<Vec<(AgentKind, PathBuf)>> {
        let mut removed = Vec::new();

        for &agent in agents {
            let target_dir = match scope {
                InstallScope::User => Self::user_skill_dir(agent),
                InstallScope::Project => {
                    let project_root = self
                        .project_dir
                        .as_deref()
                        .ok_or_else(|| anyhow::anyhow!("项目级卸载需要指定项目目录"))?;
                    Self::project_skill_dir(agent, project_root)
                }
            };

            if target_dir.exists() {
                fs::remove_dir_all(&target_dir)
                    .with_context(|| format!("删除目录失败: {:?}", target_dir))?;
                removed.push((agent, target_dir));
            }
        }

        Ok(removed)
    }

    /// 获取安装状态
    pub fn status(&self) -> HashMap<AgentKind, Vec<(InstallScope, PathBuf, bool)>> {
        let mut status = HashMap::new();

        for &agent in AgentKind::all() {
            let mut entries = Vec::new();

            // 检查用户级安装
            let user_dir = Self::user_skill_dir(agent);
            let user_installed = user_dir.join("SKILL.md").exists();
            entries.push((InstallScope::User, user_dir, user_installed));

            // 检查项目级安装（如果有项目目录）
            if let Some(ref project_root) = self.project_dir {
                let project_dir = Self::project_skill_dir(agent, project_root);
                let project_installed = project_dir.join("SKILL.md").exists();
                entries.push((InstallScope::Project, project_dir, project_installed));
            }

            status.insert(agent, entries);
        }

        status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_kind_from_name() {
        assert_eq!(AgentKind::from_name("claude"), Some(AgentKind::ClaudeCode));
        assert_eq!(
            AgentKind::from_name("codebuddy"),
            Some(AgentKind::CodeBuddy)
        );
        assert_eq!(AgentKind::from_name("gemini"), Some(AgentKind::GeminiCli));
        assert_eq!(AgentKind::from_name("codex"), Some(AgentKind::CodexCli));
        assert_eq!(AgentKind::from_name("unknown"), None);
    }

    #[test]
    fn test_agent_kind_all() {
        assert_eq!(AgentKind::all().len(), 4);
    }

    #[test]
    fn test_user_skill_dir() {
        let home = dirs::home_dir().unwrap();
        assert_eq!(
            SkillInstaller::user_skill_dir(AgentKind::ClaudeCode),
            home.join(".claude/skills/lx-cli")
        );
        assert_eq!(
            SkillInstaller::user_skill_dir(AgentKind::CodeBuddy),
            home.join(".codebuddy/skills/lx-cli")
        );
        assert_eq!(
            SkillInstaller::user_skill_dir(AgentKind::GeminiCli),
            home.join(".gemini/skills/lx-cli")
        );
        assert_eq!(
            SkillInstaller::user_skill_dir(AgentKind::CodexCli),
            home.join(".codex/skills/lx-cli")
        );
    }

    #[test]
    fn test_project_skill_dir() {
        let project = PathBuf::from("/tmp/my-project");
        assert_eq!(
            SkillInstaller::project_skill_dir(AgentKind::ClaudeCode, &project),
            PathBuf::from("/tmp/my-project/.claude/skills/lx-cli")
        );
    }
}
