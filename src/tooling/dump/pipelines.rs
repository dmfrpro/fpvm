use std::path::Path;

use crate::pipeline::pipeline::Pipeline;
use crate::stages::{codegen_stage, lexer_stage, read_from_file, semantic_stage, syntax_stage};

use super::config::DumpStage;

pub fn run_stage(input_file: &Path, stage: DumpStage) -> String {
    match stage {
        DumpStage::Lexer => run_lexer(input_file),
        DumpStage::Syntax => run_syntax(input_file),
        DumpStage::Semantic => run_semantic(input_file),
        DumpStage::Codegen => run_codegen(input_file),
        DumpStage::Vm => {
            panic!("VM stage is not implemented yet")
        }
    }
}

fn run_lexer(input_file: &Path) -> String {
    let input_file = input_file.to_path_buf();

    let pipeline = Pipeline::new(read_from_file).then(lexer_stage);

    let output = pipeline.run(&input_file);

    let diagnostics = output.diagnostics;
    let has_errors = !diagnostics.is_empty();

    match output.value {
        Some(value) if !has_errors => {
            format!("{:#?}", value)
        }

        Some(value) => {
            format!(
                "[PARTIAL]\n\n{:#?}\n\nDiagnostics:\n{:#?}\n",
                value, diagnostics,
            )
        }

        None => {
            format!("[FAILED]\n\nDiagnostics:\n{:#?}\n", diagnostics,)
        }
    }
}

fn run_syntax(input_file: &Path) -> String {
    let input_file = input_file.to_path_buf();

    let pipeline = Pipeline::new(read_from_file)
        .then(lexer_stage)
        .then(syntax_stage);

    let output = pipeline.run(&input_file);

    let diagnostics = output.diagnostics;
    let has_errors = !diagnostics.is_empty();

    match output.value {
        Some(value) if !has_errors => {
            format!("{:#?}", value)
        }

        Some(value) => {
            format!(
                "[PARTIAL]\n\n{:#?}\n\nDiagnostics:\n{:#?}\n",
                value, diagnostics,
            )
        }

        None => {
            format!("[FAILED]\n\nDiagnostics:\n{:#?}\n", diagnostics,)
        }
    }
}

fn run_semantic(input_file: &Path) -> String {
    let input_file = input_file.to_path_buf();

    let pipeline = Pipeline::new(read_from_file)
        .then(lexer_stage)
        .then(syntax_stage)
        .then(semantic_stage);

    let output = pipeline.run(&input_file);

    let diagnostics = output.diagnostics;
    let has_errors = !diagnostics.is_empty();

    match output.value {
        Some(value) if !has_errors => {
            format!("{:#?}", value)
        }

        Some(value) => {
            format!(
                "[PARTIAL]\n\n{:#?}\n\nDiagnostics:\n{:#?}\n",
                value, diagnostics,
            )
        }

        None => {
            format!("[FAILED]\n\nDiagnostics:\n{:#?}\n", diagnostics,)
        }
    }
}

fn run_codegen(input_file: &Path) -> String {
    let input_file = input_file.to_path_buf();

    let pipeline = Pipeline::new(read_from_file)
        .then(lexer_stage)
        .then(syntax_stage)
        .then(semantic_stage)
        .then(codegen_stage);

    let output = pipeline.run(&input_file);

    let diagnostics = output.diagnostics;
    let has_errors = !diagnostics.is_empty();

    match output.value {
        Some(value) if !has_errors => {
            format!("{}", value.bytecode)
        }

        Some(value) => {
            format!(
                "[PARTIAL]\n\n{}\n\nDiagnostics:\n{:#?}\n",
                value.bytecode, diagnostics,
            )
        }

        None => {
            format!("[FAILED]\n\nDiagnostics:\n{:#?}\n", diagnostics,)
        }
    }
}
