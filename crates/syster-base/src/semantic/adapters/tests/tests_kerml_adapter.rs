#![allow(clippy::unwrap_used)]

//! Tests for KermlAdapter constructors and initialization.

use super::super::kerml_adapter::KermlAdapter;
use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::resolver::Resolver;
use crate::semantic::symbol_table::{Symbol, SymbolTable};

#[test]
fn test_new_creates_adapter_with_empty_namespace() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    assert!(adapter.current_namespace.is_empty());
}

#[test]
fn test_new_creates_adapter_with_no_relationship_graph() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    assert!(adapter.relationship_graph.is_none());
}

#[test]
fn test_new_creates_adapter_with_empty_errors() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    assert!(adapter.errors.is_empty());
}

#[test]
fn test_new_adapter_can_access_symbol_table() {
    let mut table = SymbolTable::new();

    // Add a symbol to the table before creating the adapter
    table
        .insert(
            "TestSymbol".to_string(),
            Symbol::Package {
                name: "TestSymbol".to_string(),
                qualified_name: "TestSymbol".to_string(),
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let adapter = KermlAdapter::new(&mut table);

    // Verify adapter can access the symbol table
    let resolver = Resolver::new(adapter.symbol_table);
    let symbol = resolver.resolve("TestSymbol");
    assert!(symbol.is_some());
}

#[test]
fn test_new_with_empty_symbol_table() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    // Verify the adapter works with an empty symbol table
    assert!(adapter.symbol_table.all_symbols().is_empty());
}

#[test]
fn test_new_with_populated_symbol_table() {
    let mut table = SymbolTable::new();

    // Populate the symbol table with multiple symbols
    for i in 0..5 {
        table
            .insert(
                format!("Symbol{}", i),
                Symbol::Package {
                    name: format!("Symbol{}", i),
                    qualified_name: format!("Symbol{}", i),
                    scope_id: 0,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                },
            )
            .unwrap();
    }

    let adapter = KermlAdapter::new(&mut table);

    // Verify the adapter has access to all symbols
    assert_eq!(adapter.symbol_table.all_symbols().len(), 5);
}

#[test]
fn test_with_relationships_includes_relationship_graph() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    let adapter = KermlAdapter::with_relationships(&mut table, &mut graph);

    assert!(adapter.relationship_graph.is_some());
}

#[test]
fn test_with_relationships_has_empty_namespace() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    let adapter = KermlAdapter::with_relationships(&mut table, &mut graph);

    assert!(adapter.current_namespace.is_empty());
}

#[test]
fn test_with_relationships_has_empty_errors() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    let adapter = KermlAdapter::with_relationships(&mut table, &mut graph);

    assert!(adapter.errors.is_empty());
}

#[test]
fn test_multiple_adapters_from_same_table() {
    let mut table1 = SymbolTable::new();
    let mut table2 = SymbolTable::new();

    let adapter1 = KermlAdapter::new(&mut table1);
    let adapter2 = KermlAdapter::new(&mut table2);

    // Both adapters should be independently initialized
    assert!(adapter1.current_namespace.is_empty());
    assert!(adapter2.current_namespace.is_empty());
    assert!(adapter1.errors.is_empty());
    assert!(adapter2.errors.is_empty());
}

#[test]
fn test_adapter_initialization_state() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    // Verify all fields are initialized to their default/empty states
    assert!(
        adapter.current_namespace.is_empty(),
        "namespace should be empty"
    );
    assert!(
        adapter.relationship_graph.is_none(),
        "relationship_graph should be None"
    );
    assert!(adapter.errors.is_empty(), "errors should be empty");
}

#[test]
fn test_new_vs_with_relationships_difference() {
    let mut table1 = SymbolTable::new();
    let mut table2 = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    let adapter_without_graph = KermlAdapter::new(&mut table1);
    let adapter_with_graph = KermlAdapter::with_relationships(&mut table2, &mut graph);

    // The key difference is the relationship_graph field
    assert!(adapter_without_graph.relationship_graph.is_none());
    assert!(adapter_with_graph.relationship_graph.is_some());

    // All other fields should be the same
    assert!(adapter_without_graph.current_namespace.is_empty());
    assert!(adapter_with_graph.current_namespace.is_empty());
    assert!(adapter_without_graph.errors.is_empty());
    assert!(adapter_with_graph.errors.is_empty());
}

#[test]
fn test_adapter_can_be_created_in_nested_scope() {
    // Test that the adapter works correctly even when the symbol table
    // is modified before adapter creation
    let mut table = SymbolTable::new();

    {
        // Add symbols in inner scope
        table
            .insert(
                "InnerSymbol".to_string(),
                Symbol::Package {
                    name: "InnerSymbol".to_string(),
                    qualified_name: "InnerSymbol".to_string(),
                    scope_id: 0,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                },
            )
            .unwrap();
    }

    let adapter = KermlAdapter::new(&mut table);

    // Adapter should still be able to access the symbol
    assert!(
        Resolver::new(adapter.symbol_table)
            .resolve("InnerSymbol")
            .is_some()
    );
}
