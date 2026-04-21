//! `DocIR` ↔ Block Adapter
//!
//! Converts between `DocIR` (intermediate representation) and MCP Block JSON.
//! Aligned with xiaokeai MCP Server's `block_create_block_descendant` schema.
//!
//! - `ir_to_descendant()`: `DocIR` → full descendant JSON for MCP API
//! - `block_to_ir()`: Block JSON (from MCP) → `DocIR`

#![allow(dead_code)]

use super::ir::{InlineStyle, Node, NodeType};
use crate::service::block::types::{Block, BlockType};

// ============================================================
// Forward: DocIR → Block JSON (for block_create_block_descendant)
// ============================================================

/// Convert `DocIR` node tree to a complete descendant JSON structure.
///
/// Output format matches MCP `block_create_block_descendant` schema:
/// ```json
/// {
///   "block_type": "p",
///   "content": { ... },
///   "children": [ ... ]
/// }
/// ```
pub fn ir_to_descendant(node: &Node) -> serde_json::Value {
    match &node.node_type {
        NodeType::Document => ir_document(node),
        _ => ir_block(node),
    }
}

fn ir_document(node: &Node) -> serde_json::Value {
    let children: Vec<serde_json::Value> = node.children.iter().map(ir_block).collect();
    serde_json::json!({
        "type": "document",
        "content": {},
        "children": children,
    })
}

/// Core conversion: single `DocIR` node → MCP Block JSON
fn ir_block(node: &Node) -> serde_json::Value {
    match &node.node_type {
        NodeType::Paragraph => ir_paragraph(&node.children),
        NodeType::Heading { level } => ir_heading(*level, &node.children),
        NodeType::BlockQuote => ir_quote(&node.children),

        // === Container types (content in children[]) ===
        NodeType::Callout { icon, color } => {
            ir_callout(color.as_deref(), icon.as_deref(), &node.children)
        }
        NodeType::ColumnList => ir_column_list(&node.children),
        NodeType::Column { width_ratio } => ir_column(*width_ratio, &node.children),
        NodeType::Table => ir_table(&node.children),
        NodeType::TableRow => ir_table_row(&node.children),
        NodeType::TableCell {
            align,
            background_color,
            col_span,
            row_span,
            vertical_align,
        } => ir_table_cell(
            align.as_deref(),
            background_color.as_deref(),
            col_span.unwrap_or(0),
            row_span.unwrap_or(0),
            vertical_align.as_deref(),
            &node.children,
        ),

        // === Leaf types ===
        NodeType::BulletedList => ir_bullet_list(&node.children),
        NodeType::NumberedList => ir_numbered_list(&node.children),
        NodeType::Task { done, .. } => ir_task(*done, node.task_name.as_deref(), &node.children),
        NodeType::CodeBlock { language } => {
            ir_code_block(language.as_deref(), node.text.as_deref().unwrap_or(""))
        }
        NodeType::Divider => ir_divider(),
        NodeType::Image {
            file_id,
            caption,
            width,
            height,
            align,
        } => ir_image(
            file_id.as_deref(),
            caption.as_deref(),
            *width,
            *height,
            align.as_deref(),
        ),
        NodeType::Toggle => ir_toggle(&node.children),
        NodeType::Mermaid { .. } | NodeType::PlantUml { .. } => ir_diagram(
            &node.node_type,
            node.diagram_content.as_deref().unwrap_or(""),
        ),
        NodeType::SmartSheet { smartsheet_id } => ir_smartsheet(smartsheet_id.as_deref()),
        NodeType::Attachment {
            file_id,
            session_id,
            view_type,
        } => ir_attachment(
            file_id.as_deref(),
            session_id.as_deref(),
            view_type.as_deref(),
        ),
        NodeType::Video {
            file_id,
            width,
            height,
            align,
        } => ir_video(file_id.as_deref(), *width, *height, align.as_deref()),

        // === Inline/auxiliary (should not be top-level blocks) ===
        NodeType::Text | NodeType::Link { .. } | NodeType::MathBlock { .. } => {
            // Wrap as paragraph
            ir_paragraph(std::slice::from_ref(node))
        }
        NodeType::Document => ir_document(node),
    }
}

// ---- Inline text conversion (MCP elements[] format) ----

/// Convert `DocIR` inline nodes to MCP `content.text[].elements[]` format
fn ir_inlines(inlines: &[Node]) -> Vec<serde_json::Value> {
    inlines.iter().flat_map(ir_inline_element).collect()
}

fn ir_inline_element(node: &Node) -> Vec<serde_json::Value> {
    match &node.node_type {
        NodeType::Text => {
            let text = node.text.as_deref().unwrap_or("");
            if text.is_empty() {
                return vec![];
            }

            let mut text_run = serde_json::json!({ "content": text });
            let mut style = serde_json::json!({});
            let mut has_style = false;

            if let Some(ref is) = node.inline_style {
                if is.bold {
                    style["bold"] = true.into();
                    has_style = true;
                }
                if is.italic {
                    style["italic"] = true.into();
                    has_style = true;
                }
                if is.underline {
                    style["underline"] = true.into();
                    has_style = true;
                }
                if is.strike_through {
                    style["strikethrough"] = true.into();
                    has_style = true;
                }
                if is.inline_code {
                    style["inline_code"] = true.into();
                    has_style = true;
                }
                if let Some(ref c) = is.link {
                    style["link"] = c.clone().into();
                    has_style = true;
                }
                if let Some(ref c) = is.text_color {
                    style["text_color"] = c.clone().into();
                    has_style = true;
                }
                if let Some(ref c) = is.background_color {
                    style["background_color"] = c.clone().into();
                    has_style = true;
                }
            }

            if has_style {
                text_run["text_style"] = style;
            }
            vec![serde_json::json!({
                "text_run": text_run,
            })]
        }
        NodeType::Link { href } => {
            let link_text: String = node
                .children
                .iter()
                .map(super::ir::Node::plain_content)
                .collect();
            if link_text.is_empty() && node.text.is_none() {
                return vec![];
            }
            let display_text = if !link_text.is_empty() {
                &link_text
            } else {
                node.text.as_deref().unwrap_or("")
            };

            let mut text_run = serde_json::json!({ "content": display_text });
            text_run["text_style"] = serde_json::json!({ "link": href });
            vec![serde_json::json!({ "text_run": text_run })]
        }
        _ => {
            let text = node.plain_content();
            if text.is_empty() {
                return vec![];
            }
            vec![serde_json::json!({
                "text_run": { "content": text },
            })]
        }
    }
}

// ---- Block type converters (matching real MCP schema) ----

fn ir_paragraph(inlines: &[Node]) -> serde_json::Value {
    serde_json::json!({
        "block_type": "p",
        "content": {
            "text": { "elements": ir_inlines(inlines) }
        },
        "children": []
    })
}

fn ir_heading(level: u8, inlines: &[Node]) -> serde_json::Value {
    let type_str = format!("h{}", level);
    serde_json::json!({
        "block_type": type_str,
        "content": {
            "text": { "elements": ir_inlines(inlines) }
        },
        "children": []
    })
}

fn ir_quote(inlines: &[Node]) -> serde_json::Value {
    serde_json::json!({
        "block_type": "quote",
        "content": {
            "text": { "elements": ir_inlines(inlines) }
        },
        "children": []
    })
}

/// Callout — container type with children sub-blocks
/// Real schema: type=callout, content={ color?, icon?, callout:true }, children=[...]
fn ir_callout(color: Option<&str>, icon: Option<&str>, children: &[Node]) -> serde_json::Value {
    let mut content = serde_json::json!({
        "callout": true,
    });
    if let Some(c) = color {
        content["color"] = c.into();
    }
    if let Some(i) = icon {
        content["icon"] = i.into();
    }

    let child_json: Vec<serde_json::Value> = children.iter().map(ir_block).collect();
    serde_json::json!({
        "block_type": "callout",
        "content": content,
        "children": child_json
    })
}

fn ir_bullet_list(inlines: &[Node]) -> serde_json::Value {
    serde_json::json!({
        "block_type": "bulleted_list",
        "content": {
            "text": { "elements": ir_inlines(inlines) }
        },
        "children": []
    })
}

fn ir_numbered_list(inlines: &[Node]) -> serde_json::Value {
    serde_json::json!({
        "block_type": "numbered_list",
        "content": {
            "text": { "elements": ir_inlines(inlines) }
        },
        "children": []
    })
}

/// Task — content: { name, done, assignees?, `due_at`? }
fn ir_task(done: bool, name: Option<&str>, _inlines: &[Node]) -> serde_json::Value {
    let mut content = serde_json::json!({
        "done": done,
    });
    if let Some(n) = name {
        content["name"] = n.into();
    }
    serde_json::json!({
        "block_type": "task",
        "content": content,
        "children": []
    })
}

fn ir_code_block(language: Option<&str>, code: &str) -> serde_json::Value {
    let lang = language.unwrap_or("");
    serde_json::json!({
        "block_type": "code",
        "content": {
            "language": lang,
            "text": code
        },
        "children": []
    })
}

/// Image — content: { `file_id`, caption?, width?, height?, align? }
fn ir_image(
    file_id: Option<&str>,
    caption: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
    align: Option<&str>,
) -> serde_json::Value {
    let mut content = serde_json::json!({});
    if let Some(fid) = file_id {
        content["file_id"] = fid.into();
    }
    if let Some(c) = caption {
        content["caption"] = c.into();
    }
    if let Some(w) = width {
        content["width"] = serde_json::json!(w);
    }
    if let Some(h) = height {
        content["height"] = serde_json::json!(h);
    }
    if let Some(a) = align {
        content["align"] = a.into();
    }
    serde_json::json!({
        "block_type": "image",
        "content": content,
        "children": []
    })
}

fn ir_divider() -> serde_json::Value {
    serde_json::json!({
        "block_type": "divider",
        "content": {},
        "children": []
    })
}

fn ir_toggle(children: &[Node]) -> serde_json::Value {
    let child_json: Vec<serde_json::Value> = children.iter().map(ir_block).collect();
    serde_json::json!({
        "block_type": "toggle",
        "content": {},
        "children": child_json
    })
}

/// Table — content: {}, children: [`table_row`]
fn ir_table(rows: &[Node]) -> serde_json::Value {
    let child_json: Vec<serde_json::Value> = rows.iter().map(ir_block).collect();
    serde_json::json!({
        "block_type": "table",
        "content": {
            "column_size": 0,
            "row_size": rows.len() as i64,
        },
        "children": child_json
    })
}

fn ir_table_row(cells: &[Node]) -> serde_json::Value {
    let child_json: Vec<serde_json::Value> = cells.iter().map(ir_block).collect();
    serde_json::json!({
        "block_type": "table_row",
        "content": {},
        "children": child_json
    })
}

/// Table Cell — container with children, optional alignment/span attrs
fn ir_table_cell(
    align: Option<&str>,
    bg_color: Option<&str>,
    col_span: u32,
    row_span: u32,
    v_align: Option<&str>,
    children: &[Node],
) -> serde_json::Value {
    let mut content = serde_json::json!({});
    if let Some(a) = align {
        content["align"] = a.into();
    }
    if let Some(bg) = bg_color {
        content["background_color"] = bg.into();
    }
    if col_span > 0 {
        content["col_span"] = serde_json::json!(col_span);
    }
    if row_span > 0 {
        content["row_span"] = serde_json::json!(row_span);
    }
    if let Some(v) = v_align {
        content["vertical_align"] = v.into();
    }

    let child_json: Vec<serde_json::Value> = children.iter().map(ir_block).collect();
    serde_json::json!({
        "block_type": "table_cell",
        "content": content,
        "children": child_json
    })
}

fn ir_column_list(columns: &[Node]) -> serde_json::Value {
    let child_json: Vec<serde_json::Value> = columns.iter().map(ir_block).collect();
    serde_json::json!({
        "block_type": "column_list",
        "content": {},
        "children": child_json
    })
}

/// Column — content: { `width_ratio`: number }, children: [...blocks...]
fn ir_column(width_ratio: Option<f64>, children: &[Node]) -> serde_json::Value {
    let mut content = serde_json::json!({});
    if let Some(wr) = width_ratio {
        content["width_ratio"] = serde_json::json!(wr);
    }
    let child_json: Vec<serde_json::Value> = children.iter().map(ir_block).collect();
    serde_json::json!({
        "block_type": "column",
        "content": content,
        "children": child_json
    })
}

/// Mermaid / `PlantUML` diagram — content: { content: string }
fn ir_diagram(node_type: &NodeType, code: &str) -> serde_json::Value {
    let bt = match node_type {
        NodeType::Mermaid { .. } => "mermaid",
        NodeType::PlantUml { .. } => "plantuml",
        _ => return serde_json::json!({ "block_type": "unknown", "content": {}, "children": [] }),
    };
    serde_json::json!({
        "block_type": bt,
        "content": { "content": code },
        "children": []
    })
}

fn ir_smartsheet(smartsheet_id: Option<&str>) -> serde_json::Value {
    let mut content = serde_json::json!({});
    if let Some(sid) = smartsheet_id {
        content["smartsheet_id"] = sid.into();
    }
    serde_json::json!({
        "block_type": "smartsheet",
        "content": content,
        "children": []
    })
}

/// Attachment — content: { `file_id`?, `session_id`?, `view_type`? }
fn ir_attachment(
    file_id: Option<&str>,
    session_id: Option<&str>,
    view_type: Option<&str>,
) -> serde_json::Value {
    let mut content = serde_json::json!({});
    if let Some(fid) = file_id {
        content["file_id"] = fid.into();
    }
    if let Some(sid) = session_id {
        content["session_id"] = sid.into();
    }
    if let Some(vt) = view_type {
        content["view_type"] = vt.into();
    }
    serde_json::json!({
        "block_type": "attachment",
        "content": content,
        "children": []
    })
}

/// Video — content: { `file_id`, width?, height?, align? }
fn ir_video(
    file_id: Option<&str>,
    width: Option<u32>,
    height: Option<u32>,
    align: Option<&str>,
) -> serde_json::Value {
    let mut content = serde_json::json!({});
    if let Some(fid) = file_id {
        content["file_id"] = fid.into();
    }
    if let Some(w) = width {
        content["width"] = serde_json::json!(w);
    }
    if let Some(h) = height {
        content["height"] = serde_json::json!(h);
    }
    if let Some(a) = align {
        content["align"] = a.into();
    }
    serde_json::json!({
        "block_type": "video",
        "content": content,
        "children": []
    })
}

// ============================================================
// Reverse: Block JSON (MCP) → DocIR
// ============================================================

/// Convert MCP Block array into a `DocIR` Document node
pub fn block_to_ir(blocks: &[Block]) -> Node {
    let children: Vec<Node> = blocks.iter().map(block_node_to_ir).collect();
    Node::document(children)
}

fn block_node_to_ir(block: &Block) -> Node {
    match &block.block_type {
        BlockType::Paragraph => {
            let inlines = extract_elements(block.content.get("text"));
            Node::paragraph(inlines)
        }
        BlockType::H1 => Node::heading(1, extract_elements(block.content.get("text"))),
        BlockType::H2 => Node::heading(2, extract_elements(block.content.get("text"))),
        BlockType::H3 => Node::heading(3, extract_elements(block.content.get("text"))),
        BlockType::H4 => Node::heading(4, extract_elements(block.content.get("text"))),
        BlockType::H5 => Node::heading(5, extract_elements(block.content.get("text"))),
        BlockType::Quote => {
            let is_callout = block
                .content
                .get("callout")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            if is_callout {
                let color = block
                    .content
                    .get("color")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let icon = block
                    .content
                    .get("icon")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let children: Vec<Node> = block.children.iter().map(block_node_to_ir).collect();
                Node::callout(color.as_deref(), icon.as_deref(), children)
            } else {
                Node::quote(extract_elements(block.content.get("text")))
            }
        }
        BlockType::Callout => {
            let color = block
                .content
                .get("color")
                .and_then(|v| v.as_str())
                .map(String::from);
            let icon = block
                .content
                .get("icon")
                .and_then(|v| v.as_str())
                .map(String::from);
            let children: Vec<Node> = block.children.iter().map(block_node_to_ir).collect();
            Node::callout(color.as_deref(), icon.as_deref(), children)
        }
        BlockType::BulletList | BlockType::ListItem => {
            Node::bullet_item(extract_elements(block.content.get("text")))
        }
        BlockType::NumberedList => Node::numbered_item(extract_elements(block.content.get("text"))),
        BlockType::Task => {
            let done = block
                .content
                .get("done")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let name = block
                .content
                .get("name")
                .and_then(|v| v.as_str())
                .map(String::from);
            Node::task(done, name.unwrap_or_default())
        }
        BlockType::Code => {
            let language = block.content.get("language").and_then(|v| v.as_str());
            let code = block.text.clone().unwrap_or_default();
            Node::code_block(language, &code)
        }
        BlockType::Image => {
            let file_id = block
                .content
                .get("file_id")
                .and_then(|v| v.as_str())
                .map(String::from);
            let caption = block
                .content
                .get("caption")
                .and_then(|v| v.as_str())
                .or(block.text.as_deref())
                .map(String::from);
            Node::image(file_id.as_deref(), caption.as_deref())
        }
        BlockType::Divider => Node::divider(),
        BlockType::Table => {
            let rows: Vec<Node> = block.children.iter().map(block_node_to_ir).collect();
            Node::table(rows)
        }
        BlockType::TableRow => {
            let cells: Vec<Node> = block.children.iter().map(block_node_to_ir).collect();
            Node::table_row(cells)
        }
        BlockType::TableCell => Node::table_cell(extract_elements(block.content.get("text"))),
        BlockType::ColumnList => {
            let cols: Vec<Node> = block.children.iter().map(block_node_to_ir).collect();
            Node::column_list(cols)
        }
        BlockType::Column => {
            let width_ratio = block
                .content
                .get("width_ratio")
                .and_then(serde_json::Value::as_f64);
            let children: Vec<Node> = block.children.iter().map(block_node_to_ir).collect();
            Node::column(width_ratio, children)
        }
        BlockType::Toggle => {
            let children: Vec<Node> = block.children.iter().map(block_node_to_ir).collect();
            Node::toggle(children)
        }
        BlockType::Mermaid => {
            let code = block
                .content
                .get("content")
                .and_then(|v| v.as_str())
                .or(block.text.as_deref())
                .unwrap_or("")
                .to_string();
            Node::mermaid(&code)
        }
        BlockType::PlantUml => {
            let code = block
                .content
                .get("content")
                .and_then(|v| v.as_str())
                .or(block.text.as_deref())
                .unwrap_or("")
                .to_string();
            Node::plantuml(&code)
        }
        BlockType::Attachment => Node::plain_text(block.text.as_deref().unwrap_or("[attachment]")),
        BlockType::Video => Node::plain_text(block.text.as_deref().unwrap_or("[video]")),
        BlockType::Unknown(s) => Node::plain_text(format!("[unknown block type: {s}]")),
    }
}

/// Extract inline Nodes from MCP `content.text.elements[]` or fallback to plain string
fn extract_elements(text_field: Option<&serde_json::Value>) -> Vec<Node> {
    match text_field {
        None | Some(serde_json::Value::Array(_)) => vec![],
        Some(serde_json::Value::String(s)) => vec![Node::plain_text(s.clone())],
        Some(obj) if obj.is_object() => {
            // Try elements array first
            if let Some(arr) = obj
                .get("elements")
                .or_else(|| obj.get("text"))
                .and_then(|v| v.as_array())
            {
                arr.iter().filter_map(extract_element).collect()
            } else {
                // Fallback: treat as plain text
                vec![Node::plain_text(format!("{obj}"))]
            }
        }
        _ => vec![],
    }
}

/// Extract from modern MCP elements[{ `text_run`: {...} }] format
fn extract_element(value: &serde_json::Value) -> Option<Node> {
    let text_run = value.get("text_run")?;
    let content = text_run.get("content")?.as_str()?.to_string();
    if content.is_empty() {
        return None;
    }

    let style = text_run.get("text_style").map(extract_text_style);
    let inline_style = style.filter(|s| !s.is_plain());

    Some(Node {
        node_type: NodeType::Text,
        text: Some(content),
        inline_style,
        ..Default::default()
    })
}

/// Legacy: extract from old text[] format (backward compat)
fn extract_element_legacy(value: &serde_json::Value) -> Option<Node> {
    let t = value.get("type")?.as_str()?;
    match t {
        "text" => {
            let text = value.get("text")?.as_str()?.to_string();
            let bold = value.get("bold")?.as_bool().unwrap_or(false);
            let italic = value
                .get("italic")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let strike = value
                .get("strikeThrough")
                .or_else(|| value.get("strikethrough"))
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            let underline = value
                .get("underline")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);

            let style = if bold || italic || strike || underline {
                Some(InlineStyle {
                    bold,
                    italic,
                    underline,
                    strike_through: strike,
                    ..Default::default()
                })
            } else {
                None
            };

            Some(Node {
                node_type: NodeType::Text,
                text: Some(text),
                inline_style: style,
                ..Default::default()
            })
        }
        "mention" => Some(Node::plain_text(
            value
                .get("text")?
                .as_str()
                .unwrap_or("@unknown")
                .to_string(),
        )),
        "equation" => Some(Node::plain_text(format!(
            "${}$",
            value.get("text")?.as_str().unwrap_or("")
        ))),
        _ => None,
    }
}

fn extract_text_style(style_obj: &serde_json::Value) -> InlineStyle {
    InlineStyle {
        bold: style_obj
            .get("bold")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        italic: style_obj
            .get("italic")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        underline: style_obj
            .get("underline")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        strike_through: style_obj
            .get("strikethrough")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        inline_code: style_obj
            .get("inline_code")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        link: style_obj
            .get("link")
            .and_then(|v| v.as_str())
            .map(String::from),
        text_color: style_obj
            .get("text_color")
            .and_then(|v| v.as_str())
            .map(String::from),
        background_color: style_obj
            .get("background_color")
            .and_then(|v| v.as_str())
            .map(String::from),
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_paragraph_to_json() {
        let node = Node::paragraph(vec![Node::plain_text("hello "), Node::bold("world")]);
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "p");
        assert_eq!(
            json["content"]["text"]["elements"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            json["content"]["text"]["elements"][0]["text_run"]["content"],
            "hello "
        );
        assert_eq!(
            json["content"]["text"]["elements"][1]["text_run"]["content"],
            "world"
        );
        assert_eq!(
            json["content"]["text"]["elements"][1]["text_run"]["text_style"]["bold"],
            true
        );
    }

    #[test]
    fn test_ir_heading_to_json() {
        let node = Node::heading(3, vec![Node::plain_text("Subtitle")]);
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "h3");
        assert_eq!(
            json["content"]["text"]["elements"][0]["text_run"]["content"],
            "Subtitle"
        );
    }

    #[test]
    fn test_ir_callout_to_json() {
        let node = Node::callout(
            Some("red"),
            Some("🚧"),
            vec![Node::paragraph(vec![Node::plain_text("Warning")])],
        );
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "callout");
        assert_eq!(json["content"]["callout"], true);
        assert_eq!(json["content"]["color"], "red");
        assert_eq!(json["content"]["icon"], "🚧");
        // Children should contain the paragraph inside
        assert_eq!(json["children"][0]["block_type"], "p");
    }

    #[test]
    fn test_ir_callout_with_icon_only() {
        let node = Node::callout(None, Some("📝"), vec![Node::plain_text("Note")]);
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "callout");
        assert_eq!(json["content"]["icon"], "📝");
        assert!(json["content"].get("color").is_none());
    }

    #[test]
    fn test_ir_todo_to_json() {
        let done = Node::task(true, "Completed task");
        let json = ir_block(&done);
        assert_eq!(json["block_type"], "task");
        assert_eq!(json["content"]["done"], true);
        assert_eq!(json["content"]["name"], "Completed task");

        let undone = Node::task(false, "Pending task");
        let json2 = ir_block(&undone);
        assert_eq!(json2["content"]["done"], false);
    }

    #[test]
    fn test_ir_code_block_to_json() {
        let node = Node::code_block(Some("typescript"), "const x = 1;");
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "code");
        assert_eq!(json["content"]["language"], "typescript");
        assert_eq!(json["content"]["text"], "const x = 1;");
    }

    #[test]
    fn test_ir_image_to_json() {
        let node = Node::image(Some("fid_123"), Some("alt text"));
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "image");
        assert_eq!(json["content"]["file_id"], "fid_123");
        assert_eq!(json["content"]["caption"], "alt text");
    }

    #[test]
    fn test_ir_column_to_json() {
        let col = Node::column(
            Some(0.5),
            vec![Node::paragraph(vec![Node::plain_text("A")])],
        );
        let json = ir_block(&col);
        assert_eq!(json["block_type"], "column");
        assert_eq!(json["content"]["width_ratio"], 0.5);
        assert_eq!(json["children"][0]["block_type"], "p");
    }

    #[test]
    fn test_ir_mermaid_to_json() {
        let node = Node::mermaid("graph LR\n  A --> B");
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "mermaid");
        assert_eq!(json["content"]["content"], "graph LR\n  A --> B");
    }

    #[test]
    fn test_ir_toggle_to_json() {
        let node = Node::toggle(vec![Node::paragraph(vec![Node::plain_text(
            "hidden content",
        )])]);
        let json = ir_block(&node);
        assert_eq!(json["block_type"], "toggle");
        assert_eq!(json["children"][0]["block_type"], "p");
    }

    #[test]
    fn test_ir_document_to_json() {
        let doc = Node::document(vec![
            Node::heading(1, vec![Node::plain_text("Title")]),
            Node::paragraph(vec![Node::plain_text("Body")]),
        ]);
        let json = ir_to_descendant(&doc);
        assert_eq!(json["type"], "document");
        assert_eq!(json["children"].as_array().unwrap().len(), 2);
        assert_eq!(json["children"][0]["block_type"], "h1");
        assert_eq!(json["children"][1]["block_type"], "p");
    }

    #[test]
    fn test_full_roundtrip_mdx_parse_adapter() {
        let mdx = r#"# Title

This is **important**.

<Callout icon="🚧" borderColor="red">
## Warning
Check before proceeding.
</Callout>

- [ ] Task A
- [x] Task B

```rust
fn main() {}
```

---
"#;

        let doc = crate::service::block::mdx::parser::parse_mdx(mdx).unwrap();
        let json = ir_to_descendant(&doc);
        let children = json["children"].as_array().unwrap();

        assert!(
            children.len() >= 7,
            "expected at least 7 children, got {}",
            children.len()
        );

        // h1
        assert_eq!(children[0]["block_type"], "h1");

        // Paragraph with bold
        let para = &children[1];
        assert_eq!(para["block_type"], "p");
        let inlines = para["content"]["text"]["elements"].as_array().unwrap();
        assert_eq!(inlines.len(), 3); // "This is ", bold("important"), "."
        assert_eq!(inlines[1]["text_run"]["text_style"]["bold"], true);

        // Callout as container type
        let callout_idx = children.iter().position(|c| {
            c["block_type"] == "callout"
                || (c["block_type"] == "quote" && c["content"].get("callout").is_some())
        });
        assert!(
            callout_idx.is_some(),
            "should have a callout/quote+callout block"
        );

        // Todos
        let todos: Vec<_> = children
            .iter()
            .filter(|c| c["block_type"] == "task")
            .collect();
        assert_eq!(todos.len(), 2);

        // Code block
        assert!(children.iter().any(|c| c["block_type"] == "code"));

        // Divider
        assert!(children.iter().any(|c| c["block_type"] == "divider"));
    }

    #[test]
    fn test_block_to_ir_simple() {
        let blocks = vec![
            Block {
                id: "b1".to_string(),
                block_type: BlockType::H2,
                text: Some("Heading".to_string()),
                content: serde_json::json!({"text": "Heading"}),
                children: vec![],
            },
            Block {
                id: "b2".to_string(),
                block_type: BlockType::Paragraph,
                text: Some("Content here.".to_string()),
                content: serde_json::json!({"text": "Content here."}),
                children: vec![],
            },
        ];
        let doc = block_to_ir(&blocks);
        assert_eq!(doc.children.len(), 2);
        assert_eq!(doc.children[0].node_type, NodeType::Heading { level: 2 });
        assert_eq!(doc.children[1].node_type, NodeType::Paragraph);
    }

    #[test]
    fn test_block_to_ir_with_styles_modern() {
        let block = Block {
            id: "b1".to_string(),
            block_type: BlockType::Paragraph,
            text: None,
            content: serde_json::json!({
                "text": {
                    "elements": [
                        {"text_run": {"content": "normal "}},
                        {"text_run": {"content": "bold", "text_style": {"bold": true}}},
                        {"text_run": {"content": " end"}}
                    ]
                }
            }),
            children: vec![],
        };
        let doc = block_to_ir(std::slice::from_ref(&block));
        let p = &doc.children[0];
        assert_eq!(p.children.len(), 3);
        assert_eq!(p.children[0].plain_content(), "normal ");
        assert!(p.children[0].inline_style.is_none());
        assert!(p.children[1].inline_style.as_ref().unwrap().bold);
    }

    #[test]
    fn test_block_to_ir_callout_container() {
        let block = Block {
            id: "b1".to_string(),
            block_type: BlockType::Callout,
            text: None,
            content: serde_json::json!({
                "color": "#FF0000",
                "icon": "\u{1f4dd}",
                "callout": true
            }),
            children: vec![Block {
                id: "b2".to_string(),
                block_type: BlockType::Paragraph,
                text: Some("Inner paragraph".to_string()),
                content: serde_json::json!({"text": "Inner paragraph"}),
                children: vec![],
            }],
        };
        let doc = block_to_ir(std::slice::from_ref(&block));
        let co = &doc.children[0];
        assert!(matches!(co.node_type, NodeType::Callout { .. }));
        if let NodeType::Callout { color, icon } = &co.node_type {
            assert_eq!(color.as_deref(), Some("#FF0000"));
            assert_eq!(icon.as_deref(), Some("📝"));
        }
        // Callout should have inner child
        assert_eq!(co.children.len(), 1);
        assert_eq!(co.children[0].node_type, NodeType::Paragraph);
    }
}
