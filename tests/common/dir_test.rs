use std::path::{Path, PathBuf};

use crate::common::utils::{collect_files, green, red, yellow};
use compiler::pipeline;
use compiler::stages::StageOutput;

#[derive(Clone, Copy)]
pub enum Expected {
    Failure,
    Success,
}

pub fn test_pipeline_for_dir<F, T>(dir: &Path, mut run_pipeline: F, expected: Expected)
where
    F: FnMut(&PathBuf) -> StageOutput<T>,
{
    let files = collect_files(dir);

    assert!(!files.is_empty(), "no test file found in {}", dir.display());

    let mut failed = Vec::new();
    let mut passed = Vec::new();

    for path in files {
        let output = run_pipeline(&path);

        let is_success = output.value.is_some() && output.diagnostics.is_empty();
        let expected_success = match expected {
            Expected::Success => true,
            Expected::Failure => false,
        };

        // case passed
        if is_success == expected_success {
            println!("{} {}", green("[OK]"), path.display());
            passed.push(path);
        }
        // case failed
        else {
            println!("{} {}", red("[ERR]"), path.display());
            failed.push(format!("{} failed", path.display()));
        }

        if !output.diagnostics.is_empty() {
            // pipeline::types::print_diagnostics(output.diagnostics);
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
    assert!(
        failed.is_empty(),
        "syntax pipeline failed on {} file(s)",
        failed.len()
    );
}
