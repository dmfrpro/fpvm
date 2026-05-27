use std::collections::HashMap;

use crate::syntax::node::MultilinePosition;

use super::error::SymbolTableError;

pub type SymbolId = usize;
pub type ScopeId = usize;
pub type FunctionId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Global,
    Local,
    Argument,
    Function,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    Global,
    Function,
    Lambda,
    Prog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionKind {
    TopLevel,
    Named,
    Lambda,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub id: SymbolId,
    pub name: String,
    pub kind: SymbolKind,

    pub scope_id: ScopeId,
    pub function_id: Option<FunctionId>,

    pub label: String,
    pub declared_at: Option<MultilinePosition>,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub kind: ScopeKind,

    pub parent: Option<ScopeId>,
    pub function_id: Option<FunctionId>,

    pub owner_span: Option<MultilinePosition>,

    pub symbols_by_name: HashMap<String, SymbolId>,
}

#[derive(Debug, Clone)]
pub struct CaptureInfo {
    pub symbol_id: SymbolId,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub id: FunctionId,
    pub kind: FunctionKind,

    pub name: Option<String>,
    pub label: String,

    pub parent_function_id: Option<FunctionId>,
    pub scope_id: ScopeId,

    pub owner_span: Option<MultilinePosition>,

    pub args: Vec<SymbolId>,
    pub locals: Vec<SymbolId>,
    pub captures: Vec<CaptureInfo>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
    pub scopes: Vec<Scope>,
    pub functions: Vec<FunctionInfo>,

    pub global_scope_id: ScopeId,
    pub entry_function_id: FunctionId,

    next_symbol_id: SymbolId,
    next_scope_id: ScopeId,
    next_function_id: FunctionId,
}

impl SymbolTable {
    pub fn new(program_span: Option<MultilinePosition>) -> Self {
        let mut table = Self {
            symbols: Vec::new(),
            scopes: Vec::new(),
            functions: Vec::new(),

            global_scope_id: 0,
            entry_function_id: 0,

            next_symbol_id: 0,
            next_scope_id: 0,
            next_function_id: 0,
        };

        let global_scope_id =
            table.create_scope(ScopeKind::Global, None, None, program_span.clone());

        let top_level_function_id = table.create_function(
            FunctionKind::TopLevel,
            Some("__top_level".to_string()),
            "__top_level".to_string(),
            None,
            global_scope_id,
            program_span,
        );

        table.global_scope_id = global_scope_id;
        table.entry_function_id = top_level_function_id;

        table.scopes[global_scope_id].function_id = Some(top_level_function_id);

        table
    }

    pub fn create_scope(
        &mut self,
        kind: ScopeKind,
        parent: Option<ScopeId>,
        function_id: Option<FunctionId>,
        owner_span: Option<MultilinePosition>,
    ) -> ScopeId {
        let id = self.next_scope_id;
        self.next_scope_id += 1;

        self.scopes.push(Scope {
            id,
            kind,
            parent,
            function_id,
            owner_span,
            symbols_by_name: HashMap::new(),
        });

        id
    }

    pub fn create_function(
        &mut self,
        kind: FunctionKind,
        name: Option<String>,
        label: String,
        parent_function_id: Option<FunctionId>,
        scope_id: ScopeId,
        owner_span: Option<MultilinePosition>,
    ) -> FunctionId {
        let id = self.next_function_id;
        self.next_function_id += 1;

        self.functions.push(FunctionInfo {
            id,
            kind,
            name,
            label,
            parent_function_id,
            scope_id,
            owner_span,
            args: Vec::new(),
            locals: Vec::new(),
            captures: Vec::new(),
        });

        id
    }

    pub fn declare_symbol(
        &mut self,
        name: impl Into<String>,
        kind: SymbolKind,
        scope_id: ScopeId,
        function_id: Option<FunctionId>,
        declared_at: Option<MultilinePosition>,
    ) -> Result<SymbolId, SymbolTableError> {
        let name = name.into();

        if scope_id >= self.scopes.len() {
            return Err(SymbolTableError::UnknownScope { scope_id });
        }

        if let Some(function_id) = function_id {
            if function_id >= self.functions.len() {
                return Err(SymbolTableError::UnknownFunction { function_id });
            }
        }

        if self.scopes[scope_id].symbols_by_name.contains_key(&name) {
            return Err(SymbolTableError::DuplicateSymbol {
                name,
                scope_id,
                span: declared_at,
            });
        }

        let id = self.next_symbol_id;
        self.next_symbol_id += 1;

        let label = Self::make_symbol_label(kind, &name, id);

        let symbol = Symbol {
            id,
            name: name.clone(),
            kind,
            scope_id,
            function_id,
            label,
            declared_at,
        };

        self.symbols.push(symbol);
        self.scopes[scope_id].symbols_by_name.insert(name, id);

        if let Some(function_id) = function_id {
            self.attach_symbol_to_function(function_id, id, kind)?;
        }

        Ok(id)
    }

    fn attach_symbol_to_function(
        &mut self,
        function_id: FunctionId,
        symbol_id: SymbolId,
        kind: SymbolKind,
    ) -> Result<(), SymbolTableError> {
        let function = self
            .functions
            .get_mut(function_id)
            .ok_or(SymbolTableError::UnknownFunction { function_id })?;

        match kind {
            SymbolKind::Argument => function.args.push(symbol_id),
            SymbolKind::Local => function.locals.push(symbol_id),
            _ => {}
        }

        Ok(())
    }

    fn make_symbol_label(kind: SymbolKind, name: &str, id: SymbolId) -> String {
        let prefix = match kind {
            SymbolKind::Global => "global",
            SymbolKind::Local => "local",
            SymbolKind::Argument => "arg",
            SymbolKind::Function => "func",
        };

        format!("{}_{}_{}", prefix, name, id)
    }

    pub fn lookup_in_scope(&self, scope_id: ScopeId, name: &str) -> Option<SymbolId> {
        self.scopes
            .get(scope_id)?
            .symbols_by_name
            .get(name)
            .copied()
    }

    pub fn lookup(&self, mut scope_id: ScopeId, name: &str) -> Option<SymbolId> {
        loop {
            let scope = self.scopes.get(scope_id)?;

            if let Some(symbol_id) = scope.symbols_by_name.get(name) {
                return Some(*symbol_id);
            }

            match scope.parent {
                Some(parent) => scope_id = parent,
                None => return None,
            }
        }
    }

    pub fn add_capture(
        &mut self,
        function_id: FunctionId,
        symbol_id: SymbolId,
    ) -> Result<(), SymbolTableError> {
        if symbol_id >= self.symbols.len() {
            return Err(SymbolTableError::UnknownSymbol {
                name: format!("#{}", symbol_id),
                span: None,
            });
        }

        let function = self
            .functions
            .get_mut(function_id)
            .ok_or(SymbolTableError::UnknownFunction { function_id })?;

        let already_exists = function
            .captures
            .iter()
            .any(|capture| capture.symbol_id == symbol_id);

        if !already_exists {
            function.captures.push(CaptureInfo { symbol_id });
        }

        Ok(())
    }

    pub fn symbol(&self, id: SymbolId) -> Option<&Symbol> {
        self.symbols.get(id)
    }

    pub fn scope(&self, id: ScopeId) -> Option<&Scope> {
        self.scopes.get(id)
    }

    pub fn function(&self, id: FunctionId) -> Option<&FunctionInfo> {
        self.functions.get(id)
    }

    pub fn symbols(&self) -> &[Symbol] {
        &self.symbols
    }

    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }

    pub fn functions(&self) -> &[FunctionInfo] {
        &self.functions
    }
}
