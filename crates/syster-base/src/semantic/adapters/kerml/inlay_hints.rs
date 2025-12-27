//! Inlay hint extraction for KerML files
//!
//! Provides inline annotations for types, parameter names, and other contextual information.

use crate::core::Position;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::{InlayHint, InlayHintKind};
use crate::syntax::kerml::ast::{Element, Feature, FeatureMember, KerMLFile};

/// Extract inlay hints for a KerML file
pub fn extract_inlay_hints(
    file: &KerMLFile,
    symbol_table: &SymbolTable,
    range: Option<(Position, Position)>,
) -> Vec<InlayHint> {
    let mut hints = Vec::new();

    for element in &file.elements {
        collect_hints(element, symbol_table, range, &mut hints);
    }

    hints
}

fn collect_hints(
    element: &Element,
    symbol_table: &SymbolTable,
    range: Option<(Position, Position)>,
    hints: &mut Vec<InlayHint>,
) {
    match element {
        Element::Package(p) => {
            for child in &p.elements {
                collect_hints(child, symbol_table, range, hints);
            }
        }
        Element::Feature(f) => {
            collect_feature_hints(f, symbol_table, range, hints);
        }
        _ => {}
    }
}

fn collect_feature_hints(
    feature: &Feature,
    symbol_table: &SymbolTable,
    range: Option<(Position, Position)>,
    hints: &mut Vec<InlayHint>,
) {
    // Check if feature is in the requested range
    if let Some(((start, end), span)) = range.zip(feature.span.as_ref())
        && (span.start < start || span.end > end)
    {
        return;
    }

    // Add type hint if feature has no explicit type
    if let Some(name) = &feature.name {
        // Check if feature has explicit typing relationship
        let has_typing = feature
            .body
            .iter()
            .any(|member| matches!(member, FeatureMember::Typing(_)));

        if !has_typing && feature.span.is_some() {
            // Try to infer type from symbol table
            if let Some(symbol) = symbol_table.lookup(name) {
                let type_name = match symbol {
                    crate::semantic::symbol_table::Symbol::Feature { feature_type, .. } => {
                        feature_type.as_ref()
                    }
                    _ => None,
                };

                if let (Some(type_name), Some(span)) = (type_name, &feature.span) {
                    // Position hint after the name
                    let hint_pos = Position {
                        line: span.start.line,
                        column: span.start.column + name.len(),
                    };

                    hints.push(InlayHint {
                        position: hint_pos,
                        label: format!(":  {type_name}"),
                        kind: InlayHintKind::Type,
                        padding_left: false,
                        padding_right: true,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_hints() {
        let file = KerMLFile {
            namespace: None,
            elements: vec![],
        };
        let symbol_table = SymbolTable::new();
        let hints = extract_inlay_hints(&file, &symbol_table, None);
        assert!(hints.is_empty());
    }
}
