use super::token::{Position, Span, Token, TokenKind};

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
    pos: Position,
}

impl Lexer {
    pub fn new_empty() -> Self {
        Self {
            input: String::new(),
            pos: Position::new(),
        }
    }

    pub fn new(input: String) -> Self {
        Self {
            input,
            pos: Position::new(),
        }
    }

    pub fn push_line(&mut self, line: &str) {
        if !line.is_empty() {
            self.input.push_str(line.trim_end());
            self.input.push('\n');
        }
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

        if self.pos.offset >= self.input.len() {
            return Ok(None);
        }

        let start = self.pos.clone();
        let ch = self.peek_char().unwrap();

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
                TokenKind::QuoteSign
            }

            c if c.is_ascii_digit()
                || ((c == '-' || c == '+')
                    && self.peek_nth_char(1).is_some_and(|n| n.is_ascii_digit())) =>
            {
                let (kind, end) = self.read_number()?;
                return Ok(Some(Token {
                    kind,
                    span: Span { start, end },
                }));
            }

            _ => {
                let (lexeme, end) = self.read_lexeme();
                let kind = classify_lexeme(&lexeme).map_err(|k| LexError {
                    kind: k,
                    span: Span {
                        start: start.clone(),
                        end: end.clone(),
                    },
                })?;

                return Ok(Some(Token {
                    kind,
                    span: Span { start, end },
                }));
            }
        };

        Ok(Some(Token {
            kind: tok,
            span: Span {
                start,
                end: self.pos.clone(),
            },
        }))
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
                    if c == '\n' {
                        break;
                    }
                }
                continue;
            }

            break;
        }
    }

    fn read_lexeme(&mut self) -> (String, Position) {
        let mut s = String::new();
        while let Some(c) = self.peek_char() {
            if is_delim(c) {
                break;
            }
            s.push(c);
            self.bump_char();
        }
        (s, self.pos.clone())
    }

    fn read_number(&mut self) -> Result<(TokenKind, Position), LexError> {
        // int = [+|-] Integer
        // real = [+|-] Integer.Integer

        let mut s = String::new();
        let start = self.pos.clone();

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
        if self.peek_char() == Some('.')
            && self.peek_nth_char(1).is_some_and(|n| n.is_ascii_digit())
        {
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
        // If after a number we have a non-delimiter, consume the whole junk suffix
        // to avoid producing a second token (e.g. "123abc" -> error + "abc").
        if self.peek_char().is_some_and(|c| !is_delim(c)) {
            // consume all chars until delimeter to achive correct behaviour:
            // input: 123abc
            // lexer output (wrong): UnexpectedChar('a') Identifier("abc") 
            // lexer output (good): InvalidNumber("123abc")
            while let Some(c) = self.peek_char() {
                if is_delim(c) {
                    break;
                }
                s.push(c);
                self.bump_char();
            }

            return Err(LexError {
                kind: LexErrorKind::InvalidNumber(s),
                span: Span {
                    start,
                    end: self.pos.clone(),
                },
            });
        }

        let total_digits = int_digits + frac_digits;
        if total_digits == 0 {
            return Err(LexError {
                kind: LexErrorKind::InvalidNumber(s),
                span: Span {
                    start,
                    end: self.pos.clone(),
                },
            });
        }

        let kind = if is_real {
            TokenKind::Real(s)
        } else {
            TokenKind::Integer(s)
        };
        Ok((kind, self.pos.clone()))
    }

    fn peek_char(&self) -> Option<char> {
        self.input.as_str()[self.pos.offset..].chars().next()
    }

    fn peek_nth_char(&self, n: usize) -> Option<char> {
        self.input.as_str()[self.pos.offset..].chars().nth(n)
    }

    fn bump_char(&mut self) -> Option<char> {
        let c = self.peek_char()?;
        if c == '\n' {
            self.pos.col = 1;
            self.pos.line += 1;
        } else {
            self.pos.col += 1;
        }
        self.pos.offset += c.len_utf8();
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
        "quote" => Ok(TokenKind::QuoteKeyword),
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
    let first = lex
        .chars()
        .next()
        .ok_or_else(|| LexErrorKind::InvalidIdentifier(lex.to_string()))?;

    if first.is_ascii_digit() {
        return Err(LexErrorKind::InvalidIdentifier(lex.to_string()));
    }

    if !lex.chars().all(is_ident_char) {
        return Err(LexErrorKind::InvalidIdentifier(lex.to_string()));
    }

    Ok(())
}

// set of characters allowed for the identifier
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_')
}
