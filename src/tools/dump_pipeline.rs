use std::fs;
use std::path::{Path, PathBuf};

use compiler::pipeline::pipeline::Pipeline;
use compiler::stages::{lexer_stage, read_from_file, semantic_stage, syntax_stage, codegen_stage};

fn read_input_dir_from_args() -> PathBuf {
    std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .expect("usage: dump_symbol_tables <input-dir>")
}

fn collect_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in
        fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read dir {}: {e}", dir.display()))
    {
        let path = entry
            .unwrap_or_else(|e| panic!("cannot read dir entry: {e}"))
            .path();

        if path.is_file() {
            files.push(path);
        }
    }

    files.sort();
    files
}

fn make_output_path(input_dir: &Path, input_file: &Path, output_dir: &Path) -> PathBuf {
    let relative_path = input_file.strip_prefix(input_dir).unwrap_or(input_file);

    output_dir.join(relative_path).with_extension("output")
}

fn main() {
    let input_dir = &read_input_dir_from_args();
    let output_dir = Path::new("target/symbol-tables");

    fs::create_dir_all(output_dir)
        .unwrap_or_else(|e| panic!("cannot create output dir {}: {e}", output_dir.display()));

    let files = collect_files(input_dir);

    for path in files {
        let output_path = make_output_path(input_dir, &path, output_dir);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("cannot create dir {}: {e}", parent.display()));
        }

        let pipeline = Pipeline::new(read_from_file)
            .then(lexer_stage)
            .then(syntax_stage)
            .then(semantic_stage)
            .then(codegen_stage);
        let output = pipeline.run(&path.clone());
        let mut report = String::new();

        report.push_str(&format!("Input file: {}\n", path.display()));
        report.push_str(&format!("Output file: {}\n", output_path.display()));
        report.push_str("\n========================================\n\n");

        match output.value {
            Some(value) if output.diagnostics.is_empty() => {
                report.push_str("[OK]\n\n");
                report.push_str(&format!("{}\n", value));
            }

            Some(value) => {
                report.push_str("[PARTIAL]\n\n");
                report.push_str(&format!("{}\n", value));

                report.push_str("\nDiagnostics:\n");
                report.push_str(&format!("{:#?}\n", output.diagnostics));
            }

            None => {
                report.push_str("[FAILED]\n\n");
                report.push_str("Diagnostics:\n");
                report.push_str(&format!("{:#?}\n", output.diagnostics));
            }
        }

        fs::write(&output_path, report)
            .unwrap_or_else(|e| panic!("cannot write file {}: {e}", output_path.display()));

        println!("[OK] {} -> {}", path.display(), output_path.display());
    }
}
