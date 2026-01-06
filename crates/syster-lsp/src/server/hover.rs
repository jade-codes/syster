use super::LspServer;
use super::helpers::{format_rich_hover, uri_to_path};
use async_lsp::lsp_types::{Hover, HoverContents, MarkedString, Position, Url};

impl LspServer {
    /// Get hover information for a symbol at the given position
    ///
    /// Uses AST span tracking to find the exact element under the cursor,
    /// then provides rich information including relationships and documentation.
    pub fn get_hover(&self, uri: &Url, position: Position) -> Option<Hover> {
        let path = uri_to_path(uri)?;

        // Find symbol at position - returns qualified name string
        let (qualified_name, hover_range) = self.find_symbol_at_position(&path, position)?;

        // Look up the symbol using the resolver
        let resolver = self.resolver();
        let symbol = resolver.resolve(&qualified_name)?;

        // Format rich hover content with relationships
        let content = format_rich_hover(symbol, self.workspace());

        Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(content)),
            range: Some(hover_range),
        })
    }
}
