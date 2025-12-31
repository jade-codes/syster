//! Tests for selection range extraction

use syster::core::{Position, Span};
use syster::semantic::selection::{find_kerml_selection_spans, find_sysml_selection_spans};
use syster::syntax::kerml::ast::{Element as KerMLElement, KerMLFile, Package as KerMLPackage};
use syster::syntax::sysml::ast::{Element as SysMLElement, Package as SysMLPackage, SysMLFile};

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
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

// =============================================================================
// SysML selection span tests
// =============================================================================

#[test]
fn test_sysml_find_selection_spans_empty_file() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };
    let pos = Position::new(1, 5);
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_sysml_find_selection_spans_position_in_package() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };
    let pos = Position::new(2, 5); // Inside package
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 0);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_sysml_find_selection_spans_position_outside() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };
    let pos = Position::new(10, 5); // Outside package
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_sysml_find_selection_spans_nested_packages() {
    let inner_package = SysMLPackage {
        name: Some("Inner".to_string()),
        elements: vec![],
        span: Some(make_span(2, 4, 4, 5)),
    };
    let outer_package = SysMLPackage {
        name: Some("Outer".to_string()),
        elements: vec![SysMLElement::Package(inner_package)],
        span: Some(make_span(0, 0, 6, 1)),
    };
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(outer_package)],
    };

    let pos = Position::new(3, 5); // Inside inner package
    let spans = find_sysml_selection_spans(&file, pos);

    // Should have 2 spans: inner (smaller) first, outer (larger) second
    assert_eq!(spans.len(), 2);
    // First span should be inner (smaller)
    assert_eq!(spans[0].start.line, 2);
    // Second span should be outer (larger)
    assert_eq!(spans[1].start.line, 0);
}

// =============================================================================
// KerML selection span tests
// =============================================================================

#[test]
fn test_kerml_find_selection_spans_empty_file() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![],
    };
    let pos = Position::new(1, 5);
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_kerml_find_selection_spans_position_in_package() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };
    let pos = Position::new(2, 5); // Inside package
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 0);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_kerml_find_selection_spans_position_outside() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };
    let pos = Position::new(10, 5); // Outside package
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}
