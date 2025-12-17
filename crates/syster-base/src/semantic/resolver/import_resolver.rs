use crate::{
    semantic::resolver::name_resolver::Resolver,
    syntax::sysml::ast::{Element, SysMLFile},
};

impl<'a> Resolver<'a> {
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

    pub fn resolve_import(&self, import_path: &str) -> Vec<String> {
        if Self::is_wildcard_import(import_path) {
            self.resolve_wildcard_import(import_path)
        } else if self.resolve_qualified(import_path).is_some() {
            vec![import_path.to_string()]
        } else {
            vec![]
        }
    }

    fn resolve_wildcard_import(&self, import_path: &str) -> Vec<String> {
        if import_path == "*" {
            return self
                .symbol_table
                .all_symbols()
                .into_iter()
                .filter_map(|(_, symbol)| {
                    let qname = symbol.qualified_name();
                    if !qname.contains("::") {
                        Some(qname.to_string())
                    } else {
                        None
                    }
                })
                .collect();
        }

        // Remove trailing ::*
        let prefix = import_path.strip_suffix("::*").unwrap_or(import_path);

        // Find all direct children of the prefix
        self.symbol_table
            .all_symbols()
            .into_iter()
            .filter_map(|(_, symbol)| {
                let qname = symbol.qualified_name();

                // Check if this symbol is a direct child of prefix
                if let Some(remainder) = qname.strip_prefix(prefix)
                    && let Some(remainder) = remainder.strip_prefix("::")
                {
                    // Only include direct children (no nested ::)
                    if !remainder.contains("::") {
                        return Some(qname.to_string());
                    }
                }
                None
            })
            .collect()
    }
}
