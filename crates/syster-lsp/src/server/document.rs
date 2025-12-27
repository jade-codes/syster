use std::path::PathBuf;

use super::LspServer;
use super::helpers::apply_text_edit;
use async_lsp::lsp_types::{TextDocumentContentChangeEvent, Url};
use syster::core::constants::{KERML_EXT, SYSML_EXT};

impl LspServer {
    /// Convert URI to path and validate extension
    fn uri_to_sysml_path(&self, uri: &Url) -> Result<PathBuf, String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {uri}"))?;

        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "File has no extension".to_string())?;

        match ext {
            SYSML_EXT => Ok(path),
            KERML_EXT => Err("KerML files not yet fully supported".to_string()),
            _ => Err(format!("Unsupported file extension: {ext}")),
        }
    }

    /// Parse text and update workspace, returning whether parse succeeded
    fn parse_into_workspace(&mut self, path: &PathBuf, text: &str) {
        let parse_result = syster::project::file_loader::parse_with_result(text, path);
        self.parse_errors.insert(path.clone(), parse_result.errors);

        if let Some(file) = parse_result.content {
            // Use update if file exists, otherwise add
            if self.workspace.get_file(path).is_some() {
                self.workspace.update_file(path, file);
            } else {
                self.workspace.add_file(path.clone(), file);
            }
            let _ = self.workspace.populate_affected();
        } else {
            // Parse failed - remove stale file from workspace
            self.workspace.remove_file(path);
        }
    }

    /// Open a document and add it to the workspace
    pub fn open_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        self.ensure_workspace_loaded()?;
        let path = self.uri_to_sysml_path(uri)?;
        self.document_texts.insert(path.clone(), text.to_string());
        self.parse_into_workspace(&path, text);
        Ok(())
    }

    /// Update an open document with new content
    pub fn change_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        self.ensure_workspace_loaded()?;
        let path = self.uri_to_sysml_path(uri)?;
        self.document_texts.insert(path.clone(), text.to_string());
        self.parse_into_workspace(&path, text);
        Ok(())
    }

    /// Close a document - optionally remove from workspace
    /// For now, we keep documents in workspace even after close
    /// to maintain cross-file references
    pub fn close_document(&mut self, _uri: &Url) -> Result<(), String> {
        // We don't remove from workspace to keep cross-file references working
        // In the future, might want to track "open" vs "workspace" files separately
        Ok(())
    }

    /// Apply an incremental text change to a document
    ///
    /// This method updates the document content based on LSP TextDocumentContentChangeEvent
    /// and re-parses the file. Supports both ranged changes and full document updates.
    pub fn apply_incremental_change(
        &mut self,
        uri: &Url,
        change: &TextDocumentContentChangeEvent,
    ) -> Result<(), String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {uri}"))?;

        // Get current document text, or empty string if document not yet opened
        let current_text = self.document_texts.get(&path).cloned().unwrap_or_default();

        // Apply the change
        let new_text = if let Some(range) = &change.range {
            // Incremental change with range
            // If document is empty and this is the first edit, treat it as full replacement
            if current_text.is_empty() {
                change.text.clone()
            } else {
                apply_text_edit(&current_text, range, &change.text)?
            }
        } else {
            // Full document replacement (shouldn't happen with INCREMENTAL sync, but handle it)
            change.text.clone()
        };

        // If document wasn't opened yet, treat this as opening it
        if !self.document_texts.contains_key(&path) {
            self.open_document(uri, &new_text)
        } else {
            // Re-parse and update with the new text
            self.change_document(uri, &new_text)
        }
    }
}
