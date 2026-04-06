/// IR data structures for diagram representation.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DiagramIR {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
    pub tree_roots: Vec<TreeInfo>,
    pub flow_graphs: Vec<FlowInfo>,
}

#[derive(Debug, Clone)]
pub struct TreeInfo {
    pub root: String,
    pub direction: crate::parse::Direction,
    pub parent_map: HashMap<String, String>, // child -> parent
    pub children_order: HashMap<String, Vec<String>>, // parent -> ordered children
}

#[derive(Debug, Clone)]
pub struct FlowInfo {
    pub direction: crate::parse::Direction,
    pub adjacency: Vec<(String, String, crate::parse::Arrow)>, // (from, to, arrow)
    pub node_order: Vec<String>, // declaration order for deterministic layout
    pub line_aware: bool, // reorder children to avoid line crossings, default true
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub style: NodeStyle,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub arrow: crate::parse::Arrow,
    pub label: Option<String>,
    pub style: EdgeStyle,
}

#[derive(Debug, Clone, Default)]
pub struct NodeStyle {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<f64>,
    pub font_size: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct EdgeStyle {
    pub color: Option<String>,
    pub stroke_width: Option<f64>,
    pub line_style: crate::parse::LineStyle,
}
