#![allow(clippy::unwrap_used)]

use super::*;

/// Test finding a symbol in a direct namespace (Package::Symbol)
#[test]
fn test_lookup_recursive_import_direct_child() {
    let mut table = SymbolTable::new();

    // Create a symbol at Package::Symbol
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Package::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_recursive_import should find it with namespace "Package" and name "Symbol"
    let result = table.lookup_recursive_import("Symbol", "Package");
    assert!(result.is_some());
    assert_eq!(result.unwrap().qualified_name(), "Package::Symbol");
}

/// Test finding a deeply nested symbol (Package::Sub::SubSub::Symbol)
#[test]
fn test_lookup_recursive_import_deeply_nested() {
    let mut table = SymbolTable::new();

    // Create a deeply nested symbol
    table
        .insert(
            "DeepSymbol".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "DeepSymbol".to_string(),
                qualified_name: "Package::Sub::SubSub::DeepSymbol".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    // lookup_recursive_import should find it
    let result = table.lookup_recursive_import("DeepSymbol", "Package");
    assert!(result.is_some());
    assert_eq!(
        result.unwrap().qualified_name(),
        "Package::Sub::SubSub::DeepSymbol"
    );
}

/// Test finding a symbol when multiple levels exist but only one matches
#[test]
fn test_lookup_recursive_import_specific_match() {
    let mut table = SymbolTable::new();

    // Create symbols at different levels with the same name
    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "Root::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "MyClass2".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "Package::Sub::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Search in "Package" namespace - should find Package::Sub::MyClass, not Root::MyClass
    let result = table.lookup_recursive_import("MyClass", "Package");
    assert!(result.is_some());
    assert_eq!(result.unwrap().qualified_name(), "Package::Sub::MyClass");
}

/// Test that no match returns None
#[test]
fn test_lookup_recursive_import_no_match() {
    let mut table = SymbolTable::new();

    // Create a symbol that won't match
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "OtherPackage::Symbol".to_string(),
            },
        )
        .unwrap();

    // lookup_recursive_import in "Package" namespace should return None
    let result = table.lookup_recursive_import("Symbol", "Package");
    assert!(result.is_none());
}

/// Test that symbol name not matching returns None
#[test]
fn test_lookup_recursive_import_wrong_name() {
    let mut table = SymbolTable::new();

    // Create a symbol
    table
        .insert(
            "RealSymbol".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "RealSymbol".to_string(),
                qualified_name: "Package::RealSymbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_recursive_import with wrong name should return None
    let result = table.lookup_recursive_import("WrongSymbol", "Package");
    assert!(result.is_none());
}

/// Test with empty symbol table
#[test]
fn test_lookup_recursive_import_empty_table() {
    let table = SymbolTable::new();

    // lookup_recursive_import on empty table should return None
    let result = table.lookup_recursive_import("Symbol", "Package");
    assert!(result.is_none());
}

/// Test that prefix matching is strict (Package::Symbol vs Package2::Symbol)
#[test]
fn test_lookup_recursive_import_strict_prefix() {
    let mut table = SymbolTable::new();

    // Create symbol in Package2
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Package2::Symbol".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // lookup_recursive_import in "Package" should not match "Package2"
    let result = table.lookup_recursive_import("Symbol", "Package");
    assert!(result.is_none());
}

/// Test that suffix matching is strict (Package::Symbol vs Package::Symbol2)
#[test]
fn test_lookup_recursive_import_strict_suffix() {
    let mut table = SymbolTable::new();

    // Create symbol Symbol2
    table
        .insert(
            "Symbol2".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                usage_type: Some("Part".to_string()),
                semantic_role: None,
                references: Vec::new(),
                name: "Symbol2".to_string(),
                qualified_name: "Package::Symbol2".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    // lookup_recursive_import for "Symbol" should not match "Symbol2"
    let result = table.lookup_recursive_import("Symbol", "Package");
    assert!(result.is_none());
}

/// Test finding symbols across multiple scopes
#[test]
fn test_lookup_recursive_import_multiple_scopes() {
    let mut table = SymbolTable::new();

    // Add symbol in root scope
    table
        .insert(
            "Symbol1".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol1".to_string(),
                qualified_name: "Package::Symbol1".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Enter new scope and add another symbol
    table.enter_scope();
    table
        .insert(
            "Symbol2".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol2".to_string(),
                qualified_name: "Package::Sub::Symbol2".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_recursive_import should find symbols in any scope
    let result1 = table.lookup_recursive_import("Symbol1", "Package");
    assert!(result1.is_some());
    assert_eq!(result1.unwrap().qualified_name(), "Package::Symbol1");

    let result2 = table.lookup_recursive_import("Symbol2", "Package");
    assert!(result2.is_some());
    assert_eq!(result2.unwrap().qualified_name(), "Package::Sub::Symbol2");
}

/// Test with symbols at the same depth but different intermediate namespaces
#[test]
fn test_lookup_recursive_import_same_depth_different_paths() {
    let mut table = SymbolTable::new();

    // Package::A::Target
    table
        .insert(
            "Target1".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Target".to_string(),
                qualified_name: "Package::A::Target".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Package::B::Target
    table
        .insert(
            "Target2".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Target".to_string(),
                qualified_name: "Package::B::Target".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_recursive_import should find one of them (first match)
    let result = table.lookup_recursive_import("Target", "Package");
    assert!(result.is_some());
    let qname = result.unwrap().qualified_name();
    // Should be either Package::A::Target or Package::B::Target
    assert!(
        qname == "Package::A::Target" || qname == "Package::B::Target",
        "Expected Package::A::Target or Package::B::Target, got {}",
        qname
    );
}

/// Test with a single-part namespace
#[test]
fn test_lookup_recursive_import_single_part_namespace() {
    let mut table = SymbolTable::new();

    // Create P::MySymbol
    table
        .insert(
            "MySymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MySymbol".to_string(),
                qualified_name: "P::MySymbol".to_string(),
            },
        )
        .unwrap();

    // lookup_recursive_import with single-character namespace
    let result = table.lookup_recursive_import("MySymbol", "P");
    assert!(result.is_some());
    assert_eq!(result.unwrap().qualified_name(), "P::MySymbol");
}

/// Test with nested namespace (searching within a sub-namespace)
#[test]
fn test_lookup_recursive_import_nested_namespace() {
    let mut table = SymbolTable::new();

    // Create Root::Package::Sub::Symbol
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Root::Package::Sub::Symbol".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    // Search with nested namespace "Root::Package"
    let result = table.lookup_recursive_import("Symbol", "Root::Package");
    assert!(result.is_some());
    assert_eq!(
        result.unwrap().qualified_name(),
        "Root::Package::Sub::Symbol"
    );
}

/// Test that qualified names without the namespace prefix are not matched
#[test]
fn test_lookup_recursive_import_does_not_match_without_prefix() {
    let mut table = SymbolTable::new();

    // Create a symbol without the Package prefix
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_recursive_import should not find it
    let result = table.lookup_recursive_import("Symbol", "Package");
    assert!(result.is_none());
}

/// Test with different symbol types
#[test]
fn test_lookup_recursive_import_different_symbol_types() {
    let mut table = SymbolTable::new();

    // Add Package type
    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Pkg".to_string(),
                qualified_name: "Root::Pkg".to_string(),
            },
        )
        .unwrap();

    // Add Classifier type
    table
        .insert(
            "Class".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Class".to_string(),
                qualified_name: "Root::Class".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Add Feature type
    table
        .insert(
            "Feat".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Feat".to_string(),
                qualified_name: "Root::Feat".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    // Add Definition type
    table
        .insert(
            "Def".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Def".to_string(),
                qualified_name: "Root::Def".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add Usage type
    table
        .insert(
            "Use".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                usage_type: None,
                semantic_role: None,
                references: Vec::new(),
                name: "Use".to_string(),
                qualified_name: "Root::Use".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    // Add Alias type
    table
        .insert(
            "Als".to_string(),
            Symbol::Alias {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Als".to_string(),
                qualified_name: "Root::Als".to_string(),
                target: "Root::Class".to_string(),
            },
        )
        .unwrap();

    // All types should be findable
    assert!(table.lookup_recursive_import("Pkg", "Root").is_some());
    assert!(table.lookup_recursive_import("Class", "Root").is_some());
    assert!(table.lookup_recursive_import("Feat", "Root").is_some());
    assert!(table.lookup_recursive_import("Def", "Root").is_some());
    assert!(table.lookup_recursive_import("Use", "Root").is_some());
    assert!(table.lookup_recursive_import("Als", "Root").is_some());
}
