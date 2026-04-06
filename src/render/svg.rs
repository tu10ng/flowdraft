/// SVG rendering from DiagramIR.
use svg::node::element::{self, path::Data, Definitions, Marker, Rectangle, Text};
use svg::Document;

use crate::ir::{DiagramIR, Edge, Node};
use crate::parse::Arrow;
use crate::style::defaults::*;

pub fn render_svg(ir: &DiagramIR) -> String {
    // Calculate bounding box
    let (min_x, min_y, max_x, max_y) = bounding_box(ir);
    let padding = SVG_PADDING;
    let width = max_x - min_x + padding * 2.0;
    let height = max_y - min_y + padding * 2.0;
    let offset_x = -min_x + padding;
    let offset_y = -min_y + padding;

    let mut doc = Document::new()
        .set("width", width)
        .set("height", height)
        .set("viewBox", (0, 0, width as i64, height as i64))
        .set("xmlns", "http://www.w3.org/2000/svg");

    // Add arrow marker definitions
    let defs = Definitions::new()
        .add(arrow_marker("arrow-forward", false))
        .add(arrow_marker("arrow-backward", true));
    doc = doc.add(defs);

    // Add a background
    doc = doc.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", BG_COLOR),
    );

    // Render tree edges (parent-child connections)
    for tree_info in &ir.tree_roots {
        for (child_id, parent_id) in &tree_info.parent_map {
            if let (Some(parent), Some(child)) =
                (ir.nodes.get(parent_id), ir.nodes.get(child_id))
            {
                let edge_svg = render_tree_edge(parent, child, offset_x, offset_y);
                doc = doc.add(edge_svg);
            }
        }
    }

    // Render explicit edges
    for edge in &ir.edges {
        if let (Some(from_node), Some(to_node)) =
            (ir.nodes.get(&edge.from), ir.nodes.get(&edge.to))
        {
            let edge_svg = render_edge(from_node, to_node, edge, offset_x, offset_y);
            doc = doc.add(edge_svg);
            if let Some(ref label) = edge.label {
                let label_svg = render_edge_label(from_node, to_node, label, edge, offset_x, offset_y);
                doc = doc.add(label_svg);
            }
        }
    }

    // Render nodes
    for node in ir.nodes.values() {
        let (rect, text) = render_node(node, offset_x, offset_y);
        doc = doc.add(rect);
        doc = doc.add(text);
    }

    doc.to_string()
}

fn bounding_box(ir: &DiagramIR) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for node in ir.nodes.values() {
        let left = node.x - node.width / 2.0;
        let right = node.x + node.width / 2.0;
        let top = node.y - node.height / 2.0;
        let bottom = node.y + node.height / 2.0;
        min_x = min_x.min(left);
        min_y = min_y.min(top);
        max_x = max_x.max(right);
        max_y = max_y.max(bottom);
    }

    if min_x == f64::INFINITY {
        (0.0, 0.0, 100.0, 100.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

fn render_node(
    node: &Node,
    offset_x: f64,
    offset_y: f64,
) -> (element::Rectangle, element::Group) {
    let x = node.x + offset_x - node.width / 2.0;
    let y = node.y + offset_y - node.height / 2.0;

    let fill = node
        .style
        .fill
        .as_deref()
        .unwrap_or(NODE_FILL);
    let stroke = node
        .style
        .stroke
        .as_deref()
        .unwrap_or(NODE_STROKE);

    let rect = element::Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", node.width)
        .set("height", node.height)
        .set("rx", NODE_CORNER_RADIUS)
        .set("ry", NODE_CORNER_RADIUS)
        .set("fill", fill)
        .set("stroke", stroke)
        .set("stroke-width", NODE_STROKE_WIDTH);

    let text = Text::new(&node.label)
        .set("x", node.x + offset_x)
        .set("y", node.y + offset_y + FONT_SIZE / 3.0) // approximate vertical centering
        .set("text-anchor", "middle")
        .set("font-family", FONT_FAMILY)
        .set("font-size", FONT_SIZE)
        .set("fill", TEXT_COLOR);

    let group = element::Group::new().add(text);

    (rect, group)
}

fn render_tree_edge(
    parent: &Node,
    child: &Node,
    offset_x: f64,
    offset_y: f64,
) -> element::Path {
    let px = parent.x + offset_x;
    let py = parent.y + offset_y + parent.height / 2.0;
    let cx = child.x + offset_x;
    let cy = child.y + offset_y - child.height / 2.0;
    let mid_y = (py + cy) / 2.0;

    let data = Data::new()
        .move_to((px, py))
        .line_to((px, mid_y))
        .line_to((cx, mid_y))
        .line_to((cx, cy));

    element::Path::new()
        .set("d", data)
        .set("fill", "none")
        .set("stroke", EDGE_COLOR)
        .set("stroke-width", EDGE_STROKE_WIDTH)
}

fn render_edge(
    from: &Node,
    to: &Node,
    edge: &Edge,
    offset_x: f64,
    offset_y: f64,
) -> element::Line {
    let color = edge
        .style
        .color
        .as_deref()
        .unwrap_or(EDGE_COLOR);

    // Connect from edge of nodes
    let (fx, fy) = edge_point(from, to, offset_x, offset_y);
    let (tx, ty) = edge_point(to, from, offset_x, offset_y);

    let mut line = element::Line::new()
        .set("x1", fx)
        .set("y1", fy)
        .set("x2", tx)
        .set("y2", ty)
        .set("stroke", color)
        .set("stroke-width", EDGE_STROKE_WIDTH);

    match edge.arrow {
        Arrow::Forward => {
            line = line.set("marker-end", "url(#arrow-forward)");
        }
        Arrow::Backward => {
            line = line.set("marker-start", "url(#arrow-backward)");
        }
        Arrow::Both => {
            line = line.set("marker-start", "url(#arrow-backward)");
            line = line.set("marker-end", "url(#arrow-forward)");
        }
        Arrow::None => {}
    }

    line
}

fn render_edge_label(
    from: &Node,
    to: &Node,
    label: &str,
    edge: &Edge,
    offset_x: f64,
    offset_y: f64,
) -> element::Text {
    let color = edge
        .style
        .color
        .as_deref()
        .unwrap_or(TEXT_COLOR);

    let mx = (from.x + to.x) / 2.0 + offset_x;
    let my = (from.y + to.y) / 2.0 + offset_y - 6.0;

    Text::new(label)
        .set("x", mx)
        .set("y", my)
        .set("text-anchor", "middle")
        .set("font-family", FONT_FAMILY)
        .set("font-size", FONT_SIZE * 0.85)
        .set("fill", color)
}

/// Calculate the point on the edge of `node` closest to `target`.
fn edge_point(node: &Node, target: &Node, offset_x: f64, offset_y: f64) -> (f64, f64) {
    let nx = node.x + offset_x;
    let ny = node.y + offset_y;
    let tx = target.x + offset_x;
    let ty = target.y + offset_y;

    let dx = tx - nx;
    let dy = ty - ny;
    let hw = node.width / 2.0;
    let hh = node.height / 2.0;

    if dx.abs() < 1e-9 && dy.abs() < 1e-9 {
        return (nx, ny);
    }

    // Find intersection with node rectangle
    let scale_x = if dx.abs() > 1e-9 { hw / dx.abs() } else { f64::INFINITY };
    let scale_y = if dy.abs() > 1e-9 { hh / dy.abs() } else { f64::INFINITY };
    let scale = scale_x.min(scale_y);

    (nx + dx * scale, ny + dy * scale)
}

fn arrow_marker(id: &str, reversed: bool) -> Marker {
    let path = if reversed {
        Data::new().move_to((10, 0)).line_to((0, 5)).line_to((10, 10))
    } else {
        Data::new().move_to((0, 0)).line_to((10, 5)).line_to((0, 10))
    };

    Marker::new()
        .set("id", id)
        .set("viewBox", "0 0 10 10")
        .set("refX", if reversed { 0 } else { 10 })
        .set("refY", 5)
        .set("markerWidth", 8)
        .set("markerHeight", 8)
        .set("orient", "auto-start-reverse")
        .add(
            element::Path::new()
                .set("d", path)
                .set("fill", EDGE_COLOR),
        )
}
