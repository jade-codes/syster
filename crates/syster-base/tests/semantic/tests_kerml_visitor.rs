#![allow(clippy::unwrap_used)]
use syster::semantic::resolver::Resolver;

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

    assert!(Resolver::new(&symbol_table).resolve("MyPackage").is_some());
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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("Vehicle").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("Temperature").unwrap();
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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("mass").unwrap();
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

    assert!(
        Resolver::new(&symbol_table)
            .resolve("OuterPackage")
            .is_some()
    );

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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("calculateArea").unwrap();
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

    assert!(Resolver::new(&symbol_table).resolve("Vehicle").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Car").is_some());

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

    assert!(Resolver::new(&symbol_table).resolve("Real").is_some());
    assert!(Resolver::new(&symbol_table).resolve("mass").is_some());

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

    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("Shape").unwrap();
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
    let resolver = Resolver::new(&symbol_table);
    let symbol = resolver.resolve("timestamp");
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

    assert!(
        Resolver::new(&symbol_table)
            .resolve("baseFeature")
            .is_some()
    );
    assert!(
        Resolver::new(&symbol_table)
            .resolve("derivedFeature")
            .is_some()
    );

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
    assert!(Resolver::new(&symbol_table).resolve("MyPackage").is_some());
}

#[test]
fn test_kerml_visitor_handles_multiple_packages() {
    let source = r#"
        package Package1;
        package Package2;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();
    for _e in file.elements.iter() {}

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();
    for (_name, _) in symbol_table.all_symbols() {}

    assert!(Resolver::new(&symbol_table).resolve("Package1").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Package2").is_some());
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

    assert!(
        Resolver::new(&symbol_table)
            .resolve("EmptyPackage")
            .is_some()
    );
}

#[test]
fn test_kerml_identifier_in_feature_value_not_treated_as_definition() {
    // Reproduces the issue from Performances.kerml where identifiers in feature_value expressions
    // (like "default thisPerformance") were incorrectly being treated as feature definitions
    let source = r#"
        behavior Performance {
            feature redefines dispatchScope default thisPerformance;
            feature thisPerformance : Performance [1] default self;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();
    let mut adapter = KermlAdapter::with_relationships(&mut symbol_table, &mut graph);

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
