use crate::core::Span;
use crate::core::constants::{
    PROPERTY_REFERENCE_RELATIONSHIPS, REL_TYPING, TYPE_REFERENCE_RELATIONSHIPS,
};
use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::workspace::Workspace;
use crate::syntax::SyntaxFile;
use std::path::{Path, PathBuf};

/// Normalize a file path by:
/// 1. For stdlib files (containing "sysml.library/"), extract the relative path within sysml.library
/// 2. For other files, use canonical path comparison
///
/// This handles the case where stdlib files exist in multiple locations:
/// - Source: /workspaces/syster/crates/syster-base/sysml.library/...
/// - Build: /workspaces/syster/target/release/sysml.library/...
fn normalize_path(path: &str) -> String {
    // Check if this is a stdlib file
    if let Some(idx) = path.find("sysml.library/") {
        // Extract the path relative to sysml.library/
        let relative_path = &path[idx..];
        return relative_path.to_string();
    }

    // For non-stdlib files, try to canonicalize (resolves symlinks and makes absolute)
    if let Ok(canonical) = Path::new(path).canonicalize() {
        return canonical.to_string_lossy().to_string();
    }

    // If canonicalization fails (file doesn't exist yet), do simple normalization
    let path_buf = PathBuf::from(path);
    let normalized = if path_buf.is_absolute() {
        path_buf
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join(path_buf)
    };

    normalized.to_string_lossy().to_string()
}

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
        // Span columns are character offsets (from Pest)
        let char_length = if span.start.line == span.end.line {
            span.end.column.saturating_sub(span.start.column)
        } else {
            // Multi-line spans: just use a reasonable default
            // (semantic tokens are typically single-line)
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

/// Token types for semantic highlighting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Namespace = 0,
    Type = 1,
    Variable = 2,
    Property = 3,
    Keyword = 4,
}

/// Collects semantic tokens from a symbol table
pub struct SemanticTokenCollector;

impl SemanticTokenCollector {
    /// Collect semantic tokens from a symbol table for a specific file
    pub fn collect_from_symbols(symbol_table: &SymbolTable, file_path: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();

        // Normalize the requested file path for comparison
        let normalized_path = normalize_path(file_path);

        // Iterate through all symbols in the table
        for (_name, symbol) in symbol_table.all_symbols() {
            // Only include symbols from this file
            if let Some(source_file) = symbol.source_file() {
                let normalized_source = normalize_path(source_file);
                if normalized_source != normalized_path {
                    continue;
                }
            } else {
                continue;
            }

            // Only add tokens for symbols with spans
            if let Some(span) = symbol.span() {
                let token_type = Self::map_symbol_to_token_type(symbol);
                tokens.push(SemanticToken::from_span(&span, token_type));
            }
        }

        // Sort tokens by position (line, then column)
        tokens.sort_by_key(|t| (t.line, t.column));

        tokens
    }

    /// Collect semantic tokens from workspace (includes type references from relationship graph)
    pub fn collect_from_workspace(
        workspace: &Workspace<SyntaxFile>,
        file_path: &str,
    ) -> Vec<SemanticToken> {
        let mut tokens = Self::collect_from_symbols(workspace.symbol_table(), file_path);

        // Add type reference tokens from relationship graph
        let type_tokens = Self::extract_type_references_from_graph(
            workspace.symbol_table(),
            workspace.relationship_graph(),
            file_path,
        );
        tokens.extend(type_tokens);
        tokens.sort_by_key(|t| (t.line, t.column));

        tokens
    }

    /// Extract type reference tokens from the relationship graph.
    /// This replaces AST traversal with semantic data lookup.
    fn extract_type_references_from_graph(
        symbol_table: &SymbolTable,
        relationship_graph: &RelationshipGraph,
        file_path: &str,
    ) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();
        let normalized_path = normalize_path(file_path);

        // For each symbol in this file, check if it has relationships with spans
        for (_name, symbol) in symbol_table.all_symbols() {
            // Only check symbols from this file
            if let Some(source_file) = symbol.source_file() {
                if normalize_path(source_file) != normalized_path {
                    continue;
                }
            } else {
                continue;
            }

            let qname = symbol.qualified_name();

            // Typing (one-to-one relationship) → Type token
            if let Some((_target, Some(span))) =
                relationship_graph.get_one_to_one_with_span(REL_TYPING, qname)
            {
                tokens.push(SemanticToken::from_span(span, TokenType::Type));
            }

            // Type reference relationships (specialization, satisfy, etc.) → Type tokens
            for rel_type in TYPE_REFERENCE_RELATIONSHIPS {
                if let Some(targets_with_spans) =
                    relationship_graph.get_one_to_many_with_spans(rel_type, qname)
                {
                    for (_target, span_opt) in targets_with_spans {
                        if let Some(span) = span_opt {
                            tokens.push(SemanticToken::from_span(span, TokenType::Type));
                        }
                    }
                }
            }

            // Property reference relationships (redefinition, subsetting, etc.) → Property tokens
            for rel_type in PROPERTY_REFERENCE_RELATIONSHIPS {
                if let Some(targets_with_spans) =
                    relationship_graph.get_one_to_many_with_spans(rel_type, qname)
                {
                    for (_target, span_opt) in targets_with_spans {
                        if let Some(span) = span_opt {
                            tokens.push(SemanticToken::from_span(span, TokenType::Property));
                        }
                    }
                }
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
