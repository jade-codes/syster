//! Tests for collect_containing_spans function in selection.rs
//!
//! These tests verify the behavior of containing span collection through the public API.
//! The function `collect_containing_spans` is a private helper that dispatches to specific
//! element handlers based on the element type, collecting spans that contain a given position.

use crate::core::Position;
use crate::semantic::adapters::selection::find_sysml_selection_spans;
use crate::syntax::sysml::ast::{
    Comment, Definition, DefinitionKind, Element, Package, Relationships, SysMLFile, Usage,
    UsageKind,
};

use super::tests_helpers::make_span;

#[test]
fn test_collect_containing_spans_package_element() {
    // Test collecting spans for a package element
    let package = Package {
        name: Some("Test".to_string()),
        elements: vec![],
        span: Some(make_span(0, 0, 5, 1)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    let pos = Position::new(2, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_containing_spans_definition_element() {
    // Test collecting spans for a definition element
    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(0, 0, 3, 1)),
        short_name: None,
        short_name_span: None,
        is_abstract: false,
        is_variation: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(definition)],
    };

    let pos = Position::new(1, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_containing_spans_usage_element() {
    // Test collecting spans for a usage element
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(0, 0, 2, 1)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    let pos = Position::new(1, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_containing_spans_comment_element() {
    // Test collecting spans for a comment element
    let comment = Comment {
        content: "Test comment".to_string(),
        span: Some(make_span(0, 0, 0, 20)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(comment)],
    };

    let pos = Position::new(0, 10);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_containing_spans_multiple_element_types() {
    // Test collecting spans with multiple different element types
    let package = Package {
        name: Some("Models".to_string()),
        elements: vec![],
        span: Some(make_span(0, 0, 2, 1)),
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(3, 0, 5, 1)),
        short_name: None,
        short_name_span: None,
        is_abstract: false,
        is_variation: false,
    };

    let comment = Comment {
        content: "Comment".to_string(),
        span: Some(make_span(6, 0, 6, 15)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Package(package),
            Element::Definition(definition),
            Element::Comment(comment),
        ],
    };

    // Position in definition
    let pos = Position::new(4, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 3);
}

#[test]
fn test_collect_containing_spans_empty_file() {
    // Test with empty file
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };

    let pos = Position::new(1, 5);
    let spans = find_sysml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}

#[test]
fn test_collect_containing_spans_no_matching_position() {
    // Test when position doesn't match any element
    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(0, 0, 3, 1)),
        short_name: None,
        short_name_span: None,
        is_abstract: false,
        is_variation: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(definition)],
    };

    let pos = Position::new(10, 10);
    let spans = find_sysml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}
