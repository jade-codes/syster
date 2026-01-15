#![allow(clippy::unwrap_used)]

//! Comprehensive tests for KerML inlay hints extraction.
//!
//! Tests the public API `extract_inlay_hints` which internally uses
//! `collect_hints` and `collect_feature_hints`.

use crate::core::{Position, Span};
use crate::semantic::adapters::kerml::inlay_hints::extract_inlay_hints;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::{InlayHint, InlayHintKind};
use crate::syntax::kerml::ast::{
    Element, Feature, FeatureMember, KerMLFile, Package, TypingRelationship,
};

// =============================================================================
// Helper functions
// =============================================================================

fn make_position(line: usize, column: usize) -> Position {
    Position { line, column }
}

fn make_span(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Span {
    Span {
        start: make_position(start_line, start_col),
        end: make_position(end_line, end_col),
    }
}

fn create_feature_symbol(name: &str, type_name: Option<&str>) -> Symbol {
    Symbol::Feature {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        feature_type: type_name.map(String::from),
        source_file: None,
        span: None,
    }
}

// =============================================================================
// Tests for extract_inlay_hints (public API)
// =============================================================================

#[test]
fn test_extract_inlay_hints_empty_file() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![],
    };
    let symbol_table = SymbolTable::new();
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_feature_without_name() {
    // Feature without a name should not generate hints
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: None,
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 10)),
        })],
    };
    let symbol_table = SymbolTable::new();
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_feature_without_span() {
    // Feature without a span should not generate hints even if it has a name
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: None, // No span
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_feature_with_explicit_typing() {
    // Feature with explicit typing relationship should not generate hints
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![FeatureMember::Typing(TypingRelationship {
                typed: "Real".to_string(),
                span: None,
            })],
            span: Some(make_span(1, 0, 1, 15)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_feature_not_in_symbol_table() {
    // Feature not in symbol table should not generate hints
    let symbol_table = SymbolTable::new(); // Empty table

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 15)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_feature_symbol_without_type() {
    // Feature in symbol table but without a type should not generate hints
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("speed".to_string(), create_feature_symbol("speed", None))
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 15)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_feature_with_inferred_type() {
    // Feature with inferred type from symbol table should generate hint
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 15)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(
        hints[0],
        InlayHint {
            position: Position {
                line: 1,
                column: 5 // After "speed" (0 + 5)
            },
            label: ":  Real".to_string(),
            kind: InlayHintKind::Type,
            padding_left: false,
            padding_right: true,
        }
    );
}

#[test]
fn test_extract_inlay_hints_multiple_features() {
    // Multiple features with different scenarios
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();
    symbol_table
        .insert(
            "distance".to_string(),
            create_feature_symbol("distance", Some("Length")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![
            Element::Feature(Feature {
                name: Some("speed".to_string()),
                direction: None,
                is_const: false,
                is_derived: false,
                body: vec![],
                span: Some(make_span(1, 0, 1, 15)),
            }),
            Element::Feature(Feature {
                name: Some("distance".to_string()),
                direction: None,
                is_const: false,
                is_derived: false,
                body: vec![],
                span: Some(make_span(2, 0, 2, 20)),
            }),
        ],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 2);
    assert_eq!(hints[0].label, ":  Real");
    assert_eq!(hints[1].label, ":  Length");
}

#[test]
fn test_extract_inlay_hints_non_feature_symbol_type() {
    // Symbol that is not a Feature should not generate hints
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "Vehicle".to_string(),
            Symbol::Classifier {
                name: "Vehicle".to_string(),
                qualified_name: "Vehicle".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("Vehicle".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 15)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

// =============================================================================
// Tests for collect_hints (via extract_inlay_hints - Package traversal)
// =============================================================================

#[test]
fn test_extract_inlay_hints_nested_package() {
    // Test that nested packages are traversed correctly
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("Pkg".to_string()),
            elements: vec![Element::Feature(Feature {
                name: Some("speed".to_string()),
                direction: None,
                is_const: false,
                is_derived: false,
                body: vec![],
                span: Some(make_span(2, 4, 2, 20)),
            })],
            span: Some(make_span(1, 0, 3, 0)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":  Real");
}

#[test]
fn test_extract_inlay_hints_deeply_nested_packages() {
    // Test deeply nested packages
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "value".to_string(),
            create_feature_symbol("value", Some("Integer")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("OuterPkg".to_string()),
            elements: vec![Element::Package(Package {
                name: Some("InnerPkg".to_string()),
                elements: vec![Element::Feature(Feature {
                    name: Some("value".to_string()),
                    direction: None,
                    is_const: false,
                    is_derived: false,
                    body: vec![],
                    span: Some(make_span(3, 8, 3, 25)),
                })],
                span: Some(make_span(2, 4, 4, 4)),
            })],
            span: Some(make_span(1, 0, 5, 0)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":  Integer");
}

#[test]
fn test_extract_inlay_hints_package_with_mixed_elements() {
    // Package with features and other element types
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("x".to_string(), create_feature_symbol("x", Some("Real")))
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("Mixed".to_string()),
            elements: vec![
                Element::Feature(Feature {
                    name: Some("x".to_string()),
                    direction: None,
                    is_const: false,
                    is_derived: false,
                    body: vec![],
                    span: Some(make_span(2, 4, 2, 15)),
                }),
                // Other element types are ignored by collect_hints
                Element::Import(crate::syntax::kerml::ast::Import {
                    path: "SomePackage".to_string(),
                    path_span: None,
                    is_recursive: false,
                    is_public: false,
                    kind: crate::syntax::kerml::ast::ImportKind::Normal,
                    span: None,
                }),
            ],
            span: Some(make_span(1, 0, 4, 0)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":  Real");
}

// =============================================================================
// Tests for collect_feature_hints (via extract_inlay_hints - Range filtering)
// =============================================================================

#[test]
fn test_extract_inlay_hints_with_range_inside() {
    // Feature inside the requested range should generate hints
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(5, 0, 5, 15)),
        })],
    };

    let range = Some((make_position(1, 0), make_position(10, 0)));
    let hints = extract_inlay_hints(&file, &symbol_table, range);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":  Real");
}

#[test]
fn test_extract_inlay_hints_with_range_before() {
    // Feature starting before the range should be filtered out
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 15)), // Before range
        })],
    };

    let range = Some((make_position(5, 0), make_position(10, 0)));
    let hints = extract_inlay_hints(&file, &symbol_table, range);

    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_with_range_after() {
    // Feature ending after the range should be filtered out
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(15, 0, 15, 15)), // After range
        })],
    };

    let range = Some((make_position(5, 0), make_position(10, 0)));
    let hints = extract_inlay_hints(&file, &symbol_table, range);

    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_with_range_at_boundary_start() {
    // Feature at the exact start of range
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(5, 0, 5, 15)), // At range start
        })],
    };

    let range = Some((make_position(5, 0), make_position(10, 0)));
    let hints = extract_inlay_hints(&file, &symbol_table, range);

    assert_eq!(hints.len(), 1);
}

#[test]
fn test_extract_inlay_hints_with_range_at_boundary_end() {
    // Feature ending exactly at range boundary should be included
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "speed".to_string(),
            create_feature_symbol("speed", Some("Real")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("speed".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(9, 0, 10, 0)), // Ends exactly at range end
        })],
    };

    let range = Some((make_position(5, 0), make_position(10, 0)));
    let hints = extract_inlay_hints(&file, &symbol_table, range);

    assert_eq!(hints.len(), 1);
}

#[test]
fn test_extract_inlay_hints_with_range_multiple_features_filtered() {
    // Mix of features inside and outside range
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("a".to_string(), create_feature_symbol("a", Some("Type1")))
        .unwrap();
    symbol_table
        .insert("b".to_string(), create_feature_symbol("b", Some("Type2")))
        .unwrap();
    symbol_table
        .insert("c".to_string(), create_feature_symbol("c", Some("Type3")))
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![
            Element::Feature(Feature {
                name: Some("a".to_string()),
                direction: None,
                is_const: false,
                is_derived: false,
                body: vec![],
                span: Some(make_span(1, 0, 1, 5)), // Before range
            }),
            Element::Feature(Feature {
                name: Some("b".to_string()),
                direction: None,
                is_const: false,
                is_derived: false,
                body: vec![],
                span: Some(make_span(5, 0, 5, 5)), // Inside range
            }),
            Element::Feature(Feature {
                name: Some("c".to_string()),
                direction: None,
                is_const: false,
                is_derived: false,
                body: vec![],
                span: Some(make_span(15, 0, 15, 5)), // After range
            }),
        ],
    };

    let range = Some((make_position(4, 0), make_position(10, 0)));
    let hints = extract_inlay_hints(&file, &symbol_table, range);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":  Type2");
}

// =============================================================================
// Tests for hint position calculation
// =============================================================================

#[test]
fn test_extract_inlay_hints_position_after_short_name() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("x".to_string(), create_feature_symbol("x", Some("Real")))
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("x".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 10, 1, 20)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].position.line, 1);
    assert_eq!(hints[0].position.column, 11); // 10 + 1 (length of "x")
}

#[test]
fn test_extract_inlay_hints_position_after_long_name() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert(
            "veryLongFeatureName".to_string(),
            create_feature_symbol("veryLongFeatureName", Some("ComplexType")),
        )
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("veryLongFeatureName".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(5, 20, 5, 50)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].position.line, 5);
    assert_eq!(hints[0].position.column, 39); // 20 + 19 (length of "veryLongFeatureName")
}

// =============================================================================
// Tests for hint formatting
// =============================================================================

#[test]
fn test_extract_inlay_hints_label_format() {
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("x".to_string(), create_feature_symbol("x", Some("MyType")))
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("x".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 10)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":  MyType");
    assert_eq!(hints[0].kind, InlayHintKind::Type);
    assert!(!hints[0].padding_left);
    assert!(hints[0].padding_right);
}

// =============================================================================
// Edge cases
// =============================================================================

#[test]
fn test_extract_inlay_hints_empty_package() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("EmptyPkg".to_string()),
            elements: vec![],
            span: Some(make_span(1, 0, 3, 0)),
        })],
    };
    let symbol_table = SymbolTable::new();
    let hints = extract_inlay_hints(&file, &symbol_table, None);
    assert!(hints.is_empty());
}

#[test]
fn test_extract_inlay_hints_feature_with_non_typing_members() {
    // Feature with other members (not typing) should still generate hints
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("x".to_string(), create_feature_symbol("x", Some("Real")))
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("x".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![FeatureMember::Subsetting(
                crate::syntax::kerml::ast::Subsetting {
                    subset: "base".to_string(),
                    span: None,
                },
            )],
            span: Some(make_span(1, 0, 1, 10)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].label, ":  Real");
}

#[test]
fn test_extract_inlay_hints_zero_length_feature_name() {
    // Edge case: empty string name should not crash
    let mut symbol_table = SymbolTable::new();
    symbol_table
        .insert("".to_string(), create_feature_symbol("", Some("Real")))
        .unwrap();

    let file = KerMLFile {
        namespace: None,
        elements: vec![Element::Feature(Feature {
            name: Some("".to_string()),
            direction: None,
            is_const: false,
            is_derived: false,
            body: vec![],
            span: Some(make_span(1, 0, 1, 10)),
        })],
    };
    let hints = extract_inlay_hints(&file, &symbol_table, None);

    assert_eq!(hints.len(), 1);
    assert_eq!(hints[0].position.column, 0); // 0 + 0 (length of "")
}
