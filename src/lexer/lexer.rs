use super::token::{Span, Token, TokenKind};

#[derive(Debug)]
pub enum LexErrorKind {
    UnexpectedChar(char),
    InvalidNumber(String),
}

#[derive(Debug)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    len: usize,
    finished: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            len: input.len(),
            finished: false,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_ws_and_comments();

        if self.pos >= self.len {
            self.finished = true;
            return Ok(Token {
                kind: TokenKind::Eof,
                span: Span {
                    start: self.len,
                    end: self.len,
                },
            });
        }

        let start = self.pos;
        let ch: char = self.peek_char().ok_or_else(|| LexError {
            kind: LexErrorKind::UnexpectedChar('\0'),
            span: Span { start, end: start },
        })?;

        let tok = match ch {
            '(' => {
                self.bump_char();
                TokenKind::LParen
            }
            ')' => {
                self.bump_char();
                TokenKind::RParen
            }
            '\'' => {
                self.bump_char();
                TokenKind::Quote
            }
            '.' => {
                self.bump_char();
                TokenKind::Dot
            }
            '+' => {
                self.bump_char();
                TokenKind::Plus
            }
            '-' => {
                self.bump_char();
                TokenKind::Minus
            }

            c if c.is_ascii_digit() => {
                let (digits, end) = self.read_integer_digits(start)?;
                return Ok(Token {
                    kind: TokenKind::Integer(digits),
                    span: Span { start, end },
                });
            }

            _ => {
                // Null(null), bool, Atom
                let (lexeme, end) = self.read_lexeme();
                let kind = classify_lexeme(&lexeme).map_err(|k| LexError {
                    kind: k,
                    span: Span { start, end },
                })?;
                return Ok(Token {
                    kind,
                    span: Span { start, end },
                });
            }
        };

        Ok(Token {
            kind: tok,
            span: Span {
                start,
                end: self.pos,
            },
        })
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            self.skip_whitespace();
            if self.peek_char() == Some('#') {
                self.skip_comment_hash();
                continue;
            }
            break;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.bump_char();
            } else {
                break;
            }
        }
    }

    fn skip_comment_hash(&mut self) {
        // comment starts with char #
        while let Some(c) = self.peek_char() {
            self.bump_char();
            if c == '\n' {
                break;
            }
        }
    }

    fn read_lexeme(&mut self) -> (String, usize) {
        let mut s = String::new();
        while let Some(c) = self.peek_char() {
            if is_delim(c) {
                break;
            }
            s.push(c);
            self.bump_char();
        }
        (s, self.pos)
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let c = self.peek_char()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn read_integer_digits(&mut self, start: usize) -> Result<(String, usize), LexError> {
        let mut s = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                s.push(c);
                self.bump_char();
            } else {
                break;
            }
        }

        let end = self.pos;

        if let Some(c) = self.peek_char() {
            if c.is_alphabetic() {
                return Err(LexError {
                    kind: LexErrorKind::UnexpectedChar(c),
                    span: Span { start, end },
                });
            }

        }

        Ok((s, end))
    }
}

fn is_delim(c: char) -> bool {
    c.is_whitespace()
        || c == '('
        || c == ')'
        || c == '\''
        || c == '#'
        || c == '.'
        || c == '+'
        || c == '-'
}

fn classify_lexeme(lex: &str) -> Result<TokenKind, LexErrorKind> {
    match lex {
        "true" => return Ok(TokenKind::Bool(true)),
        "false" => return Ok(TokenKind::Bool(false)),
        "null" => return Ok(TokenKind::Null),
        _ => {}
    }

    // everything else is an atom
    Ok(TokenKind::Atom(lex.to_string()))
}