use crate::core::Span;
use crate::parser::kerml::Rule;
use crate::syntax::kerml::ast::enums::{ClassifierKind, FeatureDirection};
use crate::syntax::kerml::ast::parsers::ParseError;
use pest::iterators::Pair;

/// Convert pest Span to our Span type
pub fn to_span(pest_span: pest::Span) -> Span {
    let (sl, sc) = pest_span.start_pos().line_col();
    let (el, ec) = pest_span.end_pos().line_col();
    Span::from_coords(sl - 1, sc - 1, el - 1, ec - 1)
}

/// Recursively find name in nested identification rules
/// Skips feature_value to avoid extracting identifiers from default value expressions
/// Skips feature_specialization_part to avoid extracting qualified names from redefinitions
pub fn find_name<'a>(pairs: impl Iterator<Item = Pair<'a, Rule>>) -> Option<String> {
    for pair in pairs {
        // Skip feature_value to avoid extracting identifiers from expressions
        // Skip feature_specialization_part to avoid extracting qualified names from redefinitions
        if matches!(
            pair.as_rule(),
            Rule::feature_value | Rule::feature_specialization_part
        ) {
            continue;
        }

        if matches!(
            pair.as_rule(),
            Rule::identification | Rule::name | Rule::identifier
        ) {
            return Some(pair.as_str().to_string());
        }
        if let Some(name) = find_name(pair.into_inner()) {
            return Some(name);
        }
    }
    None
}

/// Recursively find identifier and return (name, span)
/// Skips feature_value to avoid extracting identifiers from default value expressions
/// Skips feature_specialization_part to avoid extracting qualified names from redefinitions
pub fn find_identifier_span<'a>(
    pairs: impl Iterator<Item = Pair<'a, Rule>>,
) -> (Option<String>, Option<Span>) {
    for pair in pairs {
        // Skip feature_value to avoid extracting identifiers from expressions
        // Skip feature_specialization_part to avoid extracting qualified names from redefinitions
        if matches!(
            pair.as_rule(),
            Rule::feature_value | Rule::feature_specialization_part
        ) {
            continue;
        }

        if matches!(
            pair.as_rule(),
            Rule::identification | Rule::name | Rule::identifier
        ) {
            return (
                Some(pair.as_str().to_string()),
                Some(to_span(pair.as_span())),
            );
        }
        if let (Some(name), Some(span)) = find_identifier_span(pair.into_inner()) {
            return (Some(name), Some(span));
        }
    }
    (None, None)
}

/// Check if a pair has a specific flag (with recursion into modifiers)
pub fn has_flag(pair: &Pair<Rule>, flag: Rule) -> bool {
    if pair.as_rule() == flag {
        return true;
    }
    // Recurse into modifier containers
    if pair.as_rule() == Rule::feature_modifier || pair.as_rule() == Rule::feature_prefix_modifiers
    {
        return pair.clone().into_inner().any(|p| has_flag(&p, flag));
    }
    // Also check direct children for the flag (e.g., readonly directly under feature)
    pair.clone()
        .into_inner()
        .any(|inner| inner.as_rule() == flag || has_flag(&inner, flag))
}

/// Extract readonly and derived flags from pairs
pub fn extract_flags(pairs: &[Pair<Rule>]) -> (bool, bool) {
    let readonly = pairs.iter().any(|p| has_flag(p, Rule::readonly));
    let derived = pairs.iter().any(|p| has_flag(p, Rule::derived));
    (readonly, derived)
}

/// Extract feature direction from pairs
pub fn extract_direction(pairs: &[Pair<Rule>]) -> Option<FeatureDirection> {
    pairs
        .iter()
        .find(|p| p.as_rule() == Rule::feature_direction_kind)
        .and_then(|p| match p.as_str() {
            "in" => Some(FeatureDirection::In),
            "out" => Some(FeatureDirection::Out),
            "inout" => Some(FeatureDirection::InOut),
            _ => None,
        })
}

/// Check if a rule represents a classifier
pub fn is_classifier_rule(r: Rule) -> bool {
    matches!(
        r,
        Rule::type_def
            | Rule::classifier
            | Rule::data_type
            | Rule::class
            | Rule::structure
            | Rule::behavior
            | Rule::function
            | Rule::association
            | Rule::association_structure
            | Rule::metaclass
    )
}

/// Map pest Rule to ClassifierKind
pub fn to_classifier_kind(rule: Rule) -> Result<ClassifierKind, ParseError> {
    Ok(match rule {
        Rule::type_def => ClassifierKind::Type,
        Rule::classifier => ClassifierKind::Classifier,
        Rule::data_type => ClassifierKind::DataType,
        Rule::class => ClassifierKind::Class,
        Rule::structure => ClassifierKind::Structure,
        Rule::behavior => ClassifierKind::Behavior,
        Rule::function => ClassifierKind::Function,
        Rule::association => ClassifierKind::Association,
        Rule::association_structure => ClassifierKind::AssociationStructure,
        Rule::metaclass => ClassifierKind::Metaclass,
        _ => return Err(ParseError::no_match()),
    })
}
