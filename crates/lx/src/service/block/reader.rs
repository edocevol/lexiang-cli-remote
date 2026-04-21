//! 块树读取与查询

use super::types::{Block, BlockMatch, BlockType};
use super::BlockService;
use anyhow::{bail, Result};

#[allow(dead_code)]
impl BlockService {
    /// 获取块树
    ///
    /// - `recursive=true`: 获取所有子孙节点（嵌套结构）
    /// - `recursive=false`: 仅获取直接子节点
    pub async fn get_tree(&self, block_id: &str, recursive: bool) -> Result<Block> {
        let result = self
            .mcp
            .call_tool(
                "block_list_block_children",
                serde_json::json!({
                    "block_id": block_id,
                    "recursive": recursive,
                }),
            )
            .await?;

        // MCP 返回格式: { "data": { "blocks": [...] } } 或 { "blocks": [...] }
        let blocks_json = result
            .get("data")
            .and_then(|d| d.get("blocks"))
            .or_else(|| result.get("blocks"))
            .and_then(|b| b.as_array());

        let children = match blocks_json {
            Some(arr) => arr.iter().map(Block::from_json).collect(),
            None => Vec::new(),
        };

        // 构造一个虚拟根节点包含子块
        Ok(Block {
            id: block_id.to_string(),
            block_type: BlockType::Paragraph,
            text: None,
            content: serde_json::json!({}),
            children,
        })
    }

    /// 获取单个块详情
    pub async fn describe(&self, block_id: &str) -> Result<Block> {
        let result = self
            .mcp
            .call_tool(
                "block_describe_block",
                serde_json::json!({ "block_id": block_id }),
            )
            .await?;

        let block_json = result
            .get("data")
            .and_then(|d| d.get("block"))
            .unwrap_or(&result);

        Ok(Block::from_json(block_json))
    }

    /// 在块树中按类型查找所有匹配块
    pub async fn find_by_type(&self, root_id: &str, block_type: &BlockType) -> Result<Vec<Block>> {
        let tree = self.get_tree(root_id, true).await?;
        Ok(tree.find_by_type(block_type).into_iter().cloned().collect())
    }

    /// 在块树中按标题文本查找
    pub async fn find_by_heading(
        &self,
        root_id: &str,
        heading_text: &str,
    ) -> Result<Option<Block>> {
        let tree = self.get_tree(root_id, true).await?;
        Ok(tree.find_heading(heading_text).cloned())
    }

    /// 收集某个标题下的所有块（直到下一个同级或更高级标题）
    ///
    /// 返回: (`heading_block`, `section_blocks`)
    /// - `heading_block`: 标题块本身
    /// - `section_blocks`: 标题之后、下一个同级标题之前的所有块
    pub async fn collect_section(
        &self,
        root_id: &str,
        heading_text: &str,
    ) -> Result<(Block, Vec<Block>)> {
        let tree = self.get_tree(root_id, true).await?;

        // 在直接子节点中查找标题
        let children = &tree.children;
        let mut heading_idx = None;
        let mut heading_level = 0u8;

        for (i, child) in children.iter().enumerate() {
            if child.block_type.is_heading() {
                let clean = heading_text.trim_start_matches('#').trim();
                if let Some(ref text) = child.text {
                    if text.trim() == clean {
                        heading_idx = Some(i);
                        heading_level = child.block_type.heading_level();
                        break;
                    }
                }
            }
        }

        let Some(heading_idx) = heading_idx else {
            bail!("Heading not found: {}", heading_text);
        };

        let heading_block = children[heading_idx].clone();

        // 收集标题之后的块，直到遇到同级或更高级标题
        let mut section_blocks = Vec::new();
        for child in children.iter().skip(heading_idx + 1) {
            if child.block_type.is_heading() && child.block_type.heading_level() <= heading_level {
                break;
            }
            section_blocks.push(child.clone());
        }

        Ok((heading_block, section_blocks))
    }

    /// 在块树中按文本内容搜索（递归、不区分大小写）
    ///
    /// 支持三种搜索模式:
    /// - `text`: 子串匹配文本内容
    /// - `heading`: 精确匹配标题文本
    /// - `type`: 按块类型过滤
    pub async fn find_blocks(
        &self,
        root_id: &str,
        query: &str,
        mode: FindMode,
        entry_id: Option<&str>,
    ) -> Result<Vec<BlockMatch>> {
        let tree = self.get_tree_with_entry(root_id, entry_id, true).await?;
        match mode {
            FindMode::Text => Ok(tree.find_text(query)),
            FindMode::Heading => {
                // 精确匹配标题
                let clean = query.trim_start_matches('#').trim();
                let matches = tree
                    .find_text_recursive_heading(clean, &mut Vec::new())
                    .into_iter()
                    .map(|(block, path)| BlockMatch {
                        id: block.id.clone(),
                        block_type: block.block_type.clone(),
                        text: block.text.clone(),
                        path,
                    })
                    .collect();
                Ok(matches)
            }
            FindMode::Type => {
                let block_type = BlockType::from_str(query);
                let found = tree.find_by_type(&block_type);
                Ok(found
                    .into_iter()
                    .map(|b| BlockMatch {
                        id: b.id.clone(),
                        block_type: b.block_type.clone(),
                        text: b.text.clone(),
                        path: vec![b.id.clone()],
                    })
                    .collect())
            }
        }
    }

    /// 获取块树（带可选 `entry_id`）
    async fn get_tree_with_entry(
        &self,
        block_id: &str,
        entry_id: Option<&str>,
        recursive: bool,
    ) -> Result<Block> {
        let mut args = serde_json::json!({
            "with_descendants": recursive,
        });
        if !block_id.is_empty() {
            args["parent_block_id"] = serde_json::json!(block_id);
        }
        if let Some(eid) = entry_id {
            args["entry_id"] = serde_json::json!(eid);
        }

        let result = self
            .mcp
            .call_tool("block_list_block_children", args)
            .await?;

        // MCP 返回格式: { "data": { "blocks": [...] } } 或 { "children": [...] }
        let blocks_json = result
            .get("data")
            .and_then(|d| d.get("blocks").or_else(|| d.get("children")))
            .or_else(|| result.get("blocks"))
            .or_else(|| result.get("children"))
            .and_then(|b| b.as_array());

        let children = match blocks_json {
            Some(arr) => arr.iter().map(Block::from_json).collect(),
            None => Vec::new(),
        };

        Ok(Block {
            id: block_id.to_string(),
            block_type: BlockType::Paragraph,
            text: None,
            content: serde_json::json!({}),
            children,
        })
    }
}

/// 搜索模式
#[derive(Debug, Clone, Copy)]
pub enum FindMode {
    /// 文本子串搜索（不区分大小写）
    Text,
    /// 标题精确匹配
    Heading,
    /// 块类型过滤
    Type,
}

impl Block {
    /// 递归查找标题并返回路径
    fn find_text_recursive_heading<'a>(
        &'a self,
        clean_query: &str,
        path: &mut Vec<String>,
    ) -> Vec<(&'a Block, Vec<String>)> {
        let mut result = Vec::new();
        path.push(self.id.clone());
        if self.block_type.is_heading() {
            if let Some(ref text) = self.text {
                if text.trim() == clean_query {
                    result.push((self, path.clone()));
                }
            }
        }
        for child in &self.children {
            result.extend(child.find_text_recursive_heading(clean_query, path));
        }
        path.pop();
        result
    }
}
