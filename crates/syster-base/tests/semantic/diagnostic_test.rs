#![allow(clippy::unwrap_used)]

use syster::semantic::types::diagnostic::{Diagnostic, Location, Position, Range, Severity};

// ============================================================================
// Tests for Diagnostic::warning (#334)
// ============================================================================

#[test]
fn test_warning_creates_warning_severity() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(0, 0), Position::new(0, 10)),
    );
    let diagnostic = Diagnostic::warning("Test warning", location);

    assert_eq!(diagnostic.severity, Severity::Warning);
}

#[test]
fn test_warning_stores_message() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(0, 0), Position::new(0, 10)),
    );
    let diagnostic = Diagnostic::warning("Test warning message", location);

    assert_eq!(diagnostic.message, "Test warning message");
}

#[test]
fn test_warning_stores_location() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(5, 10), Position::new(5, 20)),
    );
    let diagnostic = Diagnostic::warning("Warning", location.clone());

    assert_eq!(diagnostic.location.file, "test.sysml");
    assert_eq!(diagnostic.location.range.start.line, 5);
    assert_eq!(diagnostic.location.range.start.column, 10);
    assert_eq!(diagnostic.location.range.end.line, 5);
    assert_eq!(diagnostic.location.range.end.column, 20);
}

#[test]
fn test_warning_no_code_by_default() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(0, 0), Position::new(0, 10)),
    );
    let diagnostic = Diagnostic::warning("Warning", location);

    assert_eq!(diagnostic.code, None);
}

#[test]
fn test_warning_with_code() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(0, 0), Position::new(0, 10)),
    );
    let diagnostic = Diagnostic::warning("Warning", location).with_code("W001");

    assert_eq!(diagnostic.severity, Severity::Warning);
    assert_eq!(diagnostic.code, Some("W001".to_string()));
}

#[test]
fn test_warning_accepts_string_types() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(0, 0), Position::new(0, 10)),
    );

    // Test with &str
    let diagnostic1 = Diagnostic::warning("Warning from &str", location.clone());
    assert_eq!(diagnostic1.message, "Warning from &str");

    // Test with String
    let diagnostic2 = Diagnostic::warning("Warning from String".to_string(), location.clone());
    assert_eq!(diagnostic2.message, "Warning from String");

    // Test with owned String
    let msg = format!("Warning {}", 42);
    let diagnostic3 = Diagnostic::warning(msg, location);
    assert_eq!(diagnostic3.message, "Warning 42");
}

#[test]
fn test_warning_multiline_location() {
    let location = Location::new(
        "model.sysml",
        Range::new(Position::new(10, 5), Position::new(15, 30)),
    );
    let diagnostic = Diagnostic::warning("Multiline warning", location);

    assert_eq!(diagnostic.location.range.start.line, 10);
    assert_eq!(diagnostic.location.range.start.column, 5);
    assert_eq!(diagnostic.location.range.end.line, 15);
    assert_eq!(diagnostic.location.range.end.column, 30);
}

#[test]
fn test_warning_empty_message() {
    let location = Location::new(
        "test.sysml",
        Range::new(Position::new(0, 0), Position::new(0, 1)),
    );
    let diagnostic = Diagnostic::warning("", location);

    assert_eq!(diagnostic.message, "");
    assert_eq!(diagnostic.severity, Severity::Warning);
}

#[test]
fn test_warning_long_file_path() {
    let location = Location::new(
        "/very/long/path/to/some/deeply/nested/directory/structure/model.sysml",
        Range::new(Position::new(0, 0), Position::new(0, 10)),
    );
    let diagnostic = Diagnostic::warning("Warning in nested file", location);

    assert_eq!(
        diagnostic.location.file,
        "/very/long/path/to/some/deeply/nested/directory/structure/model.sysml"
    );
}
