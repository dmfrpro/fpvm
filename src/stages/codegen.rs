use crate::pipeline::stage::StageOutput;

use crate::pipeline::types::Diagnostic;

use crate::stages::types::CheckedProgram;

use crate::codegen::CodeGenerator;

pub fn codegen_stage(prog: CheckedProgram) -> StageOutput<String> {
    let mut generator = CodeGenerator::new(&prog.symbol_table);

    match generator.generate(&prog.ast) {
        Ok(program) => StageOutput::ok(program.to_string()),
        Err(error) => StageOutput::error(vec![Diagnostic::error(format!(
            "Codegen error: {:?}",
            error
        ))]),
    }
}
