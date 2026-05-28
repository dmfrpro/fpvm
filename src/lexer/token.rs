use crate::lexer::LexErrorKind;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub offset: usize,
    pub col: usize,
    pub line: usize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            offset: 0,
            col: 1,
            line: 1,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            offset: 0,
            col: 1,
            line: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    // punctuation
    LParen,
    RParen,

    // keywords
    QuoteKeyword,
    QuoteSign,
    Setq,
    Func,
    Lambda,
    Prog,
    Cond,
    While,
    Return,
    Break,

    // literals
    Integer(String),
    Real(String),
    Bool(bool),
    Null,

    Identifier(String),

    // invalid token for parser recovery
    Invalid(LexErrorKind),
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line:{} col:{}", self.line, self.col)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::RParen => write!(f, "RParen"),

            TokenKind::QuoteKeyword => write!(f, "QuoteKeyword"),
            TokenKind::QuoteSign => write!(f, "QuoteSign"),
            TokenKind::Setq => write!(f, "Setq"),
            TokenKind::Func => write!(f, "Func"),
            TokenKind::Lambda => write!(f, "Lambda"),
            TokenKind::Prog => write!(f, "Prog"),
            TokenKind::Cond => write!(f, "Cond"),
            TokenKind::While => write!(f, "While"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::Break => write!(f, "Break"),

            TokenKind::Integer(value) => write!(f, "Integer({value})"),
            TokenKind::Real(value) => write!(f, "Real({value})"),
            TokenKind::Bool(value) => write!(f, "Bool({value})"),
            TokenKind::Null => write!(f, "Null"),

            TokenKind::Identifier(name) => write!(f, "Identifier({name})"),

            TokenKind::Invalid(error) => write!(f, "Invalid({error:?})"),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self.kind.to_string();

        write!(f, "{kind:<20} {}", self.span)
    }
}
