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
    Ok(render::render_svg(&ir))
}
