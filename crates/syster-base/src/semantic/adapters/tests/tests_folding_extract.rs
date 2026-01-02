//! Comprehensive tests for folding range extraction
//!
//! This test module covers `extract_folding_ranges` function for both KerML and SysML,
//! testing various element nesting scenarios, edge cases, and error conditions.

use crate::core::{Position, Span};
use crate::semantic::adapters::folding_ranges::{
    extract_kerml_folding_ranges, extract_sysml_folding_ranges,
};
use crate::syntax::kerml::ast::{
    Classifier, ClassifierKind, ClassifierMember, Comment as KerMLComment, Element as KerMLElement,
    Feature, FeatureMember, KerMLFile, Package as KerMLPackage,
};
use crate::syntax::sysml::ast::{
    Comment as SysMLComment, Definition, DefinitionKind, DefinitionMember, Element as SysMLElement,
    Package as SysMLPackage, Relationships, SysMLFile, Usage, UsageKind, UsageMember,
};

/// Helper to create a span from line numbers
fn make_span(start_line: usize, end_line: usize) -> Span {
    Span {
        start: Position {
            line: start_line,
            column: 0,
        },
        end: Position {
            line: end_line,
            column: 1,
        },
    }
}

/// Helper to create a Definition with a span
fn make_definition(
    kind: DefinitionKind,
    name: Option<String>,
    body: Vec<DefinitionMember>,
    span: Span,
) -> Definition {
    let mut def = Definition::new(kind, name, Relationships::default(), body);
    def.span = Some(span);
    def
}

/// Helper to create a Usage with a span
fn make_usage(kind: UsageKind, name: Option<String>, body: Vec<UsageMember>, span: Span) -> Usage {
    let mut usage = Usage::new(kind, name, Relationships::default(), body);
    usage.span = Some(span);
    usage
}

// =============================================================================
// KerML Tests - extract_folding_ranges
// =============================================================================

#[test]
fn test_kerml_nested_packages() {
    // Test nested packages: Package -> Package -> Package
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Outer".to_string()),
            span: Some(make_span(1, 10)),
            elements: vec![KerMLElement::Package(KerMLPackage {
                name: Some("Inner".to_string()),
                span: Some(make_span(2, 9)),
                elements: vec![KerMLElement::Package(KerMLPackage {
                    name: Some("DeepInner".to_string()),
                    span: Some(make_span(3, 8)),
                    elements: vec![],
                })],
            })],
        })],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "Should have 3 foldable packages");
    assert!(!ranges[0].is_comment);
    assert!(!ranges[1].is_comment);
    assert!(!ranges[2].is_comment);

    // Verify sorting by start line
    assert!(ranges[0].span.start.line == 1);
    assert!(ranges[1].span.start.line == 2);
    assert!(ranges[2].span.start.line == 3);
}

#[test]
fn test_kerml_classifier_in_package() {
    // Test package containing classifier
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("MyPackage".to_string()),
            span: Some(make_span(1, 10)),
            elements: vec![KerMLElement::Classifier(Classifier {
                kind: ClassifierKind::Class,
                is_abstract: false,
                name: Some("MyClass".to_string()),
                span: Some(make_span(2, 9)),
                body: vec![],
            })],
        })],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2, "Should have package and classifier");
    assert_eq!(ranges[0].span.start.line, 1);
    assert_eq!(ranges[1].span.start.line, 2);
}

#[test]
fn test_kerml_feature_in_classifier() {
    // Test classifier containing features
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Classifier(Classifier {
            kind: ClassifierKind::Class,
            is_abstract: false,
            name: Some("MyClass".to_string()),
            span: Some(make_span(1, 10)),
            body: vec![
                ClassifierMember::Feature(Feature {
                    name: Some("feature1".to_string()),
                    direction: None,
                    is_readonly: false,
                    is_derived: false,
                    span: Some(make_span(2, 5)),
                    body: vec![],
                }),
                ClassifierMember::Feature(Feature {
                    name: Some("feature2".to_string()),
                    direction: None,
                    is_readonly: false,
                    is_derived: false,
                    span: Some(make_span(6, 9)),
                    body: vec![],
                }),
            ],
        })],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "Should have classifier and 2 features");
}

#[test]
fn test_kerml_comments_in_classifier() {
    // Test classifier with comment members
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Classifier(Classifier {
            kind: ClassifierKind::Class,
            is_abstract: false,
            name: Some("MyClass".to_string()),
            span: Some(make_span(1, 10)),
            body: vec![ClassifierMember::Comment(KerMLComment {
                content: "Comment in class".to_string(),
                about: vec![],
                locale: None,
                span: Some(make_span(2, 4)),
            })],
        })],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2, "Should have classifier and comment");
    assert!(!ranges[0].is_comment, "First range is classifier");
    assert!(ranges[1].is_comment, "Second range is comment");
}

#[test]
fn test_kerml_comments_in_feature() {
    // Test feature with comment members
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Feature(Feature {
            name: Some("myFeature".to_string()),
            direction: None,
            is_readonly: false,
            is_derived: false,
            span: Some(make_span(1, 10)),
            body: vec![FeatureMember::Comment(KerMLComment {
                content: "Comment in feature".to_string(),
                about: vec![],
                locale: None,
                span: Some(make_span(2, 4)),
            })],
        })],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2, "Should have feature and comment");
    assert!(!ranges[0].is_comment, "First range is feature");
    assert!(ranges[1].is_comment, "Second range is comment");
}

#[test]
fn test_kerml_elements_without_spans() {
    // Test that elements without spans are ignored
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Package(KerMLPackage {
                name: Some("NoSpan".to_string()),
                span: None, // No span
                elements: vec![],
            }),
            KerMLElement::Classifier(Classifier {
                kind: ClassifierKind::Class,
                is_abstract: false,
                name: Some("WithSpan".to_string()),
                span: Some(make_span(5, 10)),
                body: vec![],
            }),
        ],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1, "Should only include element with span");
    assert_eq!(ranges[0].span.start.line, 5);
}

#[test]
fn test_kerml_import_and_annotation_ignored() {
    // Test that Import and Annotation elements are properly ignored
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Import(crate::syntax::kerml::ast::Import {
                path: "Some::Path".to_string(),
                path_span: None,
                is_recursive: false,
                kind: crate::syntax::kerml::ast::ImportKind::Normal,
                span: Some(make_span(1, 1)),
            }),
            KerMLElement::Annotation(crate::syntax::kerml::ast::Annotation {
                reference: "SomeAnnotation".to_string(),
                span: Some(make_span(2, 2)),
            }),
            KerMLElement::Package(KerMLPackage {
                name: Some("Package".to_string()),
                span: Some(make_span(3, 5)),
                elements: vec![],
            }),
        ],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1, "Import and Annotation should be ignored");
    assert_eq!(ranges[0].span.start.line, 3);
}

#[test]
fn test_kerml_mixed_nesting_with_comments() {
    // Test complex nesting: Package -> Classifier -> Feature with comments at each level
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Comment(KerMLComment {
                content: "Top-level comment".to_string(),
                about: vec![],
                locale: None,
                span: Some(make_span(1, 3)),
            }),
            KerMLElement::Package(KerMLPackage {
                name: Some("Pkg".to_string()),
                span: Some(make_span(4, 20)),
                elements: vec![
                    KerMLElement::Comment(KerMLComment {
                        content: "Package comment".to_string(),
                        about: vec![],
                        locale: None,
                        span: Some(make_span(5, 7)),
                    }),
                    KerMLElement::Classifier(Classifier {
                        kind: ClassifierKind::Class,
                        is_abstract: false,
                        name: Some("MyClass".to_string()),
                        span: Some(make_span(8, 19)),
                        body: vec![
                            ClassifierMember::Comment(KerMLComment {
                                content: "Classifier comment".to_string(),
                                about: vec![],
                                locale: None,
                                span: Some(make_span(9, 11)),
                            }),
                            ClassifierMember::Feature(Feature {
                                name: Some("feat".to_string()),
                                direction: None,
                                is_readonly: false,
                                is_derived: false,
                                span: Some(make_span(12, 18)),
                                body: vec![FeatureMember::Comment(KerMLComment {
                                    content: "Feature comment".to_string(),
                                    about: vec![],
                                    locale: None,
                                    span: Some(make_span(13, 15)),
                                })],
                            }),
                        ],
                    }),
                ],
            }),
        ],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 7, "Should extract all foldable elements");

    // Verify all comments are marked correctly
    let comment_count = ranges.iter().filter(|r| r.is_comment).count();
    assert_eq!(comment_count, 4, "Should have 4 comments");

    // Verify sorting by start line
    for i in 1..ranges.len() {
        assert!(
            ranges[i - 1].span.start.line <= ranges[i].span.start.line,
            "Ranges should be sorted by start line"
        );
    }
}

#[test]
fn test_kerml_empty_bodies() {
    // Test elements with empty bodies
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Package(KerMLPackage {
                name: Some("Empty".to_string()),
                span: Some(make_span(1, 3)),
                elements: vec![],
            }),
            KerMLElement::Classifier(Classifier {
                kind: ClassifierKind::Class,
                is_abstract: false,
                name: Some("EmptyClass".to_string()),
                span: Some(make_span(4, 6)),
                body: vec![],
            }),
            KerMLElement::Feature(Feature {
                name: Some("emptyFeature".to_string()),
                direction: None,
                is_readonly: false,
                is_derived: false,
                span: Some(make_span(7, 9)),
                body: vec![],
            }),
        ],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "Empty bodies should still be foldable");
}

#[test]
fn test_kerml_multiple_comments_in_sequence() {
    // Test multiple comments in sequence
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Comment(KerMLComment {
                content: "Comment 1".to_string(),
                about: vec![],
                locale: None,
                span: Some(make_span(1, 3)),
            }),
            KerMLElement::Comment(KerMLComment {
                content: "Comment 2".to_string(),
                about: vec![],
                locale: None,
                span: Some(make_span(4, 6)),
            }),
            KerMLElement::Comment(KerMLComment {
                content: "Comment 3".to_string(),
                about: vec![],
                locale: None,
                span: Some(make_span(7, 9)),
            }),
        ],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "All comments should be extracted");
    assert!(
        ranges.iter().all(|r| r.is_comment),
        "All should be comments"
    );
}

#[test]
fn test_kerml_deep_nesting() {
    // Test deep nesting (4 levels)
    let file = KerMLFile {
        namespace: None,
        elements: vec![KerMLElement::Package(KerMLPackage {
            name: Some("Level1".to_string()),
            span: Some(make_span(1, 20)),
            elements: vec![KerMLElement::Package(KerMLPackage {
                name: Some("Level2".to_string()),
                span: Some(make_span(2, 19)),
                elements: vec![KerMLElement::Classifier(Classifier {
                    kind: ClassifierKind::Class,
                    is_abstract: false,
                    name: Some("Level3".to_string()),
                    span: Some(make_span(3, 18)),
                    body: vec![ClassifierMember::Feature(Feature {
                        name: Some("Level4".to_string()),
                        direction: None,
                        is_readonly: false,
                        is_derived: false,
                        span: Some(make_span(4, 17)),
                        body: vec![],
                    })],
                })],
            })],
        })],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 4, "Should extract 4 levels of nesting");
    assert_eq!(ranges[0].span.start.line, 1);
    assert_eq!(ranges[1].span.start.line, 2);
    assert_eq!(ranges[2].span.start.line, 3);
    assert_eq!(ranges[3].span.start.line, 4);
}

// =============================================================================
// SysML Tests - extract_folding_ranges
// =============================================================================

#[test]
fn test_sysml_nested_packages() {
    // Test nested packages: Package -> Package -> Package
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Outer".to_string()),
            span: Some(make_span(1, 10)),
            elements: vec![SysMLElement::Package(SysMLPackage {
                name: Some("Inner".to_string()),
                span: Some(make_span(2, 9)),
                elements: vec![SysMLElement::Package(SysMLPackage {
                    name: Some("DeepInner".to_string()),
                    span: Some(make_span(3, 8)),
                    elements: vec![],
                })],
            })],
        })],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "Should have 3 foldable packages");
    assert!(!ranges[0].is_comment);
    assert!(!ranges[1].is_comment);
    assert!(!ranges[2].is_comment);

    // Verify sorting by start line
    assert!(ranges[0].span.start.line == 1);
    assert!(ranges[1].span.start.line == 2);
    assert!(ranges[2].span.start.line == 3);
}

#[test]
fn test_sysml_definition_in_package() {
    // Test package containing definition
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("MyPackage".to_string()),
            span: Some(make_span(1, 10)),
            elements: vec![SysMLElement::Definition(make_definition(
                DefinitionKind::Part,
                Some("MyPart".to_string()),
                vec![],
                make_span(2, 9),
            ))],
        })],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2, "Should have package and definition");
    assert_eq!(ranges[0].span.start.line, 1);
    assert_eq!(ranges[1].span.start.line, 2);
}

#[test]
fn test_sysml_usage_in_definition() {
    // Test definition containing usages
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Definition(make_definition(
            DefinitionKind::Part,
            Some("MyPart".to_string()),
            vec![
                DefinitionMember::Usage(Box::new(make_usage(
                    UsageKind::Part,
                    Some("usage1".to_string()),
                    vec![],
                    make_span(2, 5),
                ))),
                DefinitionMember::Usage(Box::new(make_usage(
                    UsageKind::Part,
                    Some("usage2".to_string()),
                    vec![],
                    make_span(6, 9),
                ))),
            ],
            make_span(1, 10),
        ))],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "Should have definition and 2 usages");
}

#[test]
fn test_sysml_nested_usages() {
    // Test usage containing nested usages
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Usage(make_usage(
            UsageKind::Part,
            Some("outer".to_string()),
            vec![UsageMember::Usage(Box::new(make_usage(
                UsageKind::Part,
                Some("inner".to_string()),
                vec![],
                make_span(2, 5),
            )))],
            make_span(1, 10),
        ))],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2, "Should have outer and inner usage");
}

#[test]
fn test_sysml_comments_in_definition() {
    // Test definition with comment members
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Definition(make_definition(
            DefinitionKind::Part,
            Some("MyPart".to_string()),
            vec![DefinitionMember::Comment(Box::new(SysMLComment {
                content: "Comment in definition".to_string(),
                span: Some(make_span(2, 4)),
            }))],
            make_span(1, 10),
        ))],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2, "Should have definition and comment");
    assert!(!ranges[0].is_comment, "First range is definition");
    assert!(ranges[1].is_comment, "Second range is comment");
}

#[test]
fn test_sysml_comments_in_usage() {
    // Test usage with comment members
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Usage(make_usage(
            UsageKind::Part,
            Some("myUsage".to_string()),
            vec![UsageMember::Comment(SysMLComment {
                content: "Comment in usage".to_string(),
                span: Some(make_span(2, 4)),
            })],
            make_span(1, 10),
        ))],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 2, "Should have usage and comment");
    assert!(!ranges[0].is_comment, "First range is usage");
    assert!(ranges[1].is_comment, "Second range is comment");
}

#[test]
fn test_sysml_elements_without_spans() {
    // Test that elements without spans are ignored
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Package(SysMLPackage {
                name: Some("NoSpan".to_string()),
                span: None, // No span
                elements: vec![],
            }),
            SysMLElement::Definition(make_definition(
                DefinitionKind::Part,
                Some("WithSpan".to_string()),
                vec![],
                make_span(5, 10),
            )),
        ],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1, "Should only include element with span");
    assert_eq!(ranges[0].span.start.line, 5);
}

#[test]
fn test_sysml_import_and_alias_ignored() {
    // Test that Import and Alias elements are properly ignored
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Import(crate::syntax::sysml::ast::Import {
                path: "Some::Path".to_string(),
                path_span: None,
                is_recursive: false,
                span: Some(make_span(1, 1)),
            }),
            SysMLElement::Alias(crate::syntax::sysml::ast::Alias {
                name: Some("MyAlias".to_string()),
                target: "Target".to_string(),
                target_span: None,
                span: Some(make_span(2, 2)),
            }),
            SysMLElement::Package(SysMLPackage {
                name: Some("Package".to_string()),
                span: Some(make_span(3, 5)),
                elements: vec![],
            }),
        ],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 1, "Import and Alias should be ignored");
    assert_eq!(ranges[0].span.start.line, 3);
}

#[test]
fn test_sysml_mixed_nesting_with_comments() {
    // Test complex nesting: Package -> Definition -> Usage with comments at each level
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Comment(SysMLComment {
                content: "Top-level comment".to_string(),
                span: Some(make_span(1, 3)),
            }),
            SysMLElement::Package(SysMLPackage {
                name: Some("Pkg".to_string()),
                span: Some(make_span(4, 20)),
                elements: vec![
                    SysMLElement::Comment(SysMLComment {
                        content: "Package comment".to_string(),
                        span: Some(make_span(5, 7)),
                    }),
                    SysMLElement::Definition(make_definition(
                        DefinitionKind::Part,
                        Some("MyPart".to_string()),
                        vec![
                            DefinitionMember::Comment(Box::new(SysMLComment {
                                content: "Definition comment".to_string(),
                                span: Some(make_span(9, 11)),
                            })),
                            DefinitionMember::Usage(Box::new(make_usage(
                                UsageKind::Part,
                                Some("usage".to_string()),
                                vec![UsageMember::Comment(SysMLComment {
                                    content: "Usage comment".to_string(),
                                    span: Some(make_span(13, 15)),
                                })],
                                make_span(12, 18),
                            ))),
                        ],
                        make_span(8, 19),
                    )),
                ],
            }),
        ],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 7, "Should extract all foldable elements");

    // Verify all comments are marked correctly
    let comment_count = ranges.iter().filter(|r| r.is_comment).count();
    assert_eq!(comment_count, 4, "Should have 4 comments");

    // Verify sorting by start line
    for i in 1..ranges.len() {
        assert!(
            ranges[i - 1].span.start.line <= ranges[i].span.start.line,
            "Ranges should be sorted by start line"
        );
    }
}

#[test]
fn test_sysml_empty_bodies() {
    // Test elements with empty bodies
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Package(SysMLPackage {
                name: Some("Empty".to_string()),
                span: Some(make_span(1, 3)),
                elements: vec![],
            }),
            SysMLElement::Definition(make_definition(
                DefinitionKind::Part,
                Some("EmptyPart".to_string()),
                vec![],
                make_span(4, 6),
            )),
            SysMLElement::Usage(make_usage(
                UsageKind::Part,
                Some("emptyUsage".to_string()),
                vec![],
                make_span(7, 9),
            )),
        ],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "Empty bodies should still be foldable");
}

#[test]
fn test_sysml_multiple_comments_in_sequence() {
    // Test multiple comments in sequence
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Comment(SysMLComment {
                content: "Comment 1".to_string(),
                span: Some(make_span(1, 3)),
            }),
            SysMLElement::Comment(SysMLComment {
                content: "Comment 2".to_string(),
                span: Some(make_span(4, 6)),
            }),
            SysMLElement::Comment(SysMLComment {
                content: "Comment 3".to_string(),
                span: Some(make_span(7, 9)),
            }),
        ],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3, "All comments should be extracted");
    assert!(
        ranges.iter().all(|r| r.is_comment),
        "All should be comments"
    );
}

#[test]
fn test_sysml_deep_nesting() {
    // Test deep nesting (4 levels)
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![SysMLElement::Package(SysMLPackage {
            name: Some("Level1".to_string()),
            span: Some(make_span(1, 20)),
            elements: vec![SysMLElement::Package(SysMLPackage {
                name: Some("Level2".to_string()),
                span: Some(make_span(2, 19)),
                elements: vec![SysMLElement::Definition(make_definition(
                    DefinitionKind::Part,
                    Some("Level3".to_string()),
                    vec![DefinitionMember::Usage(Box::new(make_usage(
                        UsageKind::Part,
                        Some("Level4".to_string()),
                        vec![],
                        make_span(4, 17),
                    )))],
                    make_span(3, 18),
                ))],
            })],
        })],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 4, "Should extract 4 levels of nesting");
    assert_eq!(ranges[0].span.start.line, 1);
    assert_eq!(ranges[1].span.start.line, 2);
    assert_eq!(ranges[2].span.start.line, 3);
    assert_eq!(ranges[3].span.start.line, 4);
}

// =============================================================================
// Edge Cases for Both Languages
// =============================================================================

#[test]
fn test_kerml_single_line_elements_filtered() {
    // Verify that single-line elements are filtered out
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Package(KerMLPackage {
                name: Some("OneLine".to_string()),
                span: Some(make_span(5, 5)), // Same start and end line
                elements: vec![],
            }),
            KerMLElement::Comment(KerMLComment {
                content: "Single line".to_string(),
                about: vec![],
                locale: None,
                span: Some(make_span(10, 10)), // Same start and end line
            }),
        ],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 0, "Single-line elements should be filtered");
}

#[test]
fn test_sysml_single_line_elements_filtered() {
    // Verify that single-line elements are filtered out
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Package(SysMLPackage {
                name: Some("OneLine".to_string()),
                span: Some(make_span(5, 5)), // Same start and end line
                elements: vec![],
            }),
            SysMLElement::Comment(SysMLComment {
                content: "Single line".to_string(),
                span: Some(make_span(10, 10)), // Same start and end line
            }),
        ],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 0, "Single-line elements should be filtered");
}

#[test]
fn test_kerml_unsorted_input_produces_sorted_output() {
    // Verify that unsorted input is sorted by start line
    let file = KerMLFile {
        namespace: None,
        elements: vec![
            KerMLElement::Package(KerMLPackage {
                name: Some("Last".to_string()),
                span: Some(make_span(20, 25)),
                elements: vec![],
            }),
            KerMLElement::Package(KerMLPackage {
                name: Some("First".to_string()),
                span: Some(make_span(1, 5)),
                elements: vec![],
            }),
            KerMLElement::Package(KerMLPackage {
                name: Some("Middle".to_string()),
                span: Some(make_span(10, 15)),
                elements: vec![],
            }),
        ],
    };

    let ranges = extract_kerml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3);
    assert_eq!(ranges[0].span.start.line, 1, "Should be sorted");
    assert_eq!(ranges[1].span.start.line, 10, "Should be sorted");
    assert_eq!(ranges[2].span.start.line, 20, "Should be sorted");
}

#[test]
fn test_sysml_unsorted_input_produces_sorted_output() {
    // Verify that unsorted input is sorted by start line
    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            SysMLElement::Package(SysMLPackage {
                name: Some("Last".to_string()),
                span: Some(make_span(20, 25)),
                elements: vec![],
            }),
            SysMLElement::Package(SysMLPackage {
                name: Some("First".to_string()),
                span: Some(make_span(1, 5)),
                elements: vec![],
            }),
            SysMLElement::Package(SysMLPackage {
                name: Some("Middle".to_string()),
                span: Some(make_span(10, 15)),
                elements: vec![],
            }),
        ],
    };

    let ranges = extract_sysml_folding_ranges(&file);
    assert_eq!(ranges.len(), 3);
    assert_eq!(ranges[0].span.start.line, 1, "Should be sorted");
    assert_eq!(ranges[1].span.start.line, 10, "Should be sorted");
    assert_eq!(ranges[2].span.start.line, 20, "Should be sorted");
}
