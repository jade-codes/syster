#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use crate::{
    parser::kerml::{KerMLParser, Rule},
    syntax::kerml::ast::parsers::{
        parse_classifier, parse_comment, parse_documentation, parse_feature, parse_import,
        parse_package,
    },
};
use pest::Parser;

#[test]
fn test_kerml_package_from_pest() {
    let input = "package TestPackage { }";
    let mut pairs = KerMLParser::parse(Rule::package, input).unwrap();
    let package = parse_package(&mut pairs).unwrap();
    assert_eq!(package.name, Some("TestPackage".to_string()));
    assert_eq!(package.elements.len(), 0);
}

#[test]
fn test_kerml_comment() {
    let input = "comment /* This is a test */";
    let mut pairs = KerMLParser::parse(Rule::comment_annotation, input).unwrap();
    let comment = parse_comment(&mut pairs).unwrap();
    assert!(comment.content.contains("This is a test"));
}

#[test]
fn test_kerml_doc() {
    let input = "doc /* This is a test */";
    let mut pairs = KerMLParser::parse(Rule::documentation, input).unwrap();
    let doc = parse_documentation(&mut pairs).unwrap();
    assert!(doc.comment.content.contains("/* This is a test */"));
}

#[test]
fn test_kerml_type_classifier() {
    let input = "type MyType;";
    let mut pairs = KerMLParser::parse(Rule::type_def, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Type);
    assert_eq!(classifier.name, Some("MyType".to_string()));
    assert!(!classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_classifier() {
    let input = "abstract class MyClass;";

    let mut pairs = KerMLParser::parse(Rule::class, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
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
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::DataType);
    assert_eq!(classifier.name, Some("MyData".to_string()));
}

#[test]
fn test_kerml_structure() {
    let input = "struct MyStruct;";
    let mut pairs = KerMLParser::parse(Rule::structure, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Structure);
    assert_eq!(classifier.name, Some("MyStruct".to_string()));
}

#[test]
fn test_kerml_behavior() {
    let input = "behavior MyBehavior;";
    let mut pairs = KerMLParser::parse(Rule::behavior, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Behavior);
    assert_eq!(classifier.name, Some("MyBehavior".to_string()));
}

#[test]
fn test_kerml_function() {
    let input = "function MyFunc;";
    let mut pairs = KerMLParser::parse(Rule::function, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Function);
    assert_eq!(classifier.name, Some("MyFunc".to_string()));
}

#[test]
fn test_kerml_package_parsing() {
    let input = "package TestPkg { }";
    let mut pairs = KerMLParser::parse(Rule::package, input).unwrap();
    let package = parse_package(&mut pairs).unwrap();
    assert_eq!(package.name, Some("TestPkg".to_string()));
    assert_eq!(package.elements.len(), 0);
}

#[test]
fn test_kerml_class_parsing() {
    let input = "class MyClass;";
    let mut pairs = KerMLParser::parse(Rule::class, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Class);
    assert_eq!(classifier.name, Some("MyClass".to_string()));
    assert!(!classifier.is_abstract);
}

#[test]
fn test_kerml_association() {
    let input = "assoc MyAssoc;";
    let mut pairs = KerMLParser::parse(Rule::association, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Association);
    assert_eq!(classifier.name, Some("MyAssoc".to_string()));
    assert!(!classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_association() {
    let input = "abstract assoc AbstractAssoc;";
    let mut pairs = KerMLParser::parse(Rule::association, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Association);
    assert_eq!(classifier.name, Some("AbstractAssoc".to_string()));
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_association_structure() {
    let input = "assoc struct MyAssocStruct;";
    let mut pairs = KerMLParser::parse(Rule::association_structure, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::AssociationStructure);
    assert_eq!(classifier.name, Some("MyAssocStruct".to_string()));
}

#[test]
fn test_kerml_metaclass() {
    let input = "metaclass MyMetaclass;";
    let mut pairs = KerMLParser::parse(Rule::metaclass, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Metaclass);
    assert_eq!(classifier.name, Some("MyMetaclass".to_string()));
}

#[test]
fn test_kerml_feature() {
    let input = "feature myFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = parse_feature(pairs.next().unwrap());
    assert_eq!(feature.name, Some("myFeature".to_string()));
    assert_eq!(feature.direction, None);
    assert!(!feature.is_const);
    assert!(!feature.is_derived);
}

#[test]
fn test_kerml_feature_with_direction() {
    let input = "in feature inputFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = parse_feature(pairs.next().unwrap());
    assert_eq!(feature.name, Some("inputFeature".to_string()));
    assert_eq!(feature.direction, Some(FeatureDirection::In));
}

#[test]
fn test_kerml_const_feature() {
    let input = "const feature constFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = parse_feature(pairs.next().unwrap());
    assert_eq!(feature.name, Some("constFeature".to_string()));
    assert!(feature.is_const);
    assert!(!feature.is_derived);
}

#[test]
fn test_kerml_derived_feature() {
    let input = "derived feature derivedFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = parse_feature(pairs.next().unwrap());
    assert_eq!(feature.name, Some("derivedFeature".to_string()));
    assert!(!feature.is_const);
    assert!(feature.is_derived);
}

#[test]
fn test_kerml_import() {
    let input = "import MyPackage::*;";
    let mut pairs = KerMLParser::parse(Rule::import, input).unwrap();
    // Import::from_pest expects the children of the import rule
    let import_pair = pairs.next().unwrap();
    let import = parse_import(&mut import_pair.into_inner()).unwrap();
    assert!(!import.path.is_empty());
}

#[test]
fn test_kerml_classifier_keyword() {
    let input = "classifier MyClassifier;";
    let mut pairs = KerMLParser::parse(Rule::classifier, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Classifier);
    assert_eq!(classifier.name, Some("MyClassifier".to_string()));
}

#[test]
fn test_kerml_abstract_type() {
    let input = "abstract type AbstractType;";
    let mut pairs = KerMLParser::parse(Rule::type_def, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Type);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_datatype() {
    let input = "abstract datatype AbstractData;";
    let mut pairs = KerMLParser::parse(Rule::data_type, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::DataType);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_structure() {
    let input = "abstract struct AbstractStruct;";
    let mut pairs = KerMLParser::parse(Rule::structure, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Structure);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_behavior() {
    let input = "abstract behavior AbstractBehavior;";
    let mut pairs = KerMLParser::parse(Rule::behavior, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Behavior);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_abstract_function() {
    let input = "abstract function AbstractFunc;";
    let mut pairs = KerMLParser::parse(Rule::function, input).unwrap();
    let classifier = parse_classifier(pairs.next().unwrap()).unwrap();
    assert_eq!(classifier.kind, ClassifierKind::Function);
    assert!(classifier.is_abstract);
}

#[test]
fn test_kerml_feature_with_multiple_modifiers() {
    let input = "in const feature multiModFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = parse_feature(pairs.next().unwrap());
    assert_eq!(feature.name, Some("multiModFeature".to_string()));
    assert_eq!(feature.direction, Some(FeatureDirection::In));
    assert!(feature.is_const);
}

#[test]
fn test_kerml_out_feature() {
    let input = "out feature outputFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = parse_feature(pairs.next().unwrap());
    assert_eq!(feature.direction, Some(FeatureDirection::Out));
}

#[test]
fn test_kerml_inout_feature() {
    let input = "inout feature bidirectionalFeature;";
    let mut pairs = KerMLParser::parse(Rule::feature, input).unwrap();
    let feature = parse_feature(pairs.next().unwrap());
    assert_eq!(feature.direction, Some(FeatureDirection::InOut));
}

#[test]
fn test_find_identifier_span_skips_feature_value() {
    // Test that identifiers in feature_value expressions are not extracted as names
    // This is an ANONYMOUS feature (no name) that redefines dispatchScope with a default value
    let input = "feature redefines dispatchScope default thisPerformance;";
    let pairs = KerMLParser::parse(Rule::feature, input).unwrap();

    let (name, _span) = crate::syntax::kerml::ast::utils::find_identifier_span(pairs);

    // Should return None since this is an anonymous feature (no identification)
    // "dispatchScope" is in the redefinition clause, not the feature name
    // "thisPerformance" is in the default value expression
    assert_eq!(name, None);
}

#[test]
fn test_find_identifier_span_skips_relationship_parts() {
    // Test that identifiers in specialization/redefinition parts are skipped
    let input = "feature myFeature specializes BaseFeature::someFeature;";
    let pairs = KerMLParser::parse(Rule::feature, input).unwrap();

    let (name, _span) = crate::syntax::kerml::ast::utils::find_identifier_span(pairs);

    // Should find "myFeature", not "BaseFeature" or "someFeature" from the specialization
    assert_eq!(name, Some("myFeature".to_string()));
}

#[test]
fn test_find_identifier_span_with_feature_typing() {
    // Test that typed features work correctly
    let input = "feature myFeature : MyType;";
    let pairs = KerMLParser::parse(Rule::feature, input).unwrap();

    let (name, _span) = crate::syntax::kerml::ast::utils::find_identifier_span(pairs);

    // Should find "myFeature", not "MyType" from the typing
    assert_eq!(name, Some("myFeature".to_string()));
}

#[test]
fn test_find_name_skips_feature_value() {
    // Test that find_name correctly handles anonymous features with redefinitions
    // This is an ANONYMOUS feature (no name) that redefines dispatchScope with a default value
    let input = "feature redefines dispatchScope default thisPerformance;";
    let pairs = KerMLParser::parse(Rule::feature, input).unwrap();

    let name = crate::syntax::kerml::ast::utils::find_name(pairs);

    // Should return None since this is an anonymous feature (no identification)
    // "dispatchScope" is in the redefinition clause, not the feature name
    // "thisPerformance" is in the default value expression
    assert_eq!(name, None);
}

#[test]
fn test_find_name_fallback_when_no_identifier_span() {
    // Test that find_name works as a fallback when identifier extraction fails
    let input = "feature testFeature;";
    let pairs = KerMLParser::parse(Rule::feature, input).unwrap();

    let name = crate::syntax::kerml::ast::utils::find_name(pairs);

    assert_eq!(name, Some("testFeature".to_string()));
}

#[test]
fn test_find_name_struct_with_qualified_specialization() {
    // Test that struct names are extracted correctly even with qualified specializations
    let input = "struct MyStruct specializes Base::Parent;";
    let pairs = KerMLParser::parse(Rule::structure, input).unwrap();

    let name = crate::syntax::kerml::ast::utils::find_name(pairs);

    // Should find "MyStruct", not "Base" or "Parent" from the specialization
    assert_eq!(name, Some("MyStruct".to_string()));
}

#[test]
fn test_find_name_class_with_multiple_relationships() {
    // Test class with multiple heritage relationships
    let input = "class MyClass specializes Foo::Bar, Baz;";
    let pairs = KerMLParser::parse(Rule::class, input).unwrap();

    let name = crate::syntax::kerml::ast::utils::find_name(pairs);

    // Should find "MyClass" only
    assert_eq!(name, Some("MyClass".to_string()));
}
