//! Tests for try_push_span function in SysML selection module
//!
//! Issue #142: Tests the private `try_push_span` function through various element
//! types and boundary conditions in `find_selection_spans`. The function tries to
//! push a span if it contains the position.
//!
//! All tests follow the principle of testing through the public API only.

use syster::core::{Position, Span};
use syster::semantic::adapters::selection::find_sysml_selection_spans;
use syster::syntax::sysml::ast::{Alias, Comment, Element, Import, Package, SysMLFile};

// =============================================================================
// Helper Functions
// =============================================================================

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span::from_coords(start_line, start_col, end_line, end_col)
}

fn make_position(line: usize, column: usize) -> Position {
    Position::new(line, column)
}

fn make_package(name: &str, span: Option<Span>) -> Package {
    Package {
        name: Some(name.to_string()),
        elements: vec![],
        span,
    }
}

fn make_comment(content: &str, span: Option<Span>) -> Comment {
    Comment {
        content: content.to_string(),
        span,
    }
}

fn make_import(path: &str, span: Option<Span>) -> Import {
    Import {
        path: path.to_string(),
        is_recursive: false,
        span,
    }
}

fn make_alias(name: &str, span: Option<Span>) -> Alias {
    Alias {
        name: Some(name.to_string()),
        target: "Target".to_string(),
        span,
    }
}

// =============================================================================
// Tests for try_push_span (Issue #142) - Tested through element types
// =============================================================================

#[test]
fn test_try_push_span_with_none_span() {
    // Element with None span should not be collected
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package("Test", None))],
    };
    let pos = make_position(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_try_push_span_with_comment() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test comment",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = make_position(1, 10);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_with_import() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Import(make_import(
            "Test::Package",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = make_position(1, 10);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_with_alias() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Alias(make_alias(
            "TestAlias",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = make_position(1, 10);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_position_at_boundary_start() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 10); // Exactly at start
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_try_push_span_position_at_boundary_end() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 20); // Exactly at end
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_try_push_span_position_before_span() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 9); // Just before start
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_try_push_span_position_after_span() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 21); // Just after end
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}
