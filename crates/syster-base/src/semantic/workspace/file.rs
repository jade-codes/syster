//! Workspace file representation

use std::path::PathBuf;

/// Represents a file in the workspace with its path and parsed content
#[derive(Debug)]
pub struct WorkspaceFile<T> {
    path: PathBuf,
    content: T,
    version: u32,
    populated: bool,
}

impl<T> WorkspaceFile<T> {
    pub fn new(path: PathBuf, content: T) -> Self {
        Self {
            path,
            content,
            version: 0,
            populated: false,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn is_populated(&self) -> bool {
        self.populated
    }

    pub(super) fn set_populated(&mut self, populated: bool) {
        self.populated = populated;
    }

    pub(super) fn update_content(&mut self, content: T) {
        self.content = content;
        self.version += 1;
        self.populated = false; // Need to re-populate after content change
    }
}
