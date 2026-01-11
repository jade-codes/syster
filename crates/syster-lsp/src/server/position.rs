use super::LspServer;
use async_lsp::lsp_types::{Position, Range};
use std::path::PathBuf;
use syster::core::Span;
use syster::semantic::symbol_table::Symbol;

impl LspServer {
    /// Find the symbol and range at the given position by querying the AST
    pub fn find_symbol_at_position(
        &self,
        path: &PathBuf,
        position: Position,
    ) -> Option<(String, Range)> {
        use super::helpers::span_to_lsp_range;

        let file_path_str = path.to_string_lossy().to_string();

        // Get document text to extract word at cursor
        let source = self.document_texts.get(path)?;

        let line = source.lines().nth(position.line as usize)?;

        // First try to extract a qualified name (e.g., "ISQ::MassValue")
        // This handles hovering on import statements like "import ISQ::MassValue"
        let word = syster::core::text_utils::extract_qualified_name_at_cursor(
            line,
            position.character as usize,
        )
        .or_else(|| {
            // Fall back to simple word extraction
            syster::core::text_utils::extract_word_at_cursor(line, position.character as usize)
        })?;

        // Calculate word range in source (for hover highlight)
        let word_start = line[..position.character as usize]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);
        let word_range = Range {
            start: Position {
                line: position.line,
                character: word_start as u32,
            },
            end: Position {
                line: position.line,
                character: (word_start + word.len()) as u32,
            },
        };

        // If the word is a qualified name (contains ::), try to resolve it.
        // First try fully qualified (e.g., "ISQ::MassValue" in import statements),
        // then try relative qualified (e.g., "Inner::Vehicle" where Inner is in scope).
        if word.contains("::") {
            let resolver = self.resolver();
            // Try fully qualified first
            if let Some(symbol) = resolver.resolve_qualified(&word) {
                let qualified_name = symbol.qualified_name().to_string();
                let range = symbol
                    .span()
                    .map(|s| span_to_lsp_range(&s))
                    .unwrap_or(word_range);
                return Some((qualified_name, range));
            }
            // Try relative qualified in file's scope (e.g., "Inner::Vehicle" where Inner is visible)
            if let Some(scope_id) = self
                .workspace
                .symbol_table()
                .get_scope_for_file(&file_path_str)
                && let Some(symbol) = resolver.resolve_in_scope(&word, scope_id)
            {
                let qualified_name = symbol.qualified_name().to_string();
                let range = symbol
                    .span()
                    .map(|s| span_to_lsp_range(&s))
                    .unwrap_or(word_range);
                return Some((qualified_name, range));
            }

            // Fallback: search for symbols in this file whose qualified name ends with the pattern.
            // This handles nested scopes where the cursor is inside a package body but
            // get_scope_for_file returns the file's root scope.
            let suffix = format!("::{}", word);
            for symbol in self.workspace.symbol_table().iter_symbols() {
                if symbol.source_file() == Some(&file_path_str)
                    && symbol.qualified_name().ends_with(&suffix)
                {
                    let qualified_name = symbol.qualified_name().to_string();
                    let range = symbol
                        .span()
                        .map(|s| span_to_lsp_range(&s))
                        .unwrap_or(word_range);
                    return Some((qualified_name, range));
                }
            }
        }

        // Check if the word matches a symbol defined in THIS file.
        // This handles hovering on definitions like "part def Engine".
        // When there are multiple symbols with the same name, prefer the one whose
        // span contains the cursor position.
        let mut exact_match: Option<(&Symbol, Span)> = None;
        let mut any_match: Option<(&Symbol, Range)> = None;

        for symbol in self.workspace.symbol_table().iter_symbols() {
            if symbol.name() == word
                && symbol.source_file() == Some(&file_path_str)
                && let Some(span) = symbol.span()
            {
                // Check if cursor is within the symbol's span (exact match)
                if position.line == span.start.line as u32
                    && position.character >= span.start.column as u32
                    && position.character <= span.end.column as u32
                {
                    exact_match = Some((symbol, span));
                }
                // Track any match in case no exact match found
                if any_match.is_none() {
                    any_match = Some((symbol, span_to_lsp_range(&span)));
                }
            }
        }

        // Prefer exact match (cursor on definition) over any match
        if let Some((symbol, span)) = exact_match {
            return Some((
                symbol.qualified_name().to_string(),
                span_to_lsp_range(&span),
            ));
        }
        if let Some((symbol, range)) = any_match {
            return Some((symbol.qualified_name().to_string(), range));
        }

        // Try to resolve using the file's scope for proper import resolution.
        // Use the Resolver with scope context for consistent resolution behavior.
        let resolver = self.resolver();
        if let Some(scope_id) = self
            .workspace
            .symbol_table()
            .get_scope_for_file(&file_path_str)
            && let Some(symbol) = resolver.resolve_in_scope(&word, scope_id)
        {
            let qualified_name = symbol.qualified_name().to_string();
            let range = symbol
                .span()
                .map(|s| span_to_lsp_range(&s))
                .unwrap_or(word_range);
            return Some((qualified_name, range));
        }

        // If resolution fails, the symbol is not in scope (e.g., import was removed).
        // Do NOT fall back to searching all symbols by simple name - that bypasses
        // import resolution and shows stale information.
        None
    }
}
