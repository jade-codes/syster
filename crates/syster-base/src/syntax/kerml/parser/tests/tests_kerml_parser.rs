#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use crate::syntax::kerml::parser::{load_and_parse, parse_content, parse_with_result};
use std::path::{Path, PathBuf};

// ============================================================================
// Tests for load_and_parse function (Issue #356)
// ============================================================================

#[test]
fn test_load_and_parse_valid_kerml_file() {
    // Create a temporary valid .kerml file
    let test_dir = std::env::temp_dir().join("kerml_parser_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("valid.kerml");
    std::fs::write(
        &test_file,
        "package TestPackage {\n    class TestClass;\n}\n",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(
        result.is_ok(),
        "Expected successful parsing of valid .kerml file"
    );

    let kerml_file = result.unwrap();
    assert_eq!(
        kerml_file.elements.len(),
        1,
        "Expected one top-level element"
    );
}

#[test]
fn test_load_and_parse_valid_sysml_file() {
    // Create a temporary valid .sysml file
    let test_dir = std::env::temp_dir().join("kerml_parser_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("valid.sysml");
    std::fs::write(
        &test_file,
        "package TestPackage {\n    part def TestPart;\n}\n",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(
        result.is_ok(),
        "Expected successful parsing of valid .sysml file"
    );

    let kerml_file = result.unwrap();
    assert_eq!(
        kerml_file.elements.len(),
        1,
        "Expected one top-level element"
    );
}

#[test]
fn test_load_and_parse_invalid_extension() {
    let test_dir = std::env::temp_dir().join("kerml_parser_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("invalid.txt");
    std::fs::write(
        &test_file,
        "package TestPackage {\n    class TestClass;\n}\n",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Expected error for invalid file extension");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Unsupported file extension"),
        "Error message should mention unsupported extension: {error_msg}"
    );
}

#[test]
fn test_load_and_parse_nonexistent_file() {
    let test_file = PathBuf::from("/tmp/nonexistent_file_12345.kerml");

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Expected error for non-existent file");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Failed to read"),
        "Error message should mention failed read: {error_msg}"
    );
}

#[test]
fn test_load_and_parse_invalid_syntax() {
    let test_dir = std::env::temp_dir().join("kerml_parser_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("invalid_syntax.kerml");
    // Missing semicolon after class declaration
    std::fs::write(
        &test_file,
        "package TestPackage {\n    class TestClass\n}\n",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Expected error for invalid syntax");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Parse error"),
        "Error message should mention parse error: {error_msg}"
    );
}

#[test]
fn test_load_and_parse_empty_file() {
    let test_dir = std::env::temp_dir().join("kerml_parser_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("empty.kerml");
    std::fs::write(&test_file, "").unwrap();

    let result = load_and_parse(&test_file);
    assert!(
        result.is_ok(),
        "Expected successful parsing of empty .kerml file"
    );

    let kerml_file = result.unwrap();
    assert_eq!(
        kerml_file.elements.len(),
        0,
        "Expected no top-level elements in empty file"
    );
}

// ============================================================================
// Tests for parse_content function (Issue #352)
// ============================================================================

#[test]
fn test_parse_content_valid_kerml() {
    let content = "package TestPackage {\n    class TestClass;\n}\n";
    let path = Path::new("test.kerml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Expected successful parsing of valid KerML content"
    );

    let kerml_file = result.unwrap();
    assert_eq!(
        kerml_file.elements.len(),
        1,
        "Expected one top-level element"
    );
}

#[test]
fn test_parse_content_empty_string() {
    let content = "";
    let path = Path::new("empty.kerml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Expected successful parsing of empty content"
    );

    let kerml_file = result.unwrap();
    assert_eq!(
        kerml_file.elements.len(),
        0,
        "Expected no elements in empty content"
    );
}

#[test]
fn test_parse_content_syntax_error() {
    let content = "package TestPackage {\n    class TestClass\n}\n"; // Missing semicolon
    let path = Path::new("test.kerml");

    let result = parse_content(content, path);
    assert!(result.is_err(), "Expected error for invalid syntax");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Parse error"),
        "Error message should mention parse error: {error_msg}"
    );
}

#[test]
fn test_parse_content_error_includes_path() {
    let content = "invalid syntax here!@#$";
    let path = Path::new("/some/test/path.kerml");

    let result = parse_content(content, path);
    assert!(result.is_err(), "Expected error for invalid content");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("path.kerml"),
        "Error message should include the file path: {error_msg}"
    );
}

#[test]
fn test_parse_content_multiple_elements() {
    let content = "class FirstClass;\nclass SecondClass;";
    let path = Path::new("test.kerml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Expected successful parsing of multiple elements"
    );

    let kerml_file = result.unwrap();
    assert_eq!(
        kerml_file.elements.len(),
        2,
        "Expected two top-level elements"
    );
}

#[test]
fn test_parse_content_with_package() {
    // Test package instead of namespace, which is more commonly used
    let content = "package MyPackage;";
    let path = Path::new("test.kerml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Expected successful parsing with package declaration"
    );

    let kerml_file = result.unwrap();
    // An empty package becomes a namespace declaration
    assert!(
        kerml_file.namespace.is_some(),
        "Expected namespace to be present"
    );
    assert_eq!(
        kerml_file.namespace.unwrap().name,
        "MyPackage",
        "Expected correct namespace name"
    );
}

// ============================================================================
// Tests for parse_content internal behavior (Issue #353)
// Testing the closure/internal logic through the public API
// ============================================================================

#[test]
fn test_parse_content_pest_parser_integration() {
    // Test that parse_content properly uses pest parser and constructs AST
    let content = "package TestPkg {\n    class MyClass {\n        feature myFeature;\n    }\n}";
    let path = Path::new("test.kerml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Expected successful parsing and AST construction"
    );

    let kerml_file = result.unwrap();
    assert_eq!(kerml_file.elements.len(), 1, "Expected one package");
}

#[test]
fn test_parse_content_pest_error_handling() {
    // Test error handling when pest parser fails
    let content = "this is completely invalid @#$%^&*";
    let path = Path::new("test.kerml");

    let result = parse_content(content, path);
    assert!(result.is_err(), "Expected parse error from pest parser");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Parse error in test.kerml"),
        "Expected formatted parse error message: {error_msg}"
    );
}

#[test]
fn test_parse_content_ast_construction() {
    // Test that AST is properly constructed from pest pairs
    let content = "class SimpleClass;";
    let path = Path::new("test.kerml");

    let result = parse_content(content, path);
    assert!(result.is_ok(), "Expected successful AST construction");

    let kerml_file = result.unwrap();
    assert_eq!(kerml_file.elements.len(), 1, "Expected one element in AST");
}

#[test]
fn test_parse_content_complex_structure() {
    // Test parsing of more complex structures to ensure closure handles them
    let content = r#"
        package ComplexPackage {
            class BaseClass;
            class DerivedClass specializes BaseClass {
                feature attr1;
                feature attr2;
            }
        }
    "#;
    let path = Path::new("complex.kerml");

    let result = parse_content(content, path);
    assert!(
        result.is_ok(),
        "Expected successful parsing of complex structure"
    );

    let kerml_file = result.unwrap();
    assert_eq!(kerml_file.elements.len(), 1, "Expected one package element");
}

// ============================================================================
// Tests for parse_with_result function (additional coverage)
// ============================================================================

#[test]
fn test_parse_with_result_success() {
    let content = "class TestClass;";
    let path = Path::new("test.kerml");

    let result = parse_with_result(content, path);
    assert!(result.is_ok(), "Expected successful parse result");
    assert!(result.content.is_some(), "Expected content to be present");
    assert!(result.errors.is_empty(), "Expected no errors");
}

#[test]
fn test_parse_with_result_invalid_extension() {
    let content = "class TestClass;";
    let path = Path::new("test.txt");

    let result = parse_with_result(content, path);
    assert!(result.has_errors(), "Expected parse result to have errors");
    assert!(result.content.is_none(), "Expected no content");
    assert_eq!(result.errors.len(), 1, "Expected one error");
}

#[test]
fn test_parse_with_result_syntax_error() {
    let content = "class TestClass"; // Missing semicolon
    let path = Path::new("test.kerml");

    let result = parse_with_result(content, path);
    assert!(result.has_errors(), "Expected parse result to have errors");
    assert!(result.content.is_none(), "Expected no content");

    let error = &result.errors[0];
    assert_eq!(
        error.kind,
        crate::core::ParseErrorKind::SyntaxError,
        "Expected syntax error kind"
    );
}

#[test]
fn test_parse_with_result_error_position() {
    let content = "class TestClass"; // Missing semicolon at end of line
    let path = Path::new("test.kerml");

    let result = parse_with_result(content, path);
    assert!(result.has_errors(), "Expected parse result to have errors");

    let error = &result.errors[0];
    // Position should be present and valid (no need to check >= 0 for usize)
    assert!(
        error.position.line < 1000,
        "Expected reasonable line number"
    );
    assert!(
        error.position.column < 1000,
        "Expected reasonable column number"
    );
}

#[test]
fn test_parse_with_result_empty_content() {
    let content = "";
    let path = Path::new("empty.kerml");

    let result = parse_with_result(content, path);
    assert!(result.is_ok(), "Expected successful parse of empty content");
    assert!(result.content.is_some(), "Expected content to be present");

    let kerml_file = result.content.unwrap();
    assert_eq!(kerml_file.elements.len(), 0, "Expected no elements");
}
