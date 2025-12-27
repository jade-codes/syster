//! Selection range extraction for KerML files
//!
//! Finds hierarchical selection ranges at a given position in the AST.

use crate::core::{Position, Span};
use crate::syntax::kerml::ast::{
    Classifier, ClassifierMember, Element, Feature, FeatureMember, KerMLFile, Package,
};

/// Find selection spans at a position in a KerML file
///
/// Returns spans ordered from innermost (smallest) to outermost (largest).
/// Returns empty vector if the position is not within any AST node.
pub fn find_selection_spans(file: &KerMLFile, position: Position) -> Vec<Span> {
    let mut spans: Vec<Span> = Vec::new();

    for element in &file.elements {
        if collect_containing_spans(element, position, &mut spans) {
            break;
        }
    }

    if spans.is_empty() {
        return Vec::new();
    }

    // Sort by range size (smallest first for innermost)
    spans.sort_by(|a, b| {
        let size_a = range_size(a);
        let size_b = range_size(b);
        size_a.cmp(&size_b)
    });

    spans
}

/// Calculate a rough "size" of a span for sorting
fn range_size(span: &Span) -> usize {
    let lines = span.end.line.saturating_sub(span.start.line);
    let cols = if lines == 0 {
        span.end.column.saturating_sub(span.start.column)
    } else {
        span.end.column + 100
    };
    lines * 100 + cols
}

/// Try to push a span if it contains the position
fn try_push_span(span: &Option<Span>, position: Position, spans: &mut Vec<Span>) -> bool {
    if let Some(span) = span
        && span.contains(position)
    {
        spans.push(*span);
        return true;
    }
    false
}

/// Recursively collect all spans that contain the position
fn collect_containing_spans(element: &Element, position: Position, spans: &mut Vec<Span>) -> bool {
    match element {
        Element::Package(p) => collect_package_spans(p, position, spans),
        Element::Classifier(c) => collect_classifier_spans(c, position, spans),
        Element::Feature(f) => collect_feature_spans(f, position, spans),
        Element::Comment(c) => try_push_span(&c.span, position, spans),
        Element::Import(i) => try_push_span(&i.span, position, spans),
        Element::Annotation(a) => try_push_span(&a.span, position, spans),
    }
}

fn collect_package_spans(package: &Package, position: Position, spans: &mut Vec<Span>) -> bool {
    if !try_push_span(&package.span, position, spans) {
        return false;
    }

    for child in &package.elements {
        if collect_containing_spans(child, position, spans) {
            return true;
        }
    }

    true
}

fn collect_classifier_spans(
    classifier: &Classifier,
    position: Position,
    spans: &mut Vec<Span>,
) -> bool {
    if !try_push_span(&classifier.span, position, spans) {
        return false;
    }

    for member in &classifier.body {
        match member {
            ClassifierMember::Feature(f) => {
                if collect_feature_spans(f, position, spans) {
                    return true;
                }
            }
            ClassifierMember::Comment(c) => {
                if try_push_span(&c.span, position, spans) {
                    return true;
                }
            }
            _ => {}
        }
    }

    true
}

fn collect_feature_spans(feature: &Feature, position: Position, spans: &mut Vec<Span>) -> bool {
    if !try_push_span(&feature.span, position, spans) {
        return false;
    }

    for member in &feature.body {
        if let FeatureMember::Comment(c) = member
            && try_push_span(&c.span, position, spans)
        {
            return true;
        }
    }

    true
}
