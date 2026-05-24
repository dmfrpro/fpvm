use crate::codegen::brancher::BrancherStack;
use crate::symbol_table::{
    FunctionId, FunctionInfo, FunctionKind, ScopeId, ScopeKind, SymbolId, SymbolKind, SymbolTable,
};
use crate::syntax::node::{MultilinePosition, Node, NodeKind};

use super::{BytecodeFunction, BytecodeProgram, CodegenError, Instruction};

#[derive(Debug, Clone)]
pub(crate) enum ReturnTarget {
    Function,
    Prog(String),
}

pub struct CodeGenerator<'a> {
    pub(crate) symbol_table: &'a SymbolTable,

    pub(crate) current_function_id: Option<FunctionId>,
    pub(crate) current_scope_id: Option<ScopeId>,

    pub(crate) loop_context: BrancherStack,

    pub(crate) return_targets: Vec<ReturnTarget>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            current_function_id: None,
            current_scope_id: None,
            loop_context: BrancherStack {
                index: 0,
                loop_branches: vec![],
            },
            return_targets: Vec::new(),
        }
    }

    pub fn generate(&mut self, ast: &Node) -> Result<BytecodeProgram, CodegenError> {
        let entry_function = self
            .symbol_table
            .function(self.symbol_table.entry_function_id)
            .ok_or_else(|| CodegenError::InternalError {
                message: "missing entry function".to_string(),
            })?;

        let mut program = BytecodeProgram::new(entry_function.label.clone());

        self.generate_globals(&mut program)?;

        for function_info in self.symbol_table.functions() {
            let function = self.generate_function(function_info, ast)?;
            program.add_function(function);
        }

        Ok(program)
    }

    fn generate_globals(&self, program: &mut BytecodeProgram) -> Result<(), CodegenError> {
        for symbol in self.symbol_table.symbols() {
            if symbol.kind == SymbolKind::Global {
                program.add_global(symbol.label.clone());
            }
        }

        Ok(())
    }

    fn generate_function(
        &mut self,
        function_info: &FunctionInfo,
        ast: &Node,
    ) -> Result<BytecodeFunction, CodegenError> {
        let mut function = self.generate_function_header(function_info)?;

        let previous_function = self.current_function_id;
        let previous_scope = self.current_scope_id;

        self.current_function_id = Some(function_info.id);
        self.current_scope_id = Some(function_info.scope_id);

        match function_info.kind {
            FunctionKind::TopLevel => {
                self.return_targets.push(ReturnTarget::Function);

                self.compile_top_level_script(ast, &mut function)?;

                // top level returns null
                function.emit(Instruction::LoadNull);

                self.return_targets.pop();
            }

            FunctionKind::Named | FunctionKind::Lambda => {
                self.return_targets.push(ReturnTarget::Function);

                let owner_node = self.find_function_owner_node(ast, function_info)?;
                let body_node = self.function_body_node(owner_node)?;

                self.compile_expr(body_node, &mut function)?;

                self.return_targets.pop();
            }
        }

        if !matches!(function.body.last(), Some(Instruction::Ret)) {
            function.emit(Instruction::Ret);
        }

        self.current_function_id = previous_function;
        self.current_scope_id = previous_scope;

        Ok(function)
    }

    fn generate_function_header(
        &self,
        function: &FunctionInfo,
    ) -> Result<BytecodeFunction, CodegenError> {
        let mut bytecode_function = BytecodeFunction::new(function.label.clone());

        for capture in &function.captures {
            let symbol = self.symbol_table.symbol(capture.symbol_id).ok_or_else(|| {
                CodegenError::InternalError {
                    message: format!("missing captured symbol #{}", capture.symbol_id),
                }
            })?;

            bytecode_function.captures.push(symbol.label.clone());
        }

        for arg_id in &function.args {
            let symbol =
                self.symbol_table
                    .symbol(*arg_id)
                    .ok_or_else(|| CodegenError::InternalError {
                        message: format!("missing arg symbol #{}", arg_id),
                    })?;

            bytecode_function.args.push(symbol.label.clone());
        }

        for local_id in &function.locals {
            let symbol =
                self.symbol_table
                    .symbol(*local_id)
                    .ok_or_else(|| CodegenError::InternalError {
                        message: format!("missing local symbol #{}", local_id),
                    })?;

            bytecode_function.locals.push(symbol.label.clone());
        }

        Ok(bytecode_function)
    }

    pub(crate) fn current_scope(&self) -> Result<ScopeId, CodegenError> {
        self.current_scope_id
            .ok_or_else(|| CodegenError::InternalError {
                message: "missing current scope".to_string(),
            })
    }

    pub(crate) fn current_function(&self) -> Result<FunctionId, CodegenError> {
        self.current_function_id
            .ok_or_else(|| CodegenError::InternalError {
                message: "missing current function".to_string(),
            })
    }

    pub(crate) fn is_top_level(&self) -> bool {
        self.current_function_id == Some(self.symbol_table.entry_function_id)
    }

    pub(crate) fn find_function_by_owner_span(
        &self,
        node: &Node,
    ) -> Result<&FunctionInfo, CodegenError> {
        self.symbol_table
            .functions()
            .iter()
            .find(|function| {
                function
                    .owner_span
                    .as_ref()
                    .is_some_and(|span| Self::same_span(span, &node.span))
            })
            .ok_or_else(|| CodegenError::InternalError {
                message: format!("function info not found for node span {}", node.span),
            })
    }

    pub(crate) fn find_scope_by_owner_span(
        &self,
        node: &Node,
        kind: ScopeKind,
    ) -> Result<ScopeId, CodegenError> {
        self.symbol_table
            .scopes()
            .iter()
            .find(|scope| {
                scope.kind == kind
                    && scope
                        .owner_span
                        .as_ref()
                        .is_some_and(|span| Self::same_span(span, &node.span))
            })
            .map(|scope| scope.id)
            .ok_or_else(|| CodegenError::InternalError {
                message: format!("scope info not found for node span {}", node.span),
            })
    }

    pub(crate) fn emit_load_symbol(
        &self,
        symbol_id: SymbolId,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let current_function = self.current_function()?;

        let symbol =
            self.symbol_table
                .symbol(symbol_id)
                .ok_or_else(|| CodegenError::InternalError {
                    message: format!("missing symbol #{}", symbol_id),
                })?;

        match symbol.kind {
            SymbolKind::Global => {
                function.emit(Instruction::LoadGlobal(symbol.label.clone()));
            }

            SymbolKind::Argument => {
                if symbol.function_id == Some(current_function) {
                    function.emit(Instruction::LoadArg(symbol.label.clone()));
                } else {
                    function.emit(Instruction::LoadCapture(symbol.label.clone()));
                }
            }

            SymbolKind::Local => {
                if symbol.function_id == Some(current_function) {
                    function.emit(Instruction::LoadLocal(symbol.label.clone()));
                } else {
                    function.emit(Instruction::LoadCapture(symbol.label.clone()));
                }
            }

            SymbolKind::Function => {
                function.emit(Instruction::LoadFunc(symbol.label.clone()));
            }
        }

        Ok(())
    }

    pub(crate) fn emit_set_symbol(
        &self,
        symbol_id: SymbolId,
        function: &mut BytecodeFunction,
    ) -> Result<(), CodegenError> {
        let current_function = self.current_function()?;

        let symbol =
            self.symbol_table
                .symbol(symbol_id)
                .ok_or_else(|| CodegenError::InternalError {
                    message: format!("missing symbol #{}", symbol_id),
                })?;

        match symbol.kind {
            SymbolKind::Global => {
                function.emit(Instruction::SetGlobal(symbol.label.clone()));
            }

            SymbolKind::Argument => {
                if symbol.function_id == Some(current_function) {
                    function.emit(Instruction::SetArg(symbol.label.clone()));
                } else {
                    function.emit(Instruction::SetCapture(symbol.label.clone()));
                }
            }

            SymbolKind::Local => {
                if symbol.function_id == Some(current_function) {
                    function.emit(Instruction::SetLocal(symbol.label.clone()));
                } else {
                    function.emit(Instruction::SetCapture(symbol.label.clone()));
                }
            }

            SymbolKind::Function => {
                return Err(CodegenError::InvalidNode {
                    message: format!("cannot assign to function symbol '{}'", symbol.name),
                });
            }
        }

        Ok(())
    }

    pub(crate) fn lookup_symbol(&self, name: &str) -> Result<SymbolId, CodegenError> {
        let current_scope = self.current_scope()?;

        self.symbol_table
            .lookup(current_scope, name)
            .ok_or_else(|| CodegenError::MissingSymbol {
                name: name.to_string(),
            })
    }

    fn find_function_owner_node<'b>(
        &self,
        root: &'b Node,
        function: &FunctionInfo,
    ) -> Result<&'b Node, CodegenError> {
        let Some(owner_span) = &function.owner_span else {
            return Err(CodegenError::InternalError {
                message: format!("function '{}' has no owner span", function.label),
            });
        };

        Self::find_function_node_by_span(root, owner_span).ok_or_else(|| {
            CodegenError::InternalError {
                message: format!(
                    "AST function node not found for function '{}'",
                    function.label
                ),
            }
        })
    }

    fn find_function_node_by_span<'b>(
        node: &'b Node,
        span: &MultilinePosition,
    ) -> Option<&'b Node> {
        match &node.kind {
            NodeKind::FuncNode(name, args, body) => {
                if Self::same_span(&node.span, span) {
                    return Some(node);
                }

                Self::find_function_node_by_span(name, span)
                    .or_else(|| Self::find_function_node_by_span(args, span))
                    .or_else(|| Self::find_function_node_by_span(body, span))
            }

            NodeKind::LambdaNode(args, body) => {
                if Self::same_span(&node.span, span) {
                    return Some(node);
                }

                Self::find_function_node_by_span(args, span)
                    .or_else(|| Self::find_function_node_by_span(body, span))
            }

            NodeKind::QuoteNode(inner)
            | NodeKind::ElementNode(inner)
            | NodeKind::ListNode(inner)
            | NodeKind::ProgramNode(inner)
            | NodeKind::ReturnNode(inner) => Self::find_function_node_by_span(inner, span),

            NodeKind::SetqNode(left, right)
            | NodeKind::ProgNode(left, right)
            | NodeKind::WhileNode(left, right) => Self::find_function_node_by_span(left, span)
                .or_else(|| Self::find_function_node_by_span(right, span)),

            NodeKind::CondNode(cond, then_node, else_node) => {
                Self::find_function_node_by_span(cond, span)
                    .or_else(|| Self::find_function_node_by_span(then_node, span))
                    .or_else(|| {
                        else_node
                            .as_deref()
                            .and_then(|else_node| Self::find_function_node_by_span(else_node, span))
                    })
            }

            NodeKind::ElementsNode(elements) => elements
                .iter()
                .find_map(|element| Self::find_function_node_by_span(element, span)),

            NodeKind::NullNode
            | NodeKind::BoolNode(_)
            | NodeKind::IntNode(_)
            | NodeKind::RealNode(_)
            | NodeKind::Identifier(_)
            | NodeKind::BreakNode
            | NodeKind::ErrorNode => None,
        }
    }

    fn function_body_node<'b>(&self, node: &'b Node) -> Result<&'b Node, CodegenError> {
        match &node.kind {
            NodeKind::FuncNode(_, _, body) => Ok(body),
            NodeKind::LambdaNode(_, body) => Ok(body),
            _ => Err(CodegenError::InvalidNode {
                message: format!("expected function or lambda node, got {:?}", node.kind),
            }),
        }
    }

    fn same_span(left: &MultilinePosition, right: &MultilinePosition) -> bool {
        left.start_line == right.start_line
            && left.start_col == right.start_col
            && left.end_line == right.end_line
            && left.end_col == right.end_col
    }
}
