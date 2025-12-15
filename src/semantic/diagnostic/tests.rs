#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn test_diagnostic_creation() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(0, 5), Position::new(0, 10)),
    );

    let diag = Diagnostic::error("Undefined symbol", location.clone());

    assert_eq!(diag.severity, Severity::Error);
    assert_eq!(diag.message, "Undefined symbol");
    assert_eq!(diag.location.file, "test.sysml");
    assert_eq!(diag.location.range.start.line, 0);
    assert_eq!(diag.location.range.start.column, 5);
    assert_eq!(diag.code, None);
}

#[test]
fn test_diagnostic_with_code() {
    let location = Location::new("test.sysml", Range::single(0, 5));

    let diag = Diagnostic::error("Parse error", location).with_code("E001");

    assert_eq!(diag.code, Some("E001".to_string()));
}

#[test]
fn test_warning_diagnostic() {
    let location = Location::new("test.sysml", Range::single(2, 10));

    let diag = Diagnostic::warning("Unused variable", location);

    assert_eq!(diag.severity, Severity::Warning);
}

#[test]
fn test_single_char_range() {
    let range = Range::single(5, 10);

    assert_eq!(range.start.line, 5);
    assert_eq!(range.start.column, 10);
    assert_eq!(range.end.line, 5);
    assert_eq!(range.end.column, 11);
}

#[test]
fn test_diagnostic_display() {
    let location = Location::new("test.sysml", Range::single(0, 5));
    let diag = Diagnostic::error("Test error", location);

    let display = format!("{}", diag);

    assert!(display.contains("test.sysml:1:6")); // 1-indexed display
    assert!(display.contains("Error"));
    assert!(display.contains("Test error"));
}

#[test]
fn test_multiline_range() {
    let range = Range::new(Position::new(0, 5), Position::new(2, 10));

    assert_eq!(range.start.line, 0);
    assert_eq!(range.end.line, 2);
}
