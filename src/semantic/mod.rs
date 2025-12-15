//! # Semantic Analysis
//!
//! This module provides semantic analysis for SysML v2 and KerML models, transforming
//! parsed ASTs into a queryable semantic model with cross-file symbol resolution.
//!
//! ## Architecture
//!
//! The semantic analysis phase consists of several interconnected components:
//!
//! - **Symbol Table** (`symbol_table`): Global registry of all named elements
//! - **Relationship Graphs** (`graph`): Tracks specialization, typing, subsetting, etc.
//! - **Name Resolver** (`resolver`): Resolves qualified names to symbols
//! - **Workspace** (`workspace`): Manages multi-file projects
//! - **Analyzer** (`analyzer`): Orchestrates validation passes
//!
//! ## Pipeline
//!
//! ```text
//! Parsed ASTs → Symbol Table Population → Import Resolution →
//! Relationship Building → Semantic Validation → Complete Model
//! ```
//!
//! ## Usage Example
//!
//! ```rust
//! use syster::semantic::Workspace;
//!
//! // Create workspace and add files
//! let mut workspace = Workspace::new();
//! workspace.add_file(path, parsed_content);
//!
//! // Populate symbol table and resolve imports
//! workspace.populate_all().unwrap();
//!
//! // Query the model
//! let symbol = workspace.symbol_table().lookup("Package::Element");
//! ```
//!
//! See [SEMANTIC_ANALYSIS.md](../../docs/SEMANTIC_ANALYSIS.md) for detailed documentation.

pub mod analyzer;
pub mod diagnostic;
pub mod error;
pub mod graph;
pub mod resolver;
pub mod symbol_table;
pub mod workspace;

pub use analyzer::{AnalysisContext, SemanticAnalyzer};
pub use diagnostic::{Diagnostic, Location as DiagnosticLocation, Position, Range, Severity};
pub use error::{Location, SemanticError, SemanticErrorKind, SemanticResult};
pub use graph::RelationshipGraph;
pub use resolver::NameResolver;
pub use symbol_table::SymbolTable;
pub use workspace::Workspace;

// ============================================================================
// Type Aliases and Documentation
// ============================================================================

/// A fully qualified name that uniquely identifies a symbol across all files.
///
/// Qualified names use `::` as a separator and follow the scope hierarchy:
///
/// # Format
///
/// `Package::Subpackage::Element::Member`
///
/// # Examples
///
/// ```text
/// "Automotive"                        → Package at root
/// "Automotive::Engine"                → Package nested in Automotive
/// "Automotive::Engine::V8"            → Definition in Engine package
/// "Automotive::Engine::V8::cylinders" → Feature of V8 definition
/// ```
///
/// # Usage
///
/// ```rust
/// use syster::semantic::QualifiedName;
///
/// let qname: QualifiedName = "Package::Element".to_string();
/// let symbol = symbol_table.lookup(&qname);
/// ```
///
/// # Invariants
///
/// - No leading or trailing `::`
/// - No empty components between `::`
/// - Case-sensitive
/// - Must resolve to exactly one symbol in the symbol table
pub type QualifiedName = String;

/// A simple (unqualified) name of a symbol.
///
/// This is the declared name without package/scope qualifiers.
///
/// # Examples
///
/// ```text
/// For qualified name "Automotive::Engine::V8":
/// - SimpleName is "V8"
///
/// For qualified name "Package::feature":
/// - SimpleName is "feature"
/// ```
pub type SimpleName = String;

/// Unique identifier for a scope (package or classifier context).
///
/// Scopes are assigned incrementally during symbol table population:
/// - Scope 0: Root/global scope
/// - Scope 1+: Nested scopes (packages, classifiers)
///
/// # Usage
///
/// Used to track the parent scope of each symbol for name resolution.
///
/// ```rust
/// use syster::semantic::ScopeId;
///
/// let scope_id: ScopeId = symbol.scope_id();
/// ```
pub type ScopeId = usize;

/// Path to a source file in the workspace.
///
/// May be absolute or relative depending on how files were added to the workspace.
///
/// # Examples
///
/// ```text
/// "/workspaces/project/src/model.sysml"
/// "sysml.library/Systems Library/Parts.sysml"
/// ```
pub type SourceFilePath = String;
