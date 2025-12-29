#![allow(clippy::unwrap_used)]

use super::*;

/// Test that lookup_mut finds symbols in the current scope (root only)
/// This verifies the scope chain includes the current scope
#[test]
fn test_lookup_in_root_scope() {
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

    // lookup_mut should find it (scope chain: [0])
    let found = table.lookup_mut("RootSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "RootSymbol");
}

/// Test that lookup_mut finds symbols in parent scope
/// This verifies the scope chain includes parent scopes
#[test]
fn test_lookup_through_one_level() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "ParentSymbol".to_string(),
        qualified_name: "ParentSymbol".to_string(),
    };

    table.insert("ParentSymbol".to_string(), symbol).unwrap();

    // Enter child scope
    table.enter_scope();

    // lookup_mut from child should find parent symbol (scope chain: [1, 0])
    let found = table.lookup_mut("ParentSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "ParentSymbol");
}

/// Test that lookup_mut traverses multiple levels
/// This verifies the scope chain includes all ancestors
#[test]
fn test_lookup_through_multi_level() {
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

    // Create 3-level hierarchy: 0 -> 1 -> 2
    table.enter_scope(); // scope 1
    table.enter_scope(); // scope 2

    // lookup_mut from deepest scope should find root symbol (scope chain: [2, 1, 0])
    let found = table.lookup_mut("RootSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "RootSymbol");
}

/// Test that lookup_mut doesn't find symbols in deeper child scopes
/// This verifies the scope chain goes UP the tree, not down
#[test]
fn test_lookup_does_not_search_children() {
    let mut table = SymbolTable::new();

    // Enter child scope and insert symbol there
    table.enter_scope();
    let symbol = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "ChildSymbol".to_string(),
        qualified_name: "ChildSymbol".to_string(),
    };

    table.insert("ChildSymbol".to_string(), symbol).unwrap();

    // Exit back to root scope
    table.exit_scope();

    // lookup_mut from root should NOT find child symbol (scope chain: [0])
    let found = table.lookup_mut("ChildSymbol");
    assert!(found.is_none());
}

/// Test scope chain correctness with symbols at different levels
/// This verifies the scope chain is built correctly for middle scopes
#[test]
fn test_lookup_from_middle_scope() {
    let mut table = SymbolTable::new();

    // Insert symbol in root
    let root_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "RootSymbol".to_string(),
        qualified_name: "RootSymbol".to_string(),
    };

    table.insert("RootSymbol".to_string(), root_symbol).unwrap();

    // Create hierarchy: 0 -> 1 -> 2 -> 3
    table.enter_scope(); // scope 1

    let middle_symbol = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MiddleSymbol".to_string(),
        qualified_name: "MiddleSymbol".to_string(),
    };

    table.insert("MiddleSymbol".to_string(), middle_symbol).unwrap();

    table.enter_scope(); // scope 2
    table.enter_scope(); // scope 3

    // Exit to scope 2 (middle scope)
    table.exit_scope();

    // From scope 2, should find both root and middle symbols (scope chain: [2, 1, 0])
    let found_root = table.lookup_mut("RootSymbol");
    assert!(found_root.is_some());
    assert_eq!(found_root.unwrap().name(), "RootSymbol");

    let found_middle = table.lookup_mut("MiddleSymbol");
    assert!(found_middle.is_some());
    assert_eq!(found_middle.unwrap().name(), "MiddleSymbol");
}

/// Test deeply nested scopes
/// This verifies the scope chain handles deep hierarchies correctly
#[test]
fn test_lookup_deeply_nested() {
    let mut table = SymbolTable::new();

    // Insert symbol in root
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "DeepSymbol".to_string(),
        qualified_name: "DeepSymbol".to_string(),
    };

    table.insert("DeepSymbol".to_string(), symbol).unwrap();

    // Create 5-level hierarchy
    for _ in 1..=5 {
        table.enter_scope();
    }

    // From deepest scope, should find root symbol (scope chain: [5, 4, 3, 2, 1, 0])
    let found = table.lookup_mut("DeepSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "DeepSymbol");
}

/// Test that sibling scopes don't see each other's symbols
/// This verifies scope chain respects parent-child relationships
#[test]
fn test_lookup_sibling_scopes() {
    let mut table = SymbolTable::new();

    // Create scope 1 and add symbol
    table.enter_scope();
    let symbol1 = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Sibling1Symbol".to_string(),
        qualified_name: "Sibling1Symbol".to_string(),
    };

    table.insert("Sibling1Symbol".to_string(), symbol1).unwrap();

    // Exit and create scope 2 (sibling of scope 1)
    table.exit_scope();
    table.enter_scope();

    let symbol2 = Symbol::Package {
        scope_id: 2,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Sibling2Symbol".to_string(),
        qualified_name: "Sibling2Symbol".to_string(),
    };

    table.insert("Sibling2Symbol".to_string(), symbol2).unwrap();

    // From scope 2, should find own symbol but not sibling's
    let found_own = table.lookup_mut("Sibling2Symbol");
    assert!(found_own.is_some());

    let found_sibling = table.lookup_mut("Sibling1Symbol");
    assert!(found_sibling.is_none()); // Should NOT find sibling's symbol
}

/// Test scope precedence - current scope overrides parent
/// This verifies the scope chain searches in correct order (current first)
#[test]
fn test_lookup_scope_precedence() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope
    let root_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "OverrideTest".to_string(),
        qualified_name: "Root::OverrideTest".to_string(),
    };

    table.insert("OverrideTest".to_string(), root_symbol).unwrap();

    // Enter child scope and insert symbol with same name
    table.enter_scope();
    let child_symbol = Symbol::Package {
        scope_id: 1,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "OverrideTest".to_string(),
        qualified_name: "Child::OverrideTest".to_string(),
    };

    table.insert("OverrideTest".to_string(), child_symbol).unwrap();

    // Should find child symbol (scope chain searches current scope first)
    let found = table.lookup_mut("OverrideTest");
    assert!(found.is_some());
    let found_symbol = found.unwrap();
    assert_eq!(found_symbol.qualified_name(), "Child::OverrideTest");
}

/// Test complex scope navigation doesn't break scope chain
/// This verifies scope chain is based on scope ID, not navigation history
#[test]
fn test_lookup_after_navigation() {
    let mut table = SymbolTable::new();

    // Insert symbol in root
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "NavTest".to_string(),
        qualified_name: "NavTest".to_string(),
    };

    table.insert("NavTest".to_string(), symbol).unwrap();

    // Complex navigation
    table.enter_scope(); // scope 1
    table.enter_scope(); // scope 2
    table.exit_scope(); // back to 1
    table.exit_scope(); // back to 0
    table.enter_scope(); // scope 3 (child of 0)

    // From scope 3, should still find root symbol
    let found = table.lookup_mut("NavTest");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "NavTest");
}
