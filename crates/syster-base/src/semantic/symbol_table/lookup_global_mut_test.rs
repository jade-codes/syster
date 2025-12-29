#![allow(clippy::unwrap_used)]

use super::*;

/// Test finding a symbol in the root scope
#[test]
fn test_lookup_global_mut_in_root_scope() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope (scope 0)
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "RootSymbol".to_string(),
        qualified_name: "RootSymbol".to_string(),
    };

    table.insert("RootSymbol".to_string(), symbol).unwrap();

    // lookup_global_mut should find the symbol
    let found = table.lookup_global_mut("RootSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "RootSymbol");
}

/// Test finding a symbol in a nested scope
#[test]
fn test_lookup_global_mut_in_nested_scope() {
    let mut table = SymbolTable::new();

    // Enter a child scope
    table.enter_scope();

    // Insert symbol in child scope (scope 1)
    let symbol = Symbol::Classifier {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "ChildSymbol".to_string(),
        qualified_name: "Root::ChildSymbol".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };

    table.insert("ChildSymbol".to_string(), symbol).unwrap();

    // lookup_global_mut should find the symbol even from a nested scope
    let found = table.lookup_global_mut("ChildSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "ChildSymbol");
}

/// Test finding a symbol across multiple scopes
#[test]
fn test_lookup_global_mut_across_multiple_scopes() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let root_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "RootPkg".to_string(),
        qualified_name: "RootPkg".to_string(),
    };
    table.insert("RootPkg".to_string(), root_symbol).unwrap();

    // Enter scope 1 and insert a symbol
    table.enter_scope();
    let scope1_symbol = Symbol::Classifier {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Scope1Class".to_string(),
        qualified_name: "RootPkg::Scope1Class".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table
        .insert("Scope1Class".to_string(), scope1_symbol)
        .unwrap();

    // Enter scope 2 and insert a symbol
    table.enter_scope();
    let scope2_symbol = Symbol::Feature {
        scope_id: 2,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Scope2Feature".to_string(),
        qualified_name: "RootPkg::Scope1Class::Scope2Feature".to_string(),
        feature_type: Some("String".to_string()),
    };
    table
        .insert("Scope2Feature".to_string(), scope2_symbol)
        .unwrap();

    // Exit back to root scope
    table.exit_scope();
    table.exit_scope();

    // From root scope, lookup_global_mut should still find symbols in nested scopes
    assert!(table.lookup_global_mut("RootPkg").is_some());
    assert!(table.lookup_global_mut("Scope1Class").is_some());
    assert!(table.lookup_global_mut("Scope2Feature").is_some());

    // Verify each symbol is found correctly
    let root = table.lookup_global_mut("RootPkg").unwrap();
    assert_eq!(root.name(), "RootPkg");

    let class = table.lookup_global_mut("Scope1Class").unwrap();
    assert_eq!(class.name(), "Scope1Class");

    let feature = table.lookup_global_mut("Scope2Feature").unwrap();
    assert_eq!(feature.name(), "Scope2Feature");
}

/// Test when symbol doesn't exist (returns None)
#[test]
fn test_lookup_global_mut_nonexistent_symbol() {
    let mut table = SymbolTable::new();

    // Add some symbols
    table
        .insert(
            "ExistingSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "ExistingSymbol".to_string(),
                qualified_name: "ExistingSymbol".to_string(),
            },
        )
        .unwrap();

    // Try to find a non-existent symbol
    let found = table.lookup_global_mut("NonExistentSymbol");
    assert!(found.is_none());
}

/// Test mutable access to modify symbol
#[test]
fn test_lookup_global_mut_mutable_access() {
    let mut table = SymbolTable::new();

    // Insert a symbol
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MutablePkg".to_string(),
        qualified_name: "MutablePkg".to_string(),
    };

    table.insert("MutablePkg".to_string(), symbol).unwrap();

    // Get mutable reference and verify initial state
    let found = table.lookup_global_mut("MutablePkg");
    assert!(found.is_some());

    let symbol_mut = found.unwrap();
    assert_eq!(symbol_mut.references().len(), 0);

    // Add a reference
    symbol_mut.add_reference(SymbolReference {
        file: "test.sysml".to_string(),
        span: crate::core::Span {
            start: crate::core::Position { line: 1, column: 1 },
            end: crate::core::Position {
                line: 1,
                column: 10,
            },
        },
    });

    // Verify the reference was added by looking it up again
    let found_again = table.lookup_global_mut("MutablePkg");
    assert!(found_again.is_some());
    assert_eq!(found_again.unwrap().references().len(), 1);
}

/// Test with different symbol types
#[test]
fn test_lookup_global_mut_different_symbol_types() {
    let mut table = SymbolTable::new();

    // Insert Package
    table
        .insert(
            "MyPackage".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyPackage".to_string(),
                qualified_name: "MyPackage".to_string(),
            },
        )
        .unwrap();

    // Insert Classifier
    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Insert Feature
    table
        .insert(
            "MyFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyFeature".to_string(),
                qualified_name: "MyClass::MyFeature".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    // Insert Definition
    table
        .insert(
            "MyDef".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyDef".to_string(),
                qualified_name: "MyDef".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Insert Usage
    table
        .insert(
            "MyUsage".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                usage_type: None,
                semantic_role: None,
                references: Vec::new(),
                name: "MyUsage".to_string(),
                qualified_name: "MyUsage".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    // Insert Alias
    table
        .insert(
            "MyAlias".to_string(),
            Symbol::Alias {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyAlias".to_string(),
                qualified_name: "MyAlias".to_string(),
                target: "MyPackage".to_string(),
            },
        )
        .unwrap();

    // Verify all types can be found
    let pkg = table.lookup_global_mut("MyPackage");
    assert!(pkg.is_some());
    assert!(matches!(pkg.unwrap(), Symbol::Package { .. }));

    let class = table.lookup_global_mut("MyClass");
    assert!(class.is_some());
    assert!(matches!(class.unwrap(), Symbol::Classifier { .. }));

    let feature = table.lookup_global_mut("MyFeature");
    assert!(feature.is_some());
    assert!(matches!(feature.unwrap(), Symbol::Feature { .. }));

    let def = table.lookup_global_mut("MyDef");
    assert!(def.is_some());
    assert!(matches!(def.unwrap(), Symbol::Definition { .. }));

    let usage = table.lookup_global_mut("MyUsage");
    assert!(usage.is_some());
    assert!(matches!(usage.unwrap(), Symbol::Usage { .. }));

    let alias = table.lookup_global_mut("MyAlias");
    assert!(alias.is_some());
    assert!(matches!(alias.unwrap(), Symbol::Alias { .. }));
}

/// Test finding first occurrence when duplicate names exist in different scopes
#[test]
fn test_lookup_global_mut_first_occurrence_with_duplicates() {
    let mut table = SymbolTable::new();

    // Insert symbol with name "Duplicate" in root scope (scope 0)
    let root_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Duplicate".to_string(),
        qualified_name: "Root::Duplicate".to_string(),
    };
    table.insert("Duplicate".to_string(), root_symbol).unwrap();

    // Enter scope 1 and insert symbol with same name
    table.enter_scope();
    let scope1_symbol = Symbol::Classifier {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Duplicate".to_string(),
        qualified_name: "Root::Child::Duplicate".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("Duplicate".to_string(), scope1_symbol).unwrap();

    // Enter scope 2 and insert symbol with same name
    table.enter_scope();
    let scope2_symbol = Symbol::Feature {
        scope_id: 2,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Duplicate".to_string(),
        qualified_name: "Root::Child::GrandChild::Duplicate".to_string(),
        feature_type: Some("String".to_string()),
    };
    table.insert("Duplicate".to_string(), scope2_symbol).unwrap();

    // Exit back to root
    table.exit_scope();
    table.exit_scope();

    // lookup_global_mut should find the first occurrence (in scope 0)
    let found = table.lookup_global_mut("Duplicate");
    assert!(found.is_some());
    
    let symbol = found.unwrap();
    // The first occurrence should be from scope 0 (Package type)
    assert!(matches!(symbol, Symbol::Package { .. }));
    assert_eq!(symbol.qualified_name(), "Root::Duplicate");
    assert_eq!(symbol.scope_id(), 0);
}

/// Test shadowing scenarios - symbol with same name in multiple scopes
#[test]
fn test_lookup_global_mut_shadowing() {
    let mut table = SymbolTable::new();

    // Insert "Symbol" in root scope
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Symbol".to_string(),
            },
        )
        .unwrap();

    // Enter scope and shadow with same name
    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
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

    // lookup_global_mut finds first occurrence (root scope)
    let found = table.lookup_global_mut("Symbol");
    assert!(found.is_some());
    
    // Should be the Package from scope 0, not the Classifier from scope 1
    let symbol = found.unwrap();
    assert!(matches!(symbol, Symbol::Package { .. }));
    assert_eq!(symbol.scope_id(), 0);
}

/// Test global search vs scope chain - verify it searches all scopes regardless of current scope
#[test]
fn test_lookup_global_mut_searches_all_scopes() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    table
        .insert(
            "RootSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "RootSymbol".to_string(),
                qualified_name: "RootSymbol".to_string(),
            },
        )
        .unwrap();

    // Enter scope 1 and add a symbol
    table.enter_scope();
    table
        .insert(
            "Scope1Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Scope1Symbol".to_string(),
                qualified_name: "RootSymbol::Scope1Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Enter scope 2 (but don't add any symbols here)
    table.enter_scope();

    // Enter scope 3 and add a symbol
    let scope3_id = table.enter_scope();
    table
        .insert(
            "Scope3Symbol".to_string(),
            Symbol::Feature {
                scope_id: scope3_id,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Scope3Symbol".to_string(),
                qualified_name: "Root::Child::GrandChild::Scope3Symbol".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    // Exit back to scope 2
    table.exit_scope();

    // From scope 2, lookup_global_mut should still find:
    // - RootSymbol (in scope 0, which is in parent chain)
    // - Scope1Symbol (in scope 1, which is in parent chain)
    // - Scope3Symbol (in scope 3, which is NOT in parent chain)
    
    // This demonstrates that lookup_global_mut searches ALL scopes,
    // not just the current scope chain
    assert!(table.lookup_global_mut("RootSymbol").is_some());
    assert!(table.lookup_global_mut("Scope1Symbol").is_some());
    assert!(table.lookup_global_mut("Scope3Symbol").is_some());

    // Verify we found the correct symbols
    let root = table.lookup_global_mut("RootSymbol").unwrap();
    assert_eq!(root.scope_id(), 0);

    let scope1 = table.lookup_global_mut("Scope1Symbol").unwrap();
    assert_eq!(scope1.scope_id(), 1);

    let scope3 = table.lookup_global_mut("Scope3Symbol").unwrap();
    assert_eq!(scope3.scope_id(), scope3_id);
}

/// Test that lookup_global_mut works when called from different scopes
#[test]
fn test_lookup_global_mut_from_different_current_scopes() {
    let mut table = SymbolTable::new();

    // Add symbol in root
    table
        .insert(
            "GlobalSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "GlobalSymbol".to_string(),
                qualified_name: "GlobalSymbol".to_string(),
            },
        )
        .unwrap();

    // From root scope
    let from_root = table.lookup_global_mut("GlobalSymbol");
    assert!(from_root.is_some());
    assert_eq!(from_root.unwrap().name(), "GlobalSymbol");

    // Enter nested scope
    table.enter_scope();

    // From nested scope
    let from_nested = table.lookup_global_mut("GlobalSymbol");
    assert!(from_nested.is_some());
    assert_eq!(from_nested.unwrap().name(), "GlobalSymbol");

    // Enter deeper nested scope
    table.enter_scope();

    // From deeper nested scope
    let from_deeper = table.lookup_global_mut("GlobalSymbol");
    assert!(from_deeper.is_some());
    assert_eq!(from_deeper.unwrap().name(), "GlobalSymbol");

    // All should find the same symbol regardless of current scope
}

/// Test lookup_global_mut with empty symbol table
#[test]
fn test_lookup_global_mut_empty_table() {
    let mut table = SymbolTable::new();

    // Try to find any symbol in empty table
    let found = table.lookup_global_mut("AnySymbol");
    assert!(found.is_none());
}

/// Test lookup_global_mut after removing symbols from file
#[test]
fn test_lookup_global_mut_after_removing_symbols() {
    let mut table = SymbolTable::new();

    // Add symbol from file1
    table.set_current_file(Some("file1.sysml".to_string()));
    table
        .insert(
            "File1Symbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: Some("file1.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "File1Symbol".to_string(),
                qualified_name: "File1Symbol".to_string(),
            },
        )
        .unwrap();

    // Add symbol from file2
    table.set_current_file(Some("file2.sysml".to_string()));
    table
        .insert(
            "File2Symbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: Some("file2.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "File2Symbol".to_string(),
                qualified_name: "File2Symbol".to_string(),
            },
        )
        .unwrap();

    // Verify both symbols can be found
    assert!(table.lookup_global_mut("File1Symbol").is_some());
    assert!(table.lookup_global_mut("File2Symbol").is_some());

    // Remove symbols from file1
    table.remove_symbols_from_file("file1.sysml");

    // File1Symbol should no longer be found
    assert!(table.lookup_global_mut("File1Symbol").is_none());

    // File2Symbol should still be found
    assert!(table.lookup_global_mut("File2Symbol").is_some());
}
