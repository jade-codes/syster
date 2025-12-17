use crate::core::operation::OperationResult;
use crate::semantic::types::WorkspaceEvent;
use crate::semantic::workspace::{Workspace, WorkspaceFile};
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

impl Workspace {
    /// Adds a file to the workspace
    pub fn add_file(&mut self, path: PathBuf, content: SyntaxFile) {
        let _ = {
            // Extract imports from the file
            let imports = content.extract_imports();
            self.file_imports.insert(path.clone(), imports);

            let file = WorkspaceFile::new(path.clone(), content);
            self.files.insert(path.clone(), file);

            let event = WorkspaceEvent::FileAdded { path };
            OperationResult::<(), String, WorkspaceEvent>::success((), Some(event))
        }
        .publish(self);
    }

    /// Gets a reference to a file in the workspace
    pub fn get_file(&self, path: &PathBuf) -> Option<&WorkspaceFile> {
        self.files.get(path)
    }

    /// Updates an existing file's content (for LSP document sync)
    pub fn update_file(&mut self, path: &PathBuf, content: SyntaxFile) -> bool {
        // Check if file exists first
        if !self.files.contains_key(path) {
            return false;
        }

        // Emit event BEFORE clearing dependencies so listeners can query the graph
        let _ = {
            let event = WorkspaceEvent::FileUpdated { path: path.clone() };
            OperationResult::<(), String, WorkspaceEvent>::success((), Some(event))
        }
        .publish(self);

        // Now update the file
        if let Some(file) = self.files.get_mut(path) {
            // Clear old dependencies
            self.dependency_graph.remove_file(path);

            // Extract new imports
            let imports = content.extract_imports();
            self.file_imports.insert(path.clone(), imports);

            file.update_content(content);
            true
        } else {
            false
        }
    }

    /// Removes a file from the workspace
    pub fn remove_file(&mut self, path: &PathBuf) -> bool {
        let existed = self.files.remove(path).is_some();
        if existed {
            self.dependency_graph.remove_file(path);
            self.file_imports.remove(path);

            let _ = {
                let event = WorkspaceEvent::FileRemoved { path: path.clone() };
                OperationResult::<(), String, WorkspaceEvent>::success((), Some(event))
            }
            .publish(self);
        }
        existed
    }

    /// Marks a file as unpopulated (needing re-population)
    pub(super) fn mark_file_unpopulated(&mut self, path: &PathBuf) {
        if let Some(file) = self.files.get_mut(path) {
            file.set_populated(false);
        }
    }

    /// Marks a file as populated
    pub(super) fn mark_file_populated(&mut self, path: &PathBuf) {
        if let Some(file) = self.files.get_mut(path) {
            file.set_populated(true);
        }
    }
}
