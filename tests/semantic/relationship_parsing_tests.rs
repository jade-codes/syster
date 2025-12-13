#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use syster::language::sysml::syntax::{Definition, Usage};
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_parse_definition_with_specialization() {
    let source = "part def Car :> Vehicle;";

    let pairs = SysMLParser::parse(Rule::part_definition, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let def = Definition::from_pest(&mut pairs.unwrap());
    assert!(def.is_ok(), "Failed to convert to AST: {:?}", def.err());

    let def = def.unwrap();
    assert_eq!(def.name, Some("Car".to_string()));
    assert_eq!(
        def.relationships.specializes.len(),
        1,
        "Expected 1 specialization, got: {:?}",
        def.relationships.specializes
    );
    assert_eq!(def.relationships.specializes[0], "Vehicle");
}

#[test]
fn test_parse_usage_with_typed_by() {
    let source = "part vehicle : Vehicle;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let usage = Usage::from_pest(&mut pairs.unwrap());
    assert!(usage.is_ok(), "Failed to convert to AST: {:?}", usage.err());

    let usage = usage.unwrap();
    assert_eq!(usage.name, Some("vehicle".to_string()));
    assert_eq!(
        usage.relationships.typed_by,
        Some("Vehicle".to_string()),
        "Expected typed_by = Vehicle, got: {:?}",
        usage.relationships.typed_by
    );
}

#[test]
fn test_parse_usage_with_subsets() {
    let source = "part vehicle2 :> vehicle1;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let usage = Usage::from_pest(&mut pairs.unwrap());
    assert!(usage.is_ok(), "Failed to convert to AST: {:?}", usage.err());

    let usage = usage.unwrap();
    assert_eq!(usage.name, Some("vehicle2".to_string()));
    assert_eq!(
        usage.relationships.subsets.len(),
        1,
        "Expected 1 subset, got: {:?}",
        usage.relationships.subsets
    );
    assert_eq!(usage.relationships.subsets[0], "vehicle1");
}

#[test]
fn test_parse_usage_with_redefines() {
    let source = "part vehicle2 :>> vehicle1;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let usage = Usage::from_pest(&mut pairs.unwrap());
    assert!(usage.is_ok(), "Failed to convert to AST: {:?}", usage.err());

    let usage = usage.unwrap();
    assert_eq!(usage.name, Some("vehicle2".to_string()));
    assert_eq!(
        usage.relationships.redefines.len(),
        1,
        "Expected 1 redefinition, got: {:?}",
        usage.relationships.redefines
    );
    assert_eq!(usage.relationships.redefines[0], "vehicle1");
}
