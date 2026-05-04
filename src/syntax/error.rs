use std::fmt;
use std::fmt::Display;

use crate::syntax::node::MultilinePosition;

#[derive(Debug)]
pub enum SyntaxErrorKind {
    Error, // General Error
    InvalidNumber,
    UnexpectedToken,
    InvalidToken,
    UnrecognizedEof,
    ExtraToken,
}

impl Display for SyntaxErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            SyntaxErrorKind::Error => "Error",
            SyntaxErrorKind::InvalidNumber => "InvalidNumber",
            SyntaxErrorKind::UnexpectedToken => "UnexpectedToken",
            SyntaxErrorKind::InvalidToken => "InvalidToken",
            SyntaxErrorKind::UnrecognizedEof => "UnrecognizedEof",
            SyntaxErrorKind::ExtraToken => "ExtraToken",
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug)]
pub struct SyntaxError {
    kind: SyntaxErrorKind,
    message: Option<String>,
    span: MultilinePosition,
}

impl SyntaxError {
    pub fn new(kind: SyntaxErrorKind, message: Option<String>, span: MultilinePosition) -> Self {
        Self {
            kind,
            message,
            span,
        }
    }

    pub fn from_parse_error(
        err: lalrpop_util::ParseError<
            crate::lexer::token::Position,
            crate::lexer::token::TokenKind,
            SyntaxError,
        >,
    ) -> Self {
        match err {
            lalrpop_util::ParseError::InvalidToken { location } => SyntaxError::new(
                SyntaxErrorKind::InvalidToken,
                None,
                MultilinePosition::from_position(location),
            ),
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => SyntaxError::new(
                SyntaxErrorKind::UnrecognizedEof,
                Some(format!("Expected: {}", expected.join(", "))),
                MultilinePosition::from_position(location),
            ),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                let (start, found, end) = token;
                SyntaxError::new(
                    SyntaxErrorKind::UnexpectedToken,
                    Some(format!(
                        "Found: {:?}. Expected: {}",
                        found,
                        expected.join(", ")
                    )),
                    MultilinePosition::from_positions(start, end),
                )
            }
            lalrpop_util::ParseError::ExtraToken { token } => {
                let (start, found, end) = token;
                SyntaxError::new(
                    SyntaxErrorKind::ExtraToken,
                    Some(format!("Found: {:?}", found)),
                    MultilinePosition::from_positions(start, end),
                )
            }
            lalrpop_util::ParseError::User { error } => error,
        }
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.message.is_some() {
            writeln!(
                f,
                "{}: {} at {}",
                self.kind,
                self.message.clone().unwrap(),
                self.span
            )
        } else {
            writeln!(f, "{} at {}", self.kind, self.span)
        }
    }
}
