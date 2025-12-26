//! Folding range support for the LSP server
//!
//! This module converts semantic layer folding ranges to LSP types.
//! All AST analysis is delegated to the semantic adapters.

use super::LspServer;
use std::path::Path;
use syster::semantic::folding::{extract_kerml_folding_ranges, extract_sysml_folding_ranges};
use syster::semantic::types::{FoldableRange, FoldingKind};
use syster::syntax::SyntaxFile;
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};

impl LspServer {
    /// Get all foldable regions in a document using the parsed AST
    pub fn get_folding_ranges(&self, file_path: &Path) -> Vec<FoldingRange> {
        let Some(workspace_file) = self.workspace.files().get(file_path) else {
            return Vec::new();
        };

        let mut ranges = match workspace_file.content() {
            SyntaxFile::SysML(sysml_file) => {
                let semantic_ranges = extract_sysml_folding_ranges(sysml_file);
                self.convert_folding_ranges(semantic_ranges)
            }
            SyntaxFile::KerML(kerml_file) => {
                let semantic_ranges = extract_kerml_folding_ranges(kerml_file);
                self.convert_folding_ranges(semantic_ranges)
            }
        };

        ranges.sort_by_key(|r| r.start_line);
        ranges
    }

    /// Convert semantic FoldableRange to LSP FoldingRange
    fn convert_folding_ranges(&self, ranges: Vec<FoldableRange>) -> Vec<FoldingRange> {
        ranges
            .into_iter()
            .map(|r| self.to_lsp_folding_range(r))
            .collect()
    }

    /// Convert a single semantic FoldableRange to LSP FoldingRange
    fn to_lsp_folding_range(&self, range: FoldableRange) -> FoldingRange {
        FoldingRange {
            start_line: range.span.start.line as u32,
            start_character: Some(range.span.start.column as u32),
            end_line: range.span.end.line as u32,
            end_character: Some(range.span.end.column as u32),
            kind: Some(match range.kind {
                FoldingKind::Region => FoldingRangeKind::Region,
                FoldingKind::Comment => FoldingRangeKind::Comment,
            }),
            collapsed_text: range.collapsed_text,
        }
    }
}
