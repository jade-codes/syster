#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use syster::language::sysml::SymbolTablePopulator;
use syster::language::sysml::populator::{
    REL_EXHIBIT, REL_INCLUDE, REL_PERFORM, REL_REDEFINITION, REL_SATISFY, REL_SPECIALIZATION,
    REL_SUBSETTING, REL_TYPING,
};
use syster::language::sysml::syntax::Element;
use syster::language::sysml::syntax::SysMLFile;
use syster::parser::{SysMLParser, sysml::Rule};
use syster::semantic::RelationshipGraph;
use syster::semantic::symbol_table::SymbolTable;

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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // vehicle1 : Vehicle
    assert_eq!(
        relationship_graph.get_one_to_one(REL_TYPING, "vehicle1"),
        Some(&"Vehicle".to_string())
    );

    // vehicle2 :> vehicle1
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SUBSETTING, "vehicle2"),
        Some(&["vehicle1".to_string()][..])
    );

    // vehicle3 :>> vehicle2
    assert_eq!(
        relationship_graph.get_one_to_many(REL_REDEFINITION, "vehicle3"),
        Some(&["vehicle2".to_string()][..])
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Direct relationships
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Vehicle"),
        Some(&["Thing".to_string()][..])
    );
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Car"),
        Some(&["Vehicle".to_string()][..])
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // C specializes both A and B
    let c_specializes = relationship_graph.get_one_to_many(REL_SPECIALIZATION, "C");
    assert!(c_specializes.is_some());
    let specializes = c_specializes.unwrap();
    assert_eq!(specializes.len(), 2);
    assert!(specializes.contains(&"A".to_string()));
    assert!(specializes.contains(&"B".to_string()));
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

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
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SUBSETTING, "myVehicle"),
        Some(&["baseVehicle".to_string()][..])
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // SpecializedAction specializes BaseAction
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "SpecializedAction"),
        Some(&["BaseAction".to_string()][..])
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "DerivedReq"),
        Some(&["BaseReq".to_string()][..])
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // specialized subsets both v1 and v2
    let subsets = relationship_graph.get_one_to_many(REL_SUBSETTING, "specialized");
    assert!(subsets.is_some());
    let subsets = subsets.unwrap();
    assert_eq!(subsets.len(), 2);
    assert!(subsets.contains(&"v1".to_string()));
    assert!(subsets.contains(&"v2".to_string()));
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // v2 redefines v1
    assert_eq!(
        relationship_graph.get_one_to_many(REL_REDEFINITION, "v2"),
        Some(&["v1".to_string()][..])
    );

    // v3 redefines v2
    assert_eq!(
        relationship_graph.get_one_to_many(REL_REDEFINITION, "v3"),
        Some(&["v2".to_string()][..])
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify all specializations
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Car"),
        Some(&["Vehicle".to_string()][..])
    );
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Drive"),
        Some(&["Move".to_string()][..])
    );
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "CarSafetyReq"),
        Some(&["SafetyReq".to_string()][..])
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify symbols exist
    assert!(symbol_table.lookup("SafetyReq").is_some());
    assert!(symbol_table.lookup("SafetyCase").is_some());

    // Verify satisfy relationship
    let satisfies = relationship_graph.get_one_to_many(REL_SATISFY, "SafetyCase");
    assert!(satisfies.is_some(), "Expected satisfy relationship");
    assert_eq!(satisfies.unwrap(), &["SafetyReq".to_string()][..]);
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify satisfy relationship works with full 'requirement' keyword
    let satisfies = relationship_graph.get_one_to_many(REL_SATISFY, "SafetyCase");
    assert!(
        satisfies.is_some(),
        "Expected satisfy relationship with requirement keyword"
    );
    assert_eq!(satisfies.unwrap(), &["SafetyReq".to_string()][..]);
}

#[test]
fn test_perform_action_relationship() {
    let source = "action def Move; part def Robot { perform Move; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify symbols exist
    assert!(symbol_table.lookup("Move").is_some());
    assert!(symbol_table.lookup("Robot").is_some());

    // Verify perform relationship
    let performs = relationship_graph.get_one_to_many(REL_PERFORM, "Robot");
    assert!(performs.is_some(), "Expected perform relationship");
    assert_eq!(performs.unwrap(), &["Move".to_string()][..]);
}

#[test]
fn test_exhibit_state_relationship() {
    let source = "state def Moving; part def Vehicle { exhibit Moving; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

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
    assert_eq!(exhibits.unwrap(), &["Moving".to_string()][..]);
}

#[test]
fn test_include_use_case_relationship() {
    let source = "use case def Login; use case def ManageAccount { include Login; }";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify symbols exist
    assert!(symbol_table.lookup("Login").is_some());
    assert!(symbol_table.lookup("ManageAccount").is_some());

    // Verify include relationship
    let includes = relationship_graph.get_one_to_many(REL_INCLUDE, "ManageAccount");
    assert!(includes.is_some(), "Expected include relationship");
    assert_eq!(includes.unwrap(), &["Login".to_string()][..]);
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify multiple satisfy relationships
    let satisfies = relationship_graph.get_one_to_many(REL_SATISFY, "TestCase");
    assert!(satisfies.is_some());
    let satisfies = satisfies.unwrap();
    println!(
        "Found {} satisfy relationships: {:?}",
        satisfies.len(),
        satisfies
    );
    assert_eq!(satisfies.len(), 2);
    assert!(satisfies.contains(&"Req1".to_string()));
    assert!(satisfies.contains(&"Req2".to_string()));
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
        SymbolTablePopulator::with_relationships(&mut symbol_table, &mut relationship_graph);

    populator.populate(&file).unwrap();

    // Verify structural relationship (specialization) works
    assert_eq!(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "SpecializedPart"),
        Some(&["BasePart".to_string()][..])
    );
}
