use super::{
    enums::Element,
    parsers::{parse_definition, parse_usage},
    types::{Alias, Comment, Definition, Import, NamespaceDeclaration, Package, SysMLFile, Usage},
    utils::{find_in, is_definition_rule, is_usage_rule, to_span},
};
use crate::parser::sysml::Rule;
use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::Pairs;

// ============================================================================
// FromPest implementations
// ============================================================================

macro_rules! impl_from_pest {
    ($type:ty, $body:expr) => {
        impl<'pest> FromPest<'pest> for $type {
            type Rule = Rule;
            type FatalError = Void;
            fn from_pest(pest: &mut Pairs<'pest, Rule>) -> Result<Self, ConversionError<Void>> {
                $body(pest)
            }
        }
    };
}

impl_from_pest!(Package, |pest: &mut Pairs<Rule>| {
    let mut name = None;
    let mut elements = Vec::new();
    let mut span = None;

    for pair in pest {
        match pair.as_rule() {
            Rule::package_declaration => {
                if let Some(p) = find_in(&pair, Rule::identification) {
                    name = Some(p.as_str().to_string());
                    // Set span to the identifier, not the whole declaration
                    span = Some(to_span(p.as_span()));
                }
            }
            Rule::package_body => {
                elements = pair
                    .into_inner()
                    .filter(|p| p.as_rule() == Rule::package_body_items)
                    .flat_map(|p| p.into_inner())
                    .filter(|p| p.as_rule() == Rule::package_body_element)
                    .filter_map(|p| Element::from_pest(&mut p.into_inner()).ok())
                    .collect();
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

impl_from_pest!(Definition, |pest: &mut Pairs<Rule>| {
    parse_definition(pest.next().ok_or(ConversionError::NoMatch)?)
});

impl_from_pest!(Usage, |pest: &mut Pairs<Rule>| {
    Ok(parse_usage(pest.next().ok_or(ConversionError::NoMatch)?))
});

impl_from_pest!(Comment, |pest: &mut Pairs<Rule>| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::comment_annotation {
        return Err(ConversionError::NoMatch);
    }
    Ok(Comment {
        content: pair.as_str().to_string(),
        span: Some(to_span(pair.as_span())),
    })
});

impl_from_pest!(Import, |pest: &mut Pairs<Rule>| {
    let mut is_recursive = false;
    let mut path = String::new();
    let mut path_span = None;
    let mut span = None;

    for pair in pest {
        if pair.as_rule() == Rule::imported_reference {
            path = pair.as_str().to_string();
            // Capture the span of the imported path, not the whole import statement
            span = Some(to_span(pair.as_span()));
            // Also capture the path_span for semantic tokens
            path_span = Some(to_span(pair.as_span()));
            is_recursive = pair
                .clone()
                .into_inner()
                .any(|p| p.as_rule() == Rule::recursive_marker);
        }
    }

    Ok(Import {
        path,
        path_span,
        is_recursive,
        span,
    })
});

impl_from_pest!(Alias, |pest: &mut Pairs<Rule>| {
    let mut name = None;
    let mut target = String::new();
    let mut target_span = None;
    let mut span = None;

    for pair in pest {
        span.get_or_insert_with(|| to_span(pair.as_span()));
        match pair.as_rule() {
            Rule::identification => name = Some(pair.as_str().to_string()),
            Rule::element_reference => {
                target = pair.as_str().to_string();
                target_span = Some(to_span(pair.as_span()));
            }
            _ => {}
        }
    }

    Ok(Alias {
        name,
        target,
        target_span,
        span,
    })
});

impl_from_pest!(Element, |pest: &mut Pairs<Rule>| {
    let mut pair = pest.next().ok_or(ConversionError::NoMatch)?;

    if pair.as_rule() == Rule::visibility {
        pair = pest.next().ok_or(ConversionError::NoMatch)?;
    }

    Ok(match pair.as_rule() {
        Rule::package | Rule::library_package | Rule::package_declaration => {
            Element::Package(Package::from_pest(&mut pair.into_inner())?)
        }
        Rule::definition_member_element
        | Rule::usage_member
        | Rule::definition_element
        | Rule::usage_element
        | Rule::occurrence_usage_element
        | Rule::structure_usage_element
        | Rule::behavior_usage_element
        | Rule::non_occurrence_usage_element => Element::from_pest(&mut pair.into_inner())?,
        r if is_definition_rule(r) => Element::Definition(parse_definition(pair)?),
        r if is_usage_rule(r) => Element::Usage(parse_usage(pair)),
        Rule::comment_annotation => Element::Comment(Comment::from_pest(&mut pair.into_inner())?),
        Rule::import => Element::Import(Import::from_pest(&mut pair.into_inner())?),
        Rule::alias_member_element => Element::Alias(Alias::from_pest(&mut pair.into_inner())?),
        _ => return Err(ConversionError::NoMatch),
    })
});

impl_from_pest!(SysMLFile, |pest: &mut Pairs<Rule>| {
    let model = pest.next().ok_or(ConversionError::NoMatch)?;
    if model.as_rule() != Rule::model {
        return Err(ConversionError::NoMatch);
    }

    let mut elements = Vec::new();
    let mut namespace = None;
    let mut namespaces = Vec::new();

    for pair in model.into_inner() {
        if pair.as_rule() == Rule::namespace_element
            && let Ok(element) = Element::from_pest(&mut pair.into_inner())
        {
            // Track all package declarations (Issue #10)
            if let Element::Package(ref pkg) = element
                && pkg.elements.is_empty()
                && let Some(ref name) = pkg.name
            {
                let ns = NamespaceDeclaration {
                    name: name.clone(),
                    span: pkg.span,
                };

                // Keep first namespace for backward compatibility
                if namespace.is_none() {
                    namespace = Some(ns.clone());
                }

                // Collect all namespaces
                namespaces.push(ns);
            }
            elements.push(element);
        }
    }

    Ok(SysMLFile {
        namespace,
        namespaces,
        elements,
    })
});
