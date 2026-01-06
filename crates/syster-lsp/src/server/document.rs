use std::path::PathBuf;

use super::LspServer;
use super::helpers::apply_text_edit;
use async_lsp::lsp_types::{TextDocumentContentChangeEvent, Url};
use syster::core::constants::is_supported_extension;

impl LspServer {
    /// Apply a text change without re-parsing (fast path for debouncing)
    ///
    /// This method updates the text buffer only. Call `parse_document` after
    /// debounce delay to actually parse the updated content.
    pub fn apply_text_change_only(
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

        // Update text buffer only - parsing happens later via parse_document
        self.document_texts.insert(path, new_text);
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

    /// Open a document and add it to the workspace
    pub fn open_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        self.ensure_workspace_loaded()?;
        let path = self.uri_to_model_path(uri)?;
        self.document_texts.insert(path.clone(), text.to_string());
        self.parse_into_workspace(&path, text);
        Ok(())
    }

    /// Parse a document that already has updated text
    /// Called after debounce delay
    pub fn parse_document(&mut self, uri: &Url) {
        // Validate file extension before parsing
        let path = match self.uri_to_model_path(uri) {
            Ok(p) => p,
            Err(_) => return, // Unsupported file type, skip parsing
        };

        // Ensure workspace is loaded before parsing
        if self.ensure_workspace_loaded().is_err() {
            return;
        }

        // Get current text and parse it
        if let Some(text) = self.document_texts.get(&path).cloned() {
            self.parse_into_workspace(&path, &text);
        }
    }

    /// Parse text and update workspace
    fn parse_into_workspace(&mut self, path: &PathBuf, text: &str) {
        let parse_result = syster::project::file_loader::parse_with_result(text, path);
        self.parse_errors.insert(path.clone(), parse_result.errors);

        if let Some(file) = parse_result.content {
            // Use update if file exists, otherwise add
            let file_exists = self.workspace.get_file(path).is_some();
            if file_exists {
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

    /// Convert URI to path and validate extension is SysML or KerML
    fn uri_to_model_path(&self, uri: &Url) -> Result<PathBuf, String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {uri}"))?;

        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "File has no extension".to_string())?;

        if is_supported_extension(ext) {
            Ok(path)
        } else {
            Err(format!("Unsupported file extension: {ext}"))
        }
    }
}
