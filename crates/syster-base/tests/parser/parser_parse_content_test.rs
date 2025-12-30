#![allow(clippy::unwrap_used)]

use std::path::Path;
use syster::syntax::sysml::parser::parse_content;

// ============================================================================
// Tests for parse_content function (Issue #191)
// These tests verify the behavior of syster::syntax::sysml::parser::parse_content
// ============================================================================

#[test]
fn test_parse_content_valid_part_definition() {
    // Test successful parsing of basic SysML content
    let content = "part def Vehicle;";
    let path = Path::new("test.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should successfully parse valid part definition"
    );

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one element parsed"
    );
}

#[test]
fn test_parse_content_multiple_elements() {
    // Test parsing multiple elements in sequence
    let content = r#"
        part def Vehicle;
        part def Engine;
        attribute def Speed;
    "#;
    let path = Path::new("test.sysml");

    let result = parse_content(content, path);
    assert!(result.is_ok(), "Should parse multiple elements");

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        3,
        "Should have three elements parsed"
    );
}

#[test]
fn test_parse_content_with_package() {
    // Test parsing content with package body (braces)
    let content = r#"
        package TestPackage {
            part def Vehicle;
        }
    "#;
    let path = Path::new("test.sysml");

    let result = parse_content(content, path);
    assert!(result.is_ok(), "Should parse package with content");

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one package element"
    );
}

#[test]
fn test_parse_content_empty_string() {
    // Test parsing empty content - should succeed (empty file is valid)
    let content = "";
    let path = Path::new("test.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should handle empty content successfully: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_whitespace_only() {
    // Test parsing content with only whitespace
    let content = "   \n\t\n   ";
    let path = Path::new("test.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should handle whitespace-only content: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_comment_only() {
    // Test parsing content with only comments
    let content = r#"
        // This is a comment
        /* This is a block comment */
    "#;
    let path = Path::new("test.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should handle comment-only content: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_complex_structure() {
    // Test parsing more complex nested structures
    let content = r#"
        package ComplexPackage {
            part def Vehicle {
                part engine : Engine;
                attribute speed : Real;
            }
            
            part def Engine {
                attribute power : Real;
            }
        }
    "#;
    let path = Path::new("complex.sysml");

    let result = parse_content(content, path);
    assert!(result.is_ok(), "Should parse complex nested structure");
}

// ============================================================================
// Tests for parse error handling (parse_error map_err closure)
// These tests verify the map_err closure that formats parse errors
// ============================================================================

#[test]
fn test_parse_content_invalid_syntax_error_message() {
    // Test that parse errors are properly formatted with path
    // This tests the closure: |e| format!("Parse error in {}: {}", path.display(), e)
    let content = "this is not valid sysml @#$%^&";
    let path = Path::new("invalid.sysml");

    let result = parse_content(content, path);
    assert!(result.is_err(), "Should fail on invalid syntax");

    let error = result.unwrap_err();
    assert!(
        error.contains("Parse error in"),
        "Error should contain 'Parse error in', got: {}",
        error
    );
    assert!(
        error.contains("invalid.sysml"),
        "Error should contain file path, got: {}",
        error
    );
}

#[test]
fn test_parse_content_incomplete_definition_error() {
    // Test error formatting with incomplete definition
    let content = "part def"; // Incomplete - missing name and semicolon
    let path = Path::new("incomplete.sysml");

    let result = parse_content(content, path);
    assert!(result.is_err(), "Should fail on incomplete definition");

    let error = result.unwrap_err();
    assert!(
        error.contains("Parse error in"),
        "Error should be formatted with 'Parse error in'"
    );
    assert!(
        error.contains("incomplete.sysml"),
        "Error should include the file path"
    );
}

#[test]
fn test_parse_content_missing_semicolon_error() {
    // Test error with missing semicolon
    let content = "part def Vehicle"; // Missing semicolon
    let path = Path::new("missing_semicolon.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_err(),
        "Should fail when semicolon is missing: {:?}",
        result.ok()
    );

    let error = result.unwrap_err();
    assert!(
        error.contains("Parse error in"),
        "Error should be properly formatted"
    );
    assert!(
        error.contains("missing_semicolon.sysml"),
        "Error should include path"
    );
}

#[test]
fn test_parse_content_error_with_nested_path() {
    // Test error message formatting with nested directory path
    let content = "invalid syntax here @#$";
    let path = Path::new("deeply/nested/directory/structure/file.sysml");

    let result = parse_content(content, path);
    assert!(result.is_err(), "Should fail on invalid syntax");

    let error = result.unwrap_err();
    assert!(
        error.contains("Parse error in"),
        "Error should have proper prefix"
    );
    assert!(
        error.contains("deeply/nested/directory/structure/file.sysml"),
        "Error should include full path, got: {}",
        error
    );
}

#[test]
fn test_parse_content_error_with_special_chars_in_path() {
    // Test error message with special characters in path
    let content = "bad syntax";
    let path = Path::new("test-file_name (1).sysml");

    let result = parse_content(content, path);
    assert!(result.is_err(), "Should fail on invalid syntax");

    let error = result.unwrap_err();
    assert!(
        error.contains("test-file_name (1).sysml"),
        "Error should preserve special characters in path"
    );
}

// ============================================================================
// Tests for AST construction error handling
// These tests verify the second map_err closure that formats AST errors
// ============================================================================

// Note: AST construction errors are difficult to trigger directly because
// the from_pest implementation usually handles most cases that parse correctly.
// The second closure formats AST errors as:
// |e| format!("AST error in {}: {:?}", path.display(), e)
//
// This would require content that:
// 1. Parses successfully by the Pest parser
// 2. Fails during AST construction (from_pest)
//
// Such cases are rare but could theoretically occur with malformed
// parse tree structures that don't match AST expectations.
// For comprehensive coverage, we've tested the parse error path extensively.

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_parse_content_unicode_characters() {
    // Test parsing with unicode characters in identifiers
    let content = "part def Vehicle; // 中文注释";
    let path = Path::new("unicode.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should handle unicode in comments: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_very_long_content() {
    // Test with a larger content block
    let mut content = String::from("package LargePackage {\n");
    for i in 0..100 {
        content.push_str(&format!("    part def Part{};\n", i));
    }
    content.push_str("}\n");

    let path = Path::new("large.sysml");
    let result = parse_content(&content, path);
    assert!(
        result.is_ok(),
        "Should handle large content blocks: {:?}",
        result.err()
    );

    let sysml_file = result.unwrap();
    assert!(!sysml_file.elements.is_empty(), "Should parse all elements");
}

#[test]
fn test_parse_content_crlf_line_endings() {
    // Test with Windows-style line endings
    let content = "part def Vehicle;\r\npart def Engine;\r\n";
    let path = Path::new("crlf.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should handle CRLF line endings: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_mixed_line_endings() {
    // Test with mixed line endings (Unix + Windows)
    let content = "part def Vehicle;\npart def Engine;\r\npart def Wheel;";
    let path = Path::new("mixed.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should handle mixed line endings: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_multiple_packages() {
    // Test parsing multiple packages (issue #10 compatibility)
    let content = r#"
        package Vehicle;
        package Engine;
        package Transmission;
    "#;
    let path = Path::new("multi_pkg.sysml");

    let result = parse_content(content, path);
    assert!(result.is_ok(), "Should parse multiple packages");

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.namespaces.len(),
        3,
        "Should track all package declarations"
    );
}

#[test]
fn test_parse_content_with_imports() {
    // Test parsing content with import statements
    let content = r#"
        package TestPackage {
            import StandardLibrary::*;
            part def Vehicle;
        }
    "#;
    let path = Path::new("imports.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should parse content with imports: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_nested_packages() {
    // Test parsing nested package structures
    let content = r#"
        package OuterPackage {
            package InnerPackage {
                part def Vehicle;
            }
        }
    "#;
    let path = Path::new("nested.sysml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Should parse nested packages: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_content_trailing_semicolon_only() {
    // Test with just a semicolon
    // This should either succeed with empty elements or fail with a clear error.
    // Either behavior is acceptable as long as it doesn't panic, and simply
    // calling parse_content here will cause the test to fail if a panic occurs.
    let content = ";";
    let path = Path::new("semicolon.sysml");

    let _ = parse_content(content, path);
}

#[test]
fn test_parse_content_path_display_consistency() {
    // Test that different path styles are consistently displayed in errors
    let content = "invalid @#$";

    let paths = vec![
        Path::new("file.sysml"),
        Path::new("./file.sysml"),
        Path::new("dir/file.sysml"),
        Path::new("./dir/file.sysml"),
    ];

    for path in paths {
        let result = parse_content(content, path);
        assert!(result.is_err(), "Should fail for path: {:?}", path);

        let error = result.unwrap_err();
        assert!(
            error.contains("Parse error in"),
            "Error should have correct format for path: {:?}",
            path
        );
    }
}
