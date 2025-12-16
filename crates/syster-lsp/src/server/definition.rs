use super::LspServer;
use super::helpers::{extract_word_at_cursor, span_to_lsp_range};
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

        // Get document text to extract word at position
        let text = self.document_texts.get(&path)?;

        // Find symbol at position using AST spans (this gives us the containing element)
        let (element_name, _hover_range) = self.find_symbol_at_position(&path, position)?;

        // Extract the actual word under the cursor - this might be different from element_name
        // if the cursor is on a type reference (e.g., ": Car" in "part myCar : Car")
        let cursor_word = extract_word_at_cursor(text, position)?;

        // Try to look up the word under cursor first (handles type references)
        // Then fall back to the element name (handles hovering on the element itself)
        let lookup_name = if cursor_word != element_name {
            // Cursor is on something other than the element name (likely a type reference)
            &cursor_word
        } else {
            // Cursor is on the element itself
            &element_name
        };

        // Look up symbol in workspace
        // Try qualified lookup first, then simple name lookup, then search all symbols
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(lookup_name)
            .or_else(|| self.workspace.symbol_table().lookup(lookup_name))
            .or_else(|| {
                // Fallback: search all symbols for matching simple name
                self.workspace
                    .symbol_table()
                    .all_symbols()
                    .into_iter()
                    .find(|(_key, sym)| sym.name() == lookup_name)
                    .map(|(_, sym)| sym)
            })?;

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
