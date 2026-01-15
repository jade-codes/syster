//! KerML AST parsing.
//!
//! This module provides parsing for efficient AST construction.

use super::enums::{ClassifierMember, Element, FeatureMember, ImportKind};
use super::types::{
    Annotation, Classifier, Comment, Feature, Import, KerMLFile, NamespaceDeclaration, Package,
    Redefinition, Specialization, Subsetting, TypingRelationship,
};
use super::utils::{
    extract_direction, extract_flags, find_identifier_span, find_name, is_classifier_rule,
    to_classifier_kind, to_span,
};
use crate::core::Span;
use crate::parser::kerml::Rule;
use crate::syntax::kerml::model::types::Documentation;
use pest::iterators::{Pair, Pairs};

/// Parse error type for AST construction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn no_match() -> Self {
        Self {
            message: "No matching rule".to_string(),
        }
    }

    pub fn invalid_rule(rule: &str) -> Self {
        Self {
            message: format!("Invalid rule: {rule}"),
        }
    }
}

// ============================================================================
// Body parsing
// ============================================================================

/// Parse classifier body members
fn parse_classifier_body(pair: &Pair<Rule>) -> Vec<ClassifierMember> {
    let mut members = Vec::new();
    extract_classifier_members(pair, &mut members);
    members
}

fn extract_classifier_members(pair: &Pair<Rule>, members: &mut Vec<ClassifierMember>) {
    match pair.as_rule() {
        Rule::heritage => {
            // Heritage contains specialization, subsetting, redefinition, etc
            for inner in pair.clone().into_inner() {
                extract_classifier_members(&inner, members);
            }
        }
        Rule::specialization => {
            // Parse "specializes General"
            for inner in pair.clone().into_inner() {
                if let Rule::inheritance = inner.as_rule()
                    && let Some(general) = extract_reference(&inner)
                {
                    members.push(ClassifierMember::Specialization(Specialization {
                        general,
                        span: Some(to_span(inner.as_span())),
                    }));
                }
            }
        }
        Rule::feature => {
            members.push(ClassifierMember::Feature(parse_feature(pair.clone())));
        }
        Rule::parameter_membership | Rule::return_parameter_membership => {
            // Function parameters like "in x: NumericalValue[1]" or "return : NumericalValue[1]"
            // Parse as features to capture their typing relationships
            members.push(ClassifierMember::Feature(parse_feature(pair.clone())));
        }
        Rule::import => {
            if let Some(path) = extract_import_path(pair) {
                let is_recursive = detect_is_recursive(pair);
                let kind = detect_import_kind(pair);
                let path_span = extract_import_path_span(pair);
                members.push(ClassifierMember::Import(Import {
                    path,
                    path_span,
                    is_recursive,
                    is_public: false, // Classifier-level imports default to private
                    kind,
                    span: Some(to_span(pair.as_span())),
                }));
            }
        }
        _ => {
            for inner in pair.clone().into_inner() {
                extract_classifier_members(&inner, members);
            }
        }
    }
}

/// Parse feature body members
fn parse_feature_body(pair: &Pair<Rule>) -> Vec<FeatureMember> {
    let mut members = Vec::new();
    extract_feature_members(pair, &mut members);
    members
}

fn extract_feature_members(pair: &Pair<Rule>, members: &mut Vec<FeatureMember>) {
    match pair.as_rule() {
        Rule::feature_typing => {
            // Parse ": Type"
            for inner in pair.clone().into_inner() {
                if let Some(typed) = extract_reference(&inner) {
                    members.push(FeatureMember::Typing(TypingRelationship {
                        typed,
                        span: Some(to_span(inner.as_span())),
                    }));
                }
            }
        }
        Rule::redefinition => {
            // Parse "redefines Base"
            for inner in pair.clone().into_inner() {
                if let Rule::inheritance = inner.as_rule()
                    && let Some(redefined) = extract_reference(&inner)
                {
                    members.push(FeatureMember::Redefinition(Redefinition {
                        redefined,
                        span: Some(to_span(inner.as_span())),
                    }));
                }
            }
        }
        Rule::subsetting => {
            // Parse "subsets General"
            for inner in pair.clone().into_inner() {
                if let Rule::inheritance = inner.as_rule()
                    && let Some(subset) = extract_reference(&inner)
                {
                    members.push(FeatureMember::Subsetting(Subsetting {
                        subset,
                        span: Some(to_span(inner.as_span())),
                    }));
                }
            }
        }
        _ => {
            for inner in pair.clone().into_inner() {
                extract_feature_members(&inner, members);
            }
        }
    }
}

/// Extract a reference name from an inheritance or element_reference rule
fn extract_reference(pair: &Pair<Rule>) -> Option<String> {
    match pair.as_rule() {
        Rule::inheritance
        | Rule::element_reference
        | Rule::qualified_reference_chain
        | Rule::relationship
        | Rule::feature_type
        | Rule::feature_or_chain => {
            // Recursively search for qualified_reference_chain or identifier
            for inner in pair.clone().into_inner() {
                match inner.as_rule() {
                    Rule::qualified_reference_chain => {
                        return Some(inner.as_str().trim().to_string());
                    }
                    Rule::identifier => {
                        return Some(inner.as_str().trim().to_string());
                    }
                    _ => {
                        if let Some(found) = extract_reference(&inner) {
                            return Some(found);
                        }
                    }
                }
            }
            // If no inner rules found, try the text itself
            if matches!(
                pair.as_rule(),
                Rule::qualified_reference_chain | Rule::identifier
            ) {
                return Some(pair.as_str().trim().to_string());
            }
            None
        }
        Rule::identifier => Some(pair.as_str().trim().to_string()),
        _ => None,
    }
}

/// Extract import path
fn extract_import_path(pair: &Pair<Rule>) -> Option<String> {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::qualified_reference_chain => return Some(inner.as_str().trim().to_string()),
            Rule::element_reference => return extract_reference(&inner),
            _ => {
                if let Some(found) = extract_import_path(&inner) {
                    return Some(found);
                }
            }
        }
    }
    None
}

/// Extract import path span for semantic token highlighting
fn extract_import_path_span(pair: &Pair<Rule>) -> Option<Span> {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::qualified_reference_chain => return Some(to_span(inner.as_span())),
            Rule::element_reference => return Some(to_span(inner.as_span())),
            _ => {
                if let Some(found) = extract_import_path_span(&inner) {
                    return Some(found);
                }
            }
        }
    }
    None
}

/// Detect if an import is recursive by checking for "all" keyword or recursive import kinds
fn detect_is_recursive(pair: &Pair<Rule>) -> bool {
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::import_all => return true,
            Rule::import_kind => {
                // Check for recursive import kinds: "::**" or "::*::**"
                let kind_str = inner.as_str();
                if kind_str.contains("**") {
                    return true;
                }
            }
            _ => {
                if detect_is_recursive(&inner) {
                    return true;
                }
            }
        }
    }
    false
}

/// Detect import kind from the import_kind rule
fn detect_import_kind(pair: &Pair<Rule>) -> ImportKind {
    for inner in pair.clone().into_inner() {
        if let Rule::import_kind = inner.as_rule() {
            return match inner.as_str() {
                "::*" => ImportKind::All,
                "::**" => ImportKind::Recursive,
                "::*::**" => ImportKind::All,
                _ => ImportKind::Normal,
            };
        } else {
            let kind = detect_import_kind(&inner);
            if kind != ImportKind::Normal {
                return kind;
            }
        }
    }
    ImportKind::Normal
}

// ============================================================================
// Main parsers
// ============================================================================

/// Parse a classifier from a pest pair
pub fn parse_classifier(pair: Pair<Rule>) -> Result<Classifier, ParseError> {
    let kind = to_classifier_kind(pair.as_rule())?;
    let pairs: Vec<_> = pair.clone().into_inner().collect();

    // Find the identifier and its span
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    // Parse body by searching through all children
    let body = parse_classifier_body(&pair);

    Ok(Classifier {
        kind,
        is_abstract: pairs.iter().any(|p| p.as_rule() == Rule::abstract_marker),
        name,
        body,
        span,
    })
}

/// Parse a feature from a pest pair
pub fn parse_feature(pair: Pair<Rule>) -> Feature {
    let pairs: Vec<_> = pair.clone().into_inner().collect();
    let (is_const, is_derived) = extract_flags(&pairs);

    // Find the identifier and its span
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    // Parse body by searching through all children
    let body = parse_feature_body(&pair);

    Feature {
        name,
        direction: extract_direction(&pairs),
        is_const,
        is_derived,
        body,
        span,
    }
}

/// Parse a package from pest pairs
pub fn parse_package(pest: &mut Pairs<Rule>) -> Result<Package, ParseError> {
    let mut elements = Vec::new();
    let pairs: Vec<_> = pest.collect();

    for pair in &pairs {
        if pair.as_rule() == Rule::namespace_body {
            elements = pair
                .clone()
                .into_inner()
                .filter(|p| p.as_rule() == Rule::namespace_body_elements)
                .flat_map(|p| p.into_inner())
                .filter(|p| p.as_rule() == Rule::namespace_body_element)
                .filter_map(|p| parse_element(&mut p.into_inner()).ok())
                .collect();
        }
    }

    // Use find_identifier_span to get the name AND its span together
    // This ensures the span points to the package name, not 'standard' or 'library'
    let (name, span) = find_identifier_span(pairs.into_iter());

    Ok(Package {
        name,
        elements,
        span,
    })
}

/// Parse a comment from pest pairs
pub fn parse_comment(pest: &mut Pairs<Rule>) -> Result<Comment, ParseError> {
    let mut content = String::new();
    let mut span = None;

    for pair in pest {
        span.get_or_insert_with(|| to_span(pair.as_span()));
        if pair.as_rule() == Rule::comment_annotation {
            content = pair.as_str().to_string();
        }
    }

    Ok(Comment {
        content,
        about: Vec::new(),
        locale: None,
        span,
    })
}

/// Parse documentation from pest pairs
pub fn parse_documentation(pest: &mut Pairs<Rule>) -> Result<Documentation, ParseError> {
    let pair = pest.next().ok_or(ParseError::no_match())?;
    if pair.as_rule() != Rule::documentation {
        return Err(ParseError::no_match());
    }

    let span = Some(to_span(pair.as_span()));
    let content = pair.as_str().to_string();

    Ok(Documentation {
        comment: Comment {
            content,
            about: Vec::new(),
            locale: None,
            span,
        },
        span,
    })
}

/// Parse an import from pest pairs
pub fn parse_import(pest: &mut Pairs<Rule>) -> Result<Import, ParseError> {
    let mut path = String::new();
    let mut path_span = None;
    let mut is_recursive = false;
    let mut span = None;

    for pair in pest {
        if pair.as_rule() == Rule::imported_reference {
            span = Some(to_span(pair.as_span()));
            // imported_reference contains element_reference and optional import_kind
            for child in pair.into_inner() {
                match child.as_rule() {
                    Rule::element_reference => {
                        path = child.as_str().to_string();
                        path_span = Some(to_span(child.as_span()));
                    }
                    Rule::import_kind => {
                        is_recursive = child.as_str().contains("**");
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(Import {
        path,
        path_span,
        is_recursive,
        is_public: false, // Will be overridden by parse_element if public
        kind: ImportKind::Normal,
        span,
    })
}

/// Parse an element from pest pairs
pub fn parse_element(pest: &mut Pairs<Rule>) -> Result<Element, ParseError> {
    let mut pair = pest.next().ok_or(ParseError::no_match())?;
    let mut is_public = false;

    // Capture visibility prefix
    if pair.as_rule() == Rule::visibility_kind {
        is_public = pair.as_str().trim() == "public";
        pair = pest.next().ok_or(ParseError::no_match())?;
    }

    Ok(match pair.as_rule() {
        // Package rules
        Rule::package | Rule::library_package => {
            Element::Package(parse_package(&mut pair.into_inner())?)
        }

        // Wrapper rules - recurse
        Rule::namespace_body_element
        | Rule::non_feature_member
        | Rule::non_feature_element
        | Rule::namespace_feature_member
        | Rule::typed_feature_member => parse_element(&mut pair.into_inner())?,

        // Classifier rules
        r if is_classifier_rule(r) => Element::Classifier(parse_classifier(pair)?),

        // Feature rules
        Rule::feature | Rule::feature_element => Element::Feature(parse_feature(pair)),

        // Other elements
        Rule::comment_annotation => Element::Comment(parse_comment(&mut pair.into_inner())?),
        Rule::annotation => Element::Annotation(Annotation {
            reference: pair.as_str().to_string(),
            span: Some(to_span(pair.as_span())),
        }),
        Rule::import => {
            let mut import = parse_import(&mut pair.into_inner())?;
            import.is_public = is_public;
            Element::Import(import)
        }

        _ => return Err(ParseError::no_match()),
    })
}

/// Parse a KerMLFile from pest pairs
pub fn parse_file(pest: &mut Pairs<Rule>) -> Result<KerMLFile, ParseError> {
    let model = pest.next().ok_or(ParseError::no_match())?;
    if model.as_rule() != Rule::file {
        return Err(ParseError::no_match());
    }

    let mut elements = Vec::new();
    let mut namespace = None;

    for pair in model.into_inner() {
        if pair.as_rule() == Rule::namespace_element
            && let Ok(element) = parse_element(&mut pair.into_inner())
        {
            if let Element::Package(ref pkg) = element
                && namespace.is_none()
                && pkg.elements.is_empty()
                && let Some(ref name) = pkg.name
            {
                namespace = Some(NamespaceDeclaration {
                    name: name.clone(),
                    span: pkg.span,
                });
            }
            elements.push(element);
        }
    }

    Ok(KerMLFile {
        namespace,
        elements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::kerml::KerMLParser;
    use crate::syntax::kerml::ast::enums::ClassifierKind;
    use pest::Parser;

    #[test]
    fn test_classifier_with_specialization() {
        let source = "classifier Car specializes Vehicle;";

        let parsed = KerMLParser::parse(Rule::classifier, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        assert_eq!(classifier.name, Some("Car".to_string()));
        assert_eq!(classifier.body.len(), 1, "Should have 1 specialization");

        // Check specialization
        if let ClassifierMember::Specialization(spec) = &classifier.body[0] {
            assert_eq!(spec.general, "Vehicle");
            assert!(spec.span.is_some(), "Should have span for specialization");
        } else {
            panic!("First member should be a Specialization");
        }
    }

    #[test]
    fn test_classifier_with_multiple_specializations() {
        let source = "classifier SportsCar specializes Car, Vehicle;";

        let parsed = KerMLParser::parse(Rule::classifier, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        assert_eq!(classifier.name, Some("SportsCar".to_string()));
        assert_eq!(classifier.body.len(), 2, "Should have 2 specializations");

        // Check both specializations
        let specializations: Vec<_> = classifier
            .body
            .iter()
            .filter_map(|m| match m {
                ClassifierMember::Specialization(s) => Some(s.general.as_str()),
                _ => None,
            })
            .collect();

        assert_eq!(specializations, vec!["Car", "Vehicle"]);
    }

    #[test]
    fn test_feature_with_typing() {
        let source = "feature mass : Real;";

        let parsed = KerMLParser::parse(Rule::feature, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let feature = parse_feature(parsed);

        assert_eq!(feature.name, Some("mass".to_string()));
        assert_eq!(feature.body.len(), 1, "Should have 1 typing relationship");

        // Check typing
        if let FeatureMember::Typing(typing) = &feature.body[0] {
            assert_eq!(typing.typed, "Real");
            assert!(typing.span.is_some(), "Should have span for type");
        } else {
            panic!("First member should be a Typing relationship");
        }
    }

    #[test]
    fn test_feature_with_redefinition() {
        let source = "feature velocity redefines speed;";

        let parsed = KerMLParser::parse(Rule::feature, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let feature = parse_feature(parsed);

        assert_eq!(feature.name, Some("velocity".to_string()));
        assert_eq!(feature.body.len(), 1, "Should have 1 redefinition");

        // Check redefinition
        if let FeatureMember::Redefinition(redef) = &feature.body[0] {
            assert_eq!(redef.redefined, "speed");
            assert!(redef.span.is_some(), "Should have span for redefinition");
        } else {
            panic!("First member should be a Redefinition");
        }
    }

    #[test]
    fn test_feature_with_subsetting() {
        let source = "feature x subsets position;";

        let parsed = KerMLParser::parse(Rule::feature, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let feature = parse_feature(parsed);

        assert_eq!(feature.name, Some("x".to_string()));
        assert_eq!(feature.body.len(), 1, "Should have 1 subsetting");

        // Check subsetting
        if let FeatureMember::Subsetting(subset) = &feature.body[0] {
            assert_eq!(subset.subset, "position");
            assert!(subset.span.is_some(), "Should have span for subsetting");
        } else {
            panic!("First member should be a Subsetting");
        }
    }

    #[test]
    fn test_abstract_classifier() {
        let source = "abstract classifier Shape;";

        let parsed = KerMLParser::parse(Rule::classifier, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        assert_eq!(classifier.name, Some("Shape".to_string()));
        assert!(
            classifier.is_abstract,
            "Classifier should be marked as abstract"
        );
    }

    #[test]
    fn test_const_feature() {
        let source = "const feature constant : Real;";

        let parsed = KerMLParser::parse(Rule::feature, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let feature = parse_feature(parsed);

        assert_eq!(feature.name, Some("constant".to_string()));
        assert!(feature.is_const, "Feature should be marked as const");
    }

    #[test]
    fn test_classifier_with_name_span() {
        let source = "classifier Vehicle;";

        let parsed = KerMLParser::parse(Rule::classifier, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        assert_eq!(classifier.name, Some("Vehicle".to_string()));
        assert!(
            classifier.span.is_some(),
            "Classifier should have span for name"
        );
    }

    #[test]
    fn test_datatype_classifier() {
        let source = "datatype Real;";

        let parsed = KerMLParser::parse(Rule::data_type, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        assert_eq!(classifier.name, Some("Real".to_string()));
        assert_eq!(classifier.kind, ClassifierKind::DataType);
    }

    #[test]
    fn test_function_classifier() {
        let source = "function calculateArea;";

        let parsed = KerMLParser::parse(Rule::function, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        assert_eq!(classifier.name, Some("calculateArea".to_string()));
        assert_eq!(classifier.kind, ClassifierKind::Function);
    }

    #[test]
    fn test_function_with_parameters_extracts_typing() {
        // Test that function parameters like "in x: NumericalValue[1]" are parsed as features
        // with typing relationships, so their type references get semantic token highlighting
        let source = r#"abstract function '+' { in x: NumericalValue[1]; in y: NumericalValue[0..1]; return : NumericalValue[1]; }"#;

        let parsed = KerMLParser::parse(Rule::function, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        assert_eq!(classifier.name, Some("'+'".to_string()));
        assert_eq!(classifier.kind, ClassifierKind::Function);

        // Should have 3 features: in x, in y, return
        let features: Vec<_> = classifier
            .body
            .iter()
            .filter_map(|m| match m {
                ClassifierMember::Feature(f) => Some(f),
                _ => None,
            })
            .collect();

        assert_eq!(
            features.len(),
            3,
            "Should have 3 features (in x, in y, return)"
        );

        // Check that parameters have typing relationships captured
        for feature in &features {
            let typing_members: Vec<_> = feature
                .body
                .iter()
                .filter(|m| matches!(m, FeatureMember::Typing(_)))
                .collect();
            assert!(
                !typing_members.is_empty(),
                "Feature {:?} should have typing relationship for NumericalValue",
                feature.name
            );

            // Check that the typing has a span (needed for semantic tokens)
            if let Some(FeatureMember::Typing(typing)) = typing_members.first() {
                assert_eq!(typing.typed, "NumericalValue");
                assert!(
                    typing.span.is_some(),
                    "Typing relationship should have a span for semantic token highlighting"
                );
            }
        }
    }

    // ========================================================================
    // ParseError tests
    // ========================================================================

    #[test]
    fn test_parse_error_no_match() {
        let error = ParseError::no_match();
        assert_eq!(error.message, "No matching rule");
    }

    #[test]
    fn test_parse_error_invalid_rule() {
        let error = ParseError::invalid_rule("unknown_rule");
        assert_eq!(error.message, "Invalid rule: unknown_rule");
    }

    // ========================================================================
    // Parameter membership parsing tests
    // ========================================================================

    #[test]
    fn test_parameter_membership_parsed_as_feature() {
        // Test that parameter_membership (e.g., "in x: Type") is parsed as a Feature
        // so its typing relationship is captured for semantic tokens
        let source = "function compute { in value: Real; }";

        let parsed = KerMLParser::parse(Rule::function, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        // Find features in the body
        let features: Vec<_> = classifier
            .body
            .iter()
            .filter_map(|m| match m {
                ClassifierMember::Feature(f) => Some(f),
                _ => None,
            })
            .collect();

        assert!(
            !features.is_empty(),
            "Parameter should be parsed as feature"
        );

        // Check that the parameter has a typing relationship
        let param = features.first().expect("Should have parameter feature");
        let has_typing = param
            .body
            .iter()
            .any(|m| matches!(m, FeatureMember::Typing(_)));
        assert!(
            has_typing,
            "Parameter feature should have typing relationship for Real"
        );
    }

    #[test]
    fn test_return_parameter_membership_parsed_as_feature() {
        // Test that return_parameter_membership is parsed as a Feature
        let source = "function getValue { return : Integer; }";

        let parsed = KerMLParser::parse(Rule::function, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let classifier = parse_classifier(parsed).expect("Should convert to Classifier");

        // Find features in the body
        let features: Vec<_> = classifier
            .body
            .iter()
            .filter_map(|m| match m {
                ClassifierMember::Feature(f) => Some(f),
                _ => None,
            })
            .collect();

        assert!(
            !features.is_empty(),
            "Return parameter should be parsed as feature"
        );

        // Check that the return parameter has a typing relationship to Integer
        let has_integer_typing = features.iter().any(|f| {
            f.body.iter().any(|m| match m {
                FeatureMember::Typing(t) => t.typed == "Integer",
                _ => false,
            })
        });
        assert!(
            has_integer_typing,
            "Return parameter should have typing relationship for Integer"
        );
    }
}
