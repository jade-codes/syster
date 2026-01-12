//! Syntax Factory
//!
//! Unified entry points for syntax file operations.
//! This is the only place in the semantic layer that dispatches on SysML vs KerML.

use crate::core::{Position, Span};
use crate::semantic::graphs::ReferenceIndex;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::{FoldingRangeInfo, InlayHint, SemanticError};
use crate::syntax::SyntaxFile;

use super::inlay_hints::{extract_kerml_inlay_hints, extract_sysml_inlay_hints};
use super::kerml::folding_ranges::extract_folding_ranges as extract_kerml_folding_ranges;
use super::kerml::selection::find_selection_spans as find_kerml_selection_spans;
use super::sysml::folding_ranges::extract_folding_ranges as extract_sysml_folding_ranges;
use super::sysml::selection::find_selection_spans as find_sysml_selection_spans;
use super::{KermlAdapter, SysmlAdapter};

/// Populates a syntax file into the symbol table using the appropriate adapter
pub fn populate_syntax_file(
    syntax_file: &SyntaxFile,
    symbol_table: &mut SymbolTable,
    reference_index: &mut ReferenceIndex,
) -> Result<(), Vec<SemanticError>> {
    match syntax_file {
        SyntaxFile::SysML(sysml_file) => {
            let mut adapter = SysmlAdapter::with_index(symbol_table, reference_index);
            adapter.populate(sysml_file)
        }
        SyntaxFile::KerML(kerml_file) => {
            let mut adapter = KermlAdapter::with_index(symbol_table, reference_index);
            adapter.populate(kerml_file)
        }
    }
}

/// Extract folding ranges from any syntax file
pub fn extract_folding_ranges(file: &SyntaxFile) -> Vec<FoldingRangeInfo> {
    match file {
        SyntaxFile::SysML(sysml) => extract_sysml_folding_ranges(sysml),
        SyntaxFile::KerML(kerml) => extract_kerml_folding_ranges(kerml),
    }
}

/// Find selection spans at a position in any syntax file
pub fn find_selection_spans(file: &SyntaxFile, position: Position) -> Vec<Span> {
    match file {
        SyntaxFile::SysML(sysml) => find_sysml_selection_spans(sysml, position),
        SyntaxFile::KerML(kerml) => find_kerml_selection_spans(kerml, position),
    }
}

/// Extract inlay hints from any syntax file
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
