//! Tests for collect_usage_spans function in SysML selection module
//!
//! Issue #143: Tests the private `collect_usage_spans` function through nested
//! usages in `find_selection_spans`. The function recursively collects all spans
//! that contain the position within usage elements.
//!
//! All tests follow the principle of testing through the public API only.

use crate::core::{Position, Span};
use crate::semantic::adapters::selection::find_sysml_selection_spans;
use crate::syntax::sysml::ast::{Comment, Element, SysMLFile, Usage, UsageKind, UsageMember};

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
        short_name: None,
        short_name_span: None,
        relationships: Default::default(),
        body,
        span,
        is_derived: false,
        is_const: false,
    }
}

fn make_comment(content: &str, span: Option<Span>) -> Comment {
    Comment {
        content: content.to_string(),
        span,
    }
}

// =============================================================================
// Tests for collect_usage_spans (Issue #143) - Tested through nested usages
// =============================================================================

#[test]
fn test_collect_usage_spans_empty_usage() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(make_usage(
            "Empty",
            UsageKind::Part,
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ))],
    };
    let pos = make_position(2, 0);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_usage_spans_with_nested_usage() {
    let inner = make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 2, 3, 3)),
        vec![],
    );
    let outer = Element::Usage(make_usage(
        "Outer",
        UsageKind::Part,
        Some(make_span(1, 0, 5, 1)),
        vec![UsageMember::Usage(Box::new(inner))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Inner
    assert_eq!(spans[1].start.line, 1); // Outer
}

#[test]
fn test_collect_usage_spans_with_comment_member() {
    let comment = make_comment("Comment", Some(make_span(2, 2, 2, 10)));
    let usage = Element::Usage(make_usage(
        "Parent",
        UsageKind::Part,
        Some(make_span(1, 0, 5, 1)),
        vec![UsageMember::Comment(comment)],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![usage],
    };
    let pos = make_position(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Comment
    assert_eq!(spans[1].start.line, 1); // Usage
}

#[test]
fn test_collect_usage_spans_deeply_nested() {
    // Test deeply nested usage -> usage -> usage -> comment
    let comment = make_comment("Deep", Some(make_span(4, 6, 4, 15)));
    let level3 = make_usage(
        "Level3",
        UsageKind::Part,
        Some(make_span(3, 4, 5, 5)),
        vec![UsageMember::Comment(comment)],
    );
    let level2 = make_usage(
        "Level2",
        UsageKind::Part,
        Some(make_span(2, 2, 6, 3)),
        vec![UsageMember::Usage(Box::new(level3))],
    );
    let level1 = Element::Usage(make_usage(
        "Level1",
        UsageKind::Part,
        Some(make_span(1, 0, 7, 1)),
        vec![UsageMember::Usage(Box::new(level2))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![level1],
    };
    let pos = make_position(4, 10);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 4);
    assert_eq!(spans[0].start.line, 4); // Comment
    assert_eq!(spans[1].start.line, 3); // Level3
    assert_eq!(spans[2].start.line, 2); // Level2
    assert_eq!(spans[3].start.line, 1); // Level1
}

#[test]
fn test_collect_usage_spans_stops_at_first_matching_member() {
    let usage1 = make_usage(
        "First",
        UsageKind::Part,
        Some(make_span(2, 0, 3, 1)),
        vec![],
    );
    let usage2 = make_usage(
        "Second",
        UsageKind::Part,
        Some(make_span(4, 0, 5, 1)),
        vec![],
    );
    let parent = Element::Usage(make_usage(
        "Parent",
        UsageKind::Part,
        Some(make_span(1, 0, 6, 1)),
        vec![
            UsageMember::Usage(Box::new(usage1)),
            UsageMember::Usage(Box::new(usage2)),
        ],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![parent],
    };
    let pos = make_position(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // First usage
    assert_eq!(spans[1].start.line, 1); // Parent
}

#[test]
fn test_collect_usage_spans_with_different_usage_kinds() {
    // Test various usage kinds work the same
    let kinds = vec![
        UsageKind::Part,
        UsageKind::Port,
        UsageKind::Action,
        UsageKind::Item,
        UsageKind::Attribute,
    ];

    for kind in kinds {
        let usage = Element::Usage(make_usage(
            "Test",
            kind.clone(),
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ));
        let file = SysMLFile {
            namespace: None,
            namespaces: vec![],
            elements: vec![usage],
        };
        let pos = make_position(2, 0);
        let spans = find_sysml_selection_spans(&file, pos);

        assert_eq!(spans.len(), 1, "Failed for kind {kind:?}");
    }
}

#[test]
fn test_collect_usage_spans_returns_false_when_not_containing() {
    let usage = Element::Usage(make_usage(
        "Test",
        UsageKind::Part,
        Some(make_span(1, 0, 3, 1)),
        vec![],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![usage],
    };
    let pos = make_position(5, 0); // Outside usage
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}
