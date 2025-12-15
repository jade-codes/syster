use crate::semantic::Workspace;
use std::path::PathBuf;

use super::file_loader;

/// Loads workspace files on demand
pub struct WorkspaceLoader;

impl WorkspaceLoader {
    pub fn new() -> Self {
        Self
    }

    /// Loads a single SysML or KerML file into the workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be read
    /// - The file has an invalid extension
    /// - The file fails to parse
    /// - AST construction fails
    pub fn load_file<P: Into<PathBuf>>(
        &self,
        path: P,
        workspace: &mut Workspace,
    ) -> Result<(), String> {
        let path = path.into();
        self.load_file_internal(&path, workspace)
    }

    /// Loads all SysML and KerML files from a directory recursively.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The directory does not exist or is not a directory
    /// - Any file in the directory tree cannot be read
    /// - Any file fails to parse
    pub fn load_directory<P: Into<PathBuf>>(
        &self,
        path: P,
        workspace: &mut Workspace,
    ) -> Result<(), String> {
        let path = path.into();
        if !path.exists() || !path.is_dir() {
            return Err(format!("Directory not found: {}", path.display()));
        }
        self.load_directory_recursive(&path, workspace)
    }

    fn load_directory_recursive(
        &self,
        dir: &PathBuf,
        workspace: &mut Workspace,
    ) -> Result<(), String> {
        let paths = file_loader::collect_file_paths(dir)?;

        for path in paths {
            self.load_file_internal(&path, workspace)?;
        }

        Ok(())
    }

    fn load_file_internal(&self, path: &PathBuf, workspace: &mut Workspace) -> Result<(), String> {
        let file = file_loader::load_and_parse(path)?;
        workspace.add_file(path.clone(), file);
        Ok(())
    }
}

impl Default for WorkspaceLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
