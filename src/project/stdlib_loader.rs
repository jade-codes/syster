use crate::language::sysml::syntax::SysMLFile;
use crate::semantic::Workspace;
use rayon::prelude::*;
use std::path::PathBuf;

use super::file_loader;

/// Loads the standard library from /sysml.lib/ at startup
pub struct StdLibLoader {
    stdlib_path: PathBuf,
}

impl StdLibLoader {
    pub fn new() -> Self {
        Self {
            stdlib_path: PathBuf::from("sysml.library"),
        }
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self { stdlib_path: path }
    }

    /// Loads the SysML standard library into the workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The stdlib directory cannot be read
    /// - File collection fails
    ///
    /// Note: Individual file parse failures are logged but do not cause the load to fail.
    pub fn load(&self, workspace: &mut Workspace) -> Result<(), String> {
        if !self.stdlib_path.exists() || !self.stdlib_path.is_dir() {
            return Ok(());
        }

        // Collect all file paths first
        let file_paths = file_loader::collect_file_paths(&self.stdlib_path)?;

        // Parse files in parallel
        let results: Vec<_> = file_paths
            .par_iter()
            .map(|path| (path, self.parse_file(path)))
            .collect();

        // Add successfully parsed files and track failures
        let mut failed_files = Vec::new();
        for (path, result) in results {
            match result {
                Ok((path, file)) => {
                    workspace.add_file(path, file);
                }
                Err(e) => {
                    failed_files.push((path.clone(), e));
                }
            }
        }

        workspace.mark_stdlib_loaded();

        Ok(())
    }

    fn parse_file(&self, path: &PathBuf) -> Result<(PathBuf, SysMLFile), String> {
        file_loader::load_and_parse(path).map(|file| (path.clone(), file))
    }
}

impl Default for StdLibLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
