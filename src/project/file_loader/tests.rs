use super::*;
use std::path::PathBuf;

#[test]
fn test_parse_content_sysml() {
    // TDD: Test parsing SysML content from string (for LSP)
    let content = "part def Vehicle;";
    let path = PathBuf::from("test.sysml");

    let result = parse_content(content, &path);
    assert!(result.is_ok(), "Should parse valid SysML content");

    let file = result.unwrap();
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

    let file = result.unwrap();
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
