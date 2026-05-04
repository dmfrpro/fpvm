pub mod codegen;
pub mod lex;
pub mod semantic;
pub mod source_readers;
pub mod syntax;
pub mod types;

pub use crate::pipeline::pipeline::Pipeline;
pub use crate::pipeline::stage::StageOutput;

pub use lex::lexer_stage;
pub use semantic::semantic_stage;
pub use source_readers::{read_from_stdin, read_from_file};
pub use syntax::syntax_stage;
pub use codegen::codegen_stage;