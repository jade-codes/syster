use super::utils::{
    extract_definition_flags, extract_flags, extract_relationships, find_identifier_span,
    find_name, is_body_rule, is_usage_rule, to_def_kind, to_span, to_usage_kind,
};
use super::{
    enums::{DefinitionMember, UsageMember},
    types::{Comment, Definition, Usage},
};
use crate::parser::sysml::Rule;
use from_pest::{ConversionError, Void};
use pest::iterators::Pair;

// ============================================================================
// Body parsing
// ============================================================================

/// Parse definition body members
pub fn parse_def_body(pair: &Pair<Rule>) -> Vec<DefinitionMember> {
    let mut members = Vec::new();
    extract_def_members(pair, &mut members);
    members
}

fn extract_def_members(pair: &Pair<Rule>, members: &mut Vec<DefinitionMember>) {
    if is_usage_rule(pair.as_rule()) {
        members.push(DefinitionMember::Usage(Box::new(parse_usage(pair.clone()))));
    } else {
        for inner in pair.clone().into_inner() {
            extract_def_members(&inner, members);
        }
    }
}

/// Parse usage body members
pub fn parse_usage_body(pair: &Pair<Rule>) -> Vec<UsageMember> {
    let mut members = Vec::new();
    extract_usage_members(pair, &mut members);
    members
}

fn extract_usage_members(pair: &Pair<Rule>, members: &mut Vec<UsageMember>) {
    match pair.as_rule() {
        Rule::documentation | Rule::block_comment => {
            members.push(UsageMember::Comment(Comment {
                content: pair.as_str().to_string(),
                span: Some(to_span(pair.as_span())),
            }));
        }
        _ if is_usage_rule(pair.as_rule()) => {
            // Parse nested usage
            members.push(UsageMember::Usage(Box::new(parse_usage(pair.clone()))));
        }
        _ => {
            for inner in pair.clone().into_inner() {
                extract_usage_members(&inner, members);
            }
        }
    }
}

// ============================================================================
// Main parsers
// ============================================================================

/// Parse a definition from a pest pair
pub fn parse_definition(pair: Pair<Rule>) -> Result<Definition, ConversionError<Void>> {
    let kind = to_def_kind(pair.as_rule())?;

    // Recursively find the body rule (it may be nested in definition_suffix)
    let body = find_body_in_tree(&pair)
        .map(|p| parse_def_body(&p))
        .unwrap_or_default();

    // Find the identifier and its span
    let pairs: Vec<_> = pair.clone().into_inner().collect();
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    // Extract definition flags (abstract, variation)
    let (is_abstract, is_variation) = extract_definition_flags(&pairs);

    Ok(Definition {
        kind,
        name,
        relationships: extract_relationships(&pair),
        body,
        span,
        is_abstract,
        is_variation,
    })
}

fn find_body_in_tree<'i>(pair: &Pair<'i, Rule>) -> Option<Pair<'i, Rule>> {
    if is_body_rule(pair.as_rule()) {
        return Some(pair.clone());
    }
    for inner in pair.clone().into_inner() {
        if let Some(found) = find_body_in_tree(&inner) {
            return Some(found);
        }
    }
    None
}

/// Parse a usage from a pest pair
pub fn parse_usage(pair: Pair<Rule>) -> Usage {
    let kind = to_usage_kind(pair.as_rule()).unwrap();
    let pairs: Vec<_> = pair.clone().into_inner().collect();

    let body = pairs
        .iter()
        .find_map(|p| find_body_in_tree(p))
        .map(|p| parse_usage_body(&p))
        .unwrap_or_default();

    let (is_derived, is_readonly) = extract_flags(&pairs);

    // Find the identifier and its span
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    Usage {
        kind,
        name,
        relationships: extract_relationships(&pair),
        body,
        span,
        is_derived,
        is_readonly,
    }
}
