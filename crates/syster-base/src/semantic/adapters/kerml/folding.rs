//! Folding range extraction for KerML files

use crate::core::Span;
use crate::syntax::kerml::ast::{ClassifierMember, Element, FeatureMember, KerMLFile};

/// A simple folding range with span and whether it's a comment
#[derive(Debug, Clone)]
pub struct FoldingSpan {
    pub span: Span,
    pub is_comment: bool,
}

/// Extract all foldable ranges from a KerML file
pub fn extract_folding_ranges(file: &KerMLFile) -> Vec<FoldingSpan> {
    let mut ranges = Vec::new();

    for element in &file.elements {
        collect_ranges(element, &mut ranges);
    }

    ranges.retain(|r| r.span.end.line > r.span.start.line);
    ranges.sort_by_key(|r| r.span.start.line);
    ranges
}

/// Try to push a folding span if present
fn try_push(span: &Option<Span>, is_comment: bool, ranges: &mut Vec<FoldingSpan>) {
    if let Some(span) = span {
        ranges.push(FoldingSpan {
            span: *span,
            is_comment,
        });
    }
}

/// Recursively collect folding ranges from an element and its children
fn collect_ranges(element: &Element, ranges: &mut Vec<FoldingSpan>) {
    match element {
        Element::Package(p) => {
            try_push(&p.span, false, ranges);
            for child in &p.elements {
                collect_ranges(child, ranges);
            }
        }
        Element::Classifier(c) => {
            try_push(&c.span, false, ranges);
            for member in &c.body {
                match member {
                    ClassifierMember::Feature(f) => {
                        collect_ranges(&Element::Feature(f.clone()), ranges)
                    }
                    ClassifierMember::Comment(c) => {
                        collect_ranges(&Element::Comment(c.clone()), ranges)
                    }
                    _ => {}
                }
            }
        }
        Element::Feature(f) => {
            try_push(&f.span, false, ranges);
            for member in &f.body {
                if let FeatureMember::Comment(c) = member {
                    collect_ranges(&Element::Comment(c.clone()), ranges);
                }
            }
        }
        Element::Comment(c) => try_push(&c.span, true, ranges),
        Element::Import(_) | Element::Annotation(_) => {}
    }
}
