//! Folding range support for the LSP server
//!
//! This module builds LSP FoldingRange types directly from the semantic adapters.

use super::LspServer;
use super::helpers::span_to_folding_range;
use async_lsp::lsp_types::{FoldingRange, FoldingRangeKind};
use std::path::Path;
use syster::semantic::{
    FoldingRangeInfo, extract_kerml_folding_ranges, extract_sysml_folding_ranges,
};
use syster::syntax::SyntaxFile;

impl LspServer {
    /// Get all foldable regions in a document using the parsed AST
    pub fn get_folding_ranges(&self, file_path: &Path) -> Vec<FoldingRange> {
        let Some(workspace_file) = self.workspace.files().get(file_path) else {
            return Vec::new();
        };

        // Helper to convert FoldingRangeInfo to LSP FoldingRange
        let to_folding_range = |info: FoldingRangeInfo| {
            let kind = if info.is_comment {
                FoldingRangeKind::Comment
            } else {
                FoldingRangeKind::Region
            };
            span_to_folding_range(&info.span, kind)
        };

        let mut ranges: Vec<FoldingRange> = match workspace_file.content() {
            SyntaxFile::SysML(sysml_file) => extract_sysml_folding_ranges(sysml_file)
                .into_iter()
                .map(to_folding_range)
                .collect(),
            SyntaxFile::KerML(kerml_file) => extract_kerml_folding_ranges(kerml_file)
                .into_iter()
                .map(to_folding_range)
                .collect(),
        };

        ranges.sort_by_key(|r| r.start_line);
        ranges
    }
}
