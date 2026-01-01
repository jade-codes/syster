#![allow(clippy::unwrap_used)]

use super::super::OneToManyGraph;
use crate::core::Span;

// ============================================================================
// Tests for OneToManyGraph::new() (Issue #223)
// ============================================================================

#[test]
fn test_new_creates_empty_graph() {
    let graph = OneToManyGraph::new();

    // New graph should have no relationships
    assert_eq!(graph.get_targets("anything"), None);
    assert_eq!(graph.get_sources("anything").len(), 0);
}

#[test]
fn test_new_default_equivalent() {
    let graph1 = OneToManyGraph::new();
    let graph2 = OneToManyGraph::default();

    // Both should behave identically
    assert_eq!(graph1.get_targets("test"), graph2.get_targets("test"));
}

// ============================================================================
// Tests for add() and get_targets()
// ============================================================================

#[test]
fn test_add_single_relationship() {
    let mut graph = OneToManyGraph::new();

    graph.add("Car".to_string(), "Vehicle".to_string(), None);

    let targets = graph.get_targets("Car").unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0], "Vehicle");
}

#[test]
fn test_add_multiple_targets_same_source() {
    let mut graph = OneToManyGraph::new();

    graph.add("Car".to_string(), "Vehicle".to_string(), None);
    graph.add("Car".to_string(), "Asset".to_string(), None);

    let targets = graph.get_targets("Car").unwrap();
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&&"Vehicle".to_string()));
    assert!(targets.contains(&&"Asset".to_string()));
}

#[test]
fn test_add_with_span() {
    let mut graph = OneToManyGraph::new();
    let span = Span::from_coords(0, 0, 0, 10);

    graph.add("Car".to_string(), "Vehicle".to_string(), Some(span));

    let targets = graph.get_targets_with_spans("Car").unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].0, "Vehicle");
    assert!(targets[0].1.is_some());
}

#[test]
fn test_get_targets_nonexistent_source() {
    let graph = OneToManyGraph::new();

    assert_eq!(graph.get_targets("NonExistent"), None);
}

#[test]
fn test_get_targets_with_spans_empty() {
    let graph = OneToManyGraph::new();

    assert_eq!(graph.get_targets_with_spans("NonExistent"), None);
}

// ============================================================================
// Tests for get_sources() (Issues #220, #216, #214)
// ============================================================================

#[test]
fn test_get_sources_single_source() {
    let mut graph = OneToManyGraph::new();

    graph.add("Car".to_string(), "Vehicle".to_string(), None);

    let sources = graph.get_sources("Vehicle");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0], "Car");
}

#[test]
fn test_get_sources_multiple_sources() {
    let mut graph = OneToManyGraph::new();

    graph.add("Car".to_string(), "Vehicle".to_string(), None);
    graph.add("Truck".to_string(), "Vehicle".to_string(), None);
    graph.add("Motorcycle".to_string(), "Vehicle".to_string(), None);

    let sources = graph.get_sources("Vehicle");
    assert_eq!(sources.len(), 3);
    assert!(sources.contains(&&"Car".to_string()));
    assert!(sources.contains(&&"Truck".to_string()));
    assert!(sources.contains(&&"Motorcycle".to_string()));
}

#[test]
fn test_get_sources_no_sources() {
    let mut graph = OneToManyGraph::new();

    graph.add("Car".to_string(), "Vehicle".to_string(), None);

    let sources = graph.get_sources("NonExistent");
    assert_eq!(sources.len(), 0);
}

#[test]
fn test_get_sources_mixed_targets() {
    let mut graph = OneToManyGraph::new();

    graph.add("Car".to_string(), "Vehicle".to_string(), None);
    graph.add("Car".to_string(), "Asset".to_string(), None);
    graph.add("Truck".to_string(), "Vehicle".to_string(), None);

    // Vehicle has two sources
    let vehicle_sources = graph.get_sources("Vehicle");
    assert_eq!(vehicle_sources.len(), 2);
    assert!(vehicle_sources.contains(&&"Car".to_string()));
    assert!(vehicle_sources.contains(&&"Truck".to_string()));

    // Asset has one source
    let asset_sources = graph.get_sources("Asset");
    assert_eq!(asset_sources.len(), 1);
    assert_eq!(asset_sources[0], "Car");
}

#[test]
fn test_get_sources_with_spans() {
    let mut graph = OneToManyGraph::new();
    let span1 = Span::from_coords(0, 0, 0, 10);
    let span2 = Span::from_coords(1, 0, 1, 10);

    graph.add("Car".to_string(), "Vehicle".to_string(), Some(span1));
    graph.add("Truck".to_string(), "Vehicle".to_string(), Some(span2));

    let sources = graph.get_sources_with_spans("Vehicle");
    assert_eq!(sources.len(), 2);

    // Both sources should have spans
    assert!(sources.iter().all(|(_, span)| span.is_some()));
}

#[test]
fn test_get_sources_with_spans_no_span() {
    let mut graph = OneToManyGraph::new();

    graph.add("Car".to_string(), "Vehicle".to_string(), None);

    let sources = graph.get_sources_with_spans("Vehicle");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].0, "Car");
    assert!(sources[0].1.is_none());
}

// ============================================================================
// Tests for has_path()
// ============================================================================

#[test]
fn test_has_path_same_node() {
    let graph = OneToManyGraph::new();

    // Same node should always return true
    assert!(graph.has_path("Node", "Node"));
}

#[test]
fn test_has_path_direct() {
    let mut graph = OneToManyGraph::new();

    graph.add("A".to_string(), "B".to_string(), None);

    assert!(graph.has_path("A", "B"));
    assert!(!graph.has_path("B", "A")); // Not bidirectional
}

#[test]
fn test_has_path_transitive() {
    let mut graph = OneToManyGraph::new();

    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);
    graph.add("C".to_string(), "D".to_string(), None);

    assert!(graph.has_path("A", "D"));
    assert!(graph.has_path("A", "B"));
    assert!(graph.has_path("A", "C"));
    assert!(graph.has_path("B", "D"));
    assert!(!graph.has_path("D", "A")); // No reverse path
}

#[test]
fn test_has_path_multiple_paths() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> D
    // A -> C -> D
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("A".to_string(), "C".to_string(), None);
    graph.add("B".to_string(), "D".to_string(), None);
    graph.add("C".to_string(), "D".to_string(), None);

    assert!(graph.has_path("A", "D"));
}

#[test]
fn test_has_path_no_path() {
    let mut graph = OneToManyGraph::new();

    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("C".to_string(), "D".to_string(), None);

    assert!(!graph.has_path("A", "D"));
    assert!(!graph.has_path("B", "C"));
}

#[test]
fn test_has_path_with_cycle() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> C -> A (cycle)
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);
    graph.add("C".to_string(), "A".to_string(), None);

    // Should handle cycles without infinite loop
    assert!(graph.has_path("A", "B"));
    assert!(graph.has_path("A", "C"));
    assert!(graph.has_path("B", "A"));
}

// ============================================================================
// Tests for find_cycles() and dfs_cycles() (Issues #219, #218, #217, #215)
// ============================================================================

#[test]
fn test_find_cycles_no_cycles() {
    let mut graph = OneToManyGraph::new();

    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);

    let cycles = graph.find_cycles();
    assert_eq!(cycles.len(), 0);
}

#[test]
fn test_find_cycles_simple_cycle() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> A
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "A".to_string(), None);

    let cycles = graph.find_cycles();
    assert!(!cycles.is_empty());

    // Should find the cycle
    let has_ab_cycle = cycles.iter().any(|cycle| {
        cycle.len() == 2
            && ((cycle[0] == "A" && cycle[1] == "B") || (cycle[0] == "B" && cycle[1] == "A"))
    });
    assert!(has_ab_cycle);
}

#[test]
fn test_find_cycles_self_loop() {
    let mut graph = OneToManyGraph::new();

    // A -> A (self loop)
    graph.add("A".to_string(), "A".to_string(), None);

    let cycles = graph.find_cycles();
    assert!(!cycles.is_empty());
}

#[test]
fn test_find_cycles_complex() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> C -> A (3-node cycle)
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);
    graph.add("C".to_string(), "A".to_string(), None);

    let cycles = graph.find_cycles();
    assert!(!cycles.is_empty());

    // Should find a 3-node cycle
    let has_3node_cycle = cycles.iter().any(|cycle| cycle.len() == 3);
    assert!(has_3node_cycle);
}

#[test]
fn test_find_cycles_multiple_cycles() {
    let mut graph = OneToManyGraph::new();

    // Cycle 1: A -> B -> A
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "A".to_string(), None);

    // Cycle 2: C -> D -> C
    graph.add("C".to_string(), "D".to_string(), None);
    graph.add("D".to_string(), "C".to_string(), None);

    let cycles = graph.find_cycles();
    assert!(cycles.len() >= 2);
}

#[test]
fn test_find_cycles_empty_graph() {
    let graph = OneToManyGraph::new();

    let cycles = graph.find_cycles();
    assert_eq!(cycles.len(), 0);
}

#[test]
fn test_find_cycles_with_branches() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> C -> D (no cycle)
    // B -> E -> B (cycle)
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);
    graph.add("C".to_string(), "D".to_string(), None);
    graph.add("B".to_string(), "E".to_string(), None);
    graph.add("E".to_string(), "B".to_string(), None);

    let cycles = graph.find_cycles();
    assert!(!cycles.is_empty());

    // Should find the B-E cycle
    let has_be_cycle = cycles
        .iter()
        .any(|cycle| cycle.contains(&"B".to_string()) && cycle.contains(&"E".to_string()));
    assert!(has_be_cycle);
}

// ============================================================================
// Tests for has_circular_dependency() and dfs_circular()
// (Issues #222, #221)
// ============================================================================

#[test]
fn test_has_circular_dependency_no_cycle() {
    let mut graph = OneToManyGraph::new();

    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);

    assert!(!graph.has_circular_dependency("A"));
    assert!(!graph.has_circular_dependency("B"));
    assert!(!graph.has_circular_dependency("C"));
}

#[test]
fn test_has_circular_dependency_direct_cycle() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> A
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "A".to_string(), None);

    assert!(graph.has_circular_dependency("A"));
    assert!(graph.has_circular_dependency("B"));
}

#[test]
fn test_has_circular_dependency_self_loop() {
    let mut graph = OneToManyGraph::new();

    // A -> A
    graph.add("A".to_string(), "A".to_string(), None);

    assert!(graph.has_circular_dependency("A"));
}

#[test]
fn test_has_circular_dependency_indirect_cycle() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> C -> A
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);
    graph.add("C".to_string(), "A".to_string(), None);

    assert!(graph.has_circular_dependency("A"));
    assert!(graph.has_circular_dependency("B"));
    assert!(graph.has_circular_dependency("C"));
}

#[test]
fn test_has_circular_dependency_partial_cycle() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> C -> B (B and C in cycle, A not)
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);
    graph.add("C".to_string(), "B".to_string(), None);

    assert!(!graph.has_circular_dependency("A"));
    assert!(graph.has_circular_dependency("B"));
    assert!(graph.has_circular_dependency("C"));
}

#[test]
fn test_has_circular_dependency_isolated_node() {
    let mut graph = OneToManyGraph::new();

    graph.add("A".to_string(), "B".to_string(), None);

    // Node with no outgoing edges cannot have circular dependency
    assert!(!graph.has_circular_dependency("B"));
}

#[test]
fn test_has_circular_dependency_nonexistent_node() {
    let graph = OneToManyGraph::new();

    // Nonexistent node has no circular dependency
    assert!(!graph.has_circular_dependency("NonExistent"));
}

#[test]
fn test_has_circular_dependency_complex_graph() {
    let mut graph = OneToManyGraph::new();

    // A -> B -> D
    // B -> C -> E
    // E -> B (creates cycle: B -> C -> E -> B)
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "D".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);
    graph.add("C".to_string(), "E".to_string(), None);
    graph.add("E".to_string(), "B".to_string(), None);

    assert!(!graph.has_circular_dependency("A"));
    assert!(!graph.has_circular_dependency("D"));
    assert!(graph.has_circular_dependency("B"));
    assert!(graph.has_circular_dependency("C"));
    assert!(graph.has_circular_dependency("E"));
}

// ============================================================================
// Edge cases and boundary conditions
// ============================================================================

#[test]
fn test_empty_string_keys() {
    let mut graph = OneToManyGraph::new();

    graph.add("".to_string(), "target".to_string(), None);
    graph.add("source".to_string(), "".to_string(), None);

    let targets = graph.get_targets("");
    assert!(targets.is_some());

    let sources = graph.get_sources("");
    assert_eq!(sources.len(), 1);
}

#[test]
fn test_duplicate_relationships() {
    let mut graph = OneToManyGraph::new();

    // Add same relationship twice
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("A".to_string(), "B".to_string(), None);

    // Should have 1 entry - duplicates are deduplicated by (source, target) pair
    let targets = graph.get_targets("A").unwrap();
    assert_eq!(targets.len(), 1);
}

#[test]
fn test_different_targets_not_deduplicated() {
    let mut graph = OneToManyGraph::new();

    // Add different relationships from same source
    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("A".to_string(), "C".to_string(), None);

    // Should have 2 entries - different targets are not deduplicated
    let targets = graph.get_targets("A").unwrap();
    assert_eq!(targets.len(), 2);
}

#[test]
fn test_clone_graph() {
    let mut graph = OneToManyGraph::new();

    graph.add("A".to_string(), "B".to_string(), None);
    graph.add("B".to_string(), "C".to_string(), None);

    let cloned = graph.clone();

    // Cloned graph should have same relationships
    assert_eq!(
        graph.get_targets("A").unwrap(),
        cloned.get_targets("A").unwrap()
    );
    assert_eq!(
        graph.get_targets("B").unwrap(),
        cloned.get_targets("B").unwrap()
    );
}

#[test]
fn test_large_graph_performance() {
    let mut graph = OneToManyGraph::new();

    // Create a chain: 0 -> 1 -> 2 -> ... -> 99
    for i in 0..100 {
        graph.add(i.to_string(), (i + 1).to_string(), None);
    }

    // Should efficiently find path from start to end
    assert!(graph.has_path("0", "100"));
    assert!(!graph.has_path("100", "0"));

    // Should efficiently detect no circular dependency
    assert!(!graph.has_circular_dependency("0"));
    assert!(!graph.has_circular_dependency("50"));
}
