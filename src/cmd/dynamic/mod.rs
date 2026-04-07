use crate::config::Config;
use crate::mcp;
use crate::mcp::schema::{build_tool_args, CommandGenerator, McpSchemaCollection};
use anyhow::Result;

use super::output::{print_csv, print_markdown, print_table};

pub async fn handle_dynamic_command(args: &[String], schema: &McpSchemaCollection) -> Result<()> {
    let config = Config::load()?;

    let base = clap::Command::new("lx")
        .about("Lexiang CLI - A command-line tool for Lexiang MCP")
        .subcommand_required(true);

    let generator = CommandGenerator::new(schema);
    let ns_commands = generator.generate_namespaces();

    let mut cmd = base;
    for ns_cmd in ns_commands {
        cmd = cmd.subcommand(ns_cmd);
    }

    let matches = cmd.try_get_matches_from(args)?;

    let (namespace, sub_matches) = matches.subcommand().unwrap();
    let (subcommand, tool_matches) = sub_matches.subcommand().unwrap();

    let tool_name = find_tool_by_command(schema, namespace, subcommand)?;

    let tool_schema = schema
        .tools
        .get(&tool_name)
        .ok_or_else(|| anyhow::anyhow!("Tool schema not found: {}", tool_name))?;

    let mcp_args = build_tool_args(tool_matches, tool_schema);

    let access_token = crate::auth::get_access_token(&config).await?;

    let client = mcp::McpClient::new(&config.mcp.url, Some(access_token))?;
    let result = client.call_tool(&tool_name, mcp_args).await?;

    let format = tool_matches
        .get_one::<String>("format")
        .map(std::string::String::as_str)
        .unwrap_or("json-pretty");

    match format {
        "json" => println!("{}", result),
        "table" => print_table(&result),
        "yaml" => println!("{}", serde_yaml::to_string(&result)?),
        "csv" => print_csv(&result),
        "markdown" => print_markdown(&result),
        _ => println!("{}", serde_json::to_string_pretty(&result)?),
    }

    Ok(())
}

fn find_tool_by_command(
    schema: &McpSchemaCollection,
    namespace: &str,
    command: &str,
) -> Result<String> {
    use mcp::schema::{extract_command_name, extract_namespace};

    for category in &schema.categories {
        let cat_namespace = extract_namespace(&category.name);
        if cat_namespace == namespace {
            for tool in &category.tools {
                let cmd_name = extract_command_name(&tool.name, namespace);
                if cmd_name == command {
                    return Ok(tool.name.clone());
                }
            }
        }
    }

    anyhow::bail!(
        "Tool not found for namespace '{}' command '{}'",
        namespace,
        command
    )
}

pub fn print_help_with_dynamic_commands(schema: Option<&McpSchemaCollection>) {
    use mcp::schema::extract_namespace;

    println!("Lexiang CLI - A command-line tool for Lexiang MCP");
    println!();
    println!("Usage: lx [COMMAND]");
    println!();
    println!("Commands:");

    println!("  search         Search in knowledge base (shortcut for 'lexiang search')");
    println!("  fetch-doc      Fetch a document (shortcut for 'lexiang fetch-doc')");
    println!("  lexiang        Lexiang namespace commands");
    println!("  mcp            MCP operations");
    println!("  tools          Tools schema management");
    println!("  skill          Manage AI agent skill files (generate, install, uninstall)");
    println!("  git            Git-style commands for local workspace");
    println!("  worktree       Worktree management (manage multiple local workspaces)");
    println!("  completion     Generate shell completion script");
    println!("  login          Login via OAuth");
    println!("  logout         Logout and remove credentials");
    println!("  start          Start daemon with virtual filesystem");
    println!("  stop           Stop daemon");
    println!("  status         Show daemon status");
    println!("  version        Print version");
    println!("  update         Check for updates from GitHub releases");
    println!("  sh             Virtual shell for knowledge base exploration");

    if let Some(schema) = schema {
        println!();
        println!("Dynamic Commands (from MCP schema):");

        let mut namespaces: Vec<_> = schema.categories.iter().collect();
        namespaces.sort_by(|a, b| a.name.cmp(&b.name));

        for category in namespaces {
            let namespace = extract_namespace(&category.name);
            let desc = category.description.as_deref().unwrap_or("");
            let tool_count = category.tool_count;
            println!("  {namespace:14} {desc} ({tool_count} commands)");
        }
    }

    println!();
    println!("  help           Print this message or the help of the given subcommand(s)");
    println!();
    println!("Options:");
    println!("  -h, --help  Print help");
}
