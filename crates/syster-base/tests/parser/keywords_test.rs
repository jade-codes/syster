#![allow(clippy::unwrap_used)]

use std::path::Path;
use syster::parser::keywords::get_keywords_for_file;

// ============================================================================
// Tests for get_keywords_for_file (#358)
// ============================================================================

#[test]
fn test_get_keywords_for_file_sysml() {
    let path = Path::new("test.sysml");
    let keywords = get_keywords_for_file(path);
    assert!(!keywords.is_empty());
    assert!(keywords.contains(&"part def"));
    assert!(keywords.contains(&"package"));
}

#[test]
fn test_get_keywords_for_file_kerml() {
    let path = Path::new("test.kerml");
    let keywords = get_keywords_for_file(path);
    assert!(!keywords.is_empty());
    assert!(keywords.contains(&"classifier"));
    assert!(keywords.contains(&"datatype"));
    // KerML should not have SysML-specific keywords
    assert!(!keywords.contains(&"part def"));
}

#[test]
fn test_get_keywords_for_file_no_extension() {
    let path = Path::new("test");
    let keywords = get_keywords_for_file(path);
    // Should default to SYSML_KEYWORDS
    assert!(!keywords.is_empty());
    assert!(keywords.contains(&"part def"));
}

#[test]
fn test_get_keywords_for_file_unsupported_extension() {
    let path = Path::new("test.txt");
    let keywords = get_keywords_for_file(path);
    // Should default to SYSML_KEYWORDS
    assert!(!keywords.is_empty());
    assert!(keywords.contains(&"part def"));
}

#[test]
fn test_get_keywords_for_file_with_path() {
    let path = Path::new("/some/path/to/model.sysml");
    let keywords = get_keywords_for_file(path);
    assert!(keywords.contains(&"part def"));

    let path = Path::new("/another/path/to/model.kerml");
    let keywords = get_keywords_for_file(path);
    assert!(keywords.contains(&"classifier"));
}

#[test]
fn test_get_keywords_for_file_multiple_dots() {
    let path = Path::new("my.model.sysml");
    let keywords = get_keywords_for_file(path);
    assert!(keywords.contains(&"part def"));

    let path = Path::new("my.test.kerml");
    let keywords = get_keywords_for_file(path);
    assert!(keywords.contains(&"classifier"));
}

#[test]
fn test_get_keywords_for_file_case_sensitive() {
    // Extension matching is case-sensitive
    let path = Path::new("test.SYSML");
    let keywords = get_keywords_for_file(path);
    // Should default to SYSML_KEYWORDS (not matched as "sysml")
    assert!(keywords.contains(&"part def"));
}

#[test]
fn test_get_keywords_for_file_returns_different_arrays() {
    // Verify that .sysml and .kerml actually return different keyword sets
    let sysml_path = Path::new("test.sysml");
    let kerml_path = Path::new("test.kerml");

    let sysml_keywords = get_keywords_for_file(sysml_path);
    let kerml_keywords = get_keywords_for_file(kerml_path);

    // SysML has keywords KerML doesn't have
    assert!(sysml_keywords.contains(&"part def"));
    assert!(!kerml_keywords.contains(&"part def"));

    // KerML has keywords SysML doesn't have
    assert!(kerml_keywords.contains(&"classifier"));
    assert!(!sysml_keywords.contains(&"classifier"));
}
