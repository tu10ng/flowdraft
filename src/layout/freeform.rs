/// Freeform layout — passthrough, no automatic positioning.
use anyhow::Result;
use crate::ir::DiagramIR;
use super::Layout;

pub struct FreeformLayout;

impl Layout for FreeformLayout {
    fn apply(&self, _ir: &mut DiagramIR) -> Result<()> {
        Ok(())
    }
}
