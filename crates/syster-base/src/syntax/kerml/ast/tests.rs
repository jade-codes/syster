#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::{
    parser::kerml::{KerMLParser, Rule},
    syntax::kerml::Documentation,
};
use ::from_pest::FromPest;
use pest::Parser;

#[test]
fn test_kerml_package_from_pest() {
    let input = "package TestPackage { }";
    let mut pairs = KerMLParser::parse(Rule::package, input).unwrap();
    let package = Package::from_pest(&mut pairs).unwrap();
    assert_eq!(package.name, Some("TestPackage".to_string()));
    assert_eq!(package.elements.len(), 0);
}

#[test]
fn test_kerml_comment() {
    let input = "comment /* This is a test */";
    let mut pairs = KerMLParser::parse(Rule::comment_annotation, input).unwrap();
    let comment = Comment::from_pest(&mut pairs).unwrap();
    assert!(comment.content.contains("This is a test"));
}

#[test]
fn test_kerml_doc() {
    let input = "doc /* This is a test */";
    let mut pairs = KerMLParser::parse(Rule::documentation, input).unwrap();
    let doc = Documentation::from_pest(&mut pairs).unwrap();
    assert!(doc.comment.content.contains("/* This is a test */"));
}

#[test]
fn test_kerml_type_classifier() {
    let input = "type MyType;";
    let mut pairs = KerMLParser::parse(Rule::type_def, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Type);
    assert_eq!(classifier.name, Some("MyType".to_string()));
    assert!(!classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_classifier() {
    let input = "abstract class MyClass;";

    let mut pairs = KerMLParser::parse(Rule::class, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Class);
    assert_eq!(classifier.name, Some("MyClass".to_string()));
    assert!(
        classifier.is_abstract,
        "Expected classifier to be abstract but is_abstract was false"
    );
}

#[test]
fn test_kerml_datatype() {
    let input = "datatype MyData;";
    let mut pairs = KerMLParser::parse(Rule::data_type, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::DataType);
    assert_eq!(classifier.name, Some("MyData".to_string()));
}

#[test]
fn test_kerml_structure() {
    let input = "struct MyStruct;";
    let mut pairs = KerMLParser::parse(Rule::structure, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Structure);
    assert_eq!(classifier.name, Some("MyStruct".to_string()));
}

#[test]
fn test_kerml_behavior() {
    let input = "behavior MyBehavior;";
    let mut pairs = KerMLParser::parse(Rule::behavior, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Behavior);
    assert_eq!(classifier.name, Some("MyBehavior".to_string()));
}

#[test]
fn test_kerml_function() {
    let input = "function MyFunc;";
    let mut pairs = KerMLParser::parse(Rule::function, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Function);
    assert_eq!(classifier.name, Some("MyFunc".to_string()));
}

#[test]
fn test_kerml_package_parsing() {
    let input = "package TestPkg { }";
    let mut pairs = KerMLParser::parse(Rule::package, input).unwrap();
    let package = Package::from_pest(&mut pairs).unwrap();
    assert_eq!(package.name, Some("TestPkg".to_string()));
    assert_eq!(package.elements.len(), 0);
}

#[test]
fn test_kerml_class_parsing() {
    let input = "class MyClass;";
    let mut pairs = KerMLParser::parse(Rule::class, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Class);
    assert_eq!(classifier.name, Some("MyClass".to_string()));
    assert!(!classifier.is_abstract);
}

#[test]
fn test_kerml_association() {
    let input = "assoc MyAssoc;";
    let mut pairs = KerMLParser::parse(Rule::association, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Association);
    assert_eq!(classifier.name, Some("MyAssoc".to_string()));
    assert!(!classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_association() {
    let input = "abstract assoc AbstractAssoc;";
    let mut pairs = KerMLParser::parse(Rule::association, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Association);
    assert_eq!(classifier.name, Some("AbstractAssoc".to_string()));
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_association_structure() {
    let input = "assoc struct MyAssocStruct;";
    let mut pairs = KerMLParser::parse(Rule::association_structure, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::AssociationStructure);
    assert_eq!(classifier.name, Some("MyAssocStruct".to_string()));
}

#[test]
fn test_kerml_metaclass() {
    let input = "metaclass MyMetaclass;";
    let mut pairs = KerMLParser::parse(Rule::metaclass, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Metaclass);
    assert_eq!(classifier.name, Some("MyMetaclass".to_string()));
}

#[test]
fn test_kerml_feature() {
    let input = "feature myFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = Feature::from_pest(&mut pairs).unwrap();
    assert_eq!(feature.name, Some("myFeature".to_string()));
    assert_eq!(feature.direction, None);
    assert!(!feature.is_readonly);
    assert!(!feature.is_derived);
}

#[test]
fn test_kerml_feature_with_direction() {
    let input = "in feature inputFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = Feature::from_pest(&mut pairs).unwrap();
    assert_eq!(feature.name, Some("inputFeature".to_string()));
    assert_eq!(feature.direction, Some(FeatureDirection::In));
}

#[test]
fn test_kerml_readonly_feature() {
    let input = "readonly feature readOnlyFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = Feature::from_pest(&mut pairs).unwrap();
    assert_eq!(feature.name, Some("readOnlyFeature".to_string()));
    assert!(feature.is_readonly);
    assert!(!feature.is_derived);
}

#[test]
fn test_kerml_derived_feature() {
    let input = "derived feature derivedFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = Feature::from_pest(&mut pairs).unwrap();
    assert_eq!(feature.name, Some("derivedFeature".to_string()));
    assert!(!feature.is_readonly);
    assert!(feature.is_derived);
}

#[test]
fn test_kerml_import() {
    let input = "import MyPackage::*;";
    let mut pairs = KerMLParser::parse(Rule::import, input).unwrap();
    let import = Import::from_pest(&mut pairs).unwrap();
    assert!(!import.path.is_empty());
}

#[test]
fn test_kerml_classifier_keyword() {
    let input = "classifier MyClassifier;";
    let mut pairs = KerMLParser::parse(Rule::classifier, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Classifier);
    assert_eq!(classifier.name, Some("MyClassifier".to_string()));
}

#[test]
fn test_kerml_abstract_type() {
    let input = "abstract type AbstractType;";
    let mut pairs = KerMLParser::parse(Rule::type_def, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Type);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_datatype() {
    let input = "abstract datatype AbstractData;";
    let mut pairs = KerMLParser::parse(Rule::data_type, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::DataType);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_structure() {
    let input = "abstract struct AbstractStruct;";
    let mut pairs = KerMLParser::parse(Rule::structure, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Structure);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_behavior() {
    let input = "abstract behavior AbstractBehavior;";
    let mut pairs = KerMLParser::parse(Rule::behavior, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Behavior);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_function() {
    let input = "abstract function AbstractFunc;";
    let mut pairs = KerMLParser::parse(Rule::function, input).unwrap();
    let classifier = Classifier::from_pest(&mut pairs).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Function);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_feature_with_multiple_modifiers() {
    let input = "in readonly feature multiModFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = Feature::from_pest(&mut pairs).unwrap();
    assert_eq!(feature.name, Some("multiModFeature".to_string()));
    assert_eq!(feature.direction, Some(FeatureDirection::In));
    assert!(feature.is_readonly);
}

#[test]
fn test_kerml_out_feature() {
    let input = "out feature outputFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = Feature::from_pest(&mut pairs).unwrap();
    assert_eq!(feature.direction, Some(FeatureDirection::Out));
}

#[test]
fn test_kerml_inout_feature() {
    let input = "inout feature bidirectionalFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = Feature::from_pest(&mut pairs).unwrap();
    assert_eq!(feature.direction, Some(FeatureDirection::InOut));
}
