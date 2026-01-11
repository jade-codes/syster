//! Utility functions for SysML AST construction.
//!
//! This module provides helper functions used by the parsers for AST construction.

use crate::{core::Span, parser::sysml::Rule};
use pest::iterators::Pair;

use super::enums::{DefinitionKind, UsageKind};
use super::parsers::ParseError;

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
            | Rule::constraint_body
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
            | Rule::reference_usage
            | Rule::default_reference_usage
            | Rule::constraint_usage
            | Rule::assert_constraint_usage
            | Rule::calculation_usage
            | Rule::state_usage
            | Rule::connection_usage
            | Rule::interface_usage
            | Rule::allocation_usage
            | Rule::flow_connection_usage
            | Rule::succession_flow_connection_usage
            | Rule::directed_parameter_member
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
        if p.as_rule() == Rule::regular_name {
            if let Some(id) = p.clone().into_inner().next() {
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
        Rule::satisfy_requirement_usage => UsageKind::SatisfyRequirement,
        Rule::perform_action_usage => UsageKind::PerformAction,
        Rule::exhibit_state_usage => UsageKind::ExhibitState,
        Rule::include_use_case_usage => UsageKind::IncludeUseCase,
        Rule::occurrence_usage => UsageKind::Occurrence,
        Rule::individual_usage => UsageKind::Individual,
        Rule::portion_usage => UsageKind::Snapshot,
        Rule::reference_usage | Rule::default_reference_usage | Rule::directed_parameter_member => {
            UsageKind::Reference
        }
        Rule::constraint_usage | Rule::assert_constraint_usage => UsageKind::Constraint,
        Rule::calculation_usage => UsageKind::Calculation,
        Rule::state_usage => UsageKind::State,
        Rule::connection_usage => UsageKind::Connection,
        Rule::interface_usage => UsageKind::Interface,
        Rule::allocation_usage => UsageKind::Allocation,
        Rule::flow_connection_usage | Rule::succession_flow_connection_usage => UsageKind::Flow,
        _ => return None,
    })
}
