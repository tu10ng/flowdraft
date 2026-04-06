pub mod parse;
pub mod ir;
pub mod layout;
pub mod render;
pub mod style;

use anyhow::Result;

pub fn process(input: &str) -> Result<String> {
    let doc = parse::parse_document(input)?;
    let mut ir = ir::build_ir(&doc)?;
    let tree_layout = layout::tree::TreeLayout;
    layout::Layout::apply(&tree_layout, &mut ir)?;
    let flow_layout = layout::flow::FlowLayout;
    layout::Layout::apply(&flow_layout, &mut ir)?;
    Ok(render::render_svg(&ir))
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
}
