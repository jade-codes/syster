//! Tests for collect_feature_spans function in selection.rs
//!
//! These tests verify the behavior of feature span collection through the public API.
//! The function `collect_feature_spans` is a private helper that handles Feature nodes
//! in the AST, collecting spans that contain a given position.

use syster::core::{Position, Span};
use syster::semantic::selection::find_kerml_selection_spans;
use syster::syntax::kerml::ast::{Element, Feature, FeatureDirection, FeatureMember, KerMLFile};
use syster::syntax::kerml::model::types::Comment;

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

fn make_feature_with_span(name: &str, span: Option<Span>) -> Feature {
    Feature {
        name: Some(name.to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span,
    }
}

fn make_feature_with_body(name: &str, span: Option<Span>, body: Vec<FeatureMember>) -> Feature {
    Feature {
        name: Some(name.to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body,
        span,
    }
}

#[test]
fn test_feature_with_span_containing_position() {
    // Test that a feature with a span containing the position is collected
    let feature = make_feature_with_span("testFeature", Some(make_span(1, 0, 3, 1)));
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(2, 5); // Inside feature span
    let spans = find_kerml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
    assert_eq!(spans[0].end.line, 3);
}

#[test]
fn test_feature_with_span_not_containing_position() {
    // Test that a feature with a span not containing the position is not collected
    let feature = make_feature_with_span("testFeature", Some(make_span(1, 0, 3, 1)));
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(5, 5); // Outside feature span
    let spans = find_kerml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}

#[test]
fn test_feature_with_no_span() {
    // Test that a feature with None span is not collected
    let feature = make_feature_with_span("testFeature", None);
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(2, 5);
    let spans = find_kerml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}

#[test]
fn test_feature_with_empty_body() {
    // Test that a feature with empty body works correctly
    let feature = make_feature_with_span("emptyFeature", Some(make_span(1, 0, 2, 1)));
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(1, 5); // Inside feature span
    let spans = find_kerml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_feature_with_comment_in_body_containing_position() {
    // Test that a comment inside a feature body is collected when it contains the position
    let comment = Comment {
        content: "Test comment".to_string(),
        about: vec![],
        locale: None,
        span: Some(make_span(2, 2, 2, 20)),
    };

    let feature = make_feature_with_body(
        "featureWithComment",
        Some(make_span(1, 0, 3, 1)),
        vec![FeatureMember::Comment(comment)],
    );

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(2, 10); // Inside comment span
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have 2 spans: feature and comment
    assert_eq!(spans.len(), 2);
    // First should be comment (smaller/inner)
    assert_eq!(spans[0].start.line, 2);
    assert_eq!(spans[0].end.line, 2);
    // Second should be feature (larger/outer)
    assert_eq!(spans[1].start.line, 1);
    assert_eq!(spans[1].end.line, 3);
}

#[test]
fn test_feature_with_comment_in_body_not_containing_position() {
    // Test that a comment inside a feature body is not collected when it doesn't contain the position
    let comment = Comment {
        content: "Test comment".to_string(),
        about: vec![],
        locale: None,
        span: Some(make_span(2, 2, 2, 20)),
    };

    let feature = make_feature_with_body(
        "featureWithComment",
        Some(make_span(1, 0, 4, 1)),
        vec![FeatureMember::Comment(comment)],
    );

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(3, 5); // Inside feature but outside comment
    let spans = find_kerml_selection_spans(&file, pos);

    // Should only have feature span
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
    assert_eq!(spans[0].end.line, 4);
}

#[test]
fn test_feature_with_multiple_comments() {
    // Test feature with multiple comments in body
    let comment1 = Comment {
        content: "First comment".to_string(),
        about: vec![],
        locale: None,
        span: Some(make_span(2, 2, 2, 20)),
    };

    let comment2 = Comment {
        content: "Second comment".to_string(),
        about: vec![],
        locale: None,
        span: Some(make_span(3, 2, 3, 25)),
    };

    let feature = make_feature_with_body(
        "featureWithComments",
        Some(make_span(1, 0, 4, 1)),
        vec![
            FeatureMember::Comment(comment1),
            FeatureMember::Comment(comment2),
        ],
    );

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    // Position in second comment
    let pos = Position::new(3, 10);
    let spans = find_kerml_selection_spans(&file, pos);

    // Should have 2 spans: feature and second comment
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 3); // Second comment (inner)
    assert_eq!(spans[1].start.line, 1); // Feature (outer)
}

#[test]
fn test_feature_with_comment_with_no_span() {
    // Test that a comment with None span is not collected
    let comment = Comment {
        content: "Comment without span".to_string(),
        about: vec![],
        locale: None,
        span: None,
    };

    let feature = make_feature_with_body(
        "featureWithComment",
        Some(make_span(1, 0, 3, 1)),
        vec![FeatureMember::Comment(comment)],
    );

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(2, 5);
    let spans = find_kerml_selection_spans(&file, pos);

    // Should only have feature span
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_feature_with_different_directions() {
    // Test that feature direction doesn't affect span collection
    let feature_in = Feature {
        name: Some("inputFeature".to_string()),
        direction: Some(FeatureDirection::In),
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: Some(make_span(1, 0, 2, 1)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature_in)],
    };

    let pos = Position::new(1, 5);
    let spans = find_kerml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_feature_readonly_and_derived() {
    // Test that readonly and derived flags don't affect span collection
    let feature = Feature {
        name: Some("readonlyFeature".to_string()),
        direction: None,
        is_readonly: true,
        is_derived: true,
        body: vec![],
        span: Some(make_span(1, 0, 2, 1)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(1, 5);
    let spans = find_kerml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_feature_at_span_boundary_start() {
    // Test position exactly at the start of the feature span
    let feature = make_feature_with_span("boundaryFeature", Some(make_span(5, 10, 7, 20)));
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(5, 10); // Exactly at start
    let spans = find_kerml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
}

#[test]
fn test_feature_at_span_boundary_end() {
    // Test position exactly at the end of the feature span
    let feature = make_feature_with_span("boundaryFeature", Some(make_span(5, 10, 7, 20)));
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(7, 20); // Exactly at end
    let spans = find_kerml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
}

#[test]
fn test_feature_just_before_span() {
    // Test position just before the feature span
    let feature = make_feature_with_span("beforeFeature", Some(make_span(5, 10, 7, 20)));
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(5, 9); // Just before start
    let spans = find_kerml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}

#[test]
fn test_feature_just_after_span() {
    // Test position just after the feature span
    let feature = make_feature_with_span("afterFeature", Some(make_span(5, 10, 7, 20)));
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(feature)],
    };

    let pos = Position::new(7, 21); // Just after end
    let spans = find_kerml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}

#[test]
fn test_multiple_features_select_correct_one() {
    // Test that when multiple features exist, only the one containing the position is selected
    let feature1 = make_feature_with_span("feature1", Some(make_span(1, 0, 3, 1)));
    let feature2 = make_feature_with_span("feature2", Some(make_span(5, 0, 7, 1)));
    let feature3 = make_feature_with_span("feature3", Some(make_span(10, 0, 12, 1)));

    let file = KerMLFile {
        namespace: None,
        elements: vec![
            Element::Feature(feature1),
            Element::Feature(feature2),
            Element::Feature(feature3),
        ],
    };

    let pos = Position::new(6, 5); // Inside feature2
    let spans = find_kerml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 5);
    assert_eq!(spans[0].end.line, 7);
}
