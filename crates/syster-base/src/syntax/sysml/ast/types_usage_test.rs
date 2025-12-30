#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::tests::CountingVisitor;
use super::*;
use crate::syntax::sysml::visitor::{AstVisitor, Visitable};

// ============================================================================
// Tests for Usage::accept with generic visitor (Issue #166)
// ============================================================================

/// Custom visitor to track visit calls with detailed information
struct DetailedUsageVisitor {
    visit_usage_called: bool,
    usage_kind: Option<UsageKind>,
    usage_name: Option<String>,
    has_typed_by: bool,
    specializes_count: usize,
    subsets_count: usize,
    redefines_count: usize,
    is_derived: bool,
    is_readonly: bool,
}

impl DetailedUsageVisitor {
    fn new() -> Self {
        Self {
            visit_usage_called: false,
            usage_kind: None,
            usage_name: None,
            has_typed_by: false,
            specializes_count: 0,
            subsets_count: 0,
            redefines_count: 0,
            is_derived: false,
            is_readonly: false,
        }
    }
}

impl AstVisitor for DetailedUsageVisitor {
    fn visit_usage(&mut self, usage: &Usage) {
        self.visit_usage_called = true;
        self.usage_kind = Some(usage.kind.clone());
        self.usage_name = usage.name.clone();
        self.has_typed_by = usage.relationships.typed_by.is_some();
        self.specializes_count = usage.relationships.specializes.len();
        self.subsets_count = usage.relationships.subsets.len();
        self.redefines_count = usage.relationships.redefines.len();
        self.is_derived = usage.is_derived;
        self.is_readonly = usage.is_readonly;
    }
}

#[test]
fn test_usage_accept_calls_visit_usage() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("testPart".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
}

#[test]
fn test_usage_accept_with_named_usage() {
    let usage = Usage {
        kind: UsageKind::Action,
        name: Some("myAction".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert_eq!(visitor.usage_kind, Some(UsageKind::Action));
    assert_eq!(visitor.usage_name, Some("myAction".to_string()));
}

#[test]
fn test_usage_accept_with_anonymous_usage() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: None,
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert_eq!(visitor.usage_name, None);
}

#[test]
fn test_usage_accept_with_all_usage_kinds() {
    let usage_kinds = vec![
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

    for kind in usage_kinds {
        let usage = Usage {
            kind: kind.clone(),
            name: Some("test".to_string()),
            body: vec![],
            relationships: Relationships::none(),
            is_derived: false,
            is_readonly: false,
            span: None,
        };

        let mut visitor = DetailedUsageVisitor::new();
        usage.accept(&mut visitor);

        assert!(visitor.visit_usage_called, "Failed for kind: {:?}", kind);
        assert_eq!(visitor.usage_kind, Some(kind));
    }
}

#[test]
fn test_usage_accept_with_typed_by_relationship() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myPart".to_string()),
        body: vec![],
        relationships: Relationships {
            typed_by: Some("PartType".to_string()),
            typed_by_span: None,
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert!(visitor.has_typed_by);
}

#[test]
fn test_usage_accept_with_subsetting_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myPart".to_string()),
        body: vec![],
        relationships: Relationships {
            subsets: vec![
                SubsettingRel {
                    target: "base1".to_string(),
                    span: None,
                },
                SubsettingRel {
                    target: "base2".to_string(),
                    span: None,
                },
            ],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert_eq!(visitor.subsets_count, 2);
}

#[test]
fn test_usage_accept_with_redefinition_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("redefined".to_string()),
        body: vec![],
        relationships: Relationships {
            redefines: vec![RedefinitionRel {
                target: "original".to_string(),
                span: None,
            }],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert_eq!(visitor.redefines_count, 1);
}

#[test]
fn test_usage_accept_with_specialization_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("specialized".to_string()),
        body: vec![],
        relationships: Relationships {
            specializes: vec![
                SpecializationRel {
                    target: "Base1".to_string(),
                    span: None,
                },
                SpecializationRel {
                    target: "Base2".to_string(),
                    span: None,
                },
                SpecializationRel {
                    target: "Base3".to_string(),
                    span: None,
                },
            ],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert_eq!(visitor.specializes_count, 3);
}

#[test]
fn test_usage_accept_with_all_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("complex".to_string()),
        body: vec![],
        relationships: Relationships {
            typed_by: Some("PartType".to_string()),
            typed_by_span: None,
            specializes: vec![SpecializationRel {
                target: "BaseType".to_string(),
                span: None,
            }],
            subsets: vec![SubsettingRel {
                target: "basePart".to_string(),
                span: None,
            }],
            redefines: vec![RedefinitionRel {
                target: "originalPart".to_string(),
                span: None,
            }],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert!(visitor.has_typed_by);
    assert_eq!(visitor.specializes_count, 1);
    assert_eq!(visitor.subsets_count, 1);
    assert_eq!(visitor.redefines_count, 1);
}

#[test]
fn test_usage_accept_with_derived_modifier() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("derivedAttr".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: true,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert!(visitor.is_derived);
    assert!(!visitor.is_readonly);
}

#[test]
fn test_usage_accept_with_readonly_modifier() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("readonlyAttr".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: true,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert!(!visitor.is_derived);
    assert!(visitor.is_readonly);
}

#[test]
fn test_usage_accept_with_both_modifiers() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("specialAttr".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: true,
        is_readonly: true,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert!(visitor.is_derived);
    assert!(visitor.is_readonly);
}

#[test]
fn test_usage_accept_with_empty_body() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("emptyPart".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
}

#[test]
fn test_usage_accept_with_nested_usage_in_body() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("parent".to_string()),
        body: vec![UsageMember::Usage(Box::new(Usage {
            kind: UsageKind::Part,
            name: Some("child".to_string()),
            body: vec![],
            relationships: Relationships::none(),
            is_derived: false,
            is_readonly: false,
            span: None,
        }))],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    // Note: Usage doesn't walk its body members in accept()
    // It only calls visit_usage on itself
    assert!(visitor.visit_usage_called);
    assert_eq!(visitor.usage_name, Some("parent".to_string()));
}

#[test]
fn test_usage_accept_with_comment_in_body() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("documented".to_string()),
        body: vec![UsageMember::Comment(Comment {
            content: "This is a comment".to_string(),
            span: None,
        })],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    // Note: Usage doesn't walk its body members in accept()
    // It only calls visit_usage on itself
    assert!(visitor.visit_usage_called);
}

#[test]
fn test_usage_accept_with_span() {
    use crate::core::Span;

    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("withSpan".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: Some(Span {
            start: crate::core::Position { line: 1, column: 1 },
            end: crate::core::Position {
                line: 1,
                column: 20,
            },
        }),
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
}

#[test]
fn test_usage_accept_multiple_times() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("reusable".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();

    // Call accept multiple times
    usage.accept(&mut visitor);
    assert!(visitor.visit_usage_called);

    // Reset and call again
    visitor.visit_usage_called = false;
    usage.accept(&mut visitor);
    assert!(visitor.visit_usage_called);
}

// ============================================================================
// Tests for Usage::accept with CountingVisitor (Issue #165)
// ============================================================================

#[test]
fn test_usage_accept_counting_visitor_single() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
    assert_eq!(visitor.packages, 0);
    assert_eq!(visitor.definitions, 0);
    assert_eq!(visitor.comments, 0);
    assert_eq!(visitor.imports, 0);
    assert_eq!(visitor.aliases, 0);
    assert_eq!(visitor.namespaces, 0);
}

#[test]
fn test_usage_accept_counting_visitor_part_usage() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("myPart".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_action_usage() {
    let usage = Usage {
        kind: UsageKind::Action,
        name: Some("myAction".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_port_usage() {
    let usage = Usage {
        kind: UsageKind::Port,
        name: Some("myPort".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_item_usage() {
    let usage = Usage {
        kind: UsageKind::Item,
        name: Some("myItem".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_attribute_usage() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("myAttr".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_requirement_usage() {
    let usage = Usage {
        kind: UsageKind::Requirement,
        name: Some("myReq".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_concern_usage() {
    let usage = Usage {
        kind: UsageKind::Concern,
        name: Some("myConcern".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_case_usage() {
    let usage = Usage {
        kind: UsageKind::Case,
        name: Some("myCase".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_view_usage() {
    let usage = Usage {
        kind: UsageKind::View,
        name: Some("myView".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_enumeration_usage() {
    let usage = Usage {
        kind: UsageKind::Enumeration,
        name: Some("myEnum".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_satisfy_requirement_usage() {
    let usage = Usage {
        kind: UsageKind::SatisfyRequirement,
        name: Some("satisfy1".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_perform_action_usage() {
    let usage = Usage {
        kind: UsageKind::PerformAction,
        name: Some("perform1".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_exhibit_state_usage() {
    let usage = Usage {
        kind: UsageKind::ExhibitState,
        name: Some("exhibit1".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_include_usecase_usage() {
    let usage = Usage {
        kind: UsageKind::IncludeUseCase,
        name: Some("include1".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_anonymous() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: None,
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_with_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("complex".to_string()),
        body: vec![],
        relationships: Relationships {
            typed_by: Some("PartType".to_string()),
            typed_by_span: None,
            subsets: vec![SubsettingRel {
                target: "base".to_string(),
                span: None,
            }],
            redefines: vec![RedefinitionRel {
                target: "original".to_string(),
                span: None,
            }],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_derived() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("derived".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: true,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_readonly() {
    let usage = Usage {
        kind: UsageKind::Attribute,
        name: Some("readonly".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: true,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_with_nested_usage() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("parent".to_string()),
        body: vec![UsageMember::Usage(Box::new(Usage {
            kind: UsageKind::Part,
            name: Some("child".to_string()),
            body: vec![],
            relationships: Relationships::none(),
            is_derived: false,
            is_readonly: false,
            span: None,
        }))],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    // Usage.accept() doesn't walk children, so only parent is counted
    assert_eq!(visitor.usages, 1);
}

#[test]
fn test_usage_accept_counting_visitor_multiple_calls() {
    let usage1 = Usage {
        kind: UsageKind::Part,
        name: Some("part1".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let usage2 = Usage {
        kind: UsageKind::Action,
        name: Some("action1".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage1.accept(&mut visitor);
    usage2.accept(&mut visitor);

    assert_eq!(visitor.usages, 2);
}

#[test]
fn test_usage_accept_counting_visitor_empty_body() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("empty".to_string()),
        body: vec![],
        relationships: Relationships::none(),
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);

    assert_eq!(visitor.usages, 1);
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_usage_accept_with_all_domain_specific_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("domainSpecific".to_string()),
        body: vec![],
        relationships: Relationships {
            satisfies: vec![SatisfyRel {
                target: "requirement1".to_string(),
                span: None,
            }],
            performs: vec![PerformRel {
                target: "action1".to_string(),
                span: None,
            }],
            exhibits: vec![ExhibitRel {
                target: "state1".to_string(),
                span: None,
            }],
            includes: vec![IncludeRel {
                target: "usecase1".to_string(),
                span: None,
            }],
            asserts: vec![AssertRel {
                target: "constraint1".to_string(),
                span: None,
            }],
            verifies: vec![VerifyRel {
                target: "verification1".to_string(),
                span: None,
            }],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
}

#[test]
fn test_usage_accept_with_reference_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("withRefs".to_string()),
        body: vec![],
        relationships: Relationships {
            references: vec![
                ReferenceRel {
                    target: "ref1".to_string(),
                    span: None,
                },
                ReferenceRel {
                    target: "ref2".to_string(),
                    span: None,
                },
            ],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
}

#[test]
fn test_usage_accept_with_crosses_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("withCrosses".to_string()),
        body: vec![],
        relationships: Relationships {
            crosses: vec![CrossRel {
                target: "boundary".to_string(),
                span: None,
            }],
            ..Relationships::none()
        },
        is_derived: false,
        is_readonly: false,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
}

#[test]
fn test_usage_accept_with_maximum_relationships() {
    let usage = Usage {
        kind: UsageKind::Part,
        name: Some("maximal".to_string()),
        body: vec![],
        relationships: Relationships {
            typed_by: Some("Type".to_string()),
            typed_by_span: None,
            specializes: vec![
                SpecializationRel {
                    target: "S1".to_string(),
                    span: None,
                },
                SpecializationRel {
                    target: "S2".to_string(),
                    span: None,
                },
            ],
            subsets: vec![
                SubsettingRel {
                    target: "Sub1".to_string(),
                    span: None,
                },
                SubsettingRel {
                    target: "Sub2".to_string(),
                    span: None,
                },
            ],
            redefines: vec![
                RedefinitionRel {
                    target: "Redef1".to_string(),
                    span: None,
                },
                RedefinitionRel {
                    target: "Redef2".to_string(),
                    span: None,
                },
            ],
            references: vec![ReferenceRel {
                target: "Ref1".to_string(),
                span: None,
            }],
            crosses: vec![CrossRel {
                target: "Cross1".to_string(),
                span: None,
            }],
            satisfies: vec![SatisfyRel {
                target: "Sat1".to_string(),
                span: None,
            }],
            performs: vec![PerformRel {
                target: "Perf1".to_string(),
                span: None,
            }],
            exhibits: vec![ExhibitRel {
                target: "Exh1".to_string(),
                span: None,
            }],
            includes: vec![IncludeRel {
                target: "Inc1".to_string(),
                span: None,
            }],
            asserts: vec![AssertRel {
                target: "Ass1".to_string(),
                span: None,
            }],
            verifies: vec![VerifyRel {
                target: "Ver1".to_string(),
                span: None,
            }],
        },
        is_derived: true,
        is_readonly: true,
        span: None,
    };

    let mut visitor = DetailedUsageVisitor::new();
    usage.accept(&mut visitor);

    assert!(visitor.visit_usage_called);
    assert!(visitor.has_typed_by);
    assert_eq!(visitor.specializes_count, 2);
    assert_eq!(visitor.subsets_count, 2);
    assert_eq!(visitor.redefines_count, 2);
    assert!(visitor.is_derived);
    assert!(visitor.is_readonly);
}

#[test]
fn test_usage_new_constructor() {
    let usage = Usage::new(
        UsageKind::Part,
        Some("constructedUsage".to_string()),
        Relationships::none(),
        vec![],
    );

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("constructedUsage".to_string()));
    assert!(!usage.is_derived);
    assert!(!usage.is_readonly);
    assert!(usage.span.is_none());

    let mut visitor = CountingVisitor::new();
    usage.accept(&mut visitor);
    assert_eq!(visitor.usages, 1);
}
