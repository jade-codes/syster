//! Selection range extraction for KerML files
//!
//! Finds hierarchical selection ranges at a given position in the AST.

use crate::core::{Position, Span};
use crate::syntax::kerml::ast::{
    Classifier, ClassifierMember, Element, Feature, FeatureMember, KerMLFile, Package,
};

/// Find selection spans at a position in a KerML file
///
/// Returns spans ordered from innermost (smallest) to outermost (largest).
/// Returns empty vector if the position is not within any AST node.
pub fn find_selection_spans(file: &KerMLFile, position: Position) -> Vec<Span> {
    let mut spans: Vec<Span> = Vec::new();

    for element in &file.elements {
        if collect_containing_spans(element, position, &mut spans) {
            break;
        }
    }

    if spans.is_empty() {
        return Vec::new();
    }

    // Sort by range size (smallest first for innermost)
    spans.sort_by(|a, b| {
        let size_a = range_size(a);
        let size_b = range_size(b);
        size_a.cmp(&size_b)
    });

    spans
}

/// Calculate a rough "size" of a span for sorting
fn range_size(span: &Span) -> usize {
    let lines = span.end.line.saturating_sub(span.start.line);
    let cols = if lines == 0 {
        span.end.column.saturating_sub(span.start.column)
    } else {
        span.end.column + 100
    };
    lines * 100 + cols
}

/// Try to push a span if it contains the position
fn try_push_span(span: &Option<Span>, position: Position, spans: &mut Vec<Span>) -> bool {
    if let Some(span) = span
        && span.contains(position)
    {
        spans.push(*span);
        return true;
    }
    false
}

/// Recursively collect all spans that contain the position
fn collect_containing_spans(element: &Element, position: Position, spans: &mut Vec<Span>) -> bool {
    match element {
        Element::Package(p) => collect_package_spans(p, position, spans),
        Element::Classifier(c) => collect_classifier_spans(c, position, spans),
        Element::Feature(f) => collect_feature_spans(f, position, spans),
        Element::Comment(c) => try_push_span(&c.span, position, spans),
        Element::Import(i) => try_push_span(&i.span, position, spans),
        Element::Annotation(a) => try_push_span(&a.span, position, spans),
    }
}

fn collect_package_spans(package: &Package, position: Position, spans: &mut Vec<Span>) -> bool {
    if !try_push_span(&package.span, position, spans) {
        return false;
    }

    for child in &package.elements {
        if collect_containing_spans(child, position, spans) {
            return true;
        }
    }

    true
}

fn collect_classifier_spans(
    classifier: &Classifier,
    position: Position,
    spans: &mut Vec<Span>,
) -> bool {
    if !try_push_span(&classifier.span, position, spans) {
        return false;
    }

    for member in &classifier.body {
        match member {
            ClassifierMember::Feature(f) => {
                if collect_feature_spans(f, position, spans) {
                    return true;
                }
            }
            ClassifierMember::Comment(c) => {
                if try_push_span(&c.span, position, spans) {
                    return true;
                }
            }
            _ => {}
        }
    }

    true
}

fn collect_feature_spans(feature: &Feature, position: Position, spans: &mut Vec<Span>) -> bool {
    if !try_push_span(&feature.span, position, spans) {
        return false;
    }

    for member in &feature.body {
        if let FeatureMember::Comment(c) = member
            && try_push_span(&c.span, position, spans)
        {
            return true;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::kerml::ast::{ClassifierKind, Comment, FeatureDirection};

    // Helper to create a span from coordinates
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

    #[test]
    fn test_collect_classifier_spans_position_not_in_classifier() {
        // Classifier at lines 0-5, position outside at line 10
        let classifier = Classifier {
            kind: ClassifierKind::Class,
            is_abstract: false,
            name: Some("TestClass".to_string()),
            body: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        };
        let position = Position::new(10, 5);
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(!result, "Should return false when position is outside classifier");
        assert!(spans.is_empty(), "Should not collect any spans");
    }

    #[test]
    fn test_collect_classifier_spans_position_in_classifier_empty_body() {
        // Classifier at lines 0-5, position inside at line 2
        let classifier = Classifier {
            kind: ClassifierKind::Class,
            is_abstract: false,
            name: Some("TestClass".to_string()),
            body: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        };
        let position = Position::new(2, 5);
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(result, "Should return true when position is inside classifier");
        assert_eq!(spans.len(), 1, "Should collect classifier span");
        assert_eq!(spans[0], make_span(0, 0, 5, 1));
    }

    #[test]
    fn test_collect_classifier_spans_position_in_feature() {
        // Classifier with a feature, position inside the feature
        let feature = Feature {
            name: Some("myFeature".to_string()),
            direction: Some(FeatureDirection::In),
            is_readonly: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(2, 4, 3, 5)),
        };
        let classifier = Classifier {
            kind: ClassifierKind::Class,
            is_abstract: false,
            name: Some("TestClass".to_string()),
            body: vec![ClassifierMember::Feature(feature)],
            span: Some(make_span(0, 0, 5, 1)),
        };
        let position = Position::new(2, 10); // Inside feature span
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(result, "Should return true when position is in feature");
        assert_eq!(spans.len(), 2, "Should collect both classifier and feature spans");
        // Spans should include classifier span and feature span
        assert!(spans.contains(&make_span(0, 0, 5, 1)), "Should include classifier span");
        assert!(spans.contains(&make_span(2, 4, 3, 5)), "Should include feature span");
    }

    #[test]
    fn test_collect_classifier_spans_position_in_comment() {
        // Classifier with a comment, position inside the comment
        let comment = Comment {
            content: "This is a comment".to_string(),
            about: vec![],
            locale: None,
            span: Some(make_span(2, 4, 2, 25)),
        };
        let classifier = Classifier {
            kind: ClassifierKind::Class,
            is_abstract: false,
            name: Some("TestClass".to_string()),
            body: vec![ClassifierMember::Comment(comment)],
            span: Some(make_span(0, 0, 5, 1)),
        };
        let position = Position::new(2, 10); // Inside comment span
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(result, "Should return true when position is in comment");
        assert_eq!(spans.len(), 2, "Should collect both classifier and comment spans");
        assert!(spans.contains(&make_span(0, 0, 5, 1)), "Should include classifier span");
        assert!(spans.contains(&make_span(2, 4, 2, 25)), "Should include comment span");
    }

    #[test]
    fn test_collect_classifier_spans_multiple_members_position_in_first() {
        // Classifier with multiple members, position in the first one
        let feature1 = Feature {
            name: Some("feature1".to_string()),
            direction: None,
            is_readonly: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 4, 2, 5)),
        };
        let feature2 = Feature {
            name: Some("feature2".to_string()),
            direction: None,
            is_readonly: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(3, 4, 4, 5)),
        };
        let classifier = Classifier {
            kind: ClassifierKind::Structure,
            is_abstract: false,
            name: Some("TestStruct".to_string()),
            body: vec![
                ClassifierMember::Feature(feature1),
                ClassifierMember::Feature(feature2),
            ],
            span: Some(make_span(0, 0, 6, 1)),
        };
        let position = Position::new(1, 10); // Inside first feature
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(result, "Should return true when finding matching member");
        assert_eq!(spans.len(), 2, "Should collect classifier and first feature spans");
        assert!(spans.contains(&make_span(0, 0, 6, 1)), "Should include classifier span");
        assert!(spans.contains(&make_span(1, 4, 2, 5)), "Should include first feature span");
        assert!(!spans.contains(&make_span(3, 4, 4, 5)), "Should not include second feature span");
    }

    #[test]
    fn test_collect_classifier_spans_position_between_members() {
        // Classifier with multiple members, position between them (not in any member)
        let feature1 = Feature {
            name: Some("feature1".to_string()),
            direction: None,
            is_readonly: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 4, 2, 5)),
        };
        let feature2 = Feature {
            name: Some("feature2".to_string()),
            direction: None,
            is_readonly: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(4, 4, 5, 5)),
        };
        let classifier = Classifier {
            kind: ClassifierKind::Class,
            is_abstract: true,
            name: Some("AbstractClass".to_string()),
            body: vec![
                ClassifierMember::Feature(feature1),
                ClassifierMember::Feature(feature2),
            ],
            span: Some(make_span(0, 0, 7, 1)),
        };
        let position = Position::new(3, 5); // Between features
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(result, "Should return true for position in classifier body");
        assert_eq!(spans.len(), 1, "Should only collect classifier span");
        assert_eq!(spans[0], make_span(0, 0, 7, 1));
    }

    #[test]
    fn test_collect_classifier_spans_feature_with_nested_comment() {
        // Feature containing a comment, position in nested comment
        let nested_comment = Comment {
            content: "Nested comment".to_string(),
            about: vec![],
            locale: None,
            span: Some(make_span(2, 8, 2, 25)),
        };
        let feature = Feature {
            name: Some("featureWithComment".to_string()),
            direction: None,
            is_readonly: false,
            is_derived: false,
            body: vec![FeatureMember::Comment(nested_comment)],
            span: Some(make_span(2, 4, 3, 5)),
        };
        let classifier = Classifier {
            kind: ClassifierKind::DataType,
            is_abstract: false,
            name: Some("MyDataType".to_string()),
            body: vec![ClassifierMember::Feature(feature)],
            span: Some(make_span(0, 0, 5, 1)),
        };
        let position = Position::new(2, 15); // Inside nested comment
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(result, "Should return true when position is in nested comment");
        assert_eq!(spans.len(), 3, "Should collect classifier, feature, and comment spans");
        assert!(spans.contains(&make_span(0, 0, 5, 1)), "Should include classifier span");
        assert!(spans.contains(&make_span(2, 4, 3, 5)), "Should include feature span");
        assert!(spans.contains(&make_span(2, 8, 2, 25)), "Should include nested comment span");
    }

    #[test]
    fn test_collect_classifier_spans_no_span_on_classifier() {
        // Classifier with None span
        let classifier = Classifier {
            kind: ClassifierKind::Behavior,
            is_abstract: false,
            name: Some("MyBehavior".to_string()),
            body: vec![],
            span: None,
        };
        let position = Position::new(2, 5);
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(!result, "Should return false when classifier has no span");
        assert!(spans.is_empty(), "Should not collect any spans");
    }

    #[test]
    fn test_collect_classifier_spans_various_classifier_kinds() {
        // Test with different classifier kinds to ensure kind doesn't affect behavior
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
            let classifier = Classifier {
                kind: kind.clone(),
                is_abstract: false,
                name: Some("TestClassifier".to_string()),
                body: vec![],
                span: Some(make_span(0, 0, 5, 1)),
            };
            let position = Position::new(2, 5);
            let mut spans = Vec::new();

            let result = collect_classifier_spans(&classifier, position, &mut spans);

            assert!(result, "Should return true for classifier kind {:?}", kind);
            assert_eq!(spans.len(), 1, "Should collect one span for kind {:?}", kind);
        }
    }

    #[test]
    fn test_collect_classifier_spans_abstract_classifier() {
        // Test with abstract classifier
        let classifier = Classifier {
            kind: ClassifierKind::Class,
            is_abstract: true,
            name: Some("AbstractClass".to_string()),
            body: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        };
        let position = Position::new(2, 5);
        let mut spans = Vec::new();

        let result = collect_classifier_spans(&classifier, position, &mut spans);

        assert!(result, "Should work with abstract classifiers");
        assert_eq!(spans.len(), 1);
    }
}
