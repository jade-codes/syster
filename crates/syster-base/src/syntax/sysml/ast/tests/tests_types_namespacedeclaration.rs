#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use crate::core::Span;

// ============================================================================
// NamespaceDeclaration struct tests
// ============================================================================

#[test]
fn test_namespacedeclaration_creation() {
    let ns = NamespaceDeclaration {
        name: "TestNamespace".to_string(),
        span: None,
    };

    assert_eq!(ns.name, "TestNamespace");
    assert_eq!(ns.span, None);
}

#[test]
fn test_namespacedeclaration_with_span() {
    let span = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 13,
        },
    };

    let ns = NamespaceDeclaration {
        name: "MyNamespace".to_string(),
        span: Some(span),
    };

    assert_eq!(ns.name, "MyNamespace");
    assert_eq!(ns.span, Some(span));
}

#[test]
fn test_namespacedeclaration_empty_name() {
    let ns = NamespaceDeclaration {
        name: String::new(),
        span: None,
    };

    assert_eq!(ns.name, "");
    assert!(ns.name.is_empty());
}

#[test]
fn test_namespacedeclaration_clone() {
    let ns1 = NamespaceDeclaration {
        name: "Original".to_string(),
        span: None,
    };

    let ns2 = ns1.clone();

    assert_eq!(ns1.name, ns2.name);
    assert_eq!(ns1.span, ns2.span);
}

#[test]
fn test_namespacedeclaration_partial_eq() {
    let ns1 = NamespaceDeclaration {
        name: "SameName".to_string(),
        span: None,
    };

    let ns2 = NamespaceDeclaration {
        name: "SameName".to_string(),
        span: None,
    };

    assert_eq!(ns1, ns2);
}

#[test]
fn test_namespacedeclaration_not_eq_different_name() {
    let ns1 = NamespaceDeclaration {
        name: "FirstName".to_string(),
        span: None,
    };

    let ns2 = NamespaceDeclaration {
        name: "SecondName".to_string(),
        span: None,
    };

    assert_ne!(ns1, ns2);
}

#[test]
fn test_namespacedeclaration_not_eq_different_span() {
    let span1 = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 10,
        },
    };

    let span2 = Span {
        start: crate::core::span::Position { line: 2, column: 0 },
        end: crate::core::span::Position {
            line: 2,
            column: 10,
        },
    };

    let ns1 = NamespaceDeclaration {
        name: "SameName".to_string(),
        span: Some(span1),
    };

    let ns2 = NamespaceDeclaration {
        name: "SameName".to_string(),
        span: Some(span2),
    };

    assert_ne!(ns1, ns2);
}

#[test]
fn test_namespacedeclaration_debug_trait() {
    let ns = NamespaceDeclaration {
        name: "DebugTest".to_string(),
        span: None,
    };

    let debug_str = format!("{ns:?}");
    assert!(debug_str.contains("NamespaceDeclaration"));
    assert!(debug_str.contains("DebugTest"));
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_namespacedeclaration_qualified_name() {
    let ns = NamespaceDeclaration {
        name: "Outer::Inner::Deep".to_string(),
        span: None,
    };

    assert_eq!(ns.name, "Outer::Inner::Deep");
    assert!(ns.name.contains("::"));
}

#[test]
fn test_namespacedeclaration_unicode_name() {
    let ns = NamespaceDeclaration {
        name: "世界Namespace".to_string(),
        span: None,
    };

    assert_eq!(ns.name, "世界Namespace");
}

#[test]
fn test_namespacedeclaration_long_name() {
    let long_name = "Very".to_string() + &"Long".repeat(100) + "Namespace";
    let ns = NamespaceDeclaration {
        name: long_name.clone(),
        span: None,
    };

    assert_eq!(ns.name, long_name);
    assert!(ns.name.len() > 100);
}

#[test]
fn test_namespacedeclaration_with_special_chars() {
    let ns = NamespaceDeclaration {
        name: "Namespace_With_Underscores".to_string(),
        span: None,
    };

    assert_eq!(ns.name, "Namespace_With_Underscores");
}

#[test]
fn test_namespacedeclaration_numeric_name() {
    let ns = NamespaceDeclaration {
        name: "Namespace123".to_string(),
        span: None,
    };

    assert_eq!(ns.name, "Namespace123");
}
