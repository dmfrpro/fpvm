use std::path::{
    Path,
    PathBuf,
};

use super::config::DumpStage;

#[derive(Debug, Clone)]
pub struct DumpReport {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub content: String,
}

pub fn make_report(
    input_file: &Path,
    output_file: &Path,
    stage: DumpStage,
    pipeline_output: &str,
) -> String {
    match stage {
        DumpStage::Lexer => make_default_report(
            input_file,
            output_file,
            stage,
            pipeline_output,
        ),

        DumpStage::Syntax => make_default_report(
            input_file,
            output_file,
            stage,
            pipeline_output,
        ),

        DumpStage::Semantic => make_default_report(
            input_file,
            output_file,
            stage,
            pipeline_output,
        ),

        DumpStage::Codegen => make_codegen_report(
            input_file,
            output_file,
            pipeline_output,
        ),

        DumpStage::Vm => make_vm_report(
            input_file,
            output_file,
            pipeline_output,
        ),
    }
}

fn make_default_report(
    input_file: &Path,
    output_file: &Path,
    stage: DumpStage,
    pipeline_output: &str,
) -> String {
    let mut report = String::new();

    report.push_str(&format!("Stage: {:?}\n", stage));
    report.push_str(&format!("Input file: {}\n", input_file.display()));
    report.push_str(&format!("Output file: {}\n", output_file.display()));
    report.push_str("\n========================================\n\n");
    report.push_str(pipeline_output);

    report
}

fn make_codegen_report(
    _input_file: &Path,
    _output_file: &Path,
    pipeline_output: &str,
) -> String {
    let mut report = String::new();

    report.push_str(pipeline_output);

    report
}

fn make_vm_report(
    input_file: &Path,
    output_file: &Path,
    pipeline_output: &str,
) -> String {
    let mut report = String::new();

    report.push_str("VM output\n");
    report.push_str(&format!("Input file: {}\n", input_file.display()));
    report.push_str(&format!("Output file: {}\n", output_file.display()));
    report.push_str("\n========================================\n\n");
    report.push_str(pipeline_output);

    report
}