//! Folding range extraction for SysML files

use crate::core::Span;
use crate::syntax::sysml::ast::{DefinitionMember, Element, SysMLFile, UsageMember};

/// A simple folding range with span and whether it's a comment
#[derive(Debug, Clone)]
pub struct FoldingSpan {
    pub span: Span,
    pub is_comment: bool,
}

/// Extract all foldable ranges from a SysML file
pub fn extract_folding_ranges(file: &SysMLFile) -> Vec<FoldingSpan> {
    let mut ranges = Vec::new();

    for element in &file.elements {
        collect_ranges(element, &mut ranges);
    }

    // Keep only multiline ranges and sort by start line
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
        Element::Definition(d) => {
            if let Some(span) = &d.span {
                ranges.push(FoldingSpan {
                    span: *span,
                    is_comment: false,
                });
            }
            for member in &d.body {
                match member {
                    DefinitionMember::Usage(u) => {
                        collect_ranges(&Element::Usage((**u).clone()), ranges)
                    }
                    DefinitionMember::Comment(c) => {
                        collect_ranges(&Element::Comment((**c).clone()), ranges)
                    }
                }
            }
        }
        Element::Usage(u) => {
            if let Some(span) = &u.span {
                ranges.push(FoldingSpan {
                    span: *span,
                    is_comment: false,
                });
            }
            for member in &u.body {
                match member {
                    UsageMember::Usage(u) => collect_ranges(&Element::Usage((**u).clone()), ranges),
                    UsageMember::Comment(c) => collect_ranges(&Element::Comment(c.clone()), ranges),
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
        Element::Import(_) | Element::Alias(_) => {}
    }
}
