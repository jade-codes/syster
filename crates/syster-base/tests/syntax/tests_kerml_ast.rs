//! KerML AST construction tests
//!
//! These tests verify that parsing produces correct AST structures.

#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use pest::Parser;
use syster::parser::KerMLParser;
use syster::parser::kerml::Rule;
use syster::syntax::kerml::ast::{
    ClassifierKind, ClassifierMember, Element as AstElement, FeatureMember, parse_file,
};

#[test]
fn test_parse_scalar_values_stdlib_file() {
    let content = r#"standard library package ScalarValues {
    private import Base::DataValue;
    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
}"#;

    let pairs = KerMLParser::parse(Rule::file, content).unwrap();
    for pair in pairs.clone() {
        for inner in pair.into_inner() {
            for _inner2 in inner.into_inner() {}
        }
    }

    // Try to convert to KerMLFile
    let mut pairs = KerMLParser::parse(Rule::file, content).unwrap();
    let file = parse_file(&mut pairs).unwrap();
    for _elem in file.elements.iter() {}

    assert!(!file.elements.is_empty(), "File should have elements!");
}

#[test]
fn test_parse_classifier_with_specialization_ast() {
    let input = "classifier Car specializes Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Car".to_string()));
            assert_eq!(c.body.len(), 1, "Classifier should have 1 body member");
            match &c.body[0] {
                ClassifierMember::Specialization(s) => {
                    assert_eq!(s.general, "Vehicle");
                }
                _ => panic!("Expected Specialization"),
            }
        }
        _ => panic!("Expected Classifier"),
    }
}

#[test]
fn test_parse_classifier_with_multiple_specializations_ast() {
    let input = "classifier SportsCar specializes Car, Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("SportsCar".to_string()));
            assert_eq!(c.body.len(), 2, "Should have 2 specializations");

            let generals: Vec<String> = c
                .body
                .iter()
                .filter_map(|m| match m {
                    ClassifierMember::Specialization(s) => Some(s.general.clone()),
                    _ => None,
                })
                .collect();

            assert!(generals.contains(&"Car".to_string()));
            assert!(generals.contains(&"Vehicle".to_string()));
        }
        _ => panic!("Expected Classifier"),
    }
}

#[test]
fn test_parse_feature_with_typing_ast() {
    let input = "feature mass : Real;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("mass".to_string()));
            assert_eq!(f.body.len(), 1, "Feature should have 1 body member");
            match &f.body[0] {
                FeatureMember::Typing(t) => {
                    assert_eq!(t.typed, "Real");
                }
                _ => panic!("Expected Typing"),
            }
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_feature_with_redefinition_ast() {
    let input = "feature currentMass redefines mass;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("currentMass".to_string()));
            assert_eq!(f.body.len(), 1, "Feature should have 1 body member");
            match &f.body[0] {
                FeatureMember::Redefinition(r) => {
                    assert_eq!(r.redefined, "mass");
                }
                _ => panic!("Expected Redefinition"),
            }
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_feature_with_subsetting_ast() {
    let input = "feature wheelMass subsets mass;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("wheelMass".to_string()));
            assert_eq!(f.body.len(), 1, "Feature should have 1 body member");
            match &f.body[0] {
                FeatureMember::Subsetting(s) => {
                    assert_eq!(s.subset, "mass");
                }
                _ => panic!("Expected Subsetting"),
            }
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_feature_with_typing_and_redefinition_ast() {
    let input = "feature currentMass : Real redefines mass;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("currentMass".to_string()));
            assert_eq!(f.body.len(), 2, "Feature should have 2 body members");

            let has_typing = f
                .body
                .iter()
                .any(|m| matches!(m, FeatureMember::Typing(t) if t.typed == "Real"));
            let has_redef = f
                .body
                .iter()
                .any(|m| matches!(m, FeatureMember::Redefinition(r) if r.redefined == "mass"));

            assert!(has_typing, "Should have typing relationship");
            assert!(has_redef, "Should have redefinition relationship");
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_abstract_classifier_ast() {
    let input = "abstract classifier Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Vehicle".to_string()));
            assert!(c.is_abstract, "Classifier should be abstract");
        }
        _ => panic!("Expected Classifier"),
    }
}

#[test]
fn test_parse_readonly_feature_ast() {
    let input = r#"
        package Test {
            readonly feature id : String;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    // Extract the package and feature directly with assertions
    assert_eq!(file.elements.len(), 1, "Should have exactly one package");
    let AstElement::Package(pkg) = &file.elements[0] else {
        panic!("Expected Package, got {:?}", file.elements[0]);
    };

    assert_eq!(
        pkg.elements.len(),
        1,
        "Package should have exactly one feature"
    );
    let AstElement::Feature(f) = &pkg.elements[0] else {
        panic!("Expected Feature, got {:?}", pkg.elements[0]);
    };

    assert_eq!(f.name, Some("id".to_string()));
    assert!(f.is_readonly, "Feature should be readonly");
}

#[test]
fn test_parse_datatype_ast() {
    let input = "datatype Real;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Real".to_string()));
            assert_eq!(c.kind, ClassifierKind::DataType);
        }
        _ => panic!("Expected Classifier (DataType)"),
    }
}

#[test]
fn test_parse_function_ast() {
    let input = "function calculateArea;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("calculateArea".to_string()));
            assert_eq!(c.kind, ClassifierKind::Function);
        }
        _ => panic!("Expected Classifier (Function)"),
    }
}

#[test]
fn test_parse_classifier_with_nested_feature_ast() {
    let input = r#"classifier Vehicle {
        feature mass : Real;
    }"#;
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Vehicle".to_string()));
            assert_eq!(c.body.len(), 1, "Classifier should have 1 nested feature");
            match &c.body[0] {
                ClassifierMember::Feature(f) => {
                    assert_eq!(f.name, Some("mass".to_string()));
                    assert_eq!(f.body.len(), 1, "Feature should have typing");
                    match &f.body[0] {
                        FeatureMember::Typing(t) => {
                            assert_eq!(t.typed, "Real");
                        }
                        _ => panic!("Expected Typing"),
                    }
                }
                _ => panic!("Expected Feature"),
            }
        }
        _ => panic!("Expected Classifier"),
    }
}
