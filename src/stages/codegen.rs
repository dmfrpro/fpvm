use crate::pipeline::stage::StageOutput;

use crate::pipeline::types::Diagnostic;

use crate::stages::types::CheckedProgram;
pub fn codegen_stage(prog: CheckedProgram) -> StageOutput<String> {
    StageOutput::error(vec![Diagnostic::error(String::from(
        format!("Codegen stage not implemented! pron unused {}", prog),
    ))])
}
