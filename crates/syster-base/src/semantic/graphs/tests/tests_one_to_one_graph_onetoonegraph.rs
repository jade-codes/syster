#![allow(clippy::unwrap_used)]

use super::super::*;
use crate::core::{IStr, Span};
use std::rc::Rc;

fn istr(s: &str) -> IStr {
    Rc::from(s)
}

// ============================================================================
// Tests for new()
// ============================================================================

#[test]
fn test_new_creates_empty_graph() {
    // Test that new() creates an empty graph with no relationships
    let graph = OneToOneGraph::new();

    // Verify the graph has no relationships
    assert!(!graph.has_relationship("any_source"));
    assert_eq!(graph.get_sources("any_target").len(), 0);
    assert_eq!(graph.get_target("any_source"), None);
}

#[test]
fn test_new_is_default() {
    // Test that new() creates the same state as default()
    let graph_new = OneToOneGraph::new();
    let graph_default = OneToOneGraph::default();

    // Both should have no relationships
    assert!(!graph_new.has_relationship("source"));
    assert!(!graph_default.has_relationship("source"));
}

// ============================================================================
// Tests for has_relationship()
// ============================================================================

#[test]
fn test_has_relationship_returns_false_for_empty_graph() {
    // Test that has_relationship returns false when graph is empty
    let graph = OneToOneGraph::new();

    assert!(!graph.has_relationship("source1"));
    assert!(!graph.has_relationship("source2"));
    assert!(!graph.has_relationship(""));
}

#[test]
fn test_has_relationship_returns_true_after_add() {
    // Test that has_relationship returns true after adding a relationship
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source"), istr("target"), None, None);

    assert!(graph.has_relationship("source"));
}

#[test]
fn test_has_relationship_returns_false_for_nonexistent() {
    // Test that has_relationship returns false for sources that don't exist
    let mut graph = OneToOneGraph::new();

    graph.add(istr("existing_source"), istr("target"), None, None);

    assert!(graph.has_relationship("existing_source"));
    assert!(!graph.has_relationship("nonexistent_source"));
}

#[test]
fn test_has_relationship_multiple_sources() {
    // Test has_relationship with multiple sources
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source1"), istr("target"), None, None);
    graph.add(istr("source2"), istr("target"), None, None);
    graph.add(istr("source3"), istr("different_target"), None, None);

    assert!(graph.has_relationship("source1"));
    assert!(graph.has_relationship("source2"));
    assert!(graph.has_relationship("source3"));
    assert!(!graph.has_relationship("source4"));
}

#[test]
fn test_has_relationship_after_overwrite() {
    // Test that has_relationship still returns true after overwriting a relationship
    // (one-to-one means a source can only have one target)
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source"), istr("target1"), None, None);
    assert!(graph.has_relationship("source"));

    // Overwrite with new target
    graph.add(istr("source"), istr("target2"), None, None);
    assert!(graph.has_relationship("source"));

    // Verify it now points to target2, not target1
    assert!(matches!(graph.get_target("source"), Some(t) if t.as_ref() == "target2"));
}

// ============================================================================
// Tests for get_sources()
// ============================================================================

#[test]
fn test_get_sources_empty_graph() {
    // Test that get_sources returns empty vec for empty graph
    let graph = OneToOneGraph::new();

    let sources = graph.get_sources("target");
    assert_eq!(sources.len(), 0);
}

#[test]
fn test_get_sources_no_matching_target() {
    // Test that get_sources returns empty vec when target doesn't exist
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source1"), istr("target1"), None, None);
    graph.add(istr("source2"), istr("target2"), None, None);

    let sources = graph.get_sources("nonexistent_target");
    assert_eq!(sources.len(), 0);
}

#[test]
fn test_get_sources_single_source() {
    // Test get_sources returns correct source for a target
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source"), istr("target"), None, None);

    let sources = graph.get_sources("target");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].as_ref(), "source");
}

#[test]
fn test_get_sources_multiple_sources_same_target() {
    // Test that multiple sources can point to the same target
    // This is allowed in one-to-one (one source has one target, but multiple sources can share a target)
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source1"), istr("common_target"), None, None);
    graph.add(istr("source2"), istr("common_target"), None, None);
    graph.add(istr("source3"), istr("common_target"), None, None);

    let sources = graph.get_sources("common_target");
    assert_eq!(sources.len(), 3);

    // Check all sources are present (order doesn't matter)
    assert!(sources.iter().any(|s| s.as_ref() == "source1"));
    assert!(sources.iter().any(|s| s.as_ref() == "source2"));
    assert!(sources.iter().any(|s| s.as_ref() == "source3"));
}

#[test]
fn test_get_sources_different_targets() {
    // Test that get_sources only returns sources for the specified target
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source1"), istr("target1"), None, None);
    graph.add(istr("source2"), istr("target1"), None, None);
    graph.add(istr("source3"), istr("target2"), None, None);
    graph.add(istr("source4"), istr("target2"), None, None);

    let sources_target1 = graph.get_sources("target1");
    assert_eq!(sources_target1.len(), 2);
    assert!(sources_target1.iter().any(|s| s.as_ref() == "source1"));
    assert!(sources_target1.iter().any(|s| s.as_ref() == "source2"));

    let sources_target2 = graph.get_sources("target2");
    assert_eq!(sources_target2.len(), 2);
    assert!(sources_target2.iter().any(|s| s.as_ref() == "source3"));
    assert!(sources_target2.iter().any(|s| s.as_ref() == "source4"));
}

#[test]
fn test_get_sources_after_overwrite() {
    // Test that get_sources reflects changes after overwriting a relationship
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source"), istr("target1"), None, None);

    // source points to target1
    let sources1 = graph.get_sources("target1");
    assert_eq!(sources1.len(), 1);
    assert_eq!(sources1[0].as_ref(), "source");

    // Overwrite: source now points to target2
    graph.add(istr("source"), istr("target2"), None, None);

    // target1 should have no sources now
    let sources1_after = graph.get_sources("target1");
    assert_eq!(sources1_after.len(), 0);

    // target2 should have the source
    let sources2 = graph.get_sources("target2");
    assert_eq!(sources2.len(), 1);
    assert_eq!(sources2[0].as_ref(), "source");
}

#[test]
fn test_get_sources_with_locations() {
    // Test get_sources_with_locations returns sources and their spans
    let mut graph = OneToOneGraph::new();

    let span1 = Span::from_coords(1, 0, 1, 10);
    let span2 = Span::from_coords(2, 0, 2, 15);

    graph.add(
        istr("source1"),
        istr("target"),
        Some(istr("test.sysml")),
        Some(span1),
    );
    graph.add(
        istr("source2"),
        istr("target"),
        Some(istr("test.sysml")),
        Some(span2),
    );
    graph.add(istr("source3"), istr("target"), None, None);

    let sources_with_locs = graph.get_sources_with_locations("target");
    assert_eq!(sources_with_locs.len(), 3);

    // Find each source and verify its location
    let source1_entry = sources_with_locs
        .iter()
        .find(|(s, _)| s.as_ref() == "source1")
        .unwrap();
    assert!(source1_entry.1.map(|l| l.span == span1).unwrap_or(false));

    let source2_entry = sources_with_locs
        .iter()
        .find(|(s, _)| s.as_ref() == "source2")
        .unwrap();
    assert!(source2_entry.1.map(|l| l.span == span2).unwrap_or(false));

    let source3_entry = sources_with_locs
        .iter()
        .find(|(s, _)| s.as_ref() == "source3")
        .unwrap();
    assert!(source3_entry.1.is_none());
}

#[test]
fn test_get_sources_empty_string_target() {
    // Test get_sources with empty string as target
    let mut graph = OneToOneGraph::new();

    graph.add(istr("source"), istr(""), None, None);

    let sources = graph.get_sources("");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].as_ref(), "source");
}

// ============================================================================
// Integration tests combining multiple functions
// ============================================================================

#[test]
fn test_integration_typing_relationship() {
    // Test a realistic use case: tracking typing relationships
    // (e.g., "myFeature" is typed by "MyType")
    let mut graph = OneToOneGraph::new();

    let span = Span::from_coords(5, 10, 5, 20);

    graph.add(
        istr("myFeature"),
        istr("MyType"),
        Some(istr("test.sysml")),
        Some(span),
    );

    // Check relationship exists
    assert!(graph.has_relationship("myFeature"));

    // Get target from source
    assert!(matches!(graph.get_target("myFeature"), Some(t) if t.as_ref() == "MyType"));

    // Get source from target (reverse lookup)
    let sources = graph.get_sources("MyType");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].as_ref(), "myFeature");

    // Verify span is preserved
    let target_with_loc = graph.get_target_with_location("myFeature").unwrap();
    assert_eq!(target_with_loc.0.as_ref(), "MyType");
    assert!(target_with_loc.1.map(|l| l.span == span).unwrap_or(false));
}

#[test]
fn test_integration_multiple_features_same_type() {
    // Test multiple features typed by the same type
    let mut graph = OneToOneGraph::new();

    graph.add(istr("feature1"), istr("CommonType"), None, None);
    graph.add(istr("feature2"), istr("CommonType"), None, None);
    graph.add(istr("feature3"), istr("CommonType"), None, None);

    // All features should have the relationship
    assert!(graph.has_relationship("feature1"));
    assert!(graph.has_relationship("feature2"));
    assert!(graph.has_relationship("feature3"));

    // CommonType should have 3 sources
    let sources = graph.get_sources("CommonType");
    assert_eq!(sources.len(), 3);

    // Each feature should point to CommonType
    assert!(matches!(graph.get_target("feature1"), Some(t) if t.as_ref() == "CommonType"));
    assert!(matches!(graph.get_target("feature2"), Some(t) if t.as_ref() == "CommonType"));
    assert!(matches!(graph.get_target("feature3"), Some(t) if t.as_ref() == "CommonType"));
}

#[test]
fn test_integration_qualified_names() {
    // Test with qualified names (package::class::feature)
    let mut graph = OneToOneGraph::new();

    graph.add(
        istr("Package::Class::feature"),
        istr("Package::Types::MyType"),
        None,
        None,
    );
    graph.add(
        istr("OtherPackage::Class::feature"),
        istr("Package::Types::MyType"),
        None,
        None,
    );

    assert!(graph.has_relationship("Package::Class::feature"));
    assert!(graph.has_relationship("OtherPackage::Class::feature"));

    let sources = graph.get_sources("Package::Types::MyType");
    assert_eq!(sources.len(), 2);
    assert!(
        sources
            .iter()
            .any(|s| s.as_ref() == "Package::Class::feature")
    );
    assert!(
        sources
            .iter()
            .any(|s| s.as_ref() == "OtherPackage::Class::feature")
    );
}
