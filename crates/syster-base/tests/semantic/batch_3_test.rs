//! Comprehensive tests for batch-3 module (syster-base portion)
//!
//! This file contains tests for:
//! - Issue #147: collect_definition_spans (tested via find_selection_spans)
//! - Issue #146: collect_containing_spans (tested via find_selection_spans)
//! - Issue #145: collect_package_spans (tested via find_selection_spans)

use syster::core::{Position, Span};
use syster::semantic::selection::find_sysml_selection_spans;
use syster::syntax::sysml::ast::{
    Comment, Definition, DefinitionKind, DefinitionMember, Element, Package, Relationships,
    SysMLFile, Usage, UsageKind, UsageMember,
};

// =============================================================================
// Helper functions
// =============================================================================

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span {
        start: Position {
            line: start_line,
            column: start_col,
        },
        end: Position {
            line: end_line,
            column: end_col,
        },
    }
}

// =============================================================================
// Tests for collect_package_spans (Issue #145)
// These test the private function through the public API find_selection_spans
// =============================================================================

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
        is_abstract: false,
        is_variation: false,
    };

    let def2 = Definition {
        kind: DefinitionKind::Part,
        name: Some("Bike".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(3, 2, 4, 3)),
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

// =============================================================================
// Tests for collect_definition_spans (Issue #147)
// These test the private function through the public API find_selection_spans
// =============================================================================

#[test]
fn test_collect_definition_spans_simple_definition() {
    // Test basic definition span collection
    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(make_span(0, 0, 3, 1)),
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
        is_derived: false,
        is_readonly: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![DefinitionMember::Usage(Box::new(usage))],
        span: Some(make_span(0, 0, 5, 1)),
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
        is_derived: false,
        is_readonly: false,
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
        is_derived: false,
        is_readonly: false,
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
        is_derived: false,
        is_readonly: false,
    };

    let outer_usage = Usage {
        kind: UsageKind::Part,
        name: Some("engine".to_string()),
        relationships: Relationships::none(),
        body: vec![UsageMember::Usage(Box::new(inner_usage))],
        span: Some(make_span(2, 4, 5, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::none(),
        body: vec![DefinitionMember::Usage(Box::new(outer_usage))],
        span: Some(make_span(0, 0, 7, 1)),
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

// =============================================================================
// Tests for collect_containing_spans (Issue #146)
// These test the private function through the public API find_selection_spans
// =============================================================================

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
        is_derived: false,
        is_readonly: false,
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
