/// Layout for nodes inside a group container.
use anyhow::Result;

use crate::ir::{DiagramIR, Group};
use crate::style::Theme;

pub trait GroupLayout {
    fn apply(&self, ir: &mut DiagramIR, group: &Group, theme: &Theme) -> Result<()>;
}

/// Horizontal row layout (default).
pub struct HorizontalGroupLayout;

impl GroupLayout for HorizontalGroupLayout {
    fn apply(&self, ir: &mut DiagramIR, group: &Group, theme: &Theme) -> Result<()> {
        let anchor = match ir.nodes.get(&group.id).cloned() {
            Some(n) => n,
            None => return Ok(()),
        };
        let child_widths: Vec<f64> = group.children.iter()
            .filter_map(|id| ir.nodes.get(id))
            .map(|n| n.width)
            .collect();
        let total_width: f64 = child_widths.iter().sum::<f64>()
            + child_widths.len().saturating_sub(1) as f64 * theme.h_gap;
        let mut x = anchor.x - total_width / 2.0;
        for child_id in &group.children {
            if let Some(node) = ir.nodes.get_mut(child_id) {
                node.x = x + node.width / 2.0;
                node.y = anchor.y;
                x += node.width + theme.h_gap;
            }
        }
        Ok(())
    }
}

/// Vertical column layout.
pub struct VerticalGroupLayout;

impl GroupLayout for VerticalGroupLayout {
    fn apply(&self, ir: &mut DiagramIR, group: &Group, theme: &Theme) -> Result<()> {
        let anchor = match ir.nodes.get(&group.id).cloned() {
            Some(n) => n,
            None => return Ok(()),
        };
        let child_heights: Vec<f64> = group.children.iter()
            .filter_map(|id| ir.nodes.get(id))
            .map(|n| n.height)
            .collect();
        let total_height: f64 = child_heights.iter().sum::<f64>()
            + child_heights.len().saturating_sub(1) as f64 * theme.v_gap;
        let mut y = anchor.y - total_height / 2.0;
        for child_id in &group.children {
            if let Some(node) = ir.nodes.get_mut(child_id) {
                node.x = anchor.x;
                node.y = y + node.height / 2.0;
                y += node.height + theme.v_gap;
            }
        }
        Ok(())
    }
}
