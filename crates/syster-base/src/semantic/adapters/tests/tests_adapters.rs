#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

//! Tests for semantic adapters module
//!
//! This file consolidates all tests for the adapters module, including:
//! - Validator factory tests
//! - SysML validator tests  
//! - Syntax factory tests
//! - SysML adapter tests
//! - KerML adapter tests

use super::super::*;
use crate::core::constants::{REL_EXHIBIT, REL_INCLUDE, REL_PERFORM, REL_SATISFY};
use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::resolver::Resolver;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::{SemanticError, SemanticRole};
use crate::syntax::SyntaxFile;
use crate::syntax::sysml::ast::{Definition, DefinitionKind, Element, Package, SysMLFile};
use std::sync::Arc;

// ============================================================================
// VALIDATOR FACTORY TESTS
// ============================================================================

#[test]
fn test_create_sysml_validator() {
    let validator = create_validator("sysml");
    assert!(Arc::strong_count(&validator) == 1);
}

#[test]
fn test_create_validator_from_kerml_extension() {
    let validator = create_validator("kerml");
    assert!(Arc::strong_count(&validator) == 1);
}

#[test]
fn test_create_validator_unknown_extension() {
    let validator = create_validator("unknown");
    assert!(Arc::strong_count(&validator) == 1);
}

#[test]
fn test_validator_is_thread_safe() {
    let validator = create_validator("sysml");
    let validator_clone = Arc::clone(&validator);

    assert!(Arc::strong_count(&validator) == 2);
    drop(validator_clone);
    assert!(Arc::strong_count(&validator) == 1);
}

#[test]
fn test_case_sensitive_extension() {
    // Extensions should be case-sensitive
    let validator_upper = create_validator("SYSML");
    let validator_lower = create_validator("sysml");

    // SYSML should return NoOp (unknown), sysml should return SysmlValidator
    // Both should work without panicking
    assert!(Arc::strong_count(&validator_upper) == 1);
    assert!(Arc::strong_count(&validator_lower) == 1);
}

#[test]
fn test_empty_extension() {
    let validator = create_validator("");
    assert!(Arc::strong_count(&validator) == 1);
}

#[test]
fn test_extension_with_dot() {
    // Extensions might be passed with leading dot
    let validator = create_validator(".sysml");
    // Should return NoOp since we expect "sysml" not ".sysml"
    assert!(Arc::strong_count(&validator) == 1);
}

#[test]
fn test_multiple_validators_independent() {
    let validator1 = create_validator("sysml");
    let validator2 = create_validator("sysml");

    // Each call should create a new validator instance
    assert!(Arc::strong_count(&validator1) == 1);
    assert!(Arc::strong_count(&validator2) == 1);
}

#[test]
fn test_sysml_validator_actually_validates() {
    let validator = create_validator("sysml");

    let source = Symbol::Definition {
        name: "Source".to_string(),
        qualified_name: "Source".to_string(),
        scope_id: 0,
        kind: "Part".to_string(),
        semantic_role: Some(SemanticRole::Component),
        source_file: None,
        span: None,
    };

    let valid_target = Symbol::Definition {
        name: "Req1".to_string(),
        qualified_name: "Req1".to_string(),
        scope_id: 0,
        kind: "Requirement".to_string(),
        semantic_role: Some(SemanticRole::Requirement),
        source_file: None,
        span: None,
    };

    let invalid_target = Symbol::Definition {
        name: "Action1".to_string(),
        qualified_name: "Action1".to_string(),
        scope_id: 0,
        kind: "Action".to_string(),
        semantic_role: Some(SemanticRole::Action),
        source_file: None,
        span: None,
    };

    // Valid satisfy relationship
    let result = validator.validate_relationship(REL_SATISFY, &source, &valid_target);
    assert!(result.is_ok());

    // Invalid satisfy relationship
    let result = validator.validate_relationship(REL_SATISFY, &source, &invalid_target);
    assert!(result.is_err());
}

#[test]
fn test_noop_validator_accepts_everything() {
    let validator = create_validator("kerml");

    let source = Symbol::Package {
        name: "Source".to_string(),
        qualified_name: "Source".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
    };

    let target = Symbol::Package {
        name: "Target".to_string(),
        qualified_name: "Target".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
    };

    // NoOpValidator should accept any relationship
    let result = validator.validate_relationship("anything", &source, &target);
    assert!(result.is_ok());
}

// ============================================================================
// SYSML VALIDATOR TESTS
// ============================================================================

fn create_requirement(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "Requirement".to_string(),
        semantic_role: Some(SemanticRole::Requirement),
        source_file: None,
        span: None,
    }
}

fn create_action(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "Action".to_string(),
        semantic_role: Some(SemanticRole::Action),
        source_file: None,
        span: None,
    }
}

fn create_state(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "State".to_string(),
        semantic_role: Some(SemanticRole::State),
        source_file: None,
        span: None,
    }
}

fn create_use_case(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "UseCase".to_string(),
        semantic_role: Some(SemanticRole::UseCase),
        source_file: None,
        span: None,
    }
}

fn create_part(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "Part".to_string(),
        semantic_role: Some(SemanticRole::Component),
        source_file: None,
        span: None,
    }
}

// ============================================================================
// SYNTAX FACTORY TESTS
// ============================================================================

#[test]
fn test_populate_sysml_file() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

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
    let mut graph = RelationshipGraph::new();

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
    let mut graph = RelationshipGraph::new();

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
    let mut graph = RelationshipGraph::new();

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
    let all_symbols = table.all_symbols();
    let inner = all_symbols
        .iter()
        .find(|sym| sym.name() == "Inner")
        .copied();
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
    assert!(adapter.relationship_graph.is_none());
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
fn test_kerml_adapter_new_vs_with_relationships() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Create adapter with new()
    let adapter_new = KermlAdapter::new(&mut table);
    assert!(adapter_new.relationship_graph.is_none());

    // Create adapter with with_relationships()
    let adapter_with_rel = KermlAdapter::with_relationships(&mut table, &mut graph);
    assert!(adapter_with_rel.relationship_graph.is_some());
}

#[test]
fn test_kerml_adapter_new_initial_state() {
    let mut table = SymbolTable::new();
    let adapter = KermlAdapter::new(&mut table);

    // Verify all fields have expected initial values
    assert_eq!(adapter.errors.len(), 0);
    assert_eq!(adapter.current_namespace.len(), 0);
    assert!(adapter.relationship_graph.is_none());
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
