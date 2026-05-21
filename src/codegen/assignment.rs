use crate::syntax::node::{Node, NodeKind};

use super::{BytecodeFunction, CodeGenerator, CodegenError};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_setq(
        &mut self,
        name_node: &Node,
        value_node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let NodeKind::Identifier(name) = &name_node.kind else {
            return Err(CodegenError::InvalidNode {
                message: "setq target must be identifier".to_string(),
            });
        };

        self.compile_expr(value_node, function)?;

        let symbol_id = self.lookup_symbol(name)?;

        self.emit_set_symbol(symbol_id, function)?;

        // set* consumes the value.
        // setq is an expression, so it must leave assigned value on the stack.
        self.emit_load_symbol(symbol_id, function)?;

        Ok(())
    }
}
