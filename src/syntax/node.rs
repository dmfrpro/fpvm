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
    ErrorNode,
}

#[derive(Debug)]
pub struct Node {
    pub kind: NodeKind,
    pub span: MultilinePosition,
}

impl Node {
    pub fn new(kind: NodeKind, span: (Position, Position)) -> Self {
        Self {
            kind,
            span: MultilinePosition::from_positions(span.0, span.1),
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
                writeln!(f, "Quote")?;
                subnode.fmt_with_indent(f, indent + 2)
            }
            NodeKind::SetqNode(atom, elem) => {
                writeln!(f, "Setq")?;
                atom.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)
            }
            NodeKind::FuncNode(atom, list, elem) => {
                writeln!(f, "Func")?;
                atom.fmt_with_indent(f, indent + 2)?;
                list.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)
            }
            NodeKind::LambdaNode(list, elem) => {
                writeln!(f, "Lambda")?;
                list.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)
            }
            NodeKind::ProgNode(list, elem) => {
                writeln!(f, "Prog")?;
                list.fmt_with_indent(f, indent + 2)?;
                elem.fmt_with_indent(f, indent + 2)
            }
            NodeKind::CondNode(elem1, elem2, elem3) => {
                writeln!(f, "Cond")?;
                elem1.fmt_with_indent(f, indent + 2)?;
                elem2.fmt_with_indent(f, indent + 2)?;

                if let Some(elem) = elem3 {
                    elem.fmt_with_indent(f, indent + 2)?;
                }

                Ok(())
            }
            NodeKind::WhileNode(cond, body) => {
                writeln!(f, "While")?;
                cond.fmt_with_indent(f, indent + 2)?;
                body.fmt_with_indent(f, indent + 2)
            }
            NodeKind::ReturnNode(elem) => {
                writeln!(f, "Return")?;
                elem.fmt_with_indent(f, indent + 2)
            }
            NodeKind::BreakNode => {
                writeln!(f, "Break")
            }
            NodeKind::ElementNode(subexpr) => {
                writeln!(f, "Element")?;
                subexpr.fmt_with_indent(f, indent + 2)
            }
            NodeKind::ElementsNode(subexprs) => {
                writeln!(f, "Elements")?;
                for subexpr in subexprs {
                    subexpr.fmt_with_indent(f, indent + 2)?;
                }
                Ok(())
            }
            NodeKind::ListNode(subexpr) => {
                writeln!(f, "List")?;
                subexpr.fmt_with_indent(f, indent + 2)
            }
            NodeKind::ProgramNode(subexpr) => {
                writeln!(f, "Program")?;
                subexpr.fmt_with_indent(f, indent + 2)
            }
            NodeKind::ErrorNode => {
                writeln!(f, "Error")
            }
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

#[derive(Debug, Clone)]
pub struct MultilinePosition {
    pub start_col: usize,
    pub start_line: usize,

    pub end_col: usize,
    pub end_line: usize,
}

impl MultilinePosition {
    pub fn from_positions(start: Position, end: Position) -> Self {
        Self {
            start_col: start.col,
            start_line: start.line,
            end_col: end.col,
            end_line: end.line,
        }
    }

    pub fn from_position(position: Position) -> Self {
        Self {
            start_col: position.col,
            start_line: position.line,
            end_col: position.col,
            end_line: position.line,
        }
    }
}

impl Default for MultilinePosition {
    fn default() -> Self {
        Self {
            start_col: 1,
            start_line: 0,
            end_col: 1,
            end_line: 0,
        }
    }
}

impl fmt::Display for MultilinePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.start_line == self.end_line {
            // Single line
            if self.start_col == self.end_col {
                // Single column
                write!(f, "at line {}, column {}", self.start_line, self.start_col)
            } else {
                // Range on one line
                write!(
                    f,
                    "at line {}, columns {}-{}",
                    self.start_line, self.start_col, self.end_col
                )
            }
        } else {
            // Multi‑line range
            write!(
                f,
                "from line {}:{} to line {}:{}",
                self.start_line, self.start_col, self.end_line, self.end_col
            )
        }
    }
}
