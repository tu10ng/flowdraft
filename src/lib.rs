pub mod parse;
pub mod ir;
pub mod layout;
pub mod render;
pub mod style;

#[cfg(feature = "wasm")]
pub mod wasm;

use anyhow::Result;
use style::Theme;

#[derive(Debug, Clone)]
pub struct ProcessOptions {
    pub no_line_aware: bool,
    pub theme: Theme,
}

impl Default for ProcessOptions {
    fn default() -> Self {
        Self {
            no_line_aware: false,
            theme: Theme::default(),
        }
    }
}

pub fn process(input: &str) -> Result<String> {
    process_with_options(input, &ProcessOptions::default())
}

pub fn process_with_options(input: &str, opts: &ProcessOptions) -> Result<String> {
    use layout::group::{GroupLayout, HorizontalGroupLayout, VerticalGroupLayout};
    use parse::Direction;

    let doc = parse::parse_document(input)?;
    let (doc, groups) = parse::expand::expand_defines(doc)?;
    let mut ir = ir::build_ir(&doc, &opts.theme, groups)?;
    if opts.no_line_aware {
        for fg in &mut ir.flow_graphs {
            fg.line_aware = false;
        }
    }

    // Pre-calculate group sizes so instance nodes have correct dimensions for tree layout
    let group_padding = style::defaults::GROUP_PADDING;
    let group_title_h = style::defaults::GROUP_TITLE_HEIGHT;
    for group in &ir.groups {
        let children_count = group.children.len();
        if children_count > 0 {
            let child_widths: f64 = group.children.iter()
                .filter_map(|id| ir.nodes.get(id))
                .map(|n| n.width)
                .sum();
            let child_max_height: f64 = group.children.iter()
                .filter_map(|id| ir.nodes.get(id))
                .map(|n| n.height)
                .fold(0.0_f64, |a, b| a.max(b));
            let total_width = child_widths
                + (children_count - 1) as f64 * opts.theme.h_gap
                + group_padding * 2.0;
            let total_height = child_max_height + group_padding * 2.0 + group_title_h;

            if let Some(instance_node) = ir.nodes.get_mut(&group.id) {
                instance_node.width = total_width;
                instance_node.height = total_height;
            }
        }
    }

    let tree_layout = layout::tree::TreeLayout { h_gap: opts.theme.h_gap, v_gap: opts.theme.v_gap };
    layout::Layout::apply(&tree_layout, &mut ir)?;
    let flow_layout = layout::flow::FlowLayout { h_gap: opts.theme.h_gap, v_gap: opts.theme.v_gap };
    layout::Layout::apply(&flow_layout, &mut ir)?;

    // Position group children around the instance node anchor
    for group in &ir.groups.clone() {
        match group.direction {
            Some(Direction::Down) => VerticalGroupLayout.apply(&mut ir, group, &opts.theme)?,
            _ => HorizontalGroupLayout.apply(&mut ir, group, &opts.theme)?,
        }
    }

    // Calculate group bounding boxes from child node positions
    for group in &mut ir.groups {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for child_id in &group.children {
            if let Some(node) = ir.nodes.get(child_id) {
                min_x = min_x.min(node.x - node.width / 2.0);
                min_y = min_y.min(node.y - node.height / 2.0);
                max_x = max_x.max(node.x + node.width / 2.0);
                max_y = max_y.max(node.y + node.height / 2.0);
            }
        }
        if min_x != f64::INFINITY {
            group.x = (min_x + max_x) / 2.0;
            group.y = (min_y - group_title_h + max_y) / 2.0;
            group.width = (max_x - min_x) + group_padding * 2.0;
            group.height = (max_y - min_y) + group_padding * 2.0 + group_title_h;

            // 同步 instance 节点，使树边终点与 group 容器对齐
            if let Some(node) = ir.nodes.get_mut(&group.id) {
                node.x = group.x;
                node.y = group.y;
                node.width = group.width;
                node.height = group.height;
            }
        }
    }

    Ok(render::render_svg(&ir, &opts.theme))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_end_to_end() {
        let input = r#"(flow :right (a -> b) (b -> c -> d) (b -> e) (a -> f)) (line d -> f :desc "心跳同步")"#;
        let svg = process(input).unwrap();
        // All 6 nodes should appear in the SVG
        for name in &["a", "b", "c", "d", "e", "f"] {
            assert!(svg.contains(name), "SVG missing node: {}", name);
        }
        // The line label should appear
        assert!(svg.contains("心跳同步"), "SVG missing edge label");
    }

    #[test]
    fn test_define_instantiate_end_to_end() {
        let input = r##"
            (define server (params name)
                (cpu :label "${name} CPU")
                (eth0)
                (eth1))
            (tree :down
                (rack :label "机架"
                    (server s1 "S1")
                    (server s2 "S2")))
            (line :straight s1.eth0 -> s2.eth0 :desc "网络")
            (style s1.cpu :fill "#e8f4fd")
        "##;
        let svg = process(input).unwrap();
        // Expanded nodes should appear
        assert!(svg.contains("S1 CPU"), "SVG missing S1 CPU label");
        assert!(svg.contains("S2 CPU"), "SVG missing S2 CPU label");
        // Nodes without :label use base symbol name as default label
        assert!(svg.contains("eth0"), "SVG missing eth0 default label");
        assert!(svg.contains("eth1"), "SVG missing eth1 default label");
        // Edge label
        assert!(svg.contains("网络"), "SVG missing edge label");
        // Group containers should be rendered (check for group fill color)
        assert!(svg.contains("#f8f9fa"), "SVG missing group background");
    }

    #[test]
    fn test_line_default_curved_with_obstacle() {
        // Three nodes in a row; line from a to c should route around b
        let input = r#"(flow :right (a -> b -> c)) (line a -> c :desc "跳过")"#;
        let svg = process(input).unwrap();
        // Default line style is Curved, so SVG should contain a <path> (not <line>)
        assert!(svg.contains("跳过"), "SVG missing edge label");
        assert!(svg.contains("<path"), "SVG missing path element for curved edge");
    }

    #[test]
    fn test_line_straight_forces_line() {
        let input = r#"(flow :right (a -> b -> c)) (line :straight a -> c :desc "直线")"#;
        let svg = process(input).unwrap();
        assert!(svg.contains("直线"), "SVG missing edge label");
        assert!(svg.contains("<path"), "SVG missing path element for straight edge");
    }
}
