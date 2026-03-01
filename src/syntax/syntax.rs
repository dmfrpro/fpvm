use lalrpop_util::{ParseError, lalrpop_mod};
lalrpop_mod!(pub grammar, "/syntax/grammar.rs");

use crate::lexer::{Token, TokenKind, Position};
use crate::syntax::{Node, SyntaxError, SyntaxErrorKind, MultilinePosition};

pub fn parse_syntax(tokens: Vec<Token>) -> Result<Node, SyntaxError> {
    let token_iter = tokens.into_iter().map(|tok| (tok.span.start, tok.kind, tok.span.end));

    let parser = grammar::ProgramParser::new();
    match parser.parse(token_iter) {
        Ok(inner_result) => inner_result,
        Err(parse_error) => Err(get_parsed_error(parse_error)),
    }
}

fn get_parsed_error(err: ParseError<Position, TokenKind, SyntaxError>) -> SyntaxError {
    match err {
        ParseError::InvalidToken { location } => {
            SyntaxError::new(SyntaxErrorKind::InvalidToken, None, MultilinePosition::from_position(location))
        }
        ParseError::UnrecognizedEof { location, expected } => {
            let message = format!("Expected: {}", expected.join(", "));
            SyntaxError::new(SyntaxErrorKind::UnrecognizedEof, Some(message), MultilinePosition::from_position(location))
        }
        ParseError::UnrecognizedToken { token, expected } => {
            // token is (start, TokenKind, end)
            let (start, found, end) = token;
            let message = format!("Found: {:?}. Expected: {}", found, expected.join(", "));
            SyntaxError::new(SyntaxErrorKind::UnexpectedToken, Some(message), MultilinePosition::from_positions(start, end))
        }
        ParseError::ExtraToken { token } => {
            let (start, found, end) = token;
            let message = format!("Found: {:?}", found);
            SyntaxError::new(SyntaxErrorKind::ExtraToken, Some(message), MultilinePosition::from_positions(start, end))
        }
        ParseError::User { error } => {
            error
        }
    }
}