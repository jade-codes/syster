use super::LspServer;
use std::path::Path;
use syster::semantic::symbol_table::Symbol;
use tower_lsp::lsp_types::{FoldingRange, FoldingRangeKind};

impl LspServer {
    /// Get all foldable regions in a document
    pub fn get_folding_ranges(&self, file_path: &Path) -> Vec<FoldingRange> {
        let mut ranges = Vec::new();

        // Fold symbol definitions (packages, classifiers, features)
        for (_, symbol) in self.workspace.symbol_table().all_symbols() {
            // Only include symbols defined in this file
            if symbol.source_file() != Some(file_path.to_str().unwrap_or("")) {
                continue;
            }

            if let Some(span) = symbol.span() {
                // Only create folding range if it spans multiple lines
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

        // Fold import blocks
        if let Some(text) = self.document_texts.get(file_path) {
            ranges.extend(self.find_import_blocks(text));
            ranges.extend(self.find_comment_blocks(text));
        }

        // Sort by start line
        ranges.sort_by_key(|r| r.start_line);

        ranges
    }

    /// Find contiguous import statements
    fn find_import_blocks(&self, text: &str) -> Vec<FoldingRange> {
        let mut ranges = Vec::new();
        let mut in_import_block = false;
        let mut block_start = 0u32;
        let mut block_end = 0u32;

        for (line_idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("import ") {
                if !in_import_block {
                    // Start new import block
                    in_import_block = true;
                    block_start = line_idx as u32;
                }
                block_end = line_idx as u32;
            } else if in_import_block && !trimmed.is_empty() {
                // Non-import line ends the block
                if block_end > block_start {
                    // Only create range if multiple imports
                    ranges.push(FoldingRange {
                        start_line: block_start,
                        start_character: None,
                        end_line: block_end,
                        end_character: None,
                        kind: Some(FoldingRangeKind::Imports),
                        collapsed_text: Some("...".to_string()),
                    });
                }
                in_import_block = false;
            }
        }

        // Handle block at end of file
        if in_import_block && block_end > block_start {
            ranges.push(FoldingRange {
                start_line: block_start,
                start_character: None,
                end_line: block_end,
                end_character: None,
                kind: Some(FoldingRangeKind::Imports),
                collapsed_text: Some("...".to_string()),
            });
        }

        ranges
    }

    /// Find multi-line comment blocks
    fn find_comment_blocks(&self, text: &str) -> Vec<FoldingRange> {
        let mut ranges = Vec::new();
        let mut in_comment_block = false;
        let mut block_start = 0u32;
        let mut block_end = 0u32;

        for (line_idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                if !in_comment_block {
                    // Start new comment block
                    in_comment_block = true;
                    block_start = line_idx as u32;
                }
                block_end = line_idx as u32;
            } else if in_comment_block && !trimmed.is_empty() {
                // Non-comment line ends the block
                if block_end > block_start {
                    // Only create range if multiple comment lines
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

        ranges
    }
}
