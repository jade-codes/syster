#![allow(clippy::unwrap_used)]
use syster::semantic::resolver::Resolver;

use pest::Parser;
use syster::parser::{KerMLParser, kerml::Rule};
use syster::semantic::adapters::KermlAdapter;
use syster::semantic::graphs::ReferenceIndex;
use syster::semantic::symbol_table::{Symbol, SymbolTable};
use syster::syntax::kerml::ast::parse_file;

#[test]
fn test_kerml_visitor_creates_package_symbol() {
    let source = "package MyPackage;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(Resolver::new(&symbol_table).resolve("MyPackage").is_some());
}

#[test]
fn test_kerml_visitor_creates_classifier_symbol() {
    let source = "classifier Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
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
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
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
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
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
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(
        Resolver::new(&symbol_table)
            .resolve("OuterPackage")
            .is_some()
    );

    // Nested elements must be looked up via iter_symbols since they're in a nested scope
    let inner = symbol_table
        .iter_symbols()
        .find(|sym| sym.name() == "InnerClassifier")
        .expect("Should have 'InnerClassifier' symbol");

    match inner {
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
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
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
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    symbol_table.set_current_file(Some("test.kerml".to_string()));
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(Resolver::new(&symbol_table).resolve("Vehicle").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Car").is_some());

    // Car references Vehicle via specialization
    // get_sources("Vehicle") should contain "Car"
    let sources = graph.get_sources("Vehicle");
    assert!(
        sources.contains(&"Car"),
        "Car should be in sources for Vehicle"
    );
}

#[test]
fn test_kerml_visitor_handles_feature_typing() {
    let source = r#"
        datatype Real;
        feature mass : Real;
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    symbol_table.set_current_file(Some("test.kerml".to_string()));
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    assert!(Resolver::new(&symbol_table).resolve("Real").is_some());
    assert!(Resolver::new(&symbol_table).resolve("mass").is_some());

    // mass references Real via typing
    // get_sources("Real") should contain "mass"
    let sources = graph.get_sources("Real");
    assert!(
        sources.contains(&"mass"),
        "mass should be in sources for Real"
    );
}

#[test]
fn test_kerml_visitor_handles_abstract_classifiers() {
    let source = "abstract classifier Shape;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
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
fn test_kerml_visitor_handles_const_features() {
    let source = "const feature timestamp;";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();

    // For now, just verify the symbol exists - const modifier tracking will be added later
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
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
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

    // derivedFeature references baseFeature via redefinition
    // This is a placeholder check since derivedFeature may reference its base
    let _has_refs = !graph.get_sources("baseFeature").is_empty()
        || !graph.get_sources("derivedFeature").is_empty();
    // TODO: Add proper assertion once redefinition relationships are fully implemented
}

#[test]
fn test_kerml_visitor_handles_imports() {
    let source = r#"
        package MyPackage {
            import OtherPackage::*;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);

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
    let file = parse_file(&mut pairs).unwrap();
    for _e in file.elements.iter() {}

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
    adapter.populate(&file).unwrap();
    for _sym in symbol_table.iter_symbols() {}

    assert!(Resolver::new(&symbol_table).resolve("Package1").is_some());
    assert!(Resolver::new(&symbol_table).resolve("Package2").is_some());
}

#[test]
fn test_kerml_visitor_handles_empty_package() {
    let source = "package EmptyPackage {}";
    let mut pairs = KerMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);
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
    let file = parse_file(&mut pairs).unwrap();

    let mut symbol_table = SymbolTable::new();
    let mut graph = ReferenceIndex::new();
    let mut adapter = KermlAdapter::with_index(&mut symbol_table, &mut graph);

    let result = adapter.populate(&file);

    // Should not have duplicate symbol errors
    assert!(
        result.is_ok(),
        "Should not have duplicate symbol errors, got: {:?}",
        result.err()
    );

    // Should have exactly one "thisPerformance" symbol (the actual definition, not the reference)
    let this_perf_count = symbol_table
        .iter_symbols()
        .filter(|sym| sym.name() == "thisPerformance")
        .count();

    assert_eq!(
        this_perf_count, 1,
        "Should have exactly one 'thisPerformance' definition, got {this_perf_count}"
    );
}
