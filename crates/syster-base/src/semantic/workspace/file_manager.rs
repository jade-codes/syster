use crate::core::operation::OperationResult;
use crate::semantic::types::WorkspaceEvent;
use crate::semantic::workspace::{ParsedFile, Workspace, WorkspaceFile};
use std::path::PathBuf;

impl<T: ParsedFile> Workspace<T> {
    /// Adds a file to the workspace
    pub fn add_file(&mut self, path: PathBuf, content: T) {
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
    pub fn get_file(&self, path: &PathBuf) -> Option<&WorkspaceFile<T>> {
        self.files.get(path)
    }

    /// Updates an existing file's content (for LSP document sync)
    pub fn update_file(&mut self, path: &PathBuf, content: T) -> bool {
        // Check if file exists first
        if !self.files.contains_key(path) {
            return false;
        }

        // Emit event BEFORE modifying so listeners can query state
        let _ = {
            let event = WorkspaceEvent::FileUpdated { path: path.clone() };
            OperationResult::<(), String, WorkspaceEvent>::success((), Some(event))
        }
        .publish(self);

        // Now update the file
        if let Some(file) = self.files.get_mut(path) {
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
            self.file_imports.remove(path);

            // Remove references from this file
            let file_path_str = path.to_string_lossy().to_string();
            self.reference_index
                .remove_references_from_file(&file_path_str);

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
