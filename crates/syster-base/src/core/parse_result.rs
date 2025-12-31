use super::Position;

/// Result of parsing a file with detailed error information
#[derive(Debug)]
pub struct ParseResult<T> {
    /// The parsed content (empty/default on error)
    pub content: Option<T>,
    /// Parse errors with position information
    pub errors: Vec<ParseError>,
}

/// Detailed parse error with position information
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: Position,
    pub kind: ParseErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// Syntax error from parser
    SyntaxError,
    /// Error constructing AST
    AstError,
    /// IO error reading file
    IoError,
}

impl<T> ParseResult<T> {
    /// Creates a successful parse result
    pub fn success(content: T) -> Self {
        Self {
            content: Some(content),
            errors: vec![],
        }
    }

    /// Creates a failed parse result with errors
    pub fn with_errors(errors: Vec<ParseError>) -> Self {
        Self {
            content: None,
            errors,
        }
    }

    /// Returns true if parsing succeeded (no errors)
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// Returns true if there were parse errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl ParseError {
    pub fn syntax_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            position: Position::new(line, column),
            kind: ParseErrorKind::SyntaxError,
        }
    }

    pub fn ast_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            position: Position::new(line, column),
            kind: ParseErrorKind::AstError,
        }
    }
}
