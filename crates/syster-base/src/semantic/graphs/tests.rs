#![allow(clippy::unwrap_used)]

use super::*;
use std::path::PathBuf;

#[test]
fn test_generic_one_to_many() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );
    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Asset".to_string(),
        None,
    );

    let targets = graph.get_one_to_many("specialization", "Car").unwrap();
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&&"Vehicle".to_string()));
    assert!(targets.contains(&&"Asset".to_string()));
}

#[test]
fn test_generic_one_to_one() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_one(
        "typing",
        "myFeature".to_string(),
        "MyType".to_string(),
        None,
    );

    let target = graph.get_one_to_one("typing", "myFeature");
    assert_eq!(target, Some(&"MyType".to_string()));
}

#[test]
fn test_generic_symmetric() {
    let mut graph = RelationshipGraph::new();

    graph.add_symmetric("disjoints", "Type1".to_string(), "Type2".to_string());

    let related = graph.get_symmetric("disjoints", "Type1").unwrap();
    assert_eq!(related.len(), 1);
    assert_eq!(related[0], "Type2");
}

#[test]
fn test_multiple_relationship_types() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );
    graph.add_one_to_many(
        "redefinition",
        "Car::wheel".to_string(),
        "Vehicle::wheel".to_string(),
        None,
    );
    graph.add_one_to_one("typing", "myCar".to_string(), "Car".to_string(), None);

    let types = graph.relationship_types();
    assert_eq!(types.len(), 3);
    assert!(types.contains(&"specialization".to_string()));
    assert!(types.contains(&"redefinition".to_string()));
    assert!(types.contains(&"typing".to_string()));
}

#[test]
fn test_transitive_path() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );
    graph.add_one_to_many(
        "specialization",
        "Vehicle".to_string(),
        "Thing".to_string(),
        None,
    );

    assert!(graph.has_transitive_path("specialization", "Car", "Thing"));
    assert!(!graph.has_transitive_path("specialization", "Thing", "Car"));
}

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

#[test]
fn test_get_all_relationships() {
    // Test that get_all_relationships automatically discovers all relationship types
    let mut graph = RelationshipGraph::new();

    // Add various relationship types
    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );
    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Asset".to_string(),
        None,
    );
    graph.add_one_to_many(
        "subsetting",
        "Car".to_string(),
        "Equipment".to_string(),
        None,
    );
    graph.add_one_to_one("typing", "Car".to_string(), "CarType".to_string(), None);
    graph.add_symmetric("disjoints", "Car".to_string(), "Truck".to_string());

    // Get all relationships for "Car"
    let all_rels = graph.get_all_relationships("Car");

    // Should find 4 different relationship types
    assert_eq!(all_rels.len(), 4);

    // Check specialization (one-to-many with 2 targets)
    let spec = all_rels
        .iter()
        .find(|(rel_type, _)| rel_type == "specialization")
        .unwrap();
    assert_eq!(spec.1.len(), 2);
    assert!(spec.1.contains(&"Vehicle".to_string()));
    assert!(spec.1.contains(&"Asset".to_string()));

    // Check subsetting (one-to-many with 1 target)
    let subset = all_rels
        .iter()
        .find(|(rel_type, _)| rel_type == "subsetting")
        .unwrap();
    assert_eq!(subset.1.len(), 1);
    assert_eq!(subset.1[0], "Equipment");

    // Check typing (one-to-one)
    let typing = all_rels
        .iter()
        .find(|(rel_type, _)| rel_type == "typing")
        .unwrap();
    assert_eq!(typing.1.len(), 1);
    assert_eq!(typing.1[0], "CarType");

    // Check symmetric
    let disjoints = all_rels
        .iter()
        .find(|(rel_type, _)| rel_type == "disjoints")
        .unwrap();
    assert_eq!(disjoints.1.len(), 1);
    assert_eq!(disjoints.1[0], "Truck");
}

#[test]
fn test_get_formatted_relationships() {
    // Test that formatted relationships are human-readable
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );
    graph.add_one_to_many(
        "redefinition",
        "Car".to_string(),
        "BaseCar".to_string(),
        None,
    );
    graph.add_one_to_many(
        "subsetting",
        "Car".to_string(),
        "Equipment".to_string(),
        None,
    );
    graph.add_one_to_one("typing", "Car".to_string(), "CarType".to_string(), None);

    let formatted = graph.get_formatted_relationships("Car");

    // Should have 4 formatted strings (one per relationship)
    assert_eq!(formatted.len(), 4);

    // Check formatting - should be human-readable with proper labels
    assert!(formatted.iter().any(|s| s == "Specializes `Vehicle`"));
    assert!(formatted.iter().any(|s| s == "Redefines `BaseCar`"));
    assert!(formatted.iter().any(|s| s == "Subsets `Equipment`"));
    assert!(formatted.iter().any(|s| s == "Typed by `CarType`"));
}

#[test]
fn test_get_all_relationships_empty() {
    // Test with an element that has no relationships
    let graph = RelationshipGraph::new();
    let all_rels = graph.get_all_relationships("NonExistent");
    assert_eq!(all_rels.len(), 0);

    let formatted = graph.get_formatted_relationships("NonExistent");
    assert_eq!(formatted.len(), 0);
}
