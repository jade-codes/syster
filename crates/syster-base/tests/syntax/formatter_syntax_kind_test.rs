#![allow(clippy::unwrap_used)]

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
