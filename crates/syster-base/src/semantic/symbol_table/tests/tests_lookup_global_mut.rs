#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn test_lookup_global_mut_in_root_scope() {
    let mut table = SymbolTable::new();
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        name: "MyPackage".to_string(),
        qualified_name: "MyPackage".to_string(),
    };

    table
        .insert("MyPackage".to_string(), symbol.clone())
        .unwrap();

    let found = table.lookup_global_mut("MyPackage");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "MyPackage");
}

#[test]
fn test_lookup_global_mut_returns_none_for_nonexistent() {
    let mut table = SymbolTable::new();
    let found = table.lookup_global_mut("DoesNotExist");
    assert!(found.is_none());
}

#[test]
fn test_lookup_global_mut_across_multiple_scopes() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    table
        .insert(
            "RootSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                name: "RootSymbol".to_string(),
                qualified_name: "RootSymbol".to_string(),
            },
        )
        .unwrap();

    // Enter a new scope and insert another symbol
    table.enter_scope();
    table
        .insert(
            "NestedSymbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                name: "NestedSymbol".to_string(),
                qualified_name: "RootSymbol::NestedSymbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Enter another nested scope
    table.enter_scope();
    table
        .insert(
            "DeepSymbol".to_string(),
            Symbol::Feature {
                scope_id: 2,
                source_file: None,
                span: None,
                name: "DeepSymbol".to_string(),
                qualified_name: "RootSymbol::NestedSymbol::DeepSymbol".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    // lookup_global_mut should find symbols in any scope regardless of current scope
    assert!(table.lookup_global_mut("RootSymbol").is_some());
    assert!(table.lookup_global_mut("NestedSymbol").is_some());
    assert!(table.lookup_global_mut("DeepSymbol").is_some());

    // Exit to root scope
    table.exit_scope();
    table.exit_scope();

    // Should still find all symbols globally
    assert!(table.lookup_global_mut("RootSymbol").is_some());
    assert!(table.lookup_global_mut("NestedSymbol").is_some());
    assert!(table.lookup_global_mut("DeepSymbol").is_some());
}

#[test]
fn test_lookup_global_mut_returns_first_match() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    table
        .insert(
            "Duplicate".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: Some("file1.sysml".to_string()),
                span: None,
                name: "Duplicate".to_string(),
                qualified_name: "Duplicate".to_string(),
            },
        )
        .unwrap();

    // Enter new scope and insert another symbol with same name
    table.enter_scope();
    table
        .insert(
            "Duplicate".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: Some("file2.sysml".to_string()),
                span: None,
                name: "Duplicate".to_string(),
                qualified_name: "Scope1::Duplicate".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_global_mut should return the first match found (root scope)
    let found = table.lookup_global_mut("Duplicate");
    assert!(found.is_some());
    assert_eq!(found.unwrap().source_file(), Some("file1.sysml"));
}

#[test]
fn test_lookup_global_mut_mutability() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "MutableSymbol".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                name: "MutableSymbol".to_string(),
                qualified_name: "MutableSymbol".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    // Get mutable reference and verify it exists
    let symbol_ref = table.lookup_global_mut("MutableSymbol");
    assert!(symbol_ref.is_some());
    assert_eq!(symbol_ref.unwrap().name(), "MutableSymbol");
}

#[test]
fn test_lookup_global_mut_different_symbol_types() {
    let mut table = SymbolTable::new();

    // Insert different symbol types in different scopes
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Classifier".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                name: "Classifier".to_string(),
                qualified_name: "Package::Classifier".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Feature".to_string(),
            Symbol::Feature {
                scope_id: 2,
                source_file: None,
                span: None,
                name: "Feature".to_string(),
                qualified_name: "Package::Classifier::Feature".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Definition".to_string(),
            Symbol::Definition {
                scope_id: 3,
                source_file: None,
                span: None,
                name: "Definition".to_string(),
                qualified_name: "Package::Classifier::Feature::Definition".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Usage".to_string(),
            Symbol::Usage {
                scope_id: 4,
                source_file: None,
                span: None,
                name: "Usage".to_string(),
                qualified_name: "Package::Classifier::Feature::Definition::Usage".to_string(),
                kind: "Part".to_string(),
                usage_type: None,
                semantic_role: None,
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Alias".to_string(),
            Symbol::Alias {
                scope_id: 5,
                source_file: None,
                span: None,
                name: "Alias".to_string(),
                qualified_name: "Package::Classifier::Feature::Definition::Usage::Alias"
                    .to_string(),
                target: "Package".to_string(),
                target_span: None,
            },
        )
        .unwrap();

    // All symbol types should be found globally
    assert!(table.lookup_global_mut("Package").is_some());
    assert!(table.lookup_global_mut("Classifier").is_some());
    assert!(table.lookup_global_mut("Feature").is_some());
    assert!(table.lookup_global_mut("Definition").is_some());
    assert!(table.lookup_global_mut("Usage").is_some());
    assert!(table.lookup_global_mut("Alias").is_some());
}

#[test]
fn test_lookup_global_mut_vs_lookup_mut_difference() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    table
        .insert(
            "RootOnly".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                name: "RootOnly".to_string(),
                qualified_name: "RootOnly".to_string(),
            },
        )
        .unwrap();

    // Enter a nested scope
    table.enter_scope();
    table
        .insert(
            "NestedOnly".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                name: "NestedOnly".to_string(),
                qualified_name: "RootOnly::NestedOnly".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Enter another nested scope (child of scope 1)
    table.enter_scope();

    // From this deep scope:
    // - lookup_global_mut should find both symbols
    // - lookup_mut should find both (they're in parent chain)
    assert!(table.lookup_global_mut("RootOnly").is_some());
    assert!(table.lookup_global_mut("NestedOnly").is_some());
    assert!(table.lookup_mut("RootOnly").is_some());
    assert!(table.lookup_mut("NestedOnly").is_some());

    // Exit back to root
    table.exit_scope();
    table.exit_scope();

    // Now insert a symbol in a new branch
    table.enter_scope();
    table
        .insert(
            "OtherBranch".to_string(),
            Symbol::Feature {
                scope_id: 3,
                source_file: None,
                span: None,
                name: "OtherBranch".to_string(),
                qualified_name: "RootOnly::OtherBranch".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    // From this scope:
    // - lookup_global_mut should find "NestedOnly" (searches all scopes)
    // - lookup_mut should NOT find "NestedOnly" (different branch)
    assert!(table.lookup_global_mut("NestedOnly").is_some());
    assert!(table.lookup_mut("NestedOnly").is_none());

    // Both should find symbols in the current chain
    assert!(table.lookup_global_mut("RootOnly").is_some());
    assert!(table.lookup_mut("RootOnly").is_some());
    assert!(table.lookup_global_mut("OtherBranch").is_some());
    assert!(table.lookup_mut("OtherBranch").is_some());
}

#[test]
fn test_lookup_global_mut_with_empty_table() {
    let mut table = SymbolTable::new();
    assert!(table.lookup_global_mut("Anything").is_none());
}

#[test]
fn test_lookup_global_mut_after_scope_operations() {
    let mut table = SymbolTable::new();

    // Add symbols in various scopes
    table
        .insert(
            "Symbol1".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                name: "Symbol1".to_string(),
                qualified_name: "Symbol1".to_string(),
            },
        )
        .unwrap();

    let scope1 = table.enter_scope();
    table
        .insert(
            "Symbol2".to_string(),
            Symbol::Classifier {
                scope_id: scope1,
                source_file: None,
                span: None,
                name: "Symbol2".to_string(),
                qualified_name: "Symbol1::Symbol2".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let scope2 = table.enter_scope();
    table
        .insert(
            "Symbol3".to_string(),
            Symbol::Feature {
                scope_id: scope2,
                source_file: None,
                span: None,
                name: "Symbol3".to_string(),
                qualified_name: "Symbol1::Symbol2::Symbol3".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    // Go back to root
    table.exit_scope();
    table.exit_scope();
    assert_eq!(table.current_scope_id(), 0);

    // All symbols should still be findable globally
    assert!(table.lookup_global_mut("Symbol1").is_some());
    assert!(table.lookup_global_mut("Symbol2").is_some());
    assert!(table.lookup_global_mut("Symbol3").is_some());

    // Enter a completely new scope branch
    table.enter_scope();
    table
        .insert(
            "Symbol4".to_string(),
            Symbol::Definition {
                scope_id: 3,
                source_file: None,
                span: None,
                name: "Symbol4".to_string(),
                qualified_name: "Symbol1::Symbol4".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // All symbols should still be globally accessible
    assert!(table.lookup_global_mut("Symbol1").is_some());
    assert!(table.lookup_global_mut("Symbol2").is_some());
    assert!(table.lookup_global_mut("Symbol3").is_some());
    assert!(table.lookup_global_mut("Symbol4").is_some());
}
