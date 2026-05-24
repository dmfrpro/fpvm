use crate::syntax::node::Node;

use super::{BytecodeFunction, CodeGenerator, CodegenError, Instruction};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_cond(
        &mut self,
        cond: &Node,
        then_: &Node,
        else_: &Option<Box<Node>>,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let brancher = self
            .loop_context
            .new_brancher(super::brancher::BranchType::Cond);

        // labels
        let then_label = brancher.get_label("then".to_string());
        let end_label = brancher.get_label("end".to_string());

        // Cond
        self.compile_expr(cond, function)?;
        function.emit(Instruction::CondJump(then_label.clone()));

        // Else
        if let Some(else_expr) = else_ {
            self.compile_expr(else_expr, function)?;
        } else {
            function.emit(Instruction::LoadNull);
        }

        function.emit(Instruction::Jump(end_label.clone()));

        // Then
        function.emit(Instruction::Label(then_label));
        self.compile_expr(then_, function)?;

        function.emit(Instruction::Label(end_label));

        Ok(())
    }

    pub(crate) fn compile_while(
        &mut self,
        cond: &Node,
        body: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let brancher = self
            .loop_context
            .new_brancher(super::brancher::BranchType::While);

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
        function.emit(Instruction::Pop);
        self.loop_context.pop_brancher();

        function.emit(super::Instruction::Jump(cond_label));

        function.emit(super::Instruction::Label(end_label));
        function.emit(super::Instruction::LoadNull);

        Ok(())
    }
}
