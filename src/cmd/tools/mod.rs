use crate::config::Config;
use crate::datadir;
use crate::mcp::schema::{RuntimeSchemaManager, SchemaManager, SkillGenerator};
use crate::mcp::McpClient;
use anyhow::Result;
use std::path::PathBuf;

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
    println!("Schema saved to ~/.lefs/tools/override.json");

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
        let namespaces = manager.get_namespaces();
        if namespaces.is_empty() {
            println!("No namespaces found. Run 'lx tools sync' first.");
        } else {
            println!("Available namespaces: {}", namespaces.join(", "));
            println!("\nUse 'lx tools list --category <namespace>' to see tools in a namespace.");
        }
    }

    Ok(())
}

pub fn handle_skill(output: Option<&str>) -> Result<()> {
    let runtime = RuntimeSchemaManager::new();
    let schema = runtime
        .load()?
        .ok_or_else(|| anyhow::anyhow!("No schema found. Run 'lx tools sync' first."))?;

    let output_dir = match output {
        Some(path) => PathBuf::from(path),
        None => datadir::skills_dir(),
    };

    let generator = SkillGenerator::new(&schema, output_dir.clone());
    let files = generator.generate_all()?;

    println!("Generated {} skill files to {:?}:", files.len(), output_dir);
    for file in files {
        println!(
            "  - {}",
            file.file_name().unwrap_or_default().to_string_lossy()
        );
    }

    Ok(())
}
