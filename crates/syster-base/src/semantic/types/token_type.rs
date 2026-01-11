//! Semantic token types for syntax highlighting.
//!
//! These correspond to LSP SemanticTokenTypes and are used by both
//! the reference index (to tag references with their token type) and
//! the semantic token collector (to generate tokens for the LSP).

/// Token types for semantic highlighting.
///
/// Values correspond to LSP SemanticTokenType indices.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum TokenType {
    Namespace = 0,
    #[default]
    Type = 1,
    Variable = 2,
    Property = 3,
    Keyword = 4,
}
