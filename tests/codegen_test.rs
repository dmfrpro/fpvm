use std::fs;
use std::path::{
    Path,
    PathBuf,
};

use compiler::tooling::dump::{
    run_to_reports,
    DumpConfig,
    DumpStage,
};

fn normalize(text: &str) -> String {
    text.replace("\r\n", "\n")
        .trim_end()
        .to_string()
}

fn assert_codegen_golden(input_dir: &str, expected_dir: &str) {
    let input_path = PathBuf::from(input_dir);
    let expected_dir = PathBuf::from(expected_dir);

    let config = DumpConfig {
        input_path: input_path.clone(),
        output_dir: PathBuf::from("target/codegen-golden-output"),
        stage: DumpStage::Codegen,
    };

    let reports = run_to_reports(&config);

    assert!(
        !reports.is_empty(),
        "no input files found in {}",
        input_path.display(),
    );

    for report in reports {
        let relative_path = report
            .input_path
            .strip_prefix(&input_path)
            .unwrap_or(&report.input_path);

        let expected_path = expected_dir
            .join(relative_path)
            .with_extension("output");

        let expected = fs::read_to_string(&expected_path)
            .unwrap_or_else(|e| {
                panic!(
                    "cannot read expected file {}: {e}",
                    expected_path.display(),
                )
            });

        assert_eq!(
            normalize(&expected),
            normalize(&report.content),
            "golden mismatch for {}",
            report.input_path.display(),
        );
    }
}

#[test]
fn codegen_golden_branching() {
assert_codegen_golden(
        "tests/codegen/code/branching",
        "tests/codegen/golden/branching",
    );
}


#[test]
fn codegen_golden_eval() {
    assert_codegen_golden(
        "tests/codegen/code/eval",
        "tests/codegen/golden/eval",
    );
}

#[test]
fn codegen_golden_func_lambda() {
    assert_codegen_golden(
        "tests/codegen/code/func_lambda",
        "tests/codegen/golden/func_lambda",
    );
}

#[test]
fn codegen_golden_list() {
    assert_codegen_golden(
        "tests/codegen/code/list",
        "tests/codegen/golden/list",
    );
}

#[test]
fn codegen_golden_literals() {
    assert_codegen_golden(
        "tests/codegen/code/literals",
        "tests/codegen/golden/literals",
    );
}

#[test]
fn codegen_golden_prog() {
    assert_codegen_golden(
        "tests/codegen/code/prog",
        "tests/codegen/golden/prog",
    );
}

#[test]
fn codegen_golden_quote() {
    assert_codegen_golden(
        "tests/codegen/code/quote",
        "tests/codegen/golden/quote",
    );
}

#[test]
fn codegen_golden_setq() {
    assert_codegen_golden(
        "tests/codegen/code/setq",
        "tests/codegen/golden/setq",
    );
}

#[test]
fn codegen_golden_misc() {
    assert_codegen_golden(
        "tests/codegen/code",
        "tests/codegen/golden",
    );
}