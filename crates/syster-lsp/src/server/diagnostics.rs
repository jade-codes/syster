use super::LspServer;
use super::helpers::{position_to_lsp_position, uri_to_path};
use async_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};

impl LspServer {
    /// Get LSP diagnostics for a given file
    pub fn get_diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        let Some(path) = uri_to_path(uri) else {
            return vec![];
        };

        // Convert parse errors to LSP diagnostics
        self.parse_errors
            .get(&path)
            .map(|errors| {
                errors
                    .iter()
                    .map(|e| {
                        let pos = position_to_lsp_position(&e.position);
                        Diagnostic {
                            range: Range {
                                start: pos,
                                end: Position {
                                    line: pos.line,
                                    character: pos.character + 1,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            message: e.message.clone(),
                            ..Default::default()
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}
