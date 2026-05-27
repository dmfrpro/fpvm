use std::fmt;
use super::token::Span;

#[derive(Debug, Clone)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    InvalidNumber(String),
    InvalidIdentifier(String),
}

#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}


impl fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexErrorKind::UnexpectedChar(c) => {
                write!(f, "unexpected character '{}'", c)
            }
            LexErrorKind::InvalidNumber(s) => {
                write!(f, "invalid number '{}'", s)
            }
            LexErrorKind::InvalidIdentifier(s) => {
                write!(f, "invalid identifier '{}'", s)
            }
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:{}", self.kind, self.span)
    }
}