use crate::config::Config;
use crate::mcp::schema::{RuntimeSchemaManager, SchemaManager};
use crate::mcp::McpClient;
use anyhow::Result;

pub async fn handle_sync(config: &Config) -> Result<()> {
    println!("Syncing tool schema from MCP Server...");

    let access_token = crate::auth::get_access_token(config).await?;
    let client = McpClient::new(&config.mcp.url, Some(access_token))?;
    let runtime = RuntimeSchemaManager::new();
    let schema = runtime.sync_from_server(&client).await?;

    println!(
        "Synced {} tools in {} categories",
        schema.tools.len(),
        schema.categories.len()
    );
    println!("Schema saved to ~/.lexiang/tools/override.json");

    Ok(())
}

pub fn handle_categories() -> Result<()> {
    let manager = SchemaManager::load_from_runtime();
    let categories = manager.get_categories();

    if categories.is_empty() {
        println!("No categories found. Run 'lx tools sync' first.");
    } else {
        println!("Tool Categories ({}):", categories.len());
        for cat in categories {
            let desc = cat.description.as_deref().unwrap_or("");
            println!("  {} ({} tools) - {}", cat.name, cat.tool_count, desc);
        }
    }

    Ok(())
}

pub fn handle_version() -> Result<()> {
    let runtime = RuntimeSchemaManager::new();
    let info = runtime.get_version_info();
    println!("{}", info);
    Ok(())
}

pub fn handle_list(category: Option<&str>) -> Result<()> {
    let manager = SchemaManager::load_from_runtime();

    if let Some(cat) = category {
        let tools = manager.get_tools_by_namespace(cat);
        if tools.is_empty() {
            println!(
                "No tools found in category '{}'. Run 'lx tools sync' first.",
                cat
            );
        } else {
            println!("Tools in '{}' ({}):", cat, tools.len());
            for tool in tools {
                let desc = tool.description.as_deref().unwrap_or("");
                println!("  {} - {}", tool.name, desc);
            }
        }
    } else {
        let categories = manager.get_categories();
        if categories.is_empty() {
            println!("No categories found. Run 'lx tools sync' first.");
        } else {
            let names: Vec<_> = categories.iter().map(|c| c.name.as_str()).collect();
            println!("Available categories: {}", names.join(", "));
            println!("\nUse 'lx tools list --category <name>' to see tools in a category.");
        }
    }

    Ok(())
}
