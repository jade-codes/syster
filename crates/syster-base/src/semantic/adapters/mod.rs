//! # Semantic Adapters
//!
//! Adapters form the **architectural boundary** between language-specific syntax and
//! language-agnostic semantic analysis.
//!
//! ## Architecture
//!
//! ```text
//! Syntax Layer (AST)
//!      ↓
//! Adapters (Language-Aware) ← YOU ARE HERE
//!      ↓ (converts to SemanticRole)
//! Semantic Layer (Language-Agnostic)
//!      ↓
//! Analysis & Validation
//! ```
//!
//! ## Responsibilities
//!
//! - **Convert ASTs to Symbols**: Extract language-agnostic `Symbol` representations
//! - **Map Semantic Roles**: Convert language kinds (e.g., `DefinitionKind::Requirement`)
//!   to generic `SemanticRole` enum values
//! - **Provide Validators**: Supply language-specific validation rules that work with semantic roles
//!
//! ## Important: This is the ONLY module that imports from syntax
//!
//! Only files in `semantic/adapters/` and `semantic/processors/` should import from
//! `syntax::sysml` or `syntax::kerml`. All other semantic code must remain language-agnostic
//! and work solely with `SemanticRole`, `Symbol`, and other semantic types.
//!
//! This boundary is enforced by architecture tests in `tests/architecture_tests.rs`.

pub mod kerml;
pub mod kerml_adapter;
pub mod syntax_factory;
mod sysml;
pub mod sysml_adapter;

pub use kerml_adapter::KermlAdapter;
pub use syntax_factory::{
    extract_folding_ranges, extract_inlay_hints, find_selection_spans, populate_syntax_file,
};
pub use sysml_adapter::SysmlAdapter;

// Re-export types used by the factory functions
pub use crate::semantic::types::FoldingRangeInfo;

// Language-specific adapter functions for tests and direct access
pub mod folding_ranges {
    pub use super::kerml::folding_ranges::extract_folding_ranges as extract_kerml_folding_ranges;
    pub use super::sysml::folding_ranges::extract_folding_ranges as extract_sysml_folding_ranges;
}

pub mod selection {
    pub use super::kerml::selection::find_selection_spans as find_kerml_selection_spans;
    pub use super::sysml::selection::find_selection_spans as find_sysml_selection_spans;
}

pub mod inlay_hints {
    pub use super::kerml::inlay_hints::extract_inlay_hints as extract_kerml_inlay_hints;
    pub use super::sysml::inlay_hints::extract_inlay_hints as extract_sysml_inlay_hints;
}

#[cfg(test)]
mod tests;
