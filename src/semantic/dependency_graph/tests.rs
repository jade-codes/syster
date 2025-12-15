#![allow(clippy::unwrap_used)]

use super::*;
use std::path::PathBuf;

#[test]
fn test_empty_graph() {
    // TDD: New graph should have no dependencies
    let graph = DependencyGraph::new();

    let file = PathBuf::from("test.sysml");
    assert_eq!(graph.get_dependencies(&file).len(), 0);
    assert_eq!(graph.get_dependents(&file).len(), 0);
}

#[test]
fn test_add_single_dependency() {
    // TDD: A imports B creates dependency A->B
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");

    graph.add_dependency(&file_a, &file_b);

    // A depends on B
    let deps = graph.get_dependencies(&file_a);
    assert_eq!(deps.len(), 1);
    assert!(deps.contains(&file_b));

    // B is depended upon by A
    let dependents = graph.get_dependents(&file_b);
    assert_eq!(dependents.len(), 1);
    assert!(dependents.contains(&file_a));
}

#[test]
fn test_multiple_dependencies() {
    // TDD: A imports B and C creates A->B, A->C
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");
    let file_c = PathBuf::from("c.sysml");

    graph.add_dependency(&file_a, &file_b);
    graph.add_dependency(&file_a, &file_c);

    let deps = graph.get_dependencies(&file_a);
    assert_eq!(deps.len(), 2);
    assert!(deps.contains(&file_b));
    assert!(deps.contains(&file_c));
}

#[test]
fn test_transitive_dependencies() {
    // TDD: A->B->C creates transitive relationship
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");
    let file_c = PathBuf::from("c.sysml");

    graph.add_dependency(&file_a, &file_b);
    graph.add_dependency(&file_b, &file_c);

    // B depends on C
    let b_deps = graph.get_dependencies(&file_b);
    assert_eq!(b_deps.len(), 1);
    assert!(b_deps.contains(&file_c));

    // C is depended upon by B
    let c_dependents = graph.get_dependents(&file_c);
    assert_eq!(c_dependents.len(), 1);
    assert!(c_dependents.contains(&file_b));
}

#[test]
fn test_detect_circular_dependency_simple() {
    // TDD: A->B->A should be detected as circular
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");

    graph.add_dependency(&file_a, &file_b);
    graph.add_dependency(&file_b, &file_a);

    assert!(graph.has_circular_dependency(&file_a));
    assert!(graph.has_circular_dependency(&file_b));
}

#[test]
fn test_detect_circular_dependency_complex() {
    // TDD: A->B->C->A should be detected
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");
    let file_c = PathBuf::from("c.sysml");

    graph.add_dependency(&file_a, &file_b);
    graph.add_dependency(&file_b, &file_c);
    graph.add_dependency(&file_c, &file_a);

    assert!(graph.has_circular_dependency(&file_a));
    assert!(graph.has_circular_dependency(&file_b));
    assert!(graph.has_circular_dependency(&file_c));
}

#[test]
fn test_no_circular_dependency() {
    // TDD: Linear chain A->B->C should not be circular
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");
    let file_c = PathBuf::from("c.sysml");

    graph.add_dependency(&file_a, &file_b);
    graph.add_dependency(&file_b, &file_c);

    assert!(!graph.has_circular_dependency(&file_a));
    assert!(!graph.has_circular_dependency(&file_b));
    assert!(!graph.has_circular_dependency(&file_c));
}

#[test]
fn test_get_all_affected_files() {
    // TDD: If C changes and B->C, A->B, then A and B are affected
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");
    let file_c = PathBuf::from("c.sysml");

    graph.add_dependency(&file_a, &file_b);
    graph.add_dependency(&file_b, &file_c);

    let affected = graph.get_all_affected(&file_c);

    // When C changes, B and A are affected
    assert_eq!(affected.len(), 2);
    assert!(affected.contains(&file_b));
    assert!(affected.contains(&file_a));
}

#[test]
fn test_remove_dependency() {
    // TDD: Should be able to remove dependencies when files are deleted
    let mut graph = DependencyGraph::new();
    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");

    graph.add_dependency(&file_a, &file_b);
    assert_eq!(graph.get_dependencies(&file_a).len(), 1);

    graph.remove_file(&file_a);
    assert_eq!(graph.get_dependencies(&file_a).len(), 0);
    assert_eq!(graph.get_dependents(&file_b).len(), 0);
}

#[test]
fn test_dependencies_count() {
    // TDD: Count total number of dependencies in the graph
    let mut graph = DependencyGraph::new();
    assert_eq!(graph.dependencies_count(), 0);

    let file_a = PathBuf::from("a.sysml");
    let file_b = PathBuf::from("b.sysml");
    let file_c = PathBuf::from("c.sysml");

    // A -> B
    graph.add_dependency(&file_a, &file_b);
    assert_eq!(graph.dependencies_count(), 1);

    // A -> C (A now has 2 dependencies)
    graph.add_dependency(&file_a, &file_c);
    assert_eq!(graph.dependencies_count(), 2);

    // B -> C (total 3 dependencies)
    graph.add_dependency(&file_b, &file_c);
    assert_eq!(graph.dependencies_count(), 3);

    // Remove A (removes A->B and A->C, leaves B->C)
    graph.remove_file(&file_a);
    assert_eq!(graph.dependencies_count(), 1);
}
