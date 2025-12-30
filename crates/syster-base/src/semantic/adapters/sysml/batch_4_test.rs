#![allow(clippy::unwrap_used)]

//! Comprehensive tests for batch-4 module covering:
//! - Issue #143: collect_usage_spans (private, tested through public API)
//! - Issue #142: try_push_span (private, tested through public API)
//! - Issue #141: range_size (private, tested through sorting behavior)
//! - Issue #140: find_selection_spans (public API)
//! - Issue #132: collect_definition_hints (private, tested through public API)
//!
//! All tests follow the principle of testing through the public API only.

use crate::core::{Position, Span};
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::InlayHintKind;
use crate::syntax::sysml::ast::{
    Alias, Comment, Definition, DefinitionKind, DefinitionMember, Element, Import, Package,
    Relationships, SysMLFile, Usage, UsageKind, UsageMember,
};

use super::inlay_hints::extract_inlay_hints;
use super::selection::find_selection_spans;

// =============================================================================
// Helper Functions
// =============================================================================

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span::from_coords(start_line, start_col, end_line, end_col)
}

fn make_position(line: usize, column: usize) -> Position {
    Position::new(line, column)
}

fn make_usage(name: &str, kind: UsageKind, span: Option<Span>, body: Vec<UsageMember>) -> Usage {
    Usage {
        kind,
        name: Some(name.to_string()),
        relationships: Relationships::default(),
        body,
        span,
        is_derived: false,
        is_readonly: false,
    }
}

fn make_definition(
    name: &str,
    kind: DefinitionKind,
    span: Option<Span>,
    body: Vec<DefinitionMember>,
) -> Definition {
    Definition {
        kind,
        is_abstract: false,
        is_variation: false,
        name: Some(name.to_string()),
        relationships: Relationships::default(),
        body,
        span,
    }
}

fn make_package(name: &str, span: Option<Span>, elements: Vec<Element>) -> Package {
    Package {
        name: Some(name.to_string()),
        elements,
        span,
    }
}

fn make_comment(content: &str, span: Option<Span>) -> Comment {
    Comment {
        content: content.to_string(),
        span,
    }
}

fn make_import(path: &str, span: Option<Span>) -> Import {
    Import {
        path: path.to_string(),
        is_recursive: false,
        span,
    }
}

fn make_alias(name: &str, span: Option<Span>) -> Alias {
    Alias {
        name: Some(name.to_string()),
        target: "Target".to_string(),
        span,
    }
}

fn create_usage_symbol(name: &str, type_name: Option<&str>) -> Symbol {
    Symbol::Usage {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "Part".to_string(),
        semantic_role: None,
        usage_type: type_name.map(String::from),
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

// =============================================================================
// Tests for find_selection_spans (Issue #140)
// =============================================================================

#[test]
fn test_find_selection_spans_empty_file() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };
    let pos = make_position(1, 5);
    let spans = find_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_find_selection_spans_position_outside_all_elements() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = make_position(10, 5);
    let spans = find_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_find_selection_spans_single_package() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = make_position(3, 0);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
    assert_eq!(spans[0].end.line, 5);
}

#[test]
fn test_find_selection_spans_single_definition() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(make_definition(
            "TestDef",
            DefinitionKind::Part,
            Some(make_span(1, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = make_position(3, 0);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_find_selection_spans_single_usage() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(make_usage(
            "testUsage",
            UsageKind::Part,
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ))],
    };
    let pos = make_position(2, 0);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_find_selection_spans_stops_at_first_containing() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Package(make_package("First", Some(make_span(1, 0, 3, 1)), vec![])),
            Element::Package(make_package("Second", Some(make_span(5, 0, 8, 1)), vec![])),
        ],
    };
    let pos = make_position(2, 5);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

// =============================================================================
// Tests for range_size (Issue #141) - Tested through sorting behavior
// =============================================================================

#[test]
fn test_range_size_single_line_spans_sorted() {
    // Smaller single-line span should come first
    let inner = Element::Usage(make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 5, 2, 15)),
        vec![],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(2, 0, 2, 20)),
        vec![inner],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 10);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.column, 5); // Inner (smaller)
    assert_eq!(spans[1].start.column, 0); // Outer (larger)
}

#[test]
fn test_range_size_multi_line_spans_sorted() {
    // Span with fewer lines should come first
    let inner = Element::Usage(make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 0, 3, 10)),
        vec![],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(1, 0, 5, 10)),
        vec![inner],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 5);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Inner (2 lines)
    assert_eq!(spans[1].start.line, 1); // Outer (5 lines)
}

#[test]
fn test_range_size_three_nested_levels_sorted() {
    // Test sorting with three levels of nesting
    let innermost = make_comment("comment", Some(make_span(3, 5, 3, 15)));
    let middle = Element::Usage(make_usage(
        "Middle",
        UsageKind::Part,
        Some(make_span(2, 0, 4, 10)),
        vec![UsageMember::Comment(innermost)],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(1, 0, 6, 10)),
        vec![middle],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(3, 10);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 3);
    assert_eq!(spans[0].start.line, 3); // Innermost
    assert_eq!(spans[1].start.line, 2); // Middle
    assert_eq!(spans[2].start.line, 1); // Outer
}

#[test]
fn test_range_size_same_start_different_end() {
    // Spans starting at same position but ending differently
    let inner = Element::Usage(make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 0, 3, 5)),
        vec![],
    ));
    let outer = Element::Package(make_package(
        "Outer",
        Some(make_span(2, 0, 5, 10)),
        vec![inner],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 5);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].end.line, 3); // Inner (smaller)
    assert_eq!(spans[1].end.line, 5); // Outer (larger)
}

// =============================================================================
// Tests for try_push_span (Issue #142) - Tested through element types
// =============================================================================

#[test]
fn test_try_push_span_with_none_span() {
    // Element with None span should not be collected
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package("Test", None, vec![]))],
    };
    let pos = make_position(2, 5);
    let spans = find_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_try_push_span_with_comment() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test comment",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = make_position(1, 10);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_with_import() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Import(make_import(
            "Test::Package",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = make_position(1, 10);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_with_alias() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Alias(make_alias(
            "TestAlias",
            Some(make_span(1, 0, 1, 20)),
        ))],
    };
    let pos = make_position(1, 10);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].start.line, 1);
}

#[test]
fn test_try_push_span_position_at_boundary_start() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 10); // Exactly at start
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_try_push_span_position_at_boundary_end() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 20); // Exactly at end
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_try_push_span_position_before_span() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 9); // Just before start
    let spans = find_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

#[test]
fn test_try_push_span_position_after_span() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(make_comment(
            "Test",
            Some(make_span(5, 10, 5, 20)),
        ))],
    };
    let pos = make_position(5, 21); // Just after end
    let spans = find_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

// =============================================================================
// Tests for collect_usage_spans (Issue #143) - Tested through nested usages
// =============================================================================

#[test]
fn test_collect_usage_spans_empty_usage() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(make_usage(
            "Empty",
            UsageKind::Part,
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ))],
    };
    let pos = make_position(2, 0);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_collect_usage_spans_with_nested_usage() {
    let inner = make_usage(
        "Inner",
        UsageKind::Part,
        Some(make_span(2, 2, 3, 3)),
        vec![],
    );
    let outer = Element::Usage(make_usage(
        "Outer",
        UsageKind::Part,
        Some(make_span(1, 0, 5, 1)),
        vec![UsageMember::Usage(Box::new(inner))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![outer],
    };
    let pos = make_position(2, 5);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Inner
    assert_eq!(spans[1].start.line, 1); // Outer
}

#[test]
fn test_collect_usage_spans_with_comment_member() {
    let comment = make_comment("Comment", Some(make_span(2, 2, 2, 10)));
    let usage = Element::Usage(make_usage(
        "Parent",
        UsageKind::Part,
        Some(make_span(1, 0, 5, 1)),
        vec![UsageMember::Comment(comment)],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![usage],
    };
    let pos = make_position(2, 5);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // Comment
    assert_eq!(spans[1].start.line, 1); // Usage
}

#[test]
fn test_collect_usage_spans_deeply_nested() {
    // Test deeply nested usage -> usage -> usage -> comment
    let comment = make_comment("Deep", Some(make_span(4, 6, 4, 15)));
    let level3 = make_usage(
        "Level3",
        UsageKind::Part,
        Some(make_span(3, 4, 5, 5)),
        vec![UsageMember::Comment(comment)],
    );
    let level2 = make_usage(
        "Level2",
        UsageKind::Part,
        Some(make_span(2, 2, 6, 3)),
        vec![UsageMember::Usage(Box::new(level3))],
    );
    let level1 = Element::Usage(make_usage(
        "Level1",
        UsageKind::Part,
        Some(make_span(1, 0, 7, 1)),
        vec![UsageMember::Usage(Box::new(level2))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![level1],
    };
    let pos = make_position(4, 10);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 4);
    assert_eq!(spans[0].start.line, 4); // Comment
    assert_eq!(spans[1].start.line, 3); // Level3
    assert_eq!(spans[2].start.line, 2); // Level2
    assert_eq!(spans[3].start.line, 1); // Level1
}

#[test]
fn test_collect_usage_spans_stops_at_first_matching_member() {
    let usage1 = make_usage(
        "First",
        UsageKind::Part,
        Some(make_span(2, 0, 3, 1)),
        vec![],
    );
    let usage2 = make_usage(
        "Second",
        UsageKind::Part,
        Some(make_span(4, 0, 5, 1)),
        vec![],
    );
    let parent = Element::Usage(make_usage(
        "Parent",
        UsageKind::Part,
        Some(make_span(1, 0, 6, 1)),
        vec![
            UsageMember::Usage(Box::new(usage1)),
            UsageMember::Usage(Box::new(usage2)),
        ],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![parent],
    };
    let pos = make_position(2, 5);
    let spans = find_selection_spans(&file, pos);

    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].start.line, 2); // First usage
    assert_eq!(spans[1].start.line, 1); // Parent
}

#[test]
fn test_collect_usage_spans_with_different_usage_kinds() {
    // Test various usage kinds work the same
    let kinds = vec![
        UsageKind::Part,
        UsageKind::Port,
        UsageKind::Action,
        UsageKind::Item,
        UsageKind::Attribute,
    ];

    for kind in kinds {
        let usage = Element::Usage(make_usage(
            "Test",
            kind.clone(),
            Some(make_span(1, 0, 3, 1)),
            vec![],
        ));
        let file = SysMLFile {
            namespace: None,
            namespaces: vec![],
            elements: vec![usage],
        };
        let pos = make_position(2, 0);
        let spans = find_selection_spans(&file, pos);

        assert_eq!(spans.len(), 1, "Failed for kind {:?}", kind);
    }
}

#[test]
fn test_collect_usage_spans_returns_false_when_not_containing() {
    let usage = Element::Usage(make_usage(
        "Test",
        UsageKind::Part,
        Some(make_span(1, 0, 3, 1)),
        vec![],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![usage],
    };
    let pos = make_position(5, 0); // Outside usage
    let spans = find_selection_spans(&file, pos);
    assert!(spans.is_empty());
}

// =============================================================================
// Tests for collect_definition_hints (Issue #132) - Tested through public API
// =============================================================================

#[test]
fn test_collect_definition_hints_empty_definition() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Definition(make_definition(
            "EmptyDef",
            DefinitionKind::Part,
            Some(make_span(1, 0, 3, 0)),
            vec![],
        ))],
    };
    let symbol_table = SymbolTable::new();
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_collect_definition_hints_with_usage_member() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "part1".to_string(),
            create_usage_symbol("part1", Some("Type1")),
        )
        .unwrap();

    let usage = make_usage(
        "part1",
        UsageKind::Part,
        Some(make_span(2, 4, 2, 9)),
        vec![],
    );
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 3, 0)),
        vec![DefinitionMember::Usage(Box::new(usage))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_definition_hints_with_multiple_usages() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "part1".to_string(),
            create_usage_symbol("part1", Some("Type1")),
        )
        .unwrap();
    symbol_table
        .insert(
            "part2".to_string(),
            create_usage_symbol("part2", Some("Type2")),
        )
        .unwrap();

    let usage1 = make_usage(
        "part1",
        UsageKind::Part,
        Some(make_span(2, 4, 2, 9)),
        vec![],
    );
    let usage2 = make_usage(
        "part2",
        UsageKind::Part,
        Some(make_span(3, 4, 3, 9)),
        vec![],
    );
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 4, 0)),
        vec![
            DefinitionMember::Usage(Box::new(usage1)),
            DefinitionMember::Usage(Box::new(usage2)),
        ],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 2);
    assert!(hints[0].label.contains("Type1"));
    assert!(hints[1].label.contains("Type2"));
}

#[test]
fn test_collect_definition_hints_with_comment_member() {
    // Comment members should be skipped
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "part1".to_string(),
            create_usage_symbol("part1", Some("Type1")),
        )
        .unwrap();

    let usage = make_usage(
        "part1",
        UsageKind::Part,
        Some(make_span(2, 4, 2, 9)),
        vec![],
    );
    let comment = make_comment("A comment", Some(make_span(3, 4, 3, 13)));
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 4, 0)),
        vec![
            DefinitionMember::Usage(Box::new(usage)),
            DefinitionMember::Comment(Box::new(comment)),
        ],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Only usage should generate hint, not comment
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_definition_hints_with_different_definition_kinds() {
    // Test various definition kinds
    let kinds = vec![
        DefinitionKind::Part,
        DefinitionKind::Port,
        DefinitionKind::Action,
        DefinitionKind::State,
        DefinitionKind::Item,
        DefinitionKind::Attribute,
    ];

    for kind in kinds {
        let mut symbol_table = SymbolTable::new();
        symbol_table
            .insert(
                "part1".to_string(),
                create_usage_symbol("part1", Some("Type1")),
            )
            .unwrap();

        let usage = make_usage(
            "part1",
            UsageKind::Part,
            Some(make_span(2, 4, 2, 9)),
            vec![],
        );
        let definition = Element::Definition(make_definition(
            "TestDef",
            kind.clone(),
            Some(make_span(1, 0, 3, 0)),
            vec![DefinitionMember::Usage(Box::new(usage))],
        ));
        let file = SysMLFile {
            namespace: None,
            namespaces: vec![],
            elements: vec![definition],
        };
        let hints = extract_inlay_hints(&file, &symbol_table, None);

        assert_eq!(hints.len(), 1, "Failed for kind {:?}", kind);
    }
}

#[test]
fn test_collect_definition_hints_usage_without_type() {
    // Usage without type should not generate hint
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("part1".to_string(), create_usage_symbol("part1", None))
        .unwrap();

    let usage = make_usage(
        "part1",
        UsageKind::Part,
        Some(make_span(2, 4, 2, 9)),
        vec![],
    );
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 3, 0)),
        vec![DefinitionMember::Usage(Box::new(usage))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert!(hints.is_empty());
}

#[test]
fn test_collect_definition_hints_usage_with_explicit_typing() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "part1".to_string(),
            create_usage_symbol("part1", Some("Type1")),
        )
        .unwrap();

    let relationships = Relationships {
        typed_by: Some("ExplicitType".to_string()),
        ..Default::default()
    };

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships,
        body: vec![],
        span: Some(make_span(2, 4, 2, 9)),
        is_derived: false,
        is_readonly: false,
    };

    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 3, 0)),
        vec![DefinitionMember::Usage(Box::new(usage))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    // Should not generate hint if explicit typing exists
    assert!(hints.is_empty());
}

#[test]
fn test_collect_definition_hints_with_range_filtering() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "part1".to_string(),
            create_usage_symbol("part1", Some("Type1")),
        )
        .unwrap();
    symbol_table
        .insert(
            "part2".to_string(),
            create_usage_symbol("part2", Some("Type2")),
        )
        .unwrap();

    let usage1 = make_usage(
        "part1",
        UsageKind::Part,
        Some(make_span(2, 4, 2, 9)),
        vec![],
    );
    let usage2 = make_usage(
        "part2",
        UsageKind::Part,
        Some(make_span(10, 4, 10, 9)),
        vec![],
    );
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 11, 0)),
        vec![
            DefinitionMember::Usage(Box::new(usage1)),
            DefinitionMember::Usage(Box::new(usage2)),
        ],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };

    // Range that includes only first usage
    let range = Some((make_position(1, 0), make_position(5, 0)));
    let hints = extract_inlay_hints(&file, &symbol_table, range);

    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_collect_definition_hints_position_calculation() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "myPart".to_string(),
            create_usage_symbol("myPart", Some("MyType")),
        )
        .unwrap();

    let usage = make_usage(
        "myPart",
        UsageKind::Part,
        Some(make_span(5, 20, 5, 26)),
        vec![],
    );
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 6, 0)),
        vec![DefinitionMember::Usage(Box::new(usage))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].position.line, 5);
    assert_eq!(hints[0].position.column, 26); // 20 + 6 (length of "myPart")
}

#[test]
fn test_collect_definition_hints_label_format() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("x".to_string(), create_usage_symbol("x", Some("MyType")))
        .unwrap();

    let usage = make_usage("x", UsageKind::Part, Some(make_span(2, 4, 2, 5)), vec![]);
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 3, 0)),
        vec![DefinitionMember::Usage(Box::new(usage))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":\n MyType");
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(!hints[0].padding_left);
    assert!(hints[0].padding_right);
}

// =============================================================================
// Integration Tests - Multiple functions working together
// =============================================================================

#[test]
fn test_integration_definition_with_nested_usages_selection_and_hints() {
    // Test that both selection and hints work together with nested structure
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "inner".to_string(),
            create_usage_symbol("inner", Some("InnerType")),
        )
        .unwrap();

    let inner_usage = make_usage(
        "inner",
        UsageKind::Part,
        Some(make_span(3, 8, 3, 13)),
        vec![],
    );
    let outer_usage = make_usage(
        "outer",
        UsageKind::Part,
        Some(make_span(2, 4, 4, 4)),
        vec![UsageMember::Usage(Box::new(inner_usage))],
    );
    let definition = Element::Definition(make_definition(
        "TestDef",
        DefinitionKind::Part,
        Some(make_span(1, 0, 5, 0)),
        vec![DefinitionMember::Usage(Box::new(outer_usage))],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![definition],
    };

    // Test selection spans
    let pos = make_position(3, 10);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 3); // inner usage, outer usage, definition

    // Test inlay hints
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert_eq!(hints.len(), 1); // Only inner has a type
    assert!(hints[0].label.contains("InnerType"));
}

#[test]
fn test_integration_package_definition_usage_hierarchy() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "part".to_string(),
            create_usage_symbol("part", Some("PartType")),
        )
        .unwrap();

    let usage = make_usage(
        "part",
        UsageKind::Part,
        Some(make_span(4, 8, 4, 12)),
        vec![],
    );
    let definition = make_definition(
        "Def",
        DefinitionKind::Part,
        Some(make_span(3, 4, 5, 4)),
        vec![DefinitionMember::Usage(Box::new(usage))],
    );
    let package = Element::Package(make_package(
        "Pkg",
        Some(make_span(2, 0, 6, 0)),
        vec![Element::Definition(definition)],
    ));
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![package],
    };

    // Test selection at usage level
    let pos = make_position(4, 10);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 3); // usage, definition, package

    // Test hints
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("PartType"));
}

#[test]
fn test_edge_case_zero_position() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(0, 0, 5, 1)),
            vec![],
        ))],
    };
    let pos = make_position(0, 0);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_edge_case_large_line_numbers() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1000, 0, 2000, 1)),
            vec![],
        ))],
    };
    let pos = make_position(1500, 0);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}

#[test]
fn test_edge_case_large_column_numbers() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(make_package(
            "Test",
            Some(make_span(1, 0, 1, 1000)),
            vec![],
        ))],
    };
    let pos = make_position(1, 500);
    let spans = find_selection_spans(&file, pos);
    assert_eq!(spans.len(), 1);
}
