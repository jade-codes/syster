//! Inlay hint support for the LSP server

use super::LspServer;
use async_lsp::lsp_types::{
    InlayHint, InlayHintKind, InlayHintLabel, InlayHintParams, Position as LspPosition,
};
use syster::core::Position as BasePosition;
use syster::semantic::adapters::inlay_hints::{
    InlayHintKind as BaseInlayHintKind, extract_kerml_inlay_hints, extract_sysml_inlay_hints,
};
use syster::syntax::SyntaxFile;

impl LspServer {
    /// Get inlay hints for a document
    pub fn get_inlay_hints(&self, params: &InlayHintParams) -> Vec<InlayHint> {
        let uri = &params.text_document.uri;

        let path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return vec![],
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

        // Extract hints using the appropriate adapter
        let base_hints = match workspace_file.content() {
            SyntaxFile::SysML(sysml_file) => {
                extract_sysml_inlay_hints(sysml_file, self.workspace.symbol_table(), range)
            }
            SyntaxFile::KerML(kerml_file) => {
                extract_kerml_inlay_hints(kerml_file, self.workspace.symbol_table(), range)
            }
        };

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
