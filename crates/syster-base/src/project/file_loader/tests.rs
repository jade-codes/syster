#![allow(clippy::unwrap_used, clippy::expect_used)]

mod collection_tests;
mod parsing_tests;

use super::*;
use crate::core::ParseErrorKind;
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

/// Helper to extract SysMLFile from SyntaxFile for testing
fn unwrap_sysml(lang_file: SyntaxFile) -> crate::syntax::sysml::ast::SysMLFile {
    match lang_file {
        SyntaxFile::SysML(file) => file,
        SyntaxFile::KerML(_) => panic!("Expected SysML file in test"),
    }
}

#[test]
fn test_parse_content_sysml() {
    // TDD: Test parsing SysML content from string (for LSP)
    let content = "part def Vehicle;";
    let path = PathBuf::from("test.sysml");

    let result = parse_content(content, &path);
    assert!(result.is_ok(), "Should parse valid SysML content");

    let file = unwrap_sysml(result.unwrap());
    assert!(!file.elements.is_empty(), "Should have parsed elements");
}

#[test]
fn test_parse_content_invalid_syntax() {
    // TDD: Test error handling for invalid syntax
    let content = "this is not valid sysml @#$%";
    let path = PathBuf::from("test.sysml");

    let result = parse_content(content, &path);
    assert!(result.is_err(), "Should fail on invalid syntax");
    assert!(
        result.unwrap_err().contains("Parse error"),
        "Error should mention parse error"
    );
}

#[test]
fn test_parse_content_kerml() {
    // TDD: Test KerML support (currently returns empty)
    let content = "class Vehicle;";
    let path = PathBuf::from("test.kerml");

    let result = parse_content(content, &path);
    assert!(
        result.is_ok(),
        "Should handle KerML files (even if not fully implemented)"
    );
}

#[test]
fn test_parse_content_unsupported_extension() {
    // TDD: Test error for unsupported file types
    let content = "some content";
    let path = PathBuf::from("test.txt");

    let result = parse_content(content, &path);
    assert!(result.is_err(), "Should fail on unsupported extension");
    assert!(
        result.unwrap_err().contains("Unsupported file extension"),
        "Error should mention unsupported extension"
    );
}

#[test]
fn test_parse_content_no_extension() {
    // TDD: Test error for files without extension
    let content = "part def Vehicle;";
    let path = PathBuf::from("test");

    let result = parse_content(content, &path);
    assert!(result.is_err(), "Should fail on missing extension");
    assert!(
        result.unwrap_err().contains("Invalid file extension"),
        "Error should mention invalid extension"
    );
}

#[test]
fn test_load_and_parse_uses_parse_content() {
    // TDD: Test that load_and_parse correctly uses parse_content internally
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.sysml");
    let content = "part def Vehicle;";

    fs::write(&file_path, content).expect("Failed to write test file");

    let result = load_and_parse(&file_path);
    assert!(result.is_ok(), "Should load and parse file from disk");

    let file = unwrap_sysml(result.unwrap());
    assert!(!file.elements.is_empty(), "Should have parsed elements");
}

#[test]
fn test_parse_content_with_complex_sysml() {
    // TDD: Test parsing more complex SysML content
    let content = r#"
        package MyPackage {
            part def Vehicle {
                attribute speed: Real;
            }
            part def Car :> Vehicle;
        }
    "#;
    let path = PathBuf::from("complex.sysml");

    let result = parse_content(content, &path);
    assert!(result.is_ok(), "Should parse complex SysML content");
}

#[test]
fn test_parse_content_empty_file() {
    // TDD: Test parsing empty content
    let content = "";
    let path = PathBuf::from("empty.sysml");

    let result = parse_content(content, &path);
    // Empty content should still parse successfully (empty model)
    assert!(result.is_ok(), "Should handle empty content");
}

#[test]
fn test_parse_with_result_success() {
    // TDD: Successful parse returns no errors
    let content = "part def Vehicle;";
    let path = PathBuf::from("test.sysml");

    let result = parse_with_result(content, &path);

    assert!(result.is_ok());
    assert!(!result.has_errors());
    assert_eq!(result.errors.len(), 0);
    assert!(result.content.is_some());
    let file = unwrap_sysml(result.content.unwrap());
    assert!(!file.elements.is_empty());
}

#[test]
fn test_parse_with_result_syntax_error() {
    // TDD: Syntax error returns ParseError with position
    let content = "part def {"; // Missing name
    let path = PathBuf::from("test.sysml");

    let result = parse_with_result(content, &path);

    assert!(!result.is_ok());
    assert!(result.has_errors());
    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].kind, ParseErrorKind::SyntaxError);
}

#[test]
fn test_error_has_position_info() {
    // Use complete gibberish that cannot parse
    let content = "part def Vehicle;\n@@@ ### $$$ %%%";
    let path = PathBuf::from("test.sysml");

    let result = parse_with_result(content, &path);
    assert!(result.has_errors());

    let error = &result.errors[0];

    assert_eq!(
        error.position.line, 1,
        "Error should be on line 1 (0-indexed)"
    );

    // Error is at the beginning of the invalid line
    assert_eq!(
        error.position.column, 0,
        "Error should be at column 0 (start of invalid line)"
    );
}

#[test]
fn test_parse_error_details() {
    let content = "this is not valid sysml syntax at all!!!";
    let path = PathBuf::from("error.sysml");

    let result = parse_with_result(content, &path);

    assert!(result.has_errors());
    let error = &result.errors[0];
    assert!(!error.message.is_empty());
    assert_eq!(error.kind, ParseErrorKind::SyntaxError);
}

#[test]
fn test_unsupported_extension() {
    let content = "part def Vehicle;";
    let path = PathBuf::from("test.txt");

    let result = parse_with_result(content, &path);

    assert!(result.has_errors());
    assert!(result.errors[0].message.contains("Unsupported"));
}

#[test]
fn test_empty_file_success() {
    let content = "";
    let path = PathBuf::from("empty.sysml");

    let result = parse_with_result(content, &path);

    assert!(result.content.is_some());
    let file = unwrap_sysml(result.content.unwrap());
    assert_eq!(file.elements.len(), 0);
}
