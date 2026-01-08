#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use crate::semantic::types::diagnostic::{Diagnostic, Location, Position, Range, Severity};

// ============================================================================
// Tests for Position::new (Issue #342)
// ============================================================================

#[test]
fn test_position_new_zero() {
    let pos = Position::new(0, 0);
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_new_arbitrary_values() {
    let pos = Position::new(42, 17);
    assert_eq!(pos.line, 42);
    assert_eq!(pos.column, 17);
}

#[test]
fn test_position_new_large_values() {
    let pos = Position::new(999999, 999999);
    assert_eq!(pos.line, 999999);
    assert_eq!(pos.column, 999999);
}

#[test]
fn test_position_equality() {
    let pos1 = Position::new(10, 5);
    let pos2 = Position::new(10, 5);
    assert_eq!(pos1, pos2);
}

#[test]
fn test_position_copy() {
    let pos1 = Position::new(10, 5);
    let pos2 = pos1; // Position is Copy, so this copies
    assert_eq!(pos1, pos2);
}

// ============================================================================
// Tests for Range::new (Issue #340)
// ============================================================================

#[test]
fn test_range_new_single_line() {
    let start = Position::new(5, 10);
    let end = Position::new(5, 20);
    let range = Range::new(start, end);

    assert_eq!(range.start.line, 5);
    assert_eq!(range.start.column, 10);
    assert_eq!(range.end.line, 5);
    assert_eq!(range.end.column, 20);
}

#[test]
fn test_range_new_same_position() {
    let pos = Position::new(3, 7);
    let range = Range::new(pos, pos);

    assert_eq!(range.start, range.end);
}

#[test]
fn test_range_new_zero_positions() {
    let start = Position::new(0, 0);
    let end = Position::new(0, 0);
    let range = Range::new(start, end);

    assert_eq!(range.start.line, 0);
    assert_eq!(range.start.column, 0);
    assert_eq!(range.end.line, 0);
    assert_eq!(range.end.column, 0);
}

#[test]
fn test_range_equality() {
    let range1 = Range::new(Position::new(1, 2), Position::new(3, 4));
    let range2 = Range::new(Position::new(1, 2), Position::new(3, 4));
    assert_eq!(range1, range2);
}

#[test]
fn test_range_copy() {
    let range1 = Range::new(Position::new(1, 2), Position::new(3, 4));
    let range2 = range1; // Range is Copy, so this copies
    assert_eq!(range1, range2);
}

// ============================================================================
// Tests for Range::single (Issue #339)
// ============================================================================

#[test]
fn test_range_single_at_zero() {
    let range = Range::single(0, 0);

    assert_eq!(range.start.line, 0);
    assert_eq!(range.start.column, 0);
    assert_eq!(range.end.line, 0);
    assert_eq!(range.end.column, 1);
}

#[test]
fn test_range_single_at_line_end() {
    let range = Range::single(10, 80);

    assert_eq!(range.start.line, 10);
    assert_eq!(range.start.column, 80);
    assert_eq!(range.end.line, 10);
    assert_eq!(range.end.column, 81);
}

#[test]
fn test_range_single_large_values() {
    let range = Range::single(999, 999);

    assert_eq!(range.start.line, 999);
    assert_eq!(range.start.column, 999);
    assert_eq!(range.end.line, 999);
    assert_eq!(range.end.column, 1000);
}

#[test]
fn test_range_single_is_one_column_wide() {
    let range = Range::single(7, 15);
    assert_eq!(range.end.column - range.start.column, 1);
}

// ============================================================================
// Tests for Location::new (Issue #338)
// ============================================================================

#[test]
fn test_location_new_with_string_literal() {
    let range = Range::single(5, 10);
    let location = Location::new("test.sysml", range);

    assert_eq!(location.file, "test.sysml");
    assert_eq!(location.range.start.line, 5);
    assert_eq!(location.range.start.column, 10);
}

#[test]
fn test_location_new_with_string() {
    let range = Range::single(1, 2);
    let filename = String::from("module.kerml");
    let location = Location::new(filename, range);

    assert_eq!(location.file, "module.kerml");
}

#[test]
fn test_location_new_with_empty_filename() {
    let range = Range::single(0, 0);
    let location = Location::new("", range);

    assert_eq!(location.file, "");
    assert_eq!(location.range.start.line, 0);
}

#[test]
fn test_location_new_with_path() {
    let range = Range::new(Position::new(10, 5), Position::new(10, 15));
    let location = Location::new("src/models/vehicle.sysml", range);

    assert_eq!(location.file, "src/models/vehicle.sysml");
    assert_eq!(location.range.start.line, 10);
    assert_eq!(location.range.end.column, 15);
}

#[test]
fn test_location_equality() {
    let range = Range::single(1, 1);
    let loc1 = Location::new("file.sysml", range);
    let loc2 = Location::new("file.sysml", range);
    assert_eq!(loc1, loc2);
}

// ============================================================================
// Tests for Diagnostic::error (Issue #335)
// ============================================================================
// Note: Detailed tests for `Diagnostic::error` are defined in `tests.rs`
// (`test_diagnostic_creation`). They verify severity, message, and location.
// To avoid duplicate maintenance, we rely on that canonical test here.

#[test]
fn test_diagnostic_error_with_empty_message() {
    let location = Location::new("test.sysml", Range::single(0, 0));
    let diag = Diagnostic::error("", location);

    assert_eq!(diag.message, "");
    assert_eq!(diag.severity, Severity::Error);
}

#[test]
fn test_diagnostic_error_no_code_by_default() {
    let location = Location::new("file.sysml", Range::single(1, 1));
    let diag = Diagnostic::error("Some error", location);

    assert!(diag.code.is_none());
}

#[test]
fn test_diagnostic_error_clone() {
    let location = Location::new("test.sysml", Range::single(5, 10));
    let diag1 = Diagnostic::error("Error", location);
    let diag2 = diag1.clone();

    assert_eq!(diag1, diag2);
}

// ============================================================================
// Tests for Diagnostic::with_code (Issue #337)
// ============================================================================

#[test]
fn test_diagnostic_with_code_string() {
    let location = Location::new("file.sysml", Range::single(1, 1));
    let code = String::from("E042");
    let diag = Diagnostic::error("Error", location).with_code(code);

    assert_eq!(diag.code, Some("E042".to_string()));
}

#[test]
fn test_diagnostic_with_code_empty() {
    let location = Location::new("test.sysml", Range::single(0, 0));
    let diag = Diagnostic::error("Error", location).with_code("");

    assert_eq!(diag.code, Some("".to_string()));
}

#[test]
fn test_diagnostic_with_code_preserves_other_fields() {
    let location = Location::new("test.sysml", Range::single(5, 10));
    let diag = Diagnostic::error("My error", location.clone()).with_code("E123");

    assert_eq!(diag.severity, Severity::Error);
    assert_eq!(diag.message, "My error");
    assert_eq!(diag.location.file, "test.sysml");
}

#[test]
fn test_diagnostic_with_code_chaining() {
    let location = Location::new("file.sysml", Range::single(1, 1));
    let diag = Diagnostic::warning("Warning", location).with_code("W001");

    // Focus on chaining behavior: code should be set after chaining
    assert_eq!(diag.message, "Warning");
    assert_eq!(diag.code, Some("W001".to_string()));
}

#[test]
fn test_diagnostic_with_code_numeric_code() {
    let location = Location::new("test.sysml", Range::single(0, 0));
    let diag = Diagnostic::error("Error", location).with_code("0042");

    assert_eq!(diag.code, Some("0042".to_string()));
}

#[test]
fn test_diagnostic_with_code_complex_code() {
    let location = Location::new("test.sysml", Range::single(0, 0));
    let diag = Diagnostic::error("Error", location).with_code("SYSML-E-001");

    assert_eq!(diag.code, Some("SYSML-E-001".to_string()));
}

// ============================================================================
// Tests for Diagnostic Display (Issue #343)
// ============================================================================

#[test]
fn test_diagnostic_display_multiline_location() {
    let location = Location::new(
        "module.kerml",
        Range::new(Position::new(10, 5), Position::new(12, 3)),
    );
    let diag = Diagnostic::error("Multi-line error", location);
    let display = format!("{diag}");

    // Display shows start position
    assert!(display.contains("module.kerml"));
    assert!(display.contains("11:6")); // line 10+1, column 5+1
    assert!(display.contains("Multi-line error"));
}

#[test]
fn test_diagnostic_display_warning() {
    let location = Location::new("file.sysml", Range::single(5, 10));
    let diag = Diagnostic::warning("Unused variable", location);
    let display = format!("{diag}");

    assert!(display.contains("Warning"));
    assert!(display.contains("Unused variable"));
}

#[test]
fn test_diagnostic_display_with_code() {
    let location = Location::new("test.sysml", Range::single(0, 0));
    let diag = Diagnostic::error("Error with code", location).with_code("E042");
    let display = format!("{diag}");

    // Note: The Display impl doesn't include the code, just the message
    // This test verifies the current behavior
    assert!(display.contains("Error with code"));
    assert!(display.contains("test.sysml:1:1"));
}

#[test]
fn test_diagnostic_display_zero_indexed_to_one_indexed() {
    let location = Location::new("file.sysml", Range::single(0, 0));
    let diag = Diagnostic::error("At origin", location);
    let display = format!("{diag}");

    // Position (0, 0) should display as (1, 1)
    assert!(display.contains("1:1"));
}

#[test]
fn test_diagnostic_display_format_components() {
    let location = Location::new("src/test.sysml", Range::single(99, 49));
    let diag = Diagnostic::error("Message here", location);
    let display = format!("{diag}");

    // Verify format: file:line:column: severity: message
    assert!(display.starts_with("src/test.sysml:"));
    assert!(display.contains("100:50")); // 99+1, 49+1
    assert!(display.contains("Error"));
    assert!(display.contains("Message here"));
}

#[test]
fn test_diagnostic_display_empty_message() {
    let location = Location::new("test.sysml", Range::single(5, 10));
    let diag = Diagnostic::error("", location);
    let display = format!("{diag}");

    // Should still have file:line:column: severity: format
    assert!(display.contains("test.sysml:6:11"));
    assert!(display.contains("Error"));
}

#[test]
fn test_diagnostic_display_empty_filename() {
    let location = Location::new("", Range::single(0, 0));
    let diag = Diagnostic::error("Error in unnamed file", location);
    let display = format!("{diag}");

    assert!(display.contains(":1:1"));
    assert!(display.contains("Error in unnamed file"));
}

// ============================================================================
// Additional comprehensive edge case tests
// ============================================================================

#[test]
fn test_severity_variants() {
    // Ensure all severity variants work with diagnostics
    let location = Location::new("test.sysml", Range::single(0, 0));

    let error = Diagnostic::error("Error msg", location.clone());
    assert_eq!(error.severity, Severity::Error);

    let warning = Diagnostic::warning("Warning msg", location);
    assert_eq!(warning.severity, Severity::Warning);
}

#[test]
fn test_diagnostic_equality() {
    let location = Location::new("test.sysml", Range::single(1, 1));
    let diag1 = Diagnostic::error("Message", location.clone());
    let diag2 = Diagnostic::error("Message", location);

    assert_eq!(diag1, diag2);
}

#[test]
fn test_diagnostic_inequality_different_messages() {
    let location = Location::new("test.sysml", Range::single(1, 1));
    let diag1 = Diagnostic::error("Message1", location.clone());
    let diag2 = Diagnostic::error("Message2", location);

    assert_ne!(diag1, diag2);
}

#[test]
fn test_diagnostic_inequality_different_codes() {
    let location = Location::new("test.sysml", Range::single(1, 1));
    let diag1 = Diagnostic::error("Message", location.clone()).with_code("E001");
    let diag2 = Diagnostic::error("Message", location).with_code("E002");

    assert_ne!(diag1, diag2);
}
