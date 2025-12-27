//! # Inlay Hints Feature
//!
//! Provides type annotations that can be displayed inline in the editor.
//! This is a semantic-layer feature that delegates to language-specific
//! adapters for AST traversal.

use crate::core::Position;
use crate::semantic::SymbolTable;
use crate::semantic::adapters::inlay_hints::{
    extract_kerml_inlay_hints, extract_sysml_inlay_hints,
};
use crate::semantic::types::InlayHint;
use crate::syntax::SyntaxFile;

pub use crate::semantic::types::{InlayHint as InlayHintType, InlayHintKind as InlayHintKindType};

/// Extract inlay hints from a syntax file.
///
/// This is the semantic layer entry point for inlay hints. It automatically
/// detects the file type and delegates to the appropriate adapter.
///
/// # Arguments
///
/// * `syntax_file` - The parsed syntax file (KerML or SysML)
/// * `symbol_table` - The symbol table for type resolution
/// * `range` - Optional range to filter hints (start and end positions)
///
/// # Returns
///
/// A vector of inlay hints within the specified range (or all hints if no range specified)
pub fn extract_inlay_hints(
    syntax_file: &SyntaxFile,
    symbol_table: &SymbolTable,
    range: Option<(Position, Position)>,
) -> Vec<InlayHint> {
    match syntax_file {
        SyntaxFile::SysML(sysml_file) => extract_sysml_inlay_hints(sysml_file, symbol_table, range),
        SyntaxFile::KerML(kerml_file) => extract_kerml_inlay_hints(kerml_file, symbol_table, range),
    }
}
