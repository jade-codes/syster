use super::LspServer;
use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, Range, TextEdit, Url, WorkspaceEdit};

impl LspServer {
    /// Rename a symbol at the given position
    ///
    /// Finds all references to the symbol and generates a WorkspaceEdit
    /// to rename them all to the new name.
    pub fn get_rename_edits(
        &self,
        uri: &Url,
        position: Position,
        new_name: &str,
    ) -> Option<WorkspaceEdit> {
        let path = uri.to_file_path().ok()?;
        let (element_name, _) = self.find_symbol_at_position(&path, position)?;

        // Look up the symbol
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(&element_name)
            .or_else(|| self.workspace.symbol_table().lookup(&element_name))?;

        // Collect all locations (definition + references)
        let mut edits_by_file: HashMap<Url, Vec<TextEdit>> = HashMap::new();

        // Add definition location
        if let (Some(source_file), Some(span)) = (symbol.source_file(), symbol.span()) {
            let file_uri = Url::from_file_path(source_file).ok()?;
            edits_by_file.entry(file_uri).or_default().push(TextEdit {
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
                new_text: new_name.to_string(),
            });
        }

        // Add all reference locations
        for reference in symbol.references() {
            let file_uri = Url::from_file_path(&reference.file).ok()?;
            edits_by_file.entry(file_uri).or_default().push(TextEdit {
                range: Range {
                    start: Position {
                        line: reference.span.start.line as u32,
                        character: reference.span.start.column as u32,
                    },
                    end: Position {
                        line: reference.span.end.line as u32,
                        character: reference.span.end.column as u32,
                    },
                },
                new_text: new_name.to_string(),
            });
        }

        Some(WorkspaceEdit {
            changes: Some(edits_by_file),
            document_changes: None,
            change_annotations: None,
        })
    }
}
