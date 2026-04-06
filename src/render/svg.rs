/// SVG rendering from DiagramIR.
use svg::node::element::{self, path::Data, Definitions, Marker, Rectangle, Text};
use svg::Document;
use std::collections::HashSet;

use crate::ir::{DiagramIR, Edge, EdgeStyle, Group, Node};
use crate::parse::Arrow;
use crate::style::{self, Theme};
use super::Renderer;

pub struct SvgRenderer;

impl Renderer for SvgRenderer {
    type Output = String;

    fn render(&self, ir: &DiagramIR, theme: &Theme) -> String {
        render_svg(ir, theme)
    }
}

pub fn render_svg(ir: &DiagramIR, theme: &Theme) -> String {
    // Calculate bounding box
    let (min_x, min_y, max_x, max_y) = bounding_box(ir);
    let padding = theme.svg_padding;
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
        .add(arrow_marker("arrow-forward", false, &theme.edge_color))
        .add(arrow_marker("arrow-backward", true, &theme.edge_color));
    doc = doc.add(defs);

    // Add a background
    doc = doc.add(
        Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", theme.bg_color.as_str()),
    );

    // Render groups (behind edges and nodes)
    for group in &ir.groups {
        if group.width > 0.0 {
            let group_svg = render_group(group, offset_x, offset_y, theme);
            doc = doc.add(group_svg);
        }
    }

    // Render tree edges (parent-child connections)
    for tree_info in &ir.tree_roots {
        for (child_id, parent_id) in &tree_info.parent_map {
            if let (Some(parent), Some(child)) =
                (ir.nodes.get(parent_id), ir.nodes.get(child_id))
            {
                let edge_svg = render_tree_edge(parent, child, offset_x, offset_y, theme);
                doc = doc.add(edge_svg);
            }
        }
    }

    // Render flow edges
    for flow_info in &ir.flow_graphs {
        for (from_id, to_id, arrow) in &flow_info.adjacency {
            if let (Some(from_node), Some(to_node)) =
                (ir.nodes.get(from_id), ir.nodes.get(to_id))
            {
                let edge = Edge {
                    from: from_id.clone(),
                    to: to_id.clone(),
                    arrow: *arrow,
                    label: None,
                    style: EdgeStyle::default(),
                };
                let edge_svg = render_edge(from_node, to_node, &edge, offset_x, offset_y, theme);
                doc = doc.add(edge_svg);
            }
        }
    }

    // Render explicit edges
    for edge in &ir.edges {
        if let (Some(from_node), Some(to_node)) =
            (ir.nodes.get(&edge.from), ir.nodes.get(&edge.to))
        {
            let edge_svg = render_edge(from_node, to_node, edge, offset_x, offset_y, theme);
            doc = doc.add(edge_svg);
            if let Some(ref label) = edge.label {
                let label_svg = render_edge_label(from_node, to_node, label, edge, offset_x, offset_y, theme);
                doc = doc.add(label_svg);
            }
        }
    }

    // Render nodes (skip instance nodes that are represented by group containers)
    let group_ids: HashSet<&str> = ir.groups.iter().map(|g| g.id.as_str()).collect();
    for node in ir.nodes.values() {
        if group_ids.contains(node.id.as_str()) {
            continue;
        }
        let (rect, text) = render_node(node, offset_x, offset_y, theme);
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

    for group in &ir.groups {
        if group.width > 0.0 {
            let left = group.x - group.width / 2.0;
            let right = group.x + group.width / 2.0;
            let top = group.y - group.height / 2.0;
            let bottom = group.y + group.height / 2.0;
            min_x = min_x.min(left);
            min_y = min_y.min(top);
            max_x = max_x.max(right);
            max_y = max_y.max(bottom);
        }
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
    theme: &Theme,
) -> (element::Rectangle, element::Group) {
    let x = node.x + offset_x - node.width / 2.0;
    let y = node.y + offset_y - node.height / 2.0;

    let fill = node
        .style
        .fill
        .as_deref()
        .unwrap_or(&theme.node_fill);
    let stroke = node
        .style
        .stroke
        .as_deref()
        .unwrap_or(&theme.node_stroke);

    let rect = element::Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", node.width)
        .set("height", node.height)
        .set("rx", theme.node_corner_radius)
        .set("ry", theme.node_corner_radius)
        .set("fill", fill)
        .set("stroke", stroke)
        .set("stroke-width", theme.node_stroke_width);

    let text = Text::new(&node.label)
        .set("x", node.x + offset_x)
        .set("y", node.y + offset_y + theme.font_size / 3.0) // approximate vertical centering
        .set("text-anchor", "middle")
        .set("font-family", theme.font_family.as_str())
        .set("font-size", theme.font_size)
        .set("fill", theme.text_color.as_str());

    let group = element::Group::new().add(text);

    (rect, group)
}

fn render_group(
    group: &Group,
    offset_x: f64,
    offset_y: f64,
    theme: &Theme,
) -> element::Group {
    let title_h = style::defaults::GROUP_TITLE_HEIGHT;
    let x = group.x + offset_x - group.width / 2.0;
    let y = group.y + offset_y - group.height / 2.0;

    let fill = group.style.fill.as_deref().unwrap_or(style::defaults::GROUP_FILL);
    let stroke = group.style.stroke.as_deref().unwrap_or(style::defaults::GROUP_STROKE);

    // Background rect
    let bg = element::Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", group.width)
        .set("height", group.height)
        .set("rx", 8)
        .set("ry", 8)
        .set("fill", fill)
        .set("stroke", stroke)
        .set("stroke-width", 1.0);

    // Title bar background
    let title_bg = element::Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", group.width)
        .set("height", title_h)
        .set("rx", 8)
        .set("ry", 8)
        .set("fill", style::defaults::GROUP_TITLE_BG)
        .set("stroke", "none");

    // Cover the bottom corners of the title bar so only top is rounded
    let title_cover = element::Rectangle::new()
        .set("x", x)
        .set("y", y + title_h / 2.0)
        .set("width", group.width)
        .set("height", title_h / 2.0)
        .set("fill", style::defaults::GROUP_TITLE_BG)
        .set("stroke", "none");

    // Title text
    let title_text = Text::new(&group.label)
        .set("x", x + 10.0)
        .set("y", y + title_h / 2.0 + style::defaults::GROUP_TITLE_FONT_SIZE / 3.0)
        .set("font-family", theme.font_family.as_str())
        .set("font-size", style::defaults::GROUP_TITLE_FONT_SIZE)
        .set("fill", theme.text_color.as_str());

    element::Group::new()
        .add(bg)
        .add(title_bg)
        .add(title_cover)
        .add(title_text)
}

fn render_tree_edge(
    parent: &Node,
    child: &Node,
    offset_x: f64,
    offset_y: f64,
    theme: &Theme,
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
        .set("stroke", theme.edge_color.as_str())
        .set("stroke-width", theme.edge_stroke_width)
}

fn render_edge(
    from: &Node,
    to: &Node,
    edge: &Edge,
    offset_x: f64,
    offset_y: f64,
    theme: &Theme,
) -> element::Line {
    let color = edge
        .style
        .color
        .as_deref()
        .unwrap_or(&theme.edge_color);

    // Connect from edge of nodes
    let (fx, fy) = edge_point(from, to, offset_x, offset_y);
    let (tx, ty) = edge_point(to, from, offset_x, offset_y);

    let mut line = element::Line::new()
        .set("x1", fx)
        .set("y1", fy)
        .set("x2", tx)
        .set("y2", ty)
        .set("stroke", color)
        .set("stroke-width", theme.edge_stroke_width);

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
    theme: &Theme,
) -> element::Text {
    let color = edge
        .style
        .color
        .as_deref()
        .unwrap_or(&theme.text_color);

    let mx = (from.x + to.x) / 2.0 + offset_x;
    let my = (from.y + to.y) / 2.0 + offset_y - 6.0;

    Text::new(label)
        .set("x", mx)
        .set("y", my)
        .set("text-anchor", "middle")
        .set("font-family", theme.font_family.as_str())
        .set("font-size", theme.font_size * 0.85)
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

fn arrow_marker(id: &str, reversed: bool, edge_color: &str) -> Marker {
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
                .set("fill", edge_color),
        )
}
