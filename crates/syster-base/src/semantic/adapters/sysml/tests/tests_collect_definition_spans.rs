//! Tests for collect_definition_spans function in selection.rs
//!
//! These tests verify the behavior of definition span collection through the public API.
//! The function `collect_definition_spans` is a private helper that handles Definition nodes
//! in the AST, collecting spans that contain a given position.

use crate::core::Position;
use crate::semantic::adapters::selection::find_sysml_selection_spans;
use crate::syntax::sysml::ast::{
    Comment, Definition, DefinitionKind, DefinitionMember, Element, Relationships, SysMLFile,
    Usage, UsageKind, UsageMember,
};

use super::tests_helpers::make_span;

#[test]
fn test_collect_definition_spans_simple_definition() {
    // Test basic definition span collection
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
    assert_eq!(spans[0].start.line, 0);
    assert_eq!(spans[0].end.line, 3);
}

#[test]
fn test_collect_definition_spans_with_usage() {
    // Test definition containing a usage
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("engine".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(2, 4, 3, 5)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![DefinitionMember::Usage(Box::new(usage))],
        span: Some(make_span(0, 0, 5, 1)),
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

    let pos = Position::new(2, 6);
    let spans = find_sysml_selection_spans(&file, pos);

    // Should have both definition and usage spans
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Usage (inner)
    assert_eq!(spans[1].start.line, 0); // Definition (outer)
}

#[test]
fn test_collect_definition_spans_with_comment() {
    // Test definition containing a comment
    let comment = Comment {
        content: "Test comment".to_string(),
        span: Some(make_span(1, 4, 1, 20)),
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![DefinitionMember::Comment(Box::new(comment))],
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

    let pos = Position::new(1, 10);
    let spans = find_sysml_selection_spans(&file, pos);

    // Should have both definition and comment spans
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 1); // Comment
    assert_eq!(spans[1].start.line, 0); // Definition
}

#[test]
fn test_collect_definition_spans_multiple_members() {
    // Test definition with multiple body members
    let usage1 = Usage {
        kind: UsageKind::Part,
        name: Some("engine".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(1, 4, 2, 5)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let comment = Comment {
        content: "Comment".to_string(),
        span: Some(make_span(3, 4, 3, 15)),
    };

    let usage2 = Usage {
        kind: UsageKind::Part,
        name: Some("wheels".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(4, 4, 5, 5)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![
            DefinitionMember::Usage(Box::new(usage1)),
            DefinitionMember::Comment(Box::new(comment)),
            DefinitionMember::Usage(Box::new(usage2)),
        ],
        span: Some(make_span(0, 0, 7, 1)),
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

    // Position in second usage
    let pos = Position::new(4, 6);
    let spans = find_sysml_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 4); // Second usage
    assert_eq!(spans[1].start.line, 0); // Definition
}

#[test]
fn test_collect_definition_spans_nested_usages() {
    // Test definition with nested usage
    let inner_usage = Usage {
        kind: UsageKind::Part,
        name: Some("piston".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(3, 8, 4, 9)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let outer_usage = Usage {
        kind: UsageKind::Part,
        name: Some("engine".to_string()),
        relationships: Relationships::none(),
        body: vec![UsageMember::Usage(Box::new(inner_usage))],
        span: Some(make_span(2, 4, 5, 5)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![DefinitionMember::Usage(Box::new(outer_usage))],
        span: Some(make_span(0, 0, 7, 1)),
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

    let pos = Position::new(3, 10);
    let spans = find_sysml_selection_spans(&file, pos);

    // Should have definition, outer usage, and inner usage spans
    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].start.line, 3); // Inner usage
    assert_eq!(spans[1].start.line, 2); // Outer usage
    assert_eq!(spans[2].start.line, 0); // Definition
}

#[test]
fn test_collect_definition_spans_position_outside() {
    // Test position outside definition
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

    let pos = Position::new(10, 0);
    let spans = find_sysml_selection_spans(&file, pos);

    assert!(spans.is_empty());
}
