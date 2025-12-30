//! Folding range extraction for SysML files

use crate::semantic::types::FoldingRangeInfo;
use crate::syntax::sysml::ast::{DefinitionMember, Element, SysMLFile, UsageMember};

/// Extract all foldable ranges from a SysML file
pub fn extract_folding_ranges(file: &SysMLFile) -> Vec<FoldingRangeInfo> {
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
fn collect_ranges(element: &Element, ranges: &mut Vec<FoldingRangeInfo>) {
    match element {
        Element::Package(p) => {
            if let Some(span) = &p.span {
                ranges.push(FoldingRangeInfo::code(*span));
            }
            for child in &p.elements {
                collect_ranges(child, ranges);
            }
        }
        Element::Definition(d) => {
            if let Some(span) = &d.span {
                ranges.push(FoldingRangeInfo::code(*span));
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
                ranges.push(FoldingRangeInfo::code(*span));
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
                ranges.push(FoldingRangeInfo::comment(*span));
            }
        }
        Element::Import(_) | Element::Alias(_) => {}
    }
}
