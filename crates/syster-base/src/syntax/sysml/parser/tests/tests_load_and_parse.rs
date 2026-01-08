#![allow(clippy::unwrap_used)]

use crate::syntax::sysml::parser::load_and_parse;

// ============================================================================
// Tests for load_and_parse function (Issue #193)
// crate::syntax::sysml::parser::load_and_parse
// ============================================================================

#[test]
fn test_load_and_parse_valid_sysml_file() {
    // Create a temporary valid .sysml file
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("valid_load.sysml");
    std::fs::write(&test_file, "part def Vehicle;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(
        result.is_ok(),
        "Should successfully parse valid .sysml file"
    );

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one element parsed"
    );
}

#[test]
fn test_load_and_parse_valid_kerml_extension() {
    // Test current behavior: .kerml extension is accepted by the SysML parser,
    // and the file is still parsed using the SysML grammar (content is SysML here).
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("valid_load.kerml");
    std::fs::write(
        &test_file,
        "package TestPackage {\n    part def TestPart;\n}\n",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(
        result.is_ok(),
        "Should accept .kerml extension and parse SysML content"
    );

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one top-level element"
    );
}

#[test]
fn test_load_and_parse_invalid_extension() {
    // Test that invalid file extensions are rejected
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("invalid.txt");
    std::fs::write(&test_file, "part def Vehicle;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Should fail for invalid file extension");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Unsupported file extension"),
        "Error should mention unsupported extension: {error_msg}"
    );
}

#[test]
fn test_load_and_parse_nonexistent_file() {
    // Test error handling for non-existent files
    let test_file = std::env::temp_dir().join("nonexistent_batch2_test_12345.sysml");

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Should fail for non-existent file");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Failed to read"),
        "Error should mention failed read: {error_msg}"
    );
}

#[test]
fn test_load_and_parse_invalid_syntax() {
    // Test that parse errors are properly reported
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("invalid_syntax.sysml");
    // Missing semicolon
    std::fs::write(&test_file, "part def Vehicle").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_err(), "Should fail for invalid syntax");

    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("Parse error"),
        "Error should mention parse error: {error_msg}"
    );
}

#[test]
fn test_load_and_parse_empty_file() {
    // Empty files are valid SysML
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("empty.sysml");
    std::fs::write(&test_file, "").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should successfully parse empty file");

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        0,
        "Should have no elements in empty file"
    );
}

#[test]
fn test_load_and_parse_with_package() {
    // Test parsing file with package declaration
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("with_package.sysml");
    std::fs::write(&test_file, "package MyPackage {\n    part def Vehicle;\n}").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should parse file with package");

    let sysml_file = result.unwrap();
    assert_eq!(
        sysml_file.elements.len(),
        1,
        "Should have one package element"
    );
}

#[test]
fn test_load_and_parse_multiple_elements() {
    // Test parsing file with multiple elements
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("multiple.sysml");
    std::fs::write(
        &test_file,
        "part def Car;\npart def Truck;\naction def Drive;",
    )
    .unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should parse multiple elements");

    let sysml_file = result.unwrap();
    assert_eq!(sysml_file.elements.len(), 3, "Should have three elements");
}

#[test]
fn test_load_and_parse_with_unicode() {
    // Test handling of unicode content
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("unicode.sysml");
    std::fs::write(&test_file, "part def Vehicle; // 中文注释").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should handle unicode content");
}

#[test]
fn test_load_and_parse_with_crlf_line_endings() {
    // Test handling of Windows-style line endings
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("crlf.sysml");
    std::fs::write(&test_file, "part def Vehicle;\r\npart def Engine;\r\n").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should handle CRLF line endings");

    let sysml_file = result.unwrap();
    assert_eq!(sysml_file.elements.len(), 2, "Should parse both elements");
}

#[test]
fn test_load_and_parse_with_imports() {
    // Test parsing file with import statements
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("with_imports.sysml");
    std::fs::write(&test_file, "import Base::*;\npart def Vehicle;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should parse file with imports");
}

#[test]
fn test_load_and_parse_ast_construction() {
    // Test that the AST is properly constructed from loaded file
    let test_dir = std::env::temp_dir().join("batch_2_tests");
    std::fs::create_dir_all(&test_dir).unwrap();

    let test_file = test_dir.join("ast_test.sysml");
    std::fs::write(&test_file, "package TestPkg;\npart def TestDef;").unwrap();

    let result = load_and_parse(&test_file);
    assert!(result.is_ok(), "Should successfully construct AST");

    let sysml_file = result.unwrap();
    // Should have namespace and elements
    assert!(
        sysml_file.namespace.is_some(),
        "Should have namespace declaration"
    );
    assert!(!sysml_file.elements.is_empty(), "Should have elements");
}
