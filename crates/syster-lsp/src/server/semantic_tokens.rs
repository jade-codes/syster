use crate::server::core::LspServer;
use std::path::PathBuf;
use syster::semantic::processors::SemanticTokenCollector;
use tower_lsp::lsp_types::{
    SemanticToken as LspSemanticToken, SemanticTokenType, SemanticTokens, SemanticTokensLegend,
    SemanticTokensResult,
};

impl LspServer {
    /// Get semantic tokens for a document
    ///
    /// Thin adapter that calls the semantic layer and converts to LSP format
    pub fn get_semantic_tokens(&self, uri: &str) -> Option<SemanticTokensResult> {
        let path = PathBuf::from(uri.trim_start_matches("file://"));

        // Get workspace file and use processor to collect tokens
        let workspace_file = self.workspace.files().get(&path)?;
        let tokens = SemanticTokenCollector::collect(workspace_file);

        // Convert to LSP format with delta encoding
        let mut lsp_tokens = Vec::new();
        let mut prev_line = 0u32;
        let mut prev_start = 0u32;

        for token in tokens {
            let delta_line = token.line - prev_line;
            let delta_start = if delta_line == 0 {
                token.column - prev_start
            } else {
                token.column
            };

            lsp_tokens.push(LspSemanticToken {
                delta_line,
                delta_start,
                length: token.length,
                token_type: token.token_type as u32,
                token_modifiers_bitset: 0,
            });

            prev_line = token.line;
            prev_start = token.column;
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
