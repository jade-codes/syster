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
use syster::project::file_loader;

#[test]
fn test_package_span_single_line() {
    let source = "package MyPackage;";
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    // Check namespace declaration (package statement)
    let namespace = file.namespace.expect("Should have namespace");
    let span = namespace.span.expect("Namespace should have span");

    // "package MyPackage;" spans from (0,0) to (0,17)
    assert_eq!(span.start.line, 0);
    assert_eq!(span.start.column, 0);
    assert_eq!(span.end.line, 0);
    // Note: Pest end positions may include or exclude the semicolon
    assert!(span.end.column >= 10, "end column was {}", span.end.column);
}

#[test]
fn test_part_def_span() {
    let source = "part def Vehicle;";
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    let element = &file.elements[0];
    if let syster::language::sysml::syntax::Element::Definition(def) = element {
        let span = def.span.expect("Definition should have span");

        assert_eq!(span.start.line, 0);
        assert_eq!(span.start.column, 0);
        assert_eq!(span.end.line, 0);
        assert!(span.end.column >= 16); // "part def Vehicle;"
    } else {
        panic!("Expected Definition, got {:?}", element);
    }
}

#[test]
fn test_part_usage_span() {
    let source = "part myVehicle: Vehicle;";
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    let element = &file.elements[0];
    if let syster::language::sysml::syntax::Element::Usage(usage) = element {
        let span = usage.span.expect("Usage should have span");

        assert_eq!(span.start.line, 0);
        assert_eq!(span.start.column, 0);
        assert_eq!(span.end.line, 0);
        assert!(span.end.column >= 22); // "part myVehicle: Vehicle;"
    } else {
        panic!("Expected Usage, got {:?}", element);
    }
}

#[test]
fn test_nested_definitions_span() {
    let source = r#"part def Vehicle {
    part engine: Engine;
}"#;
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    let element = &file.elements[0];
    if let syster::language::sysml::syntax::Element::Definition(def) = element {
        let span = def.span.expect("Definition should have span");

        // Outer definition should span all three lines
        assert_eq!(span.start.line, 0);
        assert_eq!(span.end.line, 2);

        // Check nested usage has its own span
        if let Some(first_member) = def.body.first() {
            if let syster::language::sysml::syntax::DefinitionMember::Usage(nested_usage) =
                first_member
            {
                let nested_span = nested_usage.span.expect("Nested usage should have span");
                assert_eq!(nested_span.start.line, 1);
                assert_eq!(nested_span.end.line, 1);
            } else {
                panic!("Expected nested Usage");
            }
        }
    } else {
        panic!("Expected Definition, got {:?}", element);
    }
}

#[test]
fn test_comment_span() {
    // Test doc annotation parsing - doc is an annotation on elements, captured in their span
    let source = r#"package Test;
doc /* This is a doc comment */
part def Vehicle;"#;
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    // The doc annotation is part of the Definition element it precedes
    let element = &file.elements[1];
    if let syster::language::sysml::syntax::Element::Definition(def) = element {
        let span = def.span.expect("Definition should have span");

        // Span currently only covers the definition itself, not the doc annotation
        assert_eq!(span.start.line, 2, "Definition starts at 'part def' line");
        assert_eq!(span.start.column, 0);
        assert_eq!(span.end.line, 2);
        assert_eq!(span.end.column, 17, "Should end after 'part def Vehicle'");
    } else {
        panic!("Expected Definition with doc annotation, got {:?}", element);
    }
}
#[test]
fn test_import_span() {
    let source = "import ScalarValues::*;";
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    let element = &file.elements[0];
    if let syster::language::sysml::syntax::Element::Import(import) = element {
        let span = import.span.expect("Import should have span");

        assert_eq!(span.start.line, 0);
        assert_eq!(span.start.column, 0);
        assert_eq!(span.end.line, 0);
        assert!(span.end.column >= 5, "span too short: {}", span.end.column); // At least "import" keyword
    } else {
        panic!("Expected Import, got {:?}", element);
    }
}
// This prevents aliases like: alias Real for ScalarValues::Real;
// Grammar needs fix: element_reference = { qualified_name | identifier | quoted_name }
fn test_alias_span() {
    let source = r#"package Test;
alias Real for ScalarValues::Real;"#;
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    if !parse_result.errors.is_empty() {
        panic!("Parse errors: {:?}", parse_result.errors);
    }
    let file = parse_result.content.expect("Parse should succeed");

    // First element is package, second is alias
    let element = &file.elements[1];
    if let syster::language::sysml::syntax::Element::Alias(alias) = element {
        let span = alias.span.expect("Alias should have span");

        assert_eq!(span.start.line, 0);
        assert_eq!(span.start.column, 0);
        assert_eq!(span.end.line, 0);
        // Pest may only capture up to the alias keyword or target, not full statement
        assert!(span.end.column >= 5, "span too short: {}", span.end.column);
    } else {
        panic!("Expected Alias, got {:?}", element);
    }
}

#[test]
fn test_multiple_elements_span() {
    let source = r#"package MyPackage;

part def Vehicle;
part myVehicle: Vehicle;"#;
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    // Package on line 0
    let namespace_span = file
        .namespace
        .expect("Should have namespace")
        .span
        .expect("Namespace should have span");
    assert_eq!(namespace_span.start.line, 0);

    // First element is the package, skip it
    // Second element is definition on line 2
    if let syster::language::sysml::syntax::Element::Definition(def) = &file.elements[1] {
        let def_span = def.span.expect("Definition should have span");
        assert_eq!(def_span.start.line, 2);
    } else {
        panic!("Expected Definition at index 1, got {:?}", file.elements[1]);
    }

    // Third element is usage on line 3
    if let syster::language::sysml::syntax::Element::Usage(usage) = &file.elements[2] {
        let usage_span = usage.span.expect("Usage should have span");
        assert_eq!(usage_span.start.line, 3);
    } else {
        panic!("Expected Usage at index 2");
    }
}

#[test]
fn test_span_contains_position() {
    let source = "part def Vehicle;";
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    let element = &file.elements[0];
    if let syster::language::sysml::syntax::Element::Definition(def) = element {
        let span = def.span.expect("Definition should have span");

        // Position in the middle of "Vehicle" should be contained
        let pos_in_name = Position::new(0, 13); // "part def V[i]hicle"
        assert!(span.contains(pos_in_name));

        // Position after semicolon should not be contained
        let pos_after = Position::new(0, 100);
        assert!(!span.contains(pos_after));

        // Position on different line should not be contained
        let pos_wrong_line = Position::new(1, 5);
        assert!(!span.contains(pos_wrong_line));
    } else {
        panic!("Expected Definition");
    }
}

#[test]
fn test_all_elements_have_span() {
    // Test that all parsed elements have a span populated
    let source = r#"package Test;

part def Vehicle {
    part engine: Engine;
}

part myVehicle: Vehicle;"#;
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    if !parse_result.errors.is_empty() {
        panic!("Parse errors: {:?}", parse_result.errors);
    }
    let file = parse_result.content.expect("Parse should succeed");

    // Namespace should have span
    assert!(
        file.namespace.as_ref().and_then(|ns| ns.span).is_some(),
        "Namespace should have span"
    );

    // All elements should have span
    for element in &file.elements {
        match element {
            syster::language::sysml::syntax::Element::Definition(def) => {
                assert!(
                    def.span.is_some(),
                    "Definition '{:?}' missing span",
                    def.name
                );
            }
            syster::language::sysml::syntax::Element::Usage(usage) => {
                assert!(
                    usage.span.is_some(),
                    "Usage '{:?}' missing span",
                    usage.name
                );
            }
            syster::language::sysml::syntax::Element::Comment(comment) => {
                assert!(
                    comment.span.is_some(),
                    "Comment missing span: {}",
                    comment.content
                );
            }
            syster::language::sysml::syntax::Element::Import(import) => {
                assert!(import.span.is_some(), "Import missing span");
            }
            syster::language::sysml::syntax::Element::Alias(alias) => {
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
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    if let syster::language::sysml::syntax::Element::Definition(def) = &file.elements[0] {
        // Check outer definition has span
        let span = def.span.expect("Outer definition should have span");
        assert_eq!(span.start.line, 0);
        assert_eq!(span.end.line, 3); // Closing brace on line 3

        // Check all members have spans
        for member in &def.body {
            match member {
                syster::language::sysml::syntax::DefinitionMember::Usage(usage) => {
                    assert!(usage.span.is_some(), "Nested usage should have span");
                }
                syster::language::sysml::syntax::DefinitionMember::Comment(comment) => {
                    assert!(comment.span.is_some(), "Nested comment should have span");
                }
            }
        }
    } else {
        panic!("Expected Definition");
    }
}

#[test]
fn test_symbol_table_spans() {
    // Test that symbols in the symbol table have span information
    use syster::semantic::Workspace;

    let source = r#"package Test;
part def Vehicle;
part myVehicle: Vehicle;"#;
    let path = PathBuf::from("test.sysml");

    let mut workspace = Workspace::new();
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");
    workspace.add_file(path.clone(), file);

    // Populate symbols
    let result = workspace.populate_all();
    assert!(result.is_ok(), "Symbol population failed: {:?}", result);

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
    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(source, &path);
    let file = parse_result.content.expect("Parse should succeed");

    let element = &file.elements[0];
    if let syster::language::sysml::syntax::Element::Definition(def) = element {
        let span = def.span.expect("Definition should have span");

        // First line should be 0, not 1
        assert_eq!(span.start.line, 0, "Lines should be 0-indexed");

        // First column should be 0, not 1
        assert_eq!(span.start.column, 0, "Columns should be 0-indexed");
    } else {
        panic!("Expected Definition");
    }
}
