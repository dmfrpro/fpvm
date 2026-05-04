use std::fs;
use std::path::{Path, PathBuf};

use compiler::pipeline::pipeline::Pipeline;
use compiler::pipeline::types::print_diagnostics;
use compiler::stages::StageOutput;
use compiler::stages::lexer_stage;
use compiler::stages::read_from_file;
use compiler::stages::syntax_stage;
use compiler::stages::types::ParsedProgram;

fn collect_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in
        fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read dir {}: {e}", dir.display()))
    {
        let path = entry
            .unwrap_or_else(|e| panic!("cannot read dir entry {e}"))
            .path();

        if path.is_file() {
            files.push(path);
        }
    }

    files.sort();
    files
}

fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

fn yellow(s: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", s)
}

fn run_syntax_pipeline(path: &PathBuf) -> StageOutput<ParsedProgram> {
    let pipeline = Pipeline::new(read_from_file)
        .then(lexer_stage)
        .then(syntax_stage);

    pipeline.run(path)
}


fn dir_test_syntax(dir: &Path) {
    let files = collect_files(dir);

    assert!(!files.is_empty(), "no test file found in {}", dir.display());

    let mut failed = Vec::new();
    let mut passed = Vec::new();

    for path in files {
        let output = run_syntax_pipeline(&path);

        match (output.value, output.diagnostics.is_empty()) {
            (Some(_), true) => {
                println!("{} {}", green("[OK]"), path.display());
                passed.push(path);
            }
            (Some(_), false) => {
                println!("{} {}", yellow("[WARN]"), path.display());
                print_diagnostics(output.diagnostics);
                failed.push(format!("{} parsed with diagnostics", path.display()));
            }
            (None, _) => {
                println!("{} {}", red("[ERR]"), path.display());
                print_diagnostics(output.diagnostics);
                failed.push(format!("{} failed", path.display()));
            }
        }
    }

    println!("Summary:");
    println!("  passed: {}", passed.len());
    println!("  failed: {}", failed.len());

    if !failed.is_empty() {
        println!();
        println!("Failed files:");
        for file in &failed {
            println!("  - {file}");
        }
    }
    assert!(failed.is_empty(), "syntax pipeline failed on {} file(s)", failed.len());
}

#[test]
fn test_code_dir() {
    dir_test_syntax(Path::new("tests/code"));
}

#[test]
fn test_semantic_errors_dir() {
    dir_test_syntax(Path::new("tests/semantic_errors"));
}