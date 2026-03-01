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
}
