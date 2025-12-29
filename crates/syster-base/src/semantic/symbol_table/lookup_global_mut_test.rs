#![allow(clippy::unwrap_used)]

use super::*;

/// Test finding a symbol in the root scope
#[test]
fn test_lookup_global_mut_in_root_scope() {
    let mut table = SymbolTable::new();

    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "RootPackage".to_string(),
        qualified_name: "RootPackage".to_string(),
    };

    table.insert("RootPackage".to_string(), symbol).unwrap();

    // lookup_global_mut should find the symbol in root scope
    let found = table.lookup_global_mut("RootPackage");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "RootPackage");
}

/// Test finding a symbol in a nested scope
#[test]
fn test_lookup_global_mut_in_nested_scope() {
    let mut table = SymbolTable::new();

    // Enter nested scope
    table.enter_scope();

    let symbol = Symbol::Classifier {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "NestedClass".to_string(),
        qualified_name: "Package::NestedClass".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };

    table.insert("NestedClass".to_string(), symbol).unwrap();

    // Return to root scope
    table.exit_scope();

    // lookup_global_mut should find the symbol even from root scope
    // (unlike lookup_mut which only searches the scope chain)
    let found = table.lookup_global_mut("NestedClass");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "NestedClass");
}

/// Test that symbol not found returns None
#[test]
fn test_lookup_global_mut_not_found() {
    let mut table = SymbolTable::new();

    // Add a different symbol
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

/// Test finding first match when multiple scopes have the same symbol name
#[test]
fn test_lookup_global_mut_first_match() {
    let mut table = SymbolTable::new();

    // Add symbol in root scope
    table
        .insert(
            "CommonName".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "CommonName".to_string(),
                qualified_name: "CommonName".to_string(),
            },
        )
        .unwrap();

    // Enter new scope and add symbol with same name
    table.enter_scope();
    table
        .insert(
            "CommonName".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "CommonName".to_string(),
                qualified_name: "Package::CommonName".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_global_mut returns the first match found (root scope comes first in iteration)
    let found = table.lookup_global_mut("CommonName");
    assert!(found.is_some());
    let symbol = found.unwrap();
    // Should find the Package (from root scope), not the Classifier
    assert!(matches!(symbol, Symbol::Package { .. }));
    assert_eq!(symbol.qualified_name(), "CommonName");
}

/// Test mutable access - adding a reference to the found symbol
#[test]
fn test_lookup_global_mut_mutable_access() {
    let mut table = SymbolTable::new();

    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MutablePkg".to_string(),
        qualified_name: "MutablePkg".to_string(),
    };

    table.insert("MutablePkg".to_string(), symbol).unwrap();

    // Get mutable reference
    let found = table.lookup_global_mut("MutablePkg");
    assert!(found.is_some());

    let symbol_mut = found.unwrap();
    assert_eq!(symbol_mut.references().len(), 0);

    // Add a reference using mutable access
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
    let found_again = table.lookup_global_mut("MutablePkg");
    assert!(found_again.is_some());
    assert_eq!(found_again.unwrap().references().len(), 1);
}

/// Test finding symbols across multiple nested scopes
#[test]
fn test_lookup_global_mut_multiple_scopes() {
    let mut table = SymbolTable::new();

    // Add symbol at root (scope 0)
    table
        .insert(
            "Level0".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Level0".to_string(),
                qualified_name: "Level0".to_string(),
            },
        )
        .unwrap();

    // Add symbols in nested scopes
    table.enter_scope(); // scope 1
    table
        .insert(
            "Level1".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Level1".to_string(),
                qualified_name: "Level0::Level1".to_string(),
            },
        )
        .unwrap();

    table.enter_scope(); // scope 2
    table
        .insert(
            "Level2".to_string(),
            Symbol::Package {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Level2".to_string(),
                qualified_name: "Level0::Level1::Level2".to_string(),
            },
        )
        .unwrap();

    // From any scope, lookup_global_mut should find all symbols
    assert!(table.lookup_global_mut("Level0").is_some());
    assert!(table.lookup_global_mut("Level1").is_some());
    assert!(table.lookup_global_mut("Level2").is_some());

    // Exit to root scope
    table.exit_scope();
    table.exit_scope();

    // From root scope, should still find all symbols globally
    assert!(table.lookup_global_mut("Level0").is_some());
    assert!(table.lookup_global_mut("Level1").is_some());
    assert!(table.lookup_global_mut("Level2").is_some());
}

/// Test with different symbol types
#[test]
fn test_lookup_global_mut_different_symbol_types() {
    let mut table = SymbolTable::new();

    // Package
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

    // Classifier
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

    // Feature
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

    // Definition
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

    // Usage
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

    // Alias
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

    // Verify all symbol types can be found
    assert!(table.lookup_global_mut("MyPackage").is_some());
    assert!(table.lookup_global_mut("MyClass").is_some());
    assert!(table.lookup_global_mut("MyFeature").is_some());
    assert!(table.lookup_global_mut("MyDef").is_some());
    assert!(table.lookup_global_mut("MyUsage").is_some());
    assert!(table.lookup_global_mut("MyAlias").is_some());

    // Verify correct types
    assert!(matches!(
        table.lookup_global_mut("MyPackage").unwrap(),
        Symbol::Package { .. }
    ));
    assert!(matches!(
        table.lookup_global_mut("MyClass").unwrap(),
        Symbol::Classifier { .. }
    ));
    assert!(matches!(
        table.lookup_global_mut("MyFeature").unwrap(),
        Symbol::Feature { .. }
    ));
    assert!(matches!(
        table.lookup_global_mut("MyDef").unwrap(),
        Symbol::Definition { .. }
    ));
    assert!(matches!(
        table.lookup_global_mut("MyUsage").unwrap(),
        Symbol::Usage { .. }
    ));
    assert!(matches!(
        table.lookup_global_mut("MyAlias").unwrap(),
        Symbol::Alias { .. }
    ));
}

/// Test with empty symbol table
#[test]
fn test_lookup_global_mut_empty_table() {
    let mut table = SymbolTable::new();

    // Empty table should return None for any lookup
    let found = table.lookup_global_mut("AnySymbol");
    assert!(found.is_none());
}

/// Test lookup_global_mut finds symbols in sibling scopes
/// This demonstrates the key difference from lookup_mut:
/// lookup_global_mut searches ALL scopes, not just the scope chain
#[test]
fn test_lookup_global_mut_sibling_scopes() {
    let mut table = SymbolTable::new();

    // Create first child scope
    table.enter_scope(); // scope 1
    table
        .insert(
            "SiblingA".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SiblingA".to_string(),
                qualified_name: "Root::SiblingA".to_string(),
            },
        )
        .unwrap();

    // Return to root
    table.exit_scope();

    // Create second child scope (sibling to first)
    table.enter_scope(); // scope 2
    table
        .insert(
            "SiblingB".to_string(),
            Symbol::Package {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SiblingB".to_string(),
                qualified_name: "Root::SiblingB".to_string(),
            },
        )
        .unwrap();

    // From scope 2, lookup_mut would NOT find SiblingA (not in scope chain)
    // But lookup_global_mut SHOULD find it (searches all scopes)
    let found = table.lookup_global_mut("SiblingA");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "SiblingA");

    // And should still find SiblingB in current scope
    let found_b = table.lookup_global_mut("SiblingB");
    assert!(found_b.is_some());
    assert_eq!(found_b.unwrap().name(), "SiblingB");
}

/// Test that modifying a symbol through lookup_global_mut
/// affects the symbol in its original scope
#[test]
fn test_lookup_global_mut_modification_persists() {
    let mut table = SymbolTable::new();

    // Add symbol in nested scope
    table.enter_scope(); // scope 1
    table
        .insert(
            "TestSymbol".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "TestSymbol".to_string(),
                qualified_name: "Root::TestSymbol".to_string(),
            },
        )
        .unwrap();

    // Move to different scope
    table.exit_scope();
    table.enter_scope(); // scope 2 (sibling to scope 1)

    // Modify the symbol from a different scope using lookup_global_mut
    {
        let symbol = table.lookup_global_mut("TestSymbol").unwrap();
        symbol.add_reference(SymbolReference {
            file: "test.sysml".to_string(),
            span: crate::core::Span {
                start: crate::core::Position { line: 5, column: 1 },
                end: crate::core::Position {
                    line: 5,
                    column: 10,
                },
            },
        });
    }

    // Verify modification persists
    let symbol = table.lookup_global_mut("TestSymbol").unwrap();
    assert_eq!(symbol.references().len(), 1);
    assert_eq!(symbol.references()[0].file, "test.sysml");
}

/// Test lookup_global_mut with symbols that have been added after scope changes
#[test]
fn test_lookup_global_mut_after_scope_changes() {
    let mut table = SymbolTable::new();

    // Add symbol in root
    table
        .insert(
            "Initial".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Initial".to_string(),
                qualified_name: "Initial".to_string(),
            },
        )
        .unwrap();

    // Enter and exit multiple scopes
    table.enter_scope(); // scope 1
    table.enter_scope(); // scope 2
    table.exit_scope(); // back to scope 1
    table.enter_scope(); // scope 3

    // Add symbol in current scope (3)
    table
        .insert(
            "AfterChanges".to_string(),
            Symbol::Classifier {
                scope_id: 3,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "AfterChanges".to_string(),
                qualified_name: "Initial::AfterChanges".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Both should be findable globally
    assert!(table.lookup_global_mut("Initial").is_some());
    assert!(table.lookup_global_mut("AfterChanges").is_some());

    // Exit all the way back to root
    table.exit_scope(); // scope 1
    table.exit_scope(); // scope 0

    // Both should still be findable from root
    assert!(table.lookup_global_mut("Initial").is_some());
    assert!(table.lookup_global_mut("AfterChanges").is_some());
}
