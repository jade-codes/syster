//! Tests for folding range extraction

use syster::core::{Position, Span};
use syster::semantic::folding::{extract_kerml_folding_ranges, extract_sysml_folding_ranges};
use syster::semantic::types::{FoldableRange, FoldingKind};
use syster::syntax::kerml::ast::{
    Comment as KerMLComment, Element as KerMLElement, KerMLFile, Package as KerMLPackage,
};
use syster::syntax::sysml::ast::{
    Comment as SysMLComment, Element as SysMLElement, Package as SysMLPackage, SysMLFile,
};

fn make_span(start_line: usize, end_line: usize) -> Span {
    Span {
        start: Position {
            line: start_line,
            column: 0,
        },
        end: Position {
            line: end_line,
            column: 1,
        },
    }
}

// =============================================================================
// FoldableRange type tests
// =============================================================================

#[test]
fn test_foldable_range_new() {
    let span = make_span(1, 5);
    let range = FoldableRange::new(span, FoldingKind::Region);

    assert_eq!(range.span, span);
    assert_eq!(range.kind, FoldingKind::Region);
    assert!(range.collapsed_text.is_none());
}

#[test]
fn test_foldable_range_with_collapsed_text() {
    let span = make_span(1, 5);
    let range = FoldableRange::new(span, FoldingKind::Comment).with_collapsed_text("/* ... */");

    assert_eq!(range.kind, FoldingKind::Comment);
    assert_eq!(range.collapsed_text, Some("/* ... */".to_string()));
}

#[test]
fn test_is_multiline_true() {
    let span = make_span(1, 3);
    let range = FoldableRange::new(span, FoldingKind::Region);

    assert!(range.is_multiline());
}

#[test]
fn test_is_multiline_false_same_line() {
    let span = Span {
        start: Position { line: 5, column: 0 },
        end: Position {
            line: 5,
            column: 20,
        },
    };
    let range = FoldableRange::new(span, FoldingKind::Region);

    assert!(!range.is_multiline());
}

#[test]
fn test_folding_kind_equality() {
    assert_eq!(FoldingKind::Region, FoldingKind::Region);
    assert_eq!(FoldingKind::Comment, FoldingKind::Comment);
    assert_ne!(FoldingKind::Region, FoldingKind::Comment);
}

// =============================================================================
// SysML folding extraction tests
// =============================================================================

#[test]
fn test_sysml_extract_folding_ranges_empty_file() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };
    let ranges = extract_sysml_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_sysml_extract_folding_ranges_filters_single_line() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 1)), // Same line = not foldable
        })],
    };
    let ranges = extract_sysml_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_sysml_extract_folding_ranges_includes_multiline() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 5)), // Multi-line = foldable
        })],
    };
    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].kind, FoldingKind::Region);
}

#[test]
fn test_sysml_extract_folding_ranges_comment_kind() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Comment(SysMLComment {
            content: "A long comment".to_string(),
            span: Some(make_span(1, 3)),
        })],
    };
    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].kind, FoldingKind::Comment);
}

#[test]
fn test_sysml_extract_folding_ranges_sorted_by_line() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Package(SysMLPackage {
                name: Some("Second".to_string()),
                elements: vec![],
                span: Some(make_span(10, 15)),
            }),
            SysMLElement::Package(SysMLPackage {
                name: Some("First".to_string()),
                elements: vec![],
                span: Some(make_span(1, 5)),
            }),
        ],
    };
    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2);
    assert!(ranges[0].span.start.line < ranges[1].span.start.line);
}

// =============================================================================
// KerML folding extraction tests
// =============================================================================

#[test]
fn test_kerml_extract_folding_ranges_empty_file() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![],
    };
    let ranges = extract_kerml_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_kerml_extract_folding_ranges_filters_single_line() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 1)), // Same line = not foldable
        })],
    };
    let ranges = extract_kerml_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_kerml_extract_folding_ranges_includes_multiline() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 5)), // Multi-line = foldable
        })],
    };
    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].kind, FoldingKind::Region);
}

#[test]
fn test_kerml_extract_folding_ranges_comment_kind() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Comment(KerMLComment {
            content: "A long comment".to_string(),
            about: vec![],
            locale: None,
            span: Some(make_span(1, 3)),
        })],
    };
    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0].kind, FoldingKind::Comment);
}

#[test]
fn test_kerml_extract_folding_ranges_sorted_by_line() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Package(KerMLPackage {
                name: Some("Second".to_string()),
                elements: vec![],
                span: Some(make_span(10, 15)),
            }),
            KerMLElement::Package(KerMLPackage {
                name: Some("First".to_string()),
                elements: vec![],
                span: Some(make_span(1, 5)),
            }),
        ],
    };
    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2);
    assert!(ranges[0].span.start.line < ranges[1].span.start.line);
}
