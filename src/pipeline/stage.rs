use crate::pipeline::types::Diagnostic;

pub struct StageOutput<T> {
    pub value: Option<T>,
    pub diagnostics: Vec<Diagnostic>,
}

impl<T> StageOutput<T> {
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            diagnostics: Vec::new(),
        }
    }

    pub fn error(diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            value: None,
            diagnostics: diagnostics,
        }
    }

    pub fn ok_with_diagnostics(value: T, diagnostics: Vec<Diagnostic>) -> Self {
        Self {
            value: Some(value),
            diagnostics: diagnostics,
        }
    }
}