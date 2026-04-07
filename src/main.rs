mod auth;
mod cmd;
mod config;
mod daemon;
mod datadir;
pub mod shell;
mod vfs;
mod worktree;

mod json_rpc;

mod mcp;

use clap::Parser;
use cmd::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 检查并执行数据目录迁移
    if let Some(legacy_dir) = datadir::check_migration_needed() {
        eprintln!("检测到旧数据目录: {}", legacy_dir.display());
        eprintln!("正在迁移到: {}", datadir::datadir().display());
        match datadir::migrate_from_legacy() {
            Ok(()) => eprintln!("迁移完成"),
            Err(e) => eprintln!("迁移失败（将继续使用新目录）: {}", e),
        }
    }

    let args: Vec<String> = std::env::args().collect();

    // 加载 schema（用于动态命令）
    let schema = cmd::load_schema();

    // 检查是否是 --help 或 -h（需要显示包含动态命令的帮助）
    let is_help = args.iter().any(|a| a == "--help" || a == "-h");
    let is_root_help = is_help && args.len() == 2;

    if is_root_help {
        // 显示包含动态命令的帮助
        cmd::print_help_with_dynamic_commands(schema.as_ref());
        return Ok(());
    }

    // 检查是否是动态命令
    if args.len() >= 2 {
        if let Some(ref schema) = schema {
            let potential_namespace = &args[1];
            if schema
                .get_namespaces()
                .contains(&potential_namespace.to_string())
            {
                // 这是一个动态命令，构建并执行
                return cmd::handle_dynamic_command(&args, schema).await;
            }
        }
    }

    // 常规命令处理
    let cli = Cli::parse();
    let config = config::Config::load()?;

    match cli.command {
        Some(Commands::Version) => {
            println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        }
        Some(Commands::Login) => {
            let _token = auth::login().await?;
            println!("登录成功");
        }
        Some(Commands::Logout) => {
            auth::logout()?;
            println!("已登出");
        }
        Some(Commands::Start { mount, size }) => {
            use std::path::PathBuf;
            let mount_point = mount.map(PathBuf::from);
            let daemon = daemon::DaemonManager::new(mount_point, Some(size));
            daemon.start()?;
        }
        Some(Commands::Stop) => {
            let daemon = daemon::DaemonManager::new(None, None);
            daemon.stop()?;
        }
        Some(Commands::Status) => {
            let daemon = daemon::DaemonManager::new(None, None);
            let status = daemon.status()?;
            if status.running {
                println!("守护进程运行中 (PID: {:?})", status.pid);
                if let Some(vfs) = status.vfs_status {
                    println!("虚拟文件系统: {:?}", vfs.mount_point);
                    println!("大小: {}MB", vfs.size_mb);
                }
            } else {
                println!("守护进程未运行");
            }
        }
        Some(Commands::Mcp { command }) => match command {
            cmd::McpCommands::List => cmd::list_tools(&config).await?,
            cmd::McpCommands::Call { name, params } => {
                let params: serde_json::Value = params
                    .map(|p| serde_json::from_str(&p))
                    .transpose()?
                    .unwrap_or(serde_json::json!({}));
                cmd::call_tool(&config, &name, params).await?;
            }
        },
        Some(Commands::Tools { command }) => match command {
            cmd::ToolsCommands::Sync => cmd::handle_sync(&config).await?,
            cmd::ToolsCommands::Categories => cmd::handle_categories()?,
            cmd::ToolsCommands::Version => cmd::handle_version()?,
            cmd::ToolsCommands::List { category } => cmd::handle_list(category.as_deref())?,
            cmd::ToolsCommands::Skill { output } => cmd::handle_skill(output.as_deref())?,
        },
        Some(Commands::Worktree { command }) => {
            cmd::git::handle_workspace_command(command, &config).await?;
        }
        Some(Commands::Git { command }) => {
            cmd::handle_git_command(command, &config).await?;
        }
        Some(Commands::Completion { shell }) => {
            Cli::generate_completion(shell);
        }
        Some(Commands::Sh { space, path, exec }) => {
            if let Some(command) = exec {
                // 单次执行模式
                let mut bash =
                    cmd::exec_command(&config, space.as_deref(), path.as_deref()).await?;
                let output = bash.exec(&command).await?;
                if !output.stdout.is_empty() {
                    print!("{}", output.stdout);
                }
                if !output.stderr.is_empty() {
                    eprint!("{}", output.stderr);
                }
                std::process::exit(output.exit_code);
            } else {
                // REPL 交互模式
                cmd::start_repl(&config, space.as_deref(), path.as_deref()).await?;
            }
        }
        None => println!("Use 'lx --help' for usage"),
    }

    Ok(())
}
