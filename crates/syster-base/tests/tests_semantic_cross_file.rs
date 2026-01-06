#![allow(clippy::unwrap_used)]
use syster::semantic::resolver::Resolver;

fn assert_targets_eq(result: Option<Vec<&str>>, expected: &[&str]) {
    match result {
        Some(targets) => {
            assert_eq!(targets, expected);
        }
        None => panic!("Expected Some({expected:?}), got None"),
    }
}

use from_pest::FromPest;
use pest::Parser;
use std::path::PathBuf;
use syster::core::constants::REL_SPECIALIZATION;
use syster::parser::SysMLParser;
use syster::parser::sysml::Rule;
use syster::semantic::adapters::SysmlAdapter;
use syster::semantic::symbol_table::SymbolTable;
use syster::semantic::{RelationshipGraph, Workspace};
use syster::syntax::SyntaxFile;
use syster::syntax::sysml::ast::SysMLFile;

#[test]
fn test_cross_file_specialization() {
    // File 1 defines a base type
    let file1_source = "part def Vehicle;";

    // File 2 references the type from file 1
    let file2_source = "part def Car :> Vehicle;";

    // Parse both files
    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    // Create a shared symbol table and relationship graph
    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Populate from file 1 with source tracking
    symbol_table.set_current_file(Some("base.sysml".to_string()));
    let mut populator1 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator1.populate(&file1).unwrap();

    // Populate from file 2 - this should be able to resolve Vehicle from file 1
    symbol_table.set_current_file(Some("derived.sysml".to_string()));
    let mut populator2 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    let result = populator2.populate(&file2);

    // This test will fail initially because cross-file resolution isn't implemented
    assert!(
        result.is_ok(),
        "Failed to populate file 2: {:?}",
        result.err()
    );

    // Verify both symbols are in the table
    assert!(Resolver::new(&symbol_table).resolve("Vehicle").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Car").is_some());

    // Verify source files are tracked
    assert_eq!(
        Resolver::new(&symbol_table)
            .resolve("Vehicle")
            .unwrap()
            .source_file(),
        Some("base.sysml")
    );
    assert_eq!(
        Resolver::new(&symbol_table)
            .resolve("Car")
            .unwrap()
            .source_file(),
        Some("derived.sysml")
    );

    // Verify the specialization relationship was created
    assert_targets_eq(
        relationship_graph.get_one_to_many(REL_SPECIALIZATION, "Car"),
        &["Vehicle"],
    );
}

#[test]
fn test_cross_file_typing() {
    // File 1 defines a type
    let file1_source = "part def Vehicle;";

    // File 2 creates a usage of that type
    let file2_source = "part myCar : Vehicle;";

    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Populate both files into shared symbol table
    let mut populator1 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator1.populate(&file1).unwrap();

    let mut populator2 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    let result = populator2.populate(&file2);

    assert!(
        result.is_ok(),
        "Failed to populate file 2: {:?}",
        result.err()
    );
    assert!(Resolver::new(&symbol_table).resolve("Vehicle").is_some());
    assert!(Resolver::new(&symbol_table).resolve("myCar").is_some());
}

#[test]
fn test_cross_file_transitive_relationships() {
    // File 1: Base type
    let file1_source = "part def Thing;";

    // File 2: Intermediate type specializing Thing
    let file2_source = "part def Vehicle :> Thing;";

    // File 3: Final type specializing Vehicle
    let file3_source = "part def Car :> Vehicle;";

    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    let mut pairs3 = SysMLParser::parse(Rule::model, file3_source).unwrap();
    let file3 = SysMLFile::from_pest(&mut pairs3).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Populate all three files with file tracking
    symbol_table.set_current_file(Some("file1.sysml".to_string()));
    let mut populator1 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator1.populate(&file1).unwrap();

    symbol_table.set_current_file(Some("file2.sysml".to_string()));
    let mut populator2 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator2.populate(&file2).unwrap();

    symbol_table.set_current_file(Some("file3.sysml".to_string()));
    let mut populator3 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    let result = populator3.populate(&file3);

    assert!(
        result.is_ok(),
        "Failed to populate file 3: {:?}",
        result.err()
    );

    // Verify all symbols exist with correct source files
    let resolver = Resolver::new(&symbol_table);
    let thing_symbol = resolver.resolve("Thing").unwrap();
    assert_eq!(thing_symbol.source_file(), Some("file1.sysml"));

    let resolver = Resolver::new(&symbol_table);
    let vehicle_symbol = resolver.resolve("Vehicle").unwrap();
    assert_eq!(vehicle_symbol.source_file(), Some("file2.sysml"));

    let resolver = Resolver::new(&symbol_table);
    let car_symbol = resolver.resolve("Car").unwrap();
    assert_eq!(car_symbol.source_file(), Some("file3.sysml"));

    // Verify transitive relationships across files
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Car", "Vehicle"));
    assert!(relationship_graph.has_transitive_path(REL_SPECIALIZATION, "Car", "Thing"));
}

#[test]
fn test_unresolved_cross_file_reference() {
    // File references a type that doesn't exist in any file
    let source = "part def Car :> NonExistentVehicle;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let mut populator =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);

    let result = populator.populate(&file);

    // Should fail or report an error about unresolved reference
    // (Depending on how we want to handle this - might be a warning instead)
    assert!(
        result.is_err()
            || Resolver::new(&symbol_table)
                .resolve("NonExistentVehicle")
                .is_none()
    );
}

#[test]
fn test_symbol_source_tracking() {
    // This test demonstrates tracking which file each symbol comes from
    let file1_source = "part def Vehicle;";
    let file2_source = "part def Car :> Vehicle;";

    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Populate file 1 with source tracking
    symbol_table.set_current_file(Some("vehicle.sysml".to_string()));
    let mut populator1 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator1.populate(&file1).unwrap();

    // Populate file 2 with source tracking
    symbol_table.set_current_file(Some("car.sysml".to_string()));
    let mut populator2 =
        SysmlAdapter::with_relationships(&mut symbol_table, &mut relationship_graph);
    populator2.populate(&file2).unwrap();

    // We can now query which file a symbol came from
    let resolver = Resolver::new(&symbol_table);
    let vehicle_symbol = resolver.resolve("Vehicle").unwrap();
    let resolver = Resolver::new(&symbol_table);
    let car_symbol = resolver.resolve("Car").unwrap();

    assert_eq!(vehicle_symbol.source_file(), Some("vehicle.sysml"));
    assert_eq!(car_symbol.source_file(), Some("car.sysml"));
    assert_eq!(vehicle_symbol.name(), "Vehicle");
    assert_eq!(car_symbol.name(), "Car");
}

#[test]
fn test_workspace_with_file_paths() {
    // Test a proper workspace setup where each file has an identifier
    // and we can track dependencies and provide better error messages

    let mut workspace = Workspace::<SyntaxFile>::new();

    // File 1: Base type
    let file1_source = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    // File 2: Intermediate type
    let file2_source = "part def Car :> Vehicle;";
    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    // File 3: Final type
    let file3_source = "part def SportsCar :> Car;";
    let mut pairs3 = SysMLParser::parse(Rule::model, file3_source).unwrap();
    let file3 = SysMLFile::from_pest(&mut pairs3).unwrap();

    // Add files to workspace
    workspace.add_file(
        PathBuf::from("base/vehicle.sysml"),
        syster::syntax::SyntaxFile::SysML(file1),
    );
    workspace.add_file(
        PathBuf::from("derived/car.sysml"),
        syster::syntax::SyntaxFile::SysML(file2),
    );
    workspace.add_file(
        PathBuf::from("derived/sports_car.sysml"),
        syster::syntax::SyntaxFile::SysML(file3),
    );

    // Verify file count
    assert_eq!(workspace.file_count(), 3);

    // Populate all files
    let result = workspace.populate_all();
    assert!(
        result.is_ok(),
        "Failed to populate workspace: {:?}",
        result.err()
    );

    // Verify all symbols are in the shared symbol table with correct source files
    let resolver = Resolver::new(workspace.symbol_table());
    let vehicle = resolver.resolve("Vehicle").unwrap();
    assert_eq!(vehicle.source_file(), Some("base/vehicle.sysml"));

    let car = resolver.resolve("Car").unwrap();
    assert_eq!(car.source_file(), Some("derived/car.sysml"));

    let sports_car = resolver.resolve("SportsCar").unwrap();
    assert_eq!(sports_car.source_file(), Some("derived/sports_car.sysml"));

    // Verify relationships across files
    assert_targets_eq(
        workspace
            .relationship_graph()
            .get_one_to_many(REL_SPECIALIZATION, "Car"),
        &["Vehicle"],
    );
    assert_targets_eq(
        workspace
            .relationship_graph()
            .get_one_to_many(REL_SPECIALIZATION, "SportsCar"),
        &["Car"],
    );

    // Verify transitive relationships
    assert!(workspace.relationship_graph().has_transitive_path(
        REL_SPECIALIZATION,
        "SportsCar",
        "Car"
    ));
    assert!(workspace.relationship_graph().has_transitive_path(
        REL_SPECIALIZATION,
        "SportsCar",
        "Vehicle"
    ));
}
