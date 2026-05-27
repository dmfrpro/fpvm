use crate::syntax::node::MultilinePosition;

use super::table::{FunctionId, ScopeId};

#[derive(Debug, Clone)]
pub enum SymbolTableError {
    DuplicateSymbol {
        name: String,
        scope_id: ScopeId,
        span: Option<MultilinePosition>,
    },

    UnknownSymbol {
        name: String,
        span: Option<MultilinePosition>,
    },

    UnknownScope {
        scope_id: ScopeId,
    },

    UnknownFunction {
        function_id: FunctionId,
    },
}
