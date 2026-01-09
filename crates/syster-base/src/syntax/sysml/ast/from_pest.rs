use super::{
    enums::Element,
    parsers::{parse_definition, parse_usage},
    types::{Alias, Comment, Definition, Import, NamespaceDeclaration, Package, SysMLFile, Usage},
    utils::{
        extract_name_from_identification, find_in, is_definition_rule, is_usage_rule, to_span,
    },
};
use crate::parser::sysml::Rule;
use from_pest::{ConversionError, FromPest, Void};
use pest::iterators::{Pair, Pairs};

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
                    // Use extract_name_from_identification to properly handle short names
                    // e.g., `<USCU> USCustomaryUnits` → name = "USCustomaryUnits"
                    // e.g., `<kg>` → name = "kg" (if only short name is present)
                    let (extracted_name, extracted_span) = extract_name_from_identification(p);
                    name = extracted_name;
                    span = extracted_span;
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
    let mut is_public = false;
    let mut path = String::new();
    let mut path_span = None;
    let mut span = None;

    // Helper to extract visibility from import_prefix
    fn extract_visibility(pair: &Pair<Rule>) -> bool {
        for child in pair.clone().into_inner() {
            if child.as_rule() == Rule::visibility {
                return child.as_str().trim() == "public";
            }
        }
        false
    }

    // Helper to extract path and visibility from membership_import or namespace_import
    fn extract_from_import_rule(
        pair: Pair<Rule>,
    ) -> (String, Option<crate::core::Span>, bool, bool) {
        let mut path = String::new();
        let mut span = None;
        let mut is_recursive = false;
        let mut is_public = false;

        for child in pair.into_inner() {
            match child.as_rule() {
                Rule::import_prefix => {
                    is_public = extract_visibility(&child);
                }
                Rule::imported_membership | Rule::imported_namespace => {
                    path = child.as_str().to_string();
                    span = Some(to_span(child.as_span()));
                    is_recursive = child
                        .clone()
                        .into_inner()
                        .any(|p| p.as_rule() == Rule::recursive_marker);
                }
                Rule::qualified_name => {
                    path = child.as_str().to_string();
                    span = Some(to_span(child.as_span()));
                }
                _ => {}
            }
        }
        (path, span, is_recursive, is_public)
    }

    for pair in pest {
        match pair.as_rule() {
            Rule::import_prefix => {
                // import_prefix contains: visibility? ~ import_token ~ import_all?
                is_public = extract_visibility(&pair);
            }
            Rule::membership_import | Rule::namespace_import => {
                let (p, s, r, pub_flag) = extract_from_import_rule(pair);
                path = p;
                path_span = s;
                span = s;
                is_recursive = r;
                is_public = pub_flag;
            }
            Rule::imported_membership | Rule::imported_namespace => {
                path = pair.as_str().to_string();
                span = Some(to_span(pair.as_span()));
                path_span = Some(to_span(pair.as_span()));
                is_recursive = pair
                    .clone()
                    .into_inner()
                    .any(|p| p.as_rule() == Rule::recursive_marker);
            }
            _ => {}
        }
    }

    Ok(Import {
        path,
        path_span,
        is_recursive,
        is_public,
        span,
    })
});

impl_from_pest!(Alias, |pest: &mut Pairs<Rule>| {
    let mut name = None;
    let mut target = String::new();
    let mut target_span = None;
    let mut span = None;

    for pair in pest {
        match pair.as_rule() {
            Rule::identification => {
                // Use extract_name_from_identification to properly handle short names
                let (extracted_name, extracted_span) = extract_name_from_identification(pair);
                name = extracted_name;
                span = extracted_span;
            }
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

    // Check for visibility prefix (public/private/protected)
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

    // model = { SOI ~ root_namespace ~ EOI }
    // root_namespace = { package_body_element* }
    // namespace_element = { package_body_element } (legacy alias)
    for pair in model.into_inner() {
        // Handle both root_namespace container and legacy namespace_element
        let inner_pairs = if pair.as_rule() == Rule::root_namespace {
            // New grammar structure: get package_body_elements from root_namespace
            pair.into_inner().collect::<Vec<_>>()
        } else if pair.as_rule() == Rule::namespace_element
            || pair.as_rule() == Rule::package_body_element
        {
            // Direct element (legacy or package_body_element)
            vec![pair]
        } else {
            continue;
        };

        for element_pair in inner_pairs {
            // Each package_body_element or namespace_element contains the actual element
            if let Ok(element) = Element::from_pest(&mut element_pair.into_inner()) {
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
    }

    Ok(SysMLFile {
        namespace,
        namespaces,
        elements,
    })
});
