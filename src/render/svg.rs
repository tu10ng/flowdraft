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
                    style: EdgeStyle {
                        line_style: crate::parse::LineStyle::Straight,
                        ..EdgeStyle::default()
                    },
                };
                let edge_svg = render_edge(from_node, to_node, &edge, offset_x, offset_y, theme, ir);
                doc = doc.add(edge_svg);
            }
        }
    }

    // Render explicit edges
    for edge in &ir.edges {
        if let (Some(from_node), Some(to_node)) =
            (ir.nodes.get(&edge.from), ir.nodes.get(&edge.to))
        {
            let edge_svg = render_edge(from_node, to_node, edge, offset_x, offset_y, theme, ir);
            doc = doc.add(edge_svg);
            if let Some(ref label) = edge.label {
                let route = compute_edge_route(from_node, to_node, edge, offset_x, offset_y, ir);
                let label_svg = render_edge_label(&route, label, edge, theme);
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

// --- Obstacle avoidance routing ---

#[derive(Debug, Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    fn from_node(node: &Node, offset_x: f64, offset_y: f64) -> Self {
        Rect {
            x: node.x + offset_x - node.width / 2.0,
            y: node.y + offset_y - node.height / 2.0,
            w: node.width,
            h: node.height,
        }
    }

    fn from_group(group: &Group, offset_x: f64, offset_y: f64) -> Self {
        Rect {
            x: group.x + offset_x - group.width / 2.0,
            y: group.y + offset_y - group.height / 2.0,
            w: group.width,
            h: group.height,
        }
    }

    fn expanded(&self, margin: f64) -> Self {
        Rect {
            x: self.x - margin,
            y: self.y - margin,
            w: self.w + margin * 2.0,
            h: self.h + margin * 2.0,
        }
    }

    fn left(&self) -> f64 { self.x }
    fn right(&self) -> f64 { self.x + self.w }
    fn top(&self) -> f64 { self.y }
    fn bottom(&self) -> f64 { self.y + self.h }
    fn cx(&self) -> f64 { self.x + self.w / 2.0 }
    fn cy(&self) -> f64 { self.y + self.h / 2.0 }
}

/// Liang-Barsky line-segment vs AABB intersection test.
fn line_intersects_aabb(x1: f64, y1: f64, x2: f64, y2: f64, r: &Rect) -> bool {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let p = [-dx, dx, -dy, dy];
    let q = [
        x1 - r.left(),
        r.right() - x1,
        y1 - r.top(),
        r.bottom() - y1,
    ];

    let mut t_min = 0.0_f64;
    let mut t_max = 1.0_f64;

    for i in 0..4 {
        if p[i].abs() < 1e-12 {
            if q[i] < 0.0 {
                return false;
            }
        } else {
            let t = q[i] / p[i];
            if p[i] < 0.0 {
                t_min = t_min.max(t);
            } else {
                t_max = t_max.min(t);
            }
            if t_min > t_max {
                return false;
            }
        }
    }
    true
}

/// Find the first obstacle hit by the line segment, returning its index.
fn first_hit(start: (f64, f64), end: (f64, f64), obstacles: &[Rect]) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for (i, obs) in obstacles.iter().enumerate() {
        if line_intersects_aabb(start.0, start.1, end.0, end.1, obs) {
            let dist = (obs.cx() - start.0).powi(2) + (obs.cy() - start.1).powi(2);
            if best.map_or(true, |(_, d)| dist < d) {
                best = Some((i, dist));
            }
        }
    }
    best.map(|(i, _)| i)
}

fn collect_obstacles(
    ir: &DiagramIR,
    exclude: &HashSet<&str>,
    offset_x: f64,
    offset_y: f64,
    margin: f64,
) -> Vec<Rect> {
    let group_ids: HashSet<&str> = ir.groups.iter().map(|g| g.id.as_str()).collect();
    // Only skip children of groups that are NOT excluded (those are covered by the group rect).
    // Children of excluded groups should participate in obstacle detection normally.
    let covered_children: HashSet<&str> = ir.groups.iter()
        .filter(|g| !exclude.contains(g.id.as_str()))
        .flat_map(|g| g.children.iter().map(|c| c.as_str()))
        .collect();

    let mut rects = Vec::new();

    // Add groups as obstacles (excluding source/target groups)
    for group in &ir.groups {
        if group.width > 0.0 && !exclude.contains(group.id.as_str()) {
            rects.push(Rect::from_group(group, offset_x, offset_y).expanded(margin));
        }
    }

    // Add nodes that are not in excluded set, not group instance nodes, and not covered by a non-excluded group rect
    for node in ir.nodes.values() {
        if exclude.contains(node.id.as_str()) {
            continue;
        }
        if group_ids.contains(node.id.as_str()) {
            continue;
        }
        if covered_children.contains(node.id.as_str()) {
            continue;
        }
        rects.push(Rect::from_node(node, offset_x, offset_y).expanded(margin));
    }

    rects
}

fn compute_route(
    start: (f64, f64),
    end: (f64, f64),
    obstacles: &[Rect],
    depth: usize,
) -> Vec<(f64, f64)> {
    if depth == 0 {
        return vec![start, end];
    }

    let hit = match first_hit(start, end, obstacles) {
        Some(i) => i,
        None => return vec![start, end],
    };

    let obs = &obstacles[hit];
    let dx = (end.0 - start.0).abs();
    let dy = (end.1 - start.1).abs();

    // Choose bypass side: for mostly-horizontal lines, go above or below;
    // for mostly-vertical lines, go left or right.
    let waypoint = if dx >= dy {
        // Horizontal-ish: bypass above or below
        let mid_x = obs.cx();
        let above_dist = (start.1 - obs.top()).abs();
        let below_dist = (start.1 - obs.bottom()).abs();
        if above_dist <= below_dist {
            (mid_x, obs.top())
        } else {
            (mid_x, obs.bottom())
        }
    } else {
        // Vertical-ish: bypass left or right
        let mid_y = obs.cy();
        let left_dist = (start.0 - obs.left()).abs();
        let right_dist = (start.0 - obs.right()).abs();
        if left_dist <= right_dist {
            (obs.left(), mid_y)
        } else {
            (obs.right(), mid_y)
        }
    };

    // Exclude the bypassed obstacle from recursive calls to avoid infinite loops
    let remaining: Vec<Rect> = obstacles.iter().enumerate()
        .filter(|(i, _)| *i != hit)
        .map(|(_, r)| *r)
        .collect();

    let mut path = compute_route(start, waypoint, &remaining, depth - 1);
    path.pop(); // remove duplicate waypoint
    path.extend(compute_route(waypoint, end, &remaining, depth - 1));
    path
}

/// Build the SVG path data for a smooth cubic Bezier through waypoints.
fn smooth_bezier_path(points: &[(f64, f64)]) -> String {
    if points.len() < 2 {
        return String::new();
    }
    if points.len() == 2 {
        let (x0, y0) = points[0];
        let (x1, y1) = points[1];
        let dx = x1 - x0;
        let dy = y1 - y0;
        let cx1 = x0 + dx * 0.3;
        let cy1 = y0 + dy * 0.1;
        let cx2 = x1 - dx * 0.3;
        let cy2 = y1 - dy * 0.1;
        return format!("M {x0},{y0} C {cx1},{cy1} {cx2},{cy2} {x1},{y1}");
    }

    // Catmull-Rom to cubic Bezier conversion
    let mut d = format!("M {},{}", points[0].0, points[0].1);
    let n = points.len();
    for i in 0..n - 1 {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < n { points[i + 2] } else { points[n - 1] };

        let tension = 6.0;
        let cp1x = p1.0 + (p2.0 - p0.0) / tension;
        let cp1y = p1.1 + (p2.1 - p0.1) / tension;
        let cp2x = p2.0 - (p3.0 - p1.0) / tension;
        let cp2y = p2.1 - (p3.1 - p1.1) / tension;

        d.push_str(&format!(
            " C {cp1x},{cp1y} {cp2x},{cp2y} {},{}", p2.0, p2.1
        ));
    }
    d
}

/// Compute the route waypoints for an edge (used for both rendering and label placement).
fn compute_edge_route(
    from: &Node,
    to: &Node,
    edge: &Edge,
    offset_x: f64,
    offset_y: f64,
    ir: &DiagramIR,
) -> Vec<(f64, f64)> {
    let (fx, fy) = edge_point(from, to, offset_x, offset_y);
    let (tx, ty) = edge_point(to, from, offset_x, offset_y);

    match edge.style.line_style {
        crate::parse::LineStyle::Straight => vec![(fx, fy), (tx, ty)],
        crate::parse::LineStyle::Curved => {
            // Find which groups the source/target belong to
            let mut exclude: HashSet<&str> = HashSet::new();
            exclude.insert(edge.from.as_str());
            exclude.insert(edge.to.as_str());
            for group in &ir.groups {
                if group.children.iter().any(|c| c == &edge.from || c == &edge.to)
                    || group.id == edge.from
                    || group.id == edge.to
                {
                    exclude.insert(group.id.as_str());
                }
            }

            let obstacles = collect_obstacles(ir, &exclude, offset_x, offset_y, 8.0);
            compute_route((fx, fy), (tx, ty), &obstacles, 10)
        }
    }
}

fn render_edge(
    from: &Node,
    to: &Node,
    edge: &Edge,
    offset_x: f64,
    offset_y: f64,
    theme: &Theme,
    ir: &DiagramIR,
) -> element::Group {
    let color = edge
        .style
        .color
        .as_deref()
        .unwrap_or(&theme.edge_color);

    let route = compute_edge_route(from, to, edge, offset_x, offset_y, ir);

    let path_data = match edge.style.line_style {
        crate::parse::LineStyle::Straight => {
            let (fx, fy) = route[0];
            let (tx, ty) = route[route.len() - 1];
            format!("M {fx},{fy} L {tx},{ty}")
        }
        crate::parse::LineStyle::Curved => {
            if route.len() <= 2 {
                // No obstacles hit — draw a straight line
                let (fx, fy) = route[0];
                let (tx, ty) = route[route.len() - 1];
                format!("M {fx},{fy} L {tx},{ty}")
            } else {
                smooth_bezier_path(&route)
            }
        }
    };

    let mut path = element::Path::new()
        .set("d", path_data)
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", theme.edge_stroke_width);

    match edge.arrow {
        Arrow::Forward => {
            path = path.set("marker-end", "url(#arrow-forward)");
        }
        Arrow::Backward => {
            path = path.set("marker-start", "url(#arrow-backward)");
        }
        Arrow::Both => {
            path = path.set("marker-start", "url(#arrow-backward)");
            path = path.set("marker-end", "url(#arrow-forward)");
        }
        Arrow::None => {}
    }

    element::Group::new().add(path)
}

fn render_edge_label(
    route: &[(f64, f64)],
    label: &str,
    edge: &Edge,
    theme: &Theme,
) -> element::Text {
    let color = edge
        .style
        .color
        .as_deref()
        .unwrap_or(&theme.text_color);

    // Place label at the midpoint of the route
    let mid_idx = route.len() / 2;
    let (mx, my) = if route.len() % 2 == 0 && route.len() >= 2 {
        let (ax, ay) = route[mid_idx - 1];
        let (bx, by) = route[mid_idx];
        ((ax + bx) / 2.0, (ay + by) / 2.0 - 6.0)
    } else {
        (route[mid_idx].0, route[mid_idx].1 - 6.0)
    };

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
