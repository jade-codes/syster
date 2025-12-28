//! Inlay hint extraction for SysML files
//!
//! Provides inline annotations for types, parameter names, and other contextual information.

use crate::core::Position;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::{InlayHint, InlayHintKind};
use crate::syntax::sysml::ast::{
    Definition, DefinitionMember, Element, SysMLFile, Usage, UsageMember,
};

/// Extract inlay hints for a SysML file
pub fn extract_inlay_hints(
    file: &SysMLFile,
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
        Element::Definition(d) => {
            collect_definition_hints(d, symbol_table, range, hints);
        }
        Element::Usage(u) => {
            collect_usage_hints(u, symbol_table, range, hints);
        }
        _ => {}
    }
}

fn collect_definition_hints(
    def: &Definition,
    symbol_table: &SymbolTable,
    range: Option<(Position, Position)>,
    hints: &mut Vec<InlayHint>,
) {
    // Recurse into body
    for member in &def.body {
        if let DefinitionMember::Usage(u) = member {
            collect_usage_hints(u, symbol_table, range, hints);
        }
    }
}

fn collect_usage_hints(
    usage: &Usage,
    symbol_table: &SymbolTable,
    range: Option<(Position, Position)>,
    hints: &mut Vec<InlayHint>,
) {
    // Check if usage is in the requested range
    if let Some(((start, end), span)) = range.zip(usage.span.as_ref())
        && (span.start < start || span.end > end)
    {
        return;
    }

    // Add type hint if usage has no explicit type
    if let Some(name) = &usage.name {
        // Check if usage has typing relationships
        let has_typing = usage.relationships.typed_by.is_some();

        if !has_typing && usage.span.is_some() {
            // Try to infer type from symbol table
            if let Some(symbol) = symbol_table.lookup(name) {
                // For usage symbols, check usage_type field
                let type_name = match symbol {
                    crate::semantic::symbol_table::Symbol::Usage { usage_type, .. } => {
                        usage_type.as_ref()
                    }
                    _ => None,
                };

                if let (Some(type_name), Some(span)) = (type_name, &usage.span) {
                    // Position hint after the name
                    let hint_pos = Position {
                        line: span.start.line,
                        column: span.start.column + name.len(),
                    };

                    hints.push(InlayHint {
                        position: hint_pos,
                        label: format!(
                            ":
 {type_name}"
                        ),
                        kind: InlayHintKind::Type,
                        padding_left: false,
                        padding_right: true,
                    });
                }
            }
        }
    }

    // Recurse into body
    for member in &usage.body {
        if let UsageMember::Usage(u) = member {
            collect_usage_hints(u, symbol_table, range, hints);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_hints() {
        let file = SysMLFile {
            namespace: None,
            namespaces: vec![],
            elements: vec![],
        };
        let symbol_table = SymbolTable::new();
        let hints = extract_inlay_hints(&file, &symbol_table, None);
        assert!(hints.is_empty());
    }
}
