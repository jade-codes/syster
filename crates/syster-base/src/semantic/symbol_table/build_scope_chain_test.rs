#![allow(clippy::unwrap_used)]

use super::*;

/// Test building scope chain for root scope only
/// Expected: Chain contains only the root scope [0]
#[test]
fn test_build_scope_chain_root_only() {
    let table = SymbolTable::new();
    
    // Root scope is always 0
    let chain = table.build_scope_chain(0);
    
    assert_eq!(chain.len(), 1);
    assert_eq!(chain[0], 0);
}

/// Test building scope chain for one child scope
/// Expected: Chain contains [child_scope, root_scope] = [1, 0]
#[test]
fn test_build_scope_chain_one_level() {
    let mut table = SymbolTable::new();
    
    // Enter one child scope (scope 1)
    let child_scope = table.enter_scope();
    assert_eq!(child_scope, 1);
    
    // Build chain from child scope
    let chain = table.build_scope_chain(child_scope);
    
    assert_eq!(chain.len(), 2);
    assert_eq!(chain[0], 1); // Current scope first
    assert_eq!(chain[1], 0); // Then parent (root)
}

/// Test building scope chain for multi-level hierarchy
/// Expected: Chain contains all scopes from deepest to root
#[test]
fn test_build_scope_chain_multi_level() {
    let mut table = SymbolTable::new();
    
    // Create a 3-level hierarchy: 0 -> 1 -> 2
    let scope1 = table.enter_scope(); // scope 1
    let scope2 = table.enter_scope(); // scope 2
    
    assert_eq!(scope1, 1);
    assert_eq!(scope2, 2);
    
    // Build chain from deepest scope (2)
    let chain = table.build_scope_chain(scope2);
    
    assert_eq!(chain.len(), 3);
    assert_eq!(chain[0], 2); // Current (deepest)
    assert_eq!(chain[1], 1); // Parent
    assert_eq!(chain[2], 0); // Grandparent (root)
}

/// Test building scope chain explicitly from root scope
/// Expected: Chain contains only [0]
#[test]
fn test_build_scope_chain_from_root_explicitly() {
    let mut table = SymbolTable::new();
    
    // Create some child scopes but query from root
    table.enter_scope();
    table.enter_scope();
    
    // Build chain from root scope 0
    let chain = table.build_scope_chain(0);
    
    assert_eq!(chain.len(), 1);
    assert_eq!(chain[0], 0);
}

/// Test building scope chain from middle scope in deep hierarchy
/// Expected: Chain from middle scope to root, not including deeper scopes
#[test]
fn test_build_scope_chain_from_middle_scope() {
    let mut table = SymbolTable::new();
    
    // Create hierarchy: 0 -> 1 -> 2 -> 3
    let _scope1 = table.enter_scope(); // scope 1
    let scope2 = table.enter_scope(); // scope 2
    table.enter_scope(); // scope 3 (not used in test)
    
    // Build chain from middle scope (2)
    let chain = table.build_scope_chain(scope2);
    
    assert_eq!(chain.len(), 3);
    assert_eq!(chain[0], 2); // Current (middle)
    assert_eq!(chain[1], 1); // Parent
    assert_eq!(chain[2], 0); // Grandparent (root)
    // Note: scope 3 should NOT be in the chain
}

/// Test building scope chains from different scopes in same table
/// Expected: Each chain is independent and correct for its starting scope
#[test]
fn test_build_scope_chain_multiple_queries() {
    let mut table = SymbolTable::new();
    
    // Create hierarchy: 0 -> 1 -> 2
    let scope1 = table.enter_scope();
    let scope2 = table.enter_scope();
    
    // Build chains from different scopes
    let chain_from_root = table.build_scope_chain(0);
    let chain_from_scope1 = table.build_scope_chain(scope1);
    let chain_from_scope2 = table.build_scope_chain(scope2);
    
    // Verify chain from root
    assert_eq!(chain_from_root.len(), 1);
    assert_eq!(chain_from_root[0], 0);
    
    // Verify chain from scope 1
    assert_eq!(chain_from_scope1.len(), 2);
    assert_eq!(chain_from_scope1[0], 1);
    assert_eq!(chain_from_scope1[1], 0);
    
    // Verify chain from scope 2
    assert_eq!(chain_from_scope2.len(), 3);
    assert_eq!(chain_from_scope2[0], 2);
    assert_eq!(chain_from_scope2[1], 1);
    assert_eq!(chain_from_scope2[2], 0);
}

/// Test building scope chain for deeply nested scopes (5 levels)
/// Expected: Chain contains all 5 scopes in correct order
#[test]
fn test_build_scope_chain_deeply_nested() {
    let mut table = SymbolTable::new();
    
    // Create 5-level hierarchy: 0 -> 1 -> 2 -> 3 -> 4
    let mut last_scope = 0;
    for _ in 1..=4 {
        last_scope = table.enter_scope();
    }
    
    assert_eq!(last_scope, 4);
    
    // Build chain from deepest scope
    let chain = table.build_scope_chain(last_scope);
    
    assert_eq!(chain.len(), 5);
    assert_eq!(chain[0], 4); // Deepest
    assert_eq!(chain[1], 3);
    assert_eq!(chain[2], 2);
    assert_eq!(chain[3], 1);
    assert_eq!(chain[4], 0); // Root
}

/// Test that scope chain reflects the actual parent-child relationships
/// Expected: Chain order matches the scope creation order
#[test]
fn test_build_scope_chain_parent_child_relationship() {
    let mut table = SymbolTable::new();
    
    // scope 0 is root
    let scope1 = table.enter_scope(); // Child of 0
    let scope2 = table.enter_scope(); // Child of 1
    
    // Exit back to scope 1
    table.exit_scope();
    assert_eq!(table.current_scope_id(), scope1);
    
    // Create another child of scope 1 (this will be scope 3)
    let scope3 = table.enter_scope(); // Child of 1, sibling of 2
    assert_eq!(scope3, 3);
    
    // Build chains to verify parent relationships
    let chain2 = table.build_scope_chain(scope2);
    let chain3 = table.build_scope_chain(scope3);
    
    // Both scope 2 and scope 3 should have scope 1 as parent
    assert_eq!(chain2.len(), 3);
    assert_eq!(chain2[0], 2);
    assert_eq!(chain2[1], 1); // Parent
    assert_eq!(chain2[2], 0); // Grandparent
    
    assert_eq!(chain3.len(), 3);
    assert_eq!(chain3[0], 3);
    assert_eq!(chain3[1], 1); // Same parent as scope 2
    assert_eq!(chain3[2], 0); // Same grandparent
}

/// Test building scope chain after complex scope navigation
/// Expected: Chain is correct regardless of current scope position
#[test]
fn test_build_scope_chain_after_scope_navigation() {
    let mut table = SymbolTable::new();
    
    // Create hierarchy and navigate around
    let scope1 = table.enter_scope();
    let scope2 = table.enter_scope();
    table.exit_scope(); // Back to scope 1
    table.exit_scope(); // Back to scope 0
    table.enter_scope(); // Create scope 3 (child of 0)
    
    // Build chains from different scopes
    let chain1 = table.build_scope_chain(scope1);
    let chain2 = table.build_scope_chain(scope2);
    
    // Chains should still be correct
    assert_eq!(chain1.len(), 2);
    assert_eq!(chain1[0], 1);
    assert_eq!(chain1[1], 0);
    
    assert_eq!(chain2.len(), 3);
    assert_eq!(chain2[0], 2);
    assert_eq!(chain2[1], 1);
    assert_eq!(chain2[2], 0);
}
