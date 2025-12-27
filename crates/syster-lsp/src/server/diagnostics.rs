use super::LspServer;
use async_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};

impl LspServer {
    /// Get LSP diagnostics for a given file
    pub fn get_diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        let path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        // Convert parse errors to LSP diagnostics
        self.parse_errors
            .get(&path)
            .map(|errors| {
                errors
                    .iter()
                    .map(|e| Diagnostic {
                        range: Range {
                            start: Position {
                                line: e.position.line as u32,
                                character: e.position.column as u32,
                            },
                            end: Position {
                                line: e.position.line as u32,
                                character: (e.position.column + 1) as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: e.message.clone(),
                        ..Default::default()
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}
