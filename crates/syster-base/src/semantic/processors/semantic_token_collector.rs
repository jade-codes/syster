use crate::core::Span;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::TokenType;
use crate::semantic::workspace::Workspace;
use crate::syntax::SyntaxFile;

/// Represents a semantic token with its position and type
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticToken {
    /// Line number (0-indexed)
    pub line: u32,
    /// Column number (0-indexed)
    pub column: u32,
    /// Length of the token
    pub length: u32,
    /// Token type (corresponds to LSP SemanticTokenType)
    pub token_type: TokenType,
}

impl SemanticToken {
    /// Create a semantic token from a span and token type
    fn from_span(span: &Span, token_type: TokenType) -> Self {
        // Calculate the character length from the span
        let char_length = if span.start.line == span.end.line {
            span.end.column.saturating_sub(span.start.column)
        } else {
            // Multi-line spans: just use a reasonable default
            1
        };

        Self {
            line: span.start.line as u32,
            column: span.start.column as u32,
            length: char_length as u32,
            token_type,
        }
    }
}

/// Collects semantic tokens from a symbol table
pub struct SemanticTokenCollector;

impl SemanticTokenCollector {
    /// Collect semantic tokens from a symbol table for a specific file
    pub fn collect_from_symbols(symbol_table: &SymbolTable, file_path: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();

        // Use indexed lookup instead of iterating all symbols
        for symbol in symbol_table.get_symbols_for_file(file_path) {
            // Only add tokens for symbols with spans
            if let Some(span) = symbol.span() {
                let token_type = Self::map_symbol_to_token_type(symbol);
                tokens.push(SemanticToken::from_span(&span, token_type));
            }

            // Handle alias targets (the "for X" part of "alias Y for X")
            if let Symbol::Alias {
                target_span: Some(span),
                ..
            } = symbol
            {
                tokens.push(SemanticToken::from_span(span, TokenType::Type));
            }

            // Handle import paths (the path in "import X::Y::*")
            if let Symbol::Import {
                path_span: Some(span),
                ..
            } = symbol
            {
                tokens.push(SemanticToken::from_span(span, TokenType::Namespace));
            }
        }

        // Sort tokens by position (line, then column)
        tokens.sort_by_key(|t| (t.line, t.column));

        tokens
    }

    /// Collect semantic tokens from workspace (uses symbol table AND reference index)
    pub fn collect_from_workspace(
        workspace: &Workspace<SyntaxFile>,
        file_path: &str,
    ) -> Vec<SemanticToken> {
        let mut tokens = Self::collect_from_symbols(workspace.symbol_table(), file_path);

        // Track positions that already have tokens from symbols
        // (symbol tokens take precedence over reference tokens)
        let existing_positions: std::collections::HashSet<(u32, u32)> =
            tokens.iter().map(|t| (t.line, t.column)).collect();

        // Also collect type references from the reference index
        // Skip if there's already a token at this position (from the symbol table)
        for ref_info in workspace
            .reference_index()
            .get_references_in_file(file_path)
        {
            // Use the token type from the reference, or default to Type
            let token_type = ref_info.token_type.unwrap_or(TokenType::Type);
            let token = SemanticToken::from_span(&ref_info.span, token_type);
            if !existing_positions.contains(&(token.line, token.column)) {
                tokens.push(token);
            }
        }

        // Sort tokens by position (line, then column)
        tokens.sort_by_key(|t| (t.line, t.column));

        tokens
    }

    /// Map a Symbol to its corresponding TokenType
    fn map_symbol_to_token_type(symbol: &Symbol) -> TokenType {
        match symbol {
            Symbol::Package { .. } => TokenType::Namespace,
            Symbol::Classifier { .. } => TokenType::Type,
            Symbol::Usage { .. } | Symbol::Feature { .. } => TokenType::Property,
            Symbol::Definition { .. } => TokenType::Type,
            Symbol::Alias { .. } => TokenType::Variable,
            Symbol::Import { .. } => TokenType::Namespace,
        }
    }
}
