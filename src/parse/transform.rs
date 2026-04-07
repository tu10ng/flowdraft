/// Transform lexpr::Value into AST types.
use anyhow::{anyhow, bail, Result};
use lexpr::Value;

use super::ast::*;

/// Extract a name from a Symbol or String value.
fn value_to_name(value: &Value) -> Option<String> {
    value
        .as_symbol()
        .map(|s| s.to_string())
        .or_else(|| value.as_str().map(|s| s.to_string()))
}

pub fn parse_document(input: &str) -> Result<Document> {
    let options = lexpr::parse::Options::new()
        .with_keyword_syntax(lexpr::parse::KeywordSyntax::ColonPrefix);
    let mut forms = Vec::new();
    let mut parser = lexpr::parse::Parser::from_str_custom(input, options);
    while let Some(value) = parser.next_value().map_err(|e| anyhow!("{}", e))? {
        forms.push(parse_form(&value)?);
    }
    Ok(Document { forms })
}

fn parse_form(value: &Value) -> Result<Form> {
    let cons = value.as_cons().ok_or_else(|| anyhow!("expected a list form, got: {}", value))?;
    let command = cons
        .car()
        .as_symbol()
        .ok_or_else(|| anyhow!("expected command symbol, got: {}", cons.car()))?;

    match command {
        "tree" => parse_tree_form(value),
        "line" => parse_line_form(value),
        "style" => parse_style_form(value),
        "flow" => parse_flow_form(value),
        "define" => parse_define_form(value),
        other => bail!("unknown command: {}", other),
    }
}

fn collect_list(value: &Value) -> Vec<&Value> {
    let mut items = Vec::new();
    let mut current = value;
    while let Some(cons) = current.as_cons() {
        items.push(cons.car());
        current = cons.cdr();
    }
    items
}

fn parse_tree_form(value: &Value) -> Result<Form> {
    let items = collect_list(value);
    // items[0] = "tree", rest = options + tree body
    let mut direction = Direction::Down;
    let mut options = Vec::new();
    let mut body: Option<&Value> = None;

    let mut i = 1;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "down" => {
                    direction = Direction::Down;
                    i += 1;
                }
                "right" => {
                    direction = Direction::Right;
                    i += 1;
                }
                other => {
                    // keyword with optional value
                    let val = if i + 1 < items.len() && !items[i + 1].is_keyword() && items[i + 1].as_cons().is_none() {
                        i += 1;
                        Some(value_to_string(items[i]))
                    } else {
                        None
                    };
                    options.push((other.to_string(), val));
                    i += 1;
                }
            }
        } else if items[i].as_cons().is_some() || items[i].is_list() {
            body = Some(items[i]);
            i += 1;
        } else {
            i += 1;
        }
    }

    let root = match body {
        Some(b) => parse_tree_node(b)?,
        None => bail!("tree form missing body"),
    };

    Ok(Form::Tree {
        direction,
        options,
        root,
    })
}

fn parse_tree_node(value: &Value) -> Result<TreeNode> {
    match value {
        // Simple atom: just a name (symbol or string)
        v if v.is_symbol() || v.is_string() => {
            let name = value_to_name(v).unwrap();
            Ok(TreeNode {
                name,
                label: None,
                children: Vec::new(),
            })
        }
        // List: could be (name children...) or (name :label "text" children...)
        _ if value.as_cons().is_some() => {
            let items = collect_list(value);
            if items.is_empty() {
                bail!("empty tree node");
            }

            let name = value_to_name(items[0])
                .ok_or_else(|| anyhow!("expected node name symbol or string, got: {}", items[0]))?;

            let mut label = None;
            let mut children = Vec::new();
            let mut i = 1;

            // Check for :label option
            while i < items.len() {
                if let Some(kw) = items[i].as_keyword() {
                    if kw == "label" && i + 1 < items.len() {
                        label = Some(value_to_string(items[i + 1]));
                        i += 2;
                    } else {
                        i += 1;
                    }
                } else {
                    children.push(parse_tree_node(items[i])?);
                    i += 1;
                }
            }

            Ok(TreeNode {
                name,
                label,
                children,
            })
        }
        _ => bail!("unexpected tree node value: {}", value),
    }
}

fn parse_line_form(value: &Value) -> Result<Form> {
    let items = collect_list(value);
    // items[0] = "line", then options, then: from arrow to, then more options
    let mut line_style = LineStyle::default();
    let mut options = Vec::new();
    let mut from = None;
    let mut arrow = Arrow::Forward;
    let mut to = None;

    let mut i = 1;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "straight" => {
                    line_style = LineStyle::Straight;
                    i += 1;
                }
                "curved" => {
                    line_style = LineStyle::Curved;
                    i += 1;
                }
                other => {
                    let val = if i + 1 < items.len()
                        && !items[i + 1].is_keyword()
                        && !is_arrow_symbol(items[i + 1])
                    {
                        i += 1;
                        Some(value_to_string(items[i]))
                    } else {
                        None
                    };
                    options.push((other.to_string(), val));
                    i += 1;
                }
            }
        } else if let Some(sym) = items[i].as_symbol() {
            if is_arrow(sym) {
                arrow = parse_arrow(sym);
                i += 1;
            } else if from.is_none() {
                from = Some(sym.to_string());
                i += 1;
            } else if to.is_none() {
                to = Some(sym.to_string());
                i += 1;
            } else {
                i += 1;
            }
        } else if let Some(name) = value_to_name(items[i]) {
            // String node name
            if from.is_none() {
                from = Some(name);
                i += 1;
            } else if to.is_none() {
                to = Some(name);
                i += 1;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    Ok(Form::Line {
        line_style,
        from: from.ok_or_else(|| anyhow!("line missing 'from' node"))?,
        arrow,
        to: to.ok_or_else(|| anyhow!("line missing 'to' node"))?,
        options,
    })
}

fn parse_style_form(value: &Value) -> Result<Form> {
    let items = collect_list(value);
    // items[0] = "style", items[1] = target, rest = :key value pairs
    if items.len() < 2 {
        bail!("style form needs a target");
    }
    let target = value_to_name(items[1])
        .ok_or_else(|| anyhow!("style target must be a symbol or string"))?;

    let mut props = Vec::new();
    let mut i = 2;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            if i + 1 < items.len() {
                props.push((kw.to_string(), value_to_string(items[i + 1])));
                i += 2;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    Ok(Form::Style { target, props })
}

fn parse_define_form(value: &Value) -> Result<Form> {
    let items = collect_list(value);
    // (define server (params name) (cpu :label "${name} CPU") ...)
    if items.len() < 3 {
        bail!("define form needs at least a name and body");
    }

    let name = items[1]
        .as_symbol()
        .ok_or_else(|| anyhow!("define: expected template name symbol, got: {}", items[1]))?
        .to_string();

    // Parse (params ...) list
    let mut params = Vec::new();
    let mut body_start = 2;

    if let Some(cons) = items[2].as_cons() {
        if cons.car().as_symbol() == Some("params") {
            let param_items = collect_list(items[2]);
            for p in &param_items[1..] {
                params.push(
                    p.as_symbol()
                        .ok_or_else(|| anyhow!("define: expected param name symbol, got: {}", p))?
                        .to_string(),
                );
            }
            body_start = 3;
        }
    }

    let mut body = Vec::new();
    for item in &items[body_start..] {
        body.push(parse_tree_node(item)?);
    }

    if body.is_empty() {
        bail!("define form has empty body");
    }

    Ok(Form::Define(DefineTemplate { name, params, body }))
}

fn parse_flow_form(value: &Value) -> Result<Form> {
    let items = collect_list(value);
    // items[0] = "flow", then direction keyword, then chain sub-lists
    let mut direction = Direction::Right;
    let mut options = Vec::new();
    let mut chains = Vec::new();

    let mut i = 1;
    while i < items.len() {
        if let Some(kw) = items[i].as_keyword() {
            match kw {
                "down" => direction = Direction::Down,
                "right" => direction = Direction::Right,
                "no-line-aware" => options.push(("no-line-aware".to_string(), None)),
                _ => {}
            }
            i += 1;
        } else if items[i].as_cons().is_some() {
            chains.push(parse_flow_chain(items[i])?);
            i += 1;
        } else {
            i += 1;
        }
    }

    if chains.is_empty() {
        bail!("flow form has no chains");
    }

    Ok(Form::Flow { direction, options, chains })
}

fn parse_flow_chain(value: &Value) -> Result<FlowChain> {
    let items = collect_list(value);
    let mut segments = Vec::new();

    let mut i = 0;
    while i < items.len() {
        if is_arrow_symbol(items[i]) {
            // skip stray arrows
            i += 1;
            continue;
        }

        // Check if this item is a nested list (inline template instantiation)
        let (name, inline_node) = if items[i].as_cons().is_some() && items[i].as_cons().map(|c| c.car().as_symbol().is_some() && !is_arrow(c.car().as_symbol().unwrap_or(""))).unwrap_or(false) {
            let tree_node = parse_tree_node(items[i])?;
            // The instance_id is the first child's name (e.g., (server s3 "S3") -> s3)
            if tree_node.children.is_empty() {
                bail!("inline template instantiation in flow requires an instance id: {}", items[i]);
            }
            let instance_id = tree_node.children[0].name.clone();
            (instance_id, Some(tree_node))
        } else {
            let name = value_to_name(items[i])
                .ok_or_else(|| anyhow!("expected node name in flow chain, got: {}", items[i]))?;
            (name, None)
        };

        let arrow = if i + 1 < items.len() && is_arrow_symbol(items[i + 1]) {
            let a = parse_arrow(items[i + 1].as_symbol().unwrap());
            i += 2; // skip node + arrow
            Some(a)
        } else {
            i += 1; // last node in chain
            None
        };

        segments.push(FlowSegment { node: name, arrow, inline_node });
    }

    if segments.is_empty() {
        bail!("empty flow chain");
    }

    Ok(FlowChain { segments })
}

fn is_arrow(s: &str) -> bool {
    matches!(s, "->" | "<-" | "<->" | "--")
}

fn is_arrow_symbol(value: &Value) -> bool {
    value.as_symbol().map_or(false, is_arrow)
}

fn parse_arrow(s: &str) -> Arrow {
    match s {
        "->" => Arrow::Forward,
        "<-" => Arrow::Backward,
        "<->" => Arrow::Both,
        "--" => Arrow::None,
        _ => Arrow::Forward,
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.to_string(),
        Value::Symbol(s) => s.to_string(),
        Value::Number(n) => n.to_string(),
        _ => value.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_tree() {
        let input = "(tree :down (a (b (c d e f)) g))";
        let doc = parse_document(input).unwrap();
        assert_eq!(doc.forms.len(), 1);
        match &doc.forms[0] {
            Form::Tree {
                direction, root, ..
            } => {
                assert_eq!(*direction, Direction::Down);
                assert_eq!(root.name, "a");
                assert_eq!(root.children.len(), 2);
                assert_eq!(root.children[0].name, "b");
                assert_eq!(root.children[1].name, "g");
                // b has child c with 3 leaf children
                let b = &root.children[0];
                assert_eq!(b.children.len(), 1);
                let c = &b.children[0];
                assert_eq!(c.name, "c");
                assert_eq!(c.children.len(), 3);
                assert_eq!(c.children[0].name, "d");
                assert_eq!(c.children[1].name, "e");
                assert_eq!(c.children[2].name, "f");
            }
            _ => panic!("expected tree form"),
        }
    }

    #[test]
    fn test_parse_tree_with_label() {
        let input = r#"(tree :down (ceo :label "CEO" (dev :label "研发部") (pm :label "产品部")))"#;
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Tree { root, .. } => {
                assert_eq!(root.name, "ceo");
                assert_eq!(root.label.as_deref(), Some("CEO"));
                assert_eq!(root.children.len(), 2);
                assert_eq!(root.children[0].name, "dev");
                assert_eq!(root.children[0].label.as_deref(), Some("研发部"));
                assert_eq!(root.children[1].name, "pm");
                assert_eq!(root.children[1].label.as_deref(), Some("产品部"));
            }
            _ => panic!("expected tree form"),
        }
    }

    #[test]
    fn test_parse_line() {
        let input = r##"(line :straight :color "#ff0000" d -> e :desc "有关系")"##;
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Line {
                line_style,
                from,
                arrow,
                to,
                options,
            } => {
                assert_eq!(*line_style, LineStyle::Straight);
                assert_eq!(from, "d");
                assert_eq!(*arrow, Arrow::Forward);
                assert_eq!(to, "e");
                assert!(options.iter().any(|(k, v)| k == "color" && v.as_deref() == Some("#ff0000")));
                assert!(options.iter().any(|(k, v)| k == "desc" && v.as_deref() == Some("有关系")));
            }
            _ => panic!("expected line form"),
        }
    }

    #[test]
    fn test_parse_style() {
        let input = r##"(style leaf1 :fill "#eee" :stroke "#333")"##;
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Style { target, props } => {
                assert_eq!(target, "leaf1");
                assert_eq!(props.len(), 2);
                assert_eq!(props[0], ("fill".to_string(), "#eee".to_string()));
                assert_eq!(props[1], ("stroke".to_string(), "#333".to_string()));
            }
            _ => panic!("expected style form"),
        }
    }

    #[test]
    fn test_parse_multiple_forms() {
        let input = r#"(tree :down (a (b c))) (line :straight b -> c :desc "test")"#;
        let doc = parse_document(input).unwrap();
        assert_eq!(doc.forms.len(), 2);
    }

    #[test]
    fn test_parse_tree_string_node_names() {
        let input = r#"(tree :down ("部门" ("研发" "测试A") "测试B"))"#;
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Tree { root, .. } => {
                assert_eq!(root.name, "部门");
                assert_eq!(root.children.len(), 2);
                assert_eq!(root.children[0].name, "研发");
                assert_eq!(root.children[0].children[0].name, "测试A");
                assert_eq!(root.children[1].name, "测试B");
            }
            _ => panic!("expected tree form"),
        }
    }

    #[test]
    fn test_parse_line_string_node_names() {
        let input = r#"(line :straight "部门" -> "研发")"#;
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Line { from, to, .. } => {
                assert_eq!(from, "部门");
                assert_eq!(to, "研发");
            }
            _ => panic!("expected line form"),
        }
    }

    #[test]
    fn test_parse_style_string_target() {
        let input = r##"(style "部门" :fill "#eee")"##;
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Style { target, props } => {
                assert_eq!(target, "部门");
                assert_eq!(props[0], ("fill".to_string(), "#eee".to_string()));
            }
            _ => panic!("expected style form"),
        }
    }

    #[test]
    fn test_parse_flow_single_chain() {
        let input = "(flow :right (a -> b))";
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Flow { direction, chains, .. } => {
                assert_eq!(*direction, Direction::Right);
                assert_eq!(chains.len(), 1);
                assert_eq!(chains[0].segments.len(), 2);
                assert_eq!(chains[0].segments[0].node, "a");
                assert_eq!(chains[0].segments[0].arrow, Some(Arrow::Forward));
                assert_eq!(chains[0].segments[1].node, "b");
                assert_eq!(chains[0].segments[1].arrow, None);
            }
            _ => panic!("expected flow form"),
        }
    }

    #[test]
    fn test_parse_flow_multi_hop_chain() {
        let input = "(flow :down (a -> b -> c -> d))";
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Flow { direction, chains, .. } => {
                assert_eq!(*direction, Direction::Down);
                assert_eq!(chains.len(), 1);
                assert_eq!(chains[0].segments.len(), 4);
                assert_eq!(chains[0].segments[0].node, "a");
                assert_eq!(chains[0].segments[1].node, "b");
                assert_eq!(chains[0].segments[2].node, "c");
                assert_eq!(chains[0].segments[3].node, "d");
                assert_eq!(chains[0].segments[3].arrow, None);
            }
            _ => panic!("expected flow form"),
        }
    }

    #[test]
    fn test_parse_flow_multiple_chains() {
        let input = "(flow :right (a -> b) (b -> c -> d) (b -> e) (a -> f))";
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Flow { direction, chains, .. } => {
                assert_eq!(*direction, Direction::Right);
                assert_eq!(chains.len(), 4);
            }
            _ => panic!("expected flow form"),
        }
    }

    #[test]
    fn test_parse_flow_inline_template() {
        let input = r#"(flow :right (rack -> (server s3 "S3") -> (server s4 "S4")))"#;
        let doc = parse_document(input).unwrap();
        match &doc.forms[0] {
            Form::Flow { chains, .. } => {
                assert_eq!(chains.len(), 1);
                let segs = &chains[0].segments;
                assert_eq!(segs.len(), 3);
                assert_eq!(segs[0].node, "rack");
                assert!(segs[0].inline_node.is_none());
                assert_eq!(segs[1].node, "s3");
                assert!(segs[1].inline_node.is_some());
                let inline = segs[1].inline_node.as_ref().unwrap();
                assert_eq!(inline.name, "server");
                assert_eq!(segs[2].node, "s4");
                assert!(segs[2].inline_node.is_some());
            }
            _ => panic!("expected flow form"),
        }
    }
}