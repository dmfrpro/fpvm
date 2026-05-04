use std::fmt;

#[derive(Debug, Clone)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
}

impl Diagnostic {
    pub fn error(message: String) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: message,
        }
    }

    pub fn warning(message: String) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: message,
        }
    }

    pub fn note(message: String) -> Self {
        Self {
            level: DiagnosticLevel::Note,
            message: message,
        }
    }
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            &DiagnosticLevel::Error => "Error",
            &DiagnosticLevel::Warning => "Warning",
            &DiagnosticLevel::Note => "Note",
        };
        write!(f, "{}", message)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:{}", self.level, self.message)
    }
}
