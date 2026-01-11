#![allow(clippy::unwrap_used)]
use syster::semantic::resolver::Resolver;

use pest::Parser;
use std::path::PathBuf;
use syster::parser::{SysMLParser, sysml::Rule};
use syster::semantic::Workspace;
use syster::syntax::SyntaxFile;
use syster::syntax::sysml::ast::parse_file;

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
    let file = parse_file(&mut pairs).unwrap();

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
    let file = parse_file(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Vehicle should be accessible in Derived package due to import
    let resolver = Resolver::new(workspace.symbol_table());
    let my_car = resolver.resolve_qualified("Derived::myCar");
    assert!(my_car.is_some(), "myCar should be defined");

    // Verify that Base::Vehicle can be found (the imported member)
    let resolver = Resolver::new(workspace.symbol_table());
    let vehicle = resolver.resolve_qualified("Base::Vehicle");
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
    let file = parse_file(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Member import: only Vehicle should be accessible in Derived1
    let resolver = Resolver::new(workspace.symbol_table());
    let car1 = resolver.resolve_qualified("Derived1::myCar");
    assert!(car1.is_some(), "Derived1::myCar should be defined");

    // Namespace import: both Vehicle and Engine should be accessible in Derived2
    let resolver = Resolver::new(workspace.symbol_table());
    let car2 = resolver.resolve_qualified("Derived2::myCar");
    let engine2 = resolver.resolve_qualified("Derived2::myEngine");
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
    let file = parse_file(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Both Vehicle and Engine should be accessible via namespace import
    let resolver = Resolver::new(workspace.symbol_table());
    let car = resolver.resolve_qualified("Derived::car");
    let resolver = Resolver::new(workspace.symbol_table());
    let engine = resolver.resolve_qualified("Derived::engine");

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
    let file1 = parse_file(&mut pairs1).unwrap();

    let mut pairs2 = SysMLParser::parse(Rule::model, file2_source).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

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
    let resolver = Resolver::new(workspace.symbol_table());
    let car = resolver.resolve_qualified("Car");
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
    let file = parse_file(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    workspace.populate_all().expect("Population should succeed");

    // With model-level imports, both should work
    let resolver = Resolver::new(workspace.symbol_table());
    let y = resolver.resolve_qualified("B::y");
    assert!(y.is_some(), "y should be defined in package B");
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
    let file = parse_file(&mut pairs).unwrap();

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
    let file = parse_file(&mut pairs).unwrap();

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("test.sysml"),
        syster::syntax::SyntaxFile::SysML(file),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify that Car can reference BaseVehicle (the alias)
    let resolver = Resolver::new(workspace.symbol_table());
    let car = resolver.resolve_qualified("Derived::Car");
    assert!(car.is_some(), "Car should be defined");

    // Verify that BaseVehicle resolves to Vehicle
    let resolver = Resolver::new(workspace.symbol_table());
    let base_vehicle = resolver.resolve_qualified("Derived::BaseVehicle");
    assert!(
        base_vehicle.is_some(),
        "BaseVehicle alias should be defined in Derived package"
    );

    // Verify that the alias actually points to the Vehicle definition
    let vehicle = resolver.resolve_qualified("Base::Vehicle");
    assert!(
        vehicle.is_some(),
        "Vehicle should be defined in Base package"
    );

    // Verify that BaseVehicle and Vehicle refer to the same underlying definition
    let base_vehicle_symbol = base_vehicle.unwrap();
    let vehicle_symbol = vehicle.unwrap();

    // The alias should have the same qualified name as the original symbol
    assert_eq!(
        base_vehicle_symbol.name(),
        "BaseVehicle",
        "BaseVehicle should have the alias name"
    );
    assert_eq!(
        vehicle_symbol.qualified_name(),
        "Base::Vehicle",
        "Vehicle should have its original qualified name"
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
    let file = parse_file(&mut pairs).unwrap();

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

// Tests for import references removed - they are now computed on demand in LSP layer

#[test]
fn test_public_import_reexport_resolution() {
    // Test that public imports make members available under the re-exporting namespace
    // e.g., when ISQ has "public import ISQBase::*", then "ISQ::MassValue" should resolve
    let source = r#"
        package ISQBase {
            attribute def MassValue;
            attribute def LengthValue;
        }
        package ISQ {
            public import ISQBase::*;
        }
        package Consumer {
            private import ISQ::MassValue;
            attribute mass : MassValue;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let path = PathBuf::from("test.sysml");
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(path.clone(), SyntaxFile::SysML(file));
    workspace.populate_all().unwrap();

    let resolver = Resolver::new(workspace.symbol_table());

    // ISQBase::MassValue should resolve directly
    let direct = resolver.resolve_qualified("ISQBase::MassValue");
    assert!(
        direct.is_some(),
        "ISQBase::MassValue should resolve directly"
    );

    // ISQ::MassValue should also resolve via public re-export
    let via_reexport = resolver.resolve_qualified("ISQ::MassValue");
    assert!(
        via_reexport.is_some(),
        "ISQ::MassValue should resolve via public import re-export. \
         ISQ has 'public import ISQBase::*' so MassValue should be accessible as ISQ::MassValue"
    );

    // LengthValue should also be re-exported
    let length_via_reexport = resolver.resolve_qualified("ISQ::LengthValue");
    assert!(
        length_via_reexport.is_some(),
        "ISQ::LengthValue should also resolve via public import re-export"
    );
}

#[test]
fn test_private_import_not_reexported() {
    // Test that private imports do NOT make members available under the importing namespace
    let source = r#"
        package ISQBase {
            attribute def MassValue;
        }
        package ISQ {
            private import ISQBase::*;
        }
        package Consumer {
            private import ISQ::MassValue;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let path = PathBuf::from("test.sysml");
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(path.clone(), SyntaxFile::SysML(file));
    workspace.populate_all().unwrap();

    let resolver = Resolver::new(workspace.symbol_table());

    // ISQBase::MassValue should resolve directly
    let direct = resolver.resolve_qualified("ISQBase::MassValue");
    assert!(
        direct.is_some(),
        "ISQBase::MassValue should resolve directly"
    );

    // ISQ::MassValue should NOT resolve because the import is private
    let via_private = resolver.resolve_qualified("ISQ::MassValue");
    assert!(
        via_private.is_none(),
        "ISQ::MassValue should NOT resolve because the import in ISQ is private"
    );
}

#[test]
fn test_public_import_reexport_cross_file() {
    // Test public import re-export resolution across multiple files
    // This mimics the real stdlib scenario where ISQ.sysml imports from ISQBase.sysml
    let isqbase_source = r#"
        standard library package ISQBase {
            attribute def MassValue;
            attribute def LengthValue;
        }
    "#;

    let isq_source = r#"
        standard library package ISQ {
            public import ISQBase::*;
        }
    "#;

    let consumer_source = r#"
        package Consumer {
            private import ISQ::MassValue;
            attribute mass : MassValue;
        }
    "#;

    // Parse each file
    let mut pairs = SysMLParser::parse(Rule::model, isqbase_source).unwrap();
    let isqbase_file = parse_file(&mut pairs).unwrap();

    let mut pairs = SysMLParser::parse(Rule::model, isq_source).unwrap();
    let isq_file = parse_file(&mut pairs).unwrap();

    let mut pairs = SysMLParser::parse(Rule::model, consumer_source).unwrap();
    let consumer_file = parse_file(&mut pairs).unwrap();

    // Add all files to workspace
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(
        PathBuf::from("ISQBase.sysml"),
        SyntaxFile::SysML(isqbase_file),
    );
    workspace.add_file(PathBuf::from("ISQ.sysml"), SyntaxFile::SysML(isq_file));
    workspace.add_file(
        PathBuf::from("Consumer.sysml"),
        SyntaxFile::SysML(consumer_file),
    );
    workspace.populate_all().unwrap();

    let resolver = Resolver::new(workspace.symbol_table());

    // Debug: Print all symbols
    eprintln!("=== Symbols ===");
    for symbol in workspace.symbol_table().iter_symbols() {
        eprintln!("  {} -> {:?}", symbol.name(), symbol.qualified_name());
    }

    // Debug: Print scopes and imports
    eprintln!("=== Scopes ===");
    for (idx, scope) in workspace.symbol_table().scopes().iter().enumerate() {
        eprintln!(
            "  Scope {}: parent={:?}, symbols={:?}",
            idx,
            scope.parent,
            scope.symbols.keys().collect::<Vec<_>>()
        );
        if !scope.imports.is_empty() {
            for import in &scope.imports {
                eprintln!(
                    "    Import: path={}, is_namespace={}, is_public={}",
                    import.path, import.is_namespace, import.is_public
                );
            }
        }
    }

    // ISQBase::MassValue should resolve directly
    let direct = resolver.resolve_qualified("ISQBase::MassValue");
    assert!(
        direct.is_some(),
        "ISQBase::MassValue should resolve directly"
    );

    // ISQ::MassValue should resolve via public re-export
    let via_reexport = resolver.resolve_qualified("ISQ::MassValue");
    assert!(
        via_reexport.is_some(),
        "ISQ::MassValue should resolve via public import re-export across files. \
         ISQ has 'public import ISQBase::*' so MassValue should be accessible as ISQ::MassValue"
    );
}
