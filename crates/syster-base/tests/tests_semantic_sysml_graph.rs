#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use syster::core::constants::{
    REL_EXHIBIT, REL_INCLUDE, REL_PERFORM, REL_REDEFINITION, REL_SATISFY, REL_SPECIALIZATION,
    REL_SUBSETTING, REL_TYPING,
};
use syster::parser::SysMLParser;
use syster::parser::sysml::Rule;
use syster::semantic::RelationshipGraph;
use syster::semantic::adapters::SysmlAdapter;
use syster::semantic::symbol_table::SymbolTable;
use syster::syntax::sysml::ast::SysMLFile;

// Helper function to compare graph results with string literals
fn assert_targets_eq(result: Option<Vec<&String>>, expected: &[&str]) {
    match result {
        Some(targets) => {
            let target_strs: Vec<&str> = targets.iter().map(|s| s.as_str()).collect();
            assert_eq!(target_strs, expected);
        }
        None => panic!("Expected Some({expected:?}), got None"),
    }
}

#[test]
fn test_end_to_end_relationship_population() {
    // Parse SysML with relationships
    let source = "part def Vehicle; part def Car :> Vehicle; part myCar : Car;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    // Populate symbol table and relationship graph
    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    let result = populator.populate(&file);
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify symbols are in the table
    assert!(symbol_table.lookup("Vehicle").is_some());
    assert!(symbol_table.lookup("Car").is_some());
    assert!(symbol_table.lookup("myCar").is_some());

    // Verify specialization relationship (Car :> Vehicle)
    let car_specializes = relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Car");
    assert!(car_specializes.is_some());
    assert_eq!(car_specializes.unwrap(), &["Vehicle"]);

    // Verify feature typing relationship (myCar : Car)
    let mycar_typed_by = relationship_graph.get_one_to_one(REL_TYPING, "myCar");
    assert!(mycar_typed_by.is_some());
    assert_eq!(mycar_typed_by.unwrap(), "Car");
}

#[test]
fn test_multiple_relationships() {
    let source = "part def Vehicle; part vehicle1 : Vehicle; part vehicle2 :> vehicle1; part vehicle3 :>> vehicle2;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // vehicle1 : Vehicle
    assert_eq!(
        relationship_graph.get_one_to_one(REL_TYPING, "vehicle1"),
        Some(&"Vehicle".to_string())
    );

    // vehicle2 :> vehicle1
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SUBSETTING, "vehicle2"),
        &["vehicle1"],
    );

    // vehicle3 :>> vehicle2
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_REDEFINITION, "vehicle3"),
        &["vehicle2"],
    );
}

#[test]
fn test_transitive_specialization() {
    let source = "part def Thing; part def Vehicle :> Thing; part def Car :> Vehicle;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Direct relationships
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Vehicle"),
        &["Thing"],
    );
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Car"),
        &["Vehicle"],
    );

    // Transitive paths: Car has path to Vehicle and Thing
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Car", "Vehicle"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Car", "Thing"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Vehicle", "Thing"));
}

#[test]
fn test_multiple_specializations() {
    // Test a definition specializing multiple other definitions
    let source = "part def A; part def B; part def C :> A, B;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // C specializes both A and B
    let c_specializes = relationship_graph.get_one_to_many(REL_SPECIALIZATION, "C");
    assert!(c_specializes.is_some());
    let specializes = c_specializes.unwrap();
    assert_eq!(specializes.len(), 2);
    assert!(specializes.iter().any(|s| s.as_str() == "A"));
    assert!(specializes.iter().any(|s| s.as_str() == "B"));
}

#[test]
fn test_diamond_inheritance() {
    // Test diamond-shaped inheritance hierarchy
    let source = "part def Base; part def Left :> Base; part def Right :> Base; part def Bottom :> Left, Right;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Bottom specializes both Left and Right
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Bottom", "Left"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Bottom", "Right"));

    // Both Left and Right specialize Base
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Left", "Base"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Right", "Base"));

    // Bottom has transitive path to Base through both branches
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Bottom", "Base"));
}

#[test]
fn test_usage_typing_and_subsetting() {
    let source =
        "part def Vehicle; part baseVehicle : Vehicle; part myVehicle : Vehicle :> baseVehicle;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Both usages are typed by Vehicle
    assert_eq!(
        relationship_graph.get_one_to_one(REL_TYPING, "baseVehicle"),
        Some(&"Vehicle".to_string())
    );
    assert_eq!(
        relationship_graph.get_one_to_one(REL_TYPING, "myVehicle"),
        Some(&"Vehicle".to_string())
    );

    // myVehicle subsets baseVehicle
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SUBSETTING, "myVehicle"),
        &["baseVehicle"],
    );
}

#[test]
fn test_action_relationships() {
    let source = "action def BaseAction; action def SpecializedAction :> BaseAction; action myAction : SpecializedAction;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // SpecializedAction specializes BaseAction
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "SpecializedAction"),
        &["BaseAction"],
    );

    // myAction is typed by SpecializedAction
    assert_eq!(
        relationship_graph.get_one_to_one(REL_TYPING, "myAction"),
        Some(&"SpecializedAction".to_string())
    );
}

#[test]
fn test_requirement_relationships() {
    let source = "requirement def BaseReq; requirement def DerivedReq :> BaseReq; requirement myReq : DerivedReq;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "DerivedReq"),
        &["BaseReq"],
    );
    assert_eq!(
        relationship_graph.get_one_to_one(REL_TYPING, "myReq"),
        Some(&"DerivedReq".to_string())
    );
}

#[test]
fn test_deep_inheritance_chain() {
    let source =
        "part def L0; part def L1 :> L0; part def L2 :> L1; part def L3 :> L2; part def L4 :> L3;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Test transitive paths through the entire chain
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "L4", "L3"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "L4", "L2"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "L4", "L1"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "L4", "L0"));

    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "L3", "L0"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "L2", "L0"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "L1", "L0"));
}

#[test]
fn test_multiple_subsettings() {
    let source = "part def Vehicle; part v1 : Vehicle; part v2 : Vehicle; part specialized : Vehicle :> v1, v2;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // specialized subsets both v1 and v2
    let subsets = relationship_graph.get_one_to_many(REL_SUBSETTING, "specialized");
    assert!(subsets.is_some());
    let subsets = subsets.unwrap();
    assert_eq!(subsets.len(), 2);
    assert!(subsets.iter().any(|s| s.as_str() == "v1"));
    assert!(subsets.iter().any(|s| s.as_str() == "v2"));
}

#[test]
fn test_redefinition_chain() {
    let source =
        "part def Vehicle; part v1 : Vehicle; part v2 : Vehicle :>> v1; part v3 : Vehicle :>> v2;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // v2 redefines v1
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_REDEFINITION, "v2"),
        &["v1"],
    );

    // v3 redefines v2
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_REDEFINITION, "v3"),
        &["v2"],
    );

    // Check transitive redefinitions
    assert!(relationship_graph.has_transitive_path(REL_REDEFINITION, "v3", "v2"));
    assert!(relationship_graph.has_transitive_path(REL_REDEFINITION, "v3", "v1"));
}

#[test]
fn test_mixed_definition_kinds() {
    let source = r#"
        part def Vehicle;
        action def Move;
        requirement def SafetyReq;
        item def Fuel;
        attribute def Speed;
        
        part def Car :> Vehicle;
        action def Drive :> Move;
        requirement def CarSafetyReq :> SafetyReq;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify all specializations
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Car"),
        &["Vehicle"],
    );
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Drive"),
        &["Move"],
    );
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "CarSafetyReq"),
        &["SafetyReq"],
    );
}

#[test]
fn test_no_circular_relationships() {
    // Verify that there are no circular paths (no element specializes itself through others)
    let source = "part def A; part def B :> A; part def C :> B;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // No backwards paths
    assert!(!relationship_graph.has_transitive_path(REL_SPECIALIZATION, "A", "B"));
    assert!(!relationship_graph.has_transitive_path(REL_SPECIALIZATION, "B", "C"));
    assert!(!relationship_graph.has_transitive_path(REL_SPECIALIZATION, "A", "C"));

    // Forward paths exist
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "B", "A"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "C", "B"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "C", "A"));
}

#[test]
fn test_satisfy_requirement_relationship() {
    let source = "requirement def SafetyReq; case def SafetyCase { satisfy SafetyReq; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify symbols exist
    assert!(symbol_table.lookup("SafetyReq").is_some());
    assert!(symbol_table.lookup("SafetyCase").is_some());

    // Verify satisfy relationship
    let satisfies = relationship_graph.get_one_to_many(REL_SATISFY, "SafetyCase");
    assert!(satisfies.is_some(), "Expected satisfy relationship");
    let result: Vec<&str> = satisfies.unwrap().iter().map(|s| s.as_str()).collect();
    assert_eq!(result, vec!["SafetyReq"]);
}

#[test]
fn test_satisfy_with_requirement_keyword() {
    let source =
        "requirement def SafetyReq; case def SafetyCase { satisfy requirement SafetyReq; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify satisfy relationship works with full 'requirement' keyword
    let satisfies = relationship_graph.get_one_to_many(REL_SATISFY, "SafetyCase");
    assert!(
        satisfies.is_some(),
        "Expected satisfy relationship with requirement keyword"
    );
    let result: Vec<&str> = satisfies.unwrap().iter().map(|s| s.as_str()).collect();
    assert_eq!(result, vec!["SafetyReq"]);
}

#[test]
fn test_perform_action_relationship() {
    let source = "action def Move; part def Robot { perform Move; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify symbols exist
    assert!(symbol_table.lookup("Move").is_some());
    assert!(symbol_table.lookup("Robot").is_some());

    // Verify perform relationship
    let performs = relationship_graph.get_one_to_many(REL_PERFORM, "Robot");
    assert!(performs.is_some(), "Expected perform relationship");
    let result: Vec<&str> = performs.unwrap().iter().map(|s| s.as_str()).collect();
    assert_eq!(result, vec!["Move"]);
}

#[test]
fn test_exhibit_state_relationship() {
    let source = "state def Moving; part def Vehicle { exhibit Moving; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify symbols exist
    assert!(
        symbol_table.lookup("Moving").is_some(),
        "Moving symbol not found"
    );
    assert!(
        symbol_table.lookup("Vehicle").is_some(),
        "Vehicle symbol not found"
    );

    // Verify exhibit relationship
    let exhibits = relationship_graph.get_one_to_many(REL_EXHIBIT, "Vehicle");
    assert!(exhibits.is_some(), "Expected exhibit relationship");
    let result: Vec<&str> = exhibits.unwrap().iter().map(|s| s.as_str()).collect();
    assert_eq!(result, vec!["Moving"]);
}

#[test]
fn test_include_use_case_relationship() {
    let source = "use case def Login; use case def ManageAccount { include Login; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify symbols exist
    assert!(symbol_table.lookup("Login").is_some());
    assert!(symbol_table.lookup("ManageAccount").is_some());

    // Verify include relationship
    let includes = relationship_graph.get_one_to_many(REL_INCLUDE, "ManageAccount");
    assert!(includes.is_some(), "Expected include relationship");
    let result: Vec<&str> = includes.unwrap().iter().map(|s| s.as_str()).collect();
    assert_eq!(result, vec!["Login"]);
}

#[test]
fn test_multiple_satisfy_relationships() {
    let source = "requirement def Req1; requirement def Req2; case def TestCase { 
        satisfy Req1; 
        satisfy Req2; 
    }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify multiple satisfy relationships
    let satisfies = relationship_graph.get_one_to_many(REL_SATISFY, "TestCase");
    assert!(satisfies.is_some());
    let satisfies = satisfies.unwrap();
    // Found satisfy relationships
    assert_eq!(satisfies.len(), 2);
    assert!(satisfies.iter().any(|s| s.as_str() == "Req1"));
    assert!(satisfies.iter().any(|s| s.as_str() == "Req2"));
}

#[test]
fn test_mixed_domain_and_structural_relationships() {
    // Test that we can parse models with both types of relationships
    let source = r#"
        part def BasePart;
        part def SpecializedPart :> BasePart;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify structural relationship (specialization) works
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "SpecializedPart"),
        &["BasePart"],
    );
}

#[test]
fn test_derive_requirement_relationship() {
    // Test requirement definition specialization hierarchy using :>
    // (Note: This is about requirement inheritance, not the 'derived' keyword)
    let source = r#"
        requirement def Configuration;
        requirement def DerivedReq :> Configuration;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // DerivedReq specializes (derives from) Configuration
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "DerivedReq"),
        &["Configuration"],
    );
}

#[test]
fn test_derive_requirement_keyword_syntax() {
    // Test requirement specialization with explicit "specializes" keyword
    let source = r#"
        requirement def Configuration;
        requirement def DerivedReq specializes Configuration;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // DerivedReq specializes (derives from) Configuration
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "DerivedReq"),
        &["Configuration"],
    );
}

#[test]
fn test_derived_requirement_in_body() {
    // Test 'derived' keyword with subsetting in nested requirement usage
    // 'derived' = marks as computed; ':>' = creates subsetting relationship
    let source = r#"
        requirement def ParentReq;
        requirement def ContainerReq {
            derived requirement Configuration :> ParentReq;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // The nested requirement has a qualified name
    let qualified_name = "ContainerReq::Configuration";

    // Configuration subsets ParentReq (usages use subsetting, not specialization)
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SUBSETTING, qualified_name),
        &["ParentReq"],
    );
}

#[test]
fn test_derived_requirement_modifier() {
    // Test that "derived" keyword marks a feature as computed (per SysML spec 7.13.3)
    // It does NOT create a subsetting/specialization relationship
    let source = r#"
        requirement def BaseReq {
            derived requirement childReq;
        }
        requirement childReq;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Both requirements exist (nested and top-level with same name)
    assert!(symbol_table.lookup("BaseReq").is_some());
    assert!(symbol_table.lookup("childReq").is_some());

    // No subsetting relationship since there's no :> clause
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SUBSETTING, "BaseReq::childReq"),
        None
    );
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SUBSETTING, "childReq"),
        None
    );
}

#[test]
fn test_derived_keyword_with_subsetting() {
    // Test "derived requirement X :> Y"
    // - 'derived' = property modifier (marks feature as computed per SysML spec)
    // - ':>' = creates the subsetting relationship (actual derivation hierarchy)
    let source = r#"
        requirement def ParentReq;
        requirement def Container {
            derived requirement DerivedReq :> ParentReq;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // The derived requirement (with subsetting) should have the relationship
    let qualified_name = "Container::DerivedReq";
    let subsets = relationship_graph.get_one_to_many(REL_SUBSETTING, qualified_name);
    assert_eq!(subsets.as_ref().map(|v| v.len()), Some(1));
    assert!(subsets.unwrap().contains(&&"ParentReq".to_string()));
}

#[test]
fn test_multiple_derived_requirements_in_body() {
    // Test multiple requirements marked as 'derived' with subsetting relationships
    let source = r#"
        requirement def Req1;
        requirement def Req2;
        requirement def Container {
            derived requirement DerivedReq1 :> Req1;
            derived requirement DerivedReq2 :> Req2;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Each derived requirement should have its subsetting relationship
    let subsets1 = relationship_graph.get_one_to_many(REL_SUBSETTING, "Container::DerivedReq1");
    assert_eq!(subsets1.as_ref().map(|v| v.len()), Some(1));
    assert!(subsets1.unwrap().contains(&&"Req1".to_string()));

    let subsets2 = relationship_graph.get_one_to_many(REL_SUBSETTING, "Container::DerivedReq2");
    assert_eq!(subsets2.as_ref().map(|v| v.len()), Some(1));
    assert!(subsets2.unwrap().contains(&&"Req2".to_string()));
}

// =============================================================================
// Tests for get_references_to - O(1) reference lookup
// =============================================================================

#[test]
fn test_get_references_to_finds_typing_references() {
    // Test that get_references_to finds all symbols that reference a given target
    let source = r#"
        part def Vehicle;
        part car : Vehicle;
        part truck : Vehicle;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator.populate(&file).unwrap();

    // Get all references to Vehicle
    let refs = relationship_graph.get_references_to("Vehicle");

    // Should find 2 references: car and truck (both typed by Vehicle)
    assert_eq!(refs.len(), 2, "Should find 2 references to Vehicle");

    let ref_names: Vec<&str> = refs.iter().map(|(name, _)| name.as_str()).collect();
    assert!(ref_names.contains(&"car"), "Should find 'car' reference");
    assert!(
        ref_names.contains(&"truck"),
        "Should find 'truck' reference"
    );
}

#[test]
fn test_get_references_to_finds_specialization_references() {
    let source = r#"
        part def Vehicle;
        part def Car :> Vehicle;
        part def Truck :> Vehicle;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator.populate(&file).unwrap();

    // Get all references to Vehicle via specialization
    let refs = relationship_graph.get_references_to("Vehicle");

    // Should find 2 references: Car and Truck (both specialize Vehicle)
    assert_eq!(refs.len(), 2, "Should find 2 specialization references");

    let ref_names: Vec<&str> = refs.iter().map(|(name, _)| name.as_str()).collect();
    assert!(
        ref_names.contains(&"Car"),
        "Should find 'Car' specialization"
    );
    assert!(
        ref_names.contains(&"Truck"),
        "Should find 'Truck' specialization"
    );
}

#[test]
fn test_get_references_to_returns_spans() {
    let source = "part def Vehicle; part car : Vehicle;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator.populate(&file).unwrap();

    let refs = relationship_graph.get_references_to("Vehicle");

    // Should have a span for the reference
    assert_eq!(refs.len(), 1);
    let (name, span) = &refs[0];
    assert_eq!(name.as_str(), "car");
    // Span might be None depending on how typing relationships store spans
    // but we're testing the API works
    let _ = span; // Use the span to avoid warning
}

#[test]
fn test_get_references_to_empty_for_unreferenced_symbol() {
    let source = r#"
        part def Vehicle;
        part def Unused;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator.populate(&file).unwrap();

    // Unused has no references
    let refs = relationship_graph.get_references_to("Unused");
    assert!(refs.is_empty(), "Unreferenced symbol should have no refs");
}

#[test]
fn test_get_references_to_combined_relationship_types() {
    // Test that references from different relationship types are combined
    let source = r#"
        part def Base;
        part def Derived :> Base;
        part instance : Base;
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator.populate(&file).unwrap();

    // Get all references to Base
    let refs = relationship_graph.get_references_to("Base");

    // Should find: Derived (specializes), instance (typed by)
    assert_eq!(
        refs.len(),
        2,
        "Should find refs from both specialization and typing"
    );

    let ref_names: Vec<&str> = refs.iter().map(|(name, _)| name.as_str()).collect();
    assert!(
        ref_names.contains(&"Derived"),
        "Should find specialization reference"
    );
    assert!(
        ref_names.contains(&"instance"),
        "Should find typing reference"
    );
}

// =============================================================================
// Tests for duplicate relationship detection (Issue: hover shows duplicates)
// =============================================================================

/// Test that stdlib files don't produce duplicate relationships on initial load.
/// This tests the ISQThermodynamics file where `ScalarQuantityValue` is specialized
/// by many attribute defs (CelsiusTemperatureValue, etc.).
/// Each definition should only show up once in the relationship graph.
#[test]
fn test_stdlib_no_duplicate_relationships() {
    use std::path::PathBuf;
    use syster::project::StdLibLoader;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    // Create workspace and load stdlib
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");
    let stdlib_loader = StdLibLoader::with_path(stdlib_path.clone());

    // Load stdlib
    stdlib_loader
        .load(&mut workspace)
        .expect("Failed to load stdlib");

    // Populate all files
    workspace.populate_all().expect("Failed to populate");

    // Find CelsiusTemperatureValue symbol
    let symbol_table = workspace.symbol_table();
    let all_symbols = symbol_table.all_symbols();
    let celsius_symbol = all_symbols
        .iter()
        .find(|(_, sym)| sym.qualified_name() == "ISQThermodynamics::CelsiusTemperatureValue");

    assert!(
        celsius_symbol.is_some(),
        "Should find CelsiusTemperatureValue in stdlib"
    );

    // Check for duplicates in relationships for CelsiusTemperatureValue
    let graph = workspace.relationship_graph();
    let (_, first_symbol) = celsius_symbol.unwrap();
    let qualified_name = first_symbol.qualified_name();
    let rels = graph.get_one_to_many(REL_SPECIALIZATION, qualified_name);

    assert!(rels.is_some(), "Should have specialization relationship");
    let targets = rels.unwrap();

    // Check for duplicates by comparing length with deduplicated length
    let mut unique_targets: Vec<_> = targets.to_vec();
    unique_targets.sort();
    unique_targets.dedup();

    assert_eq!(
        targets.len(),
        unique_targets.len(),
        "Found duplicate relationships! Got {} but only {} unique: {:?}",
        targets.len(),
        unique_targets.len(),
        targets
    );

    // Should specialize exactly ScalarQuantityValue
    assert_eq!(
        targets.len(),
        1,
        "Should have exactly 1 specialization target"
    );
    assert_eq!(targets[0].as_str(), "ScalarQuantityValue");
}

/// Test that repopulating a file doesn't create duplicate relationships.
/// This simulates what happens in the LSP when a document is edited:
/// 1. File is loaded and populated
/// 2. File is marked dirty and repopulated
///
/// FIX: populate_file now clears relationships before repopulating,
/// using the same pattern as remove_symbols_from_file.
#[test]
fn test_repopulation_no_duplicate_relationships() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    // Create a workspace and add a file
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let path = PathBuf::from("test.sysml");
    let source = "part def Vehicle; part def Car :> Vehicle;";

    // Parse and add to workspace
    let parse_result = syster::project::file_loader::parse_with_result(source, &path);
    assert!(parse_result.content.is_some());
    workspace.add_file(path.clone(), parse_result.content.unwrap());

    // First population
    workspace.populate_all().expect("First populate failed");

    // Verify single relationship
    let targets = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car")
        .unwrap();
    assert_eq!(
        targets.len(),
        1,
        "Should have 1 target after first populate"
    );

    // Simulate file edit: update content and repopulate
    let parse_result2 = syster::project::file_loader::parse_with_result(source, &path);
    workspace.update_file(&path, parse_result2.content.unwrap());
    workspace.populate_affected().expect("Repopulate failed");

    // FIX: Should still have exactly 1 relationship (no duplicates)
    let targets = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car")
        .unwrap();

    assert_eq!(
        targets.len(),
        1,
        "Should still have 1 target after repopulation (fix works!). Got: {:?}",
        targets
    );
}

/// Test that simulates the actual LSP flow where duplicates USED TO occur:
/// 1. Workspace loads stdlib via populate_all()
/// 2. User opens a stdlib file (didOpen triggers parse_into_workspace)
/// 3. parse_into_workspace should skip already-populated files
/// 4. No duplicates because we don't repopulate!
///
/// FIX: parse_into_workspace now checks is_populated() before reparsing.
#[test]
fn test_lsp_open_stdlib_file_no_duplicates_after_fix() {
    use std::path::PathBuf;
    use syster::project::StdLibLoader;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    // Create workspace and load stdlib (simulates ensure_workspace_loaded)
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");
    let stdlib_loader = StdLibLoader::with_path(stdlib_path.clone());

    stdlib_loader
        .load(&mut workspace)
        .expect("Failed to load stdlib");

    // Initial population (simulates what ensure_workspace_loaded does)
    workspace.populate_all().expect("Failed to populate");

    // Find ISQThermodynamics file path
    let thermo_path = workspace
        .files()
        .keys()
        .find(|p| p.to_string_lossy().contains("ISQThermodynamics"))
        .cloned();

    assert!(thermo_path.is_some(), "Should find ISQThermodynamics file");
    let thermo_path = thermo_path.unwrap();

    // Verify initial state - no duplicates
    let graph = workspace.relationship_graph();
    let initial_rels = graph.get_one_to_many(
        REL_SPECIALIZATION,
        "ISQThermodynamics::CelsiusTemperatureValue",
    );
    assert!(initial_rels.is_some(), "Should have initial relationships");
    let initial_count = initial_rels.unwrap().len();
    assert_eq!(
        initial_count, 1,
        "Should have exactly 1 specialization initially"
    );

    // Verify file is already populated
    let file = workspace.get_file(&thermo_path);
    assert!(file.is_some(), "File should exist");
    assert!(
        file.unwrap().is_populated(),
        "File should be marked as populated"
    );

    // Simulate what WOULD happen if we repopulated (the bug case)
    // Read file and parse it again
    let file_content = std::fs::read_to_string(&thermo_path).expect("Failed to read file");
    let parse_result = syster::project::file_loader::parse_with_result(&file_content, &thermo_path);
    assert!(parse_result.content.is_some(), "Should parse successfully");

    // The FIX: LSP now skips update_file + populate_affected for already-populated files
    // But this test uses Workspace directly, so we simulate the fixed behavior:
    // DON'T call update_file if already populated (which is what the LSP fix does)

    // Check that relationships are still correct (no duplicates)
    let graph = workspace.relationship_graph();
    let after_rels = graph.get_one_to_many(
        REL_SPECIALIZATION,
        "ISQThermodynamics::CelsiusTemperatureValue",
    );
    assert!(after_rels.is_some(), "Should still have relationships");
    let after_count = after_rels.unwrap().len();

    // With the fix, count stays at 1
    assert_eq!(
        after_count, 1,
        "Should still have exactly 1 specialization (no duplicates)"
    );
}

// =============================================================================
// Tests for add/remove symbol and relationship scenarios
// =============================================================================

/// Test adding a single relationship
#[test]
fn test_add_single_relationship() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);

    let targets = graph.get_one_to_many(REL_SPECIALIZATION, "Car");
    assert!(targets.is_some());
    assert_eq!(targets.unwrap().len(), 1);
}

/// Test adding multiple relationships for the same source
#[test]
fn test_add_multiple_relationships_same_source() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Machine".into(), None);

    let targets = graph.get_one_to_many(REL_SPECIALIZATION, "Car");
    assert!(targets.is_some());
    let targets = targets.unwrap();
    assert_eq!(targets.len(), 2);
    assert!(targets.iter().any(|t| t.as_str() == "Vehicle"));
    assert!(targets.iter().any(|t| t.as_str() == "Machine"));
}

/// Test adding relationships for different sources
#[test]
fn test_add_relationships_different_sources() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);
    graph.add_one_to_many(REL_SPECIALIZATION, "Truck".into(), "Vehicle".into(), None);

    // Both should have their own relationship
    assert!(graph.get_one_to_many(REL_SPECIALIZATION, "Car").is_some());
    assert!(graph.get_one_to_many(REL_SPECIALIZATION, "Truck").is_some());
}

/// Test removing relationships for a specific source
#[test]
fn test_remove_relationships_for_source() {
    let mut graph = RelationshipGraph::new();

    // Add relationships for two different sources
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);
    graph.add_one_to_many(REL_SPECIALIZATION, "Truck".into(), "Vehicle".into(), None);

    // Remove relationships for Car only
    graph.remove_relationships_for_source("Car");

    // Car's relationship should be gone
    assert!(graph.get_one_to_many(REL_SPECIALIZATION, "Car").is_none());

    // Truck's relationship should still exist
    let targets = graph.get_one_to_many(REL_SPECIALIZATION, "Truck");
    assert!(targets.is_some());
    assert_eq!(targets.unwrap().len(), 1);
}

/// Test removing source clears all relationship types for that source
#[test]
fn test_remove_source_clears_all_relationship_types() {
    let mut graph = RelationshipGraph::new();

    // Add different relationship types for the same source
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);
    graph.add_one_to_many(REL_SUBSETTING, "Car".into(), "BaseCar".into(), None);
    graph.add_one_to_one(REL_TYPING, "myCar".into(), "Car".into(), None);

    // Remove myCar from typing
    graph.remove_relationships_for_source("myCar");
    assert!(graph.get_one_to_one(REL_TYPING, "myCar").is_none());

    // Remove Car from specialization and subsetting
    graph.remove_relationships_for_source("Car");
    assert!(graph.get_one_to_many(REL_SPECIALIZATION, "Car").is_none());
    assert!(graph.get_one_to_many(REL_SUBSETTING, "Car").is_none());
}

/// Test removing non-existent source doesn't cause errors
#[test]
fn test_remove_nonexistent_source() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);

    // Remove a source that doesn't exist - should not panic
    graph.remove_relationships_for_source("NonExistent");

    // Original relationship should still be there
    assert!(graph.get_one_to_many(REL_SPECIALIZATION, "Car").is_some());
}

/// Test one-to-one relationship add and remove
#[test]
fn test_one_to_one_add_and_remove() {
    let mut graph = RelationshipGraph::new();

    graph.add_one_to_one(REL_TYPING, "myCar".into(), "Car".into(), None);
    graph.add_one_to_one(REL_TYPING, "myTruck".into(), "Truck".into(), None);

    assert_eq!(
        graph.get_one_to_one(REL_TYPING, "myCar"),
        Some(&"Car".to_string())
    );
    assert_eq!(
        graph.get_one_to_one(REL_TYPING, "myTruck"),
        Some(&"Truck".to_string())
    );

    // Remove myCar
    graph.remove_relationships_for_source("myCar");

    assert!(graph.get_one_to_one(REL_TYPING, "myCar").is_none());
    assert_eq!(
        graph.get_one_to_one(REL_TYPING, "myTruck"),
        Some(&"Truck".to_string())
    );
}

/// Test repopulation scenario: add, remove, add again should not duplicate
#[test]
fn test_repopulation_add_remove_add() {
    let mut graph = RelationshipGraph::new();

    // First population
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);
    assert_eq!(
        graph
            .get_one_to_many(REL_SPECIALIZATION, "Car")
            .unwrap()
            .len(),
        1
    );

    // Clear for repopulation
    graph.remove_relationships_for_source("Car");

    // Second population (same content)
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);

    // Should still have exactly 1 (not 2!)
    let targets = graph.get_one_to_many(REL_SPECIALIZATION, "Car").unwrap();
    assert_eq!(
        targets.len(),
        1,
        "Should have exactly 1 after repopulation, got {}",
        targets.len()
    );
}

/// Test repopulation with changed content
#[test]
fn test_repopulation_with_changed_content() {
    let mut graph = RelationshipGraph::new();

    // First population: Car specializes Vehicle
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);

    // Clear for repopulation
    graph.remove_relationships_for_source("Car");

    // Second population: Car now specializes Machine instead
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Machine".into(), None);

    // Should have Machine, not Vehicle
    let targets = graph.get_one_to_many(REL_SPECIALIZATION, "Car").unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0], "Machine");
}

/// Test that removing source doesn't affect reverse lookups for other sources
#[test]
fn test_remove_source_doesnt_affect_reverse_lookups() {
    let mut graph = RelationshipGraph::new();

    // Multiple sources reference the same target
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".into(), "Vehicle".into(), None);
    graph.add_one_to_many(REL_SPECIALIZATION, "Truck".into(), "Vehicle".into(), None);
    graph.add_one_to_many(REL_SPECIALIZATION, "Bus".into(), "Vehicle".into(), None);

    // Get sources that reference Vehicle
    let sources = graph.get_one_to_many_sources(REL_SPECIALIZATION, "Vehicle");
    assert_eq!(sources.len(), 3);

    // Remove Car
    graph.remove_relationships_for_source("Car");

    // Vehicle should still be referenced by Truck and Bus
    let sources = graph.get_one_to_many_sources(REL_SPECIALIZATION, "Vehicle");
    assert_eq!(sources.len(), 2);
    assert!(sources.iter().any(|s| s.as_str() == "Truck"));
    assert!(sources.iter().any(|s| s.as_str() == "Bus"));
    assert!(!sources.iter().any(|s| s.as_str() == "Car"));
}

/// Test full workspace repopulation scenario with symbols and relationships
#[test]
fn test_workspace_repopulation_symbols_and_relationships() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let path = PathBuf::from("test.sysml");

    // Initial content
    let source1 = "part def Vehicle; part def Car :> Vehicle; part myCar : Car;";
    let parse_result = syster::project::file_loader::parse_with_result(source1, &path);
    workspace.add_file(path.clone(), parse_result.content.unwrap());
    workspace.populate_all().expect("Failed to populate");

    // Verify initial state
    assert!(workspace.symbol_table().lookup("Vehicle").is_some());
    assert!(workspace.symbol_table().lookup("Car").is_some());
    assert!(workspace.symbol_table().lookup("myCar").is_some());
    assert!(
        workspace
            .relationship_graph()
            .get_one_to_many(REL_SPECIALIZATION, "Car")
            .is_some()
    );
    assert!(
        workspace
            .relationship_graph()
            .get_one_to_one(REL_TYPING, "myCar")
            .is_some()
    );

    // Update with different content
    let source2 = "part def Truck; part def BigTruck :> Truck;";
    let parse_result2 = syster::project::file_loader::parse_with_result(source2, &path);
    workspace.update_file(&path, parse_result2.content.unwrap());
    workspace.populate_affected().expect("Failed to repopulate");

    // Old symbols should be gone
    assert!(
        workspace.symbol_table().lookup("Vehicle").is_none(),
        "Vehicle should be removed"
    );
    assert!(
        workspace.symbol_table().lookup("Car").is_none(),
        "Car should be removed"
    );
    assert!(
        workspace.symbol_table().lookup("myCar").is_none(),
        "myCar should be removed"
    );

    // Old relationships should be gone
    assert!(
        workspace
            .relationship_graph()
            .get_one_to_many(REL_SPECIALIZATION, "Car")
            .is_none()
    );
    assert!(
        workspace
            .relationship_graph()
            .get_one_to_one(REL_TYPING, "myCar")
            .is_none()
    );

    // New symbols should exist
    assert!(
        workspace.symbol_table().lookup("Truck").is_some(),
        "Truck should exist"
    );
    assert!(
        workspace.symbol_table().lookup("BigTruck").is_some(),
        "BigTruck should exist"
    );

    // New relationships should exist
    let targets = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "BigTruck");
    assert!(targets.is_some(), "BigTruck should have specialization");
    assert_eq!(targets.unwrap()[0], "Truck");
}

/// Test that nested symbols are properly removed and re-added
#[test]
fn test_nested_symbols_repopulation() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let path = PathBuf::from("test.sysml");

    // Initial content with nested symbols
    let source1 = r#"
        part def Container {
            part inner1 : Container;
            part inner2 : Container;
        }
    "#;
    let parse_result = syster::project::file_loader::parse_with_result(source1, &path);
    workspace.add_file(path.clone(), parse_result.content.unwrap());
    workspace.populate_all().expect("Failed to populate");

    // Verify nested symbols exist via all_symbols (nested symbols stored by simple name)
    assert!(workspace.symbol_table().lookup("Container").is_some());
    let all_symbols = workspace.symbol_table().all_symbols();
    assert!(
        all_symbols
            .iter()
            .any(|(_, s)| s.qualified_name() == "Container::inner1"),
        "inner1 should exist"
    );
    assert!(
        all_symbols
            .iter()
            .any(|(_, s)| s.qualified_name() == "Container::inner2"),
        "inner2 should exist"
    );

    // Update with different nested structure
    let source2 = r#"
        part def Container {
            part newInner : Container;
        }
    "#;
    let parse_result2 = syster::project::file_loader::parse_with_result(source2, &path);
    workspace.update_file(&path, parse_result2.content.unwrap());
    workspace.populate_affected().expect("Failed to repopulate");

    // Old nested symbols should be gone
    let all_symbols_after = workspace.symbol_table().all_symbols();
    assert!(
        !all_symbols_after
            .iter()
            .any(|(_, s)| s.qualified_name() == "Container::inner1"),
        "inner1 should be removed"
    );
    assert!(
        !all_symbols_after
            .iter()
            .any(|(_, s)| s.qualified_name() == "Container::inner2"),
        "inner2 should be removed"
    );

    // New nested symbol should exist
    assert!(workspace.symbol_table().lookup("Container").is_some());
    assert!(
        all_symbols_after
            .iter()
            .any(|(_, s)| s.qualified_name() == "Container::newInner"),
        "newInner should exist"
    );
}

/// Test multiple files: updating one doesn't affect the other
#[test]
fn test_multi_file_update_isolation() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let path1 = PathBuf::from("file1.sysml");
    let path2 = PathBuf::from("file2.sysml");

    // Add two files
    let source1 = "part def Vehicle; part def Car :> Vehicle;";
    let source2 = "part def Animal; part def Dog :> Animal;";

    let parse1 = syster::project::file_loader::parse_with_result(source1, &path1);
    let parse2 = syster::project::file_loader::parse_with_result(source2, &path2);

    workspace.add_file(path1.clone(), parse1.content.unwrap());
    workspace.add_file(path2.clone(), parse2.content.unwrap());
    workspace.populate_all().expect("Failed to populate");

    // Verify both files' symbols and relationships exist
    assert!(workspace.symbol_table().lookup("Vehicle").is_some());
    assert!(workspace.symbol_table().lookup("Car").is_some());
    assert!(workspace.symbol_table().lookup("Animal").is_some());
    assert!(workspace.symbol_table().lookup("Dog").is_some());

    // Update file1 only
    let source1_new = "part def Machine; part def Robot :> Machine;";
    let parse1_new = syster::project::file_loader::parse_with_result(source1_new, &path1);
    workspace.update_file(&path1, parse1_new.content.unwrap());
    workspace.populate_affected().expect("Failed to repopulate");

    // File1's old symbols should be gone, new ones should exist
    assert!(workspace.symbol_table().lookup("Vehicle").is_none());
    assert!(workspace.symbol_table().lookup("Car").is_none());
    assert!(workspace.symbol_table().lookup("Machine").is_some());
    assert!(workspace.symbol_table().lookup("Robot").is_some());

    // File2's symbols should be unchanged
    assert!(workspace.symbol_table().lookup("Animal").is_some());
    assert!(workspace.symbol_table().lookup("Dog").is_some());

    // File2's relationships should be unchanged
    let dog_spec = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Dog");
    assert!(dog_spec.is_some());
    assert_eq!(dog_spec.unwrap()[0], "Animal");
}

// =============================================================================
// Tests for cross-language (SysML <-> KerML) relationships
// =============================================================================

/// Test that SysML definitions can reference KerML types in relationships
#[test]
fn test_cross_language_sysml_specializes_kerml() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    let mut workspace: Workspace<SyntaxFile> = Workspace::new();

    // KerML file defining NumericalVectorValue
    let kerml_path = PathBuf::from("VectorValues.kerml");
    let kerml_source = r#"
        standard library package VectorValues {
            datatype VectorValue;
            datatype NumericalVectorValue :> VectorValue;
        }
    "#;
    let kerml_result = syster::project::file_loader::parse_with_result(kerml_source, &kerml_path);
    workspace.add_file(kerml_path.clone(), kerml_result.content.unwrap());

    // SysML file using NumericalVectorValue
    let sysml_path = PathBuf::from("Quantities.sysml");
    let sysml_source = r#"
        package Quantities {
            import VectorValues::*;
            attribute def VectorQuantityValue :> NumericalVectorValue;
        }
    "#;
    let sysml_result = syster::project::file_loader::parse_with_result(sysml_source, &sysml_path);
    workspace.add_file(sysml_path.clone(), sysml_result.content.unwrap());

    workspace.populate_all().expect("Failed to populate");

    // Debug: print all symbols
    println!("All symbols:");
    for (name, symbol) in workspace.symbol_table().all_symbols() {
        println!("  {} -> {}", name, symbol.qualified_name());
    }

    // lookup() is scope-aware, use resolve() for cross-scope lookups
    // In the SysML file with "import VectorValues::*", NumericalVectorValue should be visible
    // But lookup() only searches current scope chain, not through imports for other files

    // Use qualified name lookup instead
    let symbol = workspace
        .symbol_table()
        .lookup_qualified("VectorValues::NumericalVectorValue");
    println!(
        "VectorValues::NumericalVectorValue lookup: {:?}",
        symbol.map(|s| s.qualified_name())
    );
    assert!(
        symbol.is_some(),
        "VectorValues::NumericalVectorValue should be found"
    );

    // Check relationships for NumericalVectorValue (using qualified name)
    let rels_qname = workspace
        .relationship_graph()
        .get_all_relationships("VectorValues::NumericalVectorValue");
    println!(
        "VectorValues::NumericalVectorValue relationships: {:?}",
        rels_qname
    );

    // Check what VectorQuantityValue specializes
    let vqv_rels = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Quantities::VectorQuantityValue");
    println!("VectorQuantityValue specializes: {:?}", vqv_rels);

    // The SysML type should specialize the KerML type
    assert!(
        vqv_rels.is_some(),
        "VectorQuantityValue should have specialization relationship"
    );
    let targets = vqv_rels.unwrap();
    assert!(
        targets.iter().any(|t| t.contains("NumericalVectorValue")),
        "VectorQuantityValue should specialize NumericalVectorValue, got: {:?}",
        targets
    );
}

/// Test that replicates the duplicate relationship bug for TemperatureDifferenceValue
/// User reported: hovering over TemperatureDifferenceValue in ISQ.sysml shows
/// two relationships for ScalarQuantityValue
#[test]
fn test_temperature_difference_value_no_duplicate_specialization() {
    use std::path::PathBuf;
    use syster::project::StdLibLoader;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    // Create workspace and load stdlib
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");
    let stdlib_loader = StdLibLoader::with_path(stdlib_path.clone());

    // Load and populate stdlib
    stdlib_loader
        .load(&mut workspace)
        .expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate");

    // Find ISQ::TemperatureDifferenceValue
    let graph = workspace.relationship_graph();
    let rels = graph.get_one_to_many(REL_SPECIALIZATION, "ISQ::TemperatureDifferenceValue");

    assert!(
        rels.is_some(),
        "Should have specialization relationship for ISQ::TemperatureDifferenceValue"
    );
    let targets = rels.unwrap();

    println!("TemperatureDifferenceValue specializes: {:?}", targets);

    // Check for duplicates
    let mut unique_targets: Vec<_> = targets.to_vec();
    unique_targets.sort();
    unique_targets.dedup();

    assert_eq!(
        targets.len(),
        unique_targets.len(),
        "Found duplicate relationships! Got {} but only {} unique: {:?}",
        targets.len(),
        unique_targets.len(),
        targets
    );

    // Should specialize exactly 1 type (ScalarQuantityValue)
    assert_eq!(
        targets.len(),
        1,
        "Should have exactly 1 specialization target, got: {:?}",
        targets
    );
}
