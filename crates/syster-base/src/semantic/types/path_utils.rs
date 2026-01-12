//! Path normalization utilities for consistent file path handling.
//!
//! This module provides functions for normalizing file paths to ensure
//! consistent storage and lookup across the symbol table and reference index.

use crate::core::constants::STDLIB_DIR;
use std::path::{Path, PathBuf};

/// Normalize a file path for consistent storage and lookup.
///
/// For stdlib files (containing "sysml.library/"), extracts the relative path
/// starting from "sysml.library/". This ensures that stdlib files are always
/// referenced consistently regardless of their absolute location on disk.
///
/// For other files, attempts canonicalization to resolve symlinks and relative paths.
///
/// # Examples
///
/// ```
/// use syster::semantic::types::normalize_path;
///
/// // Stdlib paths are normalized to relative paths
/// let path = "/home/user/project/sysml.library/Systems Library/Parts.sysml";
/// assert_eq!(normalize_path(path), "sysml.library/Systems Library/Parts.sysml");
///
/// // Non-stdlib paths are canonicalized if possible
/// let path = "./src/model.sysml";
/// // Returns absolute path after canonicalization
/// ```
pub fn normalize_path(path: &str) -> String {
    // Check if this is a stdlib file
    // Use STDLIB_DIR constant with trailing slash for consistent matching
    let stdlib_pattern = format!("{STDLIB_DIR}/");
    if let Some(idx) = path.find(&stdlib_pattern) {
        return path[idx..].to_string();
    }

    // For non-stdlib files, try to canonicalize
    if let Ok(canonical) = Path::new(path).canonicalize() {
        return canonical.to_string_lossy().to_string();
    }

    // If canonicalization fails, do simple normalization
    let path_buf = PathBuf::from(path);
    let normalized = if path_buf.is_absolute() {
        path_buf
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join(path_buf)
    };
    normalized.to_string_lossy().to_string()
}

/// Normalize a PathBuf for consistent storage and lookup.
///
/// This is a convenience wrapper around `normalize_path` that accepts a Path.
pub fn normalize_pathbuf(path: &std::path::Path) -> String {
    normalize_path(&path.to_string_lossy())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_stdlib_path() {
        let path =
            "/workspaces/syster/crates/syster-base/sysml.library/Systems Library/Parts.sysml";
        assert_eq!(
            normalize_path(path),
            "sysml.library/Systems Library/Parts.sysml"
        );
    }

    #[test]
    fn test_normalize_stdlib_path_different_prefix() {
        let path = "/home/user/project/sysml.library/Domain Libraries/Analysis/TradeStudies.sysml";
        assert_eq!(
            normalize_path(path),
            "sysml.library/Domain Libraries/Analysis/TradeStudies.sysml"
        );
    }

    #[test]
    fn test_normalize_non_stdlib_relative_path() {
        // For relative paths that don't exist, it should still produce a normalized result
        let path = "test.sysml";
        let normalized = normalize_path(path);
        // Should be absolute (joined with current dir)
        assert!(normalized.ends_with("test.sysml"));
    }

    #[test]
    fn test_normalize_pathbuf() {
        let path = PathBuf::from("/some/path/sysml.library/Kernel Libraries/Base.kerml");
        assert_eq!(
            normalize_pathbuf(&path),
            "sysml.library/Kernel Libraries/Base.kerml"
        );
    }
}
