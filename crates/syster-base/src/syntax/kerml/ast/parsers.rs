use super::enums::{ClassifierMember, FeatureMember, ImportKind};
use super::types::{
    Classifier, Feature, Import, Redefinition, Specialization, Subsetting, TypingRelationship,
};
use super::utils::{
    extract_direction, extract_flags, find_identifier_span, find_name, to_classifier_kind, to_span,
};
use crate::parser::kerml::Rule;
use from_pest::{ConversionError, Void};
use pest::iterators::Pair;

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
        Rule::import => {
            if let Some(path) = extract_import_path(pair) {
                let is_recursive = detect_is_recursive(pair);
                let kind = detect_import_kind(pair);
                members.push(ClassifierMember::Import(Import {
                    path,
                    is_recursive,
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
        | Rule::relationship => {
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
pub fn parse_classifier(pair: Pair<Rule>) -> Result<Classifier, ConversionError<Void>> {
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
    let (is_readonly, is_derived) = extract_flags(&pairs);

    // Find the identifier and its span
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    // Parse body by searching through all children
    let body = parse_feature_body(&pair);

    Feature {
        name,
        direction: extract_direction(&pairs),
        is_readonly,
        is_derived,
        body,
        span,
    }
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
    fn test_readonly_feature() {
        let source = "readonly feature constant : Real;";

        let parsed = KerMLParser::parse(Rule::feature, source)
            .expect("Should parse")
            .next()
            .expect("Should have pair");

        let feature = parse_feature(parsed);

        assert_eq!(feature.name, Some("constant".to_string()));
        assert!(feature.is_readonly, "Feature should be marked as readonly");
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
}
