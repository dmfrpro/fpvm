use lalrpop_util::lalrpop_mod;

pub mod codegen;
pub mod lexer;
pub mod pipeline;
pub mod semantics;
pub mod stages;
pub mod symbol_table;
pub mod syntax;
pub mod tooling;
pub mod vm;

lalrpop_mod!(pub grammar, "/syntax/grammar.rs");
