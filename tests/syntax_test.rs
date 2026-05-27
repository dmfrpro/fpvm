use std::path::{Path, PathBuf};

mod common;
use common::{Expected, test_pipeline_for_dir};

use compiler::pipeline::pipeline::Pipeline;
use compiler::stages::StageOutput;
use compiler::stages::lexer_stage;
use compiler::stages::read_from_file;
use compiler::stages::syntax_stage;
use compiler::stages::types::ParsedProgram;

fn run_syntax_pipeline(path: &PathBuf) -> StageOutput<ParsedProgram> {
    let pipeline = Pipeline::new(read_from_file)
        .then(lexer_stage)
        .then(syntax_stage);

    pipeline.run(path)
}

#[test]
fn test_code_dir() {
    test_pipeline_for_dir(
        Path::new("tests/code"),
        run_syntax_pipeline,
        Expected::Success,
    );
}

#[test]
fn test_semantic_errors_dir() {
    test_pipeline_for_dir(
        Path::new("tests/semantic_errors"),
        run_syntax_pipeline,
        Expected::Success,
    );
}
