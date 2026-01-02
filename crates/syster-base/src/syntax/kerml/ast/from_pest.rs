use super::enums::{Element, ImportKind};
use super::parsers::{parse_classifier, parse_feature};
use super::types::{
    Annotation, Classifier, Comment, Feature, Import, KerMLFile, NamespaceDeclaration, Package,
};
use super::utils::{find_name, is_classifier_rule, to_span};
use crate::parser::kerml::Rule;
use crate::syntax::kerml::model::types::Documentation;
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
    let mut elements = Vec::new();
    let mut span = None;
    let pairs: Vec<_> = pest.collect();

    for pair in &pairs {
        span.get_or_insert_with(|| to_span(pair.as_span()));
        if pair.as_rule() == Rule::namespace_body {
            elements = pair
                .clone()
                .into_inner()
                .filter(|p| p.as_rule() == Rule::namespace_body_elements)
                .flat_map(|p| p.into_inner())
                .filter(|p| p.as_rule() == Rule::namespace_body_element)
                .filter_map(|p| Element::from_pest(&mut p.into_inner()).ok())
                .collect();
        }
    }

    Ok(Package {
        name: find_name(pairs.into_iter()),
        elements,
        span,
    })
});

impl_from_pest!(Comment, |pest: &mut Pairs<Rule>| {
    let mut content = String::new();
    let mut span = None;

    for pair in pest {
        span.get_or_insert_with(|| to_span(pair.as_span()));
        if pair.as_rule() == Rule::comment_annotation {
            content = pair.as_str().to_string();
        }
    }

    Ok(Comment {
        content,
        about: Vec::new(),
        locale: None,
        span,
    })
});

impl_from_pest!(Documentation, |pest: &mut Pairs<Rule>| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::documentation {
        return Err(ConversionError::NoMatch);
    }

    let span = Some(to_span(pair.as_span()));
    let content = pair.as_str().to_string();

    Ok(Documentation {
        comment: Comment {
            content,
            about: Vec::new(),
            locale: None,
            span,
        },
        span,
    })
});

impl_from_pest!(Classifier, |pest: &mut Pairs<Rule>| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    parse_classifier(pair)
});

impl_from_pest!(Feature, |pest: &mut Pairs<Rule>| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    Ok(parse_feature(pair))
});

impl_from_pest!(Import, |pest: &mut Pairs<Rule>| {
    let mut path = String::new();
    let mut path_span = None;
    let mut is_recursive = false;
    let mut span = None;

    for pair in pest {
        if pair.as_rule() == Rule::imported_reference {
            span = Some(to_span(pair.as_span()));
            // imported_reference contains element_reference and optional import_kind
            for child in pair.into_inner() {
                match child.as_rule() {
                    Rule::element_reference => {
                        path = child.as_str().to_string();
                        path_span = Some(to_span(child.as_span()));
                    }
                    Rule::import_kind => {
                        is_recursive = child.as_str().contains("**");
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(Import {
        path,
        path_span,
        is_recursive,
        kind: ImportKind::Normal,
        span,
    })
});

impl_from_pest!(Element, |pest: &mut Pairs<Rule>| {
    let mut pair = pest.next().ok_or(ConversionError::NoMatch)?;

    // Skip visibility prefix
    if pair.as_rule() == Rule::visibility_kind {
        pair = pest.next().ok_or(ConversionError::NoMatch)?;
    }

    Ok(match pair.as_rule() {
        // Package rules
        Rule::package | Rule::library_package => {
            Element::Package(Package::from_pest(&mut pair.into_inner())?)
        }

        // Wrapper rules - recurse
        Rule::namespace_body_element
        | Rule::non_feature_member
        | Rule::non_feature_element
        | Rule::namespace_feature_member
        | Rule::typed_feature_member => Element::from_pest(&mut pair.into_inner())?,

        // Classifier rules
        r if is_classifier_rule(r) => Element::Classifier(parse_classifier(pair)?),

        // Feature rules
        Rule::feature | Rule::feature_element => Element::Feature(parse_feature(pair)),

        // Other elements
        Rule::comment_annotation => Element::Comment(Comment::from_pest(&mut pair.into_inner())?),
        Rule::annotation => Element::Annotation(Annotation {
            reference: pair.as_str().to_string(),
            span: Some(to_span(pair.as_span())),
        }),
        Rule::import => Element::Import(Import::from_pest(&mut pair.into_inner())?),

        _ => return Err(ConversionError::NoMatch),
    })
});

impl_from_pest!(KerMLFile, |pest: &mut Pairs<Rule>| {
    let model = pest.next().ok_or(ConversionError::NoMatch)?;
    if model.as_rule() != Rule::file {
        return Err(ConversionError::NoMatch);
    }

    let mut elements = Vec::new();
    let mut namespace = None;

    for pair in model.into_inner() {
        if pair.as_rule() == Rule::namespace_element
            && let Ok(element) = Element::from_pest(&mut pair.into_inner())
        {
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

    Ok(KerMLFile {
        namespace,
        elements,
    })
});
