use super::LspServer;
use super::helpers::{span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{Position, PrepareRenameResponse, TextEdit, Url, WorkspaceEdit};
use std::collections::HashMap;

impl LspServer {
    /// Prepare rename: validate that the symbol at the position can be renamed
    /// Returns the range of the symbol and its current text, or None if rename is not valid
    pub fn prepare_rename(&self, uri: &Url, position: Position) -> Option<PrepareRenameResponse> {
        let path = uri_to_path(uri)?;
        let (element_name, range) = self.find_symbol_at_position(&path, position)?;

        // Look up the symbol to verify it exists and is renamable
        let symbol = self.workspace.symbol_table().resolve(&element_name)?;

        // Get the simple name (last component) for display
        let simple_name = symbol.name().to_string();

        // Return the range where the rename will happen and the current text
        Some(PrepareRenameResponse::RangeWithPlaceholder {
            range,
            placeholder: simple_name,
        })
    }

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
        let path = uri_to_path(uri)?;
        let (element_name, _) = self.find_symbol_at_position(&path, position)?;

        // Look up the symbol
        let symbol = self.workspace.symbol_table().resolve(&element_name)?;

        let qualified_name = symbol.qualified_name().to_string();

        // Collect all locations (definition + references)
        let mut edits_by_file: HashMap<Url, Vec<TextEdit>> = HashMap::new();

        // Add definition location
        if let (Some(source_file), Some(span)) = (symbol.source_file(), symbol.span()) {
            let file_uri = Url::from_file_path(source_file).ok()?;
            edits_by_file.entry(file_uri).or_default().push(TextEdit {
                range: span_to_lsp_range(&span),
                new_text: new_name.to_string(),
            });
        }

        // Add relationship references (typing, specialization, etc.)
        let refs = self
            .workspace
            .relationship_graph()
            .get_references_to(&qualified_name);

        for (source_qname, span) in refs {
            // Only use span from relationship graph; skip references without precise spans
            let reference_span = match span {
                Some(s) => s,
                None => continue, // Skip imprecise references
            };

            if let Some(source_symbol) =
                self.workspace.symbol_table().lookup_qualified(source_qname)
                && let Some(file) = source_symbol.source_file()
                && let Ok(file_uri) = Url::from_file_path(file)
            {
                edits_by_file.entry(file_uri).or_default().push(TextEdit {
                    range: span_to_lsp_range(reference_span),
                    new_text: new_name.to_string(),
                });
            }
        }

        // Add import references
        let import_refs = self
            .workspace
            .symbol_table()
            .get_import_references(&qualified_name);

        for (file, span) in import_refs {
            if let Ok(file_uri) = Url::from_file_path(file) {
                edits_by_file.entry(file_uri).or_default().push(TextEdit {
                    range: span_to_lsp_range(span),
                    new_text: new_name.to_string(),
                });
            }
        }

        Some(WorkspaceEdit {
            changes: Some(edits_by_file),
            document_changes: None,
            change_annotations: None,
        })
    }
}
