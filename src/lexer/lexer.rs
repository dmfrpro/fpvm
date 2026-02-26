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

#[derive(Debug)]
pub enum LexStep {
    Token(Token),
    NeedMoreInput,
    Finished,
}

pub struct Lexer {
    input: String,
    pos: usize,
    finished: bool,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            pos: 0,
            finished: false,
        }
    }

    pub fn new_empty() -> Self {
        Self::new(String::new())
    }

    pub fn push_str(&mut self, more: &str) {
        self.input.push_str(more);
    }

    pub fn push_line(&mut self, line: &str) {
        self.input.push_str(line);
        self.input.push('\n');
    }

    pub fn set_finished(&mut self) {
        self.finished = true;
    }

    pub fn collect_tokens(&mut self) -> Result<Vec<Token>, LexError> {
        let mut out = Vec::new();

        loop {
            match self.next_token()? {
                LexStep::Token(tok) => out.push(tok),
                LexStep::NeedMoreInput => break,
                LexStep::Finished => {
                    let at = self.pos;
                    out.push(Token {
                        kind: TokenKind::Eof,
                        span: Span { start: at, end: at },
                    });
                    break;
                }
            }
        }

        Ok(out)
    }

    pub fn next_token(&mut self) -> Result<LexStep, LexError> {
        self.skip_ws_and_comments();

        let len = self.input.len();

        // println!("len:{}, pos:{}", len, self.pos);
        if self.pos >= len {
            return Ok(if self.finished {
                LexStep::Finished
            } else {
                LexStep::NeedMoreInput
            });
        }

        let start = self.pos;
        let ch: char = self.peek_char().ok_or_else(|| LexError {
            kind: LexErrorKind::UnexpectedChar('\0'),
            span: Span { start, end: start },
        })?;

        match ch {
            '(' => {
                self.bump_char();
                Ok(LexStep::Token(Token {
                    kind: TokenKind::LParen,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                }))
            }
            ')' => {
                self.bump_char();
                Ok(LexStep::Token(Token {
                    kind: TokenKind::RParen,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                }))
            }
            '\'' => {
                self.bump_char();
                Ok(LexStep::Token(Token {
                    kind: TokenKind::Quote,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                }))
            }
            '.' => {
                self.bump_char();
                Ok(LexStep::Token(Token {
                    kind: TokenKind::Dot,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                }))
            }
            '+' => {
                self.bump_char();
                Ok(LexStep::Token(Token {
                    kind: TokenKind::Plus,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                }))
            }
            '-' => {
                self.bump_char();
                Ok(LexStep::Token(Token {
                    kind: TokenKind::Minus,
                    span: Span {
                        start,
                        end: self.pos,
                    },
                }))
            }

            c if c.is_ascii_digit() => {
                let (digits, end, complete) = self.read_integer_digits_streaming(start)?;
                if !complete {
                    self.pos = start; // rallback
                    return Ok(LexStep::NeedMoreInput);
                }
                return Ok(LexStep::Token(Token {
                    kind: TokenKind::Integer(digits),
                    span: Span { start, end },
                }));
            }

            _ => {
                let (lexeme, end, complete) = self.read_lexeme_streaming();
                if !complete {
                    self.pos = start; // rollback
                    return Ok(LexStep::NeedMoreInput);
                }

                let kind = classify_lexeme(&lexeme).map_err(|k| LexError {
                    kind: k,
                    span: Span { start, end },
                })?;

                return Ok(LexStep::Token(Token {
                    kind,
                    span: Span { start, end },
                }));
            }
        }
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

    fn read_lexeme_streaming(&mut self) -> (String, usize, bool) {
        let mut s = String::new();
        let mut saw_delim = false;

        while let Some(c) = self.peek_char() {
            if is_delim(c) {
                saw_delim = true;
                break;
            }
            s.push(c);
            self.bump_char();
        }

        let end = self.pos;
        let complete = saw_delim || self.finished;
        (s, end, complete)
    }

    fn peek_char(&self) -> Option<char> {
        self.input.as_str()[self.pos..].chars().next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let c = self.peek_char()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn read_integer_digits_streaming(&mut self, start: usize) -> Result<(String, usize, bool), LexError> {
        let mut s = String::new();
        let mut saw_stop = false;

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                s.push(c);
                self.bump_char();
            } else {
                saw_stop = true;
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

        let complete = saw_stop || self.finished;
        Ok((s, end, complete))
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
        "quote" => return Ok(TokenKind::Quote),
        _ => {}
    }

    // everything else is an atom
    Ok(TokenKind::Atom(lex.to_string()))
}
