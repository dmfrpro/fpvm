use std::fs;

use super::config::DumpConfig;
use super::files::{collect_input_files, make_output_path};
use super::pipelines::run_stage;
use super::report::{DumpReport, make_report};

pub fn run_to_reports(config: &DumpConfig) -> Vec<DumpReport> {
    let files = collect_input_files(&config.input_path);

    files
        .into_iter()
        .map(|input_file| {
            let output_path = make_output_path(&config.input_path, &input_file, &config.output_dir);

            let pipeline_output = run_stage(&input_file, config.stage);

            let content = make_report(&input_file, &output_path, config.stage, &pipeline_output);

            DumpReport {
                input_path: input_file,
                output_path,
                content,
            }
        })
        .collect()
}

pub fn run_and_write(config: &DumpConfig) {
    let reports = run_to_reports(config);

    for report in reports {
        if let Some(parent) = report.output_path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|e| panic!("cannot create dir {}: {e}", parent.display()));
        }

        fs::write(&report.output_path, &report.content)
            .unwrap_or_else(|e| panic!("cannot write file {}: {e}", report.output_path.display()));

        println!(
            "[OK] {} -> {}",
            report.input_path.display(),
            report.output_path.display(),
        );
    }
}
