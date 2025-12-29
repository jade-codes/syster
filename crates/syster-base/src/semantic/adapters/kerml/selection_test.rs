#![allow(clippy::unwrap_used)]

//! Tests for KerML selection span collection, specifically `collect_feature_spans`.

use super::selection::collect_feature_spans;
use crate::core::{Position, Span};
use crate::syntax::kerml::ast::{Comment, Feature, FeatureMember};

/// Helper to create a test span
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

/// Helper to create a test comment with a span
fn make_comment(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Comment {
    Comment {
        content: "test comment".to_string(),
        about: Vec::new(),
        locale: None,
        span: Some(make_span(start_line, start_col, end_line, end_col)),
    }
}

#[test]
fn test_collect_feature_spans_no_span_returns_false() {
    // Feature without a span should not be collected
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: None,
    };

    let mut spans = Vec::new();
    let position = Position::new(5, 10);

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(!result);
    assert!(spans.is_empty());
}

#[test]
fn test_collect_feature_spans_position_outside_returns_false() {
    // Feature with span that doesn't contain the position
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(5, 10); // Position before feature span

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(!result);
    assert!(spans.is_empty());
}

#[test]
fn test_collect_feature_spans_position_inside_returns_true() {
    // Feature with span containing the position
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(12, 5); // Position inside feature span

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 10);
    assert_eq!(spans[0].end.line, 15);
}

#[test]
fn test_collect_feature_spans_with_comment_not_containing_position() {
    // Feature contains a comment, but position is not in the comment
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![FeatureMember::Comment(make_comment(12, 0, 12, 20))],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(11, 5); // Position in feature but not in comment

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1); // Only feature span, not comment span
    assert_eq!(spans[0].start.line, 10);
}

#[test]
fn test_collect_feature_spans_with_comment_containing_position() {
    // Feature contains a comment that contains the position
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![FeatureMember::Comment(make_comment(12, 0, 12, 20))],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(12, 10); // Position inside the comment

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 2); // Both feature and comment spans
    assert_eq!(spans[0].start.line, 10); // Feature span
    assert_eq!(spans[1].start.line, 12); // Comment span
}

#[test]
fn test_collect_feature_spans_with_multiple_comments() {
    // Feature with multiple comments, position in one of them
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![
            FeatureMember::Comment(make_comment(11, 0, 11, 20)),
            FeatureMember::Comment(make_comment(12, 0, 12, 20)),
            FeatureMember::Comment(make_comment(13, 0, 13, 20)),
        ],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(12, 10); // Position in the second comment

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 2); // Feature span + second comment span
    assert_eq!(spans[0].start.line, 10); // Feature span
    assert_eq!(spans[1].start.line, 12); // Second comment span
}

#[test]
fn test_collect_feature_spans_with_comment_without_span() {
    // Feature with a comment that has no span
    let comment = Comment {
        content: "test comment".to_string(),
        about: Vec::new(),
        locale: None,
        span: None,
    };

    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![FeatureMember::Comment(comment)],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(12, 10);

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1); // Only feature span, comment has no span
    assert_eq!(spans[0].start.line, 10);
}

#[test]
fn test_collect_feature_spans_position_at_boundary() {
    // Test position exactly at the start boundary
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(10, 0); // Exactly at start

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(result);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_feature_spans_stops_at_first_matching_comment() {
    // When a comment contains the position, function should return true and stop
    let feature = Feature {
        name: Some("TestFeature".to_string()),
        direction: None,
        is_readonly: false,
        is_derived: false,
        body: vec![
            FeatureMember::Comment(make_comment(11, 0, 11, 20)),
            FeatureMember::Comment(make_comment(12, 0, 12, 20)), // Position here
            FeatureMember::Comment(make_comment(13, 0, 13, 20)), // Should not reach this
        ],
        span: Some(make_span(10, 0, 15, 0)),
    };

    let mut spans = Vec::new();
    let position = Position::new(12, 10);

    let result = collect_feature_spans(&feature, position, &mut spans);

    assert!(result);
    // Should only have feature span and the matching comment, not subsequent ones
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[1].start.line, 12);
}
