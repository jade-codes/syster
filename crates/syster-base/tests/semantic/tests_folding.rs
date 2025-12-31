//! Tests for folding range extraction

use syster::core::{Position, Span};
use syster::semantic::extract_folding_ranges;
use syster::syntax::SyntaxFile;
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
// SysML folding extraction tests
// =============================================================================

#[test]
fn test_sysml_extract_folding_ranges_empty_file() {
    let file = SyntaxFile::SysML(SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    });
    let ranges = extract_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_sysml_extract_folding_ranges_filters_single_line() {
    let file = SyntaxFile::SysML(SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 1)), // Same line = not foldable
        })],
    });
    let ranges = extract_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_sysml_extract_folding_ranges_includes_multiline() {
    let file = SyntaxFile::SysML(SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 5)), // Multi-line = foldable
        })],
    });
    let ranges = extract_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert!(!ranges[0].is_comment); // Package is not a comment
}

#[test]
fn test_sysml_extract_folding_ranges_comment_kind() {
    let file = SyntaxFile::SysML(SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Comment(SysMLComment {
            content: "A long comment".to_string(),
            span: Some(make_span(1, 3)),
        })],
    });
    let ranges = extract_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert!(ranges[0].is_comment); // Comment should be marked as comment
}

#[test]
fn test_sysml_extract_folding_ranges_sorted_by_line() {
    let file = SyntaxFile::SysML(SysMLFile {
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
    });
    let ranges = extract_folding_ranges(&file);
    assert_eq!(ranges.len(), 2);
    assert!(ranges[0].span.start.line < ranges[1].span.start.line);
}

// =============================================================================
// KerML folding extraction tests
// =============================================================================

#[test]
fn test_kerml_extract_folding_ranges_empty_file() {
    let file = SyntaxFile::KerML(KerMLFile {
        namespace: None,
        elements: vec![],
    });
    let ranges = extract_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_kerml_extract_folding_ranges_filters_single_line() {
    let file = SyntaxFile::KerML(KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 1)), // Same line = not foldable
        })],
    });
    let ranges = extract_folding_ranges(&file);
    assert!(ranges.is_empty());
}

#[test]
fn test_kerml_extract_folding_ranges_includes_multiline() {
    let file = SyntaxFile::KerML(KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Test".to_string()),
            elements: vec![],
            span: Some(make_span(1, 5)), // Multi-line = foldable
        })],
    });
    let ranges = extract_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert!(!ranges[0].is_comment); // Package is not a comment
}

#[test]
fn test_kerml_extract_folding_ranges_comment_kind() {
    let file = SyntaxFile::KerML(KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Comment(KerMLComment {
            content: "A long comment".to_string(),
            about: vec![],
            locale: None,
            span: Some(make_span(1, 3)),
        })],
    });
    let ranges = extract_folding_ranges(&file);
    assert_eq!(ranges.len(), 1);
    assert!(ranges[0].is_comment); // Comment should be marked as comment
}

#[test]
fn test_kerml_extract_folding_ranges_sorted_by_line() {
    let file = SyntaxFile::KerML(KerMLFile {
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
    });
    let ranges = extract_folding_ranges(&file);
    assert_eq!(ranges.len(), 2);
    assert!(ranges[0].span.start.line < ranges[1].span.start.line);
}
