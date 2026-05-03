use crate::lexer::{Lexer, Token};
use crate::pipeline::stage::StageOutput;
use crate::stages::types::Source;

pub fn lexer_stage(source: Source) -> StageOutput<Vec<Token>> {
    let mut lx = Lexer::new(source.text);

    StageOutput::ok(lx.collect_tokens())
}
