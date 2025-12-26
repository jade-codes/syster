//! Tests for the Rowan-based formatter

use super::{FormatOptions, format};

#[test]
fn test_format_simple_package() {
    let source = "package Test { }";
    let result = format(source, &FormatOptions::default());
    assert!(result.contains("package"));
    assert!(result.contains("Test"));
    assert!(result.contains("{"));
    assert!(result.contains("}"));
}

#[test]
fn test_format_preserves_line_comments() {
    let source = "// This is a comment\npackage Test { }";
    let result = format(source, &FormatOptions::default());
    assert!(
        result.contains("// This is a comment"),
        "Line comment should be preserved. Got: {}",
        result
    );
}

#[test]
fn test_format_preserves_block_comments() {
    let source = "/* Block comment */\npackage Test { }";
    let result = format(source, &FormatOptions::default());
    assert!(
        result.contains("/* Block comment */"),
        "Block comment should be preserved. Got: {}",
        result
    );
}

#[test]
fn test_format_preserves_inline_comments() {
    let source = "package Test { // inline comment\n}";
    let result = format(source, &FormatOptions::default());
    assert!(
        result.contains("// inline comment"),
        "Inline comment should be preserved. Got: {}",
        result
    );
}

#[test]
fn test_format_nested_package() {
    let source = "package Outer { package Inner { } }";
    let result = format(source, &FormatOptions::default());
    assert!(result.contains("Outer"));
    assert!(result.contains("Inner"));
}

#[test]
fn test_format_part_definition() {
    let source = "part def Vehicle { }";
    let result = format(source, &FormatOptions::default());
    assert!(result.contains("part"));
    assert!(result.contains("def"));
    assert!(result.contains("Vehicle"));
}

#[test]
fn test_format_part_usage() {
    let source = "part myPart;";
    let result = format(source, &FormatOptions::default());
    assert!(result.contains("part"));
    assert!(result.contains("myPart"));
    assert!(result.contains(";"));
}

#[test]
fn test_format_import() {
    let source = "import Pkg::*;";
    let result = format(source, &FormatOptions::default());
    assert!(result.contains("import"));
    assert!(result.contains("Pkg"));
    assert!(result.contains(";"));
}

#[test]
fn test_format_with_doc_comment() {
    let source = "doc /* Documentation */\npackage Test { }";
    let result = format(source, &FormatOptions::default());
    assert!(
        result.contains("/* Documentation */"),
        "Doc comment should be preserved. Got: {}",
        result
    );
}

#[test]
fn test_format_complex_file() {
    let source = r#"// File header comment
package Vehicle {
    // Part comment
    part def Car {
        attribute wheels : Integer;
    }
    
    part myCar : Car;
}"#;
    let result = format(source, &FormatOptions::default());

    // All comments preserved
    assert!(
        result.contains("// File header comment"),
        "Header comment missing"
    );
    assert!(result.contains("// Part comment"), "Part comment missing");

    // Structure preserved
    assert!(result.contains("package"));
    assert!(result.contains("Vehicle"));
    assert!(result.contains("part def"));
    assert!(result.contains("Car"));
}

#[test]
fn test_format_with_tabs() {
    let source = "package Test { part x; }";
    let options = FormatOptions {
        tab_size: 4,
        insert_spaces: false,
        print_width: 80,
    };
    let result = format(source, &options);
    // Just check it doesn't panic and produces output
    assert!(result.contains("package"));
}

#[test]
fn test_format_options_default() {
    let options = FormatOptions::default();
    assert_eq!(options.tab_size, 4);
    assert!(options.insert_spaces);
    assert_eq!(options.print_width, 80);
}

#[test]
fn test_format_empty_input() {
    let source = "";
    let result = format(source, &FormatOptions::default());
    assert!(result.is_empty() || result.trim().is_empty());
}

#[test]
fn test_format_whitespace_only() {
    let source = "   \n\n   ";
    let result = format(source, &FormatOptions::default());
    // Should handle gracefully
    assert!(result.len() <= source.len() + 10); // Allow some variation
}

#[test]
fn test_format_comment_only() {
    let source = "// Just a comment";
    let result = format(source, &FormatOptions::default());
    assert!(
        result.contains("// Just a comment"),
        "Comment-only file should preserve comment. Got: {}",
        result
    );
}
