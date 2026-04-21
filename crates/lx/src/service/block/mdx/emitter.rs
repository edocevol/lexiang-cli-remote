//! `DocIR` → MDX Emitter
//!
//! Serializes a `DocIR` tree back to MDX text.
//! Output follows AI Ingest Spec formatting rules:
//! - Multi-line component syntax with 4-space indent
//! - Double-quoted attributes
//! - Boolean attributes without values

#![allow(dead_code)]

use crate::service::block::ir::{BlockAttrs, InlineStyle, Node, NodeType};

/// Serialize a `DocIR` node to MDX text
pub fn emit_mdx(node: &Node) -> String {
    let mut emitter = Emitter::default();
    emitter.emit_node(node, 0);
    emitter.output.trim_end().to_string()
}

#[derive(Default)]
struct Emitter {
    output: String,
}

impl Emitter {
    fn push(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn push_indent(&mut self, depth: usize) {
        for _ in 0..depth * 4 {
            self.output.push(' ');
        }
    }

    fn newline(&mut self) {
        self.output.push('\n');
    }

    fn emit_node(&mut self, node: &Node, depth: usize) {
        match &node.node_type {
            NodeType::Document => self.emit_document(node, depth),
            NodeType::Paragraph => self.emit_paragraph(node, depth),
            NodeType::Heading { level } => self.emit_heading(*level, node, depth),
            NodeType::BlockQuote => self.emit_block_quote(node, depth),
            NodeType::Callout { color, icon } => {
                self.emit_callout(color.as_deref(), icon.as_deref(), node, depth);
            }
            NodeType::ColumnList => self.emit_column_list(node, depth),
            NodeType::Column { width_ratio } => self.emit_column(*width_ratio, node, depth),
            NodeType::Divider => self.emit_divider(depth),
            NodeType::Image {
                file_id,
                caption,
                align,
                ..
            } => self.emit_image(
                file_id.as_deref(),
                caption.as_deref(),
                align.as_deref(),
                depth,
            ),
            NodeType::Table => self.emit_table(node, depth),
            NodeType::TableRow => self.emit_table_row(node, depth),
            NodeType::TableCell { .. } => self.emit_table_cell(node, depth),
            NodeType::Task { done, .. } => self.emit_todo(*done, node, depth),
            NodeType::BulletedList => self.emit_bullet_item(node, depth),
            NodeType::NumberedList => self.emit_numbered_item(node, depth),
            NodeType::CodeBlock { language } => self.emit_code_block(
                language.as_deref(),
                node.text.as_deref().unwrap_or(""),
                depth,
            ),
            NodeType::MathBlock { .. } => self.emit_math_block(node, depth),
            NodeType::Toggle => self.emit_toggle(node, depth),
            NodeType::Mermaid { .. } => self.emit_mermaid(node, depth),
            NodeType::PlantUml { .. } => self.emit_plantuml(node, depth),
            NodeType::SmartSheet { .. } => self.emit_placeholder("smartsheet", node),
            NodeType::Attachment { .. } => self.emit_placeholder("attachment", node),
            NodeType::Video { .. } => self.emit_placeholder("video", node),
            NodeType::Text => self.emit_text_node(node),
            NodeType::Link { href } => self.emit_link(href, node),
        }
    }

    fn emit_document(&mut self, node: &Node, _depth: usize) {
        if !self.attrs_is_empty(&node.attrs) {
            self.emit_frontmatter(&node.attrs);
        }

        for (i, child) in node.children.iter().enumerate() {
            if i > 0 {
                self.newline();
                self.newline();
            }
            self.emit_node(child, 0);
        }
    }

    fn attrs_is_empty(&self, attrs: &BlockAttrs) -> bool {
        attrs.text_align.is_none()
            && attrs.block_color.is_none()
            && attrs.border_color.is_none()
            && attrs.icon.is_none()
            && attrs.width.is_none()
            && attrs.height.is_none()
    }

    fn emit_frontmatter(&mut self, attrs: &BlockAttrs) {
        self.push("---\n");
        if let Some(ref ta) = attrs.text_align {
            self.push(&format!("textAlign: {ta}\n"));
        }
        if let Some(ref bc) = attrs.block_color {
            self.push(&format!("blockColor: {bc}\n"));
        }
        if let Some(ref brc) = attrs.border_color {
            self.push(&format!("borderColor: {brc}\n"));
        }
        if let Some(ref icon) = attrs.icon {
            self.push(&format!("icon: \"{icon}\"\n"));
        }
        self.push("---\n");
    }

    fn emit_paragraph(&mut self, node: &Node, _depth: usize) {
        self.emit_inline_children(&node.children);
    }

    fn emit_heading(&mut self, level: u8, node: &Node, _depth: usize) {
        self.push(&"#".repeat(level as usize));
        self.push(" ");
        self.emit_inline_children(&node.children);
    }

    fn emit_block_quote(&mut self, node: &Node, _depth: usize) {
        let lines = self.collect_inline_lines(node);
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                self.newline();
            }
            self.push("> ");
            self.push(line);
        }
    }

    fn emit_callout(&mut self, color: Option<&str>, icon: Option<&str>, node: &Node, depth: usize) {
        self.push("<Callout");
        if let Some(ico) = icon {
            self.push(&format!(" icon=\"{ico}\""));
        }
        if let Some(bc) = color {
            self.push(&format!(" borderColor=\"{bc}\""));
        }
        self.push(">");
        self.newline();

        for child in &node.children {
            self.push_indent(depth + 1);
            self.emit_node(child, depth + 1);
            self.newline();
        }

        self.push("</Callout>");
    }

    fn emit_column_list(&mut self, node: &Node, depth: usize) {
        self.push("<ColumnList>");
        self.newline();

        for child in &node.children {
            self.push_indent(depth + 1);
            self.emit_node(child, depth + 1);
            self.newline();
        }

        self.push("</ColumnList>");
    }

    fn emit_column(&mut self, width_ratio: Option<f64>, node: &Node, depth: usize) {
        match width_ratio {
            Some(wr) => self.push(&format!("<Column width=\"{:.0}%\">", wr * 100.0)),
            None => self.push("<Column>"),
        }
        self.newline();

        for child in &node.children {
            self.push_indent(depth + 1);
            self.emit_node(child, depth + 1);
            self.newline();
        }

        self.push("</Column>");
    }

    fn emit_divider(&mut self, _depth: usize) {
        self.push("---");
    }

    fn emit_image(
        &mut self,
        file_id: Option<&str>,
        caption: Option<&str>,
        _align: Option<&str>,
        _depth: usize,
    ) {
        let src = file_id.unwrap_or("");
        match caption {
            Some(a) => self.push(&format!("![{a}]({src})")),
            None => self.push(&format!("![]({src})")),
        }
    }

    fn emit_table(&mut self, node: &Node, depth: usize) {
        self.push("<Table>");
        self.newline();

        for child in &node.children {
            self.push_indent(depth + 1);
            self.emit_node(child, depth + 1);
            self.newline();
        }

        self.push("</Table>");
    }

    fn emit_table_row(&mut self, node: &Node, depth: usize) {
        self.push("<TableRow>");
        self.newline();

        for child in &node.children {
            self.push_indent(depth + 1);
            self.emit_node(child, depth + 1);
            self.newline();
        }

        self.push("</TableRow>");
    }

    fn emit_table_cell(&mut self, node: &Node, _depth: usize) {
        self.push("<TableCell>");
        self.emit_inline_children(&node.children);
        self.push("</TableCell>");
    }

    fn emit_todo(&mut self, done: bool, node: &Node, _depth: usize) {
        let checkbox = if done { "[x]" } else { "[ ]" };
        self.push(checkbox);
        self.push(" ");
        // Task name from task_name field, fallback to children content
        if let Some(ref name) = node.task_name {
            self.push(name);
        } else {
            self.emit_inline_children(&node.children);
        }
    }

    fn emit_bullet_item(&mut self, node: &Node, _depth: usize) {
        self.push("- ");
        self.emit_inline_children(&node.children);
    }

    fn emit_numbered_item(&mut self, node: &Node, _depth: usize) {
        self.push("1. ");
        self.emit_inline_children(&node.children);
    }

    fn emit_code_block(&mut self, language: Option<&str>, code: &str, _depth: usize) {
        self.push("```");
        if let Some(lang) = language {
            self.push(lang);
        }
        self.newline();
        self.push(code);
        self.newline();
        self.push("```");
    }

    fn emit_math_block(&mut self, node: &Node, depth: usize) {
        self.push("<MathBlock");
        self.push(">");
        self.newline();
        if let Some(ref t) = node.text {
            self.push_indent(depth + 1);
            self.push(t);
            self.newline();
        }
        self.push("</MathBlock>");
    }

    fn emit_toggle(&mut self, node: &Node, depth: usize) {
        self.push("<Toggle>");
        self.newline();
        for child in &node.children {
            self.push_indent(depth + 1);
            self.emit_node(child, depth + 1);
            self.newline();
        }
        self.push("</Toggle>");
    }

    fn emit_mermaid(&mut self, node: &Node, _depth: usize) {
        self.push("```mermaid");
        self.newline();
        self.push(node.diagram_content.as_deref().unwrap_or(""));
        self.newline();
        self.push("```");
    }

    fn emit_plantuml(&mut self, node: &Node, _depth: usize) {
        self.push("```plantuml");
        self.newline();
        self.push(node.diagram_content.as_deref().unwrap_or(""));
        self.newline();
        self.push("```");
    }

    fn emit_placeholder(&mut self, name: &str, node: &Node) {
        self.push(&format!("[{name}]"));
        if let Some(ref t) = node.text {
            self.push(" ");
            self.push(t);
        }
    }

    fn emit_text_node(&mut self, node: &Node) {
        let text = node.text.as_deref().unwrap_or("");
        match node.inline_style.as_ref() {
            None => self.push(text),
            Some(style) if style.is_plain() => self.push(text),
            Some(style) => self.emit_styled_text(text, style),
        }
    }

    fn emit_styled_text(&mut self, text: &str, style: &InlineStyle) {
        self.push("<Mark");
        if style.bold {
            self.push(" bold");
        }
        if style.italic {
            self.push(" italic");
        }
        if style.underline {
            self.push(" underline");
        }
        if style.strike_through {
            self.push(" strikeThrough");
        }
        self.push(">");
        self.push(text);
        self.push("</Mark>");
    }

    fn emit_link(&mut self, href: &str, node: &Node) {
        if node.children.len() == 1 && matches!(node.children[0].node_type, NodeType::Text) {
            let text = node.children[0].text.as_deref().unwrap_or("");
            self.push(&format!("[{text}]({href})"));
        } else {
            self.push(&format!("<Link href=\"{href}\">"));
            self.emit_inline_children(&node.children);
            self.push("</Link>");
        }
    }

    fn emit_inline_children(&mut self, children: &[Node]) {
        for child in children {
            self.emit_node(child, 0);
        }
    }

    fn collect_inline_lines<'a>(&'a mut self, node: &'a Node) -> Vec<String> {
        let mut buf = String::new();
        self.collect_inline_recursive(node, &mut buf);
        vec![buf]
    }

    fn collect_inline_recursive(&mut self, node: &Node, buf: &mut String) {
        match &node.node_type {
            NodeType::Text => {
                buf.push_str(node.text.as_deref().unwrap_or(""));
            }
            NodeType::Link { href } => {
                let text = node
                    .children
                    .iter()
                    .map(super::super::ir::Node::plain_content)
                    .collect::<String>();
                buf.push_str(&format!("[{text}]({href})"));
            }
            _ => {
                for child in &node.children {
                    self.collect_inline_recursive(child, buf);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_simple_paragraph() {
        let node = Node::document(vec![Node::paragraph(vec![Node::plain_text("Hello")])]);
        let mdx = emit_mdx(&node);
        assert_eq!(mdx, "Hello");
    }

    #[test]
    fn test_emit_heading() {
        let node = Node::document(vec![Node::heading(2, vec![Node::plain_text("Title")])]);
        let mdx = emit_mdx(&node);
        assert_eq!(mdx, "## Title");
    }

    #[test]
    fn test_emit_code_block() {
        let node = Node::document(vec![Node::code_block(Some("rust"), "fn main() {}")]);
        let mdx = emit_mdx(&node);
        assert!(mdx.contains("```rust"));
        assert!(mdx.contains("fn main() {}"));
    }

    #[test]
    fn test_emit_todo() {
        let node = Node::document(vec![
            Node::task(true, "done item"),
            Node::task(false, "undone"),
        ]);
        let mdx = emit_mdx(&node);
        assert!(mdx.contains("[x] done item"));
        assert!(mdx.contains("[ ] undone"));
    }

    #[test]
    fn test_emit_bullet_list() {
        let node = Node::document(vec![
            Node::bullet_item(vec![Node::plain_text("first")]),
            Node::bullet_item(vec![Node::plain_text("second")]),
        ]);
        let mdx = emit_mdx(&node);
        assert!(mdx.contains("- first"));
        assert!(mdx.contains("- second"));
    }

    #[test]
    fn test_emit_divider() {
        let node = Node::document(vec![Node::divider()]);
        let mdx = emit_mdx(&node);
        assert_eq!(mdx, "---");
    }

    #[test]
    fn test_emit_callout() {
        let icon = "\u{1f6a7}";
        let bc = "red";
        let node = Node::document(vec![Node::callout(
            Some(bc),
            Some(icon),
            vec![Node::paragraph(vec![Node::plain_text("Warning message")])],
        )]);
        let mdx = emit_mdx(&node);
        assert!(mdx.contains("<Callout"));
        assert!(mdx.contains("icon=\"\u{1f6a7}\""));
        assert!(mdx.contains("borderColor=\"red\""));
        assert!(mdx.contains("Warning message"));
        assert!(mdx.contains("</Callout>"));
    }

    #[test]
    fn test_emit_image() {
        let src = "https://example.com/img.png";
        let alt = "example";
        let node = Node::document(vec![Node::image(Some(src), Some(alt))]);
        let mdx = emit_mdx(&node);
        assert_eq!(mdx, "![example](https://example.com/img.png)");
    }

    #[test]
    fn test_emit_bold_italic() {
        let style = InlineStyle {
            italic: true,
            ..Default::default()
        };
        let italic_node = Node {
            node_type: NodeType::Text,
            text: Some("italic".to_string()),
            attrs: Default::default(),
            inline_style: Some(style),
            href: None,
            children: vec![],
            ..Default::default()
        };
        let node = Node::document(vec![Node::paragraph(vec![
            Node::bold("bold text"),
            Node::plain_text(" and "),
            italic_node,
        ])]);
        let mdx = emit_mdx(&node);
        assert!(mdx.contains("<Mark bold>bold text</Mark>"));
        assert!(mdx.contains(" and "));
        assert!(mdx.contains("<Mark italic>italic</Mark>"));
    }

    #[test]
    fn test_emit_link() {
        let href = "https://example.com";
        let child = Node::plain_text("click me");
        let node = Node::document(vec![Node::paragraph(vec![Node::link(href, vec![child])])]);
        let mdx = emit_mdx(&node);
        assert_eq!(mdx, "[click me](https://example.com)");
    }

    #[test]
    fn test_emit_frontmatter() {
        let mut doc = Node::document(vec![Node::paragraph(vec![Node::plain_text("content")])]);
        doc.attrs.text_align = Some("center".to_string());
        doc.attrs.icon = Some("\u{1f4c4}".to_string());
        let mdx = emit_mdx(&doc);
        assert!(mdx.contains("---"));
        assert!(mdx.contains("textAlign: center"));
        assert!(mdx.contains("icon: \"\u{1f4c4}\""));
        assert!(mdx.contains("content"));
    }

    #[test]
    fn test_roundtrip_simple() {
        let original = "# Hello\n\nThis is **bold**.\n\n- Item one\n\n```rust\nlet x = 1;\n```";
        let doc = crate::service::block::mdx::parser::parse_mdx(original).unwrap();
        let emitted = emit_mdx(&doc);
        let doc2 = crate::service::block::mdx::parser::parse_mdx(&emitted).unwrap();
        assert_eq!(doc.children.len(), doc2.children.len());
    }
}
