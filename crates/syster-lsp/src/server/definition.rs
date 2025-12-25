use super::LspServer;
use super::helpers::span_to_lsp_range;
use tower_lsp::lsp_types::{Location, Position, Url};

impl LspServer {
    /// Get the definition location for a symbol at the given position
    ///
    /// This method:
    /// 1. Finds the symbol at the cursor position using AST spans
    /// 2. Looks up the symbol in the symbol table
    /// 3. Returns the location where the symbol is defined
    ///
    /// If the cursor is on a type reference, this returns the definition of that type.
    /// If the cursor is on a definition itself, this returns the location of that definition.
    pub fn get_definition(&self, uri: &Url, position: Position) -> Option<Location> {
        let path = uri.to_file_path().ok()?;
        let (element_name, _hover_range) = self.find_symbol_at_position(&path, position)?;

        // Look up symbol in workspace
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(&element_name)
            .or_else(|| self.workspace.symbol_table().lookup(&element_name))?;

        // Get definition location from symbol
        let source_file = symbol.source_file()?;
        let span = symbol.span()?;

        // Convert file path to URI
        let def_uri = Url::from_file_path(source_file).ok()?;

        Some(Location {
            uri: def_uri,
            range: span_to_lsp_range(&span),
        })
    }
}
