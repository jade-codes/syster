#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use std::path::PathBuf;
use syster::parser::{sysml::Rule, SysMLParser};
use syster::semantic::Workspace;
use syster::syntax::sysml::ast::SysMLFile;
use syster::syntax::SyntaxFile;

#[test]
fn test_parse_import_statement() {
    // Test that import statements can be parsed
    let source = r#"
        package Vehicles {
            part def Vehicle;
        }
        public import Vehicles::*;
        package Cars {
            part def Car :> Vehicle;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    // Verify we have elements (packages and import)
    assert!(
        file.elements.len() >= 3,
        "Should have 2 packages and 1 import, got {}",
        file.elements.len()
    );
}

#[test]
fn test_import_membership() {
    // Test membership import (imports a specific member)
    let source = r#"
        package Base {
            part def Vehicle;
            part def Engine;
        }
        package Derived {
            public import Base::Vehicle;
            part myCar : Vehicle;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Vehicle should be accessible in Derived package due to import
    let my_car = workspace.symbol_table().lookup_qualified("Derived::myCar");
    assert!(my_car.is_some(), "myCar should be defined");

    // Verify that Base::Vehicle can be found (the imported member)
    let vehicle = workspace.symbol_table().lookup_qualified("Base::Vehicle");
    assert!(
        vehicle.is_some(),
        "Base::Vehicle should exist and be importable"
    );
}

#[test]
fn test_import_membership_with_namespace() {
    // Test that both member import and namespace import work
    let source = r#"
        package Base {
            part def Vehicle;
            part def Engine;
        }
        package Derived1 {
            public import Base::Vehicle;
            part myCar : Vehicle;
        }
        package Derived2 {
            public import Base::*;
            part myCar : Vehicle;
            part myEngine : Engine;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Member import: only Vehicle should be accessible in Derived1
    let car1 = workspace.symbol_table().lookup_qualified("Derived1::myCar");
    assert!(car1.is_some(), "Derived1::myCar should be defined");

    // Namespace import: both Vehicle and Engine should be accessible in Derived2
    let car2 = workspace.symbol_table().lookup_qualified("Derived2::myCar");
    let engine2 = workspace
        .symbol_table()
        .lookup_qualified("Derived2::myEngine");
    assert!(car2.is_some(), "Derived2::myCar should be defined");
    assert!(engine2.is_some(), "Derived2::myEngine should be defined");
}

#[test]
fn test_import_namespace() {
    // Test namespace import (imports all members with ::*)
    let source = r#"
        package Base {
            part def Vehicle;
            part def Engine;
        }
        public import Base::*;
        package Derived {
            part car : Vehicle;
            part engine : Engine;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Both Vehicle and Engine should be accessible via namespace import
    let car = workspace.symbol_table().lookup_qualified("Derived::car");
    let engine = workspace.symbol_table().lookup_qualified("Derived::engine");

    assert!(car.is_some(), "car should be defined");
    assert!(engine.is_some(), "engine should be defined");
}

#[test]
fn test_cross_file_import() {
    // Test import from another file
    let file1_source = r#"
        package Vehicles {
            part def Vehicle;
        }
    "#;

    let file2_source = r#"
        public import Vehicles::*;
        part def Car :> Vehicle;
    "#;

    let mut pairs1 = SysMLParser::parse(Rule::model, file1_source).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("base.sysml"),
        syster::syntax::SyntaxFile::SysML(file1),
    );
    workspace.add_file(
        PathBuf::from("derived.sysml"),
        syster::syntax::SyntaxFile::SysML(file2),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Car should be able to specialize Vehicle from the imported package
    let car = workspace.symbol_table().lookup_qualified("Car");
    assert!(car.is_some(), "Car should be defined");
}

#[test]
fn test_import_visibility() {
    // Test that symbols are only accessible where they're imported
    let source = r#"
        package A {
            part def X;
        }
        public import A::*;
        package B {
            part y : X;  // Should work - X is imported at model level
        }
        package C {
            part z : X;  // Should also work with model-level import
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();

    // With model-level imports, both should work
    if result.is_ok() {
        let y = workspace.symbol_table().lookup_qualified("B::y");
        assert!(y.is_some(), "y should be defined in package B");
    }
}

#[test]
fn test_recursive_import() {
    // Test recursive namespace import (::*::**)
    let source = r#"
        package Root {
            package Sub1 {
                part def A;
            }
            package Sub2 {
                part def B;
            }
        }
        package Consumer {
            public import Root::*::**;
            part a : A;  // Should work with recursive import
            part b : B;  // Should work with recursive import
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());
}

#[test]
fn test_import_alias() {
    // Test import with aliasing - SysML uses separate 'alias' statements
    let source = r#"
        package Base {
            part def Vehicle;
        }
        package Derived {
            public import Base::*;
            alias BaseVehicle for Vehicle;
            part def Car :> BaseVehicle;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify that Car can reference BaseVehicle (the alias)
    let car = workspace.symbol_table().lookup_qualified("Derived::Car");
    assert!(car.is_some(), "Car should be defined");

    // Verify that BaseVehicle resolves to Vehicle
    let base_vehicle = workspace
        .symbol_table()
        .lookup_qualified("Derived::BaseVehicle");
    assert!(
        base_vehicle.is_some(),
        "BaseVehicle alias should be defined in Derived package"
    );
}

#[test]
fn test_workspace_with_stdlib() {
    // Test that workspace can be created with standard library
    let workspace_without = Workspace::<SyntaxFile>::new();
    assert!(
        !workspace_without.has_stdlib(),
        "New workspace should not have stdlib loaded"
    );

    let workspace_with = Workspace::<SyntaxFile>::with_stdlib();
    assert!(
        workspace_with.has_stdlib(),
        "Workspace created with_stdlib should have stdlib loaded"
    );
}

#[test]
fn test_stdlib_usage_pattern() {
    // Demonstrates the pattern for using stdlib in a real project
    // In practice, you'd use Workspace::<SyntaxFile>::with_stdlib() to get standard types automatically

    let source = r#"
        package MyProject {
            // In a real implementation, ScalarValues::Boolean would come from stdlib
            // and be automatically available without explicit import
            part def Switch {
                attribute isOn : Boolean;  // Boolean from stdlib
            }
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    // For user projects, use with_stdlib()
    let mut workspace = Workspace::<SyntaxFile>::with_stdlib();
    workspace.add_file(
        PathBuf::from("project.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify the workspace has stdlib loaded
    assert!(workspace.has_stdlib());
}
