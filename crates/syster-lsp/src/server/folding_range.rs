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
        // Try to get the parsed file from the workspace
        let ranges = if let Some(workspace_file) = self.workspace.files().get(file_path) {
            match workspace_file.content() {
                SyntaxFile::SysML(sysml_file) => {
                    // Delegate to the semantic adapter for SysML
                    let semantic_ranges = extract_sysml_folding_ranges(sysml_file);
                    self.convert_folding_ranges(semantic_ranges)
                }
                SyntaxFile::KerML(kerml_file) => {
                    // Delegate to the semantic adapter for KerML
                    let semantic_ranges = extract_kerml_folding_ranges(kerml_file);
                    self.convert_folding_ranges(semantic_ranges)
                }
            }
        } else {
            // File not in workspace, fall back to symbol-based folding
            self.collect_symbol_folding_ranges(file_path)
        };

        // Sort by start line for consistent ordering
        let mut ranges = ranges;
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

    /// Fallback: collect folding ranges from symbol table (for files not in workspace)
    fn collect_symbol_folding_ranges(&self, file_path: &Path) -> Vec<FoldingRange> {
        use syster::semantic::symbol_table::Symbol;

        let mut ranges = Vec::new();

        for (_, symbol) in self.workspace.symbol_table().all_symbols() {
            // Only include symbols defined in this file
            if symbol.source_file() != Some(file_path.to_str().unwrap_or("")) {
                continue;
            }

            if let Some(span) = symbol.span() {
                if span.start.line < span.end.line {
                    ranges.push(FoldingRange {
                        start_line: span.start.line as u32,
                        start_character: Some(span.start.column as u32),
                        end_line: span.end.line as u32,
                        end_character: Some(span.end.column as u32),
                        kind: Some(match symbol {
                            Symbol::Package { .. } => FoldingRangeKind::Region,
                            Symbol::Classifier { .. } | Symbol::Definition { .. } => {
                                FoldingRangeKind::Region
                            }
                            Symbol::Feature { .. } | Symbol::Usage { .. } => {
                                FoldingRangeKind::Region
                            }
                            Symbol::Alias { .. } => FoldingRangeKind::Region,
                        }),
                        collapsed_text: None,
                    });
                }
            }
        }

        // Also check document text for comments (fallback for when AST not available)
        if let Some(text) = self.document_texts.get(file_path) {
            self.collect_comment_blocks_from_text(text, &mut ranges);
        }

        ranges
    }

    /// Fallback: find multi-line comment blocks from raw text
    fn collect_comment_blocks_from_text(&self, text: &str, ranges: &mut Vec<FoldingRange>) {
        let mut in_comment_block = false;
        let mut block_start = 0u32;
        let mut block_end = 0u32;

        for (line_idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                if !in_comment_block {
                    in_comment_block = true;
                    block_start = line_idx as u32;
                }
                block_end = line_idx as u32;
            } else if in_comment_block && !trimmed.is_empty() {
                if block_end > block_start {
                    ranges.push(FoldingRange {
                        start_line: block_start,
                        start_character: None,
                        end_line: block_end,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Comment),
                        collapsed_text: Some("/* ... */".to_string()),
                    });
                }
                in_comment_block = false;
            }
        }

        // Handle block at end of file
        if in_comment_block && block_end > block_start {
            ranges.push(FoldingRange {
                start_line: block_start,
                start_character: None,
                end_line: block_end,
                end_character: None,
                kind: Some(FoldingRangeKind::Comment),
                collapsed_text: Some("/* ... */".to_string()),
            });
        }
    }
}
