#![allow(clippy::unwrap_used)]

use super::super::*;
use crate::core::Position;
use crate::semantic::Severity;

#[test]
fn test_publish_no_errors() {
    // TDD: Empty errors should produce no diagnostics
    let result = ParseResult::success(crate::syntax::sysml::ast::SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    });

    let diagnostics = DiagnosticPublisher::publish(&result, "test.sysml");

    assert_eq!(diagnostics.len(), 0);
}

#[test]
fn test_publish_syntax_error() {
    // TDD: Syntax error converts to diagnostic with code
    let error = ParseError {
        message: "Expected identifier".to_string(),
        position: Position::new(5, 10),
        kind: ParseErrorKind::SyntaxError,
    };
    let result: ParseResult<()> = ParseResult::with_errors(vec![error]);

    let diagnostics = DiagnosticPublisher::publish(&result, "test.sysml");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].severity, Severity::Error);
    assert_eq!(diagnostics[0].location.file, "test.sysml");
    assert_eq!(diagnostics[0].location.range.start.line, 5);
    assert_eq!(diagnostics[0].location.range.start.column, 10);
    assert_eq!(diagnostics[0].code, Some("P001".to_string()));
}

#[test]
fn test_publish_ast_error() {
    // TDD: AST error gets different code
    let error = ParseError {
        message: "Failed to construct AST".to_string(),
        position: Position::new(0, 0),
        kind: ParseErrorKind::AstError,
    };
    let result: ParseResult<()> = ParseResult::with_errors(vec![error]);

    let diagnostics = DiagnosticPublisher::publish(&result, "test.sysml");
    assert_eq!(diagnostics[0].code, Some("P002".to_string()));
}

#[test]
fn test_publish_multiple_errors() {
    // TDD: Multiple errors convert to multiple diagnostics
    let errors = vec![
        ParseError::syntax_error("Error 1", 1, 5),
        ParseError::syntax_error("Error 2", 3, 10),
    ];
    let result: ParseResult<()> = ParseResult::with_errors(errors);

    let diagnostics = DiagnosticPublisher::publish(&result, "test.sysml");
    assert_eq!(diagnostics.len(), 2);
    assert_eq!(diagnostics[0].location.range.start.line, 1);
    assert_eq!(diagnostics[1].location.range.start.line, 3);
}

#[test]
fn test_file_path_in_diagnostic() {
    // TDD: File path is preserved in diagnostic
    let error = ParseError::syntax_error("Test", 0, 0);
    let result: ParseResult<()> = ParseResult::with_errors(vec![error]);

    let diagnostics = DiagnosticPublisher::publish(&result, "src/models/complex.sysml");
    assert_eq!(diagnostics[0].location.file, "src/models/complex.sysml");
}
