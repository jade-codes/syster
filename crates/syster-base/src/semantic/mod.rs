//! # Semantic Analysis
//!
//! This module provides semantic analysis for SysML v2 and KerML models, transforming
//! parsed ASTs into a queryable semantic model with cross-file symbol resolution.

pub mod adapters;
pub mod analyzer;
pub mod graphs;
pub mod inlay_hints;
pub mod processors;
pub mod resolver;
pub mod symbol_table;
pub mod types;
pub mod workspace;

pub use adapters::selection;
pub use adapters::{SysmlAdapter, SysmlValidator, create_validator, populate_syntax_file};
pub use analyzer::{AnalysisContext, NoOpValidator, RelationshipValidator, SemanticAnalyzer};
// Re-export folding types and extraction functions
pub use adapters::folding_ranges::{extract_kerml_folding_ranges, extract_sysml_folding_ranges};

/// Backwards-compatible module path for folding range extraction.
/// 
/// This preserves the old `syster::semantic::folding::*` API while the
/// underlying implementation lives in `adapters::folding_ranges`.
pub mod folding {
    pub use super::adapters::folding_ranges::{
        extract_kerml_folding_ranges, extract_sysml_folding_ranges,
    };
}

pub use graphs::{DependencyGraph, RelationshipGraph};
pub use inlay_hints::extract_inlay_hints;
pub use processors::ReferenceCollector;
pub use resolver::{
    Resolver, extract_imports, extract_kerml_imports, is_wildcard_import, parse_import_path,
};
pub use symbol_table::SymbolTable;
pub use types::FoldingRangeInfo;
pub use types::{
    DependencyEvent, Diagnostic, Location as DiagnosticLocation, Location, Position, Range,
    SemanticError, SemanticErrorKind, SemanticResult, SemanticRole, Severity, SymbolTableEvent,
    WorkspaceEvent,
};
pub use types::{InlayHint, InlayHintKind};
pub use workspace::{ParsedFile, Workspace};

// Type alias for the common case of Workspace<SyntaxFile>
pub type SyntaxWorkspace = Workspace<crate::syntax::SyntaxFile>;

pub type QualifiedName = String;
pub type SimpleName = String;
pub type ScopeId = usize;
pub type SourceFilePath = String;
