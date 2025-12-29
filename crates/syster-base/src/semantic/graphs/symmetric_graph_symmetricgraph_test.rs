#![allow(clippy::unwrap_used)]

use super::*;

// Tests for SymmetricGraph::new() - Issue #232
#[test]
fn test_new_creates_empty_graph() {
    // Test that new() creates an empty graph with no relationships
    let graph = SymmetricGraph::new();

    // Verify no relationships exist
    assert!(graph.get_related("any_element").is_none());
    assert!(!graph.are_related("element1", "element2"));
}

// Tests for SymmetricGraph::add() - Issue #235
#[test]
fn test_add_single_relationship() {
    // Test adding a single bidirectional relationship
    let mut graph = SymmetricGraph::new();

    graph.add("Element1".to_string(), "Element2".to_string());

    // Both directions should be stored
    assert!(graph.are_related("Element1", "Element2"));
    assert!(graph.are_related("Element2", "Element1"));
}

#[test]
fn test_add_multiple_relationships_to_same_element() {
    // Test that one element can be related to multiple other elements
    let mut graph = SymmetricGraph::new();

    graph.add("A".to_string(), "B".to_string());
    graph.add("A".to_string(), "C".to_string());
    graph.add("A".to_string(), "D".to_string());

    // A should be related to B, C, and D
    let related = graph.get_related("A").unwrap();
    assert_eq!(related.len(), 3);
    assert!(related.contains(&"B".to_string()));
    assert!(related.contains(&"C".to_string()));
    assert!(related.contains(&"D".to_string()));
}

#[test]
fn test_add_duplicate_relationships() {
    // Test adding the same relationship multiple times (edge case)
    // NOTE: This documents current behavior which is likely a bug.
    // For symmetric relationships like "disjoints", duplicates don't make semantic sense.
    // The implementation should ideally use HashSet or deduplicate entries.
    let mut graph = SymmetricGraph::new();

    graph.add("X".to_string(), "Y".to_string());
    graph.add("X".to_string(), "Y".to_string());

    // Should still be related (implementation allows duplicates in the Vec)
    assert!(graph.are_related("X", "Y"));

    // Current implementation stores duplicates in the vector
    let related = graph.get_related("X").unwrap();
    assert_eq!(related.len(), 2); // Two entries of Y (documents current behavior)
}

#[test]
fn test_add_bidirectional_symmetry() {
    // Test that add(A, B) is equivalent to add(B, A) in terms of relationships
    let mut graph1 = SymmetricGraph::new();
    let mut graph2 = SymmetricGraph::new();

    graph1.add("First".to_string(), "Second".to_string());
    graph2.add("Second".to_string(), "First".to_string());

    // Both should have the same relationships
    assert!(graph1.are_related("First", "Second"));
    assert!(graph1.are_related("Second", "First"));
    assert!(graph2.are_related("First", "Second"));
    assert!(graph2.are_related("Second", "First"));
}

// Tests for SymmetricGraph::get_related() - Issue #230
#[test]
fn test_get_related_existing_element() {
    // Test getting related elements for an element that has relationships
    let mut graph = SymmetricGraph::new();

    graph.add("Type1".to_string(), "Type2".to_string());
    graph.add("Type1".to_string(), "Type3".to_string());

    let related = graph.get_related("Type1").unwrap();
    assert_eq!(related.len(), 2);
    assert!(related.contains(&"Type2".to_string()));
    assert!(related.contains(&"Type3".to_string()));
}

#[test]
fn test_get_related_nonexistent_element() {
    // Test getting related elements for an element that doesn't exist
    let graph = SymmetricGraph::new();

    assert!(graph.get_related("NonExistent").is_none());
}

#[test]
fn test_get_related_after_multiple_operations() {
    // Test that get_related works correctly after multiple add operations
    let mut graph = SymmetricGraph::new();

    graph.add("Hub".to_string(), "Spoke1".to_string());
    graph.add("Hub".to_string(), "Spoke2".to_string());
    graph.add("Hub".to_string(), "Spoke3".to_string());
    graph.add("Spoke1".to_string(), "Spoke2".to_string());

    // Hub should be connected to all spokes
    let hub_related = graph.get_related("Hub").unwrap();
    assert_eq!(hub_related.len(), 3);

    // Spoke1 should be connected to Hub and Spoke2
    let spoke1_related = graph.get_related("Spoke1").unwrap();
    assert_eq!(spoke1_related.len(), 2);
    assert!(spoke1_related.contains(&"Hub".to_string()));
    assert!(spoke1_related.contains(&"Spoke2".to_string()));
}

// Tests for SymmetricGraph::are_related() - Issues #233, #231, #229, #228, #227, #226
#[test]
fn test_are_related_existing_relationship() {
    // Test checking an existing relationship
    let mut graph = SymmetricGraph::new();

    graph.add("Alpha".to_string(), "Beta".to_string());

    assert!(graph.are_related("Alpha", "Beta"));
    assert!(graph.are_related("Beta", "Alpha")); // Symmetric
}

#[test]
fn test_are_related_nonexistent_relationship() {
    // Test checking a relationship that doesn't exist
    let mut graph = SymmetricGraph::new();

    graph.add("A".to_string(), "B".to_string());

    assert!(!graph.are_related("A", "C"));
    assert!(!graph.are_related("C", "D"));
}

#[test]
fn test_are_related_empty_graph() {
    // Test checking relationships in an empty graph
    let graph = SymmetricGraph::new();

    assert!(!graph.are_related("X", "Y"));
}

#[test]
fn test_are_related_self_relationship() {
    // Test checking if an element is related to itself
    let mut graph = SymmetricGraph::new();

    graph.add("Element".to_string(), "Element".to_string());

    // Should be related to itself if explicitly added
    assert!(graph.are_related("Element", "Element"));
    // Self-relationship is stored twice in the symmetric adjacency list
    assert_eq!(graph.get_related("Element").unwrap().len(), 2);
}

#[test]
fn test_are_related_not_transitive() {
    // Test that relationships are not transitive (A-B, B-C doesn't mean A-C)
    let mut graph = SymmetricGraph::new();

    graph.add("A".to_string(), "B".to_string());
    graph.add("B".to_string(), "C".to_string());

    // A and B are related
    assert!(graph.are_related("A", "B"));
    // B and C are related
    assert!(graph.are_related("B", "C"));
    // But A and C should NOT be related (no transitivity)
    assert!(!graph.are_related("A", "C"));
}

#[test]
fn test_are_related_with_many_relationships() {
    // Test are_related in a graph with many relationships
    let mut graph = SymmetricGraph::new();

    // Create a more complex graph
    for i in 0..5 {
        graph.add("Node0".to_string(), format!("Node{}", i + 1));
    }

    // Node0 should be related to all Node1-5
    assert!(graph.are_related("Node0", "Node1"));
    assert!(graph.are_related("Node0", "Node5"));

    // But Node1 and Node2 should not be related
    assert!(!graph.are_related("Node1", "Node2"));
}

#[test]
fn test_are_related_case_sensitive() {
    // Test that relationships are case-sensitive
    let mut graph = SymmetricGraph::new();

    graph.add("Element".to_string(), "Target".to_string());

    assert!(graph.are_related("Element", "Target"));
    assert!(!graph.are_related("element", "Target")); // Different case
    assert!(!graph.are_related("Element", "target")); // Different case
}

#[test]
fn test_are_related_with_empty_strings() {
    // Edge case: test with empty string elements
    let mut graph = SymmetricGraph::new();

    graph.add("".to_string(), "Element".to_string());

    assert!(graph.are_related("", "Element"));
    assert!(graph.are_related("Element", ""));
}

#[test]
fn test_are_related_order_independence() {
    // Test that are_related(A, B) == are_related(B, A) for all cases
    let mut graph = SymmetricGraph::new();

    graph.add("First".to_string(), "Second".to_string());

    // Should be symmetric
    assert_eq!(
        graph.are_related("First", "Second"),
        graph.are_related("Second", "First")
    );

    // Also test for non-existent relationships
    assert_eq!(
        graph.are_related("First", "Third"),
        graph.are_related("Third", "First")
    );
}
