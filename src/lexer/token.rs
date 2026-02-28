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
    // punctuation
    LParen,
    RParen,

    // keywords
    Quote,
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
