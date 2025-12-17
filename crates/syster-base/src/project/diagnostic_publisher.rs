use crate::core::{ParseError, ParseErrorKind, ParseResult};
use crate::semantic::types::diagnostic::{Diagnostic, Location, Position, Range};

/// Converts parse results into LSP-compatible diagnostics
pub struct DiagnosticPublisher;

impl DiagnosticPublisher {
    /// Converts a ParseResult into a vector of Diagnostics
    pub fn publish<T>(result: &ParseResult<T>, file_path: impl Into<String>) -> Vec<Diagnostic> {
        let file_path = file_path.into();

        result
            .errors
            .iter()
            .map(|error| Self::error_to_diagnostic(error, &file_path))
            .collect()
    }

    fn error_to_diagnostic(error: &ParseError, file_path: &str) -> Diagnostic {
        let location = Location::new(
            file_path,
            Range::new(
                Position::new(error.position.line, error.position.column),
                Position::new(error.position.line, error.position.column + 1),
            ),
        );

        let diagnostic = Diagnostic::error(&error.message, location);

        match error.kind {
            ParseErrorKind::SyntaxError => diagnostic.with_code("P001"),
            ParseErrorKind::AstError => diagnostic.with_code("P002"),
            ParseErrorKind::IoError => diagnostic.with_code("P003"),
        }
    }
}

#[cfg(test)]
mod tests;
