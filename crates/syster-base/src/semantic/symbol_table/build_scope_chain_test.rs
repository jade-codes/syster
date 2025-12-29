#![allow(clippy::unwrap_used)]

use super::*;

/// Test building scope chain from the root scope (scope 0)
/// Expected: Chain should contain only the root scope [0]
#[test]
fn test_build_chain_from_root_scope() {
    let table = SymbolTable::new();
    
    // build_scope_chain is private, so we need to test it indirectly
    // through lookup_mut which uses it internally
    // However, for direct testing, we'll create a table and access the internal method
    // Since it's private, we'll test through the public API behavior
    
    // At root scope (0), the chain should be [0]
    let chain = table.build_scope_chain(0);
    assert_eq!(chain, vec![0]);
}

/// Test building scope chain from a child scope
/// Expected: Chain should contain child first, then parent [1, 0]
#[test]
fn test_build_chain_from_child_scope() {
    let mut table = SymbolTable::new();
    
    // Enter a child scope (scope 1)
    table.enter_scope();
    
    let chain = table.build_scope_chain(1);
    assert_eq!(chain, vec![1, 0]);
}

/// Test building scope chain from grandchild scope
/// Expected: Chain should be [2, 1, 0]
#[test]
fn test_build_chain_from_grandchild_scope() {
    let mut table = SymbolTable::new();
    
    // Enter child scope (scope 1)
    table.enter_scope();
    
    // Enter grandchild scope (scope 2)
    table.enter_scope();
    
    let chain = table.build_scope_chain(2);
    assert_eq!(chain, vec![2, 1, 0]);
}

/// Test building scope chain from deeply nested scope
/// Expected: Chain should contain all ancestors in order [4, 3, 2, 1, 0]
#[test]
fn test_build_chain_deeply_nested() {
    let mut table = SymbolTable::new();
    
    // Create 4 levels of nesting
    table.enter_scope(); // scope 1
    table.enter_scope(); // scope 2
    table.enter_scope(); // scope 3
    table.enter_scope(); // scope 4
    
    let chain = table.build_scope_chain(4);
    assert_eq!(chain, vec![4, 3, 2, 1, 0]);
}

/// Test that scope chain only includes direct ancestors, not siblings
#[test]
fn test_chain_excludes_siblings() {
    let mut table = SymbolTable::new();
    
    // Create first child (scope 1)
    table.enter_scope();
    let scope1 = table.current_scope_id();
    table.exit_scope();
    
    // Create second child (scope 2) - sibling to scope 1
    table.enter_scope();
    let scope2 = table.current_scope_id();
    
    // Build chain from scope 2
    let chain = table.build_scope_chain(scope2);
    
    // Chain should only include scope2 and root, not scope1
    assert_eq!(chain, vec![scope2, 0]);
    assert!(!chain.contains(&scope1));
}

/// Test that chain terminates at root even after complex navigation
#[test]
fn test_chain_after_complex_navigation() {
    let mut table = SymbolTable::new();
    
    // Build a tree structure where scope IDs increment sequentially:
    //       0
    //      / \
    //     1   3
    //    /
    //   2
    
    table.enter_scope(); // scope 1 (parent=0)
    table.enter_scope(); // scope 2 (parent=1)
    let scope2 = table.current_scope_id();
    table.exit_scope();  // back to scope 1
    table.exit_scope();  // back to scope 0
    
    table.enter_scope(); // scope 3 (parent=0, sibling of 1)
    
    // Build chain from scope 2 (should be [2, 1, 0])
    let chain = table.build_scope_chain(scope2);
    assert_eq!(chain, vec![2, 1, 0]);
}

/// Test chain includes all ancestors regardless of current scope
#[test]
fn test_chain_independent_of_current_scope() {
    let mut table = SymbolTable::new();
    
    // Create scope hierarchy: 0 -> 1 -> 2
    table.enter_scope(); // scope 1
    table.enter_scope(); // scope 2
    let scope2 = table.current_scope_id();
    table.exit_scope();
    table.exit_scope();
    
    // Now at root scope, but build chain for scope 2
    let chain = table.build_scope_chain(scope2);
    
    // Should still return the correct chain for scope 2
    assert_eq!(chain, vec![2, 1, 0]);
    // Verify we're actually at root
    assert_eq!(table.current_scope_id(), 0);
}

/// Test chain length increases with depth
#[test]
fn test_chain_length_matches_depth() {
    let mut table = SymbolTable::new();
    
    // Root scope: depth 0, chain length 1
    let chain0 = table.build_scope_chain(0);
    assert_eq!(chain0.len(), 1);
    
    table.enter_scope();
    // Depth 1, chain length 2
    let chain1 = table.build_scope_chain(1);
    assert_eq!(chain1.len(), 2);
    
    table.enter_scope();
    // Depth 2, chain length 3
    let chain2 = table.build_scope_chain(2);
    assert_eq!(chain2.len(), 3);
    
    table.enter_scope();
    // Depth 3, chain length 4
    let chain3 = table.build_scope_chain(3);
    assert_eq!(chain3.len(), 4);
}

/// Test chain order is from child to parent (current to root)
#[test]
fn test_chain_order_child_to_parent() {
    let mut table = SymbolTable::new();
    
    table.enter_scope(); // scope 1
    table.enter_scope(); // scope 2
    table.enter_scope(); // scope 3
    
    let chain = table.build_scope_chain(3);
    
    // First element should be the requested scope
    assert_eq!(chain[0], 3);
    // Last element should be root
    assert_eq!(chain[chain.len() - 1], 0);
    // Each element should be larger than the next (working backwards)
    for i in 0..chain.len() - 1 {
        assert!(chain[i] > chain[i + 1]);
    }
}

/// Test multiple sibling branches have correct chains
#[test]
fn test_multiple_sibling_branches() {
    let mut table = SymbolTable::new();
    
    // Create tree (scope IDs increment sequentially):
    //       0
    //      /|\
    //     1 3 5
    //    /     \
    //   2       6
    
    table.enter_scope(); // scope 1 (parent=0)
    table.enter_scope(); // scope 2 (parent=1)
    let scope2 = table.current_scope_id();
    table.exit_scope();  // back to 1
    table.exit_scope();  // back to 0
    
    table.enter_scope(); // scope 3 (parent=0, sibling of 1)
    table.exit_scope();  // back to 0
    
    table.enter_scope(); // scope 4 (parent=0, sibling of 1 and 3)
    table.enter_scope(); // scope 5 (parent=4)
    let scope5 = table.current_scope_id();
    
    // Check chain for scope 2: [2, 1, 0]
    let chain2 = table.build_scope_chain(scope2);
    assert_eq!(chain2, vec![2, 1, 0]);
    
    // Check chain for scope 5: [5, 4, 0]
    let chain5 = table.build_scope_chain(scope5);
    assert_eq!(chain5, vec![5, 4, 0]);
    
    // Verify chains are independent
    assert_ne!(chain2, chain5);
}

/// Test that chain terminates at root even with complex structure
#[test]
fn test_chain_always_ends_at_root() {
    let mut table = SymbolTable::new();
    
    // Create various depths
    for _ in 0..10 {
        table.enter_scope();
    }
    
    // Build chain from deepest scope (should be scope 10)
    let chain = table.build_scope_chain(10);
    
    // Last element must always be 0 (root)
    assert_eq!(chain[chain.len() - 1], 0);
    // Chain should have 11 elements (0 through 10)
    assert_eq!(chain.len(), 11);
}

/// Test chain with mixed depth siblings
#[test]
fn test_chain_with_mixed_depth_siblings() {
    let mut table = SymbolTable::new();
    
    // Create: 0 -> 1 -> 2 -> 3
    table.enter_scope(); // 1
    table.enter_scope(); // 2
    table.enter_scope(); // 3
    let scope3 = table.current_scope_id();
    table.exit_scope();
    
    // Add sibling at depth 2: 2 -> 4
    table.enter_scope(); // 4
    let scope4 = table.current_scope_id();
    
    // Chains should be different but share common ancestors
    let chain3 = table.build_scope_chain(scope3);
    let chain4 = table.build_scope_chain(scope4);
    
    assert_eq!(chain3, vec![3, 2, 1, 0]);
    assert_eq!(chain4, vec![4, 2, 1, 0]);
    
    // They diverge at the first element but share [2, 1, 0]
    assert_ne!(chain3[0], chain4[0]);
    assert_eq!(&chain3[1..], &chain4[1..]);
}
