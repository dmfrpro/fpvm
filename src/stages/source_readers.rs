use crate::{
    pipeline::types::Diagnostic,
    stages::{StageOutput, types::Source},
};

use std::io::{self, Read};

pub fn read_from_stdin(_: ()) -> StageOutput<Source> {
    let mut src = String::new();

    if let Err(e) = io::stdin().read_to_string(&mut src) {
        let l = String::from(format!("Failed to read stdin: {}", e));
        return StageOutput::error(vec![Diagnostic::error(l)]);
    }

    StageOutput::ok(Source::new(src))
}
