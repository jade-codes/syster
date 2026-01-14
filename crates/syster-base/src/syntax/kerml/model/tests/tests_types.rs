#![allow(clippy::unwrap_used)]

use super::super::*;

// Helper function to create a basic LiteralExpression for testing
fn create_literal_expression() -> LiteralExpression {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let feature = Feature {
        type_,
        is_nonunique: false,
        is_ordered: false,
        direction: None,
        is_composite: None,
        is_derived: None,
        is_end: None,
        is_portion: None,
        is_const: None,
        value: None,
        write: None,
        crossing_feature: None,
    };
    let step = Step { feature };
    let expression = Expression { step, result: None };
    LiteralExpression { expression }
}

// Helper function to create a LiteralExpression with a specific name
fn create_literal_expression_with_name(name: &str) -> LiteralExpression {
    let element = Element {
        declared_name: Some(name.to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let feature = Feature {
        type_,
        is_nonunique: false,
        is_ordered: false,
        direction: None,
        is_composite: None,
        is_derived: None,
        is_end: None,
        is_portion: None,
        is_const: None,
        value: None,
        write: None,
        crossing_feature: None,
    };
    let step = Step { feature };
    let expression = Expression { step, result: None };
    LiteralExpression { expression }
}

// ============================================================================
// Tests for Element
// ============================================================================

#[test]
fn test_element_new_empty() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    assert_eq!(element.declared_name, None);
    assert_eq!(element.declared_short_name, None);
}

#[test]
fn test_element_with_name() {
    let element = Element {
        declared_name: Some("MyElement".to_string()),
        declared_short_name: None,
    };
    assert_eq!(element.declared_name, Some("MyElement".to_string()));
}

#[test]
fn test_element_with_short_name() {
    let element = Element {
        declared_name: None,
        declared_short_name: Some("ME".to_string()),
    };
    assert_eq!(element.declared_short_name, Some("ME".to_string()));
}

#[test]
fn test_element_with_both_names() {
    let element = Element {
        declared_name: Some("MyElement".to_string()),
        declared_short_name: Some("ME".to_string()),
    };
    assert_eq!(element.declared_name, Some("MyElement".to_string()));
    assert_eq!(element.declared_short_name, Some("ME".to_string()));
}

#[test]
fn test_element_clone() {
    let element1 = Element {
        declared_name: Some("Original".to_string()),
        declared_short_name: None,
    };
    let element2 = element1.clone();
    assert_eq!(element1, element2);
}

#[test]
fn test_element_partial_eq() {
    let element1 = Element {
        declared_name: Some("Same".to_string()),
        declared_short_name: None,
    };
    let element2 = Element {
        declared_name: Some("Same".to_string()),
        declared_short_name: None,
    };
    assert_eq!(element1, element2);
}

#[test]
fn test_element_not_equal_different_name() {
    let element1 = Element {
        declared_name: Some("First".to_string()),
        declared_short_name: None,
    };
    let element2 = Element {
        declared_name: Some("Second".to_string()),
        declared_short_name: None,
    };
    assert_ne!(element1, element2);
}

// ============================================================================
// Tests for Membership and is_alias field (KEY TESTS FOR types-alias)
// ============================================================================

#[test]
fn test_membership_is_alias_false() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: false,
    };
    assert!(!membership.is_alias, "is_alias should be false");
}

#[test]
fn test_membership_is_alias_true() {
    let element = Element {
        declared_name: Some("AliasedElement".to_string()),
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: true,
    };
    assert!(membership.is_alias, "is_alias should be true");
}

#[test]
fn test_membership_alias_toggle() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };

    let membership_not_alias = Membership {
        relationship: relationship.clone(),
        is_alias: false,
    };
    let membership_is_alias = Membership {
        relationship,
        is_alias: true,
    };

    assert_ne!(
        membership_not_alias, membership_is_alias,
        "Memberships with different is_alias values should not be equal"
    );
}

#[test]
fn test_membership_clone_preserves_alias() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: true,
    };

    let cloned = membership.clone();
    assert!(
        cloned.is_alias,
        "Cloned membership should preserve is_alias value"
    );
    assert_eq!(
        membership, cloned,
        "Cloned membership should equal original"
    );
}

// ============================================================================
// Tests for OwningMembership (wraps Membership)
// ============================================================================

#[test]
fn test_owning_membership_with_alias() {
    let element = Element {
        declared_name: Some("OwnedElement".to_string()),
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: true,
    };
    let owning = OwningMembership { membership };

    assert!(
        owning.membership.is_alias,
        "OwningMembership should preserve is_alias"
    );
}

#[test]
fn test_owning_membership_without_alias() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: false,
    };
    let owning = OwningMembership { membership };

    assert!(
        !owning.membership.is_alias,
        "OwningMembership should preserve is_alias as false"
    );
}

// ============================================================================
// Tests for Namespace
// ============================================================================

#[test]
fn test_namespace_empty() {
    let element = Element {
        declared_name: Some("EmptyNamespace".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    assert_eq!(namespace.prefixes.len(), 0);
    assert_eq!(namespace.children.len(), 0);
}

#[test]
fn test_namespace_with_children() {
    let element = Element {
        declared_name: Some("ParentNamespace".to_string()),
        declared_short_name: None,
    };

    let child_element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let child_relationship = Relationship {
        element: child_element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let child_membership = Membership {
        relationship: child_relationship,
        is_alias: false,
    };

    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![NamespaceChild::Membership(Box::new(child_membership))],
    };

    assert_eq!(namespace.children.len(), 1);
}

// ============================================================================
// Tests for Type
// ============================================================================

#[test]
fn test_type_basic() {
    let element = Element {
        declared_name: Some("MyType".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };

    assert!(!type_.is_sufficient);
    assert_eq!(type_.is_abstract, None);
}

#[test]
fn test_type_abstract() {
    let element = Element {
        declared_name: Some("AbstractType".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: Some(AbstractMarker::Abstract),
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };

    assert_eq!(type_.is_abstract, Some(AbstractMarker::Abstract));
}

#[test]
fn test_type_sufficient() {
    let element = Element {
        declared_name: Some("SufficientType".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: true,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };

    assert!(type_.is_sufficient);
}

// ============================================================================
// Tests for Feature
// ============================================================================

#[test]
fn test_feature_basic() {
    let element = Element {
        declared_name: Some("MyFeature".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let feature = Feature {
        type_,
        is_nonunique: false,
        is_ordered: false,
        direction: None,
        is_composite: None,
        is_derived: None,
        is_end: None,
        is_portion: None,
        is_const: None,
        value: None,
        write: None,
        crossing_feature: None,
    };

    assert!(!feature.is_nonunique);
    assert!(!feature.is_ordered);
}

#[test]
fn test_feature_ordered() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let feature = Feature {
        type_,
        is_nonunique: false,
        is_ordered: true,
        direction: None,
        is_composite: None,
        is_derived: None,
        is_end: None,
        is_portion: None,
        is_const: None,
        value: None,
        write: None,
        crossing_feature: None,
    };

    assert!(feature.is_ordered);
}

#[test]
fn test_feature_composite() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let feature = Feature {
        type_,
        is_nonunique: false,
        is_ordered: false,
        direction: None,
        is_composite: Some(CompositeMarker::Composite),
        is_derived: None,
        is_end: None,
        is_portion: None,
        is_const: None,
        value: None,
        write: None,
        crossing_feature: None,
    };

    assert_eq!(feature.is_composite, Some(CompositeMarker::Composite));
}

#[test]
fn test_feature_readonly() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let feature = Feature {
        type_,
        is_nonunique: false,
        is_ordered: false,
        direction: None,
        is_composite: None,
        is_derived: None,
        is_end: None,
        is_portion: None,
        is_const: Some(ReadonlyMarker::Readonly),
        value: None,
        write: None,
        crossing_feature: None,
    };

    assert_eq!(feature.is_const, Some(ReadonlyMarker::Readonly));
}

#[test]
fn test_feature_derived() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let feature = Feature {
        type_,
        is_nonunique: false,
        is_ordered: false,
        direction: None,
        is_composite: None,
        is_derived: Some(DerivedMarker::Derived),
        is_end: None,
        is_portion: None,
        is_const: None,
        value: None,
        write: None,
        crossing_feature: None,
    };

    assert_eq!(feature.is_derived, Some(DerivedMarker::Derived));
}

// ============================================================================
// Tests for Relationship
// ============================================================================

#[test]
fn test_relationship_basic() {
    let element = Element {
        declared_name: Some("MyRelationship".to_string()),
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };

    assert_eq!(relationship.visibility, None);
    assert_eq!(relationship.elements.len(), 0);
}

#[test]
fn test_relationship_with_visibility() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: Some(VisibilityKind::Public),
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };

    assert_eq!(relationship.visibility, Some(VisibilityKind::Public));
}

// ============================================================================
// Tests for Import
// ============================================================================

#[test]
fn test_import_basic() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let import = Import {
        relationship,
        imports_all: false,
        is_recursive: false,
        is_namespace: None,
    };

    assert!(!import.imports_all);
    assert!(!import.is_recursive);
}

#[test]
fn test_import_all() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let import = Import {
        relationship,
        imports_all: true,
        is_recursive: false,
        is_namespace: None,
    };

    assert!(import.imports_all);
}

#[test]
fn test_import_recursive() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let import = Import {
        relationship,
        imports_all: true,
        is_recursive: true,
        is_namespace: None,
    };

    assert!(import.is_recursive);
}

#[test]
fn test_import_namespace() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let import = Import {
        relationship,
        imports_all: true,
        is_recursive: false,
        is_namespace: Some(NamespaceMarker::Namespace),
    };

    assert_eq!(import.is_namespace, Some(NamespaceMarker::Namespace));
}

// ============================================================================
// Tests for Literal Expressions
// ============================================================================

#[test]
fn test_literal_boolean_true() {
    let literal_expression = create_literal_expression();
    let literal_bool = LiteralBoolean {
        literal_expression,
        literal: true,
    };

    assert!(literal_bool.literal);
}

#[test]
fn test_literal_boolean_false() {
    let literal_expression = create_literal_expression();
    let literal_bool = LiteralBoolean {
        literal_expression,
        literal: false,
    };

    assert!(!literal_bool.literal);
}

#[test]
fn test_literal_string() {
    let literal_expression = create_literal_expression();
    let literal_str = LiteralString {
        literal_expression,
        literal: "Hello, World!".to_string(),
    };

    assert_eq!(literal_str.literal, "Hello, World!");
}

#[test]
fn test_literal_number_positive() {
    let literal_expression = create_literal_expression();
    let literal_num = LiteralNumber {
        literal_expression,
        literal: 42.0,
    };

    assert_eq!(literal_num.literal, 42.0);
}

#[test]
fn test_literal_number_negative() {
    let literal_expression = create_literal_expression();
    let literal_num = LiteralNumber {
        literal_expression,
        literal: -25.5,
    };

    assert_eq!(literal_num.literal, -25.5);
}

#[test]
fn test_literal_number_zero() {
    let literal_expression = create_literal_expression();
    let literal_num = LiteralNumber {
        literal_expression,
        literal: 0.0,
    };

    assert_eq!(literal_num.literal, 0.0);
}

// ============================================================================
// Tests for LiteralNumber PartialEq implementation
// ============================================================================

#[test]
fn test_literal_number_eq_same_values() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: 42.5,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: 42.5,
    };

    assert_eq!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_different_literals() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: 42.5,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: 100.0,
    };

    assert_ne!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_different_literal_expressions() {
    let literal_expression1 = create_literal_expression_with_name("First");
    let literal_expression2 = create_literal_expression_with_name("Second");

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: 42.5,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: 42.5,
    };

    // Should not be equal because literal_expressions are different
    assert_ne!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_negative_zero() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: 0.0,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: -0.0,
    };

    // In IEEE 754, 0.0 == -0.0
    assert_eq!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_infinity() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: f64::INFINITY,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: f64::INFINITY,
    };

    assert_eq!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_neg_infinity() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: f64::NEG_INFINITY,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: f64::NEG_INFINITY,
    };

    assert_eq!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_positive_vs_negative_infinity() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: f64::INFINITY,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: f64::NEG_INFINITY,
    };

    assert_ne!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_nan() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: f64::NAN,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: f64::NAN,
    };

    // NaN != NaN according to IEEE 754
    assert_ne!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_very_small_numbers() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: f64::MIN_POSITIVE,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: f64::MIN_POSITIVE,
    };

    assert_eq!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_very_large_numbers() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: f64::MAX,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: f64::MAX,
    };

    assert_eq!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_clone() {
    let literal_expression = create_literal_expression();
    let literal_num1 = LiteralNumber {
        literal_expression,
        literal: 123.456,
    };

    let literal_num2 = literal_num1.clone();

    assert_eq!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_positive_negative() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: 42.0,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: -42.0,
    };

    assert_ne!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_precision() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    // Test that the implementation uses direct f64 comparison
    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: 0.1 + 0.2,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: 0.3,
    };

    // Due to floating point precision, 0.1 + 0.2 != 0.3
    // This tests that the implementation uses == and not approximate equality
    assert_ne!(literal_num1, literal_num2);
}

#[test]
fn test_literal_number_eq_exact_same_precision_value() {
    let literal_expression1 = create_literal_expression();
    let literal_expression2 = create_literal_expression();

    let value = 0.1 + 0.2;

    let literal_num1 = LiteralNumber {
        literal_expression: literal_expression1,
        literal: value,
    };
    let literal_num2 = LiteralNumber {
        literal_expression: literal_expression2,
        literal: value,
    };

    // Same exact value should be equal
    assert_eq!(literal_num1, literal_num2);
}

// ============================================================================
// Tests for Classifier types
// ============================================================================

#[test]
fn test_class_creation() {
    let element = Element {
        declared_name: Some("MyClass".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let classifier = Classifier { type_ };
    let class = Class { classifier };

    assert_eq!(
        class.classifier.type_.namespace.element.declared_name,
        Some("MyClass".to_string())
    );
}

#[test]
fn test_datatype_creation() {
    let element = Element {
        declared_name: Some("MyDataType".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let classifier = Classifier { type_ };
    let datatype = DataType { classifier };

    assert_eq!(
        datatype.classifier.type_.namespace.element.declared_name,
        Some("MyDataType".to_string())
    );
}

#[test]
fn test_structure_creation() {
    let element = Element {
        declared_name: Some("MyStructure".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let type_ = Type {
        namespace,
        is_sufficient: false,
        is_abstract: None,
        heritage: vec![],
        type_relationships: vec![],
        multiplicity: None,
    };
    let classifier = Classifier { type_ };
    let class = Class { classifier };
    let structure = Structure { class };

    assert_eq!(
        structure
            .class
            .classifier
            .type_
            .namespace
            .element
            .declared_name,
        Some("MyStructure".to_string())
    );
}

// ============================================================================
// Tests for Package types
// ============================================================================

#[test]
fn test_package_creation() {
    let element = Element {
        declared_name: Some("MyPackage".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let package = Package { namespace };

    assert_eq!(
        package.namespace.element.declared_name,
        Some("MyPackage".to_string())
    );
}

#[test]
fn test_library_package_standard() {
    let element = Element {
        declared_name: Some("StandardLibrary".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let package = Package { namespace };
    let lib_package = LibraryPackage {
        package,
        is_standard: true,
    };

    assert!(lib_package.is_standard);
}

#[test]
fn test_library_package_non_standard() {
    let element = Element {
        declared_name: Some("CustomLibrary".to_string()),
        declared_short_name: None,
    };
    let namespace = Namespace {
        element,
        prefixes: vec![],
        children: vec![],
    };
    let package = Package { namespace };
    let lib_package = LibraryPackage {
        package,
        is_standard: false,
    };

    assert!(!lib_package.is_standard);
}

// ============================================================================
// Tests for Comment and Documentation
// ============================================================================

#[test]
fn test_comment_basic() {
    let comment = Comment {
        content: "This is a comment".to_string(),
        about: vec![],
        locale: None,
        span: None,
    };

    assert_eq!(comment.content, "This is a comment");
}

#[test]
fn test_comment_with_locale() {
    let comment = Comment {
        content: "Ceci est un commentaire".to_string(),
        about: vec![],
        locale: Some("fr-FR".to_string()),
        span: None,
    };

    assert_eq!(comment.locale, Some("fr-FR".to_string()));
}

#[test]
fn test_documentation() {
    let comment = Comment {
        content: "Documentation content".to_string(),
        about: vec![],
        locale: None,
        span: None,
    };
    let doc = Documentation {
        comment,
        span: None,
    };

    assert_eq!(doc.comment.content, "Documentation content");
}

// ============================================================================
// Tests for Enum variants
// ============================================================================

#[test]
fn test_abstract_marker() {
    let marker = AbstractMarker::Abstract;
    assert_eq!(marker, AbstractMarker::Abstract);
}

#[test]
fn test_composite_marker() {
    let marker = CompositeMarker::Composite;
    assert_eq!(marker, CompositeMarker::Composite);
}

#[test]
fn test_derived_marker() {
    let marker = DerivedMarker::Derived;
    assert_eq!(marker, DerivedMarker::Derived);
}

#[test]
fn test_end_marker() {
    let marker = EndMarker::End;
    assert_eq!(marker, EndMarker::End);
}

#[test]
fn test_portion_marker() {
    let marker = PortionMarker::Portion;
    assert_eq!(marker, PortionMarker::Portion);
}

#[test]
fn test_readonly_marker() {
    let marker = ReadonlyMarker::Readonly;
    assert_eq!(marker, ReadonlyMarker::Readonly);
}

#[test]
fn test_namespace_marker() {
    let marker = NamespaceMarker::Namespace;
    assert_eq!(marker, NamespaceMarker::Namespace);
}

// ============================================================================
// Tests for FeatureValue
// ============================================================================

#[test]
fn test_feature_value_default() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: false,
    };
    let owning = OwningMembership { membership };
    let feature_value = FeatureValue {
        owning_membership: owning,
        is_default: true,
        is_initial: false,
    };

    assert!(feature_value.is_default);
    assert!(!feature_value.is_initial);
}

#[test]
fn test_feature_value_initial() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: false,
    };
    let owning = OwningMembership { membership };
    let feature_value = FeatureValue {
        owning_membership: owning,
        is_default: false,
        is_initial: true,
    };

    assert!(!feature_value.is_default);
    assert!(feature_value.is_initial);
}
