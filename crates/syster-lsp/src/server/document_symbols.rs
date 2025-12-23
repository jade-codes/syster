use super::LspServer;
use std::collections::HashMap;
use std::path::Path;
use syster::semantic::symbol_table::Symbol;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

impl LspServer {
    /// Get all symbols in a document for the outline view
    pub fn get_document_symbols(&self, file_path: &Path) -> Vec<DocumentSymbol> {
        let mut flat_symbols = Vec::new();

        // Collect all symbols from this file
        for (_, symbol) in self.workspace.symbol_table().all_symbols() {
            // Only include symbols defined in this file
            if symbol.source_file() != Some(file_path.to_str().unwrap_or("")) {
                continue;
            }

            if let Some(span) = symbol.span() {
                let range = super::helpers::span_to_lsp_range(&span);
                let selection_range = range;

                let symbol_kind = match symbol {
                    Symbol::Package { .. } => SymbolKind::NAMESPACE,
                    Symbol::Classifier { .. } | Symbol::Definition { .. } => SymbolKind::CLASS,
                    Symbol::Feature { .. } | Symbol::Usage { .. } => SymbolKind::PROPERTY,
                    Symbol::Alias { .. } => SymbolKind::VARIABLE,
                };

                let doc_symbol = DocumentSymbol {
                    name: symbol.name().to_string(),
                    detail: Some(symbol.qualified_name().to_string()),
                    kind: symbol_kind,
                    range,
                    selection_range,
                    children: Some(Vec::new()),
                    tags: None,
                    #[allow(deprecated)]
                    deprecated: None,
                };

                flat_symbols.push((symbol.qualified_name().to_string(), doc_symbol));
            }
        }

        // Build hierarchy from qualified names
        self.build_symbol_hierarchy(flat_symbols)
    }

    /// Build a hierarchical structure from flat symbols using qualified names
    fn build_symbol_hierarchy(
        &self,
        flat_symbols: Vec<(String, DocumentSymbol)>,
    ) -> Vec<DocumentSymbol> {
        let mut symbol_map: HashMap<String, DocumentSymbol> = HashMap::new();

        // First, add all symbols to the map
        for (qualified_name, symbol) in flat_symbols {
            symbol_map.insert(qualified_name, symbol);
        }

        // Get all names and sort by depth (MORE "::" first, so deepest children are processed first)
        let mut all_names: Vec<String> = symbol_map.keys().cloned().collect();
        all_names.sort_by(|a, b| {
            let depth_a = a.matches("::").count();
            let depth_b = b.matches("::").count();
            depth_b.cmp(&depth_a) // Reverse order: deepest first
        });

        // Build hierarchy by moving children into parents, starting from deepest
        for qualified_name in &all_names {
            if let Some(last_separator) = qualified_name.rfind("::") {
                let parent_name = &qualified_name[..last_separator];

                // Check if parent exists and child hasn't been moved yet
                if symbol_map.contains_key(parent_name) && symbol_map.contains_key(qualified_name) {
                    // Remove child from map
                    let child = symbol_map.remove(qualified_name).unwrap();

                    // Add child to parent's children
                    if let Some(parent) = symbol_map.get_mut(parent_name)
                        && let Some(ref mut children) = parent.children
                    {
                        children.push(child);
                    }
                }
            }
        }

        // Remaining symbols in the map are root symbols
        let mut root_symbols: Vec<DocumentSymbol> = symbol_map.into_values().collect();
        root_symbols.sort_by(|a, b| a.name.cmp(&b.name));
        root_symbols
    }
}
