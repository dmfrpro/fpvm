use super::generator::ReturnTarget;
use super::{BytecodeFunction, CodeGenerator, CodegenError};
use crate::syntax::node::Node;

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_return(
        &mut self,
        value: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        self.compile_expr(value, function)?;

        match self.return_targets.last() {
            Some(ReturnTarget::Function) => {
                function.emit(super::Instruction::Ret);
            }

            Some(ReturnTarget::Prog(end_label)) => {
                function.emit(super::Instruction::Jump(end_label.clone()));
            }

            None => {
                return Err(CodegenError::InternalError {
                    message: "return target is missing".to_string(),
                });
            }
        }

        Ok(())
    }

    pub(crate) fn compile_break(
        &mut self,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        if let Some(label) = self.loop_context.peek_brancher() {
            function.emit(super::Instruction::Jump(label.to_string()));
            Ok(())
        } else {
            Err(CodegenError::InvalidNode {
                message: "break is not allowed outside while".to_string(),
            })
        }
    }
}
