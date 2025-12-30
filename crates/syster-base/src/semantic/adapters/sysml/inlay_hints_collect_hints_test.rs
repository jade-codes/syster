#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

//! Comprehensive tests for the `collect_hints` function in SysML inlay hints module.
//!
//! Tests cover:
//! - Issue #130: semantic::adapters::sysml::inlay_hints::collect_hints
//!
//! Since `collect_hints` is a private function, all tests are written to exercise it
//! through the public API (`extract_inlay_hints`).

use crate::core::{Position, Span};
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::InlayHintKind;
use crate::syntax::sysml::ast::{
    Alias, Comment, Definition, DefinitionKind, DefinitionMember, Element, Import, Package,
    Relationships, SysMLFile, Usage, UsageKind, UsageMember,
};

use super::inlay_hints::extract_inlay_hints;

// ============================================================================
// BASIC ELEMENT TYPE HANDLING
// ============================================================================

#[test]
fn test_collect_hints_empty_file() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };
    let symbol_table = SymbolTable::new();

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_package_traversal() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 4, 3, 9)),
        is_derived: false,
        is_readonly: false,
    };

    let package = Package {
        name: Some("MyPackage".to_string()),
        elements: vec![Element::Usage(usage)],
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should find usage inside package
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_hints_nested_packages() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(5, 8, 5, 13)),
        is_derived: false,
        is_readonly: false,
    };

    let inner_package = Package {
        name: Some("InnerPackage".to_string()),
        elements: vec![Element::Usage(usage)],
        span: None,
    };

    let outer_package = Package {
        name: Some("OuterPackage".to_string()),
        elements: vec![Element::Package(inner_package)],
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(outer_package)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should find usage inside nested packages
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_hints_definition_element() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "wheel".to_string(),
            Symbol::Usage {
                name: "wheel".to_string(),
                qualified_name: "wheel".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Wheel".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let nested_usage = Usage {
        kind: UsageKind::Part,
        name: Some("wheel".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 8, 3, 13)),
        is_derived: false,
        is_readonly: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::default(),
        body: vec![DefinitionMember::Usage(Box::new(nested_usage))],
        span: Some(Span::from_coords(1, 0, 4, 1)),
        is_abstract: false,
        is_variation: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(definition)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should find usage inside definition
    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(hints[0].label.contains("Wheel"));
}

#[test]
fn test_collect_hints_definition_with_multiple_usages() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "engine".to_string(),
            Symbol::Usage {
                name: "engine".to_string(),
                qualified_name: "engine".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Engine".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    symbol_table
        .insert(
            "transmission".to_string(),
            Symbol::Usage {
                name: "transmission".to_string(),
                qualified_name: "transmission".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Transmission".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage1 = Usage {
        kind: UsageKind::Part,
        name: Some("engine".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 4, 3, 10)),
        is_derived: false,
        is_readonly: false,
    };

    let usage2 = Usage {
        kind: UsageKind::Part,
        name: Some("transmission".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(4, 4, 4, 16)),
        is_derived: false,
        is_readonly: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Powertrain".to_string()),
        relationships: Relationships::default(),
        body: vec![
            DefinitionMember::Usage(Box::new(usage1)),
            DefinitionMember::Usage(Box::new(usage2)),
        ],
        span: Some(Span::from_coords(1, 0, 5, 1)),
        is_abstract: false,
        is_variation: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(definition)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should find both usages
    assert_eq!(hints.len(), 2);
    assert!(hints[0].label.contains("Engine"));
    assert!(hints[1].label.contains("Transmission"));
}

#[test]
fn test_collect_hints_definition_with_comments_only() {
    let symbol_table = SymbolTable::new();

    let comment = Comment {
        content: "This is a comment".to_string(),
        span: None,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::default(),
        body: vec![DefinitionMember::Comment(Box::new(comment))],
        span: Some(Span::from_coords(1, 0, 3, 1)),
        is_abstract: false,
        is_variation: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(definition)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // No hints because definition only contains comments
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_usage_element() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "myPart".to_string(),
            Symbol::Usage {
                name: "myPart".to_string(),
                qualified_name: "myPart".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Vehicle".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myPart".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 0, 1, 6)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should find hint for usage
    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(hints[0].label.contains("Vehicle"));
}

#[test]
fn test_collect_hints_comment_element_ignored() {
    let symbol_table = SymbolTable::new();

    let comment = Comment {
        content: "This is a top-level comment".to_string(),
        span: Some(Span::from_coords(1, 0, 1, 30)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(comment)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Comment elements should be ignored
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_import_element_ignored() {
    let symbol_table = SymbolTable::new();

    let import = Import {
        path: "SomePackage::*".to_string(),
        is_recursive: false,
        span: Some(Span::from_coords(1, 0, 1, 20)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Import(import)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Import elements should be ignored
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_alias_element_ignored() {
    let symbol_table = SymbolTable::new();

    let alias = Alias {
        name: Some("MyAlias".to_string()),
        target: "OriginalName".to_string(),
        span: Some(Span::from_coords(1, 0, 1, 25)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Alias(alias)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Alias elements should be ignored
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_mixed_element_types() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let comment = Comment {
        content: "Comment".to_string(),
        span: Some(Span::from_coords(1, 0, 1, 10)),
    };

    let import = Import {
        path: "Package::*".to_string(),
        is_recursive: false,
        span: Some(Span::from_coords(2, 0, 2, 15)),
    };

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 0, 3, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let alias = Alias {
        name: Some("A".to_string()),
        target: "B".to_string(),
        span: Some(Span::from_coords(4, 0, 4, 10)),
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Comment(comment),
            Element::Import(import),
            Element::Usage(usage),
            Element::Alias(alias),
        ],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Only usage should generate a hint
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

// ============================================================================
// NESTED STRUCTURE TESTS
// ============================================================================

#[test]
fn test_collect_hints_deeply_nested_usages() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "sensor".to_string(),
            Symbol::Usage {
                name: "sensor".to_string(),
                qualified_name: "sensor".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Sensor".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Create deeply nested: Usage -> Usage -> Usage
    let deep_usage = Usage {
        kind: UsageKind::Part,
        name: Some("sensor".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(5, 12, 5, 18)),
        is_derived: false,
        is_readonly: false,
    };

    let mid_usage = Usage {
        kind: UsageKind::Part,
        name: Some("controller".to_string()),
        relationships: Relationships::default(),
        body: vec![UsageMember::Usage(Box::new(deep_usage))],
        span: Some(Span::from_coords(3, 8, 6, 9)),
        is_derived: false,
        is_readonly: false,
    };

    let top_usage = Usage {
        kind: UsageKind::Part,
        name: Some("system".to_string()),
        relationships: Relationships::default(),
        body: vec![UsageMember::Usage(Box::new(mid_usage))],
        span: Some(Span::from_coords(1, 4, 7, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(top_usage)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should find the deeply nested usage
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Sensor"));
}

#[test]
fn test_collect_hints_usage_with_comment_members() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "innerPart".to_string(),
            Symbol::Usage {
                name: "innerPart".to_string(),
                qualified_name: "innerPart".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("InnerType".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let comment = Comment {
        content: "Inner comment".to_string(),
        span: None,
    };

    let inner_usage = Usage {
        kind: UsageKind::Part,
        name: Some("innerPart".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 4, 3, 13)),
        is_derived: false,
        is_readonly: false,
    };

    let outer_usage = Usage {
        kind: UsageKind::Part,
        name: Some("outerPart".to_string()),
        relationships: Relationships::default(),
        body: vec![
            UsageMember::Comment(comment),
            UsageMember::Usage(Box::new(inner_usage)),
        ],
        span: Some(Span::from_coords(1, 0, 4, 1)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(outer_usage)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should find the inner usage, comment should be ignored
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("InnerType"));
}

#[test]
fn test_collect_hints_package_definition_usage_hierarchy() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "component".to_string(),
            Symbol::Usage {
                name: "component".to_string(),
                qualified_name: "component".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Component".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("component".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(4, 8, 4, 17)),
        is_derived: false,
        is_readonly: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("System".to_string()),
        relationships: Relationships::default(),
        body: vec![DefinitionMember::Usage(Box::new(usage))],
        span: Some(Span::from_coords(2, 4, 5, 5)),
        is_abstract: false,
        is_variation: false,
    };

    let package = Package {
        name: Some("RootPackage".to_string()),
        elements: vec![Element::Definition(definition)],
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should traverse Package -> Definition -> Usage
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Component"));
}

// ============================================================================
// RANGE FILTERING TESTS
// ============================================================================

#[test]
fn test_collect_hints_range_filter_excludes_before() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Usage at line 2
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(2, 0, 2, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    // Range starts after the usage (lines 5-10)
    let range = (
        Position { line: 5, column: 0 },
        Position {
            line: 10,
            column: 0,
        },
    );
    let hints = extract_inlay_hints(&file, &symbol_table, Some(range));

    // Usage is before the range, should be filtered out
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_range_filter_excludes_after() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Usage at line 10
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(10, 0, 10, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    // Range ends before the usage (lines 1-5)
    let range = (
        Position { line: 1, column: 0 },
        Position { line: 5, column: 0 },
    );
    let hints = extract_inlay_hints(&file, &symbol_table, Some(range));

    // Usage is after the range, should be filtered out
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_range_filter_includes_in_range() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Usage at line 5
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(5, 0, 5, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    // Range includes the usage (lines 1-10)
    let range = (
        Position { line: 1, column: 0 },
        Position {
            line: 10,
            column: 0,
        },
    );
    let hints = extract_inlay_hints(&file, &symbol_table, Some(range));

    // Usage is in the range, should be included
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_hints_range_filter_nested_usages() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "inner1".to_string(),
            Symbol::Usage {
                name: "inner1".to_string(),
                qualified_name: "inner1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    symbol_table
        .insert(
            "inner2".to_string(),
            Symbol::Usage {
                name: "inner2".to_string(),
                qualified_name: "inner2".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type2".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // First inner usage at line 3 (in range)
    let inner1 = Usage {
        kind: UsageKind::Part,
        name: Some("inner1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 4, 3, 10)),
        is_derived: false,
        is_readonly: false,
    };

    // Second inner usage at line 15 (out of range - starts after range end of line 10)
    let inner2 = Usage {
        kind: UsageKind::Part,
        name: Some("inner2".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(15, 4, 15, 10)),
        is_derived: false,
        is_readonly: false,
    };

    // Outer usage must be fully within range for recursion to happen (lines 1-20)
    let outer = Usage {
        kind: UsageKind::Part,
        name: Some("outer".to_string()),
        relationships: Relationships::default(),
        body: vec![
            UsageMember::Usage(Box::new(inner1)),
            UsageMember::Usage(Box::new(inner2)),
        ],
        span: Some(Span::from_coords(1, 0, 20, 0)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(outer)],
    };

    // Range covers lines 1-10 (inner1 is included, inner2 at line 15 is excluded)
    let range = (
        Position { line: 1, column: 0 },
        Position {
            line: 10,
            column: 50,
        },
    );
    let hints = extract_inlay_hints(&file, &symbol_table, Some(range));

    // The outer usage starts before range (line 1) and ends after range (line 20)
    // So it gets filtered out and recursion never happens. Expected: 0 hints
    assert_eq!(hints.len(), 0);
}

#[test]
fn test_collect_hints_range_filter_boundary_start() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Usage exactly at range start (line 5)
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(5, 0, 5, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    // Range starts exactly at the usage line
    let range = (
        Position { line: 5, column: 0 },
        Position {
            line: 10,
            column: 0,
        },
    );
    let hints = extract_inlay_hints(&file, &symbol_table, Some(range));

    // Usage at start boundary should be included
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_hints_range_filter_boundary_end() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Usage that ends before range end (line 10, column 5)
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(10, 0, 10, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    // Range ends after the usage ends (line 10, column 10)
    let range = (
        Position { line: 5, column: 0 },
        Position {
            line: 10,
            column: 10,
        },
    );
    let hints = extract_inlay_hints(&file, &symbol_table, Some(range));

    // Usage fully contained in range should be included
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_collect_hints_multiple_packages_with_same_usage_names() {
    let mut symbol_table = SymbolTable::new();

    // Both usages have the same name but should be handled independently
    symbol_table
        .insert(
            "part".to_string(),
            Symbol::Usage {
                name: "part".to_string(),
                qualified_name: "part".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage1 = Usage {
        kind: UsageKind::Part,
        name: Some("part".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 4, 3, 8)),
        is_derived: false,
        is_readonly: false,
    };

    let usage2 = Usage {
        kind: UsageKind::Part,
        name: Some("part".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(7, 4, 7, 8)),
        is_derived: false,
        is_readonly: false,
    };

    let package1 = Package {
        name: Some("Package1".to_string()),
        elements: vec![Element::Usage(usage1)],
        span: None,
    };

    let package2 = Package {
        name: Some("Package2".to_string()),
        elements: vec![Element::Usage(usage2)],
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package1), Element::Package(package2)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Both usages should generate hints
    assert_eq!(hints.len(), 2);
    assert!(hints[0].label.contains("Type1"));
    assert!(hints[1].label.contains("Type1"));
}

#[test]
fn test_collect_hints_empty_package() {
    let symbol_table = SymbolTable::new();

    let package = Package {
        name: Some("EmptyPackage".to_string()),
        elements: vec![],
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Empty package should produce no hints
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_empty_definition() {
    let symbol_table = SymbolTable::new();

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("EmptyDef".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 0, 2, 1)),
        is_abstract: false,
        is_variation: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(definition)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Empty definition should produce no hints
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_usage_without_span() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: None, // No span
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Usage without span should not generate hints
    assert!(hints.is_empty());
}

#[test]
fn test_collect_hints_multiple_element_types_in_package() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let comment = Comment {
        content: "Comment".to_string(),
        span: None,
    };

    let import = Import {
        path: "SomePackage::*".to_string(),
        is_recursive: false,
        span: None,
    };

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(5, 4, 5, 9)),
        is_derived: false,
        is_readonly: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Def1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(7, 4, 8, 5)),
        is_abstract: false,
        is_variation: false,
    };

    let package = Package {
        name: Some("MixedPackage".to_string()),
        elements: vec![
            Element::Comment(comment),
            Element::Import(import),
            Element::Usage(usage),
            Element::Definition(definition),
        ],
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Only the usage should generate a hint
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_hints_no_range_filter() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type1".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    symbol_table
        .insert(
            "part2".to_string(),
            Symbol::Usage {
                name: "part2".to_string(),
                qualified_name: "part2".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Type2".to_string()),
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let usage1 = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 0, 1, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let usage2 = Usage {
        kind: UsageKind::Part,
        name: Some("part2".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(100, 0, 100, 5)),
        is_derived: false,
        is_readonly: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage1), Element::Usage(usage2)],
    };

    // No range filter - should include all usages
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 2);
    assert!(hints[0].label.contains("Type1"));
    assert!(hints[1].label.contains("Type2"));
}
