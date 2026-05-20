use std::collections::HashMap;

use crate::syntax::{MultilinePosition, Node};

use super::{FunctionId, ScopeId, SymbolTable, SymbolTableError};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct SpanKey {
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
}

impl From<&MultilinePosition> for SpanKey {
    fn from(span: &MultilinePosition) -> Self {
        Self {
            start_line: span.start_line,
            start_col: span.start_col,
            end_line: span.end_line,
            end_col: span.end_col,
        }
    }
}

pub struct SymbolTableBuilder {
    pub(super) table: SymbolTable,

    pub(super) scope_stack: Vec<ScopeId>,
    pub(super) function_stack: Vec<FunctionId>,

    pub(super) scope_by_span: HashMap<SpanKey, ScopeId>,
    pub(super) function_by_span: HashMap<SpanKey, FunctionId>,

    pub(super) errors: Vec<SymbolTableError>,
}

impl SymbolTableBuilder {
    pub fn new(program_span: Option<crate::syntax::node::MultilinePosition>) -> Self {
        let table = SymbolTable::new(program_span);

        Self {
            scope_stack: vec![table.global_scope_id],
            function_stack: vec![table.entry_function_id],

            scope_by_span: HashMap::new(),
            function_by_span: HashMap::new(),

            errors: Vec::new(),
            table,
        }
    }

    pub fn build(mut self, ast: &Node) -> Result<SymbolTable, Vec<SymbolTableError>> {
        // declaration pass 1
        self.visit_node(ast);

        if !self.errors.is_empty() {
            return Err(self.errors);
        }

        // capture collection pass 2
        self.reset_context();
        self.collect_captures(ast);

        if self.errors.is_empty() {
            Ok(self.table)
        } else {
            Err(self.errors)
        }
    }

    pub(super) fn current_scope(&self) -> ScopeId {
        *self
            .scope_stack
            .last()
            .expect("scope stack must not be empty")
    }

    pub(super) fn current_function(&self) -> FunctionId {
        *self
            .function_stack
            .last()
            .expect("function stack must not be empty")
    }

    pub(super) fn push_error(&mut self, error: SymbolTableError) {
        self.errors.push(error);
    }

    fn reset_context(&mut self) {
        self.scope_stack.clear();
        self.function_stack.clear();

        self.scope_stack.push(self.table.global_scope_id);
        self.function_stack.push(self.table.entry_function_id);
    }
}
