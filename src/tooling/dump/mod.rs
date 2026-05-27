pub mod config;

mod files;
mod pipelines;
mod report;
mod runner;

pub use config::{DumpConfig, DumpStage, parse_args};

pub use report::DumpReport;

pub use runner::{run_and_write, run_to_reports};
