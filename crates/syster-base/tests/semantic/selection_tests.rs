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

#[test]
fn test_sysml_find_selection_spans_three_level_nested_packages() {
    let innermost_package = SysMLPackage {
        name: Some("Innermost".to_string()),
        elements: vec![],
        span: Some(make_span(4, 8, 6, 9)),
    };
    let middle_package = SysMLPackage {
        name: Some("Middle".to_string()),
        elements: vec![SysMLElement::Package(innermost_package)],
        span: Some(make_span(2, 4, 8, 5)),
    };
    let outer_package = SysMLPackage {
        name: Some("Outer".to_string()),
        elements: vec![SysMLElement::Package(middle_package)],
        span: Some(make_span(0, 0, 10, 1)),
    };
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(outer_package)],
    };

    let pos = Position::new(5, 10); // Inside innermost package
    let spans = find_sysml_selection_spans(&file, pos);

    // Should have 3 spans: innermost, middle, outer (ordered by size)
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].start.line, 4); // innermost
    assert_eq!(spans[1].start.line, 2); // middle
    assert_eq!(spans[2].start.line, 0); // outer
}

#[test]
fn test_sysml_find_selection_spans_sibling_packages() {
    let package1 = SysMLPackage {
        name: Some("Package1".to_string()),
        elements: vec![],
        span: Some(make_span(0, 0, 5, 1)),
    };
    let package2 = SysMLPackage {
        name: Some("Package2".to_string()),
        elements: vec![],
        span: Some(make_span(7, 0, 12, 1)),
    };
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Package(package1),
            SysMLElement::Package(package2),
        ],
    };

    // Position in first package
    let pos1 = Position::new(2, 5);
    let spans1 = find_sysml_selection_spans(&file, pos1);
    assert_eq!(spans1.len(), 1);
    assert_eq!(spans1[0].start.line, 0);
    assert_eq!(spans1[0].end.line, 5);

    // Position in second package
    let pos2 = Position::new(9, 5);
    let spans2 = find_sysml_selection_spans(&file, pos2);
    assert_eq!(spans2.len(), 1);
    assert_eq!(spans2[0].start.line, 7);
    assert_eq!(spans2[0].end.line, 12);

    // Position between packages
    let pos3 = Position::new(6, 0);
    let spans3 = find_sysml_selection_spans(&file, pos3);
    assert!(spans3.is_empty());
}

#[test]
fn test_sysml_find_selection_spans_at_package_start() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };
    // Position exactly at package start
    let pos = Position::new(0, 0);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_sysml_find_selection_spans_at_package_end() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 10)),
        })],
    };
    // Position exactly at package end
    let pos = Position::new(5, 10);
    let spans = find_sysml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_sysml_find_selection_spans_package_without_span() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: None,
        })],
    };
    let pos = Position::new(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);
    assert!(spans.is_empty());
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

#[test]
fn test_kerml_find_selection_spans_nested_packages() {
    let inner_package = KerMLPackage {
        name: Some("Inner".to_string()),
        elements: vec![],
        span: Some(make_span(2, 4, 4, 5)),
    };
    let outer_package = KerMLPackage {
        name: Some("Outer".to_string()),
        elements: vec![KerMLElement::Package(inner_package)],
        span: Some(make_span(0, 0, 6, 1)),
    };
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(outer_package)],
    };

    let pos = Position::new(3, 5); // Inside inner package
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have 2 spans: inner (smaller) first, outer (larger) second
    assert_eq!(spans.len(), 2);
    // First span should be inner (smaller)
    assert_eq!(spans[0].start.line, 2);
    // Second span should be outer (larger)
    assert_eq!(spans[1].start.line, 0);
}

#[test]
fn test_kerml_find_selection_spans_three_level_nested_packages() {
    let innermost_package = KerMLPackage {
        name: Some("Innermost".to_string()),
        elements: vec![],
        span: Some(make_span(4, 8, 6, 9)),
    };
    let middle_package = KerMLPackage {
        name: Some("Middle".to_string()),
        elements: vec![KerMLElement::Package(innermost_package)],
        span: Some(make_span(2, 4, 8, 5)),
    };
    let outer_package = KerMLPackage {
        name: Some("Outer".to_string()),
        elements: vec![KerMLElement::Package(middle_package)],
        span: Some(make_span(0, 0, 10, 1)),
    };
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(outer_package)],
    };

    let pos = Position::new(5, 10); // Inside innermost package
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have 3 spans: innermost, middle, outer (ordered by size)
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].start.line, 4); // innermost
    assert_eq!(spans[1].start.line, 2); // middle
    assert_eq!(spans[2].start.line, 0); // outer
}

#[test]
fn test_kerml_find_selection_spans_sibling_packages() {
    let package1 = KerMLPackage {
        name: Some("Package1".to_string()),
        elements: vec![],
        span: Some(make_span(0, 0, 5, 1)),
    };
    let package2 = KerMLPackage {
        name: Some("Package2".to_string()),
        elements: vec![],
        span: Some(make_span(7, 0, 12, 1)),
    };
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Package(package1),
            KerMLElement::Package(package2),
        ],
    };

    // Position in first package
    let pos1 = Position::new(2, 5);
    let spans1 = find_kerml_selection_spans(&file, pos1);
    assert_eq!(spans1.len(), 1);
    assert_eq!(spans1[0].start.line, 0);
    assert_eq!(spans1[0].end.line, 5);

    // Position in second package
    let pos2 = Position::new(9, 5);
    let spans2 = find_kerml_selection_spans(&file, pos2);
    assert_eq!(spans2.len(), 1);
    assert_eq!(spans2[0].start.line, 7);
    assert_eq!(spans2[0].end.line, 12);

    // Position between packages
    let pos3 = Position::new(6, 0);
    let spans3 = find_kerml_selection_spans(&file, pos3);
    assert!(spans3.is_empty());
}

#[test]
fn test_kerml_find_selection_spans_at_package_start() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };
    // Position exactly at package start
    let pos = Position::new(0, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_kerml_find_selection_spans_at_package_end() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 10)),
        })],
    };
    // Position exactly at package end
    let pos = Position::new(5, 10);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_kerml_find_selection_spans_package_without_span() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: None,
        })],
    };
    let pos = Position::new(2, 5);
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}
