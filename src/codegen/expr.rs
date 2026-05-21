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
            NodeKind::ProgNode(_, _) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),
            NodeKind::CondNode(_, _, _) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),
            NodeKind::WhileNode(_, _) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),
            NodeKind::ReturnNode(_) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),
            NodeKind::BreakNode => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),

            NodeKind::ElementNode(_) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),
            NodeKind::ElementsNode(_) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),
            NodeKind::ListNode(_) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),
            NodeKind::ProgramNode(_) => Err(CodegenError::UnsupportedNode {
                message: "not implemented".to_string(),
            }),

            NodeKind::ErrorNode => Err(CodegenError::InvalidNode {
                message: "cannot generate code for ErrorNode".to_string(),
            }),
        }
    }
}
