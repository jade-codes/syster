#![allow(clippy::unwrap_used)]

//! Tests for KerML relationship validator.

use super::validator::KermlValidator;
use crate::core::constants::{REL_REDEFINITION, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING};
use crate::semantic::analyzer::validation::RelationshipValidator;
use crate::semantic::symbol_table::Symbol;

fn create_classifier(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "Classifier".to_string(),
        semantic_role: None,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_feature(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "Feature".to_string(),
        semantic_role: None,
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

#[test]
fn test_specialization_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_classifier("Car");
    let target = create_classifier("Vehicle");

    let result = validator.validate_relationship(REL_SPECIALIZATION, &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_typing_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_feature("speed");
    let target = create_classifier("Real");

    let result = validator.validate_relationship(REL_TYPING, &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_redefinition_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_feature("maxSpeed");
    let target = create_feature("speed");

    let result = validator.validate_relationship(REL_REDEFINITION, &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_subsetting_relationship_accepts_any_symbols() {
    let validator = KermlValidator::new();
    let source = create_feature("vehicleSpeed");
    let target = create_feature("speed");

    let result = validator.validate_relationship(REL_SUBSETTING, &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_unknown_relationship_types_are_accepted() {
    let validator = KermlValidator::new();
    let source = create_classifier("Source");
    let target = create_classifier("Target");

    // KerML validator doesn't constrain any relationship types
    let result = validator.validate_relationship("custom_relationship", &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_default_constructor() {
    let validator = KermlValidator;
    let source = create_classifier("A");
    let target = create_classifier("B");

    let result = validator.validate_relationship(REL_TYPING, &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_validator_is_send_sync() {
    // Ensure the validator can be shared across threads
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<KermlValidator>();
}
