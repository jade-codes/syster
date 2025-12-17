use crate::core::Span;
use crate::language::sysml::syntax::Element;
use crate::semantic::workspace::WorkspaceFile;

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
    /// Create a semantic token from a span, name, and token type
    fn from_span(span: &Span, name: &str, token_type: TokenType) -> Self {
        Self {
            line: span.start.line as u32,
            column: span.start.column as u32,
            length: name.len() as u32,
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

/// Collects semantic tokens from a parsed file
pub struct SemanticTokenCollector;

impl SemanticTokenCollector {
    /// Collect semantic tokens from a workspace file
    pub fn collect(workspace_file: &WorkspaceFile) -> Vec<SemanticToken> {
        let mut tokens = Vec::new();

        // Only process SysML files for now
        if let crate::language::LanguageFile::SysML(sysml_file) = workspace_file.content() {
            for element in &sysml_file.elements {
                Self::collect_element_tokens(element, &mut tokens);
            }
        }

        tokens
    }

    /// Recursively collect tokens from an Element and its children
    fn collect_element_tokens(element: &Element, tokens: &mut Vec<SemanticToken>) {
        match element {
            Element::Package(pkg) => {
                Self::add_token_if_present(&pkg.name, &pkg.span, TokenType::Namespace, tokens);
                // Recurse into nested package elements
                for elem in &pkg.elements {
                    Self::collect_element_tokens(elem, tokens);
                }
            }
            Element::Definition(def) => {
                Self::add_token_if_present(&def.name, &def.span, TokenType::Type, tokens);
            }
            Element::Usage(usage) => {
                Self::add_token_if_present(&usage.name, &usage.span, TokenType::Property, tokens);
            }
            Element::Alias(alias) => {
                Self::add_token_if_present(&alias.name, &alias.span, TokenType::Variable, tokens);
            }
            Element::Comment(_) | Element::Import(_) => {
                // No tokens to extract from these elements
            }
        }
    }

    /// Helper to add a token if both name and span are present
    fn add_token_if_present(
        name: &Option<String>,
        span: &Option<Span>,
        token_type: TokenType,
        tokens: &mut Vec<SemanticToken>,
    ) {
        if let (Some(name), Some(span)) = (name, span) {
            tokens.push(SemanticToken::from_span(span, name, token_type));
        }
    }
}
