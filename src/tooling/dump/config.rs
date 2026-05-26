use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum DumpStage {
    Lexer,
    Syntax,
    Semantic,
    Codegen,
    Vm,
}

impl DumpStage {
    fn from_flag(flag: &str) -> Self {
        match flag {
            "--lexer" => DumpStage::Lexer,
            "--syntax" => DumpStage::Syntax,
            "--semantic" => DumpStage::Semantic,
            "--codegen" => DumpStage::Codegen,
            "--vm" => DumpStage::Vm,
            _ => panic!("unknown option: {flag}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DumpConfig {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub stage: DumpStage,
}

pub fn parse_args() -> DumpConfig {
    let mut input_path = None;
    let mut output_dir = None;
    let mut stage = DumpStage::Codegen;

    for arg in std::env::args().skip(1) {
        if arg.starts_with("--") {
            stage = DumpStage::from_flag(&arg);
            continue;
        }

        if input_path.is_none() {
            input_path = Some(PathBuf::from(arg));
            continue;
        }

        if output_dir.is_none() {
            output_dir = Some(PathBuf::from(arg));
            continue;
        }

        panic!("{}", usage());
    }

    let input_path = input_path.unwrap_or_else(|| {
        panic!("{}", usage());
    });

    let output_dir = output_dir.unwrap_or_else(|| PathBuf::from("target/output"));

    DumpConfig {
        input_path,
        output_dir,
        stage,
    }
}

fn usage() -> &'static str {
    "usage: dump_tool <input-file-or-dir> <output-dir=target/output> \
     [--lexer|--syntax|--semantic|--codegen|--vm]"
}