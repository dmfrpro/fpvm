use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Scope {
    symbols: HashMap<String, SymbolInfo>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub defined_at: crate::syntax::MultilinePosition,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, info: SymbolInfo) -> Result<(), SymbolInfo> {
        if self.symbols.contains_key(&name) {
            Err(self.symbols.get(&name).unwrap().clone())
        } else {
            self.symbols.insert(name, info);
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        self.symbols.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }
}

pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn current_scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    pub fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.lookup(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn insert(&mut self, name: String, info: SymbolInfo) -> Result<(), SymbolInfo> {
        self.current_scope_mut().insert(name, info)
    }
}
