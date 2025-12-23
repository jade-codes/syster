use super::LspServer;
use syster::core::constants::{KERML_EXT, SYSML_EXT};
use tower_lsp::lsp_types::Url;

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

        // If updating, remove old file first
        if is_update {
            self.workspace.remove_file(&path);
        }

        // If parsing succeeded, add to workspace
        if let Some(file) = parse_result.content {
            self.workspace.add_file(path, file);
            // Populate symbols - ignore semantic errors for now
            let _ = self.workspace.populate_all();
            // Sync document texts from workspace (for stdlib and imported files)
            self.sync_document_texts_from_workspace();
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
}
