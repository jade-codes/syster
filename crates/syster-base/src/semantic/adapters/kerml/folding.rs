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

/// Recursively collect folding ranges from an element and its children
fn collect_ranges(element: &Element, ranges: &mut Vec<FoldingSpan>) {
    match element {
        Element::Package(p) => {
            if let Some(span) = &p.span {
                ranges.push(FoldingSpan {
                    span: *span,
                    is_comment: false,
                });
            }
            for child in &p.elements {
                collect_ranges(child, ranges);
            }
        }
        Element::Classifier(c) => {
            if let Some(span) = &c.span {
                ranges.push(FoldingSpan {
                    span: *span,
                    is_comment: false,
                });
            }
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
            if let Some(span) = &f.span {
                ranges.push(FoldingSpan {
                    span: *span,
                    is_comment: false,
                });
            }
            for member in &f.body {
                if let FeatureMember::Comment(c) = member {
                    collect_ranges(&Element::Comment(c.clone()), ranges);
                }
            }
        }
        Element::Comment(c) => {
            if let Some(span) = &c.span {
                ranges.push(FoldingSpan {
                    span: *span,
                    is_comment: true,
                });
            }
        }
        Element::Import(_) | Element::Annotation(_) => {}
    }
}
