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

mod literals;
mod identifier;
mod assignment;
mod functions;
mod quote;
mod branching;
mod brancher;