use syster::semantic::Workspace;
use tower_lsp::lsp_types::Url;

/// Backend manages the workspace state for the LSP server
pub struct Backend {
    workspace: Workspace,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::new(),
        }
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    pub fn workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspace
    }

    /// Open a document and add it to the workspace
    pub fn open_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {}", uri))?;

        // Parse the file based on extension
        let file = if path.extension().and_then(|s| s.to_str()) == Some("sysml") {
            syster::project::file_loader::parse_content(text, &path)?
        } else if path.extension().and_then(|s| s.to_str()) == Some("kerml") {
            return Err("KerML files not yet fully supported".to_string());
        } else {
            return Err(format!(
                "Unsupported file extension: {:?}",
                path.extension()
            ));
        };

        // Add to workspace
        self.workspace.add_file(path, file);

        // Populate symbols
        self.workspace.populate_all()?;

        Ok(())
    }

    /// Update an open document with new content
    pub fn change_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {}", uri))?;

        // Parse the updated file
        let file = if path.extension().and_then(|s| s.to_str()) == Some("sysml") {
            syster::project::file_loader::parse_content(text, &path)?
        } else if path.extension().and_then(|s| s.to_str()) == Some("kerml") {
            return Err("KerML files not yet fully supported".to_string());
        } else {
            return Err(format!(
                "Unsupported file extension: {:?}",
                path.extension()
            ));
        };

        // Update in workspace (remove old, add new)
        self.workspace.remove_file(&path);
        self.workspace.add_file(path, file);

        // Repopulate symbols
        self.workspace.populate_all()?;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = Backend::new();
        assert_eq!(backend.workspace().file_count(), 0);
    }

    #[test]
    fn test_backend_provides_workspace_access() {
        let mut backend = Backend::new();

        // Should be able to access workspace mutably
        let workspace = backend.workspace_mut();
        assert_eq!(workspace.file_count(), 0);

        // Should be able to access workspace immutably
        let workspace = backend.workspace();
        assert_eq!(workspace.file_count(), 0);
    }

    #[test]
    fn test_open_sysml_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = "part def Vehicle;";

        backend.open_document(&uri, text).unwrap();

        assert_eq!(backend.workspace().file_count(), 1);
        assert!(backend.workspace().symbol_table().all_symbols().len() > 0);
    }

    #[test]
    fn test_open_invalid_sysml() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = "invalid syntax !@#$%";

        let result = backend.open_document(&uri, text);
        assert!(result.is_err());
    }

    #[test]
    fn test_open_unsupported_extension() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.txt").unwrap();
        let text = "some text";

        let result = backend.open_document(&uri, text);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file extension"));
    }

    #[test]
    fn test_open_kerml_file() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.kerml").unwrap();
        let text = "classifier Vehicle;";

        let result = backend.open_document(&uri, text);
        // KerML not yet supported
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("KerML"));
    }

    #[test]
    fn test_change_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Open initial document
        backend.open_document(&uri, "part def Car;").unwrap();
        assert_eq!(backend.workspace().file_count(), 1);
        let initial_symbols = backend.workspace().symbol_table().all_symbols().len();

        // Change document content
        backend
            .change_document(&uri, "part def Vehicle; part def Bike;")
            .unwrap();

        assert_eq!(backend.workspace().file_count(), 1);
        let updated_symbols = backend.workspace().symbol_table().all_symbols().len();
        assert!(updated_symbols > initial_symbols);
    }

    #[test]
    fn test_change_document_with_error() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Open valid document
        backend.open_document(&uri, "part def Car;").unwrap();

        // Try to change to invalid content
        let result = backend.change_document(&uri, "invalid syntax !@#");
        assert!(result.is_err());
    }

    #[test]
    fn test_change_nonexistent_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Try to change a document that was never opened
        let result = backend.change_document(&uri, "part def Car;");
        // Should succeed - change_document handles both open and update
        assert!(result.is_ok());
    }

    #[test]
    fn test_close_document() {
        let mut backend = Backend::new();
        let uri = Url::parse("file:///test.sysml").unwrap();

        // Open and close document
        backend.open_document(&uri, "part def Car;").unwrap();
        backend.close_document(&uri).unwrap();

        // Document should still be in workspace (we keep it for cross-file refs)
        assert_eq!(backend.workspace().file_count(), 1);
    }
}
