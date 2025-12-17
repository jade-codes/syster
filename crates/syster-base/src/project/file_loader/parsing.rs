use crate::core::constants::{KERML_EXT, SYSML_EXT};
use crate::project::parse_result::ParseError;
use std::fs;
use std::path::{Path, PathBuf};

/// Loads a file and returns its content as a string.
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn load_file(path: &PathBuf) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))
}

/// Validates that a file has a supported extension (.sysml or .kerml).
///
/// # Errors
///
/// Returns an error if the extension is not supported.
pub fn validate_extension(path: &Path) -> Result<&str, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| format!("Invalid file extension for {}", path.display()))?;

    match ext {
        SYSML_EXT | KERML_EXT => Ok(ext),
        _ => Err(format!("Unsupported file extension: {}", ext)),
    }
}

/// Returns the file extension if valid, for use in ParseResult contexts.
///
/// # Errors
///
/// Returns ParseError if extension is missing or unsupported.
pub fn get_extension(path: &Path) -> Result<&str, ParseError> {
    let ext = path.extension().and_then(|e| e.to_str());

    match ext {
        Some(SYSML_EXT) | Some(KERML_EXT) => Ok(ext.unwrap()),
        _ => Err(ParseError::syntax_error("Unsupported file extension", 0, 0)),
    }
}
