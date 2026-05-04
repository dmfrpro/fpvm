use std::path::{Path, PathBuf};

mod common;
use common::{test_pipeline_for_dir, Expected};

use compiler::pipeline::pipeline::Pipeline;
use compiler::stages::StageOutput;
use compiler::stages::types::CheckedProgram;
use compiler::stages::{read_from_file, lexer_stage, syntax_stage, semantic_stage};

fn run_semantic_pipeline(path: &PathBuf) -> StageOutput<CheckedProgram> {
    let pipeline = Pipeline::new(read_from_file)
        .then(lexer_stage)
        .then(syntax_stage)
        .then(semantic_stage);

    pipeline.run(path)
}

#[test]
fn test_code_dir() {
    test_pipeline_for_dir(Path::new("tests/code"), run_semantic_pipeline, Expected::Success);
}

#[test]
fn test_semantic_errors_dir() {
    test_pipeline_for_dir(Path::new("tests/semantic_errors"), run_semantic_pipeline, Expected::Failure);
}
