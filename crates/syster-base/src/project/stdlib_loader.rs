mod loader;

use crate::semantic::Workspace;
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

/// Loads the standard library from /sysml.lib/ at startup
pub struct StdLibLoader {
    stdlib_path: PathBuf,
    /// Track if stdlib has been loaded (for lazy loading)
    loaded: bool,
}

impl StdLibLoader {
    /// Creates a new eager stdlib loader (loads immediately when `load()` is called)
    pub fn new() -> Self {
        Self {
            stdlib_path: PathBuf::from("sysml.library"),
            loaded: false,
        }
    }

    /// Creates a new lazy stdlib loader (loads only when `ensure_loaded()` is called)
    pub fn lazy() -> Self {
        Self {
            stdlib_path: PathBuf::from("sysml.library"),
            loaded: false,
        }
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self {
            stdlib_path: path,
            loaded: false,
        }
    }

    /// Returns true if stdlib has been loaded by this loader
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Ensures stdlib is loaded - loads only if not already loaded
    ///
    /// # Errors
    ///
    /// Returns `Ok(true)` if stdlib was loaded, `Ok(false)` if already loaded.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The stdlib directory cannot be read
    /// - File collection fails
    ///
    /// Note: Individual file parse failures are logged but do not cause the load to fail.
    pub fn ensure_loaded(&mut self, workspace: &mut Workspace<SyntaxFile>) -> Result<bool, String> {
        // Don't reload if already loaded
        if self.loaded || workspace.has_stdlib() {
            return Ok(false);
        }

        self.load(workspace)?;
        self.loaded = true;
        Ok(true)
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
    pub fn load(&self, workspace: &mut Workspace<SyntaxFile>) -> Result<(), String> {
        loader::load(&self.stdlib_path, workspace)
    }
}

impl Default for StdLibLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
