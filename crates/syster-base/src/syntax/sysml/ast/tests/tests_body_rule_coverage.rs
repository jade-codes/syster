//! Tests to ensure all body rules in the grammar are properly handled.
//!
//! This file serves as a **self-documenting inventory** of all `*_body` rules
//! in the SysML grammar and verifies they are properly handled by `is_body_rule()`.
//!
//! ## Body Rule Categories
//!
//! ### Handled by `is_body_rule()` (nested usages extracted)
//! - `definition_body` - Generic definition body
//! - `action_body` - Action definition/usage body
//! - `calculation_body` - Calculation definition/usage body
//! - `case_body` - Case/analysis/verification/use-case bodies
//! - `constraint_body` - Constraint definition/usage body
//! - `enumeration_body` - Enumeration definition body
//! - `interface_body` - Interface definition/usage body
//! - `metadata_body` - Metadata definition body
//! - `requirement_body` - Requirement definition body
//! - `state_def_body` - State definition body
//! - `state_usage_body` - State usage body
//! - `usage_body` - Generic usage body (delegates to definition_body)
//! - `view_body` - View usage body
//! - `view_definition_body` - View definition body
//!
//! ### NOT handled by `is_body_rule()` (special handling)
//! - `package_body` - Packages have their own visitor entry point
//! - `expression_body` - Expression internals, not AST members
//! - `relationship_body` - Connection relationship bodies, special handling

use crate::parser::sysml::Rule;
use crate::syntax::sysml::ast::utils::is_body_rule;

/// All the body rules that SHOULD be handled by `is_body_rule()`.
///
/// If you add a new `*_body` rule to the grammar that should have its
/// members extracted to the AST, add it here AND to `is_body_rule()`.
const HANDLED_BODY_RULES: &[Rule] = &[
    // Definition bodies (alphabetical)
    Rule::action_body,
    Rule::calculation_body,
    Rule::case_body,
    Rule::constraint_body,
    Rule::definition_body,
    Rule::enumeration_body,
    Rule::interface_body,
    Rule::metadata_body,
    Rule::requirement_body,
    Rule::state_def_body,
    Rule::view_definition_body,
    // Usage bodies
    Rule::state_usage_body,
    Rule::usage_body,
    Rule::view_body,
];

/// Body rules that are intentionally NOT handled by `is_body_rule()`.
/// These have special handling elsewhere or don't contain extractable members.
const EXCLUDED_BODY_RULES: &[Rule] = &[
    Rule::package_body,      // Packages have their own visitor
    Rule::expression_body,   // Expression internals, not members
    Rule::relationship_body, // Connection relationships, special handling
];

#[test]
fn test_all_handled_body_rules_return_true() {
    let mut failures = Vec::new();

    for rule in HANDLED_BODY_RULES {
        if !is_body_rule(*rule) {
            failures.push(format!(
                "Rule::{:?} should be handled by is_body_rule() but returns false",
                rule
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Body rules missing from is_body_rule():\n{}",
        failures.join("\n")
    );
}

#[test]
fn test_excluded_body_rules_return_false() {
    let mut failures = Vec::new();

    for rule in EXCLUDED_BODY_RULES {
        if is_body_rule(*rule) {
            failures.push(format!(
                "Rule::{:?} should NOT be handled by is_body_rule() but returns true",
                rule
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Body rules incorrectly included in is_body_rule():\n{}",
        failures.join("\n")
    );
}

/// Test that nested usages inside interface_body are properly extracted.
/// This is the specific failure case that led to this test file.
#[test]
fn test_interface_body_extracts_port_usages() {
    use crate::parser::sysml::{Rule, SysMLParser};
    use crate::syntax::sysml::ast::parse_definition;
    use pest::Parser;

    let source = r#"interface def DataInterface {
        port dataIn : DataPort;
        port dataOut : DataPort;
    }"#;

    let mut pairs = SysMLParser::parse(Rule::interface_definition, source).expect("parse failed");
    let def = parse_definition(pairs.next().expect("no pair")).expect("AST construction failed");

    assert_eq!(def.name.as_deref(), Some("DataInterface"));

    // Check that port usages were extracted to the body
    let port_count = def
        .body
        .iter()
        .filter(|m| {
            matches!(
                m,
                crate::syntax::sysml::ast::DefinitionMember::Usage(u)
                    if u.kind == crate::syntax::sysml::ast::UsageKind::Port
            )
        })
        .count();

    assert_eq!(
        port_count, 2,
        "Expected 2 port usages in interface body, got {}",
        port_count
    );
}

/// Test that nested usages inside view_body are properly extracted.
#[test]
fn test_view_body_extracts_usages() {
    use crate::parser::sysml::{Rule, SysMLParser};
    use crate::syntax::sysml::ast::parse_usage;
    use pest::Parser;

    let source = r#"view myView : ViewDef {
        attribute name : String;
    }"#;

    let mut pairs = SysMLParser::parse(Rule::view_usage, source).expect("parse failed");
    let usage = parse_usage(pairs.next().expect("no pair"));

    assert_eq!(usage.name.as_deref(), Some("myView"));
    assert_eq!(usage.kind, crate::syntax::sysml::ast::UsageKind::View);

    // Check that attribute usage was extracted to the body
    let attr_count = usage
        .body
        .iter()
        .filter(|m| {
            matches!(
                m,
                crate::syntax::sysml::ast::UsageMember::Usage(u)
                    if u.kind == crate::syntax::sysml::ast::UsageKind::Attribute
            )
        })
        .count();

    assert_eq!(
        attr_count, 1,
        "Expected 1 attribute usage in view body, got {}",
        attr_count
    );
}
