use super::LspServer;
use super::helpers::{collect_reference_locations, span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{CodeLens, Command, Position, Url};
use syster::semantic::symbol_table::Symbol;

impl LspServer {
    /// Get code lenses for a document
    ///
    /// Shows inline commands above definitions:
    /// - "N references" - clickable to show all references
    /// - "N implementations" - for abstract definitions (future)
    pub fn get_code_lenses(&self, uri: &Url) -> Vec<CodeLens> {
        let Some(path) = uri_to_path(uri) else {
            return Vec::new();
        };

        let mut lenses = Vec::new();

        // Collect all symbols from this file
        for symbol in self.workspace.symbol_table().iter_symbols() {
            // Only include symbols defined in this file
            let Some(symbol_path) = symbol.source_file() else {
                continue;
            };
            if symbol_path != path.to_str().unwrap_or("") {
                continue;
            }

            // Only show code lens for top-level definitions (not features/usages nested deeply)
            if !self.should_show_code_lens(symbol) {
                continue;
            }

            if let Some(span) = symbol.span() {
                let range = span_to_lsp_range(&span);

                // Get reference count
                let qualified_name = symbol.qualified_name();
                let reference_count = self.count_references(qualified_name);

                // Only show code lens if there are references
                if reference_count > 0 {
                    // VS Code's editor.action.showReferences expects:
                    // 1. URI as a string
                    // 2. Position object
                    // 3. Array of Location objects
                    let uri_value = serde_json::Value::String(uri.to_string());
                    let Ok(position_value) = serde_json::to_value(Position {
                        line: range.start.line,
                        character: range.start.character,
                    }) else {
                        continue;
                    };
                    let Ok(locations_value) = serde_json::to_value(collect_reference_locations(
                        &self.workspace,
                        qualified_name,
                    )) else {
                        continue;
                    };

                    let lens = CodeLens {
                        range,
                        command: Some(Command {
                            title: format!(
                                "{} reference{}",
                                reference_count,
                                if reference_count == 1 { "" } else { "s" }
                            ),
                            command: "syster.showReferences".to_string(),
                            arguments: Some(vec![uri_value, position_value, locations_value]),
                        }),
                        data: None,
                    };
                    lenses.push(lens);
                }
            }
        }

        lenses
    }

    /// Determine if a symbol should show a code lens
    ///
    /// Shows code lens for all symbol types that have references.
    fn should_show_code_lens(&self, symbol: &Symbol) -> bool {
        match symbol {
            Symbol::Package { .. }
            | Symbol::Classifier { .. }
            | Symbol::Definition { .. }
            | Symbol::Feature { .. }
            | Symbol::Usage { .. }
            | Symbol::Alias { .. }
            | Symbol::Import { .. } => true,
        }
    }

    /// Count the number of references to a symbol
    fn count_references(&self, qualified_name: &str) -> usize {
        let locations = collect_reference_locations(&self.workspace, qualified_name);
        locations.len()
    }
}
