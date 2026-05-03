use lalrpop_util::lalrpop_mod;

pub mod lexer;
pub mod semantics;
pub mod syntax;

lalrpop_mod!(pub grammar, "/syntax/grammar.rs");

pub mod pipeline;
pub mod stages;
