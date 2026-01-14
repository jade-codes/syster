//! Tests for find_selection_spans function in SysML selection module
//!
//! Issue #140: Tests the public API `find_selection_spans` which returns
//! hierarchical selection ranges at a given position in the AST.
//!
//! All tests follow the principle of testing through the public API only.

use crate::core::{Position, Span};
use crate::semantic::adapters::selection::find_sysml_selection_spans;
use crate::syntax::sysml::ast::{
    Definition, DefinitionKind, Element, Package, SysMLFile, Usage, UsageKind,
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

fn make_package(name: &str, span: Option<Span>, elements: Vec<Element>) -> Package {
    Package {
        name: Some(name.to_string()),
        elements,
        span,
    }
}

fn make_definition(name: &str, kind: DefinitionKind, span: Option<Span>) -> Definition {
    Definition {
        kind,
        is_abstract: false,
        is_variation: false,
        name: Some(name.to_string()),
        short_name: None,
        short_name_span: None,
        relationships: Default::default(),
        body: vec![],
        span,
    }
}

fn make_usage(name: &str, kind: UsageKind, span: Option<Span>) -> Usage {
    Usage {
        kind,
        name: Some(name.to_string()),
        short_name: None,
        short_name_span: None,
        relationships: Default::default(),
        body: vec![],
        span,
        is_derived: false,
        is_const: false,
    }
}

// =============================================================================
// Tests for find_selection_spans (Issue #140)
// =============================================================================

#[test]
fn test_find_selection_spans_empty_file() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };
    let pos = make_position(1, 5);
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_find_selection_spans_position_outside_all_elements() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = make_position(10, 5);
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_find_selection_spans_single_package() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = make_position(3, 0);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_find_selection_spans_single_definition() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(make_definition(
            "TestDef",
            DefinitionKind::Part,
            Some(make_span(1, 0, 5, 1)),
        ))],
    };
    let pos = make_position(3, 0);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_find_selection_spans_single_usage() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(make_usage(
            "testUsage",
            UsageKind::Part,
            Some(make_span(1, 0, 3, 1)),
        ))],
    };
    let pos = make_position(2, 0);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_find_selection_spans_stops_at_first_containing() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Package(make_package("First", Some(make_span(1, 0, 3, 1)), vec![])),
            Element::Package(make_package("Second", Some(make_span(5, 0, 8, 1)), vec![])),
        ],
    };
    let pos = make_position(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}
