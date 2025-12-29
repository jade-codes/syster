#![allow(clippy::unwrap_used)]

//! Tests for `collect_containing_spans` function in selection module.

use super::selection::collect_containing_spans;
use crate::core::{Position, Span};
use crate::syntax::kerml::ast::{
    Annotation, Classifier, ClassifierKind, ClassifierMember, Comment, Element, Feature,
    FeatureMember, Import, ImportKind, Package,
};

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Option<Span> {
    Some(Span {
        start: Position {
            line: start_line,
            column: start_col,
        },
        end: Position {
            line: end_line,
            column: end_col,
        },
    })
}

fn make_position(line: usize, column: usize) -> Position {
    Position { line, column }
}

// =============================================================================
// Tests for simple elements (Comment, Import, Annotation)
// =============================================================================

#[test]
fn test_collect_comment_position_inside() {
    let comment = Comment {
        content: "Test comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(5, 0, 5, 20),
    };
    let element = Element::Comment(comment);
    let position = make_position(5, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 5);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_collect_comment_position_outside() {
    let comment = Comment {
        content: "Test comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(5, 0, 5, 20),
    };
    let element = Element::Comment(comment);
    let position = make_position(10, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(!result);
    assert_eq!(spans.len(), 0);
}

#[test]
fn test_collect_comment_no_span() {
    let comment = Comment {
        content: "Test comment".to_string(),
        about: vec![],
        locale: None,
        span: None,
    };
    let element = Element::Comment(comment);
    let position = make_position(5, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(!result);
    assert_eq!(spans.len(), 0);
}

#[test]
fn test_collect_import_position_inside() {
    let import = Import {
        path: "Package::Test".to_string(),
        is_recursive: false,
        kind: ImportKind::Normal,
        span: make_span(2, 0, 2, 30),
    };
    let element = Element::Import(import);
    let position = make_position(2, 15);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 2);
    assert_eq!(spans[0].end.line, 2);
}

#[test]
fn test_collect_annotation_position_inside() {
    let annotation = Annotation {
        reference: "TestAnnotation".to_string(),
        span: make_span(1, 0, 1, 25),
    };
    let element = Element::Annotation(annotation);
    let position = make_position(1, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

// =============================================================================
// Tests for Package elements
// =============================================================================

#[test]
fn test_collect_package_position_inside_empty() {
    let package = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![],
        span: make_span(0, 0, 5, 1),
    };
    let element = Element::Package(package);
    let position = make_position(2, 5);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 0);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_collect_package_with_nested_comment() {
    let comment = Comment {
        content: "Inner comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(2, 4, 2, 25),
    };
    let package = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![Element::Comment(comment)],
        span: make_span(0, 0, 5, 1),
    };
    let element = Element::Package(package);
    let position = make_position(2, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 2); // Package + Comment
    assert_eq!(spans[0].start.line, 0); // Package span
    assert_eq!(spans[1].start.line, 2); // Comment span
}

#[test]
fn test_collect_package_with_nested_elements_early_return() {
    let comment1 = Comment {
        content: "First comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(2, 4, 2, 25),
    };
    let comment2 = Comment {
        content: "Second comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(3, 4, 3, 30),
    };
    let package = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![Element::Comment(comment1), Element::Comment(comment2)],
        span: make_span(0, 0, 5, 1),
    };
    let element = Element::Package(package);
    let position = make_position(2, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    // Should collect package + first comment and return early
    assert!(result);
    assert_eq!(spans.len(), 2); // Package + first Comment only
    assert_eq!(spans[0].start.line, 0);
    assert_eq!(spans[1].start.line, 2);
}

#[test]
fn test_collect_package_position_outside() {
    let package = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![],
        span: make_span(0, 0, 5, 1),
    };
    let element = Element::Package(package);
    let position = make_position(10, 5);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(!result);
    assert_eq!(spans.len(), 0);
}

// =============================================================================
// Tests for Classifier elements
// =============================================================================

#[test]
fn test_collect_classifier_position_inside_empty() {
    let classifier = Classifier {
        kind: ClassifierKind::Class,
        is_abstract: false,
        name: Some("TestClass".to_string()),
        body: vec![],
        span: make_span(10, 0, 15, 1),
    };
    let element = Element::Classifier(classifier);
    let position = make_position(12, 5);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 10);
    assert_eq!(spans[0].end.line, 15);
}

#[test]
fn test_collect_classifier_with_feature() {
    let feature = Feature {
        name: Some("testFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: make_span(12, 4, 12, 30),
    };
    let classifier = Classifier {
        kind: ClassifierKind::Class,
        is_abstract: false,
        name: Some("TestClass".to_string()),
        body: vec![ClassifierMember::Feature(feature)],
        span: make_span(10, 0, 15, 1),
    };
    let element = Element::Classifier(classifier);
    let position = make_position(12, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 2); // Classifier + Feature
    assert_eq!(spans[0].start.line, 10); // Classifier
    assert_eq!(spans[1].start.line, 12); // Feature
}

#[test]
fn test_collect_classifier_with_comment() {
    let comment = Comment {
        content: "Feature comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(12, 4, 12, 30),
    };
    let classifier = Classifier {
        kind: ClassifierKind::Class,
        is_abstract: false,
        name: Some("TestClass".to_string()),
        body: vec![ClassifierMember::Comment(comment)],
        span: make_span(10, 0, 15, 1),
    };
    let element = Element::Classifier(classifier);
    let position = make_position(12, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 2); // Classifier + Comment
}

#[test]
fn test_collect_classifier_position_outside() {
    let classifier = Classifier {
        kind: ClassifierKind::Class,
        is_abstract: false,
        name: Some("TestClass".to_string()),
        body: vec![],
        span: make_span(10, 0, 15, 1),
    };
    let element = Element::Classifier(classifier);
    let position = make_position(20, 5);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(!result);
    assert_eq!(spans.len(), 0);
}

#[test]
fn test_collect_classifier_ignores_non_feature_non_comment_members() {
    let classifier = Classifier {
        kind: ClassifierKind::Class,
        is_abstract: false,
        name: Some("TestClass".to_string()),
        body: vec![ClassifierMember::Import(Import {
            path: "Test::Import".to_string(),
            is_recursive: false,
            kind: ImportKind::Normal,
            span: make_span(12, 4, 12, 30),
        })],
        span: make_span(10, 0, 15, 1),
    };
    let element = Element::Classifier(classifier);
    let position = make_position(12, 10);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    // Should only collect classifier span, not the import (covered by _ => {})
    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 10);
}

// =============================================================================
// Tests for Feature elements
// =============================================================================

#[test]
fn test_collect_feature_position_inside_empty() {
    let feature = Feature {
        name: Some("testFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: make_span(20, 4, 20, 30),
    };
    let element = Element::Feature(feature);
    let position = make_position(20, 15);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 20);
}

#[test]
fn test_collect_feature_with_nested_comment() {
    let comment = Comment {
        content: "Feature member comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(21, 8, 21, 40),
    };
    let feature = Feature {
        name: Some("testFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![FeatureMember::Comment(comment)],
        span: make_span(20, 4, 22, 5),
    };
    let element = Element::Feature(feature);
    let position = make_position(21, 20);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 2); // Feature + Comment
    assert_eq!(spans[0].start.line, 20);
    assert_eq!(spans[1].start.line, 21);
}

#[test]
fn test_collect_feature_position_outside() {
    let feature = Feature {
        name: Some("testFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: make_span(20, 4, 20, 30),
    };
    let element = Element::Feature(feature);
    let position = make_position(25, 15);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(!result);
    assert_eq!(spans.len(), 0);
}

// =============================================================================
// Tests for deeply nested structures
// =============================================================================

#[test]
fn test_collect_deeply_nested_spans() {
    let inner_comment = Comment {
        content: "Deep comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(14, 12, 14, 35),
    };
    let feature = Feature {
        name: Some("innerFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![FeatureMember::Comment(inner_comment)],
        span: make_span(13, 8, 15, 9),
    };
    let classifier = Classifier {
        kind: ClassifierKind::Class,
        is_abstract: false,
        name: Some("InnerClass".to_string()),
        body: vec![ClassifierMember::Feature(feature)],
        span: make_span(12, 4, 16, 5),
    };
    let package = Package {
        name: Some("OuterPackage".to_string()),
        elements: vec![Element::Classifier(classifier)],
        span: make_span(10, 0, 18, 1),
    };
    let element = Element::Package(package);
    let position = make_position(14, 20);
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 4); // Package + Classifier + Feature + Comment
    assert_eq!(spans[0].start.line, 10); // Package
    assert_eq!(spans[1].start.line, 12); // Classifier
    assert_eq!(spans[2].start.line, 13); // Feature
    assert_eq!(spans[3].start.line, 14); // Comment
}

#[test]
fn test_collect_multiple_elements_stops_at_first_match() {
    let comment1 = Comment {
        content: "First".to_string(),
        about: vec![],
        locale: None,
        span: make_span(2, 0, 2, 20),
    };
    let comment2 = Comment {
        content: "Second".to_string(),
        about: vec![],
        locale: None,
        span: make_span(4, 0, 4, 20),
    };
    let package = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![Element::Comment(comment1), Element::Comment(comment2)],
        span: make_span(0, 0, 6, 1),
    };
    let element = Element::Package(package);
    let position = make_position(4, 10); // In second comment
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    // Should stop after finding second comment (early return)
    assert!(result);
    // Package span + second comment span (first comment is skipped)
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 0); // Package
    assert_eq!(spans[1].start.line, 4); // Second comment
}

// =============================================================================
// Edge case tests
// =============================================================================

#[test]
fn test_collect_position_at_span_boundary_start() {
    let comment = Comment {
        content: "Boundary test".to_string(),
        about: vec![],
        locale: None,
        span: make_span(5, 10, 5, 30),
    };
    let element = Element::Comment(comment);
    let position = make_position(5, 10); // Exactly at start
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_position_at_span_boundary_end() {
    let comment = Comment {
        content: "Boundary test".to_string(),
        about: vec![],
        locale: None,
        span: make_span(5, 10, 5, 30),
    };
    let element = Element::Comment(comment);
    let position = make_position(5, 30); // Exactly at end
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_multiline_span() {
    let comment = Comment {
        content: "Multiline comment".to_string(),
        about: vec![],
        locale: None,
        span: make_span(10, 0, 15, 20),
    };
    let element = Element::Comment(comment);
    let position = make_position(12, 10); // Middle line
    let mut spans = Vec::new();

    let result = collect_containing_spans(&element, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 10);
    assert_eq!(spans[0].end.line, 15);
}
