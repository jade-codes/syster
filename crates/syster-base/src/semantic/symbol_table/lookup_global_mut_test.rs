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

    // Enter a nested scope
    table.enter_scope();

    // Insert symbol in nested scope (scope 1)
    let symbol = Symbol::Classifier {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "NestedSymbol".to_string(),
        qualified_name: "Root::NestedSymbol".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };

    table.insert("NestedSymbol".to_string(), symbol).unwrap();

    // Exit back to root scope
    table.exit_scope();

    // lookup_global_mut should still find the symbol even from root scope
    // This is the key difference from lookup_mut which only searches scope chain
    let found = table.lookup_global_mut("NestedSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "NestedSymbol");
}

/// Test finding a symbol in a sibling scope (not in current scope chain)
#[test]
fn test_lookup_global_mut_in_sibling_scope() {
    let mut table = SymbolTable::new();

    // Create first branch: root -> scope 1
    table.enter_scope();
    let symbol1 = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Branch1Symbol".to_string(),
        qualified_name: "Root::Branch1Symbol".to_string(),
    };
    table.insert("Branch1Symbol".to_string(), symbol1).unwrap();
    table.exit_scope();

    // Create second branch: root -> scope 2
    table.enter_scope();
    let symbol2 = Symbol::Classifier {
        scope_id: 2,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Branch2Symbol".to_string(),
        qualified_name: "Root::Branch2Symbol".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("Branch2Symbol".to_string(), symbol2).unwrap();

    // From Branch2 (scope 2), lookup_global_mut should find symbol in Branch1 (scope 1)
    // This demonstrates global search across all scopes, not just the scope chain
    let found = table.lookup_global_mut("Branch1Symbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "Branch1Symbol");
}

/// Test finding first match when duplicates exist in different scopes
#[test]
fn test_lookup_global_mut_first_match_with_duplicates() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let symbol1 = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "DuplicateName".to_string(),
        qualified_name: "DuplicateName".to_string(),
    };
    table.insert("DuplicateName".to_string(), symbol1).unwrap();

    // Enter nested scope and insert symbol with same name
    table.enter_scope();
    let symbol2 = Symbol::Classifier {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "DuplicateName".to_string(),
        qualified_name: "Root::DuplicateName".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("DuplicateName".to_string(), symbol2).unwrap();

    // lookup_global_mut should return the first match (in scope iteration order)
    let found = table.lookup_global_mut("DuplicateName");
    assert!(found.is_some());
    
    // Since scopes are stored in a vec and iterated in order,
    // the first scope (root) should be found first
    let symbol = found.unwrap();
    assert_eq!(symbol.qualified_name(), "DuplicateName");
    assert!(matches!(symbol, Symbol::Package { .. }));
}

/// Test returning None when symbol doesn't exist
#[test]
fn test_lookup_global_mut_nonexistent_symbol() {
    let mut table = SymbolTable::new();

    // Insert some symbols
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

/// Test mutable access to found symbol
#[test]
fn test_lookup_global_mut_mutable_access() {
    let mut table = SymbolTable::new();

    // Insert a symbol in nested scope
    table.enter_scope();
    let symbol = Symbol::Feature {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MutableSymbol".to_string(),
        qualified_name: "Root::MutableSymbol".to_string(),
        feature_type: Some("String".to_string()),
    };
    table.insert("MutableSymbol".to_string(), symbol).unwrap();
    table.exit_scope();

    // Get mutable reference and add a reference to it
    let found = table.lookup_global_mut("MutableSymbol");
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

    // Verify the reference was added by looking up again
    let found_again = table.lookup_global_mut("MutableSymbol");
    assert!(found_again.is_some());
    assert_eq!(found_again.unwrap().references().len(), 1);
}

/// Test with different symbol types
#[test]
fn test_lookup_global_mut_different_symbol_types() {
    let mut table = SymbolTable::new();

    // Add Package symbol
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

    // Add Classifier symbol in nested scope
    table.enter_scope();
    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "MyPackage::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Add Feature symbol in deeper nested scope
    table.enter_scope();
    table
        .insert(
            "MyFeature".to_string(),
            Symbol::Feature {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyFeature".to_string(),
                qualified_name: "MyPackage::MyClass::MyFeature".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    // Add Definition symbol in another nested scope
    table.enter_scope();
    table
        .insert(
            "MyDef".to_string(),
            Symbol::Definition {
                scope_id: 3,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyDef".to_string(),
                qualified_name: "MyPackage::MyClass::MyFeature::MyDef".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add Usage symbol in another scope
    table.enter_scope();
    table
        .insert(
            "MyUsage".to_string(),
            Symbol::Usage {
                scope_id: 4,
                source_file: None,
                span: None,
                usage_type: None,
                semantic_role: None,
                references: Vec::new(),
                name: "MyUsage".to_string(),
                qualified_name: "MyPackage::MyClass::MyFeature::MyDef::MyUsage".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    // Go back to root scope
    table.exit_scope();
    table.exit_scope();
    table.exit_scope();
    table.exit_scope();

    // From root scope, lookup_global_mut should find all symbols globally
    assert!(table.lookup_global_mut("MyPackage").is_some());
    assert!(table.lookup_global_mut("MyClass").is_some());
    assert!(table.lookup_global_mut("MyFeature").is_some());
    assert!(table.lookup_global_mut("MyDef").is_some());
    assert!(table.lookup_global_mut("MyUsage").is_some());

    // Verify they are the correct types
    let pkg = table.lookup_global_mut("MyPackage").unwrap();
    assert!(matches!(pkg, Symbol::Package { .. }));

    let class = table.lookup_global_mut("MyClass").unwrap();
    assert!(matches!(class, Symbol::Classifier { .. }));

    let feature = table.lookup_global_mut("MyFeature").unwrap();
    assert!(matches!(feature, Symbol::Feature { .. }));

    let def = table.lookup_global_mut("MyDef").unwrap();
    assert!(matches!(def, Symbol::Definition { .. }));

    let usage = table.lookup_global_mut("MyUsage").unwrap();
    assert!(matches!(usage, Symbol::Usage { .. }));
}

/// Test with deeply nested scopes
#[test]
fn test_lookup_global_mut_deeply_nested() {
    let mut table = SymbolTable::new();

    // Create 10 levels of nested scopes
    for i in 0..10 {
        let symbol = Symbol::Package {
            scope_id: i,
            source_file: None,
            span: None,
            references: Vec::new(),
            name: format!("Level{}", i),
            qualified_name: if i == 0 {
                format!("Level{}", i)
            } else {
                format!("Level0::Level{}", i)
            },
        };
        table.insert(format!("Level{}", i), symbol).unwrap();
        
        if i < 9 {
            table.enter_scope();
        }
    }

    // Exit all scopes back to root
    for _ in 1..10 {
        table.exit_scope();
    }

    // From root scope, lookup_global_mut should find all symbols in all nested scopes
    for i in 0..10 {
        let found = table.lookup_global_mut(&format!("Level{}", i));
        assert!(found.is_some(), "Should find Level{}", i);
        assert_eq!(found.unwrap().name(), format!("Level{}", i));
    }
}

/// Test that lookup_global_mut can find symbols in multiple sibling branches
#[test]
fn test_lookup_global_mut_multiple_sibling_branches() {
    let mut table = SymbolTable::new();

    // Create a root symbol
    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    // Create 5 sibling branches from root
    for i in 1..=5 {
        table.enter_scope();
        table
            .insert(
                format!("Branch{}", i),
                Symbol::Classifier {
                    scope_id: i,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: format!("Branch{}", i),
                    qualified_name: format!("Root::Branch{}", i),
                    kind: "Class".to_string(),
                    is_abstract: false,
                },
            )
            .unwrap();
        table.exit_scope();
    }

    // Enter one of the branches
    table.enter_scope(); // This creates a new scope, not one of the existing branches

    // lookup_global_mut should find symbols in all sibling branches
    for i in 1..=5 {
        let found = table.lookup_global_mut(&format!("Branch{}", i));
        assert!(found.is_some(), "Should find Branch{}", i);
    }

    // And should also find the root
    assert!(table.lookup_global_mut("Root").is_some());
}

/// Test with alias symbols
#[test]
fn test_lookup_global_mut_with_aliases() {
    let mut table = SymbolTable::new();

    // Add a real symbol
    table
        .insert(
            "RealSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "RealSymbol".to_string(),
                qualified_name: "RealSymbol".to_string(),
            },
        )
        .unwrap();

    // Add an alias in nested scope
    table.enter_scope();
    table
        .insert(
            "AliasSymbol".to_string(),
            Symbol::Alias {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "AliasSymbol".to_string(),
                qualified_name: "AliasSymbol".to_string(),
                target: "RealSymbol".to_string(),
            },
        )
        .unwrap();

    table.exit_scope();

    // lookup_global_mut should find both the alias and the real symbol
    let alias = table.lookup_global_mut("AliasSymbol");
    assert!(alias.is_some());
    assert!(matches!(alias.unwrap(), Symbol::Alias { .. }));

    let real = table.lookup_global_mut("RealSymbol");
    assert!(real.is_some());
    assert!(matches!(real.unwrap(), Symbol::Package { .. }));
}

/// Test that lookup_global_mut returns None on empty table
#[test]
fn test_lookup_global_mut_empty_table() {
    let mut table = SymbolTable::new();

    let found = table.lookup_global_mut("AnySymbol");
    assert!(found.is_none());
}

/// Test lookup_global_mut with symbols that have similar names
#[test]
fn test_lookup_global_mut_similar_names() {
    let mut table = SymbolTable::new();

    // Add symbols with similar names
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

    table.enter_scope();
    table
        .insert(
            "SymbolTest".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SymbolTest".to_string(),
                qualified_name: "Symbol::SymbolTest".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "TestSymbol".to_string(),
            Symbol::Feature {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "TestSymbol".to_string(),
                qualified_name: "Symbol::SymbolTest::TestSymbol".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    // Each should be found independently
    assert!(table.lookup_global_mut("Symbol").is_some());
    assert!(table.lookup_global_mut("SymbolTest").is_some());
    assert!(table.lookup_global_mut("TestSymbol").is_some());

    // Partial matches should not be found
    assert!(table.lookup_global_mut("Sym").is_none());
    assert!(table.lookup_global_mut("Test").is_none());
}
