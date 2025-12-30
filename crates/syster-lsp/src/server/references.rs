use super::LspServer;
use super::helpers::span_to_lsp_range;
use async_lsp::lsp_types::{Location, Position, Url};

impl LspServer {
    /// Find all references to a symbol at the given position
    ///
    /// Queries the RelationshipGraph directly for references instead of
    /// pre-computing them on every document change. This provides O(1) lookup
    /// instead of O(n) on every keystroke.
    /// Optionally includes the symbol's declaration location.
    pub fn get_references(
        &self,
        uri: &Url,
        position: Position,
        include_declaration: bool,
    ) -> Option<Vec<Location>> {
        let path = uri.to_file_path().ok()?;

        // Find the symbol at this position using AST
        let (element_qname, _) = self.find_symbol_at_position(&path, position)?;

        // Query the relationship graph for all sources that reference this target
        let refs = self
            .workspace
            .relationship_graph()
            .get_references_to(&element_qname);

        // Convert relationship references to LSP locations by looking up source symbols
        let mut locations: Vec<Location> = refs
            .into_iter()
            .filter_map(|(source_qname, span)| {
                // Only use span from relationship graph; skip imprecise references
                let reference_span = span?;

                // Look up the source symbol to get its file path
                let source_symbol = self
                    .workspace
                    .symbol_table()
                    .lookup_qualified(source_qname)?;
                let file = source_symbol.source_file()?;

                Url::from_file_path(file).ok().map(|uri| Location {
                    uri,
                    range: span_to_lsp_range(reference_span),
                })
            })
            .collect();

        // Add import references (where this symbol is imported)
        let import_refs = self
            .workspace
            .symbol_table()
            .get_import_references(&element_qname);

        for (file, span) in import_refs {
            if let Ok(uri) = Url::from_file_path(file) {
                locations.push(Location {
                    uri,
                    range: span_to_lsp_range(span),
                });
            }
        }

        if include_declaration && let Some(def) = self.get_definition(uri, position) {
            locations.push(def);
        }

        Some(locations)
    }
}
