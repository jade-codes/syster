use super::{
    enums::{DefinitionKind, DefinitionMember, Element, UsageKind},
    types::{
        Comment, Definition, Import, NamespaceDeclaration, Package, Relationships, SysMLFile, Usage,
    },
};
use crate::{core::Span, language::sysml::syntax::Alias, parser::sysml::Rule};
use from_pest::{ConversionError, FromPest, Void};

/// Convert a Pest span to our Span type (0-indexed for LSP compatibility)
fn pest_span_to_span(pest_span: pest::Span) -> Span {
    let (start_line, start_col) = pest_span.start_pos().line_col();
    let (end_line, end_col) = pest_span.end_pos().line_col();

    // Pest is 1-indexed, convert to 0-indexed for LSP
    Span::from_coords(start_line - 1, start_col - 1, end_line - 1, end_col - 1)
}

// Helper function to recursively extract usages from body items
fn extract_usages_from_body<'a>(
    pair: &pest::iterators::Pair<'a, Rule>,
    body: &mut Vec<DefinitionMember>,
) {
    match pair.as_rule() {
        // Domain-specific usages we want to capture
        Rule::exhibit_state_usage
        | Rule::perform_action_usage
        | Rule::satisfy_requirement_usage
        | Rule::include_use_case_usage
        | Rule::part_usage
        | Rule::action_usage
        | Rule::requirement_usage
        | Rule::port_usage
        | Rule::item_usage
        | Rule::attribute_usage
        | Rule::concern_usage
        | Rule::case_usage
        | Rule::view_usage => {
            // Manually build the Usage instead of using from_pest
            let kind = match rule_to_usage_kind(pair.as_rule()) {
                Ok(k) => k,
                Err(_) => return,
            };
            let name = find_name(pair.clone().into_inner());
            let relationships = extract_relationships(pair);
            let (is_derived, is_readonly) = extract_property_flags(pair);
            let span = Some(pest_span_to_span(pair.as_span()));
            body.push(DefinitionMember::Usage(Box::new(Usage {
                kind,
                name,
                relationships,
                body: vec![],
                span,
                is_derived,
                is_readonly,
            })));
        }
        // Recursively search children
        _ => {
            for inner in pair.clone().into_inner() {
                extract_usages_from_body(&inner, body);
            }
        }
    }
}

// Helper to extract property flags (derived, readonly)
fn extract_property_flags(pair: &pest::iterators::Pair<Rule>) -> (bool, bool) {
    let mut is_derived = false;
    let mut is_readonly = false;

    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::derived => is_derived = true,
            Rule::readonly => is_readonly = true,
            Rule::ref_prefix
            | Rule::basic_usage_prefix
            | Rule::occurrence_usage_prefix
            | Rule::usage_prefix => {
                let (d, r) = extract_property_flags(&inner);
                is_derived = is_derived || d;
                is_readonly = is_readonly || r;
            }
            _ => {}
        }
    }

    (is_derived, is_readonly)
}

fn find_reference(pair: &pest::iterators::Pair<Rule>) -> Option<String> {
    match pair.as_rule() {
        Rule::identifier
        | Rule::quoted_name
        | Rule::feature_reference
        | Rule::classifier_reference => {
            return Some(pair.as_str().trim().to_string());
        }
        _ => {
            for inner in pair.clone().into_inner() {
                if let Some(name) = find_reference(&inner) {
                    return Some(name);
                }
            }
        }
    }
    None
}

// Helper to map a rule to a DefinitionKind
fn rule_to_definition_kind(rule: Rule) -> Result<DefinitionKind, ConversionError<Void>> {
    match rule {
        Rule::part_definition => Ok(DefinitionKind::Part),
        Rule::action_definition => Ok(DefinitionKind::Action),
        Rule::state_definition => Ok(DefinitionKind::State),
        Rule::requirement_definition => Ok(DefinitionKind::Requirement),
        Rule::port_definition => Ok(DefinitionKind::Port),
        Rule::item_definition => Ok(DefinitionKind::Item),
        Rule::attribute_definition => Ok(DefinitionKind::Attribute),
        Rule::concern_definition => Ok(DefinitionKind::Concern),
        Rule::case_definition => Ok(DefinitionKind::Case),
        Rule::analysis_case_definition => Ok(DefinitionKind::AnalysisCase),
        Rule::verification_case_definition => Ok(DefinitionKind::VerificationCase),
        Rule::use_case_definition => Ok(DefinitionKind::UseCase),
        Rule::view_definition => Ok(DefinitionKind::View),
        Rule::viewpoint_definition => Ok(DefinitionKind::Viewpoint),
        Rule::rendering_definition => Ok(DefinitionKind::Rendering),
        _ => Err(ConversionError::NoMatch),
    }
}

// Helper to map a rule to a UsageKind
fn rule_to_usage_kind(rule: Rule) -> Result<UsageKind, ConversionError<Void>> {
    match rule {
        Rule::part_usage => Ok(UsageKind::Part),
        Rule::action_usage => Ok(UsageKind::Action),
        Rule::requirement_usage => Ok(UsageKind::Requirement),
        Rule::port_usage => Ok(UsageKind::Port),
        Rule::item_usage => Ok(UsageKind::Item),
        Rule::attribute_usage => Ok(UsageKind::Attribute),
        Rule::concern_usage => Ok(UsageKind::Concern),
        Rule::case_usage => Ok(UsageKind::Case),
        Rule::view_usage => Ok(UsageKind::View),
        Rule::satisfy_requirement_usage => Ok(UsageKind::SatisfyRequirement),
        Rule::perform_action_usage => Ok(UsageKind::PerformAction),
        Rule::exhibit_state_usage => Ok(UsageKind::ExhibitState),
        Rule::include_use_case_usage => Ok(UsageKind::IncludeUseCase),
        _ => Err(ConversionError::NoMatch),
    }
}

// Helper to recursively find name in parse tree
fn find_name(pairs: pest::iterators::Pairs<Rule>) -> Option<String> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::identifier | Rule::identification => {
                return Some(pair.as_str().to_string());
            }
            _ => {
                if let Some(name) = find_name(pair.into_inner()) {
                    return Some(name);
                }
            }
        }
    }
    None
}

fn extract_relationships(pair: &pest::iterators::Pair<Rule>) -> Relationships {
    let mut relationships = Relationships::none();

    fn find_relationships(
        pair: &pest::iterators::Pair<Rule>,
        relationships: &mut Relationships,
        depth: usize,
    ) {
        if depth > 0
            && matches!(
                pair.as_rule(),
                Rule::part_definition
                    | Rule::action_definition
                    | Rule::state_definition
                    | Rule::requirement_definition
                    | Rule::part_usage
                    | Rule::action_usage
                    | Rule::requirement_usage
            )
        {
            return;
        }

        match pair.as_rule() {
            Rule::subclassification_part => {
                // Definitions use subclassification_part for specialization
                for subclass in pair.clone().into_inner() {
                    if subclass.as_rule() == Rule::owned_subclassification
                        && let Some(target) = find_reference(&subclass)
                    {
                        relationships.specializes.push(target);
                    }
                }
            }
            Rule::satisfy_requirement_usage => {
                // Extract target from satisfy usage
                if let Some(target) = find_reference(pair) {
                    relationships.satisfies.push(target);
                }
            }
            Rule::perform_action_usage => {
                // Extract target from perform usage
                if let Some(target) = find_reference(pair) {
                    relationships.performs.push(target);
                }
            }
            Rule::exhibit_state_usage => {
                // Extract target from exhibit usage
                if let Some(target) = find_reference(pair) {
                    relationships.exhibits.push(target);
                }
            }
            Rule::include_use_case_usage => {
                // Extract target from include usage
                if let Some(target) = find_reference(pair) {
                    relationships.includes.push(target);
                }
            }
            Rule::feature_specialization => {
                // Usages use feature_specialization for various relationships
                for spec in pair.clone().into_inner() {
                    match spec.as_rule() {
                        Rule::typings => {
                            if let Some(target) = find_reference(&spec) {
                                relationships.typed_by = Some(target);
                            }
                        }
                        Rule::subsettings => {
                            for subsetting in spec.into_inner() {
                                if let Some(target) = find_reference(&subsetting) {
                                    relationships.subsets.push(target);
                                }
                            }
                        }
                        Rule::redefinitions => {
                            for redefining in spec.into_inner() {
                                if let Some(target) = find_reference(&redefining) {
                                    relationships.redefines.push(target);
                                }
                            }
                        }
                        Rule::references => {
                            for reference in spec.into_inner() {
                                if let Some(target) = find_reference(&reference) {
                                    relationships.references.push(target);
                                }
                            }
                        }
                        Rule::crosses => {
                            for cross in spec.into_inner() {
                                if let Some(target) = find_reference(&cross) {
                                    relationships.crosses.push(target);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                // Recursively search children
                for inner in pair.clone().into_inner() {
                    find_relationships(&inner, relationships, depth + 1);
                }
            }
        }
    }

    find_relationships(pair, &mut relationships, 0);
    relationships
}

macro_rules! impl_from_pest {
    ($type:ty, $body:expr) => {
        impl<'pest> FromPest<'pest> for $type {
            type Rule = Rule;
            type FatalError = Void;

            fn from_pest(
                pest: &mut pest::iterators::Pairs<'pest, Rule>,
            ) -> std::result::Result<Self, ConversionError<Void>> {
                let body_fn: fn(
                    &mut pest::iterators::Pairs<'pest, Rule>,
                ) -> std::result::Result<$type, ConversionError<Void>> = $body;
                body_fn(pest)
            }
        }
    };
}

impl_from_pest!(Package, |pest| {
    let mut name = None;
    let mut elements = Vec::new();
    let mut span = None;

    for pair in pest {
        if span.is_none() {
            span = Some(pest_span_to_span(pair.as_span()));
        }
        match pair.as_rule() {
            Rule::package_declaration => {
                name = pair
                    .into_inner()
                    .find(|p| p.as_rule() == Rule::identification)
                    .map(|id| id.as_str().to_string());
            }
            Rule::package_body => {
                // Parse package body elements
                for inner in pair.into_inner() {
                    if inner.as_rule() == Rule::package_body_items {
                        for body_item in inner.into_inner() {
                            if body_item.as_rule() == Rule::package_body_element
                                && let Ok(element) = Element::from_pest(&mut body_item.into_inner())
                            {
                                elements.push(element);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Package {
        name,
        elements,
        span,
    })
});

impl_from_pest!(Definition, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    let kind = rule_to_definition_kind(pair.as_rule())?;
    let name = find_name(pair.clone().into_inner());
    let relationships = extract_relationships(&pair);
    let span = Some(pest_span_to_span(pair.as_span()));

    // Parse body items - handle both definition_body and state_def_body
    let mut body = vec![];
    for inner in pair.clone().into_inner() {
        if inner.as_rule() == Rule::definition_body
            || inner.as_rule() == Rule::state_def_body
            || inner.as_rule() == Rule::case_body
            || inner.as_rule() == Rule::calculation_body
            || inner.as_rule() == Rule::requirement_body
        {
            // Recursively search for any usage elements
            for descendant in inner.clone().into_inner() {
                extract_usages_from_body(&descendant, &mut body);
            }
        }
    }
    Ok(Definition {
        kind,
        name,
        relationships,
        body,
        span,
        is_abstract: false,
        is_variation: false,
    })
});
impl_from_pest!(Usage, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    let kind = rule_to_usage_kind(pair.as_rule())?;
    let name = find_name(pair.clone().into_inner());
    let relationships = extract_relationships(&pair);
    let (is_derived, is_readonly) = extract_property_flags(&pair);
    let span = Some(pest_span_to_span(pair.as_span()));

    Ok(Usage {
        kind,
        name,
        relationships,
        body: vec![],
        span,
        is_derived,
        is_readonly,
    })
});
impl_from_pest!(Comment, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::comment_annotation {
        return Err(ConversionError::NoMatch);
    }
    let content = pair.as_str().to_string();
    let span = Some(pest_span_to_span(pair.as_span()));
    Ok(Comment { content, span })
});

impl_from_pest!(Import, |pest| {
    // We receive the children of Rule::import (import_prefix, imported_reference, etc.)
    let mut is_recursive = false;
    let mut path = String::new();
    let mut span = None;

    for pair in pest {
        if span.is_none() {
            span = Some(pest_span_to_span(pair.as_span()));
        }
        if pair.as_rule() == Rule::imported_reference {
            path = pair.as_str().to_string();
            // Check for recursive marker
            for inner in pair.into_inner() {
                if inner.as_rule() == Rule::recursive_marker {
                    is_recursive = true;
                }
            }
        }
    }

    Ok(Import {
        path,
        is_recursive,
        span,
    })
});

impl_from_pest!(Alias, |pest| {
    // We receive the children of Rule::alias_member_element
    let mut name = None;
    let mut target = String::new();
    let mut span = None;

    for pair in pest {
        if span.is_none() {
            span = Some(pest_span_to_span(pair.as_span()));
        }
        match pair.as_rule() {
            Rule::identification => {
                name = Some(pair.as_str().to_string());
            }
            Rule::element_reference => {
                target = pair.as_str().to_string();
            }
            _ => {}
        }
    }

    Ok(Alias { name, target, span })
});

impl_from_pest!(Element, |pest| {
    let mut pair = pest.next().ok_or(ConversionError::NoMatch)?;

    if pair.as_rule() == Rule::visibility {
        pair = pest.next().ok_or(ConversionError::NoMatch)?;
    }

    Ok(match pair.as_rule() {
        Rule::package | Rule::library_package => {
            Element::Package(Package::from_pest(&mut pair.into_inner())?)
        }
        Rule::package_declaration => Element::Package(Package::from_pest(&mut pair.into_inner())?),
        Rule::definition_member_element => Element::from_pest(&mut pair.into_inner())?,
        Rule::usage_member => Element::from_pest(&mut pair.into_inner())?,
        Rule::definition_element => Element::from_pest(&mut pair.into_inner())?,
        Rule::usage_element
        | Rule::occurrence_usage_element
        | Rule::structure_usage_element
        | Rule::behavior_usage_element
        | Rule::non_occurrence_usage_element => Element::from_pest(&mut pair.into_inner())?,
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
        | Rule::rendering_definition => {
            let kind = rule_to_definition_kind(pair.as_rule())?;
            let name = find_name(pair.clone().into_inner());
            let relationships = extract_relationships(&pair);

            // Parse body items - handle all body types
            let mut body = vec![];
            for inner in pair.clone().into_inner() {
                if inner.as_rule() == Rule::definition_body
                    || inner.as_rule() == Rule::state_def_body
                    || inner.as_rule() == Rule::case_body
                    || inner.as_rule() == Rule::calculation_body
                    || inner.as_rule() == Rule::requirement_body
                {
                    // Recursively search for any usage elements
                    for descendant in inner.clone().into_inner() {
                        extract_usages_from_body(&descendant, &mut body);
                    }
                }
            }

            Element::Definition(Definition {
                kind,
                name,
                relationships,
                body,
                span: Some(pest_span_to_span(pair.as_span())),
                is_abstract: false,  // TODO: extract from definition_prefix
                is_variation: false, // TODO: extract from definition_prefix
            })
        }
        Rule::part_usage
        | Rule::action_usage
        | Rule::requirement_usage
        | Rule::port_usage
        | Rule::item_usage
        | Rule::attribute_usage
        | Rule::concern_usage
        | Rule::case_usage
        | Rule::view_usage => {
            let kind = rule_to_usage_kind(pair.as_rule())?;
            let name = find_name(pair.clone().into_inner());
            let relationships = extract_relationships(&pair);
            let (is_derived, is_readonly) = extract_property_flags(&pair);

            Element::Usage(Usage {
                kind,
                name,
                relationships,
                body: vec![],
                span: Some(pest_span_to_span(pair.as_span())),
                is_derived,
                is_readonly,
            })
        }
        Rule::comment_annotation => Element::Comment(Comment::from_pest(&mut pair.into_inner())?),
        Rule::import => Element::Import(Import::from_pest(&mut pair.into_inner())?),
        Rule::alias_member_element => Element::Alias(Alias::from_pest(&mut pair.into_inner())?),
        _ => return Err(ConversionError::NoMatch),
    })
});

impl_from_pest!(SysMLFile, |pest| {
    let mut elements = Vec::new();
    let mut namespace = None;

    let model_pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if model_pair.as_rule() != Rule::model {
        return Err(ConversionError::NoMatch);
    }

    for pair in model_pair.into_inner() {
        if pair.as_rule() == Rule::namespace_element
            && let Ok(element) = Element::from_pest(&mut pair.into_inner())
        {
            // Only set namespace for file-level package declarations (without braces)
            // These have empty elements. Package blocks with braces have their children
            // in Package.elements and are handled by visit_package()
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
    Ok(SysMLFile {
        namespace,
        elements,
    })
});
