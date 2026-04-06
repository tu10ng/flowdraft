/// AST types for the flowdraft DSL.

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub forms: Vec<Form>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Form {
    Tree {
        direction: Direction,
        options: Vec<(String, Option<String>)>,
        root: TreeNode,
    },
    Line {
        line_style: LineStyle,
        from: String,
        arrow: Arrow,
        to: String,
        options: Vec<(String, Option<String>)>,
    },
    Style {
        target: String,
        props: Vec<(String, String)>,
    },
    Flow {
        direction: Direction,
        chains: Vec<FlowChain>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Down,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum LineStyle {
    #[default]
    Straight,
    Curved,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arrow {
    Forward,  // ->
    Backward, // <-
    Both,     // <->
    None,     // --
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeNode {
    pub name: String,
    pub label: Option<String>,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowChain {
    pub segments: Vec<FlowSegment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowSegment {
    pub node: String,
    pub arrow: Option<Arrow>, // None for last node in chain
}
