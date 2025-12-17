#![allow(clippy::unwrap_used)]
#![allow(clippy::result_large_err)]

use crate::semantic::processors::RelationshipValidator;
use crate::semantic::symbol_table::Symbol;
use crate::syntax::sysml::ast::constants::{
    SYSML_KIND_ACTION, SYSML_KIND_REQUIREMENT, SYSML_KIND_STATE, SYSML_KIND_USE_CASE,
};
use crate::syntax::sysml::validator::SysMLRelationshipValidator;

fn create_requirement(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: SYSML_KIND_REQUIREMENT.to_string(),
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_action(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: SYSML_KIND_ACTION.to_string(),
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_state(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: SYSML_KIND_STATE.to_string(),
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_use_case(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: SYSML_KIND_USE_CASE.to_string(),
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

fn create_part(name: &str) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: name.to_string(),
        scope_id: 0,
        kind: "Part".to_string(),
        source_file: None,
        span: None,
        references: Vec::new(),
    }
}

#[test]
fn test_satisfy_accepts_requirement() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_part("Source");
    let target = create_requirement("Req1");

    let result = validator.validate_relationship("satisfy", &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_satisfy_rejects_non_requirement() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_part("Source");
    let target = create_action("Action1");

    let result = validator.validate_relationship("satisfy", &source, &target);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must target a requirement")
    );
}

#[test]
fn test_perform_accepts_action() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_part("Source");
    let target = create_action("Action1");

    let result = validator.validate_relationship("perform", &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_perform_rejects_non_action() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_part("Source");
    let target = create_requirement("Req1");

    let result = validator.validate_relationship("perform", &source, &target);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must target an action")
    );
}

#[test]
fn test_exhibit_accepts_state() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_part("Source");
    let target = create_state("State1");

    let result = validator.validate_relationship("exhibit", &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_exhibit_rejects_non_state() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_part("Source");
    let target = create_action("Action1");

    let result = validator.validate_relationship("exhibit", &source, &target);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must target a state")
    );
}

#[test]
fn test_include_accepts_use_case() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_use_case("UseCase1");
    let target = create_use_case("UseCase2");

    let result = validator.validate_relationship("include", &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_include_rejects_non_use_case() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_use_case("UseCase1");
    let target = create_action("Action1");

    let result = validator.validate_relationship("include", &source, &target);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("must target a use case")
    );
}

#[test]
fn test_other_relationships_have_no_constraints() {
    let validator = SysMLRelationshipValidator::new();
    let source = create_part("Source");
    let target = create_part("Target");

    // Should accept any relationship type not in the constraint list
    let result = validator.validate_relationship("specialization", &source, &target);
    assert!(result.is_ok());

    let result = validator.validate_relationship("typing", &source, &target);
    assert!(result.is_ok());
}
