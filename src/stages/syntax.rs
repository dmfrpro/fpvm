use crate::lexer::Token;
use crate::pipeline::stage::StageOutput;

use crate::pipeline::types::Diagnostic;
use crate::stages::types::ParsedProgram;
use crate::syntax::parse_syntax;

pub fn syntax_stage(tokens: Vec<Token>) -> StageOutput<ParsedProgram> {
    // TODO: Check tokens vector for errors
    let (node, errors) = parse_syntax(tokens);

    let diagnostics: Vec<Diagnostic> = errors
        .into_iter()
        .map(|err| Diagnostic::error(err.to_string()))
        .collect();

    match (node, diagnostics.is_empty()) {
        (Some(ast), true) => StageOutput::ok(ParsedProgram { ast }),
        (Some(ast), false) => StageOutput::ok_with_diagnostics(ParsedProgram { ast }, diagnostics),
        (None, _) => StageOutput::error(diagnostics),
    }
}
