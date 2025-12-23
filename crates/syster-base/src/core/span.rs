/// Position tracking for AST nodes
///
/// Stores the source location (line/column) of AST nodes for LSP features
/// like hover, go-to-definition, and error reporting.
/// A span representing a range in source code (0-indexed for LSP compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

/// A reference to a symbol, including file and span information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolReference {
    pub file: String,
    pub span: Span,
}

/// A position in source code (0-indexed)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// Create a span from line/column coordinates
    pub fn from_coords(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            start: Position::new(start_line, start_col),
            end: Position::new(end_line, end_col),
        }
    }

    /// Check if a position falls within this span
    pub fn contains(&self, position: Position) -> bool {
        if position.line < self.start.line || position.line > self.end.line {
            return false;
        }
        if position.line == self.start.line && position.column < self.start.column {
            return false;
        }
        if position.line == self.end.line && position.column > self.end.column {
            return false;
        }
        true
    }
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
