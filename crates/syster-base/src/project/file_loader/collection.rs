use crate::core::constants::SUPPORTED_EXTENSIONS;
use std::fs;
use std::path::PathBuf;

/// Recursively collects all supported file paths from a directory.
///
/// # Errors
///
/// Returns an error if:
/// - The directory cannot be read
/// - A directory entry is invalid
pub fn collect_file_paths(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    collect_recursive(dir, &mut paths)?;
    Ok(paths)
}

fn collect_recursive(dir: &PathBuf, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {e}"))?;
        let path = entry.path();

        if path.is_dir() {
            collect_recursive(&path, paths)?;
        } else if path.is_file()
            && path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| SUPPORTED_EXTENSIONS.contains(&ext))
        {
            paths.push(path);
        }
    }

    Ok(())
}
