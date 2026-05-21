use crate::syntax::node::Node;

use super::{BytecodeFunction, CodegenError, CodeGenerator};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_quote(
        &mut self,
        quoted: &Node,
        _function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        Err(CodegenError::UnsupportedNode {
            message: format!("quote codegen is not implemented yet: {:?}", quoted.kind),
        })
    }
}