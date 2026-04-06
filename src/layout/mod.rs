pub mod tree;
pub mod freeform;
pub mod flow;
pub mod group;

use anyhow::Result;
use crate::ir::DiagramIR;

pub trait Layout {
    fn apply(&self, ir: &mut DiagramIR) -> Result<()>;
}
