use pest::Parser;
use syster::parser::kerml::KerMLParser;
use syster::parser::kerml::Rule;
use syster::syntax::kerml::ast::enums::{ClassifierMember, ImportKind};
use syster::syntax::kerml::ast::parsers::parse_classifier;

#[test]
fn test_import_normal() {
    let source = "classifier Vehicle { import Base::DataValue; }";

    let parsed = KerMLParser::parse(Rule::classifier, source)
        .expect("Should parse")
        .next()
        .expect("Should have pair");

    let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

    assert_eq!(classifier.body.len(), 1, "Should have 1 import");

    if let ClassifierMember::Import(import) = &classifier.body[0] {
        assert_eq!(import.path, "Base::DataValue");
        assert!(!import.is_recursive);
        assert_eq!(import.kind, ImportKind::Normal);
    } else {
        panic!("First member should be an Import");
    }
}

#[test]
fn test_import_all_recursive() {
    let source = "classifier Vehicle { import all Base::DataValue; }";

    let parsed = KerMLParser::parse(Rule::classifier, source)
        .expect("Should parse")
        .next()
        .expect("Should have pair");

    let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

    assert_eq!(classifier.body.len(), 1, "Should have 1 import");

    if let ClassifierMember::Import(import) = &classifier.body[0] {
        assert_eq!(import.path, "Base::DataValue");
        assert!(import.is_recursive);
        assert_eq!(import.kind, ImportKind::Normal);
    } else {
        panic!("First member should be an Import");
    }
}

#[test]
fn test_import_with_members_kind() {
    let source = "classifier Vehicle { import Base::DataValue::*; }";

    let parsed = KerMLParser::parse(Rule::classifier, source)
        .expect("Should parse")
        .next()
        .expect("Should have pair");

    let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

    assert_eq!(classifier.body.len(), 1, "Should have 1 import");

    if let ClassifierMember::Import(import) = &classifier.body[0] {
        assert_eq!(import.path, "Base::DataValue");
        assert!(!import.is_recursive);
        assert_eq!(import.kind, ImportKind::All);
    } else {
        panic!("First member should be an Import");
    }
}

#[test]
fn test_import_with_members_recursive_kind() {
    let source = "classifier Vehicle { import Base::DataValue::**; }";

    let parsed = KerMLParser::parse(Rule::classifier, source)
        .expect("Should parse")
        .next()
        .expect("Should have pair");

    let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

    assert_eq!(classifier.body.len(), 1, "Should have 1 import");

    if let ClassifierMember::Import(import) = &classifier.body[0] {
        assert_eq!(import.path, "Base::DataValue");
        assert!(import.is_recursive);
        assert_eq!(import.kind, ImportKind::Recursive);
    } else {
        panic!("First member should be an Import");
    }
}

#[test]
fn test_import_with_all_recursive_kind() {
    let source = "classifier Vehicle { import Base::DataValue::*::**; }";

    let parsed = KerMLParser::parse(Rule::classifier, source)
        .expect("Should parse")
        .next()
        .expect("Should have pair");

    let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

    assert_eq!(classifier.body.len(), 1, "Should have 1 import");

    if let ClassifierMember::Import(import) = &classifier.body[0] {
        assert_eq!(import.path, "Base::DataValue");
        assert!(import.is_recursive);
        assert_eq!(import.kind, ImportKind::All);
    } else {
        panic!("First member should be an Import");
    }
}

#[test]
fn test_import_all_with_members_recursive() {
    let source = "classifier Vehicle { import all Base::DataValue::**; }";

    let parsed = KerMLParser::parse(Rule::classifier, source)
        .expect("Should parse")
        .next()
        .expect("Should have pair");

    let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

    assert_eq!(classifier.body.len(), 1, "Should have 1 import");

    if let ClassifierMember::Import(import) = &classifier.body[0] {
        assert_eq!(import.path, "Base::DataValue");
        assert!(import.is_recursive);
        assert_eq!(import.kind, ImportKind::Recursive);
    } else {
        panic!("First member should be an Import");
    }
}
