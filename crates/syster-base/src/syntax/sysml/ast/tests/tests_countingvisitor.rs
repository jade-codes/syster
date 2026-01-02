#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use super::CountingVisitor;
use crate::syntax::sysml::visitor::Visitable;

// ============================================================================
// Tests for visit_namespace (Issue #178)
// ============================================================================

#[test]
fn test_visit_namespace_single() {
    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "TestNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 1);
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.usages, 0);
    assert_eq!(visitor.comments, 0);
    assert_eq!(visitor.imports, 0);
    assert_eq!(visitor.aliases, 0);
}

#[test]
fn test_visit_namespace_none() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 0);
}

#[test]
fn test_visit_namespace_with_elements() {
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

// ============================================================================
// Tests for visit_comment (Issue #177)
// ============================================================================

#[test]
fn test_visit_comment_single() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "This is a test comment".to_string(),
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.comments, 1);
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.usages, 0);
    assert_eq!(visitor.imports, 0);
    assert_eq!(visitor.aliases, 0);
    assert_eq!(visitor.namespaces, 0);
}

#[test]
fn test_visit_comment_multiple() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Comment(Comment {
                content: "First comment".to_string(),
                span: None,
            }),
            Element::Comment(Comment {
                content: "Second comment".to_string(),
                span: None,
            }),
            Element::Comment(Comment {
                content: "Third comment".to_string(),
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.comments, 3);
}

#[test]
fn test_visit_comment_empty_content() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "".to_string(),
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.comments, 1);
}

#[test]
fn test_visit_comment_with_other_elements() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Comment(Comment {
                content: "Documentation comment".to_string(),
                span: None,
            }),
            Element::Package(Package {
                name: Some("TestPkg".to_string()),
                elements: vec![],
                span: None,
            }),
            Element::Comment(Comment {
                content: "Another comment".to_string(),
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.comments, 2);
    assert_eq!(visitor.packages, 1);
}

// ============================================================================
// Tests for visit_import (Issue #176)
// ============================================================================

#[test]
fn test_visit_import_single() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Import(Import {
            path: "SomePackage::*".to_string(),
            path_span: None,
            is_recursive: false,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.imports, 1);
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.usages, 0);
    assert_eq!(visitor.comments, 0);
    assert_eq!(visitor.aliases, 0);
    assert_eq!(visitor.namespaces, 0);
}

#[test]
fn test_visit_import_multiple() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Import(Import {
                path: "Package1::*".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Import(Import {
                path: "Package2::Element".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Import(Import {
                path: "Package3::*::**".to_string(),
                path_span: None,
                is_recursive: true,
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.imports, 3);
}

#[test]
fn test_visit_import_recursive() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Import(Import {
            path: "Package::*::**".to_string(),
            path_span: None,
            is_recursive: true,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.imports, 1);
}

#[test]
fn test_visit_import_in_package() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(Package {
            name: Some("TestPkg".to_string()),
            elements: vec![
                Element::Import(Import {
                    path: "External::Type".to_string(),
                    path_span: None,
                    is_recursive: false,
                    span: None,
                }),
                Element::Import(Import {
                    path: "Another::Package::*".to_string(),
                    path_span: None,
                    is_recursive: false,
                    span: None,
                }),
            ],
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.imports, 2);
    assert_eq!(visitor.packages, 1);
}

// ============================================================================
// Tests for visit_usage (Issue #175)
// ============================================================================

#[test]
fn test_visit_usage_single() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(Usage {
            kind: UsageKind::Part,
            name: Some("myPart".to_string()),
            body: vec![],
            relationships: Relationships::none(),
            is_derived: false,
            is_readonly: false,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.comments, 0);
    assert_eq!(visitor.imports, 0);
    assert_eq!(visitor.aliases, 0);
    assert_eq!(visitor.namespaces, 0);
}

#[test]
fn test_visit_usage_multiple_kinds() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Usage(Usage {
                kind: UsageKind::Part,
                name: Some("part1".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_derived: false,
                is_readonly: false,
                span: None,
            }),
            Element::Usage(Usage {
                kind: UsageKind::Action,
                name: Some("action1".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_derived: false,
                is_readonly: false,
                span: None,
            }),
            Element::Usage(Usage {
                kind: UsageKind::Port,
                name: Some("port1".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_derived: false,
                is_readonly: false,
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usages, 3);
}

#[test]
fn test_visit_usage_anonymous() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(Usage {
            kind: UsageKind::Part,
            name: None,
            body: vec![],
            relationships: Relationships::none(),
            is_derived: false,
            is_readonly: false,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_visit_usage_with_relationships() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(Usage {
            kind: UsageKind::Part,
            name: Some("myPart".to_string()),
            body: vec![],
            relationships: Relationships {
                typed_by: Some("PartType".to_string()),
                typed_by_span: None,
                subsets: vec![SubsettingRel {
                    target: "basePart".to_string(),
                    span: None,
                }],
                ..Relationships::none()
            },
            is_derived: false,
            is_readonly: false,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_visit_usage_derived_readonly() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(Usage {
            kind: UsageKind::Attribute,
            name: Some("derivedAttr".to_string()),
            body: vec![],
            relationships: Relationships::none(),
            is_derived: true,
            is_readonly: true,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

// ============================================================================
// Tests for visit_alias (Issue #174)
// ============================================================================

#[test]
fn test_visit_alias_single() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Alias(Alias {
            name: Some("MyAlias".to_string()),
            target: "Target::Element".to_string(),
            target_span: None,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.aliases, 1);
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.usages, 0);
    assert_eq!(visitor.comments, 0);
    assert_eq!(visitor.imports, 0);
    assert_eq!(visitor.namespaces, 0);
}

#[test]
fn test_visit_alias_multiple() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Alias(Alias {
                name: Some("Alias1".to_string()),
                target: "Target1".to_string(),
                target_span: None,
                span: None,
            }),
            Element::Alias(Alias {
                name: Some("Alias2".to_string()),
                target: "Target2".to_string(),
                target_span: None,
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.aliases, 2);
}

#[test]
fn test_visit_alias_anonymous() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Alias(Alias {
            name: None,
            target: "Target::Element".to_string(),
            target_span: None,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.aliases, 1);
}

#[test]
fn test_visit_alias_qualified_target() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Alias(Alias {
            name: Some("LongAlias".to_string()),
            target: "Package::SubPackage::Element".to_string(),
            target_span: None,
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.aliases, 1);
}

// ============================================================================
// Integration tests - Testing multiple element types together
// ============================================================================

#[test]
fn test_visit_empty_file() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.usages, 0);
    assert_eq!(visitor.comments, 0);
    assert_eq!(visitor.imports, 0);
    assert_eq!(visitor.aliases, 0);
    assert_eq!(visitor.namespaces, 0);
}

#[test]
fn test_visit_all_element_types() {
    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "TestNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![
            Element::Import(Import {
                path: "External::*".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Comment(Comment {
                content: "File comment".to_string(),
                span: None,
            }),
            Element::Alias(Alias {
                name: Some("MyAlias".to_string()),
                target: "External::Type".to_string(),
                target_span: None,
                span: None,
            }),
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
            Element::Usage(Usage {
                kind: UsageKind::Part,
                name: Some("testUsage".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_derived: false,
                is_readonly: false,
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 1);
    assert_eq!(visitor.imports, 1);
    assert_eq!(visitor.comments, 1);
    assert_eq!(visitor.aliases, 1);
    assert_eq!(visitor.packages, 1);
    assert_eq!(visitor.definitions, 1);
    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_visit_nested_elements_in_package() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(Package {
            name: Some("OuterPackage".to_string()),
            elements: vec![
                Element::Comment(Comment {
                    content: "Package comment".to_string(),
                    span: None,
                }),
                Element::Import(Import {
                    path: "External::Type".to_string(),
                    path_span: None,
                    is_recursive: false,
                    span: None,
                }),
                Element::Definition(Definition {
                    kind: DefinitionKind::Part,
                    name: Some("InnerDef".to_string()),
                    body: vec![],
                    relationships: Relationships::none(),
                    is_abstract: false,
                    is_variation: false,
                    span: None,
                }),
                Element::Usage(Usage {
                    kind: UsageKind::Part,
                    name: Some("innerUsage".to_string()),
                    body: vec![],
                    relationships: Relationships::none(),
                    is_derived: false,
                    is_readonly: false,
                    span: None,
                }),
                Element::Alias(Alias {
                    name: Some("InnerAlias".to_string()),
                    target: "Target".to_string(),
                    target_span: None,
                    span: None,
                }),
            ],
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.packages, 1);
    assert_eq!(visitor.comments, 1);
    assert_eq!(visitor.imports, 1);
    assert_eq!(visitor.definitions, 1);
    assert_eq!(visitor.usages, 1);
    assert_eq!(visitor.aliases, 1);
}

#[test]
fn test_visit_deeply_nested_packages() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(Package {
            name: Some("Outer".to_string()),
            elements: vec![
                Element::Package(Package {
                    name: Some("Middle".to_string()),
                    elements: vec![Element::Package(Package {
                        name: Some("Inner".to_string()),
                        elements: vec![Element::Definition(Definition {
                            kind: DefinitionKind::Part,
                            name: Some("DeepDef".to_string()),
                            body: vec![],
                            relationships: Relationships::none(),
                            is_abstract: false,
                            is_variation: false,
                            span: None,
                        })],
                        span: None,
                    })],
                    span: None,
                }),
                Element::Definition(Definition {
                    kind: DefinitionKind::Part,
                    name: Some("MiddleDef".to_string()),
                    body: vec![],
                    relationships: Relationships::none(),
                    is_abstract: false,
                    is_variation: false,
                    span: None,
                }),
            ],
            span: None,
        })],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.packages, 3);
    assert_eq!(visitor.definitions, 2);
}

#[test]
fn test_visit_multiple_of_each_type() {
    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "NS1".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![
            Element::Package(Package {
                name: Some("Pkg1".to_string()),
                elements: vec![],
                span: None,
            }),
            Element::Package(Package {
                name: Some("Pkg2".to_string()),
                elements: vec![],
                span: None,
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("Def1".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_abstract: false,
                is_variation: false,
                span: None,
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Action,
                name: Some("Def2".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_abstract: false,
                is_variation: false,
                span: None,
            }),
            Element::Usage(Usage {
                kind: UsageKind::Part,
                name: Some("usage1".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_derived: false,
                is_readonly: false,
                span: None,
            }),
            Element::Usage(Usage {
                kind: UsageKind::Action,
                name: Some("usage2".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_derived: false,
                is_readonly: false,
                span: None,
            }),
            Element::Comment(Comment {
                content: "Comment 1".to_string(),
                span: None,
            }),
            Element::Comment(Comment {
                content: "Comment 2".to_string(),
                span: None,
            }),
            Element::Import(Import {
                path: "Import1::*".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Import(Import {
                path: "Import2::Element".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Alias(Alias {
                name: Some("Alias1".to_string()),
                target: "Target1".to_string(),
                target_span: None,
                span: None,
            }),
            Element::Alias(Alias {
                name: Some("Alias2".to_string()),
                target: "Target2".to_string(),
                target_span: None,
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.namespaces, 1);
    assert_eq!(visitor.packages, 2);
    assert_eq!(visitor.definitions, 2);
    assert_eq!(visitor.usages, 2);
    assert_eq!(visitor.comments, 2);
    assert_eq!(visitor.imports, 2);
    assert_eq!(visitor.aliases, 2);
}

#[test]
fn test_visit_large_file() {
    // Test with a larger number of elements
    let mut elements = Vec::new();

    // Add 10 packages
    for i in 0..10 {
        elements.push(Element::Package(Package {
            name: Some(format!("Pkg{}", i)),
            elements: vec![],
            span: None,
        }));
    }

    // Add 15 definitions
    for i in 0..15 {
        elements.push(Element::Definition(Definition {
            kind: DefinitionKind::Part,
            name: Some(format!("Def{}", i)),
            body: vec![],
            relationships: Relationships::none(),
            is_abstract: false,
            is_variation: false,
            span: None,
        }));
    }

    // Add 20 usages
    for i in 0..20 {
        elements.push(Element::Usage(Usage {
            kind: UsageKind::Part,
            name: Some(format!("usage{}", i)),
            body: vec![],
            relationships: Relationships::none(),
            is_derived: false,
            is_readonly: false,
            span: None,
        }));
    }

    // Add 5 comments
    for i in 0..5 {
        elements.push(Element::Comment(Comment {
            content: format!("Comment {}", i),
            span: None,
        }));
    }

    // Add 8 imports
    for i in 0..8 {
        elements.push(Element::Import(Import {
            path: format!("Package{}::*", i),
            path_span: None,
            is_recursive: false,
            span: None,
        }));
    }

    // Add 3 aliases
    for i in 0..3 {
        elements.push(Element::Alias(Alias {
            name: Some(format!("Alias{}", i)),
            target: format!("Target{}", i),
            target_span: None,
            span: None,
        }));
    }

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements,
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.packages, 10);
    assert_eq!(visitor.definitions, 15);
    assert_eq!(visitor.usages, 20);
    assert_eq!(visitor.comments, 5);
    assert_eq!(visitor.imports, 8);
    assert_eq!(visitor.aliases, 3);
}
