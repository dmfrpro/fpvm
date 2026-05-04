use crate::pipeline::stage::StageOutput;

use crate::pipeline::types::Diagnostic;
use crate::semantics::SemanticAnalyzer;

use crate::stages::types::{CheckedProgram, ParsedProgram};
pub fn semantic_stage(prog: ParsedProgram) -> StageOutput<CheckedProgram> {
    let analyzer = SemanticAnalyzer::new();
    let sem_errors = analyzer.analyze(&prog.ast);

    let diagnostics: Vec<Diagnostic> = sem_errors
        .into_iter()
        .map(|err| Diagnostic::error(format!("{:?}", err)))
        .collect();

    if !diagnostics.is_empty() {
        return StageOutput::error(diagnostics);
    }
    
    StageOutput::ok(CheckedProgram { ast: prog.ast })
}
