//! Inlay hint support for the LSP server

use super::LspServer;
use super::helpers::uri_to_path;
use async_lsp::lsp_types::{
    InlayHint, InlayHintKind, InlayHintLabel, InlayHintParams, Position as LspPosition,
};
use syster::core::Position as BasePosition;
use syster::semantic::{InlayHintKind as BaseInlayHintKind, extract_inlay_hints};

impl LspServer {
    /// Get inlay hints for a document
    pub fn get_inlay_hints(&self, params: &InlayHintParams) -> Vec<InlayHint> {
        let uri = &params.text_document.uri;

        let Some(path) = uri_to_path(uri) else {
            return vec![];
        };

        let workspace_file = match self.workspace.files().get(&path) {
            Some(f) => f,
            None => return vec![],
        };

        // Convert LSP range to base positions
        let range = Some((
            BasePosition {
                line: params.range.start.line as usize,
                column: params.range.start.character as usize,
            },
            BasePosition {
                line: params.range.end.line as usize,
                column: params.range.end.character as usize,
            },
        ));

        // Extract hints using the semantic layer
        let base_hints = extract_inlay_hints(
            workspace_file.content(),
            self.workspace.symbol_table(),
            range,
        );

        // Convert base hints to LSP hints
        base_hints
            .into_iter()
            .map(|hint| InlayHint {
                position: LspPosition {
                    line: hint.position.line as u32,
                    character: hint.position.column as u32,
                },
                label: InlayHintLabel::String(hint.label),
                kind: Some(match hint.kind {
                    BaseInlayHintKind::Type => InlayHintKind::TYPE,
                    BaseInlayHintKind::Parameter => InlayHintKind::PARAMETER,
                }),
                text_edits: None,
                tooltip: None,
                padding_left: Some(hint.padding_left),
                padding_right: Some(hint.padding_right),
                data: None,
            })
            .collect()
    }
}
