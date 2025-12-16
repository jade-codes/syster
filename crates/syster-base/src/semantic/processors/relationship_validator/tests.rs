use crate::semantic::processors::{NoOpValidator, RelationshipValidator};
use crate::semantic::symbol_table::Symbol;

#[test]
fn test_noop_validator_accepts_all_relationships() {
    let validator = NoOpValidator;
    let source = Symbol::Package {
        name: "Source".to_string(),
        qualified_name: "Source".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    };
    let target = Symbol::Package {
        name: "Target".to_string(),
        qualified_name: "Target".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    };

    let result = validator.validate_relationship("any_type", &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_noop_validator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NoOpValidator>();
}
