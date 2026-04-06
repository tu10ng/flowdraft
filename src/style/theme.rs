/// Unified theme configuration for the entire rendering pipeline.

#[derive(Debug, Clone)]
pub struct Theme {
    // Node sizing (from ir/build.rs)
    pub char_width: f64,
    pub node_height: f64,
    pub node_padding: f64,
    pub min_node_width: f64,
    // Layout gaps (from layout/tree.rs, flow.rs)
    pub h_gap: f64,
    pub v_gap: f64,
    // Node rendering (from style/defaults.rs)
    pub node_fill: String,
    pub node_stroke: String,
    pub node_stroke_width: f64,
    pub node_corner_radius: f64,
    // Edge rendering
    pub edge_color: String,
    pub edge_stroke_width: f64,
    // Text
    pub text_color: String,
    pub font_family: String,
    pub font_size: f64,
    // Canvas
    pub bg_color: String,
    pub svg_padding: f64,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            char_width: 9.0,
            node_height: 40.0,
            node_padding: 20.0,
            min_node_width: 80.0,
            h_gap: 20.0,
            v_gap: 30.0,
            node_fill: "#ffffff".into(),
            node_stroke: "#333333".into(),
            node_stroke_width: 1.5,
            node_corner_radius: 6.0,
            edge_color: "#555555".into(),
            edge_stroke_width: 1.5,
            text_color: "#333333".into(),
            font_family: "system-ui, -apple-system, sans-serif".into(),
            font_size: 14.0,
            bg_color: "#fafafa".into(),
            svg_padding: 40.0,
        }
    }
}
