#![allow(clippy::unwrap_used)]

use super::*;
use crate::core::Span;

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

    graph.add("source".to_string(), "target".to_string(), None);

    assert!(graph.has_relationship("source"));
}

#[test]
fn test_has_relationship_returns_false_for_nonexistent() {
    // Test that has_relationship returns false for sources that don't exist
    let mut graph = OneToOneGraph::new();

    graph.add("existing_source".to_string(), "target".to_string(), None);

    assert!(graph.has_relationship("existing_source"));
    assert!(!graph.has_relationship("nonexistent_source"));
}

#[test]
fn test_has_relationship_multiple_sources() {
    // Test has_relationship with multiple sources
    let mut graph = OneToOneGraph::new();

    graph.add("source1".to_string(), "target".to_string(), None);
    graph.add("source2".to_string(), "target".to_string(), None);
    graph.add("source3".to_string(), "different_target".to_string(), None);

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

    graph.add("source".to_string(), "target1".to_string(), None);
    assert!(graph.has_relationship("source"));

    // Overwrite with new target
    graph.add("source".to_string(), "target2".to_string(), None);
    assert!(graph.has_relationship("source"));

    // Verify it now points to target2, not target1
    assert_eq!(graph.get_target("source"), Some(&"target2".to_string()));
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

    graph.add("source1".to_string(), "target1".to_string(), None);
    graph.add("source2".to_string(), "target2".to_string(), None);

    let sources = graph.get_sources("nonexistent_target");
    assert_eq!(sources.len(), 0);
}

#[test]
fn test_get_sources_single_source() {
    // Test get_sources returns correct source for a target
    let mut graph = OneToOneGraph::new();

    graph.add("source".to_string(), "target".to_string(), None);

    let sources = graph.get_sources("target");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0], "source");
}

#[test]
fn test_get_sources_multiple_sources_same_target() {
    // Test that multiple sources can point to the same target
    // This is allowed in one-to-one (one source has one target, but multiple sources can share a target)
    let mut graph = OneToOneGraph::new();

    graph.add("source1".to_string(), "common_target".to_string(), None);
    graph.add("source2".to_string(), "common_target".to_string(), None);
    graph.add("source3".to_string(), "common_target".to_string(), None);

    let sources = graph.get_sources("common_target");
    assert_eq!(sources.len(), 3);

    // Check all sources are present (order doesn't matter)
    assert!(sources.contains(&&"source1".to_string()));
    assert!(sources.contains(&&"source2".to_string()));
    assert!(sources.contains(&&"source3".to_string()));
}

#[test]
fn test_get_sources_different_targets() {
    // Test that get_sources only returns sources for the specified target
    let mut graph = OneToOneGraph::new();

    graph.add("source1".to_string(), "target1".to_string(), None);
    graph.add("source2".to_string(), "target1".to_string(), None);
    graph.add("source3".to_string(), "target2".to_string(), None);
    graph.add("source4".to_string(), "target2".to_string(), None);

    let sources_target1 = graph.get_sources("target1");
    assert_eq!(sources_target1.len(), 2);
    assert!(sources_target1.contains(&&"source1".to_string()));
    assert!(sources_target1.contains(&&"source2".to_string()));

    let sources_target2 = graph.get_sources("target2");
    assert_eq!(sources_target2.len(), 2);
    assert!(sources_target2.contains(&&"source3".to_string()));
    assert!(sources_target2.contains(&&"source4".to_string()));
}

#[test]
fn test_get_sources_after_overwrite() {
    // Test that get_sources reflects changes after overwriting a relationship
    let mut graph = OneToOneGraph::new();

    graph.add("source".to_string(), "target1".to_string(), None);

    // source points to target1
    let sources1 = graph.get_sources("target1");
    assert_eq!(sources1.len(), 1);
    assert_eq!(sources1[0], "source");

    // Overwrite: source now points to target2
    graph.add("source".to_string(), "target2".to_string(), None);

    // target1 should have no sources now
    let sources1_after = graph.get_sources("target1");
    assert_eq!(sources1_after.len(), 0);

    // target2 should have the source
    let sources2 = graph.get_sources("target2");
    assert_eq!(sources2.len(), 1);
    assert_eq!(sources2[0], "source");
}

#[test]
fn test_get_sources_with_spans() {
    // Test get_sources_with_spans returns sources and their spans
    let mut graph = OneToOneGraph::new();

    let span1 = Span::from_coords(1, 0, 1, 10);
    let span2 = Span::from_coords(2, 0, 2, 15);

    graph.add("source1".to_string(), "target".to_string(), Some(span1));
    graph.add("source2".to_string(), "target".to_string(), Some(span2));
    graph.add("source3".to_string(), "target".to_string(), None);

    let sources_with_spans = graph.get_sources_with_spans("target");
    assert_eq!(sources_with_spans.len(), 3);

    // Find each source and verify its span
    let source1_entry = sources_with_spans
        .iter()
        .find(|(s, _)| *s == &"source1".to_string())
        .unwrap();
    assert_eq!(source1_entry.1, Some(&span1));

    let source2_entry = sources_with_spans
        .iter()
        .find(|(s, _)| *s == &"source2".to_string())
        .unwrap();
    assert_eq!(source2_entry.1, Some(&span2));

    let source3_entry = sources_with_spans
        .iter()
        .find(|(s, _)| *s == &"source3".to_string())
        .unwrap();
    assert_eq!(source3_entry.1, None);
}

#[test]
fn test_get_sources_empty_string_target() {
    // Test get_sources with empty string as target
    let mut graph = OneToOneGraph::new();

    graph.add("source".to_string(), "".to_string(), None);

    let sources = graph.get_sources("");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0], "source");
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

    graph.add("myFeature".to_string(), "MyType".to_string(), Some(span));

    // Check relationship exists
    assert!(graph.has_relationship("myFeature"));

    // Get target from source
    assert_eq!(graph.get_target("myFeature"), Some(&"MyType".to_string()));

    // Get source from target (reverse lookup)
    let sources = graph.get_sources("MyType");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0], "myFeature");

    // Verify span is preserved
    let target_with_span = graph.get_target_with_span("myFeature").unwrap();
    assert_eq!(target_with_span.0, &"MyType".to_string());
    assert_eq!(target_with_span.1, Some(&span));
}

#[test]
fn test_integration_multiple_features_same_type() {
    // Test multiple features typed by the same type
    let mut graph = OneToOneGraph::new();

    graph.add("feature1".to_string(), "CommonType".to_string(), None);
    graph.add("feature2".to_string(), "CommonType".to_string(), None);
    graph.add("feature3".to_string(), "CommonType".to_string(), None);

    // All features should have the relationship
    assert!(graph.has_relationship("feature1"));
    assert!(graph.has_relationship("feature2"));
    assert!(graph.has_relationship("feature3"));

    // CommonType should have 3 sources
    let sources = graph.get_sources("CommonType");
    assert_eq!(sources.len(), 3);

    // Each feature should point to CommonType
    assert_eq!(
        graph.get_target("feature1"),
        Some(&"CommonType".to_string())
    );
    assert_eq!(
        graph.get_target("feature2"),
        Some(&"CommonType".to_string())
    );
    assert_eq!(
        graph.get_target("feature3"),
        Some(&"CommonType".to_string())
    );
}

#[test]
fn test_integration_qualified_names() {
    // Test with qualified names (package::class::feature)
    let mut graph = OneToOneGraph::new();

    graph.add(
        "Package::Class::feature".to_string(),
        "Package::Types::MyType".to_string(),
        None,
    );
    graph.add(
        "OtherPackage::Class::feature".to_string(),
        "Package::Types::MyType".to_string(),
        None,
    );

    assert!(graph.has_relationship("Package::Class::feature"));
    assert!(graph.has_relationship("OtherPackage::Class::feature"));

    let sources = graph.get_sources("Package::Types::MyType");
    assert_eq!(sources.len(), 2);
    assert!(sources.contains(&&"Package::Class::feature".to_string()));
    assert!(sources.contains(&&"OtherPackage::Class::feature".to_string()));
}
