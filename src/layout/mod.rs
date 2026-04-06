pub mod tree;
pub mod freeform;

use anyhow::Result;
use crate::ir::DiagramIR;

pub trait Layout {
    fn apply(&self, ir: &mut DiagramIR) -> Result<()>;
}
