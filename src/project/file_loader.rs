use crate::core::constants::{KERML_EXT, SUPPORTED_EXTENSIONS, SYSML_EXT};
use crate::language::sysml::syntax::SysMLFile;
use from_pest::FromPest;
use pest::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// Loads and parses a SysML or KerML file into a SysMLFile AST.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The file has an invalid extension
/// - The file fails to parse
/// - AST construction fails
pub fn load_and_parse(path: &PathBuf) -> Result<SysMLFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    parse_content(&content, path)
}

/// Parses SysML or KerML content from a string (for LSP in-memory documents).
///
/// # Errors
///
/// Returns an error if:
/// - The file has an invalid extension
/// - The content fails to parse
/// - AST construction fails
pub fn parse_content(content: &str, path: &Path) -> Result<SysMLFile, String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| format!("Invalid file extension for {}", path.display()))?;

    match ext {
        SYSML_EXT => {
            let mut pairs =
                crate::parser::SysMLParser::parse(crate::parser::sysml::Rule::model, content)
                    .map_err(|e| format!("Parse error in {}: {}", path.display(), e))?;

            SysMLFile::from_pest(&mut pairs)
                .map_err(|e| format!("AST error in {}: {:?}", path.display(), e))
        }
        KERML_EXT => {
            // TODO: Add KerML parser support - return empty file for now
            Ok(SysMLFile {
                namespace: None,
                elements: vec![],
            })
        }
        _ => Err(format!("Unsupported file extension: {}", ext)),
    }
}

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
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
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

#[cfg(test)]
mod tests;
