#![allow(clippy::unwrap_used)]
use syster::semantic::resolver::Resolver;

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

    assert!(Resolver::new(&symbol_table).resolve("MyPackage").is_some());
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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("Vehicle").unwrap();
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
    // But attribute :>> radius creates a child scoped symbol named "radius"
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

    // In SysML v2, `attribute :>> radius` creates a scoped symbol named "radius" in each child
    // So we have: TestPkg::radius, TestPkg::Circle::radius, TestPkg::Sphere::radius
    let all_symbols = symbol_table.all_symbols();
    let radius_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "radius")
        .count();

    assert_eq!(
        radius_count, 3,
        "radius should appear 3 times (package level + Circle + Sphere), got {radius_count}"
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
    // This should NOT create duplicate symbol errors because they're in different scopes
    // In SysML v2, `attribute :>> radius` creates a scoped symbol named "radius" in each child
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

    // Each redefinition creates a symbol in the child's scope
    // So we have: ShapeItems::CircularDisc::radius, ShapeItems::Sphere::radius
    let all_symbols = symbol_table.all_symbols();
    let radius_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "radius")
        .count();

    assert_eq!(
        radius_count, 2,
        "Should have 2 radius symbols (one in each item def), got {radius_count}"
    );
}

#[test]
fn test_simple_redefinition_creates_child_symbol() {
    // When you redefine without giving it a new name: attribute :>> radius [1];
    // In SysML v2, this DOES create a symbol named "radius" in the child's scope
    // Child::radius redefines Parent::radius
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

    assert!(
        result.is_ok(),
        "Should not have errors, got: {:?}",
        result.err()
    );

    // Should have exactly 2: Parent::radius and Child::radius
    assert_eq!(
        radius_symbols.len(),
        2,
        "Should have 2 radius symbols (Parent::radius and Child::radius), got {}",
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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("myCar").unwrap();
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
    assert!(Resolver::new(&symbol_table).resolve("Car").is_some());

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
    for (_qname, _symbol) in symbol_table.all_symbols() {}
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
    assert!(specializations.contains(&"Car"));
    assert!(specializations.contains(&"Electric"));
    assert!(specializations.contains(&"Vehicle"));
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

    assert!(Resolver::new(&symbol_table).resolve("Car").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Truck").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Motorcycle").is_some());
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
    assert!(Resolver::new(&symbol_table).resolve("Vehicle").is_some());

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

    let resolver = Resolver::new(&symbol_table);
    let part_def = resolver.resolve("PartDef").unwrap();
    match part_def {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Part"),
        _ => panic!("Expected Definition symbol"),
    }

    let resolver = Resolver::new(&symbol_table);
    let action_def = resolver.resolve("ActionDef").unwrap();
    match action_def {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Action"),
        _ => panic!("Expected Definition symbol"),
    }

    let resolver = Resolver::new(&symbol_table);
    let req_def = resolver.resolve("ReqDef").unwrap();
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

    assert!(
        Resolver::new(&symbol_table)
            .resolve("OuterPackage")
            .is_some()
    );

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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("EmptyPart").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("untyped").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let vehicles = resolver.resolve("Vehicles").unwrap();
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
    let resolver = Resolver::new(&symbol_table);
    for name in ["car1", "car2", "car3"] {
        let symbol = resolver
            .resolve(name)
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

    assert!(Resolver::new(&symbol_table).resolve("SportsCar").is_some());

    // Check if redefinition relationship is recorded
    let redefinitions = graph.get_one_to_many(REL_REDEFINITION, "SportsCar");
    assert!(
        redefinitions.is_some() && redefinitions.unwrap().contains(&"Car"),
        "Should record redefinition relationship"
    );
}

#[test]
fn test_alias_definition() {
    let source = "alias MyAlias for SomeType;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

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

    let resolver = Resolver::new(&symbol_table);
    let port_def = resolver.resolve("DataPort").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let action_def = resolver.resolve("ProcessData").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let constraint_def = resolver.resolve("SpeedLimit").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let enum_def = resolver.resolve("Color").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let state_def = resolver.resolve("VehicleState").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let conn_def = resolver.resolve("DataFlow").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let intf_def = resolver.resolve("NetworkInterface").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let alloc_def = resolver.resolve("ResourceAllocation").unwrap();
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
    assert!(Resolver::new(&symbol_table).resolve("Engine").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Wheel").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Car").is_some());
    assert!(Resolver::new(&symbol_table).resolve("myCar").is_some());

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

    let resolver = Resolver::new(&symbol_table);
    let concern = resolver.resolve("Safety").unwrap();
    match concern {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "UseCase"); // Concern maps to UseCase
        }
        _ => panic!("Expected Definition symbol"),
    }

    let resolver = Resolver::new(&symbol_table);
    let requirement = resolver.resolve("SafetyRequirement").unwrap();
    match requirement {
        Symbol::Definition { kind, .. } => {
            assert_eq!(kind, "Requirement");
        }
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_identifier_in_default_value_not_treated_as_definition() {
    // Reproduces the issue from Performances.kerml where "thisPerformance" in a default value
    // was incorrectly being treated as a feature definition
    let source = r#"
        package TestPkg {
            action def Performance {
                attribute redefines dispatchScope default thisPerformance;
                attribute thisPerformance: Performance [1] default self;
            }
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = SysmlAdapter::with_relationships(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);

    // Should not have duplicate symbol errors
    assert!(
        result.is_ok(),
        "Should not have duplicate symbol errors, got: {:?}",
        result.err()
    );

    // Should have exactly one "thisPerformance" symbol (the actual definition, not the reference)
    let all_symbols = symbol_table.all_symbols();
    let this_perf_count = all_symbols
        .iter()
        .filter(|(name, _)| *name == "thisPerformance")
        .count();

    assert_eq!(
        this_perf_count, 1,
        "Should have exactly one 'thisPerformance' definition, got {this_perf_count}"
    );
}
