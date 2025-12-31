//! Folding range support for the LSP server

use super::LspServer;
use super::helpers::span_to_folding_range;
use async_lsp::lsp_types::{FoldingRange, FoldingRangeKind};
use std::path::Path;
use syster::semantic::{FoldingRangeInfo, extract_folding_ranges};

impl LspServer {
    /// Get all foldable regions in a document using the parsed AST
    pub fn get_folding_ranges(&self, file_path: &Path) -> Vec<FoldingRange> {
        let Some(workspace_file) = self.workspace.files().get(file_path) else {
            return Vec::new();
        };

        let to_lsp_range = |info: FoldingRangeInfo| {
            let kind = if info.is_comment {
                FoldingRangeKind::Comment
            } else {
                FoldingRangeKind::Region
            };
            span_to_folding_range(&info.span, kind)
        };

        let mut ranges: Vec<FoldingRange> = extract_folding_ranges(workspace_file.content())
            .into_iter()
            .map(to_lsp_range)
            .collect();

        ranges.sort_by_key(|r| r.start_line);
        ranges
    }
}
