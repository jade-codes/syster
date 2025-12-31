#![allow(clippy::unwrap_used)]

//! Tests for source position tracking (span) in AST nodes
//!
//! These tests verify that the parser correctly captures source locations
//! for all AST elements, which is essential for LSP features like:
//! - Hover (show info at exact cursor position)
//! - Go-to-definition (jump to where symbol is defined)
//! - Error reporting (highlight exact problematic code)

use std::path::PathBuf;
use syster::core::Position;
use syster::syntax::SyntaxFile;
use syster::syntax::parser::parse_with_result;
use syster::syntax::sysml::ast::SysMLFile;

/// Helper to parse SysML content and extract the SysMLFile
fn parse_sysml(source: &str) -> SysMLFile {
    let path = PathBuf::from("test.sysml");
    let parse_result = parse_with_result(source, &path);
    let language_file = parse_result.content.expect("Parse should succeed");
    match language_file {
        SyntaxFile::SysML(file) => file,
        _ => panic!("Expected SysML file"),
    }
}

#[test]
fn test_package_span_single_line() {
    let source = "package MyPackage;";

    let file = parse_sysml(source);

    // Check namespace declaration (package statement)
    let namespace = file.namespace.expect("Should have namespace");
    let span = namespace.span.expect("Namespace should have span");

    // Span captures identifier "MyPackage" at columns 8-17
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 8);
    assert_eq!(span.end.line, 0);
    assert_eq!(span.end.column, 17);
}

#[test]
fn test_part_def_span() {
    let source = "part def Vehicle;";

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Definition(def) = &file.elements[0] else {
        panic!("Expected Definition");
    };
    let span = def.span.expect("Definition should have span");

    // Span captures identifier "Vehicle" at columns 9-16
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 9);
    assert_eq!(span.end.line, 0);
    assert_eq!(span.end.column, 16);
}

#[test]
fn test_part_usage_span() {
    let source = "part myVehicle: Vehicle;";

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Usage(usage) = &file.elements[0] else {
        panic!("Expected Usage");
    };
    let span = usage.span.expect("Usage should have span");

    // Span captures identifier "myVehicle" at columns 5-14
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 5);
    assert_eq!(span.end.line, 0);
    assert_eq!(span.end.column, 14);
}

#[test]
fn test_nested_definitions_span() {
    let source = r#"part def Vehicle {
    part engine: Engine;
}"#;

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Definition(def) = &file.elements[0] else {
        panic!("Expected Definition");
    };
    let span = def.span.expect("Definition should have span");

    // Span captures identifier "Vehicle" at columns 9-16 on line 0
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 9);
    assert_eq!(span.end.line, 0);
    assert_eq!(span.end.column, 16);

    let syster::syntax::sysml::ast::DefinitionMember::Usage(nested_usage) =
        def.body.first().expect("Should have first member")
    else {
        panic!("Expected nested Usage");
    };
    let nested_span = nested_usage.span.expect("Nested usage should have span");
    // Span captures identifier "engine" at columns 9-15 on line 1
    assert_eq!(nested_span.start.line, 1);
    assert_eq!(nested_span.start.column, 9);
    assert_eq!(nested_span.end.line, 1);
    assert_eq!(nested_span.end.column, 15);
}

#[test]
fn test_comment_span() {
    // Test doc annotation parsing - doc is an annotation on elements, captured in their span
    let source = r#"package Test;
doc /* This is a doc comment */
part def Vehicle;"#;

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Definition(def) = &file.elements[1] else {
        panic!("Expected Definition");
    };
    let span = def.span.expect("Definition should have span");

    // Span captures identifier "Vehicle" at columns 9-16 on line 2
    assert_eq!(span.start.line, 2);
    assert_eq!(span.start.column, 9);
    assert_eq!(span.end.line, 2);
    assert_eq!(span.end.column, 16);
}
#[test]
fn test_import_span() {
    let source = "import ScalarValues::*;";

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Import(import) = &file.elements[0] else {
        panic!("Expected Import");
    };
    let span = import.span.expect("Import should have span");

    // Span captures import path "ScalarValues::*" at columns 7-22
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 7);
    assert_eq!(span.end.line, 0);
    assert_eq!(span.end.column, 22);
}

#[test]
fn test_alias_span() {
    let source = r#"package Test;
alias Real for ScalarValues::Real;"#;
    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Alias(alias) = &file.elements[1] else {
        panic!("Expected Alias");
    };
    let span = alias.span.expect("Alias should have span");

    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 6);
    assert_eq!(span.end.line, 1);
    assert_eq!(span.end.column, 10);
}

#[test]
fn test_multiple_elements_span() {
    let source = r#"package MyPackage;

part def Vehicle;
part myVehicle: Vehicle;"#;

    let file = parse_sysml(source);

    let namespace_span = file
        .namespace
        .expect("Should have namespace")
        .span
        .expect("Namespace should have span");
    assert_eq!(namespace_span.start.line, 0);

    let syster::syntax::sysml::ast::Element::Definition(def) = &file.elements[1] else {
        panic!("Expected Definition at index 1");
    };
    let def_span = def.span.expect("Definition should have span");
    assert_eq!(def_span.start.line, 2);

    let syster::syntax::sysml::ast::Element::Usage(usage) = &file.elements[2] else {
        panic!("Expected Usage at index 2");
    };
    let usage_span = usage.span.expect("Usage should have span");
    assert_eq!(usage_span.start.line, 3);
}

#[test]
fn test_span_contains_position() {
    let source = "part def Vehicle;";

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Definition(def) = &file.elements[0] else {
        panic!("Expected Definition");
    };
    let span = def.span.expect("Definition should have span");

    let pos_in_name = Position::new(0, 13);
    assert!(span.contains(pos_in_name));

    let pos_after = Position::new(0, 100);
    assert!(!span.contains(pos_after));

    let pos_wrong_line = Position::new(1, 5);
    assert!(!span.contains(pos_wrong_line));
}

#[test]
fn test_all_elements_have_span() {
    // Test that all parsed elements have a span populated
    let source = r#"package Test;

part def Vehicle {
    part engine: Engine;
}

part myVehicle: Vehicle;"#;
    let file = parse_sysml(source);

    // Namespace should have span
    assert!(
        file.namespace.as_ref().and_then(|ns| ns.span).is_some(),
        "Namespace should have span"
    );

    // All elements should have span
    for element in &file.elements {
        match element {
            syster::syntax::sysml::ast::Element::Definition(def) => {
                assert!(
                    def.span.is_some(),
                    "Definition '{:?}' missing span",
                    def.name
                );
            }
            syster::syntax::sysml::ast::Element::Usage(usage) => {
                assert!(
                    usage.span.is_some(),
                    "Usage '{:?}' missing span",
                    usage.name
                );
            }
            syster::syntax::sysml::ast::Element::Comment(comment) => {
                assert!(
                    comment.span.is_some(),
                    "Comment missing span: {}",
                    comment.content
                );
            }
            syster::syntax::sysml::ast::Element::Import(import) => {
                assert!(import.span.is_some(), "Import missing span");
            }
            syster::syntax::sysml::ast::Element::Alias(alias) => {
                assert!(
                    alias.span.is_some(),
                    "Alias '{:?}' missing span",
                    alias.name
                );
            }
            _ => {}
        }
    }
}

#[test]
fn test_deeply_nested_span() {
    let source = r#"part def Car {
    part engine: Engine;
    part transmission: Transmission;
}"#;

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Definition(def) = &file.elements[0] else {
        panic!("Expected Definition");
    };
    let span = def.span.expect("Outer definition should have span");
    // Span captures identifier "Car" at columns 9-12 on line 0
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 9);
    assert_eq!(span.end.line, 0);
    assert_eq!(span.end.column, 12);

    for member in &def.body {
        match member {
            syster::syntax::sysml::ast::DefinitionMember::Usage(usage) => {
                assert!(usage.span.is_some(), "Nested usage should have span");
            }
            syster::syntax::sysml::ast::DefinitionMember::Comment(comment) => {
                assert!(comment.span.is_some(), "Nested comment should have span");
            }
        }
    }
}

#[test]
fn test_symbol_table_spans() {
    // Test that symbols in the symbol table have span information
    use syster::semantic::Workspace;

    let source = r#"package Test;
part def Vehicle;
part myVehicle: Vehicle;"#;

    let mut workspace = Workspace::<SyntaxFile>::new();
    let path = PathBuf::from("test.sysml");

    let file = parse_sysml(source);
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

    // Populate symbols
    let result = workspace.populate_all();
    assert!(result.is_ok(), "Symbol population failed: {result:?}");

    let symbol_table = workspace.symbol_table();

    // Check that Package symbol has span
    let package_symbol = symbol_table
        .lookup("Test")
        .expect("Package 'Test' should be in symbol table");
    assert!(
        package_symbol.span().is_some(),
        "Package symbol should have span"
    );

    // Check that Definition symbol has span
    let def_symbol = symbol_table
        .lookup_qualified("Test::Vehicle")
        .expect("Definition 'Test::Vehicle' should be in symbol table");
    assert!(
        def_symbol.span().is_some(),
        "Definition symbol should have span"
    );

    // Check that Usage symbol has span
    let usage_symbol = symbol_table
        .lookup_qualified("Test::myVehicle")
        .expect("Usage 'Test::myVehicle' should be in symbol table");
    assert!(
        usage_symbol.span().is_some(),
        "Usage symbol should have span"
    );
}

#[test]
fn test_span_positions_are_zero_indexed() {
    // LSP uses 0-indexed positions, verify our spans match
    let source = "part def Vehicle;";

    let file = parse_sysml(source);

    let syster::syntax::sysml::ast::Element::Definition(def) = &file.elements[0] else {
        panic!("Expected Definition");
    };
    let span = def.span.expect("Definition should have span");

    // Verify 0-indexed: identifier \"Vehicle\" at columns 9-16 on line 0
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 9);
    assert_eq!(span.end.line, 0);
    assert_eq!(span.end.column, 16);
}
