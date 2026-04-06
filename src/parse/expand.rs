/// Expand define/instantiate templates in the AST.
use std::collections::HashMap;

use anyhow::{bail, Result};

use super::ast::*;

/// Info about a child node within a group.
#[derive(Debug, Clone)]
pub struct GroupChildInfo {
    pub id: String,
    pub label: Option<String>,
}

/// Info about a group created by template instantiation.
#[derive(Debug, Clone)]
pub struct GroupInfo {
    pub id: String,
    pub label: String,
    pub children: Vec<GroupChildInfo>,
    pub direction: Option<Direction>,
    pub style_props: Vec<(String, String)>,
}

/// Expand all define/instantiate patterns in the document.
/// Returns the rewritten document and a list of groups for rendering.
pub fn expand_defines(doc: Document) -> Result<(Document, Vec<GroupInfo>)> {
    // 1. Collect define templates
    let mut templates: HashMap<String, DefineTemplate> = HashMap::new();
    for form in &doc.forms {
        if let Form::Define(def) = form {
            templates.insert(def.name.clone(), def.clone());
        }
    }

    if templates.is_empty() {
        return Ok((doc, Vec::new()));
    }

    // 2. Expand forms, removing Define forms
    let mut new_forms = Vec::new();
    let mut groups = Vec::new();

    for form in doc.forms {
        match form {
            Form::Define(_) => {} // consumed
            Form::Tree { direction, options, root } => {
                let root = expand_tree_node(root, &templates, &mut groups)?;
                new_forms.push(Form::Tree { direction, options, root });
            }
            Form::Flow { direction, options, chains } => {
                let mut new_chains = Vec::new();
                for chain in chains {
                    let mut new_segments = Vec::new();
                    for mut seg in chain.segments {
                        if let Some(inline) = seg.inline_node.take() {
                            let expanded = expand_tree_node(inline, &templates, &mut groups)?;
                            seg.node = expanded.name;
                            seg.inline_node = None;
                        }
                        new_segments.push(seg);
                    }
                    new_chains.push(FlowChain { segments: new_segments });
                }
                new_forms.push(Form::Flow { direction, options, chains: new_chains });
            }
            other => new_forms.push(other),
        }
    }

    Ok((Document { forms: new_forms }, groups))
}

/// Expand a tree node. If its name matches a template, instantiate it.
fn expand_tree_node(
    node: TreeNode,
    templates: &HashMap<String, DefineTemplate>,
    groups: &mut Vec<GroupInfo>,
) -> Result<TreeNode> {
    if let Some(tmpl) = templates.get(&node.name) {
        // This node is a template instantiation.
        // Expected: (server s1 "S1") or (server s1 :name "S1")
        // Parsed as: name="server", children=[s1, "S1"] or name="server", children=[s1] with keywords
        //
        // The first child's name is the instance_id.
        // Remaining children that are leaf nodes (no children of their own) are positional args.
        // Keywords on the original node are keyword args.
        if node.children.is_empty() {
            bail!(
                "template '{}' instantiation requires at least an instance id",
                tmpl.name
            );
        }

        let instance_id = node.children[0].name.clone();
        let instance_label = node.label.clone();

        // Collect arguments: positional from remaining leaf children, keyword from labels
        let mut args = HashMap::new();
        let mut pos_args = Vec::new();

        for child in &node.children[1..] {
            // A leaf child with no children is a positional arg (its name or label is the value)
            if child.children.is_empty() {
                // Check if it looks like a keyword arg: child.label is set means :key val was parsed
                // Actually in the current parser, (server s1 :name "S1") would parse :name as keyword
                // and "S1" as label of s1 or as a separate child. Let's handle positional args.
                let val = child.label.as_deref().unwrap_or(&child.name);
                pos_args.push(val.to_string());
            }
        }

        // Map positional args to param names
        for (i, param) in tmpl.params.iter().enumerate() {
            if let Some(val) = pos_args.get(i) {
                args.insert(param.clone(), val.clone());
            }
        }

        // Expand template body with prefix and parameter substitution
        let mut group_children = Vec::new();

        for body_node in &tmpl.body {
            let expanded = instantiate_body_node(body_node, &instance_id, &args)?;
            // Recursively expand in case template body uses other templates
            let expanded = expand_tree_node(expanded, templates, groups)?;
            group_children.push(GroupChildInfo {
                id: expanded.name.clone(),
                label: expanded.label.clone(),
            });
        }

        groups.push(GroupInfo {
            id: instance_id.clone(),
            label: instance_label
                .clone()
                .unwrap_or_else(|| instance_id.clone()),
            children: group_children,
            direction: None,
            style_props: Vec::new(),
        });

        // Instance node becomes a leaf — body nodes are in GroupInfo, not tree children
        Ok(TreeNode {
            name: instance_id,
            label: instance_label,
            children: vec![],
        })
    } else {
        // Not a template — recursively expand children
        let mut new_children = Vec::new();
        for child in node.children {
            new_children.push(expand_tree_node(child, templates, groups)?);
        }
        Ok(TreeNode {
            name: node.name,
            label: node.label,
            children: new_children,
        })
    }
}

/// Instantiate a single body node: prefix its ID and substitute parameters in labels.
fn instantiate_body_node(
    node: &TreeNode,
    prefix: &str,
    args: &HashMap<String, String>,
) -> Result<TreeNode> {
    let new_name = format!("{}.{}", prefix, node.name);
    let new_label = node.label.as_ref().map(|l| substitute_params(l, args));

    let mut new_children = Vec::new();
    for child in &node.children {
        new_children.push(instantiate_body_node(child, prefix, args)?);
    }

    Ok(TreeNode {
        name: new_name,
        label: new_label,
        children: new_children,
    })
}

/// Replace ${param} placeholders with argument values.
fn substitute_params(text: &str, args: &HashMap<String, String>) -> String {
    let mut result = text.to_string();
    for (key, val) in args {
        result = result.replace(&format!("${{{}}}", key), val);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_document;

    #[test]
    fn test_expand_simple_define() {
        let input = r#"
            (define server (params name)
                (cpu :label "${name} CPU")
                (eth0 :label "ETH0"))
            (tree :down
                (rack
                    (server s1 "S1")
                    (server s2 "S2")))
        "#;
        let doc = parse_document(input).unwrap();
        let (expanded, groups) = expand_defines(doc).unwrap();

        // Define form should be removed
        assert_eq!(expanded.forms.len(), 1);

        // Check expanded tree
        match &expanded.forms[0] {
            Form::Tree { root, .. } => {
                assert_eq!(root.name, "rack");
                assert_eq!(root.children.len(), 2);

                let s1 = &root.children[0];
                assert_eq!(s1.name, "s1");
                // Body nodes are no longer tree children
                assert_eq!(s1.children.len(), 0);

                let s2 = &root.children[1];
                assert_eq!(s2.name, "s2");
                assert_eq!(s2.children.len(), 0);
            }
            _ => panic!("expected tree form"),
        }

        // Check groups
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].id, "s1");
        assert_eq!(groups[0].children[0].id, "s1.cpu");
        assert_eq!(groups[0].children[0].label.as_deref(), Some("S1 CPU"));
        assert_eq!(groups[0].children[1].id, "s1.eth0");
        assert_eq!(groups[1].id, "s2");
        assert_eq!(groups[1].children[0].id, "s2.cpu");
        assert_eq!(groups[1].children[0].label.as_deref(), Some("S2 CPU"));
    }

    #[test]
    fn test_expand_no_defines() {
        let input = "(tree :down (a b c))";
        let doc = parse_document(input).unwrap();
        let (expanded, groups) = expand_defines(doc).unwrap();
        assert_eq!(expanded.forms.len(), 1);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_expand_with_line_and_style() {
        let input = r##"
            (define server (params name)
                (cpu :label "${name} CPU")
                (eth0 :label "ETH0"))
            (tree :down
                (rack
                    (server s1 "S1")
                    (server s2 "S2")))
            (line :straight s1.eth0 -> s2.eth0 :desc "link")
            (style s1.cpu :fill "#e8f4fd")
        "##;
        let doc = parse_document(input).unwrap();
        let (expanded, _groups) = expand_defines(doc).unwrap();

        // 3 forms: tree + line + style
        assert_eq!(expanded.forms.len(), 3);

        // Line and style should pass through unchanged
        match &expanded.forms[1] {
            Form::Line { from, to, .. } => {
                assert_eq!(from, "s1.eth0");
                assert_eq!(to, "s2.eth0");
            }
            _ => panic!("expected line form"),
        }
        match &expanded.forms[2] {
            Form::Style { target, .. } => {
                assert_eq!(target, "s1.cpu");
            }
            _ => panic!("expected style form"),
        }
    }

    #[test]
    fn test_substitute_params() {
        let mut args = HashMap::new();
        args.insert("name".to_string(), "S1".to_string());
        assert_eq!(substitute_params("${name} CPU", &args), "S1 CPU");
        assert_eq!(substitute_params("no params", &args), "no params");
    }

    #[test]
    fn test_expand_define_no_params() {
        let input = r#"
            (define box
                (top :label "Top")
                (bottom :label "Bottom"))
            (tree :down
                (root
                    (box b1)))
        "#;
        let doc = parse_document(input).unwrap();
        let (expanded, groups) = expand_defines(doc).unwrap();

        match &expanded.forms[0] {
            Form::Tree { root, .. } => {
                assert_eq!(root.children.len(), 1);
                let b1 = &root.children[0];
                assert_eq!(b1.name, "b1");
                // Body nodes are no longer tree children
                assert_eq!(b1.children.len(), 0);
            }
            _ => panic!("expected tree form"),
        }

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].id, "b1");
        assert_eq!(groups[0].children.len(), 2);
        assert_eq!(groups[0].children[0].id, "b1.top");
        assert_eq!(groups[0].children[1].id, "b1.bottom");
    }

    #[test]
    fn test_expand_flow_inline_template() {
        let input = r##"
            (define server (params name)
                (cpu :label "${name} CPU")
                (eth0 :label "ETH0"))
            (flow :right
                (rack -> (server s3 "S3") -> (server s4 "S4")))
        "##;
        let doc = parse_document(input).unwrap();
        let (expanded, groups) = expand_defines(doc).unwrap();

        // Flow form should have inline_node cleared, node set to instance_id
        match &expanded.forms[0] {
            Form::Flow { chains, .. } => {
                let segs = &chains[0].segments;
                assert_eq!(segs[0].node, "rack");
                assert_eq!(segs[1].node, "s3");
                assert!(segs[1].inline_node.is_none());
                assert_eq!(segs[2].node, "s4");
                assert!(segs[2].inline_node.is_none());
            }
            _ => panic!("expected flow form"),
        }

        // Groups should be created for s3 and s4
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].id, "s3");
        assert_eq!(groups[0].children[0].id, "s3.cpu");
        assert_eq!(groups[0].children[0].label, Some("S3 CPU".to_string()));
        assert_eq!(groups[0].children[1].id, "s3.eth0");
        assert_eq!(groups[1].id, "s4");
        assert_eq!(groups[1].children[0].id, "s4.cpu");
        assert_eq!(groups[1].children[0].label, Some("S4 CPU".to_string()));
    }
}
