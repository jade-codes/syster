use std::fmt;

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

/// Position in a source file (0-indexed for LSP compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// Range in a source file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Location of a diagnostic in a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file: String,
    pub range: Range,
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

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Creates a single-character range
    pub fn single(line: usize, column: usize) -> Self {
        Self {
            start: Position::new(line, column),
            end: Position::new(line, column + 1),
        }
    }
}

impl Location {
    pub fn new(file: impl Into<String>, range: Range) -> Self {
        Self {
            file: file.into(),
            range,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {:?}: {}",
            self.location.file,
            self.location.range.start.line + 1,
            self.location.range.start.column + 1,
            self.severity,
            self.message
        )
    }
}

#[cfg(test)]
mod tests;
