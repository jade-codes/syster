use std::fmt;

// Re-export Position and Span from core for consumers of this module
pub use crate::core::{Position, Span};

/// Represents a diagnostic (error, warning, or info) with precise location
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub location: Location,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Location of a diagnostic in a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file: String,
    pub span: Span,
}

impl Diagnostic {
    /// Creates a new error diagnostic
    pub fn error(message: impl Into<String>, location: Location) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            location,
            code: None,
        }
    }

    /// Creates a new warning diagnostic
    pub fn warning(message: impl Into<String>, location: Location) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            location,
            code: None,
        }
    }

    /// Adds an error code
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
}

impl Location {
    pub fn new(file: impl Into<String>, span: Span) -> Self {
        Self {
            file: file.into(),
            span,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {:?}: {}",
            self.location.file,
            self.location.span.start.line + 1,
            self.location.span.start.column + 1,
            self.severity,
            self.message
        )
    }
}
