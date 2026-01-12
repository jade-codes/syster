//! Utility functions for SysML AST construction.
//!
//! This module provides helper functions used by the parsers for AST construction.

use crate::{core::Span, parser::sysml::Rule};
use pest::iterators::Pair;

use super::enums::{DefinitionKind, UsageKind};
use super::parsers::{ParseError, all_refs_with_spans_from, ref_with_span_from};

// ============================================================================
// Span conversion
// ============================================================================

/// Convert pest Span to our Span type
pub fn to_span(pest_span: pest::Span) -> Span {
    let (sl, sc) = pest_span.start_pos().line_col();
    let (el, ec) = pest_span.end_pos().line_col();
    Span::from_coords(sl - 1, sc - 1, el - 1, ec - 1)
}

/// Extract just the name from a reference (without span)
pub fn ref_from(pair: &Pair<Rule>) -> Option<String> {
    ref_with_span_from(pair).map(|(name, _span)| name)
}

// ============================================================================
// Rule predicates
// ============================================================================

/// Check if rule represents a body element that may contain members.
///
/// These are all the `*_body` rules in the grammar that can contain
/// nested usages, definitions, or other extractable members.
///
/// Note: Some body rules are intentionally excluded:
/// - `package_body`: Packages are handled separately with their own visitor
/// - `expression_body`: Expression internals, not AST members
/// - `relationship_body`: Connection relationship bodies, special handling
pub fn is_body_rule(r: Rule) -> bool {
    matches!(
        r,
        // Definition bodies
        Rule::definition_body
            | Rule::action_body
            | Rule::calculation_body
            | Rule::case_body
            | Rule::constraint_body
            | Rule::enumeration_body
            | Rule::interface_body
            | Rule::metadata_body
            | Rule::requirement_body
            | Rule::state_def_body
            | Rule::view_definition_body
            // Usage bodies
            | Rule::state_usage_body
            | Rule::usage_body
            | Rule::view_body
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
            | Rule::reference_usage
            | Rule::default_reference_usage
            | Rule::satisfy_requirement_usage
            | Rule::perform_action_usage
            | Rule::exhibit_state_usage
            | Rule::include_use_case_usage
            | Rule::objective_member
            | Rule::enumeration_usage
            | Rule::enumerated_value
            | Rule::state_usage
            | Rule::connection_usage
            | Rule::constraint_usage
            | Rule::assert_constraint_usage
            | Rule::calculation_usage
            | Rule::allocation_usage
            | Rule::metadata_usage
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
// Name extraction (for FromPest implementations)
// ============================================================================

/// Find first matching rule in children
pub fn find_in<'a>(pair: &Pair<'a, Rule>, rule: Rule) -> Option<Pair<'a, Rule>> {
    pair.clone().into_inner().find(|p| p.as_rule() == rule)
}

/// Extract the name from an identification rule.
/// identification = { (short_name ~ regular_name?) | regular_name }
///
/// Returns the regular_name if present, otherwise extracts the identifier from short_name.
/// For example:
/// - `<kg> kilogram` → returns ("kilogram", span of kilogram)
/// - `<kg>` → returns ("kg", span of kg identifier)
/// - `myName` → returns ("myName", span of myName)
pub fn extract_name_from_identification(
    pair: Pair<Rule>,
) -> (Option<String>, Option<crate::core::Span>) {
    let inner: Vec<_> = pair.into_inner().collect();

    // Look for regular_name first (preferred)
    for p in &inner {
        if p.as_rule() == Rule::regular_name
            && let Some(id) = p.clone().into_inner().next()
        {
            let name = if id.as_rule() == Rule::quoted_name {
                id.as_str()
                    .trim_start_matches('\'')
                    .trim_end_matches('\'')
                    .to_string()
            } else {
                id.as_str().to_string()
            };
            return (Some(name), Some(to_span(id.as_span())));
        }
    }

    // No regular_name found, look for short_name and extract identifier from within
    for p in &inner {
        if p.as_rule() == Rule::short_name {
            for inner_p in p.clone().into_inner() {
                if inner_p.as_rule() == Rule::identifier {
                    return (
                        Some(inner_p.as_str().to_string()),
                        Some(to_span(inner_p.as_span())),
                    );
                } else if inner_p.as_rule() == Rule::quoted_name {
                    let name = inner_p
                        .as_str()
                        .trim_start_matches('\'')
                        .trim_end_matches('\'')
                        .to_string();
                    return (Some(name), Some(to_span(inner_p.as_span())));
                }
            }
        }
    }

    // Fallback: if there's a direct identifier
    for p in &inner {
        if p.as_rule() == Rule::identifier {
            return (Some(p.as_str().to_string()), Some(to_span(p.as_span())));
        }
    }

    (None, None)
}

// ============================================================================
// Kind mapping
// ============================================================================

/// Map pest Rule to DefinitionKind
pub fn to_def_kind(rule: Rule) -> Result<DefinitionKind, ParseError> {
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
        _ => return Err(ParseError::no_match()),
    })
}

/// Map pest Rule to UsageKind
pub fn to_usage_kind(rule: Rule) -> Option<UsageKind> {
    Some(match rule {
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
        Rule::reference_usage
        | Rule::default_reference_usage
        | Rule::metadata_usage
        | Rule::directed_parameter_member
        | Rule::metadata_body_usage
        | Rule::metadata_body_usage_member => UsageKind::Reference,
        Rule::satisfy_requirement_usage => UsageKind::SatisfyRequirement,
        Rule::perform_action_usage => UsageKind::PerformAction,
        Rule::exhibit_state_usage => UsageKind::ExhibitState,
        Rule::include_use_case_usage => UsageKind::IncludeUseCase,
        Rule::state_usage => UsageKind::State,
        Rule::connection_usage => UsageKind::Connection,
        Rule::constraint_usage | Rule::assert_constraint_usage => UsageKind::Constraint,
        Rule::calculation_usage => UsageKind::Calculation,
        Rule::allocation_usage => UsageKind::Allocation,
        Rule::interface_usage => UsageKind::Interface,
        // Occurrence-based usages
        Rule::occurrence_usage => UsageKind::Occurrence,
        Rule::individual_usage => UsageKind::Individual,
        Rule::portion_usage => UsageKind::Snapshot,
        _ => return None,
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

/// Extract derived and constant flags from pairs
pub fn extract_flags(pairs: &[Pair<Rule>]) -> (bool, bool) {
    let derived = pairs.iter().any(|p| has_flag(p, Rule::derived_token));
    let constant = pairs.iter().any(|p| has_flag(p, Rule::constant_token));
    (derived, constant)
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
        .any(|p| has_definition_flag(p, Rule::abstract_token));
    let is_variation = pairs
        .iter()
        .any(|p| has_definition_flag(p, Rule::variation_token));
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
