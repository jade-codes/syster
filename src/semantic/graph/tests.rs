#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn test_generic_one_to_many() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many("specialization", "Car".to_string(), "Vehicle".to_string());
    graph.add_one_to_many("specialization", "Car".to_string(), "Asset".to_string());

    let targets = graph.get_one_to_many("specialization", "Car").unwrap();
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&"Vehicle".to_string()));
    assert!(targets.contains(&"Asset".to_string()));
}

#[test]
fn test_generic_one_to_one() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_one("typing", "myFeature".to_string(), "MyType".to_string());

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

    graph.add_one_to_many("specialization", "Car".to_string(), "Vehicle".to_string());
    graph.add_one_to_many(
        "redefinition",
        "Car::wheel".to_string(),
        "Vehicle::wheel".to_string(),
    );
    graph.add_one_to_one("typing", "myCar".to_string(), "Car".to_string());

    let types = graph.relationship_types();
    assert_eq!(types.len(), 3);
    assert!(types.contains(&"specialization".to_string()));
    assert!(types.contains(&"redefinition".to_string()));
    assert!(types.contains(&"typing".to_string()));
}

#[test]
fn test_transitive_path() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many("specialization", "Car".to_string(), "Vehicle".to_string());
    graph.add_one_to_many("specialization", "Vehicle".to_string(), "Thing".to_string());

    assert!(graph.has_transitive_path("specialization", "Car", "Thing"));
    assert!(!graph.has_transitive_path("specialization", "Thing", "Car"));
}
