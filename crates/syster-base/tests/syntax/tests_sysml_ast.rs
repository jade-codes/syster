//! SysML AST Construction Tests
//!
//! These tests verify that SysML source is correctly parsed into AST structures.
//! Unlike parser tests that verify grammar rules, these tests verify the semantic
//! structure of the resulting AST.

#![allow(clippy::unwrap_used)]

use std::path::PathBuf;
use syster::project::file_loader;
use syster::syntax::sysml::ast::Element;
use syster::syntax::sysml::ast::enums::DefinitionKind;

#[test]
fn test_parse_attribute_def_from_stdlib() {
    // Test actual attribute def from MeasurementReferences.sysml
    let input = r#"
    package TestPackage {
        attribute def DimensionOneUnit {
        }
    }
    "#;

    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(input, &path);
    let language_file = parse_result.content.expect("Parse should succeed");
    let file = match language_file {
        syster::syntax::SyntaxFile::SysML(f) => f,
        _ => panic!("Expected SysML file"),
    };

    // Should have 1 element (the package)
    assert_eq!(file.elements.len(), 1);

    let package = match &file.elements[0] {
        Element::Package(p) => p,
        _ => panic!("Expected Package"),
    };

    // Package should have 1 member (the attribute def)
    assert_eq!(package.elements.len(), 1, "Package should have 1 member");

    // Check the attribute def
    let member = &package.elements[0];
    let Element::Definition(def) = member else {
        panic!("Expected Definition member, got {member:?}");
    };

    assert_eq!(
        def.kind,
        DefinitionKind::Attribute,
        "Should be Attribute definition"
    );
    assert_eq!(
        def.name,
        Some("DimensionOneUnit".to_string()),
        "Should have correct name"
    );
}

#[test]
fn test_parse_abstract_attribute_def() {
    // Test ABSTRACT attribute def like in stdlib
    let input = r#"
    package MeasurementReferences {
        abstract attribute def ScalarMeasurementReference {
        }
    }
    "#;

    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(input, &path);

    assert!(
        parse_result.content.is_some(),
        "Parse should succeed. Errors: {:?}",
        parse_result.errors
    );

    let language_file = parse_result.content.expect("Parse should succeed");
    let syster::syntax::SyntaxFile::SysML(file) = language_file else {
        panic!("Expected SysML file");
    };

    // Should have 1 element (the package)
    assert_eq!(file.elements.len(), 1, "Should have 1 package");

    let Element::Package(package) = &file.elements[0] else {
        panic!("Expected Package, got {:?}", file.elements[0]);
    };

    // Package should have 1 member (the attribute def)
    assert_eq!(package.elements.len(), 1, "Package should have 1 member");

    // Check the attribute def
    let member = &package.elements[0];
    let Element::Definition(def) = member else {
        panic!("Expected Definition member, got {member:?}");
    };
    assert_eq!(
        def.kind,
        DefinitionKind::Attribute,
        "Should be Attribute definition"
    );
    assert_eq!(
        def.name,
        Some("ScalarMeasurementReference".to_string()),
        "Should have correct name"
    );
    assert!(
        def.is_abstract,
        "Should be marked as abstract - THIS IS THE BUG!"
    );
}
