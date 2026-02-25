#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,  
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenKind {
    LParen,
    RParen,
    Quote,
    Dot,
    Plus,
    Minus,

    Integer(String),
    Bool(bool),
    Null,
    Atom(String),

    Eof,
}
