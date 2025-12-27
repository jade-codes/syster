use crate::server::core::LspServer;
use crate::server::helpers::{char_offset_to_byte, char_offset_to_utf16};
use async_lsp::lsp_types::{
    SemanticToken as LspSemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensLegend,
    SemanticTokensResult,
};
use syster::semantic::processors::SemanticTokenCollector;

impl LspServer {
    /// Get semantic tokens for a document
    ///
    /// Thin adapter that calls the semantic layer and converts to LSP format
    pub fn get_semantic_tokens(&self, uri: &str) -> Option<SemanticTokensResult> {
        // Parse URI and convert to file path (handles URL decoding)
        let uri_parsed = async_lsp::lsp_types::Url::parse(uri).ok()?;
        let path = uri_parsed.to_file_path().ok()?;

        // Get the file path as a string for symbol table lookup
        let file_path_str = path.to_string_lossy();

        // Get the document text for UTF-16 conversion
        let document_text = self.document_texts.get(&path)?;
        let lines: Vec<&str> = document_text.lines().collect();

        // Collect tokens from the workspace (includes type references from AST)
        let tokens =
            SemanticTokenCollector::collect_from_workspace(&self.workspace, &file_path_str);

        // Convert to LSP format (delta encoding with UTF-16 positions)
        let mut lsp_tokens = Vec::new();
        let mut prev_line = 0;
        let mut prev_start = 0;

        for token in tokens.iter() {
            // Get the line text for conversion
            let line_text = lines.get(token.line as usize).unwrap_or(&"");

            // Columns and lengths in our spans are CHARACTER offsets/counts (from Pest)
            // We need to convert both to UTF-16

            // Convert character offset to UTF-16 code units
            let utf16_column = char_offset_to_utf16(line_text, token.column as usize);

            // Convert character offset to byte offset for text extraction
            let start_byte = char_offset_to_byte(line_text, token.column as usize);

            // Calculate end byte using character count
            let end_char = token.column as usize + token.length as usize;
            let end_byte = char_offset_to_byte(line_text, end_char);

            // Extract token text for debugging
            let token_text = if end_byte <= line_text.len() {
                &line_text[start_byte..end_byte]
            } else {
                ""
            };

            // Get UTF-16 length of the token text
            let utf16_length = token_text.chars().map(|c| c.len_utf16()).sum::<usize>() as u32;

            let delta_line = token.line - prev_line;
            let delta_start = if delta_line == 0 {
                utf16_column - prev_start
            } else {
                utf16_column
            };

            let lsp_token = LspSemanticToken {
                delta_line,
                delta_start,
                length: utf16_length,
                token_type: token.token_type as u32,
                token_modifiers_bitset: 0,
            };

            lsp_tokens.push(lsp_token);

            prev_line = token.line;
            prev_start = utf16_column;
        }

        Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: lsp_tokens,
        }))
    }

    /// Get the semantic tokens legend (token types supported)
    pub fn semantic_tokens_legend() -> SemanticTokensLegend {
        SemanticTokensLegend {
            token_types: vec![
                SemanticTokenType::NAMESPACE,
                SemanticTokenType::TYPE,
                SemanticTokenType::VARIABLE,
                SemanticTokenType::PROPERTY,
                SemanticTokenType::KEYWORD,
            ],
            token_modifiers: vec![],
        }
    }
}
