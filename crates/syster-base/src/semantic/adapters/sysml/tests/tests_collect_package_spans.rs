//! Tests for collect_package_spans function in selection.rs
//!
//! These tests verify the behavior of package span collection through the public API.
//! The function `collect_package_spans` is a private helper that handles Package nodes
//! in the AST, collecting spans that contain a given position.

use crate::core::Position;
use crate::semantic::adapters::selection::find_sysml_selection_spans;
use crate::syntax::sysml::ast::{
    Definition, DefinitionKind, Element, Package, Relationships, SysMLFile,
};

use super::tests_helpers::make_span;

#[test]
fn test_collect_package_spans_simple_package() {
    // Test basic package span collection
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(Package {
            name: Some("TestPackage".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };

    let pos = Position::new(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 0);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_collect_package_spans_position_outside_package() {
    // Test that position outside package returns empty
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(Package {
            name: Some("TestPackage".to_string()),
            elements: vec![],
            span: Some(make_span(0, 0, 5, 1)),
        })],
    };

    let pos = Position::new(10, 0);
    let spans = find_sysml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}

#[test]
fn test_collect_package_spans_nested_packages() {
    // Test nested package span collection
    let inner_package = Package {
        name: Some("InnerPackage".to_string()),
        elements: vec![],
        span: Some(make_span(2, 2, 4, 3)),
    };

    let outer_package = Package {
        name: Some("OuterPackage".to_string()),
        elements: vec![Element::Package(inner_package)],
        span: Some(make_span(0, 0, 6, 1)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(outer_package)],
    };

    let pos = Position::new(3, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    // Should have both outer and inner package spans
    assert_eq!(spans.len(), 2);
    // Inner package should be first (smaller)
    assert_eq!(spans[0].start.line, 2);
    assert_eq!(spans[0].end.line, 4);
    // Outer package should be second (larger)
    assert_eq!(spans[1].start.line, 0);
    assert_eq!(spans[1].end.line, 6);
}

#[test]
fn test_collect_package_spans_with_definition() {
    // Test package containing a definition
    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(2, 2, 3, 3)),
        short_name: None,
        short_name_span: None,
        is_abstract: false,
        is_variation: false,
    };

    let package = Package {
        name: Some("Models".to_string()),
        elements: vec![Element::Definition(definition)],
        span: Some(make_span(0, 0, 5, 1)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    let pos = Position::new(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    // Should have both package and definition spans
    assert_eq!(spans.len(), 2);
}

#[test]
fn test_collect_package_spans_multiple_children() {
    // Test package with multiple child elements
    let def1 = Definition {
        kind: DefinitionKind::Part,
        name: Some("Car".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(1, 2, 2, 3)),
        short_name: None,
        short_name_span: None,
        is_abstract: false,
        is_variation: false,
    };

    let def2 = Definition {
        kind: DefinitionKind::Part,
        name: Some("Bike".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(3, 2, 4, 3)),
        short_name: None,
        short_name_span: None,
        is_abstract: false,
        is_variation: false,
    };

    let package = Package {
        name: Some("Vehicles".to_string()),
        elements: vec![Element::Definition(def1), Element::Definition(def2)],
        span: Some(make_span(0, 0, 6, 1)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    // Position in first definition
    let pos = Position::new(1, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 1); // First definition
    assert_eq!(spans[1].start.line, 0); // Package
}
