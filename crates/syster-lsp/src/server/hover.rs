use super::LspServer;
use super::helpers::format_rich_hover;
use tower_lsp::lsp_types::{Hover, HoverContents, MarkedString, Position, Url};

impl LspServer {
    /// Get hover information for a symbol at the given position
    ///
    /// Uses AST span tracking to find the exact element under the cursor,
    /// then provides rich information including relationships and documentation.
    pub fn get_hover(&self, uri: &Url, position: Position) -> Option<Hover> {
        let path = uri.to_file_path().ok()?;

        // Find symbol at position using AST spans
        let (symbol_name, hover_range) = self.find_symbol_at_position(&path, position)?;

        // Look up symbol in workspace (try qualified name first, then simple name)
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(&symbol_name)
            .or_else(|| self.workspace.symbol_table().lookup(&symbol_name))?;

        // Format rich hover content with relationships
        let content = format_rich_hover(symbol, self.workspace());

        Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(content)),
            range: Some(hover_range),
        })
    }
}
