use super::LspServer;
use super::helpers::apply_text_edit;
use syster::core::constants::{KERML_EXT, SYSML_EXT};
use tower_lsp::lsp_types::{TextDocumentContentChangeEvent, Url};

impl LspServer {
    /// Parse and update a document in the workspace
    ///
    /// This is a helper method that handles:
    /// - Storing document text
    /// - Parsing the file
    /// - Storing parse errors
    /// - Updating the workspace
    /// - Repopulating symbols
    fn parse_and_update(&mut self, uri: &Url, text: &str, is_update: bool) -> Result<(), String> {
        // Ensure stdlib is loaded on first document open
        self.ensure_stdlib_loaded()?;

        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {uri}"))?;

        // Store document text
        self.document_texts.insert(path.clone(), text.to_string());

        // Parse the file based on extension
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "File has no extension".to_string())?;

        let parse_result = match ext {
            SYSML_EXT => syster::project::file_loader::parse_with_result(text, &path),
            KERML_EXT => return Err("KerML files not yet fully supported".to_string()),
            _ => return Err(format!("Unsupported file extension: {ext}")),
        };

        // Store parse errors
        self.parse_errors.insert(path.clone(), parse_result.errors);

        // If parsing succeeded, add or update in workspace
        if let Some(file) = parse_result.content {
            if is_update && self.workspace.get_file(&path).is_some() {
                // Fast path: update existing file without removing/re-adding
                self.workspace.update_file(&path, file);
            } else {
                // New file: remove old if exists, then add
                if is_update {
                    self.workspace.remove_file(&path);
                }
                self.workspace.add_file(path.clone(), file);
            }

            // Only populate affected (unpopulated) files, not the entire workspace
            // This is much faster than populate_all() which re-processes stdlib
            let _ = self.workspace.populate_affected();

            // Sync document texts from workspace (for stdlib and imported files)
            self.sync_document_texts_from_workspace();
        } else {
            // Parse failed - remove file from workspace if it exists
            if is_update {
                self.workspace.remove_file(&path);
            }
        }

        Ok(())
    }

    /// Open a document and add it to the workspace
    pub fn open_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        self.parse_and_update(uri, text, false)
    }

    /// Update an open document with new content
    pub fn change_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        self.parse_and_update(uri, text, true)
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
