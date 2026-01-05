#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use super::CountingVisitor;
use crate::core::Span;
use crate::syntax::sysml::visitor::{AstVisitor, Visitable};

// ============================================================================
// Usage struct tests
// ============================================================================

#[test]
fn test_usage_creation() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myPart".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: None,
        is_derived: false,
        is_readonly: false,
    };

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("myPart".to_string()));
    assert_eq!(usage.body.len(), 0);
    assert_eq!(usage.span, None);
    assert!(!usage.is_derived);
    assert!(!usage.is_readonly);
}

#[test]
fn test_usage_new_constructor() {
    let usage = Usage::new(
        UsageKind::Action,
        Some("myAction".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Action);
    assert_eq!(usage.name, Some("myAction".to_string()));
    assert_eq!(usage.body.len(), 0);
    assert_eq!(usage.span, None);
    assert!(!usage.is_derived);
    assert!(!usage.is_readonly);
}

#[test]
fn test_usage_with_span() {
    let span = Span {
        start: crate::core::span::Position {
            line: 5,
            column: 10,
        },
        end: crate::core::span::Position {
            line: 5,
            column: 20,
        },
    };

    let usage = Usage {
        kind: UsageKind::Port,
        name: Some("myPort".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(span),
        is_derived: false,
        is_readonly: false,
    };

    assert_eq!(usage.span, Some(span));
}

#[test]
fn test_usage_anonymous() {
    let usage = Usage::new(UsageKind::Part, None, Relationships::none(), vec![]);

    assert_eq!(usage.name, None);
}

#[test]
fn test_usage_derived_flag() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("derivedAttr".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: None,
        is_derived: true,
        is_readonly: false,
    };

    assert!(usage.is_derived);
    assert!(!usage.is_readonly);
}

#[test]
fn test_usage_readonly_flag() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("readonlyAttr".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: None,
        is_derived: false,
        is_readonly: true,
    };

    assert!(!usage.is_derived);
    assert!(usage.is_readonly);
}

#[test]
fn test_usage_derived_and_readonly() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("constAttr".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: None,
        is_derived: true,
        is_readonly: true,
    };

    assert!(usage.is_derived);
    assert!(usage.is_readonly);
}

#[test]
fn test_usage_with_relationships() {
    let relationships = Relationships {
        typed_by: Some("PartType".to_string()),
        typed_by_span: None,
        subsets: vec![SubsettingRel {
            target: "basePart".to_string(),
            span: None,
        }],
        ..Relationships::none()
    };

    let usage = Usage::new(
        UsageKind::Part,
        Some("specialPart".to_string()),
        relationships.clone(),
        vec![],
    );

    assert_eq!(usage.relationships.typed_by, Some("PartType".to_string()));
    assert_eq!(usage.relationships.subsets.len(), 1);
    assert_eq!(usage.relationships.subsets[0].target, "basePart");
}

#[test]
fn test_usage_clone() {
    let usage1 = Usage::new(
        UsageKind::Item,
        Some("item1".to_string()),
        Relationships::none(),
        vec![],
    );

    let usage2 = usage1.clone();

    assert_eq!(usage1.kind, usage2.kind);
    assert_eq!(usage1.name, usage2.name);
    assert_eq!(usage1.relationships, usage2.relationships);
}

#[test]
fn test_usage_partial_eq() {
    let usage1 = Usage::new(
        UsageKind::Part,
        Some("part1".to_string()),
        Relationships::none(),
        vec![],
    );

    let usage2 = Usage::new(
        UsageKind::Part,
        Some("part1".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage1, usage2);
}

#[test]
fn test_usage_not_eq_different_kind() {
    let usage1 = Usage::new(
        UsageKind::Part,
        Some("test".to_string()),
        Relationships::none(),
        vec![],
    );

    let usage2 = Usage::new(
        UsageKind::Action,
        Some("test".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_ne!(usage1, usage2);
}

#[test]
fn test_usage_not_eq_different_name() {
    let usage1 = Usage::new(
        UsageKind::Part,
        Some("part1".to_string()),
        Relationships::none(),
        vec![],
    );

    let usage2 = Usage::new(
        UsageKind::Part,
        Some("part2".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_ne!(usage1, usage2);
}

#[test]
fn test_usage_debug_trait() {
    let usage = Usage::new(
        UsageKind::Part,
        Some("testPart".to_string()),
        Relationships::none(),
        vec![],
    );

    let debug_str = format!("{:?}", usage);
    assert!(debug_str.contains("Usage"));
    assert!(debug_str.contains("testPart"));
}

// ============================================================================
// All UsageKind variants tests
// ============================================================================

#[test]
fn test_usage_kind_part() {
    let usage = Usage::new(
        UsageKind::Part,
        Some("part".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Part);
}

#[test]
fn test_usage_kind_port() {
    let usage = Usage::new(
        UsageKind::Port,
        Some("port".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Port);
}

#[test]
fn test_usage_kind_action() {
    let usage = Usage::new(
        UsageKind::Action,
        Some("action".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Action);
}

#[test]
fn test_usage_kind_item() {
    let usage = Usage::new(
        UsageKind::Item,
        Some("item".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Item);
}

#[test]
fn test_usage_kind_attribute() {
    let usage = Usage::new(
        UsageKind::Attribute,
        Some("attribute".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Attribute);
}

#[test]
fn test_usage_kind_requirement() {
    let usage = Usage::new(
        UsageKind::Requirement,
        Some("requirement".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Requirement);
}

#[test]
fn test_usage_kind_concern() {
    let usage = Usage::new(
        UsageKind::Concern,
        Some("concern".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Concern);
}

#[test]
fn test_usage_kind_case() {
    let usage = Usage::new(
        UsageKind::Case,
        Some("case".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Case);
}

#[test]
fn test_usage_kind_view() {
    let usage = Usage::new(
        UsageKind::View,
        Some("view".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::View);
}

#[test]
fn test_usage_kind_enumeration() {
    let usage = Usage::new(
        UsageKind::Enumeration,
        Some("enumeration".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Enumeration);
}

#[test]
fn test_usage_kind_satisfy_requirement() {
    let usage = Usage::new(
        UsageKind::SatisfyRequirement,
        Some("satisfy".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::SatisfyRequirement);
}

#[test]
fn test_usage_kind_perform_action() {
    let usage = Usage::new(
        UsageKind::PerformAction,
        Some("perform".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::PerformAction);
}

#[test]
fn test_usage_kind_exhibit_state() {
    let usage = Usage::new(
        UsageKind::ExhibitState,
        Some("exhibit".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::ExhibitState);
}

#[test]
fn test_usage_kind_include_use_case() {
    let usage = Usage::new(
        UsageKind::IncludeUseCase,
        Some("include".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::IncludeUseCase);
}

// ============================================================================
// Usage as Element tests
// ============================================================================

#[test]
fn test_usage_as_element() {
    let usage = Usage::new(
        UsageKind::Part,
        Some("testPart".to_string()),
        Relationships::none(),
        vec![],
    );

    let element = Element::Usage(usage.clone());

    match element {
        Element::Usage(u) => {
            assert_eq!(u.name, Some("testPart".to_string()));
            assert_eq!(u, usage);
        }
        _ => panic!("Expected Element::Usage variant"),
    }
}

#[test]
fn test_usage_element_pattern_matching() {
    let usage = Usage::new(
        UsageKind::Action,
        Some("action1".to_string()),
        Relationships::none(),
        vec![],
    );

    let element = Element::Usage(usage);

    if let Element::Usage(u) = element {
        assert_eq!(u.kind, UsageKind::Action);
        assert_eq!(u.name, Some("action1".to_string()));
    } else {
        panic!("Failed to match Element::Usage");
    }
}

// ============================================================================
// Visitable trait tests with generic visitor (Issue #166)
// ============================================================================

struct GenericUsageTestVisitor {
    usage_visited: bool,
    usage_kind: Option<UsageKind>,
    usage_name: Option<String>,
}

impl AstVisitor for GenericUsageTestVisitor {
    fn visit_usage(&mut self, usage: &Usage) {
        self.usage_visited = true;
        self.usage_kind = Some(usage.kind.clone());
        self.usage_name = usage.name.clone();
    }
}

#[test]
fn test_usage_visitable_accept_generic() {
    let usage = Usage::new(
        UsageKind::Part,
        Some("testPart".to_string()),
        Relationships::none(),
        vec![],
    );

    let mut visitor = GenericUsageTestVisitor {
        usage_visited: false,
        usage_kind: None,
        usage_name: None,
    };

    usage.accept(&mut visitor);

    assert!(visitor.usage_visited, "Usage should be visited");
    assert_eq!(
        visitor.usage_kind,
        Some(UsageKind::Part),
        "Visitor should capture usage kind"
    );
    assert_eq!(
        visitor.usage_name,
        Some("testPart".to_string()),
        "Visitor should capture usage name"
    );
}

#[test]
fn test_usage_visitable_with_multiple_visitors() {
    let usage = Usage::new(
        UsageKind::Action,
        Some("multiAction".to_string()),
        Relationships::none(),
        vec![],
    );

    let mut visitor1 = GenericUsageTestVisitor {
        usage_visited: false,
        usage_kind: None,
        usage_name: None,
    };

    let mut visitor2 = GenericUsageTestVisitor {
        usage_visited: false,
        usage_kind: None,
        usage_name: None,
    };

    usage.accept(&mut visitor1);
    usage.accept(&mut visitor2);

    assert!(visitor1.usage_visited);
    assert!(visitor2.usage_visited);
    assert_eq!(visitor1.usage_kind, visitor2.usage_kind);
    assert_eq!(visitor1.usage_name, visitor2.usage_name);
}

#[test]
fn test_usage_visitable_anonymous() {
    let usage = Usage::new(UsageKind::Part, None, Relationships::none(), vec![]);

    let mut visitor = GenericUsageTestVisitor {
        usage_visited: false,
        usage_kind: None,
        usage_name: None,
    };

    usage.accept(&mut visitor);

    assert!(visitor.usage_visited);
    assert_eq!(visitor.usage_kind, Some(UsageKind::Part));
    assert_eq!(visitor.usage_name, None);
}

#[test]
fn test_usage_visitable_with_span() {
    let span = Span {
        start: crate::core::span::Position {
            line: 10,
            column: 5,
        },
        end: crate::core::span::Position {
            line: 10,
            column: 15,
        },
    };

    let usage = Usage {
        kind: UsageKind::Port,
        name: Some("portWithSpan".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: Some(span),
        is_derived: false,
        is_readonly: false,
    };

    let mut visitor = GenericUsageTestVisitor {
        usage_visited: false,
        usage_kind: None,
        usage_name: None,
    };

    usage.accept(&mut visitor);

    assert!(visitor.usage_visited);
    assert_eq!(visitor.usage_name, Some("portWithSpan".to_string()));
}

#[test]
fn test_usage_visitable_all_kinds() {
    let kinds = vec![
        UsageKind::Part,
        UsageKind::Port,
        UsageKind::Action,
        UsageKind::Item,
        UsageKind::Attribute,
        UsageKind::Requirement,
        UsageKind::Concern,
        UsageKind::Case,
        UsageKind::View,
        UsageKind::Enumeration,
        UsageKind::SatisfyRequirement,
        UsageKind::PerformAction,
        UsageKind::ExhibitState,
        UsageKind::IncludeUseCase,
    ];

    for kind in kinds {
        let usage = Usage::new(
            kind.clone(),
            Some("test".to_string()),
            Relationships::none(),
            vec![],
        );

        let mut visitor = GenericUsageTestVisitor {
            usage_visited: false,
            usage_kind: None,
            usage_name: None,
        };

        usage.accept(&mut visitor);

        assert!(
            visitor.usage_visited,
            "Usage of kind {:?} should be visited",
            kind
        );
        assert_eq!(visitor.usage_kind, Some(kind.clone()));
    }
}

#[test]
fn test_usage_visitable_derived_readonly() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("constAttr".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: None,
        is_derived: true,
        is_readonly: true,
    };

    let mut visitor = GenericUsageTestVisitor {
        usage_visited: false,
        usage_kind: None,
        usage_name: None,
    };

    usage.accept(&mut visitor);

    assert!(visitor.usage_visited);
    assert_eq!(visitor.usage_kind, Some(UsageKind::Attribute));
}

// ============================================================================
// Visitable trait tests with CountingVisitor (Issue #165)
// ============================================================================

#[test]
fn test_usage_visitable_accept_counting_visitor() {
    let usage = Usage::new(
        UsageKind::Part,
        Some("countingTest".to_string()),
        Relationships::none(),
        vec![],
    );

    let mut visitor = CountingVisitor::new();

    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1, "Should visit exactly one usage");
    assert_eq!(visitor.packages, 0, "Should not visit any packages");
    assert_eq!(visitor.definitions, 0, "Should not visit any definitions");
    assert_eq!(visitor.comments, 0, "Should not visit any comments");
}

#[test]
fn test_usage_visitable_counting_multiple_usages() {
    let usage1 = Usage::new(
        UsageKind::Part,
        Some("part1".to_string()),
        Relationships::none(),
        vec![],
    );
    let usage2 = Usage::new(
        UsageKind::Action,
        Some("action1".to_string()),
        Relationships::none(),
        vec![],
    );
    let usage3 = Usage::new(
        UsageKind::Port,
        Some("port1".to_string()),
        Relationships::none(),
        vec![],
    );

    let mut visitor = CountingVisitor::new();

    usage1.accept(&mut visitor);
    usage2.accept(&mut visitor);
    usage3.accept(&mut visitor);

    assert_eq!(visitor.usages, 3, "Should count all three usages");
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
}

#[test]
fn test_usage_element_with_counting_visitor() {
    let usage = Usage::new(
        UsageKind::Item,
        Some("testItem".to_string()),
        Relationships::none(),
        vec![],
    );
    let element = Element::Usage(usage);

    let mut visitor = CountingVisitor::new();

    element.accept(&mut visitor);

    assert_eq!(visitor.usages, 1, "Should count usage through element");
}

#[test]
fn test_usage_counting_visitor_zero_initial() {
    let visitor = CountingVisitor::new();

    assert_eq!(visitor.usages, 0, "Initial usage count should be zero");
    assert_eq!(visitor.packages, 0, "Initial package count should be zero");
    assert_eq!(
        visitor.definitions, 0,
        "Initial definition count should be zero"
    );
}

#[test]
fn test_usage_in_file_with_counting_visitor() {
    let usage1 = Usage::new(
        UsageKind::Part,
        Some("part1".to_string()),
        Relationships::none(),
        vec![],
    );
    let usage2 = Usage::new(
        UsageKind::Action,
        Some("action1".to_string()),
        Relationships::none(),
        vec![],
    );

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Usage(usage1), Element::Usage(usage2)],
    };

    let mut visitor = CountingVisitor::new();

    file.accept(&mut visitor);

    assert_eq!(visitor.usages, 2, "Should count both usages in file");
}

#[test]
fn test_usage_counting_anonymous_usages() {
    let usage1 = Usage::new(UsageKind::Part, None, Relationships::none(), vec![]);
    let usage2 = Usage::new(UsageKind::Action, None, Relationships::none(), vec![]);

    let mut visitor = CountingVisitor::new();

    usage1.accept(&mut visitor);
    usage2.accept(&mut visitor);

    assert_eq!(visitor.usages, 2, "Should count anonymous usages correctly");
}

#[test]
fn test_usage_counting_all_kinds() {
    let kinds = [
        UsageKind::Part,
        UsageKind::Port,
        UsageKind::Action,
        UsageKind::Item,
        UsageKind::Attribute,
        UsageKind::Requirement,
        UsageKind::Concern,
        UsageKind::Case,
        UsageKind::View,
        UsageKind::Enumeration,
        UsageKind::SatisfyRequirement,
        UsageKind::PerformAction,
        UsageKind::ExhibitState,
        UsageKind::IncludeUseCase,
    ];

    let mut visitor = CountingVisitor::new();

    for kind in kinds.iter() {
        let usage = Usage::new(
            kind.clone(),
            Some("test".to_string()),
            Relationships::none(),
            vec![],
        );
        usage.accept(&mut visitor);
    }

    assert_eq!(
        visitor.usages,
        kinds.len(),
        "Should count all usage kinds correctly"
    );
}

#[test]
fn test_usage_counting_mixed_elements() {
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Package(Package {
                name: Some("Pkg".to_string()),
                elements: vec![],
                span: None,
            }),
            Element::Usage(Usage::new(
                UsageKind::Part,
                Some("part".to_string()),
                Relationships::none(),
                vec![],
            )),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("Def".to_string()),
                body: vec![],
                relationships: Relationships::none(),
                is_abstract: false,
                is_variation: false,
                span: None,
            }),
            Element::Usage(Usage::new(
                UsageKind::Action,
                Some("action".to_string()),
                Relationships::none(),
                vec![],
            )),
            Element::Comment(Comment {
                content: "test".to_string(),
                span: None,
            }),
        ],
    };

    let mut visitor = CountingVisitor::new();
    file.accept(&mut visitor);

    assert_eq!(visitor.usages, 2, "Should count only usages");
    assert_eq!(visitor.packages, 1, "Should count package");
    assert_eq!(visitor.definitions, 1, "Should count definition");
    assert_eq!(visitor.comments, 1, "Should count comment");
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_usage_with_complex_relationships() {
    let relationships = Relationships {
        typed_by: Some("ComplexType".to_string()),
        typed_by_span: None,
        specializes: vec![
            SpecializationRel {
                target: "Base1".to_string(),
                span: None,
            },
            SpecializationRel {
                target: "Base2".to_string(),
                span: None,
            },
        ],
        subsets: vec![
            SubsettingRel {
                target: "Subset1".to_string(),
                span: None,
            },
            SubsettingRel {
                target: "Subset2".to_string(),
                span: None,
            },
        ],
        redefines: vec![RedefinitionRel {
            target: "Original".to_string(),
            span: None,
        }],
        references: vec![ReferenceRel {
            target: "RefTarget".to_string(),
            span: None,
        }],
        crosses: vec![CrossRel {
            target: "CrossTarget".to_string(),
            span: None,
        }],
        satisfies: vec![SatisfyRel {
            target: "Requirement1".to_string(),
            span: None,
        }],
        performs: vec![PerformRel {
            target: "Action1".to_string(),
            span: None,
        }],
        exhibits: vec![ExhibitRel {
            target: "State1".to_string(),
            span: None,
        }],
        includes: vec![IncludeRel {
            target: "UseCase1".to_string(),
            span: None,
        }],
        asserts: vec![AssertRel {
            target: "Constraint1".to_string(),
            span: None,
        }],
        verifies: vec![VerifyRel {
            target: "Verification1".to_string(),
            span: None,
        }],
    };

    let usage = Usage::new(
        UsageKind::Part,
        Some("complexPart".to_string()),
        relationships.clone(),
        vec![],
    );

    assert_eq!(
        usage.relationships.typed_by,
        Some("ComplexType".to_string())
    );
    assert_eq!(usage.relationships.specializes.len(), 2);
    assert_eq!(usage.relationships.subsets.len(), 2);
    assert_eq!(usage.relationships.redefines.len(), 1);
    assert_eq!(usage.relationships.references.len(), 1);
    assert_eq!(usage.relationships.crosses.len(), 1);
    assert_eq!(usage.relationships.satisfies.len(), 1);
    assert_eq!(usage.relationships.performs.len(), 1);
    assert_eq!(usage.relationships.exhibits.len(), 1);
    assert_eq!(usage.relationships.includes.len(), 1);
    assert_eq!(usage.relationships.asserts.len(), 1);
    assert_eq!(usage.relationships.verifies.len(), 1);
}

#[test]
fn test_usage_with_very_long_name() {
    let long_name = "a".repeat(1000);
    let usage = Usage::new(
        UsageKind::Part,
        Some(long_name.clone()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.name, Some(long_name));
}

#[test]
fn test_usage_with_unicode_name() {
    let unicode_name = "パート_部品_🚗_Часть".to_string();
    let usage = Usage::new(
        UsageKind::Part,
        Some(unicode_name.clone()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.name, Some(unicode_name.clone()));

    let mut visitor = GenericUsageTestVisitor {
        usage_visited: false,
        usage_kind: None,
        usage_name: None,
    };

    usage.accept(&mut visitor);

    assert!(visitor.usage_visited);
    assert_eq!(visitor.usage_name, Some(unicode_name));
}

#[test]
fn test_usage_with_special_characters_in_name() {
    let special_name = "my-part_123$test".to_string();
    let usage = Usage::new(
        UsageKind::Part,
        Some(special_name.clone()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.name, Some(special_name));
}

#[test]
fn test_usage_multiple_spans() {
    let name_span = Span {
        start: crate::core::span::Position { line: 1, column: 5 },
        end: crate::core::span::Position {
            line: 1,
            column: 15,
        },
    };

    let type_span = Span {
        start: crate::core::span::Position {
            line: 1,
            column: 17,
        },
        end: crate::core::span::Position {
            line: 1,
            column: 25,
        },
    };

    let relationships = Relationships {
        typed_by: Some("MyType".to_string()),
        typed_by_span: Some(type_span),
        ..Relationships::none()
    };

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myPart".to_string()),
        relationships,
        body: vec![],
        span: Some(name_span),
        is_derived: false,
        is_readonly: false,
    };

    assert_eq!(usage.span, Some(name_span));
    assert_eq!(usage.relationships.typed_by_span, Some(type_span));
}

#[test]
fn test_usage_comparison_with_different_flags() {
    let usage1 = Usage {
        kind: UsageKind::Attribute,
        name: Some("attr".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: None,
        is_derived: true,
        is_readonly: false,
    };

    let usage2 = Usage {
        kind: UsageKind::Attribute,
        name: Some("attr".to_string()),
        relationships: Relationships::none(),
        body: vec![],
        span: None,
        is_derived: false,
        is_readonly: true,
    };

    assert_ne!(usage1, usage2, "Different flags should make usages unequal");
}

#[test]
fn test_usage_empty_name_string() {
    let usage = Usage::new(
        UsageKind::Part,
        Some(String::new()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.name, Some(String::new()));
    assert!(usage.name.as_ref().unwrap().is_empty());
}
