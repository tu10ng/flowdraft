/// Build IR from AST.
use std::collections::HashMap;

use anyhow::{bail, Result};
use unicode_width::UnicodeWidthStr;

use crate::parse::*;
use crate::parse::expand::GroupInfo;
use crate::style::Theme;
use super::types::*;

/// Extract the base symbol name from a potentially prefixed ID.
/// e.g. "s1.eth0" -> "eth0", "cpu" -> "cpu"
fn display_name(id: &str) -> &str {
    id.rsplit('.').next().unwrap_or(id)
}

pub fn build_ir(doc: &Document, theme: &Theme, group_infos: Vec<GroupInfo>) -> Result<DiagramIR> {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut edges: Vec<Edge> = Vec::new();
    let mut tree_roots: Vec<TreeInfo> = Vec::new();
    let mut flow_graphs: Vec<FlowInfo> = Vec::new();

    // Create nodes for group children first, so Style forms can reference them
    for g in &group_infos {
        for child in &g.children {
            let label = child.label.as_deref().unwrap_or_else(|| display_name(&child.id));
            let width = estimate_node_width(label, theme.char_width, theme.node_padding, theme.min_node_width);
            nodes.entry(child.id.clone()).or_insert_with(|| Node {
                id: child.id.clone(),
                label: label.to_string(),
                x: 0.0,
                y: 0.0,
                width,
                height: theme.node_height,
                style: NodeStyle::default(),
            });
        }
    }

    for form in &doc.forms {
        match form {
            Form::Tree { direction, root, .. } => {
                let mut parent_map = HashMap::new();
                let mut children_order = HashMap::new();
                collect_tree_nodes(root, &mut nodes, &mut parent_map, &mut children_order, theme);
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
            Form::Flow { direction, options, chains } => {
                let mut adjacency = Vec::new();
                let mut node_order = Vec::new();
                for chain in chains {
                    for seg in &chain.segments {
                        if !node_order.contains(&seg.node) {
                            node_order.push(seg.node.clone());
                        }
                        nodes.entry(seg.node.clone()).or_insert_with(|| {
                            let label = display_name(&seg.node);
                            let width = estimate_node_width(label, theme.char_width, theme.node_padding, theme.min_node_width);
                            Node {
                                id: seg.node.clone(),
                                label: label.to_string(),
                                x: 0.0,
                                y: 0.0,
                                width,
                                height: theme.node_height,
                                style: NodeStyle::default(),
                            }
                        });
                    }
                    // Extract edges from consecutive segments
                    for w in chain.segments.windows(2) {
                        if let Some(arrow) = w[0].arrow {
                            adjacency.push((w[0].node.clone(), w[1].node.clone(), arrow));
                        }
                    }
                }
                let line_aware = !options.iter().any(|(k, _)| k == "no-line-aware");
                flow_graphs.push(FlowInfo {
                    direction: *direction,
                    adjacency,
                    node_order,
                    line_aware,
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
            Form::Define(_) => {
                // Define forms are consumed by expand_defines() before build_ir.
                // If one reaches here, it's a no-op.
            }
        }
    }

    let groups = group_infos
        .into_iter()
        .map(|g| Group {
            id: g.id.clone(),
            label: g.label,
            children: g.children.into_iter().map(|c| c.id).collect(),
            direction: g.direction,
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            style: NodeStyle::default(),
        })
        .collect();

    Ok(DiagramIR {
        nodes,
        edges,
        tree_roots,
        flow_graphs,
        groups,
    })
}

fn estimate_node_width(label: &str, char_width: f64, node_padding: f64, min_node_width: f64) -> f64 {
    let w = UnicodeWidthStr::width(label) as f64;
    (w * char_width + node_padding).max(min_node_width)
}

fn collect_tree_nodes(
    tree_node: &TreeNode,
    nodes: &mut HashMap<String, Node>,
    parent_map: &mut HashMap<String, String>,
    children_order: &mut HashMap<String, Vec<String>>,
    theme: &Theme,
) {
    let label = tree_node
        .label
        .as_deref()
        .unwrap_or_else(|| display_name(&tree_node.name));
    let width = estimate_node_width(label, theme.char_width, theme.node_padding, theme.min_node_width);

    nodes.entry(tree_node.name.clone()).or_insert_with(|| Node {
        id: tree_node.name.clone(),
        label: label.to_string(),
        x: 0.0,
        y: 0.0,
        width,
        height: theme.node_height,
        style: NodeStyle::default(),
    });

    let child_names: Vec<String> = tree_node.children.iter().map(|c| c.name.clone()).collect();
    if !child_names.is_empty() {
        children_order.insert(tree_node.name.clone(), child_names);
    }

    for child in &tree_node.children {
        parent_map.insert(child.name.clone(), tree_node.name.clone());
        collect_tree_nodes(child, nodes, parent_map, children_order, theme);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_document;

    #[test]
    fn test_build_ir_tree() {
        let doc = parse_document("(tree :down (a (b c d)))").unwrap();
        let ir = build_ir(&doc, &Theme::default(), vec![]).unwrap();
        assert_eq!(ir.nodes.len(), 4);
        assert!(ir.nodes.contains_key("a"));
        assert!(ir.nodes.contains_key("b"));
        assert_eq!(ir.tree_roots.len(), 1);
        assert_eq!(ir.tree_roots[0].root, "a");
    }

    #[test]
    fn test_build_ir_with_line() {
        let doc = parse_document(r#"(tree :down (a (b c))) (line :straight b -> c :desc "test")"#).unwrap();
        let ir = build_ir(&doc, &Theme::default(), vec![]).unwrap();
        assert_eq!(ir.edges.len(), 1);
        assert_eq!(ir.edges[0].from, "b");
        assert_eq!(ir.edges[0].to, "c");
        assert_eq!(ir.edges[0].label.as_deref(), Some("test"));
    }

    #[test]
    fn test_cjk_width() {
        let theme = Theme::default();
        let w = estimate_node_width("研发部", theme.char_width, theme.node_padding, theme.min_node_width);
        // CJK chars are width 2 each, so 6 * 9 + 20 = 74, but min is 80
        assert!(w >= 80.0);
    }

    #[test]
    fn test_build_ir_flow() {
        let doc = parse_document("(flow :right (a -> b) (b -> c -> d) (b -> e))").unwrap();
        let ir = build_ir(&doc, &Theme::default(), vec![]).unwrap();
        assert_eq!(ir.flow_graphs.len(), 1);
        let fg = &ir.flow_graphs[0];
        assert_eq!(fg.node_order, vec!["a", "b", "c", "d", "e"]);
        assert_eq!(fg.adjacency.len(), 4); // a->b, b->c, c->d, b->e
        // All nodes created
        assert_eq!(ir.nodes.len(), 5);
        assert!(ir.nodes.contains_key("a"));
        assert!(ir.nodes.contains_key("e"));
    }

    #[test]
    fn test_build_ir_flow_shared_nodes() {
        let doc = parse_document("(flow :right (a -> b) (a -> c) (b -> d) (c -> d))").unwrap();
        let ir = build_ir(&doc, &Theme::default(), vec![]).unwrap();
        // d appears in two chains but should only be one node
        assert_eq!(ir.nodes.len(), 4);
        assert_eq!(ir.flow_graphs[0].adjacency.len(), 4);
    }
}
