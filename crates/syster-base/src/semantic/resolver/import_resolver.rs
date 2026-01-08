use crate::semantic::resolver::Resolver;
use crate::semantic::symbol_table::Symbol;

impl<'a> Resolver<'a> {
    // ============================================================
    // Import Path Utilities
    // ============================================================

    /// Parse an import path into components (split by ::)
    pub fn parse_import_path(path: &str) -> Vec<String> {
        path.split("::").map(|s| s.to_string()).collect()
    }

    /// Check if an import is a wildcard import (ends with *)
    pub fn is_wildcard_import(path: &str) -> bool {
        path.ends_with("::*") || path == "*"
    }

    // ============================================================
    // Import Resolution (for expanding import statements)
    // ============================================================

    /// Resolve an import path to the symbols it brings into scope
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
                .symbol_table()
                .iter_symbols()
                .filter_map(|symbol| {
                    let qname = symbol.qualified_name();
                    if !qname.contains("::") {
                        Some(qname.to_string())
                    } else {
                        None
                    }
                })
                .collect();
        }

        let prefix = import_path.strip_suffix("::*").unwrap_or(import_path);

        self.symbol_table()
            .iter_symbols()
            .filter_map(|symbol| {
                let qname = symbol.qualified_name();
                if let Some(remainder) = qname.strip_prefix(prefix)
                    && let Some(remainder) = remainder.strip_prefix("::")
                    && !remainder.contains("::")
                {
                    return Some(qname.to_string());
                }
                None
            })
            .collect()
    }

    // ============================================================
    // Import-based Name Resolution
    // ============================================================

    /// Resolve a simple name via imports registered in a scope.
    /// Checks each import to see if it brings `name` into scope.
    pub(super) fn resolve_via_imports(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        self.resolve_via_imports_filtered(name, scope_id, false)
    }

    /// Resolve a simple name via public imports only (for re-export resolution).
    /// Uses direct symbol table lookup to avoid recursion.
    pub(super) fn resolve_via_public_imports(
        &self,
        name: &str,
        scope_id: usize,
    ) -> Option<&Symbol> {
        self.symbol_table()
            .get_scope_imports(scope_id)
            .iter()
            .filter(|import| import.is_public)
            .find_map(|import| {
                if import.is_namespace {
                    // For wildcard imports, try direct lookup only (no recursion)
                    let namespace = import.path.trim_end_matches("::*").trim_end_matches("::**");
                    let qualified = format!("{namespace}::{name}");
                    self.symbol_table().find_by_qualified_name(&qualified)
                } else {
                    // For direct imports, check if it imports this name
                    let imports_this_name =
                        import.path.ends_with(&format!("::{name}")) || import.path == name;
                    imports_this_name
                        .then(|| self.symbol_table().find_by_qualified_name(&import.path))?
                }
            })
    }

    /// Resolve a simple name via imports, optionally filtering to public imports only.
    fn resolve_via_imports_filtered(
        &self,
        name: &str,
        scope_id: usize,
        public_only: bool,
    ) -> Option<&Symbol> {
        self.symbol_table()
            .get_scope_imports(scope_id)
            .iter()
            .filter(|import| !public_only || import.is_public)
            .find_map(|import| {
                if import.is_namespace {
                    self.try_wildcard_import(name, &import.path, import.is_recursive)
                } else {
                    self.try_direct_import(name, &import.path)
                }
            })
    }

    /// Check if a wildcard import (`Pkg::*` or `Pkg::**`) provides `name`.
    fn try_wildcard_import(
        &self,
        name: &str,
        import_path: &str,
        is_recursive: bool,
    ) -> Option<&Symbol> {
        let namespace = import_path.trim_end_matches("::*").trim_end_matches("::**");
        let qualified = format!("{namespace}::{name}");

        self.resolve_qualified(&qualified)
            .or_else(|| is_recursive.then(|| self.search_nested_namespaces(name, namespace))?)
    }

    /// Check if a direct import (`Pkg::Member`) provides `name`.
    fn try_direct_import(&self, name: &str, import_path: &str) -> Option<&Symbol> {
        // Direct import "Pkg::Foo" makes "Foo" available
        let imports_this_name = import_path.ends_with(&format!("::{name}")) || import_path == name;

        imports_this_name.then(|| self.resolve_qualified(import_path))?
    }

    /// Search all nested namespaces under `namespace` for a symbol named `name`.
    /// Used for recursive imports (`Pkg::**`).
    fn search_nested_namespaces(&self, name: &str, namespace: &str) -> Option<&Symbol> {
        let prefix = format!("{namespace}::");
        let suffix = format!("::{name}");

        // Search through all symbols in the arena
        self.symbol_table().iter_symbols().find(|symbol| {
            let qname = symbol.qualified_name();
            qname.starts_with(&prefix) && qname.ends_with(&suffix)
        })
    }
}
