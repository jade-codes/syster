use super::LspServer;
use super::helpers::{collect_reference_locations, uri_to_path};
use async_lsp::lsp_types::{Location, Position, Url};

impl LspServer {
    /// Find all references to a symbol at the given position
    ///
    /// Queries the ReferenceIndex directly for references instead of
    /// pre-computing them on every document change. This provides O(1) lookup
    /// instead of O(n) on every keystroke.
    /// Optionally includes the symbol's declaration location.
    pub fn get_references(
        &self,
        uri: &Url,
        position: Position,
        include_declaration: bool,
    ) -> Option<Vec<Location>> {
        let path = uri_to_path(uri)?;

        // Find the symbol at this position using AST
        let (element_qname, _) = self.find_symbol_at_position(&path, position)?;

        // Collect all references using shared helper
        let mut locations = collect_reference_locations(&self.workspace, &element_qname);

        if include_declaration && let Some(def) = self.get_definition(uri, position) {
            locations.push(def);
        }

        Some(locations)
    }
}
