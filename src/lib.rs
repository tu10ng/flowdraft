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
    let doc = parse::parse_document(input)?;
    let mut ir = ir::build_ir(&doc, &opts.theme)?;
    if opts.no_line_aware {
        for fg in &mut ir.flow_graphs {
            fg.line_aware = false;
        }
    }
    let tree_layout = layout::tree::TreeLayout { h_gap: opts.theme.h_gap, v_gap: opts.theme.v_gap };
    layout::Layout::apply(&tree_layout, &mut ir)?;
    let flow_layout = layout::flow::FlowLayout { h_gap: opts.theme.h_gap, v_gap: opts.theme.v_gap };
    layout::Layout::apply(&flow_layout, &mut ir)?;
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
}
