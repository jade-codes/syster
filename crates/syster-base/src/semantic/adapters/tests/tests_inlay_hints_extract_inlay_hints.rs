#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

//! Comprehensive tests for inlay hints extraction
//!
//! Tests cover:
//! - Issue #153: semantic::inlay_hints::extract_inlay_hints
//! - Issue #135: semantic::adapters::sysml::inlay_hints::extract_inlay_hints
//! - Also tests KerML adapter for completeness

use crate::core::{Position, Span};
use crate::semantic::extract_inlay_hints;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::InlayHintKind;
use crate::syntax::SyntaxFile;
use crate::syntax::kerml::ast::{Element as KerMLElement, Feature, FeatureMember, KerMLFile};
use crate::syntax::sysml::ast::{
    Definition, DefinitionKind, DefinitionMember, Element, Package, Relationships, SysMLFile,
    Usage, UsageKind, UsageMember,
};

// ============================================================================
// TESTS FOR semantic::inlay_hints::extract_inlay_hints (Issue #153)
// ============================================================================

#[test]
fn test_extract_inlay_hints_empty_sysml_file() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };
    let symbol_table = SymbolTable::new();
    let syntax_file = SyntaxFile::SysML(file);

    let hints = extract_inlay_hints(&syntax_file, &symbol_table, None);

    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_empty_kerml_file() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![],
    };
    let symbol_table = SymbolTable::new();
    let syntax_file = SyntaxFile::KerML(file);

    let hints = extract_inlay_hints(&syntax_file, &symbol_table, None);

    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_delegates_to_sysml_adapter() {
    let mut symbol_table = SymbolTable::new();

    // Add a symbol with a type
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
            },
        )
        .unwrap();

    // Create a usage without explicit type
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myPart".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 0, 1, 6)),
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
    let syntax_file = SyntaxFile::SysML(file);

    let hints = extract_inlay_hints(&syntax_file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(hints[0].label.contains("Vehicle"));
}

#[test]
fn test_extract_inlay_hints_delegates_to_kerml_adapter() {
    let mut symbol_table = SymbolTable::new();

    // Add a feature symbol with type
    symbol_table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                name: "myFeature".to_string(),
                qualified_name: "myFeature".to_string(),
                scope_id: 0,
                feature_type: Some("Real".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    // Create a feature without explicit typing
    let feature = Feature {
        name: Some("myFeature".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(1, 0, 1, 9)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };
    let syntax_file = SyntaxFile::KerML(file);

    let hints = extract_inlay_hints(&syntax_file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(hints[0].label.contains("Real"));
}

#[test]
fn test_extract_inlay_hints_respects_range_filter_sysml() {
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
    let syntax_file = SyntaxFile::SysML(file);

    // Range that excludes the usage (lines 1-3)
    let range = (
        Position { line: 1, column: 0 },
        Position { line: 3, column: 0 },
    );
    let hints = extract_inlay_hints(&syntax_file, &symbol_table, Some(range));

    // Should be empty because usage is outside range
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_respects_range_filter_kerml() {
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Feature {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                feature_type: Some("String".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    // Feature at line 10
    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(10, 0, 10, 5)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };
    let syntax_file = SyntaxFile::KerML(file);

    // Range that excludes the feature (lines 1-5)
    let range = (
        Position { line: 1, column: 0 },
        Position { line: 5, column: 0 },
    );
    let hints = extract_inlay_hints(&syntax_file, &symbol_table, Some(range));

    // Should be empty because feature is outside range
    assert!(hints.is_empty());
}

// ============================================================================
// TESTS FOR adapters::sysml::inlay_hints::extract_inlay_hints (Issue #135)
// ============================================================================

#[test]
fn test_sysml_extract_inlay_hints_empty_file() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };
    let symbol_table = SymbolTable::new();

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    assert!(hints.is_empty());
}

#[test]
fn test_sysml_usage_without_explicit_type_shows_hint() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    // Add a usage symbol with inferred type
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
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("engine".to_string()),
        relationships: Relationships::default(), // No explicit type
        body: vec![],
        span: Some(Span::from_coords(1, 4, 1, 10)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert_eq!(hints[0].label, ":\n Engine");
    assert_eq!(hints[0].position.line, 1);
    assert_eq!(hints[0].position.column, 10); // After "engine"
    assert!(!hints[0].padding_left);
    assert!(hints[0].padding_right);
}

#[test]
fn test_sysml_usage_with_explicit_type_no_hint() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let symbol_table = SymbolTable::new();

    let relationships = Relationships {
        typed_by: Some("Engine".to_string()),
        ..Default::default()
    };

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("engine".to_string()),
        relationships, // Has explicit type
        body: vec![],
        span: Some(Span::from_coords(1, 4, 1, 10)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // No hint because type is explicit
    assert!(hints.is_empty());
}

#[test]
fn test_sysml_usage_without_name_no_hint() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let symbol_table = SymbolTable::new();

    let usage = Usage {
        kind: UsageKind::Part,
        name: None, // No name
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 4, 1, 10)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // No hint because there's no name
    assert!(hints.is_empty());
}

#[test]
fn test_sysml_usage_without_span_no_hint() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: None, // No span
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // No hint because there's no span
    assert!(hints.is_empty());
}

#[test]
fn test_sysml_usage_not_in_symbol_table_no_hint() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let symbol_table = SymbolTable::new(); // Empty

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("unknownPart".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 4, 1, 15)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // No hint because symbol not found
    assert!(hints.is_empty());
}

#[test]
fn test_sysml_usage_symbol_without_type_no_hint() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    // Usage symbol without usage_type
    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Usage {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: None, // No type
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 4, 1, 9)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // No hint because usage_type is None
    assert!(hints.is_empty());
}

#[test]
fn test_sysml_non_usage_symbol_no_hint() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    // Insert a Definition symbol instead of Usage
    symbol_table
        .insert(
            "part1".to_string(),
            Symbol::Definition {
                name: "part1".to_string(),
                qualified_name: "part1".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 4, 1, 9)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // No hint because symbol is not a Usage variant
    assert!(hints.is_empty());
}

#[test]
fn test_sysml_nested_usages_in_definition() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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
            },
        )
        .unwrap();

    let nested_usage = Usage {
        kind: UsageKind::Part,
        name: Some("wheel".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 8, 3, 13)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let definition = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        relationships: Relationships::default(),
        body: vec![DefinitionMember::Usage(Box::new(nested_usage))],
        span: Some(Span::from_coords(1, 0, 4, 1)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(hints[0].label.contains("Wheel"));
}

#[test]
fn test_sysml_deeply_nested_usages() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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
            },
        )
        .unwrap();

    // Deeply nested: Usage -> Usage -> Usage
    let deep_usage = Usage {
        kind: UsageKind::Part,
        name: Some("sensor".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(5, 12, 5, 18)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let mid_usage = Usage {
        kind: UsageKind::Part,
        name: Some("controller".to_string()),
        relationships: Relationships::default(),
        body: vec![UsageMember::Usage(Box::new(deep_usage))],
        span: Some(Span::from_coords(3, 8, 6, 9)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let top_usage = Usage {
        kind: UsageKind::Part,
        name: Some("system".to_string()),
        relationships: Relationships::default(),
        body: vec![UsageMember::Usage(Box::new(mid_usage))],
        span: Some(Span::from_coords(1, 4, 7, 5)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(top_usage)],
    };

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // Should find the deeply nested usage
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Sensor"));
}

#[test]
fn test_sysml_multiple_usages_multiple_hints() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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
            },
        )
        .unwrap();

    let usage1 = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(1, 0, 1, 5)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let usage2 = Usage {
        kind: UsageKind::Part,
        name: Some("part2".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(2, 0, 2, 5)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage1), Element::Usage(usage2)],
    };

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 2);
    assert!(hints[0].label.contains("Type1"));
    assert!(hints[1].label.contains("Type2"));
}

#[test]
fn test_sysml_range_filter_excludes_usage_before_range() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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

    // Range starts after the usage
    let range = (
        Position { line: 5, column: 0 },
        Position {
            line: 10,
            column: 0,
        },
    );
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, Some(range));

    assert!(hints.is_empty());
}

#[test]
fn test_sysml_range_filter_excludes_usage_after_range() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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

    // Range ends before the usage
    let range = (
        Position { line: 1, column: 0 },
        Position { line: 5, column: 0 },
    );
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, Some(range));

    assert!(hints.is_empty());
}

#[test]
fn test_sysml_range_filter_includes_usage_in_range() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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

    // Range includes the usage
    let range = (
        Position { line: 1, column: 0 },
        Position {
            line: 10,
            column: 0,
        },
    );
    let hints = extract_sysml_inlay_hints(&file, &symbol_table, Some(range));

    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_sysml_package_traversal() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

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
            },
        )
        .unwrap();

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(3, 4, 3, 9)),
        short_name: None,
        short_name_span: None,
        is_derived: false,
        is_const: false,
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    // Should find usage inside package
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Type1"));
}

#[test]
fn test_sysml_hint_position_calculation() {
    use super::super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "myEngine".to_string(),
            Symbol::Usage {
                name: "myEngine".to_string(),
                qualified_name: "myEngine".to_string(),
                scope_id: 0,
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: Some("Engine".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    // Name "myEngine" has 8 characters
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myEngine".to_string()),
        relationships: Relationships::default(),
        body: vec![],
        span: Some(Span::from_coords(5, 10, 5, 18)),
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

    let hints = extract_sysml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    // Hint should be positioned after the name: column 10 + 8 = 18
    assert_eq!(hints[0].position.line, 5);
    assert_eq!(hints[0].position.column, 18);
}

// ============================================================================
// TESTS FOR adapters::kerml::inlay_hints::extract_inlay_hints
// ============================================================================

#[test]
fn test_kerml_extract_inlay_hints_empty_file() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let file = KerMLFile {
        namespace: None,
        elements: vec![],
    };
    let symbol_table = SymbolTable::new();

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    assert!(hints.is_empty());
}

#[test]
fn test_kerml_feature_without_explicit_typing_shows_hint() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "mass".to_string(),
            Symbol::Feature {
                name: "mass".to_string(),
                qualified_name: "mass".to_string(),
                scope_id: 0,
                feature_type: Some("Real".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let feature = Feature {
        name: Some("mass".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![], // No explicit typing
        span: Some(Span::from_coords(1, 8, 1, 12)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert_eq!(hints[0].label, ":  Real");
    assert_eq!(hints[0].position.line, 1);
    assert_eq!(hints[0].position.column, 12); // After "mass"
    assert!(!hints[0].padding_left);
    assert!(hints[0].padding_right);
}

#[test]
fn test_kerml_feature_with_explicit_typing_no_hint() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let symbol_table = SymbolTable::new();

    let feature = Feature {
        name: Some("value".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![FeatureMember::Typing(
            crate::syntax::kerml::ast::TypingRelationship {
                typed: "Integer".to_string(),
                span: None,
            },
        )], // Has typing
        span: Some(Span::from_coords(1, 8, 1, 13)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // No hint because typing is explicit
    assert!(hints.is_empty());
}

#[test]
fn test_kerml_feature_without_name_no_hint() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let symbol_table = SymbolTable::new();

    let feature = Feature {
        name: None, // No name
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(1, 8, 1, 13)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // No hint because there's no name
    assert!(hints.is_empty());
}

#[test]
fn test_kerml_feature_without_span_no_hint() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Feature {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                feature_type: Some("String".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: None, // No span
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // No hint because there's no span
    assert!(hints.is_empty());
}

#[test]
fn test_kerml_feature_not_in_symbol_table_no_hint() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let symbol_table = SymbolTable::new(); // Empty

    let feature = Feature {
        name: Some("unknownFeature".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(1, 8, 1, 22)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // No hint because symbol not found
    assert!(hints.is_empty());
}

#[test]
fn test_kerml_feature_symbol_without_type_no_hint() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    // Feature symbol without feature_type
    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Feature {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                feature_type: None, // No type
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(1, 8, 1, 13)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // No hint because feature_type is None
    assert!(hints.is_empty());
}

#[test]
fn test_kerml_non_feature_symbol_no_hint() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    // Insert a Classifier symbol instead of Feature
    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Classifier {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                kind: "Class".to_string(),
                is_abstract: false,
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(1, 8, 1, 13)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // No hint because symbol is not a Feature variant
    assert!(hints.is_empty());
}

#[test]
fn test_kerml_multiple_features_multiple_hints() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "width".to_string(),
            Symbol::Feature {
                name: "width".to_string(),
                qualified_name: "width".to_string(),
                scope_id: 0,
                feature_type: Some("Real".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    symbol_table
        .insert(
            "height".to_string(),
            Symbol::Feature {
                name: "height".to_string(),
                qualified_name: "height".to_string(),
                scope_id: 0,
                feature_type: Some("Real".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let feature1 = Feature {
        name: Some("width".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(1, 8, 1, 13)),
    };

    let feature2 = Feature {
        name: Some("height".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(2, 8, 2, 14)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Feature(feature1),
            KerMLElement::Feature(feature2),
        ],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 2);
    assert!(hints[0].label.contains("Real"));
    assert!(hints[1].label.contains("Real"));
}

#[test]
fn test_kerml_range_filter_excludes_feature_before_range() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Feature {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                feature_type: Some("Integer".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    // Feature at line 2
    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(2, 0, 2, 5)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    // Range starts after the feature
    let range = (
        Position { line: 5, column: 0 },
        Position {
            line: 10,
            column: 0,
        },
    );
    let hints = extract_kerml_inlay_hints(&file, &symbol_table, Some(range));

    assert!(hints.is_empty());
}

#[test]
fn test_kerml_range_filter_excludes_feature_after_range() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Feature {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                feature_type: Some("Integer".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    // Feature at line 10
    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(10, 0, 10, 5)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    // Range ends before the feature
    let range = (
        Position { line: 1, column: 0 },
        Position { line: 5, column: 0 },
    );
    let hints = extract_kerml_inlay_hints(&file, &symbol_table, Some(range));

    assert!(hints.is_empty());
}

#[test]
fn test_kerml_range_filter_includes_feature_in_range() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Feature {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                feature_type: Some("Integer".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    // Feature at line 5
    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(5, 0, 5, 5)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    // Range includes the feature
    let range = (
        Position { line: 1, column: 0 },
        Position {
            line: 10,
            column: 0,
        },
    );
    let hints = extract_kerml_inlay_hints(&file, &symbol_table, Some(range));

    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Integer"));
}

#[test]
fn test_kerml_package_traversal() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;
    use crate::syntax::kerml::ast::Package as KerMLPackage;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "feat1".to_string(),
            Symbol::Feature {
                name: "feat1".to_string(),
                qualified_name: "feat1".to_string(),
                scope_id: 0,
                feature_type: Some("Boolean".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let feature = Feature {
        name: Some("feat1".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(3, 4, 3, 9)),
    };

    let package = KerMLPackage {
        name: Some("MyPackage".to_string()),
        elements: vec![KerMLElement::Feature(feature)],
        span: None,
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(package)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // Should find feature inside package
    assert_eq!(hints.len(), 1);
    assert!(hints[0].label.contains("Boolean"));
}

#[test]
fn test_kerml_hint_position_calculation() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;

    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "velocity".to_string(),
            Symbol::Feature {
                name: "velocity".to_string(),
                qualified_name: "velocity".to_string(),
                scope_id: 0,
                feature_type: Some("Real".to_string()),
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    // Name "velocity" has 8 characters
    let feature = Feature {
        name: Some("velocity".to_string()),
        direction: None,
        is_const: false,
        is_derived: false,
        body: vec![],
        span: Some(Span::from_coords(7, 12, 7, 20)),
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(feature)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    // Hint should be positioned after the name: column 12 + 8 = 20
    assert_eq!(hints[0].position.line, 7);
    assert_eq!(hints[0].position.column, 20);
}

#[test]
fn test_kerml_non_feature_elements_ignored() {
    use super::super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;
    use crate::syntax::kerml::ast::Classifier;

    let symbol_table = SymbolTable::new();

    let classifier = Classifier {
        kind: crate::syntax::kerml::ast::ClassifierKind::Class,
        name: Some("MyClass".to_string()),
        is_abstract: false,
        body: vec![],
        span: None,
    };

    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Classifier(classifier)],
    };

    let hints = extract_kerml_inlay_hints(&file, &symbol_table, None);

    // No hints because we only process features
    assert!(hints.is_empty());
}
