use super::LspServer;
use super::helpers::{collect_reference_locations, span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{Position, PrepareRenameResponse, TextEdit, Url, WorkspaceEdit};
use std::collections::HashMap;

impl LspServer {
    /// Prepare rename: validate that the symbol at the position can be renamed
    /// Returns the range of the symbol and its current text, or None if rename is not valid
    pub fn prepare_rename(&self, uri: &Url, position: Position) -> Option<PrepareRenameResponse> {
        let path = uri_to_path(uri)?;
        let (element_name, range) = self.find_symbol_at_position(&path, position)?;

        // Look up the symbol using resolver to verify it exists and is renamable
        let resolver = self.resolver();
        let symbol = resolver.resolve(&element_name)?;

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

        // Look up the symbol using resolver
        let resolver = self.resolver();
        let symbol = resolver.resolve(&element_name)?;
        let qualified_name = symbol.qualified_name();

        // Collect all locations (definition + references)
        let mut edits_by_file: HashMap<Url, Vec<TextEdit>> = HashMap::new();

        // Add definition location
        if let (Some(source_file), Some(span)) = (symbol.source_file(), symbol.span())
            && let Ok(file_uri) = Url::from_file_path(source_file)
        {
            edits_by_file.entry(file_uri).or_default().push(TextEdit {
                range: span_to_lsp_range(&span),
                new_text: new_name.to_string(),
            });
        }

        // Add all reference locations using shared helper
        for location in collect_reference_locations(&self.workspace, qualified_name) {
            edits_by_file
                .entry(location.uri)
                .or_default()
                .push(TextEdit {
                    range: location.range,
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
