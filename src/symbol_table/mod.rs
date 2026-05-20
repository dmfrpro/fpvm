pub mod builder;
pub mod builder_capture_pass;
pub mod builder_declaration_pass;
pub mod error;
pub mod table;
pub mod table_fmt;

pub use builder::SymbolTableBuilder;
pub use error::SymbolTableError;
pub use table::{
    CaptureInfo, FunctionId, FunctionInfo, FunctionKind, Scope, ScopeId, ScopeKind, Symbol,
    SymbolId, SymbolKind, SymbolTable,
};
