#![allow(clippy::unwrap_used)]

use crate::core::traits::AstNode;

// ============================================================================
// Tests for AstNode::has_children (#360)
// ============================================================================

// Test struct with no children
#[derive(Debug, Clone)]
struct SimpleNode {
    #[allow(dead_code)]
    value: String,
}

impl AstNode for SimpleNode {
    fn node_type(&self) -> &'static str {
        "SimpleNode"
    }
    // Uses default implementation of has_children (returns false)
}

// Test struct with children
#[derive(Debug, Clone)]
struct ParentNode {
    children: Vec<String>,
}

impl AstNode for ParentNode {
    fn node_type(&self) -> &'static str {
        "ParentNode"
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

#[test]
fn test_has_children_default_returns_false() {
    let node = SimpleNode {
        value: "test".to_string(),
    };
    assert!(!node.has_children());
}

#[test]
fn test_has_children_with_empty_children() {
    let node = ParentNode { children: vec![] };
    assert!(!node.has_children());
}

#[test]
fn test_has_children_with_children() {
    let node = ParentNode {
        children: vec!["child1".to_string()],
    };
    assert!(node.has_children());
}

#[test]
fn test_has_children_with_multiple_children() {
    let node = ParentNode {
        children: vec![
            "child1".to_string(),
            "child2".to_string(),
            "child3".to_string(),
        ],
    };
    assert!(node.has_children());
}
