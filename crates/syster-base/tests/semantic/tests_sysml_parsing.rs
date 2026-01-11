#![allow(clippy::unwrap_used)]

use pest::Parser;
use syster::parser::{SysMLParser, sysml::Rule};
use syster::syntax::sysml::ast::{parse_definition, parse_usage};

#[test]
fn test_parse_definition_with_specialization() {
    let source = "part def Car :> Vehicle;";

    let pairs = SysMLParser::parse(Rule::part_definition, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let def = parse_definition(pairs.unwrap().next().unwrap());
    assert!(def.is_ok(), "Failed to convert to AST: {:?}", def.err());

    let def = def.unwrap();
    assert_eq!(def.name, Some("Car".to_string()));
    assert_eq!(
        def.relationships.specializes.len(),
        1,
        "Expected 1 specialization, got: {:?}",
        def.relationships.specializes
    );
    assert_eq!(def.relationships.specializes[0].target, "Vehicle");
}

#[test]
fn test_parse_usage_with_typed_by() {
    let source = "part vehicle : Vehicle;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let usage = parse_usage(pairs.unwrap().next().unwrap());

    // Usage is now returned directly, no unwrap needed
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

    let usage = parse_usage(pairs.unwrap().next().unwrap());

    // Usage is now returned directly, no unwrap needed
    assert_eq!(usage.name, Some("vehicle2".to_string()));
    assert_eq!(
        usage.relationships.subsets.len(),
        1,
        "Expected 1 subset, got: {:?}",
        usage.relationships.subsets
    );
    assert_eq!(usage.relationships.subsets[0].target, "vehicle1");
}

#[test]
fn test_parse_usage_with_redefines() {
    let source = "part vehicle2 :>> vehicle1;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let usage = parse_usage(pairs.unwrap().next().unwrap());

    // Usage is now returned directly, no unwrap needed
    assert_eq!(usage.name, Some("vehicle2".to_string()));
    assert_eq!(
        usage.relationships.redefines.len(),
        1,
        "Expected 1 redefinition, got: {:?}",
        usage.relationships.redefines
    );
    assert_eq!(usage.relationships.redefines[0].target, "vehicle1");
}

#[test]
fn test_parse_definition_with_multiple_specializations() {
    let source = "part def SportsCar :> Car, Vehicle;";

    let pairs = SysMLParser::parse(Rule::part_definition, source);
    assert!(pairs.is_ok());

    let def = parse_definition(pairs.unwrap().next().unwrap()).unwrap();
    assert_eq!(def.name, Some("SportsCar".to_string()));
    assert_eq!(def.relationships.specializes.len(), 2);
    assert_eq!(def.relationships.specializes[0].target, "Car");
    assert_eq!(def.relationships.specializes[1].target, "Vehicle");
}

#[test]
fn test_parse_usage_with_multiple_subsets() {
    let source = "part myPart :> part1, part2, part3;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok());

    let usage = parse_usage(pairs.unwrap().next().unwrap());
    assert_eq!(usage.name, Some("myPart".to_string()));
    assert_eq!(usage.relationships.subsets.len(), 3);
    assert_eq!(usage.relationships.subsets[0].target, "part1");
    assert_eq!(usage.relationships.subsets[1].target, "part2");
    assert_eq!(usage.relationships.subsets[2].target, "part3");
}

#[test]
fn test_parse_usage_with_typed_and_subsets() {
    let source = "part vehicle : VehicleDef :> basePart;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok());

    let usage = parse_usage(pairs.unwrap().next().unwrap());
    assert_eq!(usage.name, Some("vehicle".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("VehicleDef".to_string()));
    assert_eq!(usage.relationships.subsets.len(), 1);
    assert_eq!(usage.relationships.subsets[0].target, "basePart");
}

#[test]
fn test_parse_usage_with_multiple_redefines() {
    let source = "part newPart :>> oldPart1, oldPart2;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok());

    let usage = parse_usage(pairs.unwrap().next().unwrap());
    assert_eq!(usage.name, Some("newPart".to_string()));
    assert_eq!(usage.relationships.redefines.len(), 2);
    assert_eq!(usage.relationships.redefines[0].target, "oldPart1");
    assert_eq!(usage.relationships.redefines[1].target, "oldPart2");
}

#[test]
fn test_parse_action_definition_with_specialization() {
    let source = "action def Drive :> Action;";

    let pairs = SysMLParser::parse(Rule::action_definition, source);
    assert!(pairs.is_ok());

    let def = parse_definition(pairs.unwrap().next().unwrap()).unwrap();
    assert_eq!(def.name, Some("Drive".to_string()));
    assert_eq!(def.relationships.specializes.len(), 1);
    assert_eq!(def.relationships.specializes[0].target, "Action");
}

#[test]
fn test_parse_requirement_definition_with_specialization() {
    let source = "requirement def SafetyReq :> BaseRequirement;";

    let pairs = SysMLParser::parse(Rule::requirement_definition, source);
    assert!(pairs.is_ok());

    let def = parse_definition(pairs.unwrap().next().unwrap()).unwrap();
    assert_eq!(def.name, Some("SafetyReq".to_string()));
    assert_eq!(def.relationships.specializes.len(), 1);
    assert_eq!(def.relationships.specializes[0].target, "BaseRequirement");
}

#[test]
fn test_parse_anonymous_definition() {
    let source = "part def;";

    let pairs = SysMLParser::parse(Rule::part_definition, source);
    assert!(pairs.is_ok());

    let def = parse_definition(pairs.unwrap().next().unwrap()).unwrap();
    assert_eq!(def.name, None);
    assert_eq!(def.relationships.specializes.len(), 0);
}

#[test]
fn test_parse_item_definition_with_index() {
    let source = "item def Fuel :> Material;";

    let pairs = SysMLParser::parse(Rule::item_definition, source);
    assert!(pairs.is_ok());

    let def = parse_definition(pairs.unwrap().next().unwrap()).unwrap();
    assert_eq!(def.name, Some("Fuel".to_string()));
    assert_eq!(def.relationships.specializes.len(), 1);
}

#[test]
fn test_parse_attribute_definition_with_index() {
    let source = "attribute def Speed :> Measurement;";

    let pairs = SysMLParser::parse(Rule::attribute_definition, source);
    assert!(pairs.is_ok());

    let def = parse_definition(pairs.unwrap().next().unwrap()).unwrap();
    assert_eq!(def.name, Some("Speed".to_string()));
    assert_eq!(def.relationships.specializes.len(), 1);
}

#[test]
fn test_parse_usage_complex_relationships() {
    let source = "part enginePart : Engine :> vehiclePart :>> oldEngine;";

    let pairs = SysMLParser::parse(Rule::part_usage, source);
    assert!(pairs.is_ok());

    let usage = parse_usage(pairs.unwrap().next().unwrap());
    assert_eq!(usage.name, Some("enginePart".to_string()));
    assert_eq!(usage.relationships.typed_by, Some("Engine".to_string()));
    assert_eq!(usage.relationships.subsets.len(), 1);
    assert_eq!(usage.relationships.subsets[0].target, "vehiclePart");
    assert_eq!(usage.relationships.redefines.len(), 1);
    assert_eq!(usage.relationships.redefines[0].target, "oldEngine");
}

#[test]
fn test_parse_model_with_satisfy_relationship() {
    // Integration test - satisfy is parsed as part of a complete model
    let source = "requirement def SafetyReq; case def SafetyCase { satisfy SafetyReq; }";

    let pairs = SysMLParser::parse(Rule::model, source);
    assert!(
        pairs.is_ok(),
        "Failed to parse model with satisfy: {:?}",
        pairs.err()
    );
}

#[test]
fn test_parse_model_with_satisfy_requirement_keyword() {
    // Test satisfy with full 'requirement' keyword
    let source =
        "requirement def SafetyReq; case def SafetyCase { satisfy requirement SafetyReq; }";

    let pairs = SysMLParser::parse(Rule::model, source);
    assert!(
        pairs.is_ok(),
        "Failed to parse model with satisfy requirement: {:?}",
        pairs.err()
    );
}

#[test]
fn test_parse_model_with_perform_relationship() {
    let source = "action def Move; part def Robot { perform Move; }";

    let pairs = SysMLParser::parse(Rule::model, source);
    assert!(
        pairs.is_ok(),
        "Failed to parse model with perform: {:?}",
        pairs.err()
    );
}

#[test]
fn test_parse_model_with_exhibit_relationship() {
    let source = "state def Moving; part def Vehicle { exhibit Moving; }";

    let pairs = SysMLParser::parse(Rule::model, source);
    assert!(
        pairs.is_ok(),
        "Failed to parse model with exhibit: {:?}",
        pairs.err()
    );
}

#[test]
fn test_parse_model_with_include_relationship() {
    // Integration test - include is parsed as part of a complete model
    let source = "use case def Login; use case def ManageAccount { include Login; }";

    let pairs = SysMLParser::parse(Rule::model, source);
    assert!(
        pairs.is_ok(),
        "Failed to parse model with include: {:?}",
        pairs.err()
    );
}

#[test]
fn test_parse_attribute_with_quoted_name_redefines() {
    // Test that quoted names in redefines are parsed correctly without quotes
    let source = r#"attribute 'packet primary header' redefines 'packet header';"#;

    let pairs = SysMLParser::parse(Rule::attribute_usage, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let usage = parse_usage(pairs.unwrap().next().unwrap());

    // Usage is now returned directly, no unwrap needed
    // Name should NOT have quotes
    assert_eq!(
        usage.name,
        Some("packet primary header".to_string()),
        "Name should not have quotes"
    );
    // Redefines target should NOT have quotes
    assert_eq!(
        usage.relationships.redefines.len(),
        1,
        "Expected 1 redefinition"
    );
    assert_eq!(
        usage.relationships.redefines[0].target, "packet header",
        "Redefines target should not have quotes"
    );
}

#[test]
fn test_parse_part_def_with_quoted_names() {
    // Test the full scenario from the bug report
    let source = r#"part def 'Data Packet' {
        attribute 'packet primary header' redefines 'packet header' {
            attribute 'packet version number': Integer;
            attribute 'packet identification': String;
            attribute 'packet data length': Integer;
        }
        attribute redefines 'packet data field';
    }"#;

    let pairs = SysMLParser::parse(Rule::part_definition, source);
    assert!(pairs.is_ok(), "Failed to parse: {:?}", pairs.err());

    let def = parse_definition(pairs.unwrap().next().unwrap());
    assert!(def.is_ok(), "Failed to convert to AST: {:?}", def.err());

    let def = def.unwrap();
    // Part def name should NOT have quotes
    assert_eq!(
        def.name,
        Some("Data Packet".to_string()),
        "Definition name should not have quotes"
    );
}
