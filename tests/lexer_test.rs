use std::path::{Path, PathBuf};

mod common;
use common::{test_pipeline_for_dir, Expected};

use compiler::lexer::Token;
use compiler::pipeline::pipeline::Pipeline;
use compiler::stages::StageOutput;
use compiler::stages::{lexer_stage, read_from_file};

fn run_lexer_pipeline(path: &PathBuf) -> StageOutput<Vec<Token>> {
    let pipeline = Pipeline::new(read_from_file)
        .then(lexer_stage);

    pipeline.run(path)
}

#[test]
fn test_code_dir() {
    test_pipeline_for_dir(Path::new("tests/code"), run_lexer_pipeline, Expected::Success);
}

#[test]
fn test_semantic_errors_dir() {
    test_pipeline_for_dir(Path::new("tests/semantic_errors"), run_lexer_pipeline, Expected::Success);
}
