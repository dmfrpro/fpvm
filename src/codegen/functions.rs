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

        let callee = &nodes[0];
        let args = &nodes[1..];
        let argc = args.len();

        if let NodeKind::Identifier(name) = &callee.kind {
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
        let instruction = match name {
            "plus" => Instruction::Add,
            "minus" => Instruction::Sub,
            "times" => Instruction::Mul,
            "divide" => Instruction::Div,
            "mod" => Instruction::Mod,

            "equal" => Instruction::Eq,
            "nonequal" => Instruction::Neq,
            "less" => Instruction::Less,
            "lesseq" => Instruction::Leq,
            "greater" => Instruction::Greater,
            "greatereq" => Instruction::Geq,

            // Можно оставить как алиасы, если в тестах встречаются имена инструкций.
            "add" => Instruction::Add,
            "sub" => Instruction::Sub,
            "mul" => Instruction::Mul,
            "div" => Instruction::Div,
            "eq" => Instruction::Eq,
            "neq" => Instruction::Neq,
            "leq" => Instruction::Leq,
            "geq" => Instruction::Geq,

            _ => return Ok(None),
        };

        if argc != 2 {
            return Err(CodegenError::InvalidNode {
                message: format!("builtin '{}' expects 2 arguments, got {}", name, argc),
            });
        }

        Ok(Some(instruction))
    }

    pub(crate) fn compile_prog(
        &mut self,
        prog_node: &Node,
        _locals_node: &Node,
        body_node: &Node,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let previous_scope = self.current_scope_id;

        let prog_scope =
            self.find_scope_by_owner_span(prog_node, crate::symbol_table::ScopeKind::Prog)?;

        self.current_scope_id = Some(prog_scope);

        let result = self.compile_expr(body_node, function);

        self.current_scope_id = previous_scope;

        result
    }
}
