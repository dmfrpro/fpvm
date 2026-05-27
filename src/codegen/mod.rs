pub mod bytecode;
pub mod error;
mod expr;
pub mod generator;
pub mod instruction;

mod fmt;

pub use bytecode::{BytecodeFunction, BytecodeProgram};
pub use error::CodegenError;
pub use generator::CodeGenerator;
pub use instruction::Instruction;

mod assignment;
mod brancher;
mod branching;
mod control;
mod functions;
mod identifier;
mod literals;
mod prog;
mod quote;
mod script;
