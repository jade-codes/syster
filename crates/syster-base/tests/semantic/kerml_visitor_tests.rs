#![allow(clippy::unwrap_used)]

use from_pest::FromPest;
use pest::Parser;
use syster::parser::{KerMLParser, kerml::Rule};
use syster::semantic::RelationshipGraph;
use syster::semantic::adapters::KermlAdapter;
use syster::semantic::symbol_table::{Symbol, SymbolTable};
use syster::syntax::kerml::KerMLFile;

#[test]
fn test_kerml_visitor_creates_package_symbol() {
    let source = "package MyPackage;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("MyPackage").is_some());
}

#[test]
fn test_kerml_visitor_creates_classifier_symbol() {
    let source = "classifier Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Vehicle").unwrap();
    match symbol {
        Symbol::Classifier { kind, .. } => assert_eq!(kind, "Classifier"),
        _ => panic!("Expected Classifier symbol"),
    }
}

#[test]
fn test_kerml_visitor_creates_datatype_symbol() {
    let source = "datatype Temperature;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Temperature").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Datatype"),
        _ => panic!("Expected Definition symbol"),
    }
}

#[test]
fn test_kerml_visitor_creates_feature_symbol() {
    let source = "feature mass;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("mass").unwrap();
    match symbol {
        Symbol::Feature { .. } => (),
        _ => panic!("Expected Feature symbol"),
    }
}

#[test]
fn test_kerml_visitor_handles_nested_elements() {
    let source = r#"
        package OuterPackage {
            classifier InnerClassifier;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("OuterPackage").is_some());

    // Nested elements must be looked up via all_symbols since they're in a nested scope
    let all_symbols = symbol_table.all_symbols();
    let inner = all_symbols
        .iter()
        .find(|(name, _)| *name == "InnerClassifier")
        .expect("Should have 'InnerClassifier' symbol");

    match inner.1 {
        Symbol::Classifier { qualified_name, .. } => {
            assert_eq!(qualified_name, "OuterPackage::InnerClassifier");
        }
        _ => panic!("Expected Classifier symbol for InnerClassifier"),
    }
}

#[test]
fn test_kerml_visitor_creates_function_symbol() {
    let source = "function calculateArea;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("calculateArea").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Function"),
        _ => panic!("Expected Function symbol"),
    }
}

#[test]
fn test_kerml_visitor_handles_specialization_relationships() {
    let source = r#"
        classifier Vehicle;
        classifier Car specializes Vehicle;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("Vehicle").is_some());
    assert!(symbol_table.lookup("Car").is_some());

    // Verify the relationship graph has the specialization
    let relationships = graph.get_all_relationships("Car");
    assert!(!relationships.is_empty(), "Car should have relationships");
}

#[test]
fn test_kerml_visitor_handles_feature_typing() {
    let source = r#"
        datatype Real;
        feature mass : Real;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("Real").is_some());
    assert!(symbol_table.lookup("mass").is_some());

    // Verify the relationship graph has the typing relationship
    let relationships = graph.get_all_relationships("mass");
    assert!(
        !relationships.is_empty(),
        "mass should have typing relationship"
    );
}

#[test]
fn test_kerml_visitor_handles_abstract_classifiers() {
    let source = "abstract classifier Shape;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Shape").unwrap();
    match symbol {
        Symbol::Classifier {
            kind, is_abstract, ..
        } => {
            assert_eq!(kind, "Classifier");
            assert!(is_abstract, "Should be marked abstract");
        }
        _ => panic!("Expected Classifier symbol"),
    }
}

#[test]
fn test_kerml_visitor_handles_readonly_features() {
    let source = "readonly feature timestamp;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // For now, just verify the symbol exists - readonly modifier tracking will be added later
    let symbol = symbol_table.lookup("timestamp");
    assert!(symbol.is_some(), "timestamp feature should exist");
}

#[test]
fn test_kerml_visitor_handles_redefinition() {
    let source = r#"
        feature baseFeature;
        feature derivedFeature redefines baseFeature;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("baseFeature").is_some());
    assert!(symbol_table.lookup("derivedFeature").is_some());

    // Verify the relationship graph has the redefinition
    let relationships = graph.get_all_relationships("derivedFeature");
    assert!(
        !relationships.is_empty(),
        "derivedFeature should have redefinition relationship"
    );
}

#[test]
fn test_kerml_visitor_handles_imports() {
    let source = r#"
        package MyPackage {
            import OtherPackage::*;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);

    // Should not error on imports
    let result = adapter.populate(&file);
    assert!(result.is_ok(), "Should handle imports without error");
    assert!(symbol_table.lookup("MyPackage").is_some());
}

#[test]
fn test_kerml_visitor_handles_multiple_packages() {
    let source = r#"
        package Package1;
        package Package2;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    eprintln!("Namespace: {:?}", file.namespace);
    eprintln!("Elements: {}", file.elements.len());
    for (i, e) in file.elements.iter().enumerate() {
        eprintln!("  {i}: {e:?}");
    }

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    eprintln!("\nSymbols:");
    for (name, _) in symbol_table.all_symbols() {
        eprintln!("  {name}");
    }

    assert!(symbol_table.lookup("Package1").is_some());
    assert!(symbol_table.lookup("Package2").is_some());
}

#[test]
fn test_kerml_visitor_handles_empty_package() {
    let source = "package EmptyPackage {}";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(symbol_table.lookup("EmptyPackage").is_some());
}

// ============================================================
// Tests for additional classifier kinds
// ============================================================

#[test]
fn test_kerml_visitor_creates_class_symbol() {
    let source = "class Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Vehicle").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Class"),
        _ => panic!("Expected Definition symbol with kind Class"),
    }
}

#[test]
fn test_kerml_visitor_creates_structure_symbol() {
    let source = "struct Point;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Point").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Structure"),
        _ => panic!("Expected Definition symbol with kind Structure"),
    }
}

#[test]
fn test_kerml_visitor_creates_behavior_symbol() {
    let source = "behavior StartEngine;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("StartEngine").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Behavior"),
        _ => panic!("Expected Definition symbol with kind Behavior"),
    }
}

#[test]
fn test_kerml_visitor_creates_type_symbol() {
    let source = "type Length;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Length").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Type"),
        _ => panic!("Expected Definition symbol with kind Type"),
    }
}

#[test]
fn test_kerml_visitor_creates_association_symbol() {
    let source = "assoc Ownership;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("Ownership").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Association"),
        _ => panic!("Expected Definition symbol with kind Association"),
    }
}

#[test]
fn test_kerml_visitor_creates_association_structure_symbol() {
    let source = "assoc struct LinkStructure;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("LinkStructure").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "AssociationStructure"),
        _ => panic!("Expected Definition symbol with kind AssociationStructure"),
    }
}

#[test]
fn test_kerml_visitor_creates_metaclass_symbol() {
    let source = "metaclass MetaElement;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    let symbol = symbol_table.lookup("MetaElement").unwrap();
    match symbol {
        Symbol::Definition { kind, .. } => assert_eq!(kind, "Metaclass"),
        _ => panic!("Expected Definition symbol with kind Metaclass"),
    }
}

// ============================================================
// Tests for feature members and relationship tracking
// ============================================================

#[test]
fn test_kerml_visitor_handles_feature_subsetting() {
    let source = r#"
        feature velocity;
        feature speed subsets velocity;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Verify feature was created
    assert!(symbol_table.lookup("speed").is_some());

    // Verify subsetting relationship was recorded
    let relationships = graph.get_all_relationships("speed");
    assert!(relationships.iter().any(|(rel_type, targets)| {
        rel_type == "subsetting" && targets.contains(&"velocity".to_string())
    }));
}

#[test]
fn test_kerml_visitor_handles_multiple_feature_relationships() {
    let source = r#"
        feature baseCount;
        feature measurement;
        datatype Integer;
        feature count : Integer redefines baseCount subsets measurement;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Verify feature was created
    assert!(symbol_table.lookup("count").is_some());

    // Verify all three relationships
    let relationships = graph.get_all_relationships("count");
    assert!(relationships.iter().any(|(rel_type, targets)| {
        rel_type == "typing" && targets.contains(&"Integer".to_string())
    }));
    assert!(relationships.iter().any(|(rel_type, targets)| {
        rel_type == "redefinition" && targets.contains(&"baseCount".to_string())
    }));
    assert!(relationships.iter().any(|(rel_type, targets)| {
        rel_type == "subsetting" && targets.contains(&"measurement".to_string())
    }));
}

// ============================================================
// Tests for nested classifier members
// ============================================================

#[test]
fn test_kerml_visitor_handles_classifier_with_features() {
    let source = r#"
        classifier Point {
            feature x;
            feature y;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Verify classifier was created
    assert!(symbol_table.lookup("Point").is_some());

    // Verify nested features were created
    let all_symbols = symbol_table.all_symbols();
    assert!(all_symbols.iter().any(|(name, symbol)| {
        *name == "x" && matches!(symbol, Symbol::Feature { .. })
    }));
    assert!(all_symbols.iter().any(|(name, symbol)| {
        *name == "y" && matches!(symbol, Symbol::Feature { .. })
    }));
}

#[test]
fn test_kerml_visitor_handles_classifier_specialization_member() {
    let source = r#"
        classifier Dog specializes Animal;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Verify classifier was created
    assert!(symbol_table.lookup("Dog").is_some());

    // Verify specialization relationship
    let relationships = graph.get_all_relationships("Dog");
    assert!(relationships.iter().any(|(rel_type, targets)| {
        rel_type == "specialization" && targets.contains(&"Animal".to_string())
    }));
}

// ============================================================
// Tests for anonymous elements
// ============================================================

#[test]
fn test_kerml_visitor_handles_anonymous_classifier() {
    let source = r#"
        package TestPackage {
            classifier {
                feature x;
            }
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Package should exist
    assert!(symbol_table.lookup("TestPackage").is_some());

    // Anonymous classifier shouldn't create a symbol, but its feature should
    let all_symbols = symbol_table.all_symbols();
    assert!(all_symbols.iter().any(|(name, symbol)| {
        *name == "x" && matches!(symbol, Symbol::Feature { .. })
    }));
}

#[test]
fn test_kerml_visitor_handles_anonymous_feature() {
    let source = r#"
        classifier Vehicle {
            feature;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // Classifier should exist
    assert!(symbol_table.lookup("Vehicle").is_some());

    // Anonymous feature should not create a symbol
    let all_symbols = symbol_table.all_symbols();
    assert_eq!(all_symbols.len(), 1); // Only Vehicle
}

// ============================================================
// Tests for complex nested structures
// ============================================================

#[test]
fn test_kerml_visitor_handles_deeply_nested_classifiers() {
    let source = r#"
        package Level1 {
            package Level2 {
                classifier Level3 {
                    feature deepFeature;
                }
            }
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // All elements should be created
    assert!(symbol_table.lookup("Level1").is_some());
    
    let all_symbols = symbol_table.all_symbols();
    assert!(all_symbols.iter().any(|(name, _)| *name == "Level2"));
    assert!(all_symbols.iter().any(|(name, _)| *name == "Level3"));
    assert!(all_symbols.iter().any(|(name, _)| *name == "deepFeature"));
}
