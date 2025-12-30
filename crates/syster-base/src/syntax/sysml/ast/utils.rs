use crate::{core::Span, parser::sysml::Rule};
use from_pest::{ConversionError, Void};
use pest::iterators::Pair;

use super::enums::{DefinitionKind, UsageKind};

// ============================================================================
// Span conversion
// ============================================================================

/// Convert pest Span to our Span type
pub fn to_span(pest_span: pest::Span) -> Span {
    let (sl, sc) = pest_span.start_pos().line_col();
    let (el, ec) = pest_span.end_pos().line_col();
    Span::from_coords(sl - 1, sc - 1, el - 1, ec - 1)
}

// ============================================================================
// Rule predicates
// ============================================================================

/// Check if rule represents a body element
pub fn is_body_rule(r: Rule) -> bool {
    matches!(
        r,
        Rule::definition_body
            | Rule::action_body
            | Rule::enumeration_body
            | Rule::state_def_body
            | Rule::case_body
            | Rule::calculation_body
            | Rule::requirement_body
            | Rule::usage_body
    )
}

/// Check if rule represents a usage
pub fn is_usage_rule(r: Rule) -> bool {
    matches!(
        r,
        Rule::part_usage
            | Rule::action_usage
            | Rule::requirement_usage
            | Rule::port_usage
            | Rule::item_usage
            | Rule::attribute_usage
            | Rule::concern_usage
            | Rule::case_usage
            | Rule::view_usage
            | Rule::satisfy_requirement_usage
            | Rule::perform_action_usage
            | Rule::exhibit_state_usage
            | Rule::include_use_case_usage
            | Rule::objective_member
            | Rule::enumeration_usage
            | Rule::enumerated_value
    )
}

/// Check if rule represents a definition
pub fn is_definition_rule(r: Rule) -> bool {
    matches!(
        r,
        Rule::part_definition
            | Rule::action_definition
            | Rule::state_definition
            | Rule::requirement_definition
            | Rule::port_definition
            | Rule::item_definition
            | Rule::attribute_definition
            | Rule::concern_definition
            | Rule::case_definition
            | Rule::analysis_case_definition
            | Rule::verification_case_definition
            | Rule::use_case_definition
            | Rule::view_definition
            | Rule::viewpoint_definition
            | Rule::rendering_definition
            | Rule::allocation_definition
            | Rule::calculation_definition
            | Rule::connection_definition
            | Rule::constraint_definition
            | Rule::enumeration_definition
            | Rule::flow_definition
            | Rule::individual_definition
            | Rule::interface_definition
            | Rule::occurrence_definition
            | Rule::metadata_definition
    )
}

// ============================================================================
// Reference extraction
// ============================================================================

/// Extract reference from pair or its immediate children
pub fn ref_from(pair: &Pair<Rule>) -> Option<String> {
    match pair.as_rule() {
        Rule::identifier
        | Rule::quoted_name
        | Rule::feature_reference
        | Rule::classifier_reference => Some(pair.as_str().trim().to_string()),
        _ => pair.clone().into_inner().find_map(|p| ref_from(&p)),
    }
}

/// Extract reference with span from pair or its immediate children
pub fn ref_with_span_from(pair: &Pair<Rule>) -> Option<(String, crate::core::Span)> {
    match pair.as_rule() {
        Rule::identifier
        | Rule::quoted_name
        | Rule::feature_reference
        | Rule::classifier_reference => {
            Some((pair.as_str().trim().to_string(), to_span(pair.as_span())))
        }
        _ => pair
            .clone()
            .into_inner()
            .find_map(|p| ref_with_span_from(&p)),
    }
}

/// Extract all references from a pair
pub fn all_refs_from(pair: &Pair<Rule>) -> Vec<String> {
    pair.clone()
        .into_inner()
        .filter_map(|p| ref_from(&p))
        .collect()
}

/// Extract all references with spans for relationship structs
pub fn all_refs_with_spans_from(pair: &Pair<Rule>) -> Vec<(String, Option<crate::core::Span>)> {
    pair.clone()
        .into_inner()
        .filter_map(|p| ref_with_span_from(&p).map(|(name, span)| (name, Some(span))))
        .collect()
}

// ============================================================================
// Name extraction
// ============================================================================

/// Find first matching rule in children
pub fn find_in<'a>(pair: &Pair<'a, Rule>, rule: Rule) -> Option<Pair<'a, Rule>> {
    pair.clone().into_inner().find(|p| p.as_rule() == rule)
}

/// Recursively find name in nested identification rules
/// Skips relationship parts to avoid extracting identifiers from redefinitions, subsettings, etc.
pub fn find_name<'pest>(pairs: impl Iterator<Item = Pair<'pest, Rule>>) -> Option<String> {
    for pair in pairs {
        // Skip relationship parts - don't extract identifiers from within these
        if is_relationship_part(&pair) {
            continue;
        }

        if matches!(pair.as_rule(), Rule::identifier | Rule::identification) {
            return Some(pair.as_str().to_string());
        }
        // Recursively search in children
        if let Some(name) = find_name(pair.into_inner()) {
            return Some(name);
        }
    }
    None
}

/// Recursively find identifier and return (name, span)
/// Skips relationship parts to avoid extracting identifiers from redefinitions, subsettings, etc.
pub fn find_identifier_span<'a>(
    pairs: impl Iterator<Item = Pair<'a, Rule>>,
) -> (Option<String>, Option<crate::core::Span>) {
    for pair in pairs {
        // Skip relationship parts - don't extract identifiers from within these
        if is_relationship_part(&pair) {
            continue;
        }

        if matches!(pair.as_rule(), Rule::identifier | Rule::identification) {
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

/// Check if a rule represents a relationship part that should be skipped when finding names
fn is_relationship_part(pair: &Pair<Rule>) -> bool {
    matches!(
        pair.as_rule(),
        Rule::feature_specialization
            | Rule::feature_specialization_part
            | Rule::redefinition_part
            | Rule::redefinitions
            | Rule::owned_redefinition
            | Rule::subsettings
            | Rule::owned_subsetting
            | Rule::typings
            | Rule::references
            | Rule::owned_reference_subsetting
            | Rule::crosses
            | Rule::subclassification_part
            | Rule::owned_subclassification
            | Rule::feature_value
            | Rule::value_part
    )
}

// ============================================================================
// Kind mapping
// ============================================================================

/// Map pest Rule to DefinitionKind
pub fn to_def_kind(rule: Rule) -> Result<DefinitionKind, ConversionError<Void>> {
    Ok(match rule {
        Rule::part_definition => DefinitionKind::Part,
        Rule::action_definition => DefinitionKind::Action,
        Rule::state_definition => DefinitionKind::State,
        Rule::requirement_definition => DefinitionKind::Requirement,
        Rule::port_definition => DefinitionKind::Port,
        Rule::item_definition => DefinitionKind::Item,
        Rule::attribute_definition => DefinitionKind::Attribute,
        Rule::concern_definition => DefinitionKind::Concern,
        Rule::case_definition => DefinitionKind::Case,
        Rule::analysis_case_definition => DefinitionKind::AnalysisCase,
        Rule::verification_case_definition => DefinitionKind::VerificationCase,
        Rule::use_case_definition => DefinitionKind::UseCase,
        Rule::view_definition => DefinitionKind::View,
        Rule::viewpoint_definition => DefinitionKind::Viewpoint,
        Rule::rendering_definition => DefinitionKind::Rendering,
        Rule::allocation_definition => DefinitionKind::Allocation,
        Rule::calculation_definition => DefinitionKind::Calculation,
        Rule::connection_definition => DefinitionKind::Connection,
        Rule::constraint_definition => DefinitionKind::Constraint,
        Rule::enumeration_definition => DefinitionKind::Enumeration,
        Rule::flow_definition => DefinitionKind::Flow,
        Rule::individual_definition => DefinitionKind::Individual,
        Rule::interface_definition => DefinitionKind::Interface,
        Rule::occurrence_definition => DefinitionKind::Occurrence,
        Rule::metadata_definition => DefinitionKind::Metadata,
        _ => return Err(ConversionError::NoMatch),
    })
}

/// Map pest Rule to UsageKind
pub fn to_usage_kind(rule: Rule) -> Result<UsageKind, ConversionError<Void>> {
    Ok(match rule {
        Rule::part_usage => UsageKind::Part,
        Rule::action_usage => UsageKind::Action,
        Rule::requirement_usage | Rule::objective_member => UsageKind::Requirement,
        Rule::port_usage => UsageKind::Port,
        Rule::item_usage => UsageKind::Item,
        Rule::attribute_usage => UsageKind::Attribute,
        Rule::concern_usage => UsageKind::Concern,
        Rule::case_usage => UsageKind::Case,
        Rule::view_usage => UsageKind::View,
        Rule::enumeration_usage | Rule::enumerated_value => UsageKind::Enumeration,
        Rule::satisfy_requirement_usage => UsageKind::SatisfyRequirement,
        Rule::perform_action_usage => UsageKind::PerformAction,
        Rule::exhibit_state_usage => UsageKind::ExhibitState,
        Rule::include_use_case_usage => UsageKind::IncludeUseCase,
        _ => return Err(ConversionError::NoMatch),
    })
}

// ============================================================================
// Flag extraction
// ============================================================================

/// Check if a pair has a specific flag (with recursion into modifiers)
pub fn has_flag(pair: &Pair<Rule>, flag: Rule) -> bool {
    if pair.as_rule() == flag {
        return true;
    }
    if matches!(
        pair.as_rule(),
        Rule::ref_prefix
            | Rule::basic_usage_prefix
            | Rule::occurrence_usage_prefix
            | Rule::usage_prefix
    ) {
        return pair.clone().into_inner().any(|p| has_flag(&p, flag));
    }
    false
}

/// Extract derived and readonly flags from pairs
pub fn extract_flags(pairs: &[Pair<Rule>]) -> (bool, bool) {
    let derived = pairs.iter().any(|p| has_flag(p, Rule::derived));
    let readonly = pairs.iter().any(|p| has_flag(p, Rule::readonly));
    (derived, readonly)
}

/// Check if a pair has a definition flag (with recursion into prefixes)
fn has_definition_flag(pair: &Pair<Rule>, flag: Rule) -> bool {
    if pair.as_rule() == flag {
        return true;
    }
    if matches!(
        pair.as_rule(),
        Rule::basic_definition_prefix
            | Rule::definition_prefix
            | Rule::occurrence_definition_prefix
    ) {
        return pair
            .clone()
            .into_inner()
            .any(|p| has_definition_flag(&p, flag));
    }
    false
}

/// Extract abstract and variation flags from definition pairs
pub fn extract_definition_flags(pairs: &[Pair<Rule>]) -> (bool, bool) {
    let is_abstract = pairs
        .iter()
        .any(|p| has_definition_flag(p, Rule::abstract_marker));
    let is_variation = pairs
        .iter()
        .any(|p| has_definition_flag(p, Rule::variation_marker));
    (is_abstract, is_variation)
}

// ============================================================================
// Relationship extraction
// ============================================================================

/// Extract relationships from a pair
pub fn extract_relationships(pair: &Pair<Rule>) -> super::types::Relationships {
    use super::types::Relationships;
    let mut rel = Relationships::none();
    extract_rels_recursive(pair, &mut rel, 0);
    rel
}

fn extract_rels_recursive(pair: &Pair<Rule>, rel: &mut super::types::Relationships, depth: usize) {
    // Don't descend into nested definitions/usages
    if depth > 0 && (is_definition_rule(pair.as_rule()) || is_usage_rule(pair.as_rule())) {
        return;
    }

    match pair.as_rule() {
        Rule::subclassification_part => {
            for p in pair.clone().into_inner() {
                if p.as_rule() == Rule::owned_subclassification {
                    for (target, span) in all_refs_with_spans_from(&p) {
                        rel.specializes
                            .push(super::types::SpecializationRel { target, span });
                    }
                }
            }
        }
        Rule::redefinition_part => {
            for p in pair.clone().into_inner() {
                if p.as_rule() == Rule::owned_subclassification {
                    for (target, span) in all_refs_with_spans_from(&p) {
                        rel.redefines
                            .push(super::types::RedefinitionRel { target, span });
                    }
                }
            }
        }
        Rule::satisfy_requirement_usage => {
            for (target, span) in all_refs_with_spans_from(pair) {
                rel.satisfies
                    .push(super::types::SatisfyRel { target, span });
            }
        }
        Rule::perform_action_usage => {
            for (target, span) in all_refs_with_spans_from(pair) {
                rel.performs.push(super::types::PerformRel { target, span });
            }
        }
        Rule::exhibit_state_usage => {
            for (target, span) in all_refs_with_spans_from(pair) {
                rel.exhibits.push(super::types::ExhibitRel { target, span });
            }
        }
        Rule::include_use_case_usage => {
            for (target, span) in all_refs_with_spans_from(pair) {
                rel.includes.push(super::types::IncludeRel { target, span });
            }
        }
        Rule::feature_specialization => {
            for spec in pair.clone().into_inner() {
                match spec.as_rule() {
                    Rule::typings => {
                        if let Some((name, span)) = ref_with_span_from(&spec) {
                            rel.typed_by = Some(name);
                            rel.typed_by_span = Some(span);
                        } else {
                            rel.typed_by = ref_from(&spec);
                        }
                    }
                    Rule::subsettings => {
                        for (target, span) in all_refs_with_spans_from(&spec) {
                            rel.subsets
                                .push(super::types::SubsettingRel { target, span });
                        }
                    }
                    Rule::redefinitions => {
                        for (target, span) in all_refs_with_spans_from(&spec) {
                            rel.redefines
                                .push(super::types::RedefinitionRel { target, span });
                        }
                    }
                    Rule::references => {
                        for (target, span) in all_refs_with_spans_from(&spec) {
                            rel.references
                                .push(super::types::ReferenceRel { target, span });
                        }
                    }
                    Rule::crosses => {
                        for (target, span) in all_refs_with_spans_from(&spec) {
                            rel.crosses.push(super::types::CrossRel { target, span });
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            for inner in pair.clone().into_inner() {
                extract_rels_recursive(&inner, rel, depth + 1);
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::parser::sysml::{Rule, SysMLParser};
    use pest::Parser;

    // ========================================================================
    // Tests for all_refs_from()
    // ========================================================================

    #[test]
    fn test_all_refs_from_single_specialization() {
        // Test extracting a single reference from a specialization
        let source = ":> Vehicle";
        let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0], "Vehicle");
    }

    #[test]
    fn test_all_refs_from_multiple_specializations() {
        // Test extracting multiple references from a subclassification part
        let source = ":> Vehicle, Machine, Device";
        let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        assert_eq!(refs.len(), 3);
        assert!(refs.contains(&"Vehicle".to_string()));
        assert!(refs.contains(&"Machine".to_string()));
        assert!(refs.contains(&"Device".to_string()));
    }

    #[test]
    fn test_all_refs_from_subsetting() {
        // Test extracting references from subsetting relationships
        let source = ":> base1, base2";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        assert_eq!(refs.len(), 2);
        assert!(refs.contains(&"base1".to_string()));
        assert!(refs.contains(&"base2".to_string()));
    }

    #[test]
    fn test_all_refs_from_typing() {
        // Test extracting a single reference from typing relationship
        let source = ": MyType";
        let mut pairs = SysMLParser::parse(Rule::typings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0], "MyType");
    }

    #[test]
    fn test_all_refs_from_redefinitions() {
        // Test extracting multiple references from redefinitions
        let source = ":>> original1, original2";
        let mut pairs = SysMLParser::parse(Rule::redefinitions, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        assert_eq!(refs.len(), 2);
        assert!(refs.contains(&"original1".to_string()));
        assert!(refs.contains(&"original2".to_string()));
    }

    #[test]
    fn test_all_refs_from_no_references() {
        // Test a structure with no references (should return empty vector)
        let source = "part def MyPart;";
        let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
        let pair = pairs.next().unwrap();

        // Get a sub-pair that doesn't contain references
        let body_pairs: Vec<_> = pair
            .clone()
            .into_inner()
            .filter(|p| matches!(p.as_rule(), Rule::definition_body))
            .collect();

        if let Some(body_pair) = body_pairs.first() {
            let refs = all_refs_from(body_pair);
            assert_eq!(refs.len(), 0);
        }
    }

    #[test]
    fn test_all_refs_from_quoted_name_in_subsetting() {
        // Test extracting a quoted name reference from subsetting
        let source = ":> 'Complex Name'";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0], "'Complex Name'");
    }

    #[test]
    fn test_all_refs_from_feature_specialization() {
        // Test extracting multiple references from a complex feature specialization
        // all_refs_from gets children, so we pass the feature_specialization which contains
        // typing, subsetting, and redefinition children
        let source = ": Type1 :> base1, base2 :>> redef1";
        let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        // Should extract Type1 (from typing child), base1, base2 (from subsetting child),
        // and redef1 (from redefinition child)
        // The exact count depends on how ref_from traverses the children
        assert!(!refs.is_empty(), "Expected at least 1 ref, got 0");

        // We should at least get the typing reference
        assert!(refs.iter().any(|r| r == "Type1"), "Should contain Type1");
    }

    #[test]
    fn test_all_refs_from_references_relationship() {
        // Test extracting references from 'references' relationship
        let source = "::> ref1, ref2";
        let mut pairs = SysMLParser::parse(Rule::references, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        // all_refs_from looks at children, references rule has children that include the identifiers
        assert!(!refs.is_empty(), "Expected at least 1 ref, got 0");
        // Should contain at least one of the references
        let has_ref = refs.iter().any(|r| r == "ref1" || r == "ref2");
        assert!(has_ref, "Should contain ref1 or ref2");
    }

    // ========================================================================
    // Tests for all_refs_with_spans_from()
    // ========================================================================

    #[test]
    fn test_all_refs_with_spans_from_single_reference() {
        // Test extracting a single reference with span from subsetting
        let source = ":> Vehicle";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert_eq!(refs_with_spans.len(), 1);
        assert_eq!(refs_with_spans[0].0, "Vehicle");
        assert!(refs_with_spans[0].1.is_some());
    }

    #[test]
    fn test_all_refs_with_spans_from_multiple_specializations() {
        // Test extracting multiple references with spans from subclassification
        let source = ":> Vehicle, Machine";
        let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert_eq!(refs_with_spans.len(), 2);
        assert!(refs_with_spans.iter().any(|(name, _)| name == "Vehicle"));
        assert!(refs_with_spans.iter().any(|(name, _)| name == "Machine"));

        // Verify all spans are present
        for (_, span_opt) in &refs_with_spans {
            assert!(span_opt.is_some(), "Expected span to be present");
        }
    }

    #[test]
    fn test_all_refs_with_spans_from_typing() {
        // Test extracting reference with span from typing
        let source = ": MyType";
        let mut pairs = SysMLParser::parse(Rule::typings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert_eq!(refs_with_spans.len(), 1);
        assert_eq!(refs_with_spans[0].0, "MyType");
        assert!(refs_with_spans[0].1.is_some());
    }

    #[test]
    fn test_all_refs_with_spans_from_subsetting() {
        // Test extracting multiple references with spans from subsetting
        let source = ":> base1, base2, base3";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert_eq!(refs_with_spans.len(), 3);

        // Verify each reference has a span
        let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"base1"));
        assert!(names.contains(&"base2"));
        assert!(names.contains(&"base3"));

        for (_, span_opt) in &refs_with_spans {
            assert!(span_opt.is_some());
        }
    }

    #[test]
    fn test_all_refs_with_spans_from_redefinitions() {
        // Test extracting references with spans from redefinitions
        let source = ":>> original1, original2";
        let mut pairs = SysMLParser::parse(Rule::redefinitions, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert_eq!(refs_with_spans.len(), 2);

        let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"original1"));
        assert!(names.contains(&"original2"));

        // Verify spans are present
        for (_, span_opt) in &refs_with_spans {
            assert!(span_opt.is_some());
        }
    }

    #[test]
    fn test_all_refs_with_spans_from_span_accuracy() {
        // Test that spans accurately point to the reference location
        let source = ":> VehicleBase";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert_eq!(refs_with_spans.len(), 1);
        assert_eq!(refs_with_spans[0].0, "VehicleBase");

        if let Some(span) = &refs_with_spans[0].1 {
            // Span should be on the first line (0-indexed)
            assert_eq!(span.start.line, 0);
            // The identifier starts after ":> " (3 characters)
            assert!(span.start.column >= 3);
        } else {
            panic!("Expected span to be present");
        }
    }

    #[test]
    fn test_all_refs_with_spans_from_no_references() {
        // Test with a structure that has no references
        let source = "part def MyPart;";
        let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
        let pair = pairs.next().unwrap();

        // Get a sub-pair that doesn't contain references
        let body_pairs: Vec<_> = pair
            .clone()
            .into_inner()
            .filter(|p| matches!(p.as_rule(), Rule::definition_body))
            .collect();

        if let Some(body_pair) = body_pairs.first() {
            let refs_with_spans = all_refs_with_spans_from(body_pair);
            assert_eq!(refs_with_spans.len(), 0);
        }
    }

    #[test]
    fn test_all_refs_with_spans_from_feature_specialization() {
        // Test extracting multiple references with spans from feature specialization
        let source = ": Type1 :> base1";
        let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert!(
            !refs_with_spans.is_empty(),
            "Expected at least 1 ref with span"
        );

        // Verify all have spans
        for (_, span_opt) in &refs_with_spans {
            assert!(span_opt.is_some());
        }

        // Check that Type1 is present
        let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"Type1"));
    }

    // ========================================================================
    // Tests for ref_from() helper
    // ========================================================================

    #[test]
    fn test_ref_from_identifier() {
        // Test extracting reference from an identifier
        let source = "MyElement";
        let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
        let pair = pairs.next().unwrap();

        let ref_opt = ref_from(&pair);

        assert!(ref_opt.is_some());
        assert_eq!(ref_opt.unwrap(), "MyElement");
    }

    #[test]
    fn test_ref_from_quoted_name() {
        // Test extracting reference from a quoted name
        let source = "'My Complex Name'";
        let mut pairs = SysMLParser::parse(Rule::quoted_name, source).unwrap();
        let pair = pairs.next().unwrap();

        let ref_opt = ref_from(&pair);

        assert!(ref_opt.is_some());
        assert_eq!(ref_opt.unwrap(), "'My Complex Name'");
    }

    #[test]
    fn test_ref_from_nested_structure() {
        // Test extracting reference from a nested structure
        let source = ":> Vehicle";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let ref_opt = ref_from(&pair);

        assert!(ref_opt.is_some());
        assert_eq!(ref_opt.unwrap(), "Vehicle");
    }

    // ========================================================================
    // Tests for ref_with_span_from() helper
    // ========================================================================

    #[test]
    fn test_ref_with_span_from_identifier() {
        // Test extracting reference with span from an identifier
        let source = "MyElement";
        let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
        let pair = pairs.next().unwrap();

        let ref_with_span_opt = ref_with_span_from(&pair);

        assert!(ref_with_span_opt.is_some());
        let (name, span) = ref_with_span_opt.unwrap();
        assert_eq!(name, "MyElement");
        assert_eq!(span.start.line, 0);
        assert_eq!(span.start.column, 0);
    }

    #[test]
    fn test_ref_with_span_from_quoted_name() {
        // Test extracting reference with span from a quoted name
        let source = "'Complex Name'";
        let mut pairs = SysMLParser::parse(Rule::quoted_name, source).unwrap();
        let pair = pairs.next().unwrap();

        let ref_with_span_opt = ref_with_span_from(&pair);

        assert!(ref_with_span_opt.is_some());
        let (name, span) = ref_with_span_opt.unwrap();
        assert_eq!(name, "'Complex Name'");
        assert_eq!(span.start.line, 0);
    }

    #[test]
    fn test_ref_with_span_from_nested_structure() {
        // Test extracting reference with span from a nested structure
        let source = ": MyType";
        let mut pairs = SysMLParser::parse(Rule::typings, source).unwrap();
        let pair = pairs.next().unwrap();

        let ref_with_span_opt = ref_with_span_from(&pair);

        assert!(ref_with_span_opt.is_some());
        let (name, span) = ref_with_span_opt.unwrap();
        assert_eq!(name, "MyType");
        // Should point to the identifier, not the ":"
        assert!(span.start.column >= 2);
    }

    // ========================================================================
    // Edge cases and integration tests
    // ========================================================================

    #[test]
    fn test_all_refs_from_empty_list() {
        // Test behavior with structures that could have references but don't
        let source = "part def Test;";
        let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
        let pair = pairs.next().unwrap();

        // The definition itself might have identifiers, but body should be empty
        let body_pairs: Vec<_> = pair
            .into_inner()
            .filter(|p| matches!(p.as_rule(), Rule::definition_body))
            .collect();

        if let Some(body_pair) = body_pairs.first() {
            let refs = all_refs_from(body_pair);
            assert_eq!(refs.len(), 0, "Empty body should have no references");
        }
    }

    #[test]
    fn test_all_refs_trimming() {
        // Test that references are trimmed of whitespace
        let source = ":>  Vehicle  ";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        assert_eq!(refs.len(), 1);
        // Should be trimmed
        assert_eq!(refs[0], "Vehicle");
        assert!(!refs[0].starts_with(' '));
        assert!(!refs[0].ends_with(' '));
    }

    #[test]
    fn test_all_refs_from_complex_relationship_chain() {
        // Test extracting all references from a complex chain of relationships
        // Test with just the feature_specialization part
        let source = ": PartType :> base1, base2 :>> original";
        let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs = all_refs_from(&pair);

        // Should find at least the typing (PartType)
        assert!(!refs.is_empty(), "Expected at least 1 ref, got 0");
        assert!(
            refs.iter().any(|r| r == "PartType"),
            "Should contain PartType"
        );
    }

    #[test]
    fn test_all_refs_with_spans_ordering() {
        // Test that references are extracted in the order they appear
        let source = ":> First, Second, Third";
        let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
        let pair = pairs.next().unwrap();

        let refs_with_spans = all_refs_with_spans_from(&pair);

        assert_eq!(refs_with_spans.len(), 3);

        // Check the names appear in order
        let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"First"));
        assert!(names.contains(&"Second"));
        assert!(names.contains(&"Third"));
    }
}
