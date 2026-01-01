#![allow(clippy::unwrap_used)]

//! Comprehensive tests for KerML relationship validator.
//!
//! This test suite covers:
//! - Constructor methods: new() and Default::default()
//! - RelationshipValidator trait implementation
//! - All Symbol variants as source and target
//! - Edge cases: empty names, special characters, qualified names
//! - All standard KerML relationship types

use super::super::validator::KermlValidator;
use crate::core::constants::{REL_REDEFINITION, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING};
use crate::semantic::analyzer::validation::RelationshipValidator;
use crate::semantic::symbol_table::Symbol;

// Helper functions to create different Symbol variants

fn create_package(name: &str) -> Symbol {
    Symbol::Package {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_classifier(name: &str) -> Symbol {
    Symbol::Classifier {
        name: name.to_string(),
        qualified_name: name.to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_feature(name: &str) -> Symbol {
    Symbol::Feature {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        feature_type: None,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_definition(name: &str, kind: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: kind.to_string(),
        semantic_role: None,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_usage(name: &str, kind: &str) -> Symbol {
    Symbol::Usage {
        name: name.to_string(),
        qualified_name: name.to_string(),
        kind: kind.to_string(),
        semantic_role: None,
        usage_type: None,
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_alias(name: &str, target: &str) -> Symbol {
    Symbol::Alias {
        name: name.to_string(),
        qualified_name: name.to_string(),
        target: target.to_string(),
        target_span: None,
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

// ============================================================================
// Tests for Constructor Methods (Issues #404 and #406)
// ============================================================================

#[test]
fn test_new_creates_valid_validator() {
    // Test explicit new() constructor (Issue #404)
    let validator = KermlValidator::new();
    let source = create_classifier("TestSource");
    let target = create_classifier("TestTarget");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &source, &target);
    assert!(
        result.is_ok(),
        "Validator created with new() should work correctly"
    );
}

#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_default_trait_creates_valid_validator() {
    // Test Default::default() trait implementation (Issue #406)
    let validator = KermlValidator::default();
    let source = create_classifier("TestSource");
    let target = create_classifier("TestTarget");

    let result = validator.validate_relationship(REL_TYPING, &source, &target);
    assert!(
        result.is_ok(),
        "Validator created with default() should work correctly"
    );
}

#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_new_and_default_are_equivalent() {
    // Verify that new() and default() produce equivalent validators
    let validator_new = KermlValidator::new();
    let validator_default = KermlValidator::default();

    let source = create_feature("feature1");
    let target = create_feature("feature2");

    let result_new = validator_new.validate_relationship(REL_REDEFINITION, &source, &target);
    let result_default =
        validator_default.validate_relationship(REL_REDEFINITION, &source, &target);

    assert_eq!(
        result_new.is_ok(),
        result_default.is_ok(),
        "new() and default() should produce equivalent validators"
    );
}

// ============================================================================
// Tests for Standard KerML Relationships (Issue #405)
// ============================================================================

#[test]
fn test_specialization_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_classifier("Car");
    let target = create_classifier("Vehicle");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &source, &target);
    assert!(result.is_ok(), "Specialization should be accepted");
}

#[test]
fn test_typing_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_feature("speed");
    let target = create_classifier("Real");

    let result = validator.validate_relationship(REL_TYPING, &source, &target);
    assert!(result.is_ok(), "Typing should be accepted");
}

#[test]
fn test_redefinition_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_feature("maxSpeed");
    let target = create_feature("speed");

    let result = validator.validate_relationship(REL_REDEFINITION, &source, &target);
    assert!(result.is_ok(), "Redefinition should be accepted");
}

#[test]
fn test_subsetting_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_feature("vehicleSpeed");
    let target = create_feature("speed");

    let result = validator.validate_relationship(REL_SUBSETTING, &source, &target);
    assert!(result.is_ok(), "Subsetting should be accepted");
}

#[test]
fn test_unknown_relationship_types_are_accepted() {
    let validator = KermlValidator::new();
    let source = create_classifier("Source");
    let target = create_classifier("Target");

    // KerML validator doesn't constrain any relationship types
    let result = validator.validate_relationship("custom_relationship", &source, &target);
    assert!(result.is_ok(), "Custom relationships should be accepted");
}

// ============================================================================
// Tests with Different Symbol Variants
// ============================================================================

#[test]
fn test_package_symbols_in_relationships() {
    let validator = KermlValidator::new();
    let pkg1 = create_package("Package1");
    let pkg2 = create_package("Package2");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &pkg1, &pkg2);
    assert!(
        result.is_ok(),
        "Packages should be accepted in relationships"
    );
}

#[test]
fn test_classifier_symbols_in_relationships() {
    let validator = KermlValidator::new();
    let cls1 = create_classifier("Classifier1");
    let cls2 = create_classifier("Classifier2");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &cls1, &cls2);
    assert!(
        result.is_ok(),
        "Classifiers should be accepted in relationships"
    );
}

#[test]
fn test_feature_symbols_in_relationships() {
    let validator = KermlValidator::new();
    let feat1 = create_feature("feature1");
    let feat2 = create_feature("feature2");

    let result = validator.validate_relationship(REL_REDEFINITION, &feat1, &feat2);
    assert!(
        result.is_ok(),
        "Features should be accepted in relationships"
    );
}

#[test]
fn test_definition_symbols_in_relationships() {
    let validator = KermlValidator::new();
    let def1 = create_definition("Definition1", "PartDef");
    let def2 = create_definition("Definition2", "PartDef");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &def1, &def2);
    assert!(
        result.is_ok(),
        "Definitions should be accepted in relationships"
    );
}

#[test]
fn test_usage_symbols_in_relationships() {
    let validator = KermlValidator::new();
    let usage1 = create_usage("usage1", "part");
    let usage2 = create_usage("usage2", "part");

    let result = validator.validate_relationship(REL_SUBSETTING, &usage1, &usage2);
    assert!(result.is_ok(), "Usages should be accepted in relationships");
}

#[test]
fn test_alias_symbols_in_relationships() {
    let validator = KermlValidator::new();
    let alias1 = create_alias("alias1", "Target1");
    let alias2 = create_alias("alias2", "Target2");

    let result = validator.validate_relationship(REL_TYPING, &alias1, &alias2);
    assert!(
        result.is_ok(),
        "Aliases should be accepted in relationships"
    );
}

#[test]
fn test_mixed_symbol_variants_in_relationships() {
    let validator = KermlValidator::new();

    // Test various combinations of different symbol types
    let package = create_package("MyPackage");
    let classifier = create_classifier("MyClass");
    let feature = create_feature("myFeature");
    let definition = create_definition("MyDef", "PartDef");
    let usage = create_usage("myUsage", "part");
    let alias = create_alias("myAlias", "SomeTarget");

    // All combinations should be accepted by KerML validator
    assert!(
        validator
            .validate_relationship(REL_TYPING, &feature, &classifier)
            .is_ok()
    );
    assert!(
        validator
            .validate_relationship(REL_SPECIALIZATION, &definition, &classifier)
            .is_ok()
    );
    assert!(
        validator
            .validate_relationship(REL_SUBSETTING, &usage, &feature)
            .is_ok()
    );
    assert!(
        validator
            .validate_relationship(REL_REDEFINITION, &alias, &definition)
            .is_ok()
    );
    assert!(
        validator
            .validate_relationship(REL_TYPING, &package, &usage)
            .is_ok()
    );
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_empty_relationship_type_is_accepted() {
    let validator = KermlValidator::new();
    let source = create_classifier("Source");
    let target = create_classifier("Target");

    let result = validator.validate_relationship("", &source, &target);
    assert!(result.is_ok(), "Empty relationship type should be accepted");
}

#[test]
fn test_symbols_with_empty_names() {
    let validator = KermlValidator::new();
    let source = create_classifier("");
    let target = create_feature("");

    let result = validator.validate_relationship(REL_TYPING, &source, &target);
    assert!(
        result.is_ok(),
        "Symbols with empty names should be accepted"
    );
}

#[test]
fn test_symbols_with_qualified_names() {
    let validator = KermlValidator::new();
    let source = Symbol::Classifier {
        name: "Vehicle".to_string(),
        qualified_name: "Vehicles::Vehicle".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
        scope_id: 1,
        source_file: Some("vehicles.kerml".to_string()),
        span: None,
        references: Vec::new(),
    };
    let target = Symbol::Classifier {
        name: "Thing".to_string(),
        qualified_name: "Base::Thing".to_string(),
        kind: "Class".to_string(),
        is_abstract: true,
        scope_id: 0,
        source_file: Some("base.kerml".to_string()),
        span: None,
        references: Vec::new(),
    };

    let result = validator.validate_relationship(REL_SPECIALIZATION, &source, &target);
    assert!(
        result.is_ok(),
        "Symbols with qualified names should be accepted"
    );
}

#[test]
fn test_symbols_with_special_characters_in_names() {
    let validator = KermlValidator::new();
    let source = create_classifier("My-Class_123");
    let target = create_classifier("Parent.Class$v2");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &source, &target);
    assert!(
        result.is_ok(),
        "Symbols with special characters should be accepted"
    );
}

#[test]
fn test_same_source_and_target_symbol() {
    let validator = KermlValidator::new();
    let symbol = create_classifier("SelfReferencing");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &symbol, &symbol);
    assert!(
        result.is_ok(),
        "Self-referencing relationships should be accepted"
    );
}

#[test]
fn test_relationship_type_with_special_characters() {
    let validator = KermlValidator::new();
    let source = create_feature("feature1");
    let target = create_feature("feature2");

    let result = validator.validate_relationship("custom:relationship-type_v2", &source, &target);
    assert!(
        result.is_ok(),
        "Relationship types with special characters should be accepted"
    );
}

#[test]
fn test_validator_accepts_all_relationship_types_from_constants() {
    let validator = KermlValidator::new();
    let source = create_classifier("Source");
    let target = create_classifier("Target");

    // Test all standard relationship constants
    let relationship_types = [
        REL_SPECIALIZATION,
        REL_TYPING,
        REL_REDEFINITION,
        REL_SUBSETTING,
    ];

    for rel_type in relationship_types {
        let result = validator.validate_relationship(rel_type, &source, &target);
        assert!(
            result.is_ok(),
            "Relationship type '{}' should be accepted",
            rel_type
        );
    }
}

// ============================================================================
// Property and Trait Tests
// ============================================================================

#[test]
fn test_validator_is_send_sync() {
    // Ensure the validator can be shared across threads
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<KermlValidator>();
}

#[test]
fn test_validator_can_be_used_multiple_times() {
    let validator = KermlValidator::new();
    let source = create_classifier("Source");
    let target = create_classifier("Target");

    // Call validate_relationship multiple times
    for i in 0..10 {
        let result = validator.validate_relationship(REL_TYPING, &source, &target);
        assert!(result.is_ok(), "Validator should work on iteration {}", i);
    }
}

#[test]
#[allow(clippy::default_constructed_unit_structs)]
fn test_multiple_validators_are_independent() {
    let validator1 = KermlValidator::new();
    let validator2 = KermlValidator::default();

    let source = create_feature("feature");
    let target = create_classifier("Type");

    let result1 = validator1.validate_relationship(REL_TYPING, &source, &target);
    let result2 = validator2.validate_relationship(REL_TYPING, &source, &target);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert_eq!(
        result1.is_ok(),
        result2.is_ok(),
        "Independent validators should behave identically"
    );
}
