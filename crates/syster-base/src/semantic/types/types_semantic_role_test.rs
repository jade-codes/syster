#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;

// ============================================================================
// Tests for is_requirement method (issue #330)
// ============================================================================

#[test]
fn test_is_requirement_returns_true_for_requirement() {
    assert!(SemanticRole::Requirement.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_action() {
    assert!(!SemanticRole::Action.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_state() {
    assert!(!SemanticRole::State.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_use_case() {
    assert!(!SemanticRole::UseCase.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_component() {
    assert!(!SemanticRole::Component.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_interface() {
    assert!(!SemanticRole::Interface.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_port() {
    assert!(!SemanticRole::Port.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_attribute() {
    assert!(!SemanticRole::Attribute.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_connection() {
    assert!(!SemanticRole::Connection.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_constraint() {
    assert!(!SemanticRole::Constraint.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_analysis_case() {
    assert!(!SemanticRole::AnalysisCase.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_verification_case() {
    assert!(!SemanticRole::VerificationCase.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_view() {
    assert!(!SemanticRole::View.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_metadata() {
    assert!(!SemanticRole::Metadata.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_item() {
    assert!(!SemanticRole::Item.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_flow() {
    assert!(!SemanticRole::Flow.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_allocation() {
    assert!(!SemanticRole::Allocation.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_classifier() {
    assert!(!SemanticRole::Classifier.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_feature() {
    assert!(!SemanticRole::Feature.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_package() {
    assert!(!SemanticRole::Package.is_requirement());
}

#[test]
fn test_is_requirement_returns_false_for_unknown() {
    assert!(!SemanticRole::Unknown("custom".to_string()).is_requirement());
}

// ============================================================================
// Tests for is_action method (issue #332)
// ============================================================================

#[test]
fn test_is_action_returns_true_for_action() {
    assert!(SemanticRole::Action.is_action());
}

#[test]
fn test_is_action_returns_false_for_requirement() {
    assert!(!SemanticRole::Requirement.is_action());
}

#[test]
fn test_is_action_returns_false_for_state() {
    assert!(!SemanticRole::State.is_action());
}

#[test]
fn test_is_action_returns_false_for_use_case() {
    assert!(!SemanticRole::UseCase.is_action());
}

#[test]
fn test_is_action_returns_false_for_component() {
    assert!(!SemanticRole::Component.is_action());
}

#[test]
fn test_is_action_returns_false_for_interface() {
    assert!(!SemanticRole::Interface.is_action());
}

#[test]
fn test_is_action_returns_false_for_port() {
    assert!(!SemanticRole::Port.is_action());
}

#[test]
fn test_is_action_returns_false_for_attribute() {
    assert!(!SemanticRole::Attribute.is_action());
}

#[test]
fn test_is_action_returns_false_for_connection() {
    assert!(!SemanticRole::Connection.is_action());
}

#[test]
fn test_is_action_returns_false_for_constraint() {
    assert!(!SemanticRole::Constraint.is_action());
}

#[test]
fn test_is_action_returns_false_for_analysis_case() {
    assert!(!SemanticRole::AnalysisCase.is_action());
}

#[test]
fn test_is_action_returns_false_for_verification_case() {
    assert!(!SemanticRole::VerificationCase.is_action());
}

#[test]
fn test_is_action_returns_false_for_view() {
    assert!(!SemanticRole::View.is_action());
}

#[test]
fn test_is_action_returns_false_for_metadata() {
    assert!(!SemanticRole::Metadata.is_action());
}

#[test]
fn test_is_action_returns_false_for_item() {
    assert!(!SemanticRole::Item.is_action());
}

#[test]
fn test_is_action_returns_false_for_flow() {
    assert!(!SemanticRole::Flow.is_action());
}

#[test]
fn test_is_action_returns_false_for_allocation() {
    assert!(!SemanticRole::Allocation.is_action());
}

#[test]
fn test_is_action_returns_false_for_classifier() {
    assert!(!SemanticRole::Classifier.is_action());
}

#[test]
fn test_is_action_returns_false_for_feature() {
    assert!(!SemanticRole::Feature.is_action());
}

#[test]
fn test_is_action_returns_false_for_package() {
    assert!(!SemanticRole::Package.is_action());
}

#[test]
fn test_is_action_returns_false_for_unknown() {
    assert!(!SemanticRole::Unknown("custom".to_string()).is_action());
}

// ============================================================================
// Tests for is_state method (issue #331)
// ============================================================================

#[test]
fn test_is_state_returns_true_for_state() {
    assert!(SemanticRole::State.is_state());
}

#[test]
fn test_is_state_returns_false_for_requirement() {
    assert!(!SemanticRole::Requirement.is_state());
}

#[test]
fn test_is_state_returns_false_for_action() {
    assert!(!SemanticRole::Action.is_state());
}

#[test]
fn test_is_state_returns_false_for_use_case() {
    assert!(!SemanticRole::UseCase.is_state());
}

#[test]
fn test_is_state_returns_false_for_component() {
    assert!(!SemanticRole::Component.is_state());
}

#[test]
fn test_is_state_returns_false_for_interface() {
    assert!(!SemanticRole::Interface.is_state());
}

#[test]
fn test_is_state_returns_false_for_port() {
    assert!(!SemanticRole::Port.is_state());
}

#[test]
fn test_is_state_returns_false_for_attribute() {
    assert!(!SemanticRole::Attribute.is_state());
}

#[test]
fn test_is_state_returns_false_for_connection() {
    assert!(!SemanticRole::Connection.is_state());
}

#[test]
fn test_is_state_returns_false_for_constraint() {
    assert!(!SemanticRole::Constraint.is_state());
}

#[test]
fn test_is_state_returns_false_for_analysis_case() {
    assert!(!SemanticRole::AnalysisCase.is_state());
}

#[test]
fn test_is_state_returns_false_for_verification_case() {
    assert!(!SemanticRole::VerificationCase.is_state());
}

#[test]
fn test_is_state_returns_false_for_view() {
    assert!(!SemanticRole::View.is_state());
}

#[test]
fn test_is_state_returns_false_for_metadata() {
    assert!(!SemanticRole::Metadata.is_state());
}

#[test]
fn test_is_state_returns_false_for_item() {
    assert!(!SemanticRole::Item.is_state());
}

#[test]
fn test_is_state_returns_false_for_flow() {
    assert!(!SemanticRole::Flow.is_state());
}

#[test]
fn test_is_state_returns_false_for_allocation() {
    assert!(!SemanticRole::Allocation.is_state());
}

#[test]
fn test_is_state_returns_false_for_classifier() {
    assert!(!SemanticRole::Classifier.is_state());
}

#[test]
fn test_is_state_returns_false_for_feature() {
    assert!(!SemanticRole::Feature.is_state());
}

#[test]
fn test_is_state_returns_false_for_package() {
    assert!(!SemanticRole::Package.is_state());
}

#[test]
fn test_is_state_returns_false_for_unknown() {
    assert!(!SemanticRole::Unknown("custom".to_string()).is_state());
}

// ============================================================================
// Tests for is_use_case method (issue #329)
// ============================================================================

#[test]
fn test_is_use_case_returns_true_for_use_case() {
    assert!(SemanticRole::UseCase.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_requirement() {
    assert!(!SemanticRole::Requirement.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_action() {
    assert!(!SemanticRole::Action.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_state() {
    assert!(!SemanticRole::State.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_component() {
    assert!(!SemanticRole::Component.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_interface() {
    assert!(!SemanticRole::Interface.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_port() {
    assert!(!SemanticRole::Port.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_attribute() {
    assert!(!SemanticRole::Attribute.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_connection() {
    assert!(!SemanticRole::Connection.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_constraint() {
    assert!(!SemanticRole::Constraint.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_analysis_case() {
    assert!(!SemanticRole::AnalysisCase.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_verification_case() {
    assert!(!SemanticRole::VerificationCase.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_view() {
    assert!(!SemanticRole::View.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_metadata() {
    assert!(!SemanticRole::Metadata.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_item() {
    assert!(!SemanticRole::Item.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_flow() {
    assert!(!SemanticRole::Flow.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_allocation() {
    assert!(!SemanticRole::Allocation.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_classifier() {
    assert!(!SemanticRole::Classifier.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_feature() {
    assert!(!SemanticRole::Feature.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_package() {
    assert!(!SemanticRole::Package.is_use_case());
}

#[test]
fn test_is_use_case_returns_false_for_unknown() {
    assert!(!SemanticRole::Unknown("custom".to_string()).is_use_case());
}

// ============================================================================
// Tests for Display implementation (issue #333)
// ============================================================================

#[test]
fn test_display_requirement() {
    assert_eq!(SemanticRole::Requirement.to_string(), "requirement");
}

#[test]
fn test_display_action() {
    assert_eq!(SemanticRole::Action.to_string(), "action");
}

#[test]
fn test_display_state() {
    assert_eq!(SemanticRole::State.to_string(), "state");
}

#[test]
fn test_display_use_case() {
    assert_eq!(SemanticRole::UseCase.to_string(), "use case");
}

#[test]
fn test_display_component() {
    assert_eq!(SemanticRole::Component.to_string(), "component");
}

#[test]
fn test_display_interface() {
    assert_eq!(SemanticRole::Interface.to_string(), "interface");
}

#[test]
fn test_display_port() {
    assert_eq!(SemanticRole::Port.to_string(), "port");
}

#[test]
fn test_display_attribute() {
    assert_eq!(SemanticRole::Attribute.to_string(), "attribute");
}

#[test]
fn test_display_connection() {
    assert_eq!(SemanticRole::Connection.to_string(), "connection");
}

#[test]
fn test_display_constraint() {
    assert_eq!(SemanticRole::Constraint.to_string(), "constraint");
}

#[test]
fn test_display_analysis_case() {
    assert_eq!(SemanticRole::AnalysisCase.to_string(), "analysis case");
}

#[test]
fn test_display_verification_case() {
    assert_eq!(
        SemanticRole::VerificationCase.to_string(),
        "verification case"
    );
}

#[test]
fn test_display_view() {
    assert_eq!(SemanticRole::View.to_string(), "view");
}

#[test]
fn test_display_metadata() {
    assert_eq!(SemanticRole::Metadata.to_string(), "metadata");
}

#[test]
fn test_display_item() {
    assert_eq!(SemanticRole::Item.to_string(), "item");
}

#[test]
fn test_display_flow() {
    assert_eq!(SemanticRole::Flow.to_string(), "flow");
}

#[test]
fn test_display_allocation() {
    assert_eq!(SemanticRole::Allocation.to_string(), "allocation");
}

#[test]
fn test_display_classifier() {
    assert_eq!(SemanticRole::Classifier.to_string(), "classifier");
}

#[test]
fn test_display_feature() {
    assert_eq!(SemanticRole::Feature.to_string(), "feature");
}

#[test]
fn test_display_package() {
    assert_eq!(SemanticRole::Package.to_string(), "package");
}

#[test]
fn test_display_unknown_with_simple_string() {
    assert_eq!(
        SemanticRole::Unknown("custom".to_string()).to_string(),
        "custom"
    );
}

#[test]
fn test_display_unknown_with_empty_string() {
    assert_eq!(SemanticRole::Unknown("".to_string()).to_string(), "");
}

#[test]
fn test_display_unknown_with_qualified_name() {
    assert_eq!(
        SemanticRole::Unknown("Package::Type".to_string()).to_string(),
        "Package::Type"
    );
}

#[test]
fn test_display_unknown_with_special_characters() {
    assert_eq!(
        SemanticRole::Unknown("Type-With_Special.Chars!".to_string()).to_string(),
        "Type-With_Special.Chars!"
    );
}

#[test]
fn test_display_unknown_with_unicode() {
    assert_eq!(
        SemanticRole::Unknown("类型名称".to_string()).to_string(),
        "类型名称"
    );
}

#[test]
fn test_display_unknown_with_whitespace() {
    assert_eq!(
        SemanticRole::Unknown("Type With Spaces".to_string()).to_string(),
        "Type With Spaces"
    );
}

#[test]
fn test_display_consistency_with_format_macro() {
    let role = SemanticRole::Action;
    assert_eq!(format!("{}", role), "action");
    assert_eq!(role.to_string(), "action");
}

#[test]
fn test_display_all_variants_are_lowercase_or_multi_word() {
    // Test that all Display outputs follow consistent naming convention
    let variants = vec![
        (SemanticRole::Requirement, "requirement"),
        (SemanticRole::Action, "action"),
        (SemanticRole::State, "state"),
        (SemanticRole::UseCase, "use case"),
        (SemanticRole::Component, "component"),
        (SemanticRole::Interface, "interface"),
        (SemanticRole::Port, "port"),
        (SemanticRole::Attribute, "attribute"),
        (SemanticRole::Connection, "connection"),
        (SemanticRole::Constraint, "constraint"),
        (SemanticRole::AnalysisCase, "analysis case"),
        (SemanticRole::VerificationCase, "verification case"),
        (SemanticRole::View, "view"),
        (SemanticRole::Metadata, "metadata"),
        (SemanticRole::Item, "item"),
        (SemanticRole::Flow, "flow"),
        (SemanticRole::Allocation, "allocation"),
        (SemanticRole::Classifier, "classifier"),
        (SemanticRole::Feature, "feature"),
        (SemanticRole::Package, "package"),
    ];

    for (variant, expected) in variants {
        assert_eq!(variant.to_string(), expected);
    }
}
