//! Tests for span_to_lsp_range helper function (Issue #124)
//!
//! Tests the conversion of internal Span to LSP Range.

use crate::server::helpers::span_to_lsp_range;
use syster::core::{Position as CorePosition, Span};

#[test]
fn test_span_to_lsp_range_single_line() {
    let span = Span {
        start: CorePosition {
            line: 5,
            column: 10,
        },
        end: CorePosition {
            line: 5,
            column: 20,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 5);
    assert_eq!(range.start.character, 10);
    assert_eq!(range.end.line, 5);
    assert_eq!(range.end.character, 20);
}

#[test]
fn test_span_to_lsp_range_multi_line() {
    let span = Span {
        start: CorePosition { line: 1, column: 5 },
        end: CorePosition {
            line: 3,
            column: 10,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 1);
    assert_eq!(range.start.character, 5);
    assert_eq!(range.end.line, 3);
    assert_eq!(range.end.character, 10);
}

#[test]
fn test_span_to_lsp_range_zero_position() {
    let span = Span {
        start: CorePosition { line: 0, column: 0 },
        end: CorePosition { line: 0, column: 5 },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 0);
    assert_eq!(range.start.character, 0);
    assert_eq!(range.end.line, 0);
    assert_eq!(range.end.character, 5);
}

#[test]
fn test_span_to_lsp_range_large_numbers() {
    let span = Span {
        start: CorePosition {
            line: 1000,
            column: 500,
        },
        end: CorePosition {
            line: 2000,
            column: 1000,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 1000);
    assert_eq!(range.start.character, 500);
    assert_eq!(range.end.line, 2000);
    assert_eq!(range.end.character, 1000);
}

#[test]
fn test_span_to_lsp_range_single_character() {
    let span = Span {
        start: CorePosition {
            line: 10,
            column: 15,
        },
        end: CorePosition {
            line: 10,
            column: 16,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 10);
    assert_eq!(range.start.character, 15);
    assert_eq!(range.end.line, 10);
    assert_eq!(range.end.character, 16);
}
