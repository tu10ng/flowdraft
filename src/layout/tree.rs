/// Tree layout using the Reingold-Tilford algorithm.
use std::collections::HashMap;

use anyhow::Result;
use reingold_tilford::{self, Coordinate, Dimensions, NodeInfo, SmallVec};

use crate::ir::{DiagramIR, TreeInfo};
use crate::parse::Direction;
use super::Layout;

/// Temporary tree structure for the RT algorithm.
struct RTNode {
    id: String,
    children: Vec<RTNode>,
    half_width: f64,
    half_height: f64,
}

struct RTTree {
    h_gap: f64,
    v_gap: f64,
}

impl<'a> NodeInfo<&'a RTNode> for RTTree {
    type Key = String;

    fn key(&self, node: &'a RTNode) -> String {
        node.id.clone()
    }

    fn children(&self, node: &'a RTNode) -> SmallVec<&'a RTNode> {
        node.children.iter().collect()
    }

    fn dimensions(&self, node: &'a RTNode) -> Dimensions {
        Dimensions {
            top: node.half_height,
            bottom: node.half_height,
            left: node.half_width,
            right: node.half_width,
        }
    }

    fn border(&self, _node: &'a RTNode) -> Dimensions {
        Dimensions {
            top: self.v_gap / 2.0,
            bottom: self.v_gap / 2.0,
            left: self.h_gap / 2.0,
            right: self.h_gap / 2.0,
        }
    }
}

pub struct TreeLayout {
    pub h_gap: f64,
    pub v_gap: f64,
}

impl Layout for TreeLayout {
    fn apply(&self, ir: &mut DiagramIR) -> Result<()> {
        let rt_tree = RTTree { h_gap: self.h_gap, v_gap: self.v_gap };
        for tree_info in &ir.tree_roots.clone() {
            let rt_root = build_rt_tree(&tree_info.root, tree_info, &ir.nodes)?;
            let coords = reingold_tilford::layout(&rt_tree, &rt_root);
            apply_coords(ir, &coords, tree_info.direction);
        }
        Ok(())
    }
}

fn build_rt_tree(
    node_id: &str,
    tree_info: &TreeInfo,
    nodes: &HashMap<String, crate::ir::Node>,
) -> Result<RTNode> {
    let node = nodes
        .get(node_id)
        .ok_or_else(|| anyhow::anyhow!("node not found: {}", node_id))?;

    let child_ids = tree_info
        .children_order
        .get(node_id)
        .cloned()
        .unwrap_or_default();

    let mut children = Vec::new();
    for child_id in &child_ids {
        children.push(build_rt_tree(child_id, tree_info, nodes)?);
    }

    Ok(RTNode {
        id: node.id.clone(),
        children,
        half_width: node.width / 2.0,
        half_height: node.height / 2.0,
    })
}

fn apply_coords(
    ir: &mut DiagramIR,
    coords: &HashMap<String, Coordinate>,
    direction: Direction,
) {
    for (id, coord) in coords {
        if let Some(node) = ir.nodes.get_mut(id) {
            match direction {
                Direction::Down => {
                    node.x = coord.x;
                    node.y = coord.y;
                }
                Direction::Right => {
                    // Swap x and y for horizontal layout
                    node.x = coord.y;
                    node.y = coord.x;
                }
            }
        }
    }
}
