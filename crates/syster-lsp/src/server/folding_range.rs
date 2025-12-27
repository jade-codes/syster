//! Folding range support for the LSP server
//!
//! This module builds LSP FoldingRange types directly from the semantic adapters.

use super::LspServer;
use std::path::Path;
use syster::semantic::folding::{extract_kerml_folding_ranges, extract_sysml_folding_ranges};
use syster::syntax::SyntaxFile;
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};

impl LspServer {
    /// Get all foldable regions in a document using the parsed AST
    pub fn get_folding_ranges(&self, file_path: &Path) -> Vec<FoldingRange> {
        let Some(workspace_file) = self.workspace.files().get(file_path) else {
            return Vec::new();
        };

        let mut ranges: Vec<FoldingRange> = match workspace_file.content() {
            SyntaxFile::SysML(sysml_file) => extract_sysml_folding_ranges(sysml_file)
                .into_iter()
                .map(|r| FoldingRange {
                    start_line: r.span.start.line as u32,
                    start_character: Some(r.span.start.column as u32),
                    end_line: r.span.end.line as u32,
                    end_character: Some(r.span.end.column as u32),
                    kind: Some(if r.is_comment {
                        FoldingRangeKind::Comment
                    } else {
                        FoldingRangeKind::Region
                    }),
                    collapsed_text: None,
                })
                .collect(),
            SyntaxFile::KerML(kerml_file) => extract_kerml_folding_ranges(kerml_file)
                .into_iter()
                .map(|r| FoldingRange {
                    start_line: r.span.start.line as u32,
                    start_character: Some(r.span.start.column as u32),
                    end_line: r.span.end.line as u32,
                    end_character: Some(r.span.end.column as u32),
                    kind: Some(if r.is_comment {
                        FoldingRangeKind::Comment
                    } else {
                        FoldingRangeKind::Region
                    }),
                    collapsed_text: None,
                })
                .collect(),
        };

        ranges.sort_by_key(|r| r.start_line);
        ranges
    }
}
