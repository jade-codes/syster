#![allow(clippy::unwrap_used)]

use super::*;

// Simple test struct to avoid depending on syntax layer
#[derive(Debug)]
struct TestData {
    value: String,
}

#[test]
fn test_parse_result_success() {
    let data = TestData {
        value: "test".to_string(),
    };
    let result = ParseResult::success(data);

    assert!(result.is_ok());
    assert!(!result.has_errors());
    assert_eq!(result.errors.len(), 0);
    assert!(result.content.is_some());
}

#[test]
fn test_parse_result_with_errors() {
    let error = ParseError::syntax_error("Test error", 5, 10);
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![error]);

    assert!(!result.is_ok());
    assert!(result.has_errors());
    assert_eq!(result.errors.len(), 1);
    assert!(result.content.is_none());
}

#[test]
fn test_error_position() {
    let error = ParseError::syntax_error("Test", 5, 10);

    assert_eq!(error.position.line, 5);
    assert_eq!(error.position.column, 10);
    assert_eq!(error.kind, ParseErrorKind::SyntaxError);
}
