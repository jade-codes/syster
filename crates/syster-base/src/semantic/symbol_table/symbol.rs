use crate::core::Span;
use crate::semantic::types::SemanticRole;

/// A reference to a symbol from another location in the code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolReference {
    pub file: String,
    pub span: Span,
}

/// Represents a named element in a SysML/KerML model
#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Package {
        name: String,
        qualified_name: String,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<SymbolReference>,
    },
    Classifier {
        name: String,
        qualified_name: String,
        kind: String,
        is_abstract: bool,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<SymbolReference>,
    },
    Feature {
        name: String,
        qualified_name: String,
        scope_id: usize,
        feature_type: Option<String>,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<SymbolReference>,
    },
    Definition {
        name: String,
        qualified_name: String,
        kind: String,
        semantic_role: Option<SemanticRole>,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<SymbolReference>,
    },
    Usage {
        name: String,
        qualified_name: String,
        kind: String,
        semantic_role: Option<SemanticRole>,
        usage_type: Option<String>,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<SymbolReference>,
    },
    Alias {
        name: String,
        qualified_name: String,
        target: String,
        target_span: Option<Span>,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<SymbolReference>,
    },
    /// An import statement (e.g., `import ScalarValues::*`)
    Import {
        /// The import path (e.g., "ScalarValues::*")
        path: String,
        /// Span of the import path for semantic highlighting
        path_span: Option<Span>,
        /// Unique key for this import (path + scope)
        qualified_name: String,
        is_recursive: bool,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
    },
}

impl Symbol {
    /// Returns the qualified name of this symbol
    pub fn qualified_name(&self) -> &str {
        match self {
            Symbol::Package { qualified_name, .. }
            | Symbol::Classifier { qualified_name, .. }
            | Symbol::Feature { qualified_name, .. }
            | Symbol::Definition { qualified_name, .. }
            | Symbol::Usage { qualified_name, .. }
            | Symbol::Alias { qualified_name, .. }
            | Symbol::Import { qualified_name, .. } => qualified_name,
        }
    }

    /// Returns the simple name of this symbol
    pub fn name(&self) -> &str {
        match self {
            Symbol::Package { name, .. }
            | Symbol::Classifier { name, .. }
            | Symbol::Feature { name, .. }
            | Symbol::Definition { name, .. }
            | Symbol::Usage { name, .. }
            | Symbol::Alias { name, .. } => name,
            Symbol::Import { path, .. } => path,
        }
    }

    /// Returns the scope ID where this symbol was defined
    pub fn scope_id(&self) -> usize {
        match self {
            Symbol::Package { scope_id, .. }
            | Symbol::Classifier { scope_id, .. }
            | Symbol::Feature { scope_id, .. }
            | Symbol::Definition { scope_id, .. }
            | Symbol::Usage { scope_id, .. }
            | Symbol::Alias { scope_id, .. }
            | Symbol::Import { scope_id, .. } => *scope_id,
        }
    }

    /// Returns the source file path where this symbol was defined
    pub fn source_file(&self) -> Option<&str> {
        match self {
            Symbol::Package { source_file, .. }
            | Symbol::Classifier { source_file, .. }
            | Symbol::Feature { source_file, .. }
            | Symbol::Definition { source_file, .. }
            | Symbol::Usage { source_file, .. }
            | Symbol::Alias { source_file, .. }
            | Symbol::Import { source_file, .. } => source_file.as_deref(),
        }
    }

    /// Returns the source span where this symbol was defined
    pub fn span(&self) -> Option<Span> {
        match self {
            Symbol::Package { span, .. }
            | Symbol::Classifier { span, .. }
            | Symbol::Feature { span, .. }
            | Symbol::Definition { span, .. }
            | Symbol::Usage { span, .. }
            | Symbol::Alias { span, .. }
            | Symbol::Import { span, .. } => *span,
        }
    }

    /// Returns true if this symbol can be used as a type
    pub fn is_type(&self) -> bool {
        matches!(self, Symbol::Classifier { .. } | Symbol::Definition { .. })
    }

    /// Returns the type reference for Features that have one
    pub fn type_reference(&self) -> Option<&str> {
        match self {
            Symbol::Feature { feature_type, .. } => feature_type.as_deref(),
            _ => None,
        }
    }

    /// Returns all reference locations for this symbol
    pub fn references(&self) -> &[SymbolReference] {
        match self {
            Symbol::Package { references, .. }
            | Symbol::Classifier { references, .. }
            | Symbol::Feature { references, .. }
            | Symbol::Definition { references, .. }
            | Symbol::Usage { references, .. }
            | Symbol::Alias { references, .. } => references,
            Symbol::Import { .. } => &[],
        }
    }

    /// Adds a reference location to this symbol (mutable access required)
    pub fn add_reference(&mut self, reference: SymbolReference) {
        match self {
            Symbol::Package { references, .. }
            | Symbol::Classifier { references, .. }
            | Symbol::Feature { references, .. }
            | Symbol::Definition { references, .. }
            | Symbol::Usage { references, .. }
            | Symbol::Alias { references, .. } => references.push(reference),
            Symbol::Import { .. } => {} // Imports don't track references
        }
    }
}
