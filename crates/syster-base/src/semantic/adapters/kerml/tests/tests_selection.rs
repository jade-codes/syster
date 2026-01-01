//! Comprehensive tests for the kerml-selection module
//!
//! This test file covers all functions in crate::semantic::adapters::kerml::selection:
//! - range_size (private, tested through sorting behavior)
//! - try_push_span (private, tested through various element tests)
//! - collect_containing_spans (private, tested through public API)
//! - collect_package_spans (private, tested through public API)
//! - collect_classifier_spans (private, tested through public API)
//! - find_selection_spans (public API, tested directly)
//!
//! All tests follow the principle of testing through the public API only.

use crate::core::{Position, Span};
use crate::semantic::adapters::selection::find_kerml_selection_spans;
use crate::syntax::kerml::ast::{
    Annotation, Classifier, ClassifierKind, ClassifierMember, Element, Feature, FeatureMember,
    Import, ImportKind, KerMLFile, Package,
};
use crate::syntax::kerml::model::types::Comment;

// =============================================================================
// Helper Functions
// =============================================================================

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

fn make_package(name: &str, span: Option<Span>, elements: Vec<Element>) -> Package {
    Package {
        name: Some(name.to_string()),
        elements,
        span,
    }
}

fn make_classifier(
    kind: ClassifierKind,
    name: &str,
    span: Option<Span>,
    body: Vec<ClassifierMember>,
) -> Classifier {
    Classifier {
        kind,
        is_abstract: false,
        name: Some(name.to_string()),
        body,
        span,
    }
}

fn make_feature(name: &str, span: Option<Span>, body: Vec<FeatureMember>) -> Feature {
    Feature {
        name: Some(name.to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body,
        span,
    }
}

fn make_comment(content: &str, span: Option<Span>) -> Comment {
    Comment {
        content: content.to_string(),
        about: vec![],
        locale: None,
        span,
    }
}

fn make_import(path: &str, span: Option<Span>) -> Import {
    Import {
        path: path.to_string(),
        path_span: None,
        is_recursive: false,
        kind: ImportKind::Normal,
        span,
    }
}

fn make_annotation(reference: &str, span: Option<Span>) -> Annotation {
    Annotation {
        reference: reference.to_string(),
        span,
    }
}

// =============================================================================
// Tests for find_selection_spans (public API)
// =============================================================================

#[test]
fn test_find_selection_spans_empty_file() {
    // Test with empty file - should return empty vector
    let file = KerMLFile {
        namespace: None,
        elements: vec![],
    };
    let pos = Position::new(1, 5);
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_find_selection_spans_position_outside_all_elements() {
    // Test position not contained in any element
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(10, 5); // Outside package
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_find_selection_spans_single_element() {
    // Test single package containing the position
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(3, 5); // Inside package
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_find_selection_spans_stops_at_first_containing_element() {
    // Test that iteration stops when a containing element is found
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            Element::Package(make_package("First", Some(make_span(1, 0, 3, 1)), vec![])),
            Element::Package(make_package("Second", Some(make_span(5, 0, 8, 1)), vec![])),
        ],
    };
    let pos = Position::new(2, 5); // Inside first package
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
    assert_eq!(spans[0].end.line, 3);
}

// =============================================================================
// Tests for range_size (through sorting behavior)
// =============================================================================

#[test]
fn test_range_size_sorting_single_line_spans() {
    // Test that smaller single-line spans come first
    let inner = Element::Package(make_package("Inner", Some(make_span(2, 5, 2, 15)), vec![]));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(2, 0, 2, 20)),
        vec![inner],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![outer],
    };
    let pos = Position::new(2, 10); // Inside both
    let spans = find_kerml_selection_spans(&file, pos);

    // Inner (smaller) should be first
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.column, 5); // Inner
    assert_eq!(spans[1].start.column, 0); // Outer
}

#[test]
fn test_range_size_sorting_multi_line_spans() {
    // Test that spans with fewer lines come first
    let inner = Element::Feature(make_feature("Inner", Some(make_span(2, 0, 3, 10)), vec![]));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(1, 0, 5, 10)),
        vec![inner],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![outer],
    };
    let pos = Position::new(2, 5); // Inside both
    let spans = find_kerml_selection_spans(&file, pos);

    // Inner (smaller - 2 lines) should be first
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Inner
    assert_eq!(spans[1].start.line, 1); // Outer
}

#[test]
fn test_range_size_sorting_three_nested_spans() {
    // Test sorting with three levels of nesting
    let innermost = Comment {
        content: "comment".to_string(),
        about: vec![],
        locale: None,
        span: Some(make_span(3, 5, 3, 15)),
    };
    let middle = Element::Feature(make_feature(
        "Middle",
        Some(make_span(2, 0, 4, 10)),
        vec![FeatureMember::Comment(innermost)],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(1, 0, 6, 10)),
        vec![middle],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![outer],
    };
    let pos = Position::new(3, 10); // Inside all three
    let spans = find_kerml_selection_spans(&file, pos);

    // Should be sorted: innermost, middle, outer
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].start.line, 3); // Innermost comment
    assert_eq!(spans[1].start.line, 2); // Middle feature
    assert_eq!(spans[2].start.line, 1); // Outer package
}

// =============================================================================
// Tests for try_push_span (through Element types)
// =============================================================================

#[test]
fn test_try_push_span_with_none_span() {
    // Test that elements with None span are not collected
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package("Test", None, vec![]))],
    };
    let pos = Position::new(2, 5);
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_try_push_span_with_comment_element() {
    // Test Comment element through try_push_span
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Comment(make_comment(
            "Test comment",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = Position::new(1, 10);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_with_import_element() {
    // Test Import element through try_push_span
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Import(make_import(
            "Test::Package",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = Position::new(1, 10);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_with_annotation_element() {
    // Test Annotation element through try_push_span
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Annotation(make_annotation(
            "TestAnnotation",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = Position::new(1, 10);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_position_at_boundary_start() {
    // Test position exactly at start of span
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = Position::new(5, 10); // Exactly at start
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_try_push_span_position_at_boundary_end() {
    // Test position exactly at end of span
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = Position::new(5, 20); // Exactly at end
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_try_push_span_position_before_span() {
    // Test position just before span
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = Position::new(5, 9); // Just before start
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_try_push_span_position_after_span() {
    // Test position just after span
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = Position::new(5, 21); // Just after end
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

// =============================================================================
// Tests for collect_containing_spans (through various Element types)
// =============================================================================

#[test]
fn test_collect_containing_spans_dispatches_to_package() {
    // Test that Package elements are handled correctly
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(3, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_containing_spans_dispatches_to_classifier() {
    // Test that Classifier elements are handled correctly
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Classifier(make_classifier(
            ClassifierKind::Class,
            "TestClass",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(3, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_containing_spans_dispatches_to_feature() {
    // Test that Feature elements are handled correctly
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(make_feature(
            "testFeature",
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(2, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_containing_spans_with_mixed_elements() {
    // Test with multiple different element types
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            Element::Comment(make_comment("Comment", Some(make_span(1, 0, 1, 10)))),
            Element::Import(make_import("Import", Some(make_span(2, 0, 2, 10)))),
            Element::Package(make_package(
                "Package",
                Some(make_span(3, 0, 5, 10)),
                vec![],
            )),
        ],
    };
    let pos = Position::new(4, 5); // Inside package
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 3);
}

// =============================================================================
// Tests for collect_package_spans
// =============================================================================

#[test]
fn test_collect_package_spans_empty_package() {
    // Test package with no children
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package(
            "Empty",
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(2, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_package_spans_with_nested_package() {
    // Test package containing another package
    let inner = Element::Package(make_package("Inner", Some(make_span(2, 2, 3, 3)), vec![]));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(1, 0, 5, 1)),
        vec![inner],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![outer],
    };
    let pos = Position::new(2, 5); // Inside inner package
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have both packages
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Inner (smaller)
    assert_eq!(spans[1].start.line, 1); // Outer (larger)
}

#[test]
fn test_collect_package_spans_with_classifier_child() {
    // Test package containing a classifier
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::Class,
        "Child",
        Some(make_span(2, 2, 4, 3)),
        vec![],
    ));
    let package = Element::Package(make_package(
        "Parent",
        Some(make_span(1, 0, 5, 1)),
        vec![classifier],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };
    let pos = Position::new(3, 2); // Inside classifier
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have both package and classifier
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Classifier (smaller)
    assert_eq!(spans[1].start.line, 1); // Package (larger)
}

#[test]
fn test_collect_package_spans_with_feature_child() {
    // Test package containing a feature
    let feature = Element::Feature(make_feature("Child", Some(make_span(2, 2, 3, 3)), vec![]));
    let package = Element::Package(make_package(
        "Parent",
        Some(make_span(1, 0, 5, 1)),
        vec![feature],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };
    let pos = Position::new(2, 5); // Inside feature
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have both package and feature
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Feature (smaller)
    assert_eq!(spans[1].start.line, 1); // Package (larger)
}

#[test]
fn test_collect_package_spans_with_comment_child() {
    // Test package containing a comment
    let comment = Element::Comment(make_comment("Comment", Some(make_span(2, 2, 2, 10))));
    let package = Element::Package(make_package(
        "Parent",
        Some(make_span(1, 0, 5, 1)),
        vec![comment],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };
    let pos = Position::new(2, 5); // Inside comment
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have both package and comment
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Comment (smaller)
    assert_eq!(spans[1].start.line, 1); // Package (larger)
}

#[test]
fn test_collect_package_spans_stops_at_first_matching_child() {
    // Test that iteration stops when a child contains the position
    let child1 = Element::Feature(make_feature("First", Some(make_span(2, 0, 3, 1)), vec![]));
    let child2 = Element::Feature(make_feature("Second", Some(make_span(4, 0, 5, 1)), vec![]));
    let package = Element::Package(make_package(
        "Parent",
        Some(make_span(1, 0, 6, 1)),
        vec![child1, child2],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };
    let pos = Position::new(2, 5); // Inside first child
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have package and first child only
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // First child
    assert_eq!(spans[1].start.line, 1); // Package
}

#[test]
fn test_collect_package_spans_returns_false_when_span_not_containing() {
    // Test that collect_package_spans returns false when package doesn't contain position
    let package = Element::Package(make_package("Test", Some(make_span(1, 0, 3, 1)), vec![]));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };
    let pos = Position::new(5, 0); // Outside package
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_collect_package_spans_with_multiple_children_none_matching() {
    // Test package with children but position not in any child
    let child1 = Element::Feature(make_feature("First", Some(make_span(2, 0, 2, 10)), vec![]));
    let child2 = Element::Feature(make_feature("Second", Some(make_span(3, 0, 3, 10)), vec![]));
    let package = Element::Package(make_package(
        "Parent",
        Some(make_span(1, 0, 5, 1)),
        vec![child1, child2],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };
    let pos = Position::new(4, 5); // Inside package but not in any child
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have only package
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

// =============================================================================
// Tests for collect_classifier_spans
// =============================================================================

#[test]
fn test_collect_classifier_spans_empty_body() {
    // Test classifier with no body members
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Classifier(make_classifier(
            ClassifierKind::Class,
            "Empty",
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(2, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_classifier_spans_with_feature_member() {
    // Test classifier containing a feature
    let feature = make_feature("member", Some(make_span(2, 2, 3, 3)), vec![]);
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::Class,
        "Parent",
        Some(make_span(1, 0, 5, 1)),
        vec![ClassifierMember::Feature(feature)],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![classifier],
    };
    let pos = Position::new(2, 5); // Inside feature
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have both classifier and feature
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Feature (smaller)
    assert_eq!(spans[1].start.line, 1); // Classifier (larger)
}

#[test]
fn test_collect_classifier_spans_with_comment_member() {
    // Test classifier containing a comment
    let comment = make_comment("Comment", Some(make_span(2, 2, 2, 10)));
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::DataType,
        "Parent",
        Some(make_span(1, 0, 5, 1)),
        vec![ClassifierMember::Comment(comment)],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![classifier],
    };
    let pos = Position::new(2, 5); // Inside comment
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have both classifier and comment
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Comment (smaller)
    assert_eq!(spans[1].start.line, 1); // Classifier (larger)
}

#[test]
fn test_collect_classifier_spans_with_non_feature_non_comment_members() {
    // Test classifier with specialization and import members (not Feature or Comment)
    use crate::syntax::kerml::ast::Specialization;
    let spec = Specialization {
        general: "Base".to_string(),
        span: Some(make_span(2, 2, 2, 10)),
    };
    let import = make_import("Test::Import", Some(make_span(3, 2, 3, 10)));
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::Behavior,
        "Parent",
        Some(make_span(1, 0, 5, 1)),
        vec![
            ClassifierMember::Specialization(spec),
            ClassifierMember::Import(import),
        ],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![classifier],
    };
    let pos = Position::new(2, 5); // Position would be in specialization span
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have only classifier (specialization and import are skipped in match)
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_collect_classifier_spans_stops_at_first_matching_member() {
    // Test that iteration stops when a member contains the position
    let feature1 = make_feature("First", Some(make_span(2, 0, 3, 1)), vec![]);
    let feature2 = make_feature("Second", Some(make_span(4, 0, 5, 1)), vec![]);
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::Structure,
        "Parent",
        Some(make_span(1, 0, 6, 1)),
        vec![
            ClassifierMember::Feature(feature1),
            ClassifierMember::Feature(feature2),
        ],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![classifier],
    };
    let pos = Position::new(2, 5); // Inside first feature
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have classifier and first feature only
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // First feature
    assert_eq!(spans[1].start.line, 1); // Classifier
}

#[test]
fn test_collect_classifier_spans_returns_false_when_not_containing() {
    // Test that collect_classifier_spans returns false when classifier doesn't contain position
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::Function,
        "Test",
        Some(make_span(1, 0, 3, 1)),
        vec![],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![classifier],
    };
    let pos = Position::new(5, 0); // Outside classifier
    let spans = find_kerml_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_collect_classifier_spans_with_different_classifier_kinds() {
    // Test various classifier kinds
    let kinds = vec![
        ClassifierKind::Type,
        ClassifierKind::Classifier,
        ClassifierKind::DataType,
        ClassifierKind::Class,
        ClassifierKind::Structure,
        ClassifierKind::Behavior,
        ClassifierKind::Function,
        ClassifierKind::Association,
        ClassifierKind::AssociationStructure,
        ClassifierKind::Metaclass,
    ];

    for kind in kinds {
        let classifier = Element::Classifier(make_classifier(
            kind.clone(),
            "Test",
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ));
        let file = KerMLFile {
            namespace: None,
            elements: vec![classifier],
        };
        let pos = Position::new(2, 0);
        let spans = find_kerml_selection_spans(&file, pos);

        // All kinds should work the same
        assert_eq!(spans.len(), 1, "Failed for kind {:?}", kind);
    }
}

#[test]
fn test_collect_classifier_spans_with_abstract_classifier() {
    // Test abstract classifier
    let classifier = Classifier {
        kind: ClassifierKind::Class,
        is_abstract: true,
        name: Some("AbstractClass".to_string()),
        body: vec![],
        span: Some(make_span(1, 0, 3, 1)),
    };
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Classifier(classifier)],
    };
    let pos = Position::new(2, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

// =============================================================================
// Edge Cases and Integration Tests
// =============================================================================

#[test]
fn test_deeply_nested_structure() {
    // Test deeply nested package -> classifier -> feature -> comment
    let comment = make_comment("Deep", Some(make_span(4, 6, 4, 15)));
    let feature = make_feature(
        "feat",
        Some(make_span(3, 4, 5, 5)),
        vec![FeatureMember::Comment(comment)],
    );
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::Class,
        "cls",
        Some(make_span(2, 2, 6, 3)),
        vec![ClassifierMember::Feature(feature)],
    ));
    let package = Element::Package(make_package(
        "pkg",
        Some(make_span(1, 0, 7, 1)),
        vec![classifier],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };
    let pos = Position::new(4, 10); // Inside comment

    let spans = find_kerml_selection_spans(&file, pos);

    // Should have all 4 levels
    assert_eq!(spans.len(), 4);
    assert_eq!(spans[0].start.line, 4); // Comment (innermost)
    assert_eq!(spans[1].start.line, 3); // Feature
    assert_eq!(spans[2].start.line, 2); // Classifier
    assert_eq!(spans[3].start.line, 1); // Package (outermost)
}

#[test]
fn test_position_at_zero_zero() {
    // Test position at origin (0, 0)
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(0, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(0, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_multiple_top_level_elements_first_contains() {
    // Test multiple top-level elements where first contains position
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            Element::Package(make_package("First", Some(make_span(1, 0, 3, 1)), vec![])),
            Element::Package(make_package("Second", Some(make_span(5, 0, 7, 1)), vec![])),
        ],
    };
    let pos = Position::new(2, 0); // Inside first
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_multiple_top_level_elements_second_contains() {
    // Test multiple top-level elements where second contains position
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            Element::Package(make_package("First", Some(make_span(1, 0, 3, 1)), vec![])),
            Element::Package(make_package("Second", Some(make_span(5, 0, 7, 1)), vec![])),
        ],
    };
    let pos = Position::new(6, 0); // Inside second
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 5);
}

#[test]
fn test_spans_with_same_start_different_end() {
    // Test spans starting at same position but ending differently
    let inner = Element::Feature(make_feature("Inner", Some(make_span(2, 0, 3, 5)), vec![]));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(2, 0, 5, 10)),
        vec![inner],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![outer],
    };
    let pos = Position::new(2, 5); // Inside both
    let spans = find_kerml_selection_spans(&file, pos);

    // Inner (smaller) should be first
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].end.line, 3); // Inner
    assert_eq!(spans[1].end.line, 5); // Outer
}

#[test]
fn test_comment_in_feature_in_classifier_in_package() {
    // Test complete nesting: package -> classifier -> feature -> comment
    let comment = make_comment("Test", Some(make_span(4, 6, 4, 15)));
    let feature = make_feature(
        "feat",
        Some(make_span(3, 4, 5, 5)),
        vec![FeatureMember::Comment(comment)],
    );
    let classifier = Element::Classifier(make_classifier(
        ClassifierKind::Class,
        "cls",
        Some(make_span(2, 2, 6, 3)),
        vec![ClassifierMember::Feature(feature)],
    ));
    let package = Element::Package(make_package(
        "pkg",
        Some(make_span(1, 0, 7, 1)),
        vec![classifier],
    ));
    let file = KerMLFile {
        namespace: None,
        elements: vec![package],
    };

    // Test position in comment
    let pos = Position::new(4, 10);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 4);

    // Test position in feature but not comment
    let pos = Position::new(3, 4);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 3); // feature, classifier, package

    // Test position in classifier but not feature
    let pos = Position::new(2, 2);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 2); // classifier, package

    // Test position in package but not classifier
    let pos = Position::new(1, 1);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1); // package only
}

#[test]
fn test_large_line_numbers() {
    // Test with large line numbers to verify no overflow issues
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1000, 0, 2000, 1)),
            vec![],
        ))],
    };
    let pos = Position::new(1500, 0);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_large_column_numbers() {
    // Test with large column numbers
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 1, 1000)),
            vec![],
        ))],
    };
    let pos = Position::new(1, 500);
    let spans = find_kerml_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}
