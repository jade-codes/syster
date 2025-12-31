//! Selection range support for the LSP server

use super::LspServer;
use super::helpers::span_to_lsp_range;
use async_lsp::lsp_types::{Position, Range, SelectionRange};
use std::path::Path;
use syster::core::Position as CorePosition;
use syster::semantic::find_selection_spans;

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
                let spans = find_selection_spans(workspace_file.content(), core_pos);

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
            range: span_to_lsp_range(&outermost),
            parent: None,
        };

        // Build chain from outermost to innermost
        for span in iter {
            current = SelectionRange {
                range: span_to_lsp_range(&span),
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
