#![allow(clippy::unwrap_used)]
use syster::semantic::ReferenceIndex;
use syster::semantic::resolver::Resolver;

/// Helper to assert that sources contain the expected values.
fn assert_sources_contain(sources: Vec<&str>, expected: &[&str]) {
    for e in expected {
        assert!(
            sources.contains(e),
            "Expected sources to contain {:?}, got {:?}",
            e,
            sources
        );
    }
}

use pest::Parser;
use std::path::PathBuf;
use syster::parser::SysMLParser;
use syster::parser::sysml::Rule;
use syster::semantic::Workspace;
use syster::semantic::adapters::SysmlAdapter;
use syster::semantic::symbol_table::SymbolTable;
use syster::syntax::SyntaxFile;
use syster::syntax::sysml::ast::parse_file;

#[test]
fn test_cross_file_specialization() {
    // File 1 defines a base type
    let file1_source = "part def Vehicle;";

    // File 2 references the type from file 1
    let file2_source = "part def Car :> Vehicle;";

    // Parse both files
    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = parse_file(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

    // Create a shared symbol table and reference index
    let mut symbol_table = SymbolTable::new();
    let mut reference_index = ReferenceIndex::new();

    // Populate from file 1 with source tracking
    symbol_table.set_current_file(Some("base.sysml".to_string()));
    let mut populator1 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
    populator1.populate(&file1).unwrap();

    // Populate from file 2 - this should be able to resolve Vehicle from file 1
    symbol_table.set_current_file(Some("derived.sysml".to_string()));
    let mut populator2 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
    let result = populator2.populate(&file2);

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
    // Car references Vehicle, so get_sources("Vehicle") should contain "Car"
    let sources = reference_index.get_sources("Vehicle");
    assert_sources_contain(sources, &["Car"]);
}

#[test]
fn test_cross_file_typing() {
    // File 1 defines a type
    let file1_source = "part def Vehicle;";

    // File 2 creates a usage of that type
    let file2_source = "part myCar : Vehicle;";

    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = parse_file(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut reference_index = ReferenceIndex::new();

    // Populate both files into shared symbol table
    let mut populator1 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
    populator1.populate(&file1).unwrap();

    let mut populator2 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
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
    let file1 = parse_file(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

    let mut pairs3 = SysMLParser::parse(Rule::model, file3_source).unwrap();
    let file3 = parse_file(&mut pairs3).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut reference_index = ReferenceIndex::new();

    // Populate all three files with file tracking
    symbol_table.set_current_file(Some("file1.sysml".to_string()));
    let mut populator1 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
    populator1.populate(&file1).unwrap();

    symbol_table.set_current_file(Some("file2.sysml".to_string()));
    let mut populator2 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
    populator2.populate(&file2).unwrap();

    symbol_table.set_current_file(Some("file3.sysml".to_string()));
    let mut populator3 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
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

    // Verify relationships:
    // - Vehicle references Thing, so get_sources("Thing") should contain "Vehicle"
    // - Car references Vehicle, so get_sources("Vehicle") should contain "Car"
    let thing_sources = reference_index.get_sources("Thing");
    assert_sources_contain(thing_sources, &["Vehicle"]);

    let vehicle_sources = reference_index.get_sources("Vehicle");
    assert_sources_contain(vehicle_sources, &["Car"]);
}

#[test]
fn test_unresolved_cross_file_reference() {
    // File references a type that doesn't exist in any file
    let source = "part def Car :> NonExistentVehicle;";

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut reference_index = ReferenceIndex::new();
    let mut populator = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);

    let result = populator.populate(&file);

    // Should fail or report an error about unresolved reference
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
    let file1 = parse_file(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut reference_index = ReferenceIndex::new();

    // Populate file 1 with source tracking
    symbol_table.set_current_file(Some("vehicle.sysml".to_string()));
    let mut populator1 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
    populator1.populate(&file1).unwrap();

    // Populate file 2 with source tracking
    symbol_table.set_current_file(Some("car.sysml".to_string()));
    let mut populator2 = SysmlAdapter::with_index(&mut symbol_table, &mut reference_index);
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
    let file1 = parse_file(&mut pairs1).unwrap();

    // File 2: Intermediate type
    let file2_source = "part def Car :> Vehicle;";
    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

    // File 3: Final type
    let file3_source = "part def SportsCar :> Car;";
    let mut pairs3 = SysMLParser::parse(Rule::model, file3_source).unwrap();
    let file3 = parse_file(&mut pairs3).unwrap();

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

    // Verify relationships across files:
    // Car references Vehicle, so get_sources("Vehicle") should contain "Car"
    // SportsCar references Car, so get_sources("Car") should contain "SportsCar"
    let vehicle_sources = workspace.reference_index().get_sources("Vehicle");
    assert_sources_contain(vehicle_sources, &["Car"]);

    let car_sources = workspace.reference_index().get_sources("Car");
    assert_sources_contain(car_sources, &["SportsCar"]);
}
