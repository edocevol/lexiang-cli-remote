//! 统一数据目录管理
//!
//! 所有 lefs 数据统一存储在 `~/.lexiang/` 目录下，包括：
//! - auth/      OAuth 令牌
//! - tools/     MCP schema 缓存
//! - skills/    AI Agent skill 文件
//! - worktrees/ worktree 注册表

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// 旧数据目录名
const LEGACY_DIR_NAME: &str = ".lefs";
/// 新数据目录名
const DATA_DIR_NAME: &str = ".lexiang";

/// 获取统一数据目录路径 `~/.lexiang/`
///
/// 首次调用时会检查旧目录 `~/.lefs/` 并提示迁移
pub fn datadir() -> PathBuf {
    let home = dirs::home_dir().expect("Cannot determine home directory");
    home.join(DATA_DIR_NAME)
}

/// 获取旧数据目录路径 `~/.lefs/`
fn legacy_datadir() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let legacy = home.join(LEGACY_DIR_NAME);
    if legacy.exists() {
        Some(legacy)
    } else {
        None
    }
}

/// 检查是否需要迁移，返回旧目录路径（如果存在）
pub fn check_migration_needed() -> Option<PathBuf> {
    legacy_datadir()
}

/// 执行数据迁移
///
/// 将 `~/.lefs/` 下的数据迁移到 `~/.lexiang/`
pub fn migrate_from_legacy() -> Result<()> {
    let Some(legacy) = legacy_datadir() else {
        return Ok(());
    };

    let target = datadir();

    // 确保目标目录存在
    fs::create_dir_all(&target)?;

    // 迁移子目录
    let subdirs = ["auth", "tools", "skills"];
    for subdir in subdirs {
        let legacy_subdir = legacy.join(subdir);
        if legacy_subdir.exists() {
            let target_subdir = target.join(subdir);

            // 如果目标已存在，跳过
            if target_subdir.exists() {
                continue;
            }

            // 复制目录
            copy_dir_all(&legacy_subdir, &target_subdir)?;
        }
    }

    // 迁移完成后重命名旧目录为备份
    let backup = legacy.with_extension("lefs.migrated");
    fs::rename(&legacy, &backup)?;

    Ok(())
}

/// 递归复制目录
fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

/// 获取 auth 目录
pub fn auth_dir() -> PathBuf {
    let dir = datadir().join("auth");
    fs::create_dir_all(&dir).ok();
    dir
}

/// 获取 tools 目录
#[allow(dead_code)]
pub fn tools_dir() -> PathBuf {
    let dir = datadir().join("tools");
    fs::create_dir_all(&dir).ok();
    dir
}

/// 获取 skills 目录
pub fn skills_dir() -> PathBuf {
    let dir = datadir().join("skills");
    fs::create_dir_all(&dir).ok();
    dir
}

/// 获取 worktrees 注册表路径
pub fn worktrees_registry_path() -> PathBuf {
    datadir().join("worktrees.json")
}

/// 获取 worktrees 目录
#[allow(dead_code)]
pub fn worktrees_dir() -> PathBuf {
    let dir = datadir().join("worktrees");
    fs::create_dir_all(&dir).ok();
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datadir_returns_lexiang() {
        let dir = datadir();
        assert!(dir.ends_with(DATA_DIR_NAME));
    }
}
