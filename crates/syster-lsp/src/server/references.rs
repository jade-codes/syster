use super::LspServer;
use super::helpers::extract_word_at_cursor;
use tower_lsp::lsp_types::{Location, Position, Range, Url};

impl LspServer {
    /// Find all references to a symbol at the given position
    ///
    /// Returns reference locations that were collected during semantic analysis.
    /// Optionally includes the symbol's declaration location.
    pub fn get_references(
        &self,
        uri: &Url,
        position: Position,
        include_declaration: bool,
    ) -> Option<Vec<Location>> {
        let path = uri.to_file_path().ok()?;
        let text = self.document_texts.get(&path)?;
        let (element_name, _) = self.find_symbol_at_position(&path, position)?;
        let cursor_word = extract_word_at_cursor(text, position)?;
        let lookup_name = if cursor_word != element_name {
            &cursor_word
        } else {
            &element_name
        };

        // Look up the symbol - references are already collected
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(lookup_name)
            .or_else(|| self.workspace.symbol_table().lookup(lookup_name))
            .or_else(|| {
                self.workspace
                    .symbol_table()
                    .all_symbols()
                    .into_iter()
                    .find(|(_key, sym)| sym.name() == lookup_name)
                    .map(|(_, sym)| sym)
            })?;

        // Convert references to LSP locations
        let mut locations: Vec<Location> = symbol
            .references()
            .iter()
            .filter_map(|r| {
                Url::from_file_path(&r.file).ok().map(|uri| Location {
                    uri,
                    range: Range {
                        start: Position {
                            line: r.span.start.line as u32,
                            character: r.span.start.column as u32,
                        },
                        end: Position {
                            line: r.span.end.line as u32,
                            character: r.span.end.column as u32,
                        },
                    },
                })
            })
            .collect();

        if include_declaration && let Some(def) = self.get_definition(uri, position) {
            locations.push(def);
        }

        Some(locations)
    }
}
