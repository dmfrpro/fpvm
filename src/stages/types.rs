use std::fmt;
use crate::syntax::node::Node;

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
}

impl fmt::Display for ParsedProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ParsedProgram:\nAST:{}", self.ast)?;
        Ok(())
    }
}

impl fmt::Display for CheckedProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CheckedProgram:\nAST:{}", self.ast)?;
        Ok(())
    }
}