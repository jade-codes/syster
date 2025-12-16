use crate::language::sysml::syntax::{Element, SysMLFile};

/// Extracts all import paths from a SysML file
pub fn extract_imports(file: &SysMLFile) -> Vec<String> {
    file.elements
        .iter()
        .filter_map(|element| {
            if let Element::Import(import) = element {
                Some(import.path.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Parses an import path into its components (split by ::)
pub fn parse_import_path(path: &str) -> Vec<String> {
    path.split("::").map(|s| s.to_string()).collect()
}

/// Checks if an import is a wildcard import (ends with *)
pub fn is_wildcard_import(path: &str) -> bool {
    path.ends_with("::*") || path == "*"
}

#[cfg(test)]
mod tests;
