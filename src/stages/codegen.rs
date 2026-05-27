use crate::pipeline::stage::StageOutput;

use crate::pipeline::types::Diagnostic;

use crate::stages::types::{CheckedProgram, GeneratedProg};

use crate::codegen::CodeGenerator;

pub fn codegen_stage_str(prog: CheckedProgram) -> StageOutput<String> {
    let mut generator = CodeGenerator::new(&prog.symbol_table);

    match generator.generate(&prog.ast) {
        Ok(program) => StageOutput::ok(program.to_string()),
        Err(error) => StageOutput::error(vec![Diagnostic::error(format!(
            "Codegen error: {:?}",
            error
        ))]),
    }
}

pub fn codegen_stage(prog: CheckedProgram) -> StageOutput<GeneratedProg> {
    let mut generator = CodeGenerator::new(&prog.symbol_table);

    match generator.generate(&prog.ast) {
        Ok(program) => {
            let gen_prog = GeneratedProg {
                ast: prog.ast,
                bytecode: program,
            };
            StageOutput::ok(gen_prog)
        }
        Err(error) => StageOutput::error(vec![Diagnostic::error(format!(
            "Codegen error: {:?}",
            error
        ))]),
    }
}
