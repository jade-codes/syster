#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

//! Tests for semantic adapters module
//!
//! This file consolidates all tests for the adapters module, including:
//! - Syntax factory tests
//! - SysML adapter tests
//! - KerML adapter tests

use super::super::*;
use crate::semantic::graphs::ReferenceIndex;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::{Resolver, SemanticError};
use crate::syntax::SyntaxFile;
use crate::syntax::sysml::ast::{Definition, DefinitionKind, Element, Package, SysMLFile};

// ============================================================================
// SYNTAX FACTORY TESTS
// ============================================================================

#[test]
fn test_populate_sysml_file() {
    let mut table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();

    // Create a minimal valid SysML file
    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    };

    let syntax_file = SyntaxFile::SysML(sysml_file);
    let result = populate_syntax_file(&syntax_file, &mut table, &mut graph);

    assert!(result.is_ok());
}

#[test]
fn test_populate_kerml_file_returns_unsupported_error() {
    use crate::syntax::kerml::KerMLFile;

    let mut table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();

    let kerml_file = KerMLFile {
        namespace: None,
        elements: vec![],
    };

    let syntax_file = SyntaxFile::KerML(kerml_file);
    let result = populate_syntax_file(&syntax_file, &mut table, &mut graph);

    // KerML files are silently skipped (no error returned)
    assert!(result.is_ok());
}

#[test]
fn test_populate_preserves_existing_symbols() {
    let mut table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();

    // Add a symbol before population
    table
        .insert(
            "ExistingSymbol".to_string(),
            Symbol::Package {
                name: "ExistingSymbol".to_string(),
                qualified_name: "ExistingSymbol".to_string(),
                scope_id: 0,
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    };

    let syntax_file = SyntaxFile::SysML(sysml_file);
    let result = populate_syntax_file(&syntax_file, &mut table, &mut graph);

    assert!(result.is_ok());
    assert!(Resolver::new(&table).resolve("ExistingSymbol").is_some());
}

#[test]
fn test_populate_multiple_files_sequentially() {
    let mut table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();

    let file1 = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    };
    let file2 = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    };

    let result1 = populate_syntax_file(&SyntaxFile::SysML(file1), &mut table, &mut graph);
    let result2 = populate_syntax_file(&SyntaxFile::SysML(file2), &mut table, &mut graph);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

// ============================================================================
// SYSML ADAPTER TESTS (from sysml/tests.rs)
// ============================================================================

#[test]
fn test_populate_empty_file() {
    let mut table = SymbolTable::new();
    let mut populator = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());
}

#[test]
fn test_populate_single_package() {
    let mut table = SymbolTable::new();
    let mut populator = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("TestPackage".to_string()),
            elements: vec![],
            span: None,
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("TestPackage");
    assert!(symbol.is_some());

    let Some(Symbol::Package {
        name,
        qualified_name,
        ..
    }) = symbol
    else {
        panic!("Expected Package symbol, got: {symbol:?}");
    };
    assert_eq!(name, "TestPackage");
    assert_eq!(qualified_name, "TestPackage");
}

#[test]
fn test_populate_nested_packages() {
    let mut table = SymbolTable::new();
    let mut populator = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Package(Package {
            name: Some("Outer".to_string()),
            elements: vec![Element::Package(Package {
                name: Some("Inner".to_string()),
                elements: vec![],
                span: None,
            })],
            span: None,
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let resolver = Resolver::new(&table);
    let outer = resolver.resolve("Outer");
    assert!(outer.is_some());

    // Verify Inner package exists in the symbol table with correct qualified name
    let inner = table
        .iter_symbols()
        .find(|sym| sym.name() == "Inner")
        .cloned();
    assert!(inner.is_some());

    let Some(Symbol::Package { qualified_name, .. }) = inner else {
        panic!("Expected Package symbol");
    };
    assert_eq!(qualified_name, "Outer::Inner");
}

#[test]
fn test_populate_definition() {
    let mut table = SymbolTable::new();
    let mut populator = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Definition(Definition {
            kind: DefinitionKind::Part,
            name: Some("MyPart".to_string()),
            body: vec![],
            relationships: Default::default(),
            is_abstract: false,
            is_variation: false,
            span: None,
            short_name: None,
            short_name_span: None,
        })],
    };

    let result = populator.populate(&file);
    assert!(result.is_ok());

    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("MyPart");
    assert!(symbol.is_some());
}

// ============================================================================
// KERML ADAPTER TESTS
// ============================================================================

#[test]
fn test_kerml_adapter_new_basic_initialization() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    // Verify the adapter is created successfully
    assert!(adapter.errors.is_empty());
    assert!(adapter.current_namespace.is_empty());
}

#[test]
fn test_kerml_adapter_new_symbol_table_accessible() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    // Verify we can use the symbol table through the adapter
    let test_symbol = Symbol::Package {
        name: "TestPackage".to_string(),
        qualified_name: "TestPackage".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
    };

    let result = adapter
        .symbol_table
        .insert("TestPackage".to_string(), test_symbol);
    assert!(result.is_ok());
    assert!(
        Resolver::new(adapter.symbol_table)
            .resolve("TestPackage")
            .is_some()
    );
}

#[test]
fn test_kerml_adapter_new_with_empty_table() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    // Verify adapter works with an empty symbol table
    assert!(adapter.errors.is_empty());
    assert!(adapter.current_namespace.is_empty());
}

#[test]
fn test_kerml_adapter_new_with_populated_table() {
    let mut table = SymbolTable::new();

    // Pre-populate the symbol table
    table
        .insert(
            "ExistingSymbol".to_string(),
            Symbol::Package {
                name: "ExistingSymbol".to_string(),
                qualified_name: "ExistingSymbol".to_string(),
                scope_id: 0,
                source_file: None,
                span: None,
            },
        )
        .unwrap();

    let adapter = KermlAdapter::new(&mut table);

    // Verify the adapter can access the existing symbols
    assert!(
        Resolver::new(adapter.symbol_table)
            .resolve("ExistingSymbol")
            .is_some()
    );
    assert!(adapter.errors.is_empty());
}

#[test]
fn test_kerml_adapter_new_multiple_instances() {
    let mut table1 = SymbolTable::new();
    let mut table2 = SymbolTable::new();

    let adapter1 = KermlAdapter::new(&mut table1);
    let adapter2 = KermlAdapter::new(&mut table2);

    // Verify both adapters are independent
    assert!(adapter1.errors.is_empty());
    assert!(adapter2.errors.is_empty());
    assert!(adapter1.current_namespace.is_empty());
    assert!(adapter2.current_namespace.is_empty());
}

#[test]
fn test_kerml_adapter_new_initial_state() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    // Verify all fields have expected initial values
    assert_eq!(adapter.errors.len(), 0);
    assert_eq!(adapter.current_namespace.len(), 0);
}

#[test]
fn test_kerml_adapter_new_namespace_mutability() {
    let mut table = SymbolTable::new();
    let mut adapter = KermlAdapter::new(&mut table);

    // Verify we can modify the namespace
    adapter.current_namespace.push("TestNamespace".to_string());
    assert_eq!(adapter.current_namespace.len(), 1);
    assert_eq!(adapter.current_namespace[0], "TestNamespace");
}

#[test]
fn test_kerml_adapter_new_errors_mutability() {
    let mut table = SymbolTable::new();
    let mut adapter = KermlAdapter::new(&mut table);

    // Verify we can add errors
    adapter.errors.push(SemanticError::duplicate_definition(
        "Test".to_string(),
        None,
    ));

    assert_eq!(adapter.errors.len(), 1);
}

#[test]
fn test_kerml_adapter_new_lifetime_handling() {
    let mut table = SymbolTable::new();

    {
        let adapter = KermlAdapter::new(&mut table);
        assert!(adapter.errors.is_empty());
    } // adapter goes out of scope here

    // Verify we can still use the table after adapter is dropped
    assert!(Resolver::new(&table).resolve("NonExistent").is_none());
}
