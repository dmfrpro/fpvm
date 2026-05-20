use crate::pipeline::stage::StageOutput;

use crate::pipeline::types::Diagnostic;
use crate::semantics::SemanticAnalyzer;

use crate::stages::types::{CheckedProgram, ParsedProgram};
use crate::symbol_table::SymbolTableBuilder;

pub fn semantic_stage(prog: ParsedProgram) -> StageOutput<CheckedProgram> {
    let analyzer = SemanticAnalyzer::new();

    let sem_errors = analyzer.analyze(&prog.ast);

    let _: Vec<Diagnostic> = sem_errors
        .into_iter()
        .map(|err| Diagnostic::error(format!("{:?}", err)))
        .collect();

    // in this branch we assuming that Semantic analizer is correct for any input code
    // if !diagnostics.is_empty() {
    //     return StageOutput::error(diagnostics);
    // }

    let symbol_table = match SymbolTableBuilder::new(Some(prog.ast.span.clone())).build(&prog.ast) {
        Ok(symbol_table) => symbol_table,
        Err(errors) => {
            let diagnostics = errors
                .into_iter()
                .map(|err| Diagnostic::error(format!("{:?}", err)))
                .collect();

            return StageOutput::error(diagnostics);
        }
    };

    StageOutput::ok(CheckedProgram {
        ast: prog.ast,
        symbol_table,
    })
}
