use crate::core::Span;
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

    /// Collect semantic tokens from workspace (includes type references from AST)
    pub fn collect_from_workspace(
        workspace: &Workspace<SyntaxFile>,
        file_path: &str,
    ) -> Vec<SemanticToken> {
        let mut tokens = Self::collect_from_symbols(workspace.symbol_table(), file_path);

        // Add type reference tokens by extracting from syntax tree
        if let Some(workspace_file) = workspace.files().get(Path::new(file_path)) {
            let type_tokens = Self::extract_type_references(workspace_file.content(), file_path);
            tokens.extend(type_tokens);
            tokens.sort_by_key(|t| (t.line, t.column));
        }

        tokens
    }

    /// Extract type references from syntax tree (for typing relationships like `: String`)
    fn extract_type_references(syntax_file: &SyntaxFile, _file_path: &str) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();

        match syntax_file {
            SyntaxFile::SysML(file) => {
                for element in &file.elements {
                    Self::extract_type_refs_from_sysml_element(element, &mut tokens);
                }
            }
            SyntaxFile::KerML(file) => {
                for element in &file.elements {
                    Self::extract_type_refs_from_kerml_element(element, &mut tokens);
                }
            }
        }

        tokens
    }

    fn extract_type_refs_from_sysml_element(
        element: &crate::syntax::sysml::ast::enums::Element,
        tokens: &mut Vec<SemanticToken>,
    ) {
        use crate::syntax::sysml::ast::enums::Element;

        match element {
            Element::Package(pkg) => {
                for elem in &pkg.elements {
                    Self::extract_type_refs_from_sysml_element(elem, tokens);
                }
            }
            Element::Import(import) => {
                // Highlight the imported path (e.g., "ScalarValues::Real")
                if let Some(span) = &import.span {
                    tokens.push(SemanticToken::from_span(span, TokenType::Namespace));
                }
            }
            Element::Definition(def) => {
                // Check if this definition has a typed_by relationship with a span
                if let (Some(_type_name), Some(span)) = (
                    &def.relationships.typed_by,
                    &def.relationships.typed_by_span,
                ) {
                    tokens.push(SemanticToken::from_span(span, TokenType::Type));
                }
                // Recursively check body members
                for member in &def.body {
                    Self::extract_type_refs_from_def_member(member, tokens);
                }
            }
            Element::Usage(usage) => {
                if let (Some(_type_name), Some(span)) = (
                    &usage.relationships.typed_by,
                    &usage.relationships.typed_by_span,
                ) {
                    tokens.push(SemanticToken::from_span(span, TokenType::Type));
                }
            }
            _ => {}
        }
    }

    fn extract_type_refs_from_def_member(
        member: &crate::syntax::sysml::ast::enums::DefinitionMember,
        tokens: &mut Vec<SemanticToken>,
    ) {
        use crate::syntax::sysml::ast::enums::DefinitionMember;

        if let DefinitionMember::Usage(usage) = member {
            if let (Some(_type_name), Some(span)) = (
                &usage.relationships.typed_by,
                &usage.relationships.typed_by_span,
            ) {
                tokens.push(SemanticToken::from_span(span, TokenType::Type));
            }
            // Recursively check nested usage body
            for nested in &usage.body {
                Self::extract_type_refs_from_usage_member(nested, tokens);
            }
        }
    }

    fn extract_type_refs_from_usage_member(
        member: &crate::syntax::sysml::ast::enums::UsageMember,
        _tokens: &mut [SemanticToken],
    ) {
        use crate::syntax::sysml::ast::enums::UsageMember;

        match member {
            UsageMember::Comment(_) => {
                // Comments don't have type references
            }
            UsageMember::Usage(_) => {
                // Nested usages are handled by the main visitor
            }
        }
    }

    fn extract_type_refs_from_kerml_element(
        element: &crate::syntax::kerml::ast::enums::Element,
        tokens: &mut Vec<SemanticToken>,
    ) {
        use crate::syntax::kerml::ast::enums::Element;

        match element {
            Element::Import(import) => {
                // Highlight the imported path
                if let Some(span) = &import.span {
                    tokens.push(SemanticToken::from_span(span, TokenType::Namespace));
                }
            }
            Element::Classifier(classifier) => {
                // Recursively check body members
                for member in &classifier.body {
                    Self::extract_type_refs_from_classifier_member(member, tokens);
                }
            }
            Element::Feature(feature) => {
                // Check feature body for typing relationships
                for member in &feature.body {
                    Self::extract_type_refs_from_feature_member(member, tokens);
                }
            }
            _ => {}
        }
    }

    fn extract_type_refs_from_classifier_member(
        member: &crate::syntax::kerml::ast::enums::ClassifierMember,
        tokens: &mut Vec<SemanticToken>,
    ) {
        use crate::syntax::kerml::ast::enums::ClassifierMember;

        if let ClassifierMember::Feature(feature) = member {
            // Check feature body for typing relationships
            for nested in &feature.body {
                Self::extract_type_refs_from_feature_member(nested, tokens);
            }
        }
    }

    fn extract_type_refs_from_feature_member(
        member: &crate::syntax::kerml::ast::enums::FeatureMember,
        tokens: &mut Vec<SemanticToken>,
    ) {
        use crate::syntax::kerml::ast::enums::FeatureMember;

        match member {
            FeatureMember::Typing(typing) => {
                // Extract the type reference span from the typing relationship
                if let Some(span) = &typing.span {
                    tokens.push(SemanticToken::from_span(span, TokenType::Type));
                }
            }
            FeatureMember::Comment(_) => {}
            FeatureMember::Subsetting(_) => {}
            FeatureMember::Redefinition(_) => {}
        }
    }

    /// Map a Symbol to its corresponding TokenType
    fn map_symbol_to_token_type(symbol: &Symbol) -> TokenType {
        match symbol {
            Symbol::Package { .. } => TokenType::Namespace,
            Symbol::Classifier { .. } => TokenType::Type,
            Symbol::Usage { .. } | Symbol::Feature { .. } => TokenType::Property,
            Symbol::Definition { .. } => TokenType::Type,
            Symbol::Alias { .. } => TokenType::Variable,
        }
    }
}
