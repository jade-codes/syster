#![allow(clippy::unwrap_used)]

use std::path::Path;
use syster::core::constants::is_supported_extension;
use syster::core::traits::AstNode;
use syster::parser::keywords::get_keywords_for_file;
use syster::semantic::types::diagnostic::{Diagnostic, Location, Position, Range, Severity};
use syster::syntax::formatter::{FormatOptions, format_async};
use tokio_util::sync::CancellationToken;

// ============================================================================
// Tests for SysMLLanguage::kind_to_raw (#420)
// ============================================================================
// Note: kind_to_raw is a private implementation detail of the rowan::Language trait.
// We test it indirectly through the public formatter API which uses it internally.

#[test]
fn test_kind_to_raw_via_formatter_simple_package() {
    // Tests that kind_to_raw correctly converts PackageKw, LBrace, RBrace, etc.
    let source = "package Test { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("package"));
}

#[test]
fn test_kind_to_raw_via_formatter_keywords() {
    // Tests that kind_to_raw handles various SysML keywords
    let source = "part def MyPart { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    let output = result.unwrap();
    assert!(output.contains("part"));
    assert!(output.contains("def"));
}

#[test]
fn test_kind_to_raw_via_formatter_punctuation() {
    // Tests that kind_to_raw handles punctuation tokens
    let source = "package A::B { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("::"));
}

#[test]
fn test_kind_to_raw_via_formatter_comments() {
    // Tests that kind_to_raw handles comment tokens
    let source = "// Comment\npackage Test { }";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("// Comment"));
}

#[test]
fn test_kind_to_raw_via_formatter_import() {
    // Tests that kind_to_raw handles import statements
    let source = "import Package::*;";
    let result = format_async(source, &FormatOptions::default(), &CancellationToken::new());
    assert!(result.is_some());
    assert!(result.unwrap().contains("import"));
}

#[test]
fn test_kind_to_raw_via_formatter_with_cancellation() {
    // Tests that the formatter (which uses kind_to_raw) respects cancellation
    let source = "package Test { }";
    let cancel = CancellationToken::new();
    cancel.cancel();
    let result = format_async(source, &FormatOptions::default(), &cancel);
    assert!(result.is_none());
}

// ============================================================================
// Tests for is_supported_extension (#362)
// ============================================================================

#[test]
fn test_is_supported_extension_sysml() {
    assert!(is_supported_extension("sysml"));
}

#[test]
fn test_is_supported_extension_kerml() {
    assert!(is_supported_extension("kerml"));
}

#[test]
fn test_is_supported_extension_unsupported() {
    assert!(!is_supported_extension("txt"));
    assert!(!is_supported_extension("rs"));
    assert!(!is_supported_extension("md"));
    assert!(!is_supported_extension("json"));
}

#[test]
fn test_is_supported_extension_empty() {
    assert!(!is_supported_extension(""));
}

#[test]
fn test_is_supported_extension_case_sensitive() {
    // The function is case-sensitive
    assert!(!is_supported_extension("SYSML"));
    assert!(!is_supported_extension("SysML"));
    assert!(!is_supported_extension("KERML"));
    assert!(!is_supported_extension("KerML"));
}

#[test]
fn test_is_supported_extension_with_dot() {
    // Extensions should be provided without the dot
    assert!(!is_supported_extension(".sysml"));
    assert!(!is_supported_extension(".kerml"));
}

// ============================================================================
// Tests for AstNode::has_children (#360)
// ============================================================================

// Test struct with no children
#[derive(Debug, Clone)]
struct SimpleNode {
    #[allow(dead_code)]
    value: String,
}

impl AstNode for SimpleNode {
    fn node_type(&self) -> &'static str {
        "SimpleNode"
    }
    // Uses default implementation of has_children (returns false)
}

// Test struct with children
#[derive(Debug, Clone)]
struct ParentNode {
    children: Vec<String>,
}

impl AstNode for ParentNode {
    fn node_type(&self) -> &'static str {
        "ParentNode"
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

#[test]
fn test_has_children_default_returns_false() {
    let node = SimpleNode {
        value: "test".to_string(),
    };
    assert!(!node.has_children());
}

#[test]
fn test_has_children_with_empty_children() {
    let node = ParentNode { children: vec![] };
    assert!(!node.has_children());
}

#[test]
fn test_has_children_with_children() {
    let node = ParentNode {
        children: vec!["child1".to_string()],
    };
    assert!(node.has_children());
}

#[test]
fn test_has_children_with_multiple_children() {
    let node = ParentNode {
        children: vec![
            "child1".to_string(),
            "child2".to_string(),
            "child3".to_string(),
        ],
    };
    assert!(node.has_children());
}

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
