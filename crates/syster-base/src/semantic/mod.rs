//! # Semantic Analysis
//!
//! This module provides semantic analysis for SysML v2 and KerML models, transforming
//! parsed ASTs into a queryable semantic model with cross-file symbol resolution.
pub mod adapters;
pub mod analyzer;
pub mod graphs;
pub mod processors;
pub mod resolver;
pub mod symbol_table;
pub mod types;
pub mod workspace;

pub use adapters::{SysmlAdapter, SysmlValidator, create_validator, populate_syntax_file};
pub use analyzer::{AnalysisContext, NoOpValidator, RelationshipValidator, SemanticAnalyzer};
pub use graphs::{DependencyGraph, RelationshipGraph};
pub use processors::ReferenceCollector;
pub use resolver::{
    Resolver, extract_imports, extract_kerml_imports, is_wildcard_import, parse_import_path,
};
pub use symbol_table::SymbolTable;
pub use types::{
    DependencyEvent, Diagnostic, Location as DiagnosticLocation, Location, Position, Range,
    SemanticError, SemanticErrorKind, SemanticResult, SemanticRole, Severity, SymbolTableEvent,
    WorkspaceEvent,
};
pub use workspace::{ParsedFile, Workspace};

// Type alias for the common case of Workspace<SyntaxFile>
pub type SyntaxWorkspace = Workspace<crate::syntax::SyntaxFile>;

pub type QualifiedName = String;
pub type SimpleName = String;
pub type ScopeId = usize;
pub type SourceFilePath = String;
