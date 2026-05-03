use crate::lexer::Token;
use crate::pipeline::stage::StageOutput;

use crate::syntax::node;
use crate::syntax::parse_syntax;

pub fn syntax_stage(tokens: Vec<Token>) -> StageOutput<node::Node> {
    // Check tokens vector for errors
    // if you need correct sequence of tokens for parse_syntax function
    let (ast, errors) = parse_syntax(tokens);

    match (ast, errors.is_empty()) {
        (Some(node), true) => StageOutput::ok(node),
        (Some(node), false) => StageOutput::ok_with_diagnostics(node, Vec::new()),
        (None, _) => StageOutput::error(Vec::new()),
    }
}
