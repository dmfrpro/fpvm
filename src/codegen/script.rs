use std::collections::btree_map::Values;

use crate::syntax::node::{Node, NodeKind};

use super::{BytecodeFunction, CodeGenerator, CodegenError, Instruction};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_top_level_script(
        &mut self,
        node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        match &node.kind {
            NodeKind::ProgramNode(inner) | NodeKind::ElementNode(inner) => {
                self.compile_top_level_script(inner, function)
            }

            NodeKind::ElementsNode(elements) => self.compile_script_sequence(elements, function),

            _ => self.compile_script_item(node, function),
        }
    }

    pub(crate) fn compile_prog_script(
        &mut self,
        node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        match &node.kind {
            NodeKind::ElementNode(inner) => self.compile_prog_script(inner, function),

            NodeKind::ListNode(inner) => match &inner.kind {
                NodeKind::ElementsNode(elements) => {
                    self.compile_script_sequence(elements, function)
                }

                _ => Err(CodegenError::InvalidNode {
                    message: "prog body list must contain elements".to_string(),
                }),
            },

            NodeKind::ElementsNode(elements) => self.compile_script_sequence(elements, function),

            _ => self.compile_script_item(node, function),
        }
    }

    fn compile_script_sequence(
        &mut self,
        elements: &[Box<Node>],
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        for element in elements {
            self.compile_script_item(element, function)?;
        }

        Ok(())
    }

    pub(crate) fn compile_script_item(
        &mut self,
        node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        match &node.kind {
            NodeKind::SetqNode(name, value) => self.compile_setq(name, value, function, false),

            NodeKind::FuncNode(_, _, _) => {
                // in script context function declaration doesn't return
                Ok(())
            }

            NodeKind::ReturnNode(value) => self.compile_return(value, function),

            NodeKind::BreakNode => self.compile_break(function),

            NodeKind::WhileNode(cond, body) => {
                self.compile_while(cond, body, function)?;

                // while always returns `null` but in script context didn't output this value
                function.emit(Instruction::Pop);

                Ok(())
            }

            _ => {
                self.compile_expr(node, function)?;
                function.emit(Instruction::Stdout);
                Ok(())
            }
        }
    }
}
