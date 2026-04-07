pub mod dynamic;
pub mod embedded;
pub mod generator;
pub mod runtime;
pub mod skill_generator;
pub mod types;

pub use generator::{build_tool_args, CommandGenerator};
pub use runtime::RuntimeSchemaManager;
pub use skill_generator::SkillGenerator;
pub use types::*;

use crate::mcp::ToolSchema;
use std::collections::HashMap;

/// Schema 管理器 - 合并三层 schema（embedded < override < custom）
pub struct SchemaManager {
    /// 编译时嵌入的 schema
    embedded: HashMap<String, ToolSchema>,
    /// 运行时动态获取的 schema
    dynamic: HashMap<String, ToolSchema>,
    /// 完整的 schema collection（包含 category 信息）
    collection: Option<McpSchemaCollection>,
}

impl SchemaManager {
    pub fn new() -> Self {
        Self {
            embedded: embedded::load_embedded_schemas(),
            dynamic: HashMap::new(),
            collection: None,
        }
    }

    /// 从运行时配置加载
    pub fn load_from_runtime() -> Self {
        let mut manager = Self::new();

        // 尝试加载运行时 schema
        let runtime = RuntimeSchemaManager::new();
        if let Ok(Some(collection)) = runtime.load() {
            manager.collection = Some(collection);
        } else {
            // Fallback 到 embedded schema
            manager.collection = embedded::load_embedded_collection();
        }

        manager
    }

    /// 获取工具 schema（兼容旧接口）
    pub fn get_tool_schema(&self, name: &str) -> Option<ToolSchema> {
        self.dynamic
            .get(name)
            .cloned()
            .or_else(|| self.embedded.get(name).cloned())
    }

    /// 更新动态 schema（兼容旧接口）
    pub fn update_dynamic(&mut self, tools: Vec<ToolSchema>) {
        self.dynamic.clear();
        for tool in tools {
            self.dynamic.insert(tool.name.clone(), tool);
        }
    }

    /// 提取字段列表（兼容旧接口）
    pub fn extract_fields(&self, name: &str) -> Vec<String> {
        self.get_tool_schema(name)
            .and_then(|schema| schema.input_schema)
            .map(|input_schema| input_schema.properties.keys().cloned().collect())
            .unwrap_or_default()
    }

    // === 新接口 ===

    /// 获取完整的 `McpToolSchema`
    #[allow(dead_code)]
    pub fn get_mcp_tool_schema(&self, name: &str) -> Option<&McpToolSchema> {
        self.collection.as_ref()?.tools.get(name)
    }

    /// 获取所有 namespace
    pub fn get_namespaces(&self) -> Vec<String> {
        self.collection
            .as_ref()
            .map(types::McpSchemaCollection::get_namespaces)
            .unwrap_or_default()
    }

    /// 获取 namespace 下的所有工具
    pub fn get_tools_by_namespace(&self, namespace: &str) -> Vec<&McpToolSchema> {
        self.collection
            .as_ref()
            .map(|c| c.get_tools_by_namespace(namespace))
            .unwrap_or_default()
    }

    /// 获取所有 category
    pub fn get_categories(&self) -> Vec<&McpCategory> {
        self.collection
            .as_ref()
            .map(|c| c.categories.iter().collect())
            .unwrap_or_default()
    }

    /// 设置 schema collection
    #[allow(dead_code)]
    pub fn set_collection(&mut self, collection: McpSchemaCollection) {
        self.collection = Some(collection);
    }

    /// 检查是否有 schema
    #[allow(dead_code)]
    pub fn has_schema(&self) -> bool {
        self.collection.is_some() || !self.embedded.is_empty() || !self.dynamic.is_empty()
    }
}

impl Default for SchemaManager {
    fn default() -> Self {
        Self::new()
    }
}
