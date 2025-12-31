//! Folding range extraction for KerML files

use crate::semantic::types::FoldingRangeInfo;
use crate::syntax::kerml::ast::{ClassifierMember, Element, FeatureMember, KerMLFile};

/// Extract all foldable ranges from a KerML file
pub fn extract_folding_ranges(file: &KerMLFile) -> Vec<FoldingRangeInfo> {
    let mut ranges = Vec::new();

    for element in &file.elements {
        collect_ranges(element, &mut ranges);
    }

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
        Element::Classifier(c) => {
            if let Some(span) = &c.span {
                ranges.push(FoldingRangeInfo::code(*span));
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
                ranges.push(FoldingRangeInfo::code(*span));
            }
            for member in &f.body {
                if let FeatureMember::Comment(c) = member {
                    collect_ranges(&Element::Comment(c.clone()), ranges);
                }
            }
        }
        Element::Comment(c) => {
            if let Some(span) = &c.span {
                ranges.push(FoldingRangeInfo::comment(*span));
            }
        }
        Element::Import(_) | Element::Annotation(_) => {}
    }
}
