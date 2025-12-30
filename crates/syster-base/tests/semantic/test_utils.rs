//! Common test utilities for semantic tests
//!
//! This module provides shared helper functions used across multiple test files
//! to reduce code duplication and improve maintainability.

use syster::core::{Position, Span};

/// Creates a Span with the given line and column positions.
///
/// # Arguments
/// * `start_line` - Starting line number
/// * `start_col` - Starting column number
/// * `end_line` - Ending line number
/// * `end_col` - Ending column number
///
/// # Example
/// ```
/// let span = make_span(0, 0, 5, 10);
/// assert_eq!(span.start.line, 0);
/// assert_eq!(span.start.column, 0);
/// assert_eq!(span.end.line, 5);
/// assert_eq!(span.end.column, 10);
/// ```
pub fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span {
        start: Position {
            line: start_line,
            column: start_col,
        },
        end: Position {
            line: end_line,
            column: end_col,
        },
    }
}
