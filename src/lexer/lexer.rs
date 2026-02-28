use super::token::{Span, Token, TokenKind};

#[derive(Debug)]
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

pub struct Lexer {
    input: String,
    pos: usize,
}

impl Lexer {
    pub fn new_empty() -> Self {
        Self { input: String::new(), pos: 0 }
    }

    pub fn new(input: String) -> Self {
        Self { input, pos: 0 }
    }

    pub fn push_line(&mut self, line: &str) {
        self.input.push_str(line);
        self.input.push('\n');
    }

    pub fn collect_tokens(&mut self) -> Result<Vec<Token>, LexError> {
        let mut out = Vec::new();
        while let Some(tok) = self.next_token()? {
            out.push(tok);
        }
        Ok(out)
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, LexError> {
        self.skip_ws_and_comments();

        if self.pos >= self.input.len() {
            return Ok(None);
        }

        let start = self.pos;
        let ch = self.peek_char().unwrap();

        let tok = match ch {
            '(' => { self.bump_char(); TokenKind::LParen }
            ')' => { self.bump_char(); TokenKind::RParen }
            '\'' => { self.bump_char(); TokenKind::Quote }

            c if c.is_ascii_digit()
                || ((c == '-' || c == '+')
                    && self.peek_nth_char(1).is_some_and(|n| n.is_ascii_digit())) =>
            {
                let (kind, end) = self.read_number(start)?;
                return Ok(Some(Token { kind, span: Span { start, end } }));
            }

            _ => {
                let (lexeme, end) = self.read_lexeme();
                let kind = classify_lexeme(&lexeme).map_err(|k| LexError {
                    kind: k,
                    span: Span { start, end },
                })?;
                return Ok(Some(Token { kind, span: Span { start, end } }));
            }
        };

        Ok(Some(Token { kind: tok, span: Span { start, end: self.pos } }))
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            while self.peek_char().is_some_and(|c| c.is_whitespace()) {
                self.bump_char();
            }

            if self.peek_char() == Some('#') {
                // comment to end of line
                while let Some(c) = self.peek_char() {
                    self.bump_char();
                    if c == '\n' { break; }
                }
                continue;
            }

            break;
        }
    }

    fn read_lexeme(&mut self) -> (String, usize) {
        let mut s = String::new();
        while let Some(c) = self.peek_char() {
            if is_delim(c) { break; }
            s.push(c);
            self.bump_char();
        }
        (s, self.pos)
    }

    fn read_number(&mut self, start: usize) -> Result<(TokenKind, usize), LexError> {
        // int = [+|-] Integer
        // real = [+|-] Integer.Integer

        let mut s = String::new();

        // the number should follow the sign
        if let Some(c) = self.peek_char() {
            if (c == '+' || c == '-') && self.peek_nth_char(1).is_some_and(|n| n.is_ascii_digit()) {
                s.push(c);
                self.bump_char();
            }
        }

        // Integer detection
        let mut int_digits = 0usize;
        while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            let c = self.bump_char().unwrap();
            s.push(c);
            int_digits += 1;
        }

        let mut is_real = false;
        let mut frac_digits = 0usize;

        // Real detection if symbol '.' occured
        if self.peek_char() == Some('.') && self.peek_nth_char(1).is_some_and(|n| n.is_ascii_digit()) {
            is_real = true;
            s.push('.');
            self.bump_char();

            while self.peek_char().is_some_and(|c| c.is_ascii_digit()) {
                let c = self.bump_char().unwrap();
                s.push(c);
                frac_digits += 1;
            }
        }

        // Checking for the end of a number. If there is no delim then error occurs. To prevent cases such as: '123asd' '123.123abc' ...
        if let Some(c) = self.peek_char() {
            if !is_delim(c) {
                return Err(LexError {
                    kind: LexErrorKind::UnexpectedChar(c),
                    span: Span { start, end: self.pos },
                });
            }
        }

        let total_digits = int_digits + frac_digits;
        if total_digits == 0 {
            return Err(LexError {
                kind: LexErrorKind::InvalidNumber(s),
                span: Span { start, end: self.pos },
            });
        }

        let kind = if is_real { TokenKind::Real(s) } else { TokenKind::Integer(s) };
        Ok((kind, self.pos))
    }

    fn peek_char(&self) -> Option<char> {
        self.input.as_str()[self.pos..].chars().next()
    }

    fn peek_nth_char(&self, n: usize) -> Option<char> {
        self.input.as_str()[self.pos..].chars().nth(n)
    }

    fn bump_char(&mut self) -> Option<char> {
        let c = self.peek_char()?;
        self.pos += c.len_utf8();
        Some(c)
    }
}

// set of characters that are delimeters
fn is_delim(c: char) -> bool {
    c.is_whitespace() || c == '(' || c == ')' || c == '#' || c == '\''
}

fn classify_lexeme(lex: &str) -> Result<TokenKind, LexErrorKind> {
    match lex {
        // literals
        "true" => Ok(TokenKind::Bool(true)),
        "false" => Ok(TokenKind::Bool(false)),
        "null" => Ok(TokenKind::Null),

        // keywords
        "quote" => Ok(TokenKind::Quote),
        "setq" => Ok(TokenKind::Setq),
        "func" => Ok(TokenKind::Func),
        "lambda" => Ok(TokenKind::Lambda),
        "prog" => Ok(TokenKind::Prog),
        "cond" => Ok(TokenKind::Cond),
        "while" => Ok(TokenKind::While),
        "return" => Ok(TokenKind::Return),
        "break" => Ok(TokenKind::Break),

        _ => {
            validate_identifier(lex)?;
            Ok(TokenKind::Identifier(lex.to_string()))
        }
    }
}

fn validate_identifier(lex: &str) -> Result<(), LexErrorKind> {
    if lex.is_empty() {
        return Err(LexErrorKind::InvalidIdentifier(lex.to_string()));
    }

    let mut chars = lex.chars();
    let first = chars.next().unwrap();
    if first.is_ascii_digit() {
        return Err(LexErrorKind::InvalidIdentifier(lex.to_string()));
    }

    if !is_ident_char(first) {
        return Err(LexErrorKind::InvalidIdentifier(lex.to_string()));
    }

    for c in chars {
        if !is_ident_char(c) {
            return Err(LexErrorKind::InvalidIdentifier(lex.to_string()));
        }
    }

    Ok(())
}

// set of characters allowed for the identifier
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric()
        || matches!(c, '_' )
}
