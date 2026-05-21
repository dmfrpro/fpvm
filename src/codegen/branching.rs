use crate::syntax::node::Node;

use super::{BytecodeFunction, CodegenError, CodeGenerator};


impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_cond(
        &mut self,
        cond: &Node,
        then_: &Node,
        else_: &Option<Box<Node>>,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let brancher = self.loop_context.new_brancher(super::brancher::BranchType::Cond);
        
        // labels
        let then_label = brancher.get_label("then".to_string());
        let else_label = brancher.get_label("else".to_string());
        let end_label = brancher.get_label("end".to_string());

        // Cond
        self.compile_expr(cond, function)?;
        function.emit(super::Instruction::CondJump(then_label.clone()));

        let to_emit = if else_.is_some() {
            &else_label
        } else {
            &end_label
        };
        function.emit(super::Instruction::Jump(to_emit.to_string()));

        // Then
        function.emit(super::Instruction::Label(then_label.clone()));
        self.compile_expr(then_, function)?;

        if let Some(else_expr) = else_ {
            function.emit(super::Instruction::Jump(end_label.clone()));

            // Else
            function.emit(super::Instruction::Label(else_label.clone()));
            self.compile_expr(else_expr, function)?;
        }
        function.emit(super::Instruction::Label(end_label.clone()));

        Ok(())    
    }

    pub(crate) fn compile_while(
        &mut self,
        cond: &Node,
        body: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let brancher = self.loop_context.new_brancher(super::brancher::BranchType::While);
        
        // labels
        let cond_label = brancher.get_label("cond".to_string());
        let body_label = brancher.get_label("body".to_string());
        let end_label = brancher.get_label("end".to_string());

        // Cond
        function.emit(super::Instruction::Label(cond_label.clone()));
        self.compile_expr(cond, function)?;
        function.emit(super::Instruction::CondJump(body_label.clone()));
        function.emit(super::Instruction::Jump(end_label.to_string()));

        // Body
        function.emit(super::Instruction::Label(body_label.clone()));
        
        self.loop_context.push_brancher(end_label.clone());
        self.compile_expr(body, function)?;
        self.loop_context.pop_brancher();

        function.emit(super::Instruction::Label(end_label.clone()));

        Ok(())
    }
}