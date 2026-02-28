mod lexer;

pub enum NodeKind {
    // Literal Nodes
    NullNode,
    BoolNode(bool),
    IntNode(i64),
    RealNode(f64),
    Identifier(String),

    // Complex nodes
    ElementNode(Node),
    ElementsNode(Vec<Node>),
    ListNode(Node),
    ProgramNode(Node),
}

pub struct Node {
    pub kind: NodeKind,
    pub span: Span
}

impl Node {
    pub fn new(kind: NodeKind, start: usize, end: usize, line: usize) -> Self {
        Self {
            kind,
            span: Span { start, end, line }
        }
    }
}

pub enum SyntaxErrorKind {
    Error(String),
    InvalidNumber(String),
    UnexpectedParse(String),
}

