//! Tests for range_size function in SysML selection module
//!
//! Issue #141: Tests the private `range_size` function through observable sorting
//! behavior in `find_selection_spans`. The function calculates a rough "size" of
//! a span for sorting purposes.
//!
//! All tests follow the principle of testing through the public API only.

use syster::core::{Position, Span};
use syster::semantic::adapters::selection::find_sysml_selection_spans;
use syster::syntax::sysml::ast::{
    Comment, Element, Package, SysMLFile, Usage, UsageKind, UsageMember,
};

// =============================================================================
// Helper Functions
// =============================================================================

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span::from_coords(start_line, start_col, end_line, end_col)
}

fn make_position(line: usize, column: usize) -> Position {
    Position::new(line, column)
}

fn make_usage(name: &str, kind: UsageKind, span: Option<Span>, body: Vec<UsageMember>) -> Usage {
    Usage {
        kind,
        name: Some(name.to_string()),
        relationships: Default::default(),
        body,
        span,
        is_derived: false,
        is_readonly: false,
    }
}

fn make_package(name: &str, span: Option<Span>, elements: Vec<Element>) -> Package {
    Package {
        name: Some(name.to_string()),
        elements,
        span,
    }
}

fn make_comment(content: &str, span: Option<Span>) -> Comment {
    Comment {
        content: content.to_string(),
        span,
    }
}

// =============================================================================
// Tests for range_size (Issue #141) - Tested through sorting behavior
// =============================================================================

#[test]
fn test_range_size_single_line_spans_sorted() {
    // Smaller single-line span should come first
    let inner = Element::Usage(make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 5, 2, 15)),
        vec![],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(2, 0, 2, 20)),
        vec![inner],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 10);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.column, 5); // Inner (smaller)
    assert_eq!(spans[1].start.column, 0); // Outer (larger)
}

#[test]
fn test_range_size_multi_line_spans_sorted() {
    // Span with fewer lines should come first
    let inner = Element::Usage(make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 0, 3, 10)),
        vec![],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(1, 0, 5, 10)),
        vec![inner],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Inner (2 lines)
    assert_eq!(spans[1].start.line, 1); // Outer (5 lines)
}

#[test]
fn test_range_size_three_nested_levels_sorted() {
    // Test sorting with three levels of nesting
    let innermost = make_comment("comment", Some(make_span(3, 5, 3, 15)));
    let middle = Element::Usage(make_usage(
        "Middle",
        UsageKind::Part,
        Some(make_span(2, 0, 4, 10)),
        vec![UsageMember::Comment(innermost)],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(1, 0, 6, 10)),
        vec![middle],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(3, 10);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].start.line, 3); // Innermost
    assert_eq!(spans[1].start.line, 2); // Middle
    assert_eq!(spans[2].start.line, 1); // Outer
}

#[test]
fn test_range_size_same_start_different_end() {
    // Spans starting at same position but ending differently
    let inner = Element::Usage(make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 0, 3, 5)),
        vec![],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(2, 0, 5, 10)),
        vec![inner],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].end.line, 3); // Inner (smaller)
    assert_eq!(spans[1].end.line, 5); // Outer (larger)
}
