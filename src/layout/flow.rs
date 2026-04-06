/// Layered DAG layout for flow diagrams.
use std::collections::HashMap;

use anyhow::Result;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Topo;

use crate::ir::DiagramIR;
use crate::parse::Direction;
use super::Layout;

const H_GAP: f64 = 20.0;
const V_GAP: f64 = 30.0;

pub struct FlowLayout;

impl Layout for FlowLayout {
    fn apply(&self, ir: &mut DiagramIR) -> Result<()> {
        for flow_info in &ir.flow_graphs.clone() {
            layout_flow(ir, flow_info)?;
        }
        Ok(())
    }
}

fn layout_flow(ir: &mut DiagramIR, flow_info: &crate::ir::FlowInfo) -> Result<()> {
    let mut graph = DiGraph::<String, ()>::new();
    let mut index_map: HashMap<String, NodeIndex> = HashMap::new();

    // Add nodes in declaration order
    for name in &flow_info.node_order {
        let idx = graph.add_node(name.clone());
        index_map.insert(name.clone(), idx);
    }

    // Add edges
    for (from, to, _arrow) in &flow_info.adjacency {
        if let (Some(&fi), Some(&ti)) = (index_map.get(from), index_map.get(to)) {
            graph.add_edge(fi, ti, ());
        }
    }

    // Longest-path layering via topological sort
    let mut layers: HashMap<NodeIndex, usize> = HashMap::new();
    let mut topo = Topo::new(&graph);
    while let Some(nx) = topo.next(&graph) {
        let max_pred = graph
            .neighbors_directed(nx, petgraph::Direction::Incoming)
            .filter_map(|pred| layers.get(&pred))
            .max()
            .copied()
            .unwrap_or(0);
        let layer = if graph
            .neighbors_directed(nx, petgraph::Direction::Incoming)
            .next()
            .is_some()
        {
            max_pred + 1
        } else {
            0
        };
        layers.insert(nx, layer);
    }

    // Group nodes by layer, preserving declaration order within each layer
    let max_layer = layers.values().copied().max().unwrap_or(0);
    let mut layer_groups: Vec<Vec<NodeIndex>> = vec![Vec::new(); max_layer + 1];
    // Use node_order to ensure deterministic ordering within layers
    for name in &flow_info.node_order {
        if let Some(&idx) = index_map.get(name) {
            if let Some(&layer) = layers.get(&idx) {
                layer_groups[layer].push(idx);
            }
        }
    }

    // Position nodes
    for (layer_idx, group) in layer_groups.iter().enumerate() {
        for (pos, &nx) in group.iter().enumerate() {
            let node_name = &graph[nx];
            if let Some(node) = ir.nodes.get_mut(node_name) {
                match flow_info.direction {
                    Direction::Right => {
                        // layer -> x, position -> y
                        node.x = layer_idx as f64 * (node.width + H_GAP);
                        node.y = pos as f64 * (node.height + V_GAP);
                    }
                    Direction::Down => {
                        // layer -> y, position -> x
                        node.x = pos as f64 * (node.width + H_GAP);
                        node.y = layer_idx as f64 * (node.height + V_GAP);
                    }
                }
            }
        }
    }

    Ok(())
}
