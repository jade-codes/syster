#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use super::CountingVisitor;
use crate::core::Span;
use crate::syntax::sysml::visitor::{AstVisitor, Visitable};

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

    let debug_str = format!("{:?}", ns);
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

// ============================================================================
// Visitable trait tests with generic visitor (Issue #162)
// ============================================================================

struct GenericTestVisitor {
    namespace_visited: bool,
    namespace_name: Option<String>,
}

impl AstVisitor for GenericTestVisitor {
    fn visit_namespace(&mut self, namespace: &NamespaceDeclaration) {
        self.namespace_visited = true;
        self.namespace_name = Some(namespace.name.clone());
    }
}

#[test]
fn test_namespacedeclaration_visitable_accept_generic() {
    let ns = NamespaceDeclaration {
        name: "VisitorTest".to_string(),
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    ns.accept(&mut visitor);

    assert!(
        visitor.namespace_visited,
        "Namespace should be visited by generic visitor"
    );
    assert_eq!(
        visitor.namespace_name,
        Some("VisitorTest".to_string()),
        "Visitor should capture namespace name"
    );
}

#[test]
fn test_namespacedeclaration_visitable_with_multiple_visitors() {
    let ns = NamespaceDeclaration {
        name: "MultipleVisitors".to_string(),
        span: None,
    };

    let mut visitor1 = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    let mut visitor2 = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    ns.accept(&mut visitor1);
    ns.accept(&mut visitor2);

    assert!(visitor1.namespace_visited);
    assert!(visitor2.namespace_visited);
    assert_eq!(visitor1.namespace_name, visitor2.namespace_name);
}

#[test]
fn test_namespacedeclaration_visitable_empty_name() {
    let ns = NamespaceDeclaration {
        name: String::new(),
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    ns.accept(&mut visitor);

    assert!(visitor.namespace_visited);
    assert_eq!(visitor.namespace_name, Some(String::new()));
}

#[test]
fn test_namespacedeclaration_visitable_with_span() {
    let span = Span {
        start: crate::core::span::Position {
            line: 5,
            column: 10,
        },
        end: crate::core::span::Position {
            line: 5,
            column: 30,
        },
    };

    let ns = NamespaceDeclaration {
        name: "SpannedNamespace".to_string(),
        span: Some(span),
    };

    let mut visitor = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    ns.accept(&mut visitor);

    assert!(visitor.namespace_visited);
    assert_eq!(visitor.namespace_name, Some("SpannedNamespace".to_string()));
}

#[test]
fn test_namespacedeclaration_visitable_qualified_name() {
    let ns = NamespaceDeclaration {
        name: "Outer::Inner::Namespace".to_string(),
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    ns.accept(&mut visitor);

    assert!(visitor.namespace_visited);
    assert_eq!(
        visitor.namespace_name,
        Some("Outer::Inner::Namespace".to_string())
    );
}

// ============================================================================
// Visitable trait tests with CountingVisitor (Issue #161)
// ============================================================================

#[test]
fn test_namespacedeclaration_visitable_accept_counting_visitor() {
    let ns = NamespaceDeclaration {
        name: "CountingTest".to_string(),
        span: None,
    };

    let mut visitor = CountingVisitor::new();

    ns.accept(&mut visitor);

    assert_eq!(
        visitor.namespaces, 1,
        "Should count exactly one namespace visit"
    );
    assert_eq!(visitor.packages, 0, "Should not count any packages");
    assert_eq!(visitor.definitions, 0, "Should not count any definitions");
    assert_eq!(visitor.usages, 0, "Should not count any usages");
    assert_eq!(visitor.comments, 0, "Should not count any comments");
    assert_eq!(visitor.imports, 0, "Should not count any imports");
    assert_eq!(visitor.aliases, 0, "Should not count any aliases");
}

#[test]
fn test_namespacedeclaration_visitable_counting_multiple() {
    let ns1 = NamespaceDeclaration {
        name: "First".to_string(),
        span: None,
    };
    let ns2 = NamespaceDeclaration {
        name: "Second".to_string(),
        span: None,
    };
    let ns3 = NamespaceDeclaration {
        name: "Third".to_string(),
        span: None,
    };

    let mut visitor = CountingVisitor::new();

    ns1.accept(&mut visitor);
    ns2.accept(&mut visitor);
    ns3.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 3, "Should count all three namespaces");
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.usages, 0);
}

#[test]
fn test_namespacedeclaration_visitable_counting_zero_initial() {
    let visitor = CountingVisitor::new();

    assert_eq!(
        visitor.namespaces, 0,
        "Initial namespace count should be zero"
    );
}

#[test]
fn test_namespacedeclaration_visitable_counting_with_empty_name() {
    let ns = NamespaceDeclaration {
        name: String::new(),
        span: None,
    };

    let mut visitor = CountingVisitor::new();

    ns.accept(&mut visitor);

    assert_eq!(
        visitor.namespaces, 1,
        "Should count namespace even with empty name"
    );
}

#[test]
fn test_namespacedeclaration_visitable_counting_with_span() {
    let span = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 10,
        },
    };

    let ns = NamespaceDeclaration {
        name: "WithSpan".to_string(),
        span: Some(span),
    };

    let mut visitor = CountingVisitor::new();

    ns.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 1, "Should count namespace with span");
}

// ============================================================================
// NamespaceDeclaration in SysMLFile context tests
// ============================================================================

#[test]
fn test_namespacedeclaration_in_sysmlfile_single() {
    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "FileNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 1, "Should visit namespace in file");
}

#[test]
fn test_namespacedeclaration_in_sysmlfile_none() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(
        visitor.namespaces, 0,
        "Should not visit namespace when None"
    );
}

#[test]
fn test_namespacedeclaration_in_sysmlfile_with_elements() {
    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "MyNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![
            Element::Package(Package {
                name: Some("TestPkg".to_string()),
                elements: vec![],
                span: None,
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("TestDef".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_abstract: false,
                is_variation: false,
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 1);
    assert_eq!(visitor.packages, 1);
    assert_eq!(visitor.definitions, 1);
}

#[test]
fn test_namespacedeclaration_generic_visitor_in_file() {
    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "FileNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let mut visitor = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    file.accept(&mut visitor);

    assert!(
        visitor.namespace_visited,
        "Generic visitor should visit namespace in file"
    );
    assert_eq!(visitor.namespace_name, Some("FileNamespace".to_string()));
}

#[test]
fn test_namespacedeclaration_in_file_with_mixed_elements() {
    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "MixedNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![
            Element::Comment(Comment {
                content: "A comment".to_string(),
                span: None,
            }),
            Element::Import(Import {
                path: "External::*".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Package(Package {
                name: Some("InnerPkg".to_string()),
                elements: vec![],
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 1);
    assert_eq!(visitor.comments, 1);
    assert_eq!(visitor.imports, 1);
    assert_eq!(visitor.packages, 1);
}

// ============================================================================
// Integration tests with multiple namespace declarations
// ============================================================================

#[test]
fn test_namespacedeclaration_multiple_in_vector() {
    let ns_vec = vec![
        NamespaceDeclaration {
            name: "NS1".to_string(),
            span: None,
        },
        NamespaceDeclaration {
            name: "NS2".to_string(),
            span: None,
        },
        NamespaceDeclaration {
            name: "NS3".to_string(),
            span: None,
        },
    ];

    let mut visitor = CountingVisitor::new();

    for ns in &ns_vec {
        ns.accept(&mut visitor);
    }

    assert_eq!(visitor.namespaces, 3);
}

#[test]
fn test_namespacedeclaration_clone_and_visit() {
    let ns1 = NamespaceDeclaration {
        name: "Original".to_string(),
        span: None,
    };

    let ns2 = ns1.clone();

    let mut visitor1 = CountingVisitor::new();
    let mut visitor2 = CountingVisitor::new();

    ns1.accept(&mut visitor1);
    ns2.accept(&mut visitor2);

    assert_eq!(visitor1.namespaces, 1);
    assert_eq!(visitor2.namespaces, 1);
}

#[test]
fn test_namespacedeclaration_eq_and_visit() {
    let ns1 = NamespaceDeclaration {
        name: "Same".to_string(),
        span: None,
    };

    let ns2 = NamespaceDeclaration {
        name: "Same".to_string(),
        span: None,
    };

    assert_eq!(ns1, ns2, "Namespaces should be equal");

    let mut visitor = CountingVisitor::new();

    ns1.accept(&mut visitor);
    ns2.accept(&mut visitor);

    assert_eq!(
        visitor.namespaces, 2,
        "Should count both namespace visits even if equal"
    );
}

// ============================================================================
// Stress tests
// ============================================================================

#[test]
fn test_namespacedeclaration_many_accepts() {
    let ns = NamespaceDeclaration {
        name: "StressTest".to_string(),
        span: None,
    };

    let mut visitor = CountingVisitor::new();

    for _ in 0..1000 {
        ns.accept(&mut visitor);
    }

    assert_eq!(
        visitor.namespaces, 1000,
        "Should accurately count many accepts"
    );
}

#[test]
fn test_namespacedeclaration_very_long_name_with_visitor() {
    let long_name = "NS".to_string() + &"_Long".repeat(1000);
    let ns = NamespaceDeclaration {
        name: long_name.clone(),
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        namespace_visited: false,
        namespace_name: None,
    };

    ns.accept(&mut visitor);

    assert!(visitor.namespace_visited);
    assert_eq!(visitor.namespace_name, Some(long_name));
}

#[test]
fn test_namespacedeclaration_many_instances() {
    let mut visitor = CountingVisitor::new();

    for i in 0..100 {
        let ns = NamespaceDeclaration {
            name: format!("Namespace{}", i),
            span: None,
        };
        ns.accept(&mut visitor);
    }

    assert_eq!(visitor.namespaces, 100);
}
