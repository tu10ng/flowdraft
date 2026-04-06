/// Build IR from AST.
use std::collections::HashMap;

use anyhow::{bail, Result};
use unicode_width::UnicodeWidthStr;

use crate::parse::*;
use super::types::*;

const NODE_HEIGHT: f64 = 40.0;
const CHAR_WIDTH: f64 = 9.0;
const NODE_PADDING: f64 = 20.0;
const MIN_NODE_WIDTH: f64 = 80.0;

pub fn build_ir(doc: &Document) -> Result<DiagramIR> {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut tree_roots: Vec<TreeInfo> = Vec::new();

    for form in &doc.forms {
        match form {
            Form::Tree { direction, root, .. } => {
                let mut parent_map = HashMap::new();
                let mut children_order = HashMap::new();
                collect_tree_nodes(root, &mut nodes, &mut parent_map, &mut children_order);
                tree_roots.push(TreeInfo {
                    root: root.name.clone(),
                    direction: *direction,
                    parent_map,
                    children_order,
                });
            }
            Form::Line {
                line_style,
                from,
                arrow,
                to,
                options,
            } => {
                if !nodes.contains_key(from) {
                    bail!("line references unknown node: {}", from);
                }
                if !nodes.contains_key(to) {
                    bail!("line references unknown node: {}", to);
                }
                let mut edge_style = EdgeStyle {
                    line_style: *line_style,
                    ..Default::default()
                };
                let mut label = None;
                for (k, v) in options {
                    match k.as_str() {
                        "color" => edge_style.color = v.clone(),
                        "desc" => label = v.clone(),
                        _ => {}
                    }
                }
                edges.push(Edge {
                    from: from.clone(),
                    to: to.clone(),
                    arrow: *arrow,
                    label,
                    style: edge_style,
                });
            }
            Form::Style { target, props } => {
                if let Some(node) = nodes.get_mut(target) {
                    for (k, v) in props {
                        match k.as_str() {
                            "fill" => node.style.fill = Some(v.clone()),
                            "stroke" => node.style.stroke = Some(v.clone()),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(DiagramIR {
        nodes,
        edges,
        tree_roots,
    })
}

fn estimate_node_width(label: &str) -> f64 {
    let w = UnicodeWidthStr::width(label) as f64;
    (w * CHAR_WIDTH + NODE_PADDING).max(MIN_NODE_WIDTH)
}

fn collect_tree_nodes(
    tree_node: &TreeNode,
    nodes: &mut HashMap<String, Node>,
    parent_map: &mut HashMap<String, String>,
    children_order: &mut HashMap<String, Vec<String>>,
) {
    let label = tree_node
        .label
        .as_deref()
        .unwrap_or(&tree_node.name);
    let width = estimate_node_width(label);

    nodes.entry(tree_node.name.clone()).or_insert_with(|| Node {
        id: tree_node.name.clone(),
        label: label.to_string(),
        x: 0.0,
        y: 0.0,
        width,
        height: NODE_HEIGHT,
        style: NodeStyle::default(),
    });

    let child_names: Vec<String> = tree_node.children.iter().map(|c| c.name.clone()).collect();
    if !child_names.is_empty() {
        children_order.insert(tree_node.name.clone(), child_names);
    }

    for child in &tree_node.children {
        parent_map.insert(child.name.clone(), tree_node.name.clone());
        collect_tree_nodes(child, nodes, parent_map, children_order);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_document;

    #[test]
    fn test_build_ir_tree() {
        let doc = parse_document("(tree :down (a (b c d)))").unwrap();
        let ir = build_ir(&doc).unwrap();
        assert_eq!(ir.nodes.len(), 4);
        assert!(ir.nodes.contains_key("a"));
        assert!(ir.nodes.contains_key("b"));
        assert_eq!(ir.tree_roots.len(), 1);
        assert_eq!(ir.tree_roots[0].root, "a");
    }

    #[test]
    fn test_build_ir_with_line() {
        let doc = parse_document(r#"(tree :down (a (b c))) (line :straight b -> c :desc "test")"#).unwrap();
        let ir = build_ir(&doc).unwrap();
        assert_eq!(ir.edges.len(), 1);
        assert_eq!(ir.edges[0].from, "b");
        assert_eq!(ir.edges[0].to, "c");
        assert_eq!(ir.edges[0].label.as_deref(), Some("test"));
    }

    #[test]
    fn test_cjk_width() {
        let w = estimate_node_width("研发部");
        // CJK chars are width 2 each, so 6 * 9 + 20 = 74, but min is 80
        assert!(w >= 80.0);
    }
}
