use crate::syntax::node::Node;

use super::{BytecodeFunction, CodeGenerator, CodegenError, Instruction};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_func_expr(
        &mut self,
        func_node: &Node,
        _name_node: &Node,
        _args_node: &Node,
        _body_node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let function_info = self.find_function_by_owner_span(func_node)?;

        function.emit(Instruction::LoadFunc(function_info.label.clone()));

        Ok(())
    }

    pub(crate) fn compile_lambda_expr(
        &mut self,
        lambda_node: &Node,
        _args_node: &Node,
        _body_node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let function_info = self.find_function_by_owner_span(lambda_node)?;

        function.emit(Instruction::LoadFunc(function_info.label.clone()));

        Ok(())
    }

    pub(crate) fn compile_func_call(&mut self, nodes: &Vec<Box<Node>>, function: &mut BytecodeFunction) -> Result<(), CodegenError> {
        // Function call
        let function_to_call = &nodes[0];

        for arg in nodes[1..].iter() {
            self.compile_expr(arg, function)?;
        }

        let function_info = self.find_function_by_owner_span(function_to_call)?;
        function.emit(Instruction::LoadFunc(function_info.label.clone()));
        function.emit(Instruction::CallStack { argc: nodes.len() - 1 });
        Ok(())
    }
}
