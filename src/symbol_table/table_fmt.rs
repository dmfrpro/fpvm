use std::fmt;

use super::table::{FunctionKind, ScopeKind, SymbolId, SymbolKind, SymbolTable};

impl SymbolTable {
    fn format_symbol_ref(&self, symbol_id: SymbolId) -> String {
        match self.symbol(symbol_id) {
            Some(symbol) => {
                format!(
                    "#{} {} '{}': {}",
                    symbol.id, symbol.kind, symbol.name, symbol.label
                )
            }
            None => format!("#{} <missing>", symbol_id),
        }
    }

    fn format_symbol_refs(&self, symbols: &[SymbolId]) -> String {
        if symbols.is_empty() {
            return "[]".to_string();
        }

        let items = symbols
            .iter()
            .map(|symbol_id| self.format_symbol_ref(*symbol_id))
            .collect::<Vec<_>>()
            .join(", ");

        format!("[{}]", items)
    }
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolKind::Global => write!(f, "global"),
            SymbolKind::Local => write!(f, "local"),
            SymbolKind::Argument => write!(f, "argument"),
            SymbolKind::Function => write!(f, "function"),
        }
    }
}

impl fmt::Display for ScopeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScopeKind::Global => write!(f, "global"),
            ScopeKind::Function => write!(f, "function"),
            ScopeKind::Lambda => write!(f, "lambda"),
            ScopeKind::Prog => write!(f, "prog"),
        }
    }
}

impl fmt::Display for FunctionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionKind::TopLevel => write!(f, "top-level"),
            FunctionKind::Named => write!(f, "named"),
            FunctionKind::Lambda => write!(f, "lambda"),
        }
    }
}

impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SymbolTable")?;
        writeln!(f, "  global_scope_id: {}", self.global_scope_id)?;
        writeln!(f, "  entry_function_id: {}", self.entry_function_id)?;
        writeln!(f)?;

        writeln!(f, "Scopes:")?;
        for scope in &self.scopes {
            writeln!(
                f,
                "  #{} kind={} parent={:?} function={:?}",
                scope.id, scope.kind, scope.parent, scope.function_id
            )?;

            if let Some(span) = &scope.owner_span {
                writeln!(f, "    owner_span: {}", span)?;
            }

            if scope.symbols_by_name.is_empty() {
                writeln!(f, "    symbols: []")?;
            } else {
                writeln!(f, "    symbols:")?;

                let mut symbols = scope.symbols_by_name.iter().collect::<Vec<_>>();
                symbols.sort_by_key(|(name, _)| *name);

                for (name, symbol_id) in symbols {
                    writeln!(f, "      {} -> #{}", name, symbol_id)?;
                }
            }
        }

        writeln!(f)?;
        writeln!(f, "Symbols:")?;
        for symbol in &self.symbols {
            writeln!(
                f,
                "  #{} kind={} name='{}' label={} scope={} function={:?}",
                symbol.id,
                symbol.kind,
                symbol.name,
                symbol.label,
                symbol.scope_id,
                symbol.function_id
            )?;

            if let Some(span) = &symbol.declared_at {
                writeln!(f, "    declared_at: {}", span)?;
            }
        }

        writeln!(f)?;
        writeln!(f, "Functions:")?;
        for function in &self.functions {
            writeln!(
                f,
                "  #{} kind={} name={:?} label={} parent={:?} scope={}",
                function.id,
                function.kind,
                function.name,
                function.label,
                function.parent_function_id,
                function.scope_id
            )?;

            if let Some(span) = &function.owner_span {
                writeln!(f, "    owner_span: {}", span)?;
            }

            writeln!(f, "    args: {}", self.format_symbol_refs(&function.args))?;
            writeln!(
                f,
                "    locals: {}",
                self.format_symbol_refs(&function.locals)
            )?;

            if function.captures.is_empty() {
                writeln!(f, "    captures: []")?;
            } else {
                let captures = function
                    .captures
                    .iter()
                    .map(|capture| self.format_symbol_ref(capture.symbol_id))
                    .collect::<Vec<_>>()
                    .join(", ");

                writeln!(f, "    captures: [{}]", captures)?;
            }
        }

        Ok(())
    }
}
