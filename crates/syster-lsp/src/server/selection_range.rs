//! Selection range support for the LSP server
//!
//! This module builds LSP SelectionRange types directly from the semantic adapters.

use super::LspServer;
use std::path::Path;
use syster::core::Position as CorePosition;
use syster::semantic::selection::{find_kerml_selection_spans, find_sysml_selection_spans};
use syster::syntax::SyntaxFile;
use tower_lsp::lsp_types::{Position, Range, SelectionRange};

impl LspServer {
    /// Get selection ranges at the given positions in a document
    ///
    /// Returns a vector of SelectionRange chains, one for each input position.
    pub fn get_selection_ranges(
        &self,
        file_path: &Path,
        positions: Vec<Position>,
    ) -> Vec<SelectionRange> {
        let Some(workspace_file) = self.workspace.files().get(file_path) else {
            return positions
                .iter()
                .map(|p| self.default_selection_range(*p))
                .collect();
        };

        positions
            .iter()
            .map(|pos| {
                let core_pos = CorePosition::new(pos.line as usize, pos.character as usize);

                let spans = match workspace_file.content() {
                    SyntaxFile::SysML(sysml_file) => {
                        find_sysml_selection_spans(sysml_file, core_pos)
                    }
                    SyntaxFile::KerML(kerml_file) => {
                        find_kerml_selection_spans(kerml_file, core_pos)
                    }
                };

                if spans.is_empty() {
                    self.default_selection_range(*pos)
                } else {
                    self.build_selection_range_chain(spans)
                }
            })
            .collect()
    }

    /// Build a SelectionRange chain from spans (innermost to outermost)
    fn build_selection_range_chain(&self, spans: Vec<syster::core::Span>) -> SelectionRange {
        // spans are ordered from smallest (innermost) to largest (outermost)
        // We need to build a chain where innermost points to outermost as parent
        let mut iter = spans.into_iter().rev(); // Start from largest (outermost)

        let outermost = iter.next().expect("spans should not be empty");
        let mut current = SelectionRange {
            range: Range {
                start: Position {
                    line: outermost.start.line as u32,
                    character: outermost.start.column as u32,
                },
                end: Position {
                    line: outermost.end.line as u32,
                    character: outermost.end.column as u32,
                },
            },
            parent: None,
        };

        // Build chain from outermost to innermost
        for span in iter {
            current = SelectionRange {
                range: Range {
                    start: Position {
                        line: span.start.line as u32,
                        character: span.start.column as u32,
                    },
                    end: Position {
                        line: span.end.line as u32,
                        character: span.end.column as u32,
                    },
                },
                parent: Some(Box::new(current)),
            };
        }

        current
    }

    /// Create a default selection range (single character) when no AST node is found
    fn default_selection_range(&self, pos: Position) -> SelectionRange {
        SelectionRange {
            range: Range {
                start: pos,
                end: Position {
                    line: pos.line,
                    character: pos.character + 1,
                },
            },
            parent: None,
        }
    }
}
