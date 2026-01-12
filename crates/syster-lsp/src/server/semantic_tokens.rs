use crate::server::core::LspServer;
use crate::server::helpers::{char_offset_to_utf16, uri_to_path};
use async_lsp::lsp_types::{
    SemanticToken as LspSemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensLegend,
    SemanticTokensResult, Url,
};
use syster::semantic::processors::{SemanticToken, SemanticTokenCollector};
use tracing::debug;

impl LspServer {
    /// Get semantic tokens for a document
    pub fn get_semantic_tokens(&self, uri: &Url) -> Option<SemanticTokensResult> {
        let path = uri_to_path(uri)?;
        debug!("semantic_tokens: path from URI = {:?}", path);

        let document_text = self.document_texts.get(&path);
        if document_text.is_none() {
            debug!(
                "semantic_tokens: document_text NOT FOUND for path {:?}",
                path
            );
            debug!(
                "semantic_tokens: available paths: {:?}",
                self.document_texts.keys().collect::<Vec<_>>()
            );
        }
        let document_text = document_text?;
        let lines: Vec<&str> = document_text.lines().collect();

        let path_str = path.to_string_lossy();
        debug!(
            "semantic_tokens: collecting from workspace with path_str = {}",
            path_str
        );

        let tokens = SemanticTokenCollector::collect_from_workspace(&self.workspace, &path_str);

        debug!("semantic_tokens: got {} tokens", tokens.len());

        let lsp_tokens = encode_tokens_as_deltas(&tokens, &lines);

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

/// Convert semantic tokens to LSP delta-encoded format with UTF-16 positions
fn encode_tokens_as_deltas(tokens: &[SemanticToken], lines: &[&str]) -> Vec<LspSemanticToken> {
    let mut result = Vec::with_capacity(tokens.len());
    let mut prev_line = 0u32;
    let mut prev_col_utf16 = 0u32;

    for token in tokens {
        let line_text = lines.get(token.line as usize).copied().unwrap_or("");
        let col_utf16 = char_offset_to_utf16(line_text, token.column as usize);
        let end_utf16 = char_offset_to_utf16(line_text, (token.column + token.length) as usize);
        let len_utf16 = end_utf16 - col_utf16;

        let delta_line = token.line - prev_line;
        let delta_start = if delta_line == 0 {
            col_utf16 - prev_col_utf16
        } else {
            col_utf16
        };

        result.push(LspSemanticToken {
            delta_line,
            delta_start,
            length: len_utf16,
            token_type: token.token_type as u32,
            token_modifiers_bitset: 0,
        });

        prev_line = token.line;
        prev_col_utf16 = col_utf16;
    }

    result
}
