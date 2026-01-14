//! Tests for collect_definition_hints function in SysML inlay hints module
//!
//! Issue #132: Tests the private `collect_definition_hints` function through the
//! public API `extract_sysml_inlay_hints`. The function collects type hints for usage
//! members within definitions.
//!
//! All tests follow the principle of testing through the public API only.

use crate::core::{Position, Span};
use crate::semantic::adapters::inlay_hints::extract_sysml_inlay_hints;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::InlayHintKind;
use crate::syntax::sysml::ast::{
    Comment, Definition, DefinitionKind, DefinitionMember, Element, Relationships, SysMLFile,
    Usage, UsageKind,
};

// =============================================================================
// Helper Functions
// =============================================================================

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span::from_coords(start_line, start_col, end_line, end_col)
}

fn make_position(line: usize, column: usize) -> Position {
    Position::new(line, column)
}

fn make_usage(name: &str, kind: UsageKind, span: Option<Span>, _body: Vec<()>) -> Usage {
    Usage {
        kind,
        name: Some(name.to_string()),
        short_name: None,
        short_name_span: None,
        relationships: Default::default(),
        body: vec![],
        span,
        is_derived: false,
        is_const: false,
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
        short_name: None,
        short_name_span: None,
        relationships: Default::default(),
        body,
        span,
    }
}

fn make_comment(content: &str, span: Option<Span>) -> Comment {
    Comment {
        content: content.to_string(),
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
    }
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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);
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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

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
        let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

        assert_eq!(hints.len(), 1, "Failed for kind {kind:?}");
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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

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
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, range);

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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

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
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":\n MyType");
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(!hints[0].padding_left);
    assert!(hints[0].padding_right);
}
