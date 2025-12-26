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

mod kerml;
pub mod kerml_adapter;
pub mod syntax_factory;
mod sysml;
pub mod sysml_adapter;
mod validator_factory;

pub use kerml_adapter::KermlAdapter;
pub use syntax_factory::populate_syntax_file;
pub use sysml::validator::SysmlValidator;
pub use sysml_adapter::SysmlAdapter;
pub use validator_factory::create_validator;

// Re-export folding functions for each language
pub mod folding {
    pub use super::kerml::folding::extract_folding_ranges as extract_kerml_folding_ranges;
    pub use super::sysml::folding::extract_folding_ranges as extract_sysml_folding_ranges;
}

#[cfg(test)]
mod tests;
