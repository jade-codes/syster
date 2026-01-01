#![allow(clippy::unwrap_used)]

use super::super::*;

/// Test finding a symbol in the specified scope itself
#[test]
fn test_lookup_from_scope_in_current_scope() {
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

    // Lookup from scope 0 should find it
    let found = table.lookup_from_scope("RootSymbol", 0);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "RootSymbol");
}

/// Test finding a symbol in parent scope when not in specified scope
#[test]
fn test_lookup_from_scope_in_parent_scope() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope (scope 0)
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

    // Enter a child scope (scope 1)
    let child_scope = table.enter_scope();

    // Lookup from child scope should find symbol in parent
    let found = table.lookup_from_scope("ParentSymbol", child_scope);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "ParentSymbol");
}

/// Test finding a symbol in grandparent scope (multi-level traversal)
#[test]
fn test_lookup_from_scope_in_grandparent_scope() {
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
    let grandchild_scope = table.enter_scope();

    // Lookup from grandchild should find symbol in grandparent
    let found = table.lookup_from_scope("GrandparentSymbol", grandchild_scope);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "GrandparentSymbol");
}

/// Test that returns None when symbol doesn't exist in scope chain
#[test]
fn test_lookup_from_scope_not_found() {
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

    // Try to find a non-existent symbol from root scope
    let found = table.lookup_from_scope("NonExistentSymbol", 0);
    assert!(found.is_none());
}

/// Test symbol shadowing - nearest scope takes precedence
#[test]
fn test_lookup_from_scope_symbol_shadowing() {
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
    let child_scope = table.enter_scope();

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

    // Lookup from child scope should find the child scope symbol (shadowing)
    let found = table.lookup_from_scope("Symbol", child_scope);
    assert!(found.is_some());
    let symbol = found.unwrap();
    assert_eq!(symbol.qualified_name(), "Parent::Child::Symbol");
    assert!(matches!(symbol, Symbol::Classifier { .. }));
}

/// Test lookup from root scope (scope 0)
#[test]
fn test_lookup_from_scope_from_root() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "RootSymbol".to_string(),
        qualified_name: "RootSymbol".to_string(),
    };

    table.insert("RootSymbol".to_string(), symbol).unwrap();

    // Enter child scope
    table.enter_scope();

    // Lookup from root scope should only find root symbol, not check children
    let found = table.lookup_from_scope("RootSymbol", 0);
    assert!(found.is_some());
}

/// Test lookup from nested scope doesn't find symbols in sibling scopes
#[test]
fn test_lookup_from_scope_no_sibling_access() {
    let mut table = SymbolTable::new();

    // Enter first child scope
    table.enter_scope();
    let sibling1_symbol = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Sibling1".to_string(),
        qualified_name: "Sibling1".to_string(),
    };
    table
        .insert("Sibling1".to_string(), sibling1_symbol)
        .unwrap();

    // Exit back to root
    table.exit_scope();

    // Enter second child scope
    let sibling2_scope = table.enter_scope();

    // Lookup from sibling2 scope should not find sibling1's symbol
    let found = table.lookup_from_scope("Sibling1", sibling2_scope);
    assert!(found.is_none());
}

/// Test lookup with different symbol types in scope chain
#[test]
fn test_lookup_from_scope_different_symbol_types() {
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
    let feature_scope = table.enter_scope();
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

    // From feature scope, should find all three up the chain
    let pkg = table.lookup_from_scope("RootPkg", feature_scope);
    assert!(pkg.is_some());
    assert!(matches!(pkg.unwrap(), Symbol::Package { .. }));

    let class = table.lookup_from_scope("MyClass", feature_scope);
    assert!(class.is_some());
    assert!(matches!(class.unwrap(), Symbol::Classifier { .. }));

    let feature = table.lookup_from_scope("MyFeature", feature_scope);
    assert!(feature.is_some());
    assert!(matches!(feature.unwrap(), Symbol::Feature { .. }));
}

/// Test lookup from deeply nested scopes
#[test]
fn test_lookup_from_scope_deeply_nested() {
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
    let mut scope_ids = vec![0];
    for i in 1..=4 {
        let scope_id = table.enter_scope();
        scope_ids.push(scope_id);
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
    let deepest_scope = *scope_ids.last().unwrap();
    assert!(table.lookup_from_scope("Level0", deepest_scope).is_some());
    assert!(table.lookup_from_scope("Level1", deepest_scope).is_some());
    assert!(table.lookup_from_scope("Level2", deepest_scope).is_some());
    assert!(table.lookup_from_scope("Level3", deepest_scope).is_some());
    assert!(table.lookup_from_scope("Level4", deepest_scope).is_some());

    // From level 2, we should only find Level0, Level1, and Level2
    let level2_scope = scope_ids[2];
    assert!(table.lookup_from_scope("Level0", level2_scope).is_some());
    assert!(table.lookup_from_scope("Level1", level2_scope).is_some());
    assert!(table.lookup_from_scope("Level2", level2_scope).is_some());
    assert!(table.lookup_from_scope("Level3", level2_scope).is_none());
    assert!(table.lookup_from_scope("Level4", level2_scope).is_none());
}

/// Test that lookup_from_scope doesn't check child scopes
#[test]
fn test_lookup_from_scope_no_child_access() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let root_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Root".to_string(),
        qualified_name: "Root".to_string(),
    };

    table.insert("Root".to_string(), root_symbol).unwrap();

    // Enter child scope and add a symbol there
    table.enter_scope();
    let child_symbol = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Child".to_string(),
        qualified_name: "Child".to_string(),
    };

    table.insert("Child".to_string(), child_symbol).unwrap();

    // Lookup from root scope should NOT find child's symbol
    let found = table.lookup_from_scope("Child", 0);
    assert!(found.is_none());
}

/// Test lookup with alias symbols (note: lookup_from_scope doesn't resolve aliases)
#[test]
fn test_lookup_from_scope_with_alias() {
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
    let child_scope = table.enter_scope();
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

    // lookup_from_scope should find the alias (doesn't resolve it)
    let found = table.lookup_from_scope("AliasSymbol", child_scope);
    assert!(found.is_some());
    assert!(matches!(found.unwrap(), Symbol::Alias { .. }));

    // Should also find the real symbol from child scope
    let real = table.lookup_from_scope("RealSymbol", child_scope);
    assert!(real.is_some());
    assert!(matches!(real.unwrap(), Symbol::Package { .. }));
}

/// Test lookup with Definition and Usage symbols
#[test]
fn test_lookup_from_scope_definition_and_usage() {
    let mut table = SymbolTable::new();

    // Add definition at root
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

    // Add usage in child scope
    let child_scope = table.enter_scope();
    table
        .insert(
            "MyUsage".to_string(),
            Symbol::Usage {
                scope_id: 1,
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

    // From child scope, should find both
    let def = table.lookup_from_scope("MyDef", child_scope);
    assert!(def.is_some());
    assert!(matches!(def.unwrap(), Symbol::Definition { .. }));

    let usage = table.lookup_from_scope("MyUsage", child_scope);
    assert!(usage.is_some());
    assert!(matches!(usage.unwrap(), Symbol::Usage { .. }));

    // From root scope, should only find definition
    let def_from_root = table.lookup_from_scope("MyDef", 0);
    assert!(def_from_root.is_some());

    let usage_from_root = table.lookup_from_scope("MyUsage", 0);
    assert!(usage_from_root.is_none());
}

/// Test lookup multiple times from same scope (idempotence)
#[test]
fn test_lookup_from_scope_idempotent() {
    let mut table = SymbolTable::new();

    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Symbol".to_string(),
        qualified_name: "Symbol".to_string(),
    };

    table.insert("Symbol".to_string(), symbol).unwrap();

    // Multiple lookups should return the same result
    let found1 = table.lookup_from_scope("Symbol", 0);
    let found2 = table.lookup_from_scope("Symbol", 0);
    let found3 = table.lookup_from_scope("Symbol", 0);

    assert!(found1.is_some());
    assert!(found2.is_some());
    assert!(found3.is_some());
    assert_eq!(found1.unwrap().name(), found2.unwrap().name());
    assert_eq!(found2.unwrap().name(), found3.unwrap().name());
}
