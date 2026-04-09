use crate::syntax::MultilinePosition;

#[derive(Debug)]
pub enum SemanticErrorKind {
    UndefinedVariable(String),
    DuplicateDefinition(String),
    BreakOutsideLoop,
    ReturnOutsideFunction,
    InvalidSetqTarget,
    InvalidFuncName,
    InvalidLambdaParams,
}

#[derive(Debug)]
pub struct SemanticError {
    pub kind: SemanticErrorKind,
    pub span: MultilinePosition,
    pub message: Option<String>,
}

impl SemanticError {
    pub fn new(kind: SemanticErrorKind, span: MultilinePosition, message: Option<String>) -> Self {
        Self {
            kind,
            span,
            message,
        }
    }
}
