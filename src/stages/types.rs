use crate::codegen::BytecodeProgram;
use crate::symbol_table::SymbolTable;
use crate::syntax::node::Node;
use std::fmt;

pub struct Source {
    pub text: String,
}

impl Source {
    pub fn new(text: String) -> Self {
        return Self { text };
    }
}

#[derive(Debug)]
pub struct ParsedProgram {
    pub ast: Node,
}

#[derive(Debug)]
pub struct CheckedProgram {
    pub ast: Node,
    pub symbol_table: SymbolTable,
}

pub struct GeneratedProg {
    pub ast: Node,
    pub bytecode: BytecodeProgram,
}

impl fmt::Display for ParsedProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ParsedProgram:\nAST:{}", self.ast)?;
        Ok(())
    }
}

impl fmt::Display for CheckedProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "------- CheckedProgram -------")?;
        writeln!(f, "AST: {}", self.ast)?;
        writeln!(f, "table: {}", self.symbol_table)?;
        Ok(())
    }
}

impl fmt::Display for GeneratedProg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "------- CheckedProgram -------")?;
        writeln!(f, "AST: {}", self.ast)?;
        writeln!(f, "Bytecode:\n{}", self.bytecode.to_string())?;
        Ok(())
    }
}
