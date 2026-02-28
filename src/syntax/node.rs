pub enum Node {
    // Literal Nodes
    NullNode,
    BoolNode(bool),
    IntNode(i64),
    RealNode(f64),
    Identifier(String),

    ElementNode(Box<Node>),
    ElementsNode(Vec<Box<Node>>),
    ListNode(Box<Node>),
    ProgramNode(Box<Node>),
}

pub enum SyntaxErrorKind {
    Error(String),
    InvalidNumber(String),
    UnexpectedParse(String),
}

