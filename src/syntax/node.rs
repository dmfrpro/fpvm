use crate::lexer::token::Position;
use std::fmt;
use std::fmt::Display;

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

    fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
        // Helper to write spaces for indentation
        let indent_str = " ".repeat(indent);
        write!(f, "{}", indent_str)?;

        match &self.kind {
            NodeKind::NullNode => writeln!(f, "Null"),
            NodeKind::BoolNode(val) => writeln!(f, "Bool({})", val),
            NodeKind::IntNode(val) => writeln!(f, "Int({})", val),
            NodeKind::RealNode(val) => writeln!(f, "Real({})", val),
            NodeKind::Identifier(name) => writeln!(f, "Identifier({})", name),

            NodeKind::QuoteNode(subnode) => {
                writeln!(f, "Quote(")?;
                subnode.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::SetqNode(atom, elem) => {
                writeln!(f, "Setq(")?;
                atom.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::FuncNode(atom, list, elem) => {
                writeln!(f, "Func(")?;
                atom.fmt_with_indent(f, indent + 2)?;
                list.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::LambdaNode(list, elem) => {
                writeln!(f, "Func(")?;
                list.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::ProgNode(list, elem) => {
                writeln!(f, "Func(")?;
                list.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::CondNode(elem1, elem2, elem3) => {
                writeln!(f, "Func(")?;
                elem1.fmt_with_indent(f, indent + 2)?;
                elem2.fmt_with_indent(f, indent + 2)?;

                if let Some(elem) = elem3 {
                    elem.fmt_with_indent(f, indent + 2)?;
                }

                writeln!(f, "{})", indent_str)
            }
            NodeKind::WhileNode(cond, body) => {
                writeln!(f, "Func(")?;
                cond.fmt_with_indent(f, indent + 2)?;
                body.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::ReturnNode(elem) => {
                writeln!(f, "Func(")?;
                elem.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::BreakNode => {
                writeln!(f, "Break")
            }
            NodeKind::ElementNode(subexpr) => {
                writeln!(f, "Element(")?;
                subexpr.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::ElementsNode(subexprs) => {
                writeln!(f, "Elements(")?;
                for subexpr in subexprs {
                    subexpr.fmt_with_indent(f, indent + 2)?;
                }
                writeln!(f, "{})", indent_str)
            }
            NodeKind::ListNode(subexpr) => {
                writeln!(f, "List(")?;
                subexpr.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
            NodeKind::ProgramNode(subexpr) => {
                writeln!(f, "Program(")?;
                subexpr.fmt_with_indent(f, indent + 2)?;
                writeln!(f, "{})", indent_str)
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

#[derive(Debug)]
pub enum SyntaxErrorKind {
    Error(String),
    InvalidNumber(String),
    UnexpectedParse(String),
}

