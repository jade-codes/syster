#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::syntax::SyntaxFile;
use crate::syntax::parser::{load_and_parse, parse_content, parse_with_result};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to extract SysMLFile from SyntaxFile for testing
fn unwrap_sysml(lang_file: SyntaxFile) -> crate::syntax::sysml::ast::SysMLFile {
    match lang_file {
        SyntaxFile::SysML(file) => file,
        SyntaxFile::KerML(_) => panic!("Expected SysML file in test"),
    }
}

#[test]
fn test_parse_content_whitespace_only() {
    let content = "   \n\t\n   ";
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    assert!(result.is_ok());
    let file = unwrap_sysml(result.unwrap());
    assert!(
        file.elements.is_empty(),
        "Whitespace-only file should be empty"
    );
}

#[test]
fn test_parse_content_comment_only() {
    let content = "// This is a comment\n/* Block comment */";
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    assert!(result.is_ok());
    let file = unwrap_sysml(result.unwrap());
    assert!(
        file.elements.is_empty(),
        "Comment-only file should be empty"
    );
}

#[test]
fn test_parse_content_very_long_line() {
    let long_name = "A".repeat(1000);
    let content = format!("part def {long_name};");
    let path = PathBuf::from("test.sysml");
    let result = parse_content(&content, &path);

    assert!(result.is_ok(), "Should handle very long lines");
}

#[test]
fn test_parse_content_unicode_content() {
    // Unicode in comments should work
    let content = "// Véhicule (vehicle in French)\npart def Vehicle;";
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    assert!(result.is_ok(), "Should handle unicode in comments");
}

#[test]
fn test_parse_content_crlf_line_endings() {
    let content = "part def Vehicle;\r\npart def Engine;\r\n";
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    assert!(result.is_ok(), "Should handle CRLF line endings");
}

#[test]
fn test_parse_content_mixed_line_endings() {
    let content = "part def Vehicle;\npart def Engine;\r\npart def Wheel;";
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    assert!(result.is_ok(), "Should handle mixed line endings");
}

#[test]
fn test_parse_content_deeply_nested_structure() {
    let content = r#"
        package A {
            package B {
                package C {
                    package D {
                        part def DeepPart;
                    }
                }
            }
        }
    "#;
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    assert!(result.is_ok(), "Should handle deeply nested structures");
}

#[test]
fn test_parse_with_result_multiple_errors() {
    let content = r#"
        part def Invalid1 {
            part x
        }
        part def Invalid2 {
            port @#$
        }
    "#;
    let path = PathBuf::from("test.sysml");
    let result = parse_with_result(content, &path);

    assert!(!result.is_ok(), "Should fail with multiple errors");
    assert!(
        !result.errors.is_empty(),
        "Should report at least one error"
    );
}

#[test]
fn test_parse_with_result_error_position_accuracy() {
    // Use complete gibberish that cannot parse
    let content = "part def Vehicle;\n@@@ ### $$$ %%%";
    let path = PathBuf::from("test.sysml");
    let result = parse_with_result(content, &path);

    // parse_with_result does partial recovery, so may have content
    // but should have preserved the error from full parse failure
    assert!(result.has_errors(), "Should have captured the syntax error");
    assert!(!result.errors.is_empty());

    let error = &result.errors[0];
    assert_eq!(
        error.position.line, 1,
        "Error should be on line 1 (0-indexed)"
    );
}

#[test]
fn test_load_and_parse_missing_file() {
    let nonexistent = PathBuf::from("/nonexistent/test.sysml");
    let result = load_and_parse(&nonexistent);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        err.contains("Failed to read") || err.contains("nonexistent"),
        "Error should mention file read failure"
    );
}

#[test]
fn test_load_and_parse_empty_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("empty.sysml");
    fs::write(&file_path, "").expect("Failed to write empty file");

    let result = load_and_parse(&file_path);

    assert!(result.is_ok());
    let file = unwrap_sysml(result.unwrap());
    assert!(file.elements.is_empty());
}

#[test]
fn test_load_and_parse_invalid_utf8() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("invalid.sysml");

    // Write invalid UTF-8 bytes
    fs::write(&file_path, vec![0xFF, 0xFE, 0xFD]).expect("Failed to write invalid UTF-8");

    let result = load_and_parse(&file_path);

    assert!(result.is_err(), "Should fail on invalid UTF-8");
}

#[test]
fn test_parse_content_kerml_placeholder() {
    let content = "class Vehicle;";
    let path = PathBuf::from("test.kerml");
    let result = parse_content(content, &path);

    assert!(result.is_ok());
    let lang_file = result.unwrap();

    // Should return a KerML file
    match lang_file {
        SyntaxFile::KerML(kerml_file) => {
            // KerML parsing is now implemented, should have elements
            assert!(
                !kerml_file.elements.is_empty(),
                "KerML file should have parsed elements"
            );
        }
        SyntaxFile::SysML(_) => {
            panic!("Expected KerML file, got SysML");
        }
    }
}

#[test]
fn test_parse_with_result_kerml_placeholder() {
    let content = "class Vehicle;";
    let path = PathBuf::from("test.kerml");
    let result = parse_with_result(content, &path);

    // Should succeed but return empty (placeholder behavior)
    assert!(result.is_ok());
    assert!(result.errors.is_empty());
}

#[test]
fn test_load_and_parse_kerml_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.kerml");
    fs::write(&file_path, "class Base;").expect("Failed to write KerML file");

    let result = load_and_parse(&file_path);

    assert!(result.is_ok());
    // Currently placeholder - will parse when KerML implemented
}

#[test]
fn test_parse_content_case_sensitive_keywords() {
    let content = "PART DEF Vehicle;"; // Wrong case
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    assert!(result.is_err(), "Keywords should be case-sensitive");
}

#[test]
fn test_parse_content_special_characters_in_strings() {
    let content = r#"part def Vehicle { doc /* Special chars: @#$%^&*() */; }"#;
    let path = PathBuf::from("test.sysml");
    let result = parse_content(content, &path);

    // Should handle special chars in doc comments
    assert!(result.is_ok());
}

#[test]
#[cfg(unix)]
fn test_load_and_parse_symlink() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let real_file = temp_dir.path().join("real.sysml");
    fs::write(&real_file, "part def Vehicle;").expect("Failed to write file");

    let symlink = temp_dir.path().join("link.sysml");
    std::os::unix::fs::symlink(&real_file, &symlink).expect("Failed to create symlink");

    let result = load_and_parse(&symlink);
    assert!(result.is_ok(), "Should follow symlinks");
}
