use crate::{
    pipeline::types::Diagnostic,
    stages::{StageOutput, types::Source},
};

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};

pub fn read_from_stdin(_: ()) -> StageOutput<Source> {
    let mut src = String::new();

    if let Err(e) = io::stdin().read_to_string(&mut src) {
        let l = String::from(format!("Failed to read stdin: {}", e));
        return StageOutput::error(vec![Diagnostic::error(l)]);
    }

    println!("Source code: {}", src);
    StageOutput::ok(Source::new(src))
}

pub fn read_from_file(path: &PathBuf) -> StageOutput<Source> {
    let src = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            return StageOutput::error(vec![Diagnostic::error(format!(
                "failed to read file {}: {}",
                path.display(),
                e
            ))]);
        }
    };

    StageOutput::ok(Source::new(src))
}
