//! Workspace file representation

use crate::syntax::SyntaxFile;
use crate::syntax::sysml::ast::SysMLFile;
use std::path::PathBuf;

/// Represents a file in the workspace with its path and parsed content
#[derive(Debug)]
pub struct WorkspaceFile {
    path: PathBuf,
    content: SyntaxFile,
    version: u32,
    populated: bool,
}

impl WorkspaceFile {
    pub fn new(path: PathBuf, content: SyntaxFile) -> Self {
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

    pub fn content(&self) -> &SyntaxFile {
        &self.content
    }

    /// Returns the content as SysML file if it is SysML, otherwise None
    pub fn content_as_sysml(&self) -> Option<&SysMLFile> {
        match &self.content {
            SyntaxFile::SysML(file) => Some(file),
            _ => None,
        }
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

    pub(super) fn update_content(&mut self, content: SyntaxFile) {
        self.content = content;
        self.version += 1;
        self.populated = false; // Need to re-populate after content change
    }
}
