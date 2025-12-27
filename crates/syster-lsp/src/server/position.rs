use super::LspServer;
use async_lsp::lsp_types::{Position, Range};
use std::path::PathBuf;

impl LspServer {
    /// Find the symbol and range at the given position by querying the AST
    pub fn find_symbol_at_position(
        &self,
        path: &PathBuf,
        position: Position,
    ) -> Option<(String, Range)> {
        use super::helpers::span_to_lsp_range;

        // Get document text to extract word at cursor
        let source = self.document_texts.get(path)?;

        let line = source.lines().nth(position.line as usize)?;

        let word =
            syster::core::text_utils::extract_word_at_cursor(line, position.character as usize)?;

        // Try resolver first (works for qualified names and scope-aware lookups)
        let resolver = self.resolver();
        if let Some(symbol) = resolver.resolve(&word) {
            let qualified_name = symbol.qualified_name().to_string();
            let span = symbol.span()?;
            return Some((qualified_name, span_to_lsp_range(&span)));
        }

        // Fallback: search all symbols by simple name (for cross-scope references)
        for (_key, symbol) in self.workspace.symbol_table().all_symbols() {
            if symbol.name() == word {
                let qualified_name = symbol.qualified_name().to_string();
                if let Some(span) = symbol.span() {
                    return Some((qualified_name, span_to_lsp_range(&span)));
                }
            }
        }
        None
    }
}
