pub mod svg;

use crate::ir::DiagramIR;
use crate::style::Theme;

pub use svg::{render_svg, SvgRenderer};

pub trait Renderer {
    type Output;
    fn render(&self, ir: &DiagramIR, theme: &Theme) -> Self::Output;
}
