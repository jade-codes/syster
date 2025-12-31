mod loader;

use crate::core::constants::STDLIB_DIR;
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
    /// Creates a new stdlib loader with automatic path discovery.
    ///
    /// Searches for the stdlib in these locations (in order):
    /// 1. Next to the current executable (for installed binaries)
    /// 2. Current working directory (for development)
    pub fn new() -> Self {
        Self {
            stdlib_path: Self::discover_path(),
            loaded: false,
        }
    }

    /// Creates a new stdlib loader with a specific path.
    pub fn with_path(path: PathBuf) -> Self {
        Self {
            stdlib_path: path,
            loaded: false,
        }
    }

    /// Discover the stdlib path by searching common locations.
    ///
    /// Returns the first existing path, or falls back to the default.
    fn discover_path() -> PathBuf {
        // Try next to the executable first (for installed binaries)
        if let Some(exe_dir) = std::env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
        {
            let stdlib_next_to_exe = exe_dir.join(STDLIB_DIR);
            if stdlib_next_to_exe.exists() && stdlib_next_to_exe.is_dir() {
                return stdlib_next_to_exe;
            }
        }

        // Fall back to current directory / default path
        PathBuf::from(STDLIB_DIR)
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
