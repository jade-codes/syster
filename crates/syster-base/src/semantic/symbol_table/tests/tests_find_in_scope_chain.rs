#![allow(clippy::unwrap_used)]

use super::super::*;

/// Test finding a symbol in the current scope (first in chain)
#[test]
fn test_find_in_current_scope() {
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

    // lookup_mut should find the symbol using find_in_scope_chain
    let found = table.lookup_mut("RootSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "RootSymbol");
}

/// Test finding a symbol in parent scope
#[test]
fn test_find_in_parent_scope() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let parent_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "ParentSymbol".to_string(),
        qualified_name: "ParentSymbol".to_string(),
    };

    table
        .insert("ParentSymbol".to_string(), parent_symbol)
        .unwrap();

    // Enter a child scope
    table.enter_scope();

    // lookup_mut from child scope should find symbol in parent
    let found = table.lookup_mut("ParentSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "ParentSymbol");
}

/// Test finding a symbol in grandparent scope (multi-level chain)
#[test]
fn test_find_in_grandparent_scope() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope (scope 0)
    let root_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "GrandparentSymbol".to_string(),
        qualified_name: "GrandparentSymbol".to_string(),
    };

    table
        .insert("GrandparentSymbol".to_string(), root_symbol)
        .unwrap();

    // Enter child scope (scope 1)
    table.enter_scope();

    // Enter grandchild scope (scope 2)
    table.enter_scope();

    // lookup_mut from grandchild should find symbol in grandparent
    let found = table.lookup_mut("GrandparentSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "GrandparentSymbol");
}

/// Test that symbol not found in chain returns None
#[test]
fn test_symbol_not_found_in_chain() {
    let mut table = SymbolTable::new();

    // Insert a different symbol
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "ExistingSymbol".to_string(),
        qualified_name: "ExistingSymbol".to_string(),
    };

    table.insert("ExistingSymbol".to_string(), symbol).unwrap();

    // Try to find a non-existent symbol
    let found = table.lookup_mut("NonExistentSymbol");
    assert!(found.is_none());
}

/// Test that symbol in current scope takes precedence over parent scope
#[test]
fn test_symbol_precedence_current_over_parent() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let parent_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Symbol".to_string(),
        qualified_name: "Parent::Symbol".to_string(),
    };

    table.insert("Symbol".to_string(), parent_symbol).unwrap();

    // Enter child scope
    table.enter_scope();

    // Insert symbol with same name in child scope (shadowing)
    let child_symbol = Symbol::Classifier {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Symbol".to_string(),
        qualified_name: "Parent::Child::Symbol".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };

    table.insert("Symbol".to_string(), child_symbol).unwrap();

    // lookup_mut should find the child scope symbol (first in chain)
    let found = table.lookup_mut("Symbol");
    assert!(found.is_some());
    let symbol = found.unwrap();
    assert_eq!(symbol.qualified_name(), "Parent::Child::Symbol");
    // Verify it's the Classifier type, not Package
    assert!(matches!(symbol, Symbol::Classifier { .. }));
}

/// Test mutable access to found symbol
#[test]
fn test_mutable_access_to_found_symbol() {
    let mut table = SymbolTable::new();

    // Insert a symbol
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MutableSymbol".to_string(),
        qualified_name: "MutableSymbol".to_string(),
    };

    table.insert("MutableSymbol".to_string(), symbol).unwrap();

    // Get mutable reference and add a reference to it
    let found = table.lookup_mut("MutableSymbol");
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

    // Verify the reference was added
    let found_again = table.lookup_mut("MutableSymbol");
    assert!(found_again.is_some());
    assert_eq!(found_again.unwrap().references().len(), 1);
}

/// Test finding symbols in deeply nested scopes
#[test]
fn test_deeply_nested_scopes() {
    let mut table = SymbolTable::new();

    // Insert symbol at root (level 0)
    let root_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Level0".to_string(),
        qualified_name: "Level0".to_string(),
    };

    table.insert("Level0".to_string(), root_symbol).unwrap();

    // Create multiple nested scopes (levels 1-4)
    for i in 1..=4 {
        table.enter_scope();
        let symbol = Symbol::Package {
            scope_id: i,
            source_file: None,
            span: None,
            references: Vec::new(),
            name: format!("Level{}", i),
            qualified_name: format!("Level0::Level{}", i),
        };
        table.insert(format!("Level{}", i), symbol).unwrap();
    }

    // From the deepest scope (level 4), we should be able to find all symbols
    assert!(table.lookup_mut("Level0").is_some());
    assert!(table.lookup_mut("Level1").is_some());
    assert!(table.lookup_mut("Level2").is_some());
    assert!(table.lookup_mut("Level3").is_some());
    assert!(table.lookup_mut("Level4").is_some());

    // Exit to level 2
    table.exit_scope(); // level 3
    table.exit_scope(); // level 2

    // From level 2, we should not find Level3 or Level4
    assert!(table.lookup_mut("Level0").is_some());
    assert!(table.lookup_mut("Level1").is_some());
    assert!(table.lookup_mut("Level2").is_some());
    assert!(table.lookup_mut("Level3").is_none());
    assert!(table.lookup_mut("Level4").is_none());
}

/// Test with different symbol types in scope chain
#[test]
fn test_different_symbol_types_in_chain() {
    let mut table = SymbolTable::new();

    // Package at root
    table
        .insert(
            "RootPkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "RootPkg".to_string(),
                qualified_name: "RootPkg".to_string(),
            },
        )
        .unwrap();

    // Enter scope for classifier
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
                qualified_name: "RootPkg::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Enter scope for feature
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
                qualified_name: "RootPkg::MyClass::MyFeature".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    // From deepest scope, should find all three
    assert!(table.lookup_mut("RootPkg").is_some());
    assert!(table.lookup_mut("MyClass").is_some());
    assert!(table.lookup_mut("MyFeature").is_some());

    // Verify they are the correct types
    let pkg = table.lookup_mut("RootPkg").unwrap();
    assert!(matches!(pkg, Symbol::Package { .. }));

    let class = table.lookup_mut("MyClass").unwrap();
    assert!(matches!(class, Symbol::Classifier { .. }));

    let feature = table.lookup_mut("MyFeature").unwrap();
    assert!(matches!(feature, Symbol::Feature { .. }));
}

/// Test that lookup_mut works correctly after entering and exiting scopes
#[test]
fn test_scope_chain_after_enter_exit() {
    let mut table = SymbolTable::new();

    // Add symbol at root
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

    // Enter scope 1
    table.enter_scope();
    table
        .insert(
            "Child1".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Child1".to_string(),
                qualified_name: "Root::Child1".to_string(),
            },
        )
        .unwrap();

    // Enter scope 2
    table.enter_scope();
    table
        .insert(
            "Child2".to_string(),
            Symbol::Package {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Child2".to_string(),
                qualified_name: "Root::Child1::Child2".to_string(),
            },
        )
        .unwrap();

    // Verify all accessible from scope 2
    assert!(table.lookup_mut("Root").is_some());
    assert!(table.lookup_mut("Child1").is_some());
    assert!(table.lookup_mut("Child2").is_some());

    // Exit to scope 1
    table.exit_scope();
    assert!(table.lookup_mut("Root").is_some());
    assert!(table.lookup_mut("Child1").is_some());
    assert!(table.lookup_mut("Child2").is_none());

    // Re-enter scope 2
    table.enter_scope();
    // Child2 is not in the new scope 3, it was in scope 2
    assert!(table.lookup_mut("Child2").is_none());

    // Add it to new scope
    table
        .insert(
            "Child2New".to_string(),
            Symbol::Package {
                scope_id: 3,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Child2New".to_string(),
                qualified_name: "Root::Child1::Child2New".to_string(),
            },
        )
        .unwrap();

    assert!(table.lookup_mut("Child2New").is_some());
    assert!(table.lookup_mut("Child1").is_some());
    assert!(table.lookup_mut("Root").is_some());
}

/// Test with alias symbols in scope chain
#[test]
fn test_alias_symbols_in_chain() {
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

    // Add an alias in child scope
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
                target_span: None,
            },
        )
        .unwrap();

    // lookup_mut should find the alias (note: it doesn't resolve aliases)
    let found = table.lookup_mut("AliasSymbol");
    assert!(found.is_some());
    assert!(matches!(found.unwrap(), Symbol::Alias { .. }));

    // Should still find the real symbol
    let real = table.lookup_mut("RealSymbol");
    assert!(real.is_some());
    assert!(matches!(real.unwrap(), Symbol::Package { .. }));
}

/// Test with empty string as symbol name
#[test]
fn test_empty_string_name() {
    let mut table = SymbolTable::new();

    // Insert a symbol with empty name (edge case)
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "".to_string(),
        qualified_name: "".to_string(),
    };

    // Should be able to insert and find empty string name
    table.insert("".to_string(), symbol).unwrap();

    let found = table.lookup_mut("");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "");
}

/// Test with special characters in symbol name
#[test]
fn test_special_characters_in_name() {
    let mut table = SymbolTable::new();

    // Test with various special characters that might appear in SysML names
    let special_names = vec![
        "name-with-dash",
        "name_with_underscore",
        "name::with::colons",
        "name.with.dots",
        "name123",
    ];

    for name in &special_names {
        let symbol = Symbol::Package {
            scope_id: 0,
            source_file: None,
            span: None,
            references: Vec::new(),
            name: name.to_string(),
            qualified_name: name.to_string(),
        };

        table.insert(name.to_string(), symbol).unwrap();
    }

    // Verify all special names can be found
    for name in special_names {
        let found = table.lookup_mut(name);
        assert!(found.is_some(), "Should find symbol with name: {}", name);
        assert_eq!(found.unwrap().name(), name);
    }
}

/// Test finding symbol when multiple scopes exist but symbol only in one
#[test]
fn test_symbol_in_middle_of_chain() {
    let mut table = SymbolTable::new();

    // Scope 0 - no symbols
    table.enter_scope(); // Scope 1

    // Scope 1 - add a symbol here
    let symbol = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MiddleSymbol".to_string(),
        qualified_name: "MiddleSymbol".to_string(),
    };
    table.insert("MiddleSymbol".to_string(), symbol).unwrap();

    table.enter_scope(); // Scope 2 - no symbols
    table.enter_scope(); // Scope 3 - no symbols

    // From scope 3, should find symbol in scope 1 (middle of chain)
    let found = table.lookup_mut("MiddleSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "MiddleSymbol");
}
