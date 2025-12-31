#![allow(clippy::unwrap_used)]

use crate::core::{ParseError, ParseErrorKind, ParseResult};

#[allow(dead_code)]
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

#[test]
fn test_is_ok_with_success() {
    let content = TestData {
        value: "test".to_string(),
    };
    let result = ParseResult::success(content);

    assert!(result.is_ok());
}

#[test]
fn test_is_ok_with_single_error() {
    let error = ParseError::syntax_error("error message", 1, 5);
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![error]);

    assert!(!result.is_ok());
}

#[test]
fn test_is_ok_with_multiple_errors() {
    let errors = vec![
        ParseError::syntax_error("first error", 1, 0),
        ParseError::ast_error("second error", 2, 10),
        ParseError::syntax_error("third error", 3, 5),
    ];
    let result: ParseResult<TestData> = ParseResult::with_errors(errors);

    assert!(!result.is_ok());
}

#[test]
fn test_is_ok_with_empty_errors() {
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![]);

    // Even with empty errors list, is_ok should return true
    assert!(result.is_ok());
}

// ============================================================================
// Tests for ParseResult::has_errors (#369)
// ============================================================================

#[test]
fn test_has_errors_with_no_errors() {
    let content = TestData {
        value: "test".to_string(),
    };
    let result = ParseResult::success(content);

    assert!(!result.has_errors());
}

#[test]
fn test_has_errors_with_single_error() {
    let error = ParseError::syntax_error("syntax error", 5, 10);
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![error]);

    assert!(result.has_errors());
}

#[test]
fn test_has_errors_with_multiple_errors() {
    let errors = vec![
        ParseError::syntax_error("error 1", 1, 0),
        ParseError::ast_error("error 2", 5, 15),
    ];
    let result: ParseResult<TestData> = ParseResult::with_errors(errors);

    assert!(result.has_errors());
}

#[test]
fn test_has_errors_with_empty_errors() {
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![]);

    assert!(!result.has_errors());
}

// ============================================================================
// Tests for ParseResult::with_errors (#367)
// ============================================================================

#[test]
fn test_with_errors_empty_list() {
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![]);

    assert!(result.content.is_none());
    assert_eq!(result.errors.len(), 0);
    assert!(result.is_ok());
}

#[test]
fn test_with_errors_single_error() {
    let error = ParseError::syntax_error("test error", 10, 20);
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![error.clone()]);

    assert!(result.content.is_none());
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].message, "test error");
    assert_eq!(result.errors[0].position.line, 10);
    assert_eq!(result.errors[0].position.column, 20);
}

#[test]
fn test_with_errors_multiple_errors() {
    let errors = vec![
        ParseError::syntax_error("syntax error", 1, 5),
        ParseError::ast_error("ast error", 2, 10),
        ParseError::syntax_error("another syntax error", 3, 15),
    ];
    let result: ParseResult<TestData> = ParseResult::with_errors(errors);

    assert!(result.content.is_none());
    assert_eq!(result.errors.len(), 3);
    assert_eq!(result.errors[0].message, "syntax error");
    assert_eq!(result.errors[1].message, "ast error");
    assert_eq!(result.errors[2].message, "another syntax error");
}

#[test]
fn test_with_errors_content_is_none() {
    let error = ParseError::syntax_error("error", 0, 0);
    let result: ParseResult<TestData> = ParseResult::with_errors(vec![error]);

    // Verify content is None when errors are present
    assert!(result.content.is_none());
    assert!(result.has_errors());
}

// ============================================================================
// Tests for ParseError::syntax_error (#365)
// ============================================================================

#[test]
fn test_syntax_error_with_str() {
    let error = ParseError::syntax_error("test message", 5, 10);

    assert_eq!(error.message, "test message");
    assert_eq!(error.position.line, 5);
    assert_eq!(error.position.column, 10);
    assert_eq!(error.kind, ParseErrorKind::SyntaxError);
}

#[test]
fn test_syntax_error_with_string() {
    let message = String::from("error from string");
    let error = ParseError::syntax_error(message, 15, 25);

    assert_eq!(error.message, "error from string");
    assert_eq!(error.position.line, 15);
    assert_eq!(error.position.column, 25);
    assert_eq!(error.kind, ParseErrorKind::SyntaxError);
}

#[test]
fn test_syntax_error_with_zero_position() {
    let error = ParseError::syntax_error("error at start", 0, 0);

    assert_eq!(error.message, "error at start");
    assert_eq!(error.position.line, 0);
    assert_eq!(error.position.column, 0);
    assert_eq!(error.kind, ParseErrorKind::SyntaxError);
}

#[test]
fn test_syntax_error_with_large_position() {
    let error = ParseError::syntax_error("error far away", 1000, 500);

    assert_eq!(error.message, "error far away");
    assert_eq!(error.position.line, 1000);
    assert_eq!(error.position.column, 500);
    assert_eq!(error.kind, ParseErrorKind::SyntaxError);
}

#[test]
fn test_syntax_error_with_empty_message() {
    let error = ParseError::syntax_error("", 1, 1);

    assert_eq!(error.message, "");
    assert_eq!(error.position.line, 1);
    assert_eq!(error.position.column, 1);
    assert_eq!(error.kind, ParseErrorKind::SyntaxError);
}

#[test]
fn test_syntax_error_into_conversion() {
    // Test that Into trait works properly for various types
    let error1 = ParseError::syntax_error("str slice", 1, 2);
    let error2 = ParseError::syntax_error(String::from("owned string"), 3, 4);
    let error3 = ParseError::syntax_error("borrowed".to_string(), 5, 6);

    assert_eq!(error1.message, "str slice");
    assert_eq!(error2.message, "owned string");
    assert_eq!(error3.message, "borrowed");
}

// ============================================================================
// Tests for ParseError::ast_error (#364)
// ============================================================================

#[test]
fn test_ast_error_with_str() {
    let error = ParseError::ast_error("ast construction failed", 7, 14);

    assert_eq!(error.message, "ast construction failed");
    assert_eq!(error.position.line, 7);
    assert_eq!(error.position.column, 14);
    assert_eq!(error.kind, ParseErrorKind::AstError);
}

#[test]
fn test_ast_error_with_string() {
    let message = String::from("invalid ast node");
    let error = ParseError::ast_error(message, 20, 30);

    assert_eq!(error.message, "invalid ast node");
    assert_eq!(error.position.line, 20);
    assert_eq!(error.position.column, 30);
    assert_eq!(error.kind, ParseErrorKind::AstError);
}

#[test]
fn test_ast_error_with_zero_position() {
    let error = ParseError::ast_error("ast error at start", 0, 0);

    assert_eq!(error.message, "ast error at start");
    assert_eq!(error.position.line, 0);
    assert_eq!(error.position.column, 0);
    assert_eq!(error.kind, ParseErrorKind::AstError);
}

#[test]
fn test_ast_error_with_large_position() {
    let error = ParseError::ast_error("ast error far away", 9999, 8888);

    assert_eq!(error.message, "ast error far away");
    assert_eq!(error.position.line, 9999);
    assert_eq!(error.position.column, 8888);
    assert_eq!(error.kind, ParseErrorKind::AstError);
}

#[test]
fn test_ast_error_with_empty_message() {
    let error = ParseError::ast_error("", 2, 3);

    assert_eq!(error.message, "");
    assert_eq!(error.position.line, 2);
    assert_eq!(error.position.column, 3);
    assert_eq!(error.kind, ParseErrorKind::AstError);
}

#[test]
fn test_ast_error_into_conversion() {
    // Test that Into trait works properly for various types
    let error1 = ParseError::ast_error("str slice", 1, 2);
    let error2 = ParseError::ast_error(String::from("owned string"), 3, 4);
    let error3 = ParseError::ast_error("borrowed".to_string(), 5, 6);

    assert_eq!(error1.message, "str slice");
    assert_eq!(error2.message, "owned string");
    assert_eq!(error3.message, "borrowed");
}

// ============================================================================
// Additional edge case tests for comprehensive coverage
// ============================================================================

#[test]
fn test_error_kind_difference() {
    let syntax_err = ParseError::syntax_error("syntax", 1, 1);
    let ast_err = ParseError::ast_error("ast", 1, 1);

    assert_eq!(syntax_err.kind, ParseErrorKind::SyntaxError);
    assert_eq!(ast_err.kind, ParseErrorKind::AstError);
    assert_ne!(syntax_err.kind, ast_err.kind);
}

#[test]
fn test_parse_result_success_and_with_errors_difference() {
    let success = ParseResult::success(TestData {
        value: "test".to_string(),
    });
    let with_errors: ParseResult<TestData> =
        ParseResult::with_errors(vec![ParseError::syntax_error("error", 1, 1)]);

    assert!(success.content.is_some());
    assert!(with_errors.content.is_none());
    assert!(success.is_ok());
    assert!(!with_errors.is_ok());
}

#[test]
fn test_multiple_error_types_mixed() {
    let errors = vec![
        ParseError::syntax_error("syntax 1", 1, 0),
        ParseError::ast_error("ast 1", 2, 5),
        ParseError::syntax_error("syntax 2", 3, 10),
        ParseError::ast_error("ast 2", 4, 15),
    ];
    let result: ParseResult<TestData> = ParseResult::with_errors(errors);

    assert_eq!(result.errors.len(), 4);
    assert_eq!(result.errors[0].kind, ParseErrorKind::SyntaxError);
    assert_eq!(result.errors[1].kind, ParseErrorKind::AstError);
    assert_eq!(result.errors[2].kind, ParseErrorKind::SyntaxError);
    assert_eq!(result.errors[3].kind, ParseErrorKind::AstError);
}
