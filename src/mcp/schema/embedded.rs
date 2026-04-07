use crate::mcp::schema::types::{extract_command_name, extract_namespace, McpSchemaCollection};
use crate::mcp::ToolSchema;
use std::collections::HashMap;

/// 编译时嵌入的 schema JSON
const EMBEDDED_SCHEMA_JSON: &str = include_str!("embedded_schema.json");

/// 加载嵌入的 schema 到 HashMap（兼容旧接口）
pub fn load_embedded_schemas() -> HashMap<String, ToolSchema> {
    let mut map = HashMap::new();

    if let Ok(collection) = serde_json::from_str::<McpSchemaCollection>(EMBEDDED_SCHEMA_JSON) {
        for (name, tool) in collection.tools {
            // 转换 McpToolSchema 到 ToolSchema
            let input_schema = tool.input_schema.map(|s| {
                let mut properties = serde_json::Map::new();
                for (k, v) in s.properties {
                    properties.insert(k, serde_json::to_value(v).unwrap_or_default());
                }
                crate::mcp::InputSchema {
                    type_: s.type_,
                    properties,
                    required: s.required,
                }
            });

            map.insert(
                name.clone(),
                ToolSchema {
                    name,
                    description: tool.description,
                    input_schema,
                },
            );
        }
    }

    map
}

/// 加载嵌入的完整 schema collection
pub fn load_embedded_collection() -> Option<McpSchemaCollection> {
    let mut collection: McpSchemaCollection = serde_json::from_str(EMBEDDED_SCHEMA_JSON).ok()?;

    // 反序列化后 namespace 和 command_name 是 None（因为 #[serde(skip)]）
    // 需要从 categories 重新填充
    for category in &collection.categories {
        let namespace = extract_namespace(&category.name);
        for cat_tool in &category.tools {
            if let Some(tool) = collection.tools.get_mut(&cat_tool.name) {
                tool.namespace = Some(namespace.clone());
                tool.command_name = Some(extract_command_name(&cat_tool.name, &namespace));
            }
        }
    }

    Some(collection)
}
