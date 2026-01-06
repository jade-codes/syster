use super::LspServer;
use super::helpers::{span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{Location, Position, Range, Url};

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
        let path = uri_to_path(uri)?;
        let (element_name, _hover_range) = self.find_symbol_at_position(&path, position)?;

        // Look up symbol using resolver
        let resolver = self.resolver();
        let symbol = resolver.resolve(&element_name)?;

        // Get definition location from symbol
        let source_file = symbol.source_file()?;

        // Convert file path to URI
        let def_uri = Url::from_file_path(source_file).ok()?;

        // Use symbol's span if available, otherwise default to start of file
        let range = symbol
            .span()
            .map(|s| span_to_lsp_range(&s))
            .unwrap_or(Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            });

        Some(Location {
            uri: def_uri,
            range,
        })
    }
}
