use crate::syntax::node::{Node, NodeKind};

use super::{BytecodeFunction, CodeGenerator, CodegenError};

impl<'a> CodeGenerator<'a> {
    pub(crate) fn compile_expr(
        &mut self,
        node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        match &node.kind {
            // part 0
            NodeKind::NullNode => self.compile_null(function),
            NodeKind::BoolNode(value) => self.compile_bool(*value, function),
            NodeKind::IntNode(value) => self.compile_int(*value, function),
            NodeKind::RealNode(value) => self.compile_real(*value, function),
            NodeKind::Identifier(name) => self.compile_identifier(name, function),

            NodeKind::QuoteNode(expr) => self.compile_quote(expr, function),
            NodeKind::SetqNode(name, value) => self.compile_setq(name, value, function),
            NodeKind::FuncNode(name, args, body) => {
                self.compile_func_expr(node, name, args, body, function)
            }
            NodeKind::LambdaNode(args, body) => {
                self.compile_lambda_expr(node, args, body, function)
            }

            // part 1
            NodeKind::ProgNode(locals, body) => {
                self.compile_prog(node, locals, body, function)
            },
            NodeKind::CondNode(c, t, e) => self.compile_cond(c, t, e, function),
            NodeKind::WhileNode(c, b) => self.compile_while(c, b, function),
            NodeKind::ReturnNode(value) => {
                self.compile_expr(value, function)?;
                function.emit(super::Instruction::Ret);
                Ok(())
            }
            NodeKind::BreakNode => {
                if let Some(label) = self.loop_context.peek_brancher() {
                    function.emit(super::Instruction::Jump(label.to_string()));
                    Ok(())
                } else {
                        Err(CodegenError::UnsupportedNode {
                        message: "not implemented".to_string(),
                    })
                }
            }
            NodeKind::ElementNode(subnode) => self.compile_expr(subnode, function),
            NodeKind::ElementsNode(subnodes) => {
                for (index, subnode) in subnodes.iter().enumerate() {
                    self.compile_expr(subnode, function)?;

                    if index + 1 != subnodes.len() {
                        function.emit(super::Instruction::Pop);
                    }
                }
                if (subnodes.is_empty()) {
                    function.emit(super::Instruction::LoadNull);
                }

                Ok(())
            }
            NodeKind::ListNode(exprs) => {
                // Function call
                let subnodes = match &exprs.kind {
                    NodeKind::ElementsNode(sub) => sub,
                    _ => return Err(CodegenError::InternalError { message: "Unexpected node type".to_string() }),
                };
                self.compile_func_call(subnodes, function)
            }
            NodeKind::ProgramNode(exprs) => self.compile_expr(exprs, function),

            NodeKind::ErrorNode => Err(CodegenError::InvalidNode {
                message: "cannot generate code for ErrorNode".to_string(),
            }),
        }
    }
}
