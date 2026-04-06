/// Layered DAG layout for flow diagrams.
use std::collections::{HashMap, HashSet};

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

    // Phase 1: Longest-path layering via topological sort
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

    // Phase 2: Subtree-centering layout

    // 2a. Build spanning tree: each node's primary parent is its first incoming
    //     neighbor in declaration (adjacency) order.
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut has_parent: HashMap<String, bool> = HashMap::new();

    for (from, to, _) in &flow_info.adjacency {
        if has_parent.get(to).copied().unwrap_or(false) {
            continue; // already assigned a primary parent
        }
        has_parent.insert(to.clone(), true);
        children_map
            .entry(from.clone())
            .or_default()
            .push(to.clone());
    }

    // Line-aware child reordering: reorder children so that line edges
    // don't cross over sibling subtrees.
    if flow_info.line_aware && !ir.edges.is_empty() {
        fn collect_subtree(node: &str, children_map: &HashMap<String, Vec<String>>) -> HashSet<String> {
            let mut set = HashSet::new();
            let mut stack = vec![node.to_string()];
            while let Some(n) = stack.pop() {
                if set.insert(n.clone()) {
                    if let Some(kids) = children_map.get(&n) {
                        for k in kids {
                            stack.push(k.clone());
                        }
                    }
                }
            }
            set
        }

        let order_index: HashMap<&str, usize> = flow_info.node_order.iter()
            .enumerate().map(|(i, n)| (n.as_str(), i)).collect();

        let children_map_snapshot = children_map.clone();

        // Process nodes top-down (BFS from roots)
        let root_nodes: Vec<String> = flow_info
            .node_order
            .iter()
            .filter(|n| !has_parent.get(*n).copied().unwrap_or(false))
            .cloned()
            .collect();
        let mut queue: std::collections::VecDeque<String> = root_nodes.into_iter().collect();
        while let Some(node) = queue.pop_front() {
            if let Some(children) = children_map.get_mut(&node) {
                // Enqueue children for further processing
                for c in children.iter() {
                    queue.push_back(c.clone());
                }

                if children.len() < 2 { continue; }

                let node_subtree = collect_subtree(&node, &children_map_snapshot);
                let node_idx = order_index.get(node.as_str()).copied().unwrap_or(0);

                let mut to_front: Vec<usize> = Vec::new();
                let mut to_back: Vec<usize> = Vec::new();

                for (ci, child) in children.iter().enumerate() {
                    let child_subtree = collect_subtree(child, &children_map_snapshot);
                    for edge in &ir.edges {
                        let external = if child_subtree.contains(&edge.from) && !node_subtree.contains(&edge.to) {
                            Some(&edge.to)
                        } else if child_subtree.contains(&edge.to) && !node_subtree.contains(&edge.from) {
                            Some(&edge.from)
                        } else {
                            None
                        };
                        if let Some(ext) = external {
                            let ext_idx = order_index.get(ext.as_str()).copied().unwrap_or(0);
                            if ext_idx > node_idx {
                                to_back.push(ci);
                            } else {
                                to_front.push(ci);
                            }
                        }
                    }
                }

                if to_front.is_empty() && to_back.is_empty() { continue; }

                let mut front = Vec::new();
                let mut middle = Vec::new();
                let mut back = Vec::new();
                for (i, child) in children.iter().enumerate() {
                    if to_front.contains(&i) {
                        front.push(child.clone());
                    } else if to_back.contains(&i) {
                        back.push(child.clone());
                    } else {
                        middle.push(child.clone());
                    }
                }
                *children = [front, middle, back].concat();
            }
        }
    }

    let roots: Vec<String> = flow_info
        .node_order
        .iter()
        .filter(|n| !has_parent.get(*n).copied().unwrap_or(false))
        .cloned()
        .collect();

    // 2b. Compute subtree heights (cross-axis span)
    let mut subtree_heights: HashMap<String, f64> = HashMap::new();
    fn compute_subtree_height(
        node: &str,
        children_map: &HashMap<String, Vec<String>>,
        ir: &DiagramIR,
        direction: &Direction,
        cache: &mut HashMap<String, f64>,
    ) -> f64 {
        if let Some(&h) = cache.get(node) {
            return h;
        }
        let node_cross_size = match ir.nodes.get(node) {
            Some(n) => match direction {
                Direction::Right => n.height,
                Direction::Down => n.width,
            },
            None => 40.0,
        };
        let kids = children_map.get(node).cloned().unwrap_or_default();
        let h = if kids.is_empty() {
            node_cross_size
        } else {
            let sum: f64 = kids
                .iter()
                .map(|c| compute_subtree_height(c, children_map, ir, direction, cache))
                .sum();
            let gaps = V_GAP * (kids.len() as f64 - 1.0);
            let children_span = sum + gaps;
            // Subtree must be at least as tall as the node itself
            children_span.max(node_cross_size)
        };
        cache.insert(node.to_string(), h);
        h
    }

    for name in &flow_info.node_order {
        compute_subtree_height(
            name,
            &children_map,
            ir,
            &flow_info.direction,
            &mut subtree_heights,
        );
    }

    // 2c. Assign cross-axis positions recursively
    let mut cross_positions: HashMap<String, f64> = HashMap::new();
    fn assign_cross(
        node: &str,
        top: f64,
        children_map: &HashMap<String, Vec<String>>,
        subtree_heights: &HashMap<String, f64>,
        cross_positions: &mut HashMap<String, f64>,
    ) {
        let h = subtree_heights[node];
        // Center the node within its subtree band
        cross_positions.insert(node.to_string(), top + h / 2.0);

        if let Some(kids) = children_map.get(node) {
            // Center children block within the subtree band
            let children_total: f64 = kids
                .iter()
                .map(|c| subtree_heights[c.as_str()])
                .sum::<f64>()
                + V_GAP * (kids.len() as f64 - 1.0);
            let mut cursor = top + (h - children_total) / 2.0;
            for child in kids {
                let child_h = subtree_heights[child.as_str()];
                assign_cross(
                    child,
                    cursor,
                    children_map,
                    subtree_heights,
                    cross_positions,
                );
                cursor += child_h + V_GAP;
            }
        }
    }

    let mut cursor = 0.0_f64;
    for root in &roots {
        let h = subtree_heights[root.as_str()];
        assign_cross(
            root,
            cursor,
            &children_map,
            &subtree_heights,
            &mut cross_positions,
        );
        cursor += h + V_GAP;
    }

    // 2d. Compute main-axis layer positions
    let max_layer = layers.values().copied().max().unwrap_or(0);
    let mut layer_max_size = vec![0.0_f64; max_layer + 1];
    for (name, &idx) in &index_map {
        if let Some(&layer) = layers.get(&idx) {
            if let Some(node) = ir.nodes.get(name) {
                let main_size = match flow_info.direction {
                    Direction::Right => node.width,
                    Direction::Down => node.height,
                };
                layer_max_size[layer] = layer_max_size[layer].max(main_size);
            }
        }
    }

    let mut layer_starts = vec![0.0_f64; max_layer + 1];
    let mut acc = 0.0_f64;
    for i in 0..=max_layer {
        layer_starts[i] = acc;
        acc += layer_max_size[i] + H_GAP;
    }

    // Phase 3: Apply coordinates
    for (name, &idx) in &index_map {
        if let (Some(&layer), Some(&cross)) =
            (layers.get(&idx), cross_positions.get(name))
        {
            if let Some(node) = ir.nodes.get_mut(name) {
                match flow_info.direction {
                    Direction::Right => {
                        node.x = layer_starts[layer] + node.width / 2.0;
                        node.y = cross;
                    }
                    Direction::Down => {
                        node.x = cross;
                        node.y = layer_starts[layer] + node.height / 2.0;
                    }
                }
            }
        }
    }

    Ok(())
}
