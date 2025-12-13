use super::{
    enums::{DefinitionKind, Element, UsageKind},
    types::{Comment, Definition, Import, Package, Relationships, SysMLFile, Usage},
};
use crate::parser::sysml::Rule;
use from_pest::{ConversionError, FromPest, Void};

fn find_reference(pair: &pest::iterators::Pair<Rule>) -> Option<String> {
    match pair.as_rule() {
        Rule::identifier
        | Rule::quoted_name
        | Rule::feature_reference
        | Rule::classifier_reference => {
            return Some(pair.as_str().to_string());
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
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::package_declaration {
        return Err(ConversionError::NoMatch);
    }
    let name = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::identification)
        .map(|id| id.as_str().to_string());
    Ok(Package {
        name,
        elements: vec![],
    })
});

impl_from_pest!(Definition, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    let kind = rule_to_definition_kind(pair.as_rule())?;
    let name = find_name(pair.clone().into_inner());
    let relationships = extract_relationships(&pair);

    Ok(Definition {
        kind,
        name,
        relationships,
        body: vec![],
    })
});
impl_from_pest!(Usage, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    let kind = rule_to_usage_kind(pair.as_rule())?;
    let name = find_name(pair.clone().into_inner());
    let relationships = extract_relationships(&pair);

    Ok(Usage {
        kind,
        name,
        relationships,
        body: vec![],
    })
});
impl_from_pest!(Comment, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::comment_annotation {
        return Err(ConversionError::NoMatch);
    }
    let content = pair.as_str().to_string();
    Ok(Comment { content })
});

impl_from_pest!(Import, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::import {
        return Err(ConversionError::NoMatch);
    }
    let path = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::imported_reference || p.as_rule() == Rule::identification)
        .map(|p| p.as_str().to_string())
        .unwrap_or_default();
    Ok(Import {
        path,
        is_recursive: false,
    })
});

impl_from_pest!(Element, |pest| {
    let mut pair = pest.next().ok_or(ConversionError::NoMatch)?;

    if pair.as_rule() == Rule::visibility {
        pair = pest.next().ok_or(ConversionError::NoMatch)?;
    }

    Ok(match pair.as_rule() {
        Rule::package_declaration => Element::Package(Package::from_pest(&mut pair.into_inner())?),
        Rule::definition_element => Element::from_pest(&mut pair.into_inner())?,
        Rule::usage_element
        | Rule::occurrence_usage_element
        | Rule::structure_usage_element
        | Rule::behavior_usage_element
        | Rule::non_occurrence_usage_element => Element::from_pest(&mut pair.into_inner())?,
        Rule::part_definition
        | Rule::action_definition
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

            Element::Definition(Definition {
                kind,
                name,
                relationships,
                body: vec![],
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

            Element::Usage(Usage {
                kind,
                name,
                relationships,
                body: vec![],
            })
        }
        Rule::comment_annotation => Element::Comment(Comment::from_pest(&mut pair.into_inner())?),
        Rule::import => Element::Import(Import::from_pest(&mut pair.into_inner())?),
        _ => return Err(ConversionError::NoMatch),
    })
});

impl_from_pest!(SysMLFile, |pest| {
    let mut elements = Vec::new();

    let model_pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if model_pair.as_rule() != Rule::model {
        return Err(ConversionError::NoMatch);
    }

    for pair in model_pair.into_inner() {
        if pair.as_rule() == Rule::namespace_element {
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::definition_member_element | Rule::usage_member => {
                        if let Ok(element) = Element::from_pest(&mut inner.into_inner()) {
                            elements.push(element);
                        }
                    }
                    Rule::package | Rule::library_package => {
                        if let Ok(element) = Element::from_pest(&mut inner.into_inner()) {
                            elements.push(element);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(SysMLFile {
        namespace: None,
        elements,
    })
});
