use lalrpop_util::{ErrorRecovery, lalrpop_mod};
lalrpop_mod!(pub grammar, "/syntax/grammar.rs");

use crate::lexer::{Position, Token, TokenKind};
use crate::syntax::{Node, SyntaxError};

pub fn parse_syntax(tokens: Vec<Token>) -> (Option<Node>, Vec<SyntaxError>) {
    let token_iter = tokens
        .into_iter()
        .map(|tok| (tok.span.start, tok.kind, tok.span.end));

    let mut recoveries: Vec<ErrorRecovery<Position, TokenKind, SyntaxError>> = Vec::new();

    let parser = grammar::ProgramParser::new();
    let ast = match parser.parse(&mut recoveries, token_iter) {
        Ok(node) => Some(node),
        Err(parse_error) => {
            recoveries.push(ErrorRecovery {
                error: parse_error,
                dropped_tokens: Vec::new(),
            });
            None
        }
    };

    let syntax_errors = recoveries
        .into_iter()
        .map(|recovery| SyntaxError::from_parse_error(recovery.error))
        .collect();

    (ast, syntax_errors)
}
