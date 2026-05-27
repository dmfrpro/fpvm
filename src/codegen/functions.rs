use crate::symbol_table::SymbolKind;
use crate::syntax::node::{Node, NodeKind};

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

    pub(crate) fn compile_func_call(
        &mut self,
        nodes: &[Box<Node>],
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        if nodes.is_empty() {
            return Err(CodegenError::InvalidNode {
                message: "empty function call".to_string(),
            });
        }

        // Single-element list with a literal: (null), (5), (true) → just the literal
        // Single-element list with a non-function identifier: (x) → just the value
        if nodes.len() == 1 {
            match &nodes[0].kind {
                NodeKind::NullNode
                | NodeKind::BoolNode(_)
                | NodeKind::IntNode(_)
                | NodeKind::RealNode(_)
                | NodeKind::QuoteNode(_) => {
                    return self.compile_expr(&nodes[0], function);
                }
                NodeKind::Identifier(name) => {
                    // If it's not a known function, just return the value
                    // instead of trying to call it dynamically
                    if let Ok(symbol_id) = self.lookup_symbol(name) {
                        if let Some(symbol) = self.symbol_table.symbol(symbol_id) {
                            if symbol.kind != SymbolKind::Function {
                                return self.compile_expr(&nodes[0], function);
                            }
                        } else {
                            return self.compile_expr(&nodes[0], function);
                        }
                    } else {
                        return self.compile_expr(&nodes[0], function);
                    }
                }
                _ => {}
            }
        }

        let callee = &nodes[0];
        let args = &nodes[1..];
        let argc = args.len();

        if let NodeKind::Identifier(name) = &callee.kind {
            if name == "eval" {
                if argc != 1 {
                    return Err(CodegenError::InvalidNode {
                        message: format!("eval expects 1 argument, got {}", argc),
                    });
                }

                self.compile_eval_expr(&args[0], function)?;
                return Ok(());
            }

            if let Some(instruction) = Self::builtin_instruction(name, argc)? {
                for arg in args {
                    self.compile_expr(arg, function)?;
                }

                function.emit(instruction);
                return Ok(());
            }

            let symbol_id = self.lookup_symbol(name)?;
            let symbol =
                self.symbol_table
                    .symbol(symbol_id)
                    .ok_or_else(|| CodegenError::InternalError {
                        message: format!("missing symbol #{}", symbol_id),
                    })?;

            if symbol.kind == SymbolKind::Function {
                let label = symbol.label.clone();

                for arg in args {
                    self.compile_expr(arg, function)?;
                }

                function.emit(Instruction::Call(label));
                return Ok(());
            }
        }

        self.compile_expr(callee, function)?;

        for arg in args {
            self.compile_expr(arg, function)?;
        }

        function.emit(Instruction::CallStack { argc });

        Ok(())
    }

    fn builtin_instruction(name: &str, argc: usize) -> Result<Option<Instruction>, CodegenError> {
        let (instruction, expected_argc) = match name {
            "plus" => (Instruction::Add, 2),
            "minus" => (Instruction::Sub, 2),
            "times" => (Instruction::Mul, 2),
            "divide" => (Instruction::Div, 2),
            "mod" => (Instruction::Mod, 2),

            "equal" => (Instruction::Eq, 2),
            "nonequal" => (Instruction::Neq, 2),
            "less" => (Instruction::Less, 2),
            "lesseq" => (Instruction::Leq, 2),
            "greater" => (Instruction::Greater, 2),
            "greatereq" => (Instruction::Geq, 2),

            "head" => (Instruction::Head, 1),
            "tail" => (Instruction::Tail, 1),
            "cons" => (Instruction::Cons, 2),
            "isnull" => (Instruction::IsNull, 1),
            "length" => (Instruction::Length, 1),
            "or" => (Instruction::Or, 2),
            "not" => (Instruction::Not, 1),
            "islist" => (Instruction::IsList, 1),

            _ => return Ok(None),
        };

        if argc != expected_argc {
            return Err(CodegenError::InvalidNode {
                message: format!(
                    "builtin '{}' expects {} arguments, got {}",
                    name, expected_argc, argc
                ),
            });
        }

        Ok(Some(instruction))
    }

    pub(crate) fn compile_eval_expr(
        &mut self,
        arg: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        match &arg.kind {
            NodeKind::NullNode
            | NodeKind::BoolNode(_)
            | NodeKind::IntNode(_)
            | NodeKind::RealNode(_)
            | NodeKind::Identifier(_)
            | NodeKind::ListNode(_) => self.compile_expr(arg, function)?,

            NodeKind::ElementNode(inner) => self.compile_eval_expr(inner, function)?,

            _ => self.compile_expr(arg, function)?,
        }

        function.emit(Instruction::Eval);
        Ok(())
    }
}
