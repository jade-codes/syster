#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use syster::core::constants::*;
use syster::parser::{SysMLParser, sysml::Rule};
use syster::semantic::RelationshipGraph;
use syster::semantic::adapters::SysmlAdapter;
use syster::semantic::symbol_table::{Symbol, SymbolTable};
use syster::syntax::sysml::ast::SysMLFile;

#[test]
fn test_visitor_creates_package_symbol() {
    let source = "package MyPackage;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("MyPackage").is_some());
}

#[test]
fn test_visitor_creates_definition_symbol() {
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Vehicle").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Part"),
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_qualified_redefinition_does_not_create_duplicate_symbols() {
    let source = r#"
        package TestPkg {
            item def Shell {
                item edges {
                    item vertices;
                }
            }
            
            item def Disc :> Shell {
                item :>> edges {
                    ref item :>> Shell::edges::vertices;
                }
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    // This should not produce duplicate symbol errors
    let result = adapter.populate(&file);
    assert!(
        result.is_ok(),
        "Should not have errors, got: {:?}",
        result.err()
    );

    // Shell should be defined exactly once
    let all_symbols = symbol_table.all_symbols();
    let shell_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "Shell")
        .count();
    assert_eq!(
        shell_count, 1,
        "Shell should be defined exactly once, got {shell_count} definitions"
    );
}

#[test]
fn test_same_name_in_different_namespaces_creates_two_symbols() {
    let source = r#"
        package Namespace1 {
            item def Shell;
        }
        
        package Namespace2 {
            item def Shell;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);
    assert!(
        result.is_ok(),
        "Should not have errors, got: {:?}",
        result.err()
    );

    // Should have two Shell symbols, one in each namespace
    let all_symbols = symbol_table.all_symbols();
    let shell_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(name, _)| *name == "Shell")
        .collect();

    assert_eq!(
        shell_symbols.len(),
        2,
        "Should have exactly 2 Shell definitions in different namespaces, got {}",
        shell_symbols.len()
    );

    // Verify they have different qualified names
    let qualified_names: Vec<String> = shell_symbols
        .iter()
        .filter_map(|(_, symbol)| match symbol {
            Symbol::Definition { qualified_name, .. } => Some(qualified_name.clone()),
            _ => None,
        })
        .collect();

    assert!(qualified_names.contains(&"Namespace1::Shell".to_string()));
    assert!(qualified_names.contains(&"Namespace2::Shell".to_string()));
}

#[test]
fn test_comma_separated_redefinitions_do_not_create_duplicate_symbols() {
    let source = r#"
        package TestPkg {
            item def Disc {
                attribute semiMajorAxis;
                attribute semiMinorAxis;
                item shape {
                    attribute semiMajorAxis;
                    attribute semiMinorAxis;
                }
            }
            
            item def Circle {
                attribute semiMajorAxis;
                attribute semiMinorAxis;
            }
            
            item def CircularDisc :> Disc {
                item :>> shape : Circle {
                    attribute :>> Disc::shape::semiMajorAxis, Circle::semiMajorAxis;
                    attribute :>> Disc::shape::semiMinorAxis, Circle::semiMinorAxis;
                }
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);
    assert!(
        result.is_ok(),
        "Should not have errors from comma-separated redefinitions, got: {:?}",
        result.err()
    );

    // Disc and Circle should each be defined exactly once
    let all_symbols = symbol_table.all_symbols();
    let disc_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "Disc")
        .count();
    let circle_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "Circle")
        .count();

    assert_eq!(
        disc_count, 1,
        "Disc should be defined exactly once, got {disc_count} definitions"
    );
    assert_eq!(
        circle_count, 1,
        "Circle should be defined exactly once, got {circle_count} definitions"
    );
}

#[test]
fn test_attribute_reference_in_expression_not_treated_as_definition() {
    // Pattern: attribute :>> semiMajorAxis [1] = radius;
    // The "radius" in the expression should NOT create a symbol
    let source = r#"
        package TestPkg {
            attribute radius : Real;
            
            item def Circle {
                attribute :>> radius [1];
                attribute :>> semiMajorAxis [1] = radius;
                attribute :>> semiMinorAxis [1] = radius;
            }
            
            item def Sphere {
                attribute :>> radius [1];
                attribute :>> semiAxis1 [1] = radius;
                attribute :>> semiAxis2 [1] = radius;
                attribute :>> semiAxis3 [1] = radius;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);

    // Should succeed without duplicate symbol errors
    assert!(
        result.is_ok(),
        "Should not have duplicate symbol errors, got: {:?}",
        result.err()
    );

    // radius should be defined exactly once at package level
    let all_symbols = symbol_table.all_symbols();
    let radius_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "radius")
        .count();

    assert_eq!(
        radius_count, 1,
        "radius should be defined exactly once at package level, got {radius_count} definitions"
    );
}

#[test]
fn test_inline_attribute_definitions_with_same_name_create_duplicates() {
    // Pattern: item :>> shape : Circle { attribute :>> semiMajor, Circle::semiMajor; }
    // Multiple inline body definitions might be creating duplicates
    let source = r#"
        package TestPkg {
            item def Circle {
                attribute radius;
            }
            
            item def CircularDisc {
                item :>> shape : Circle {
                    attribute :>> radius;
                }
                item :>> edges : Circle {
                    attribute :>> radius;
                }
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);

    // Should succeed without duplicate symbol errors
    assert!(
        result.is_ok(),
        "Should not have duplicate symbol errors, got: {:?}",
        result.err()
    );
}

#[test]
fn test_radius_redefinition_in_multiple_items_no_duplicates() {
    // Test case from ShapeItems.sysml: multiple item definitions each redefine "radius"
    // This should NOT create duplicate symbols because they're in different scopes
    let source = r#"
        package ShapeItems {
            item def CircularDisc {
                attribute :>> radius [1];
            }
            
            item def Sphere {
                attribute :>> radius [1];
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);

    // Should succeed without duplicate symbol errors
    assert!(
        result.is_ok(),
        "Should not have duplicate symbol errors, got: {:?}",
        result.err()
    );

    // Check that "radius" doesn't appear as a symbol at all (it's redefined, not defined)
    let all_symbols = symbol_table.all_symbols();
    let radius_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "radius")
        .count();

    assert_eq!(
        radius_count, 0,
        "radius should not appear as a symbol (it's only redefined), got {radius_count} definitions"
    );
}

#[test]
fn test_simple_redefinition_creates_no_new_symbol() {
    // When you redefine without giving it a new name: attribute :>> radius [1];
    // This should NOT create a new symbol named "radius"
    let source = r#"
        package TestPkg {
            item def Parent {
                attribute radius;
            }
            
            item def Child :> Parent {
                attribute :>> radius [1];
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    eprintln!("AST: {file:#?}");

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);

    // Count radius symbols
    let all_symbols = symbol_table.all_symbols();
    let radius_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(name, _)| *name == "radius")
        .collect();

    eprintln!("Found {} radius symbols:", radius_symbols.len());
    for (name, symbol) in &radius_symbols {
        eprintln!("  '{name}': {symbol:?}");
    }

    assert!(
        result.is_ok(),
        "Should not have errors, got: {:?}",
        result.err()
    );

    // Should have exactly 1 radius: the one in Parent
    // The redefinition in Child should NOT create a new symbol
    assert_eq!(
        radius_symbols.len(),
        1,
        "Should have 1 radius symbol (in Parent only), got {}",
        radius_symbols.len()
    );
}

#[test]
fn test_visitor_creates_usage_symbol() {
    let source = "part myCar : Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("myCar").unwrap();
    match symbol {
        Symbol::Usage { usage_type, .. } => {
            assert_eq!(usage_type.as_deref(), Some("Vehicle"));
        }
        _ => panic!("Expected Usage symbol"),
    }
}

#[test]
fn test_visitor_records_specialization_relationship() {
    let source = "part def Car :> Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let specializations = graph.get_one_to_many(REL_SPECIALIZATION, "Car").unwrap();
    assert_eq!(specializations, &["Vehicle"]);
}

#[test]
fn test_visitor_records_typing_relationship() {
    let source = "part myCar : Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let typing = graph.get_one_to_one(REL_TYPING, "myCar").unwrap();
    assert_eq!(typing, "Vehicle");
}

#[test]
fn test_visitor_handles_nested_usage() {
    let source = r#"part def Car { attribute mass : Real; }"#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Check that Car definition exists
    assert!(symbol_table.lookup("Car").is_some());

    // Check that mass exists and has the correct qualified name
    let all_symbols = symbol_table.all_symbols();
    let mass_symbol = all_symbols
        .iter()
        .find(|(name, _)| *name == "mass")
        .expect("Should have 'mass' symbol");

    match mass_symbol.1 {
        Symbol::Usage { qualified_name, .. } => {
            assert_eq!(qualified_name, "Car::mass");
        }
        _ => panic!("Expected Usage symbol for mass"),
    }
}

#[test]
fn test_debug_symbol_table_contents() {
    let source = r#"part def Car { attribute mass : Real; }"#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    println!("\n=== All symbols in table ===");
    for (qname, symbol) in symbol_table.all_symbols() {
        println!("  '{qname}' -> {symbol:?}");
    }
    println!("=== End of symbols ===\n");
}

#[test]
fn test_multiple_specializations() {
    let source = "part def ElectricCar :> Car, Electric, Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let specializations = graph
        .get_one_to_many(REL_SPECIALIZATION, "ElectricCar")
        .unwrap();
    assert_eq!(specializations.len(), 3);
    assert!(specializations.contains(&"Car".to_string()));
    assert!(specializations.contains(&"Electric".to_string()));
    assert!(specializations.contains(&"Vehicle".to_string()));
}

#[test]
fn test_multiple_symbols_in_same_scope() {
    let source = r#"
        part def Car;
        part def Truck;
        part def Motorcycle;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("Car").is_some());
    assert!(symbol_table.lookup("Truck").is_some());
    assert!(symbol_table.lookup("Motorcycle").is_some());
}

#[test]
fn test_deeply_nested_symbols() {
    let source = r#"
        part def Vehicle {
            part engine {
                attribute cylinders : Integer;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();
    // Check all three levels exist
    assert!(symbol_table.lookup("Vehicle").is_some());

    let all_symbols = symbol_table.all_symbols();
    let engine = all_symbols
        .iter()
        .find(|(name, _)| *name == "engine")
        .expect("Should have 'engine' symbol");

    match engine.1 {
        Symbol::Usage { qualified_name, .. } => {
            assert_eq!(qualified_name, "Vehicle::engine");
        }
        _ => panic!("Expected Usage symbol for engine"),
    }

    let cylinders = all_symbols
        .iter()
        .find(|(name, _)| *name == "cylinders")
        .expect("Should have 'cylinders' symbol");

    match cylinders.1 {
        Symbol::Usage { qualified_name, .. } => {
            assert_eq!(qualified_name, "Vehicle::engine::cylinders");
        }
        _ => panic!("Expected Usage symbol for cylinders"),
    }
}

#[test]
fn test_different_definition_kinds() {
    let source = r#"
        part def PartDef;
        action def ActionDef;
        requirement def ReqDef;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let part_def = symbol_table.lookup("PartDef").unwrap();
    match part_def {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Part"),
        _ => panic!("Expected Definition symbol"),
    }

    let action_def = symbol_table.lookup("ActionDef").unwrap();
    match action_def {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Action"),
        _ => panic!("Expected Definition symbol"),
    }

    let req_def = symbol_table.lookup("ReqDef").unwrap();
    match req_def {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Requirement"),
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_scoped_symbols_with_same_name() {
    let source = r#"
        part def Car {
            attribute speed : Real;
        }
        part def Plane {
            attribute speed : Real;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Both should exist with different qualified names
    let all_symbols = symbol_table.all_symbols();
    let speed_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(name, _)| *name == "speed")
        .collect();

    // We should have exactly 2 symbols named "speed" (this might fail if scoping is wrong!)
    assert_eq!(
        speed_symbols.len(),
        2,
        "Should have 2 'speed' symbols in different scopes"
    );

    let qualified_names: Vec<String> = speed_symbols
        .iter()
        .map(|(_, symbol)| match symbol {
            Symbol::Usage { qualified_name, .. } => qualified_name.clone(),
            _ => panic!("Expected Usage symbol"),
        })
        .collect();

    assert!(qualified_names.contains(&"Car::speed".to_string()));
    assert!(qualified_names.contains(&"Plane::speed".to_string()));
}

#[test]
fn test_nested_packages() {
    let source = r#"
        package OuterPackage {
            package InnerPackage {
                part def Component;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("OuterPackage").is_some());

    let all_symbols = symbol_table.all_symbols();
    let inner = all_symbols
        .iter()
        .find(|(name, _)| *name == "InnerPackage")
        .expect("Should have 'InnerPackage' symbol");

    match inner.1 {
        Symbol::Package { qualified_name, .. } => {
            assert_eq!(qualified_name, "OuterPackage::InnerPackage");
        }
        _ => panic!("Expected Package symbol for InnerPackage"),
    }
}

#[test]
fn test_empty_definition() {
    let source = "part def EmptyPart { }";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("EmptyPart").unwrap();
    match symbol {
        Symbol::Definition { name, .. } => assert_eq!(name, "EmptyPart"),
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_usage_without_type() {
    let source = "part untyped;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("untyped").unwrap();
    match symbol {
        Symbol::Usage { usage_type, .. } => {
            assert_eq!(
                usage_type, &None,
                "Usage without type should have None as usage_type"
            );
        }
        _ => panic!("Expected Usage symbol"),
    }
}

#[test]
fn test_qualified_names_are_correct() {
    let source = r#"
        package Vehicles {
            part def Car {
                attribute mass : Real;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let vehicles = symbol_table.lookup("Vehicles").unwrap();
    match vehicles {
        Symbol::Package { qualified_name, .. } => {
            assert_eq!(qualified_name, "Vehicles");
        }
        _ => panic!("Expected Package symbol"),
    }

    let all_symbols = symbol_table.all_symbols();
    let car = all_symbols
        .iter()
        .find(|(name, _)| *name == "Car")
        .expect("Should have 'Car' symbol");

    match car.1 {
        Symbol::Definition { qualified_name, .. } => {
            assert_eq!(qualified_name, "Vehicles::Car");
        }
        _ => panic!("Expected Definition symbol"),
    }

    let mass = all_symbols
        .iter()
        .find(|(name, _)| *name == "mass")
        .expect("Should have 'mass' symbol");

    match mass.1 {
        Symbol::Usage { qualified_name, .. } => {
            assert_eq!(qualified_name, "Vehicles::Car::mass");
        }
        _ => panic!("Expected Usage symbol"),
    }
}

#[test]
fn test_multiple_usages_of_same_type() {
    let source = r#"
        part car1 : Vehicle;
        part car2 : Vehicle;
        part car3 : Vehicle;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // All three should exist and have the same typing
    for name in ["car1", "car2", "car3"] {
        let symbol = symbol_table
            .lookup(name)
            .unwrap_or_else(|| panic!("Should have '{name}' symbol"));
        match symbol {
            Symbol::Usage { usage_type, .. } => {
                assert_eq!(usage_type.as_deref(), Some("Vehicle"));
            }
            _ => panic!("Expected Usage symbol for {name}"),
        }

        let typing = graph.get_one_to_one(REL_TYPING, name).unwrap();
        assert_eq!(typing, "Vehicle");
    }
}

#[test]
fn test_redefinition_relationship() {
    let source = "part def SportsCar :>> Car;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("SportsCar").is_some());

    // Check if redefinition relationship is recorded
    let redefinitions = graph.get_one_to_many(REL_REDEFINITION, "SportsCar");
    assert!(
        redefinitions.is_some() && redefinitions.unwrap().contains(&"Car".to_string()),
        "Should record redefinition relationship"
    );
}

#[test]
fn test_alias_definition() {
    let source = "alias MyAlias for SomeType;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    println!(
        "Elements in file: {:?}",
        file.elements
            .iter()
            .map(|e| format!("{e:?}"))
            .collect::<Vec<_>>()
    );

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    println!(
        "Symbols: {:?}",
        symbol_table
            .all_symbols()
            .iter()
            .map(|(n, _)| n)
            .collect::<Vec<_>>()
    );

    let symbol = symbol_table
        .all_symbols()
        .into_iter()
        .find(|(name, _)| *name == "MyAlias")
        .map(|(_, s)| s);
    assert!(symbol.is_some(), "Alias should be in symbol table");

    match symbol.unwrap() {
        Symbol::Alias { target, .. } => {
            assert_eq!(target, "SomeType");
        }
        _ => panic!("Expected Alias symbol"),
    }
}

#[test]
fn test_import_statement() {
    let source = "import Vehicles::*;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Imports should be recorded in the symbol table's scope
    // This test checks that the adapter processes imports without crashing
}

#[test]
fn test_port_definition_and_usage() {
    let source = r#"
        port def DataPort;
        part def Component {
            port input : DataPort;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let port_def = symbol_table.lookup("DataPort").unwrap();
    match port_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Port");
        }
        _ => panic!("Expected Definition symbol"),
    }

    let all_symbols = symbol_table.all_symbols();
    let input_port = all_symbols
        .iter()
        .find(|(name, _)| *name == "input")
        .expect("Should have 'input' port");

    match input_port.1 {
        Symbol::Usage {
            kind,
            qualified_name,
            ..
        } => {
            assert_eq!(kind, "Port");
            assert_eq!(qualified_name, "Component::input");
        }
        _ => panic!("Expected Usage symbol for port"),
    }
}

#[test]
fn test_action_with_parameters() {
    let source = r#"
        action def ProcessData {
            in item data : String;
            out item result : Integer;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let action_def = symbol_table.lookup("ProcessData").unwrap();
    match action_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Action");
        }
        _ => panic!("Expected Definition symbol"),
    }

    // Check that parameters exist
    let all_symbols = symbol_table.all_symbols();
    let has_data = all_symbols.iter().any(|(name, _)| *name == "data");
    let has_result = all_symbols.iter().any(|(name, _)| *name == "result");

    assert!(has_data, "Should have 'data' parameter");
    assert!(has_result, "Should have 'result' parameter");
}

#[test]
fn test_constraint_definition() {
    let source = r#"
        constraint def SpeedLimit {
            attribute maxSpeed : Real;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let constraint_def = symbol_table.lookup("SpeedLimit").unwrap();
    match constraint_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Constraint");
        }
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_enumeration_definition() {
    let source = r#"
        enum def Color {
            Red;
            Green;
            Blue;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let enum_def = symbol_table.lookup("Color").unwrap();
    match enum_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Enumeration");
        }
        _ => panic!("Expected Definition symbol"),
    }

    // Check for enum values
    let all_symbols = symbol_table.all_symbols();
    let has_red = all_symbols.iter().any(|(name, _)| *name == "Red");
    let has_green = all_symbols.iter().any(|(name, _)| *name == "Green");
    let has_blue = all_symbols.iter().any(|(name, _)| *name == "Blue");

    assert!(has_red, "Should have enum value 'Red'");
    assert!(has_green, "Should have enum value 'Green'");
    assert!(has_blue, "Should have enum value 'Blue'");
}

#[test]
fn test_state_definition() {
    let source = r#"
        state def VehicleState {
            entry; then idle;
            state idle;
            state moving;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let state_def = symbol_table.lookup("VehicleState").unwrap();
    match state_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "State");
        }
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_connection_definition() {
    let source = "connection def DataFlow;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let conn_def = symbol_table.lookup("DataFlow").unwrap();
    match conn_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Connection");
        }
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_interface_definition() {
    let source = "interface def NetworkInterface;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let intf_def = symbol_table.lookup("NetworkInterface").unwrap();
    match intf_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Interface");
        }
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_allocation_definition() {
    let source = "allocation def ResourceAllocation;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let alloc_def = symbol_table.lookup("ResourceAllocation").unwrap();
    match alloc_def {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Allocation");
        }
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_mixed_definitions_and_usages() {
    let source = r#"
        part def Engine;
        part def Wheel;
        part def Car {
            part engine : Engine;
            part wheel1 : Wheel;
            part wheel2 : Wheel;
        }
        part myCar : Car;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // All definitions should exist
    assert!(symbol_table.lookup("Engine").is_some());
    assert!(symbol_table.lookup("Wheel").is_some());
    assert!(symbol_table.lookup("Car").is_some());
    assert!(symbol_table.lookup("myCar").is_some());

    // Check nested parts
    let all_symbols = symbol_table.all_symbols();
    assert!(all_symbols.iter().any(|(name, _)| *name == "engine"));
    assert!(all_symbols.iter().any(|(name, _)| *name == "wheel1"));
    assert!(all_symbols.iter().any(|(name, _)| *name == "wheel2"));
}

#[test]
fn test_concern_and_requirement() {
    let source = r#"
        concern def Safety;
        requirement def SafetyRequirement;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let concern = symbol_table.lookup("Safety").unwrap();
    match concern {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "UseCase"); // Concern maps to UseCase
        }
        _ => panic!("Expected Definition symbol"),
    }

    let requirement = symbol_table.lookup("SafetyRequirement").unwrap();
    match requirement {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Requirement");
        }
        _ => panic!("Expected Definition symbol"),
    }
}
