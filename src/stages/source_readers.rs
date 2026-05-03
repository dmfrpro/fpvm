use crate::stages::{StageOutput, types::Source};

use std::io::{self, Read};

pub fn read_from_stdin(_: ()) -> StageOutput<Source> {
    let mut src = String::new();

    if let Err(e) = io::stdin().read_to_string(&mut src) {
        return StageOutput::error(Vec::new());
        // Diagnostic::new format!("Failed to read stdin: {}", e)]),
    }

    StageOutput::ok(Source::new(src))
}
