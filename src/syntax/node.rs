use crate::lexer::token::Position;

#[derive(Debug)]
pub enum NodeKind {
    // Literal Nodes
    NullNode,
    BoolNode(bool),
    IntNode(i64),
    RealNode(f64),
    Identifier(String),

    // Special Forms
    QuoteNode(Box<Node>),
    SetqNode(Box<Node>, Box<Node>),
    FuncNode(Box<Node>, Box<Node>, Box<Node>),
    LambdaNode(Box<Node>, Box<Node>),
    ProgNode(Box<Node>, Box<Node>),
    CondNode(Box<Node>, Box<Node>, Option<Box<Node>>),
    WhileNode(Box<Node>, Box<Node>),
    ReturnNode(Box<Node>),
    BreakNode,

    // Complex nodes
    ElementNode(Box<Node>),
    ElementsNode(Vec<Box<Node>>),
    ListNode(Box<Node>),
    ProgramNode(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub span: (Position, Position)
}

impl Node {
    pub fn new(kind: NodeKind, span: (Position, Position)) -> Self {
        Self {
            kind,
            span,
        }
    }
}

#[derive(Debug)]
pub enum SyntaxErrorKind {
    Error(String),
    InvalidNumber(String),
    UnexpectedParse(String),
}

