//! # Name Resolver
//!
//! Resolves symbol names to their definitions in the symbol table, supporting both
//! simple names and qualified (multi-part) names.
//!
//! ## Resolution Algorithm
//!
//! ### Simple Names
//! Simple names (e.g., `Vehicle`) are looked up directly in the symbol table:
//! 1. Check current scope
//! 2. Walk up scope chain to parent scopes
//! 3. Check imported symbols
//!
//! ### Qualified Names
//! Qualified names (e.g., `Automotive::Engine::V8`) are resolved step-by-step:
//! 1. Look up first component (`Automotive`)
//! 2. Enter that symbol's scope
//! 3. Look up next component (`Engine`) within that scope
//! 4. Repeat until all components resolved
//!
//! ## Example
//!
//! ```rust
//! use syster::semantic::{NameResolver, SymbolTable};
//!
//! let symbol_table = SymbolTable::new();
//! let resolver = NameResolver::new(&symbol_table);
//!
//! // Simple lookup
//! let symbol = resolver.resolve("Vehicle");
//!
//! // Qualified lookup
//! let symbol = resolver.resolve_qualified("Automotive::Engine::V8");
//! ```
//!
//! ## Relationship to Import System
//!
//! Name resolution is affected by import statements:
//! - `import Package::*` makes all members of Package visible
//! - `import Package::Member` makes specific member visible
//! - Aliases (`import X as Y`) are resolved during lookup
//!
//! See [Import Resolution](../../docs/SEMANTIC_ANALYSIS.md#import-resolution) for details.

use crate::semantic::import_extractor::is_wildcard_import;
use crate::semantic::symbol_table::{Symbol, SymbolTable};

pub struct NameResolver<'a> {
    symbol_table: &'a SymbolTable,
}

impl<'a> NameResolver<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        if name.contains("::") {
            self.resolve_qualified(name)
        } else {
            self.symbol_table.lookup(name)
        }
    }

    pub fn resolve_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
        let parts: Vec<&str> = qualified_name.split("::").collect();
        if parts.is_empty() {
            return None;
        }

        let mut current_symbol = self.symbol_table.lookup(parts[0])?;

        for part in parts.iter().skip(1) {
            match current_symbol {
                Symbol::Package { qualified_name, .. } => {
                    let next_name = format!("{}::{}", qualified_name, part);
                    current_symbol = self.find_in_scope(&next_name)?;
                }
                Symbol::Classifier { qualified_name, .. } => {
                    let next_name = format!("{}::{}", qualified_name, part);
                    current_symbol = self.find_in_scope(&next_name)?;
                }
                _ => return None,
            }
        }

        Some(current_symbol)
    }

    fn find_in_scope(&self, qualified_name: &str) -> Option<&Symbol> {
        for (_, symbol) in self.symbol_table.all_symbols() {
            let symbol_qname = match symbol {
                Symbol::Package { qualified_name, .. } => qualified_name,
                Symbol::Classifier { qualified_name, .. } => qualified_name,
                Symbol::Feature { qualified_name, .. } => qualified_name,
                Symbol::Definition { qualified_name, .. } => qualified_name,
                Symbol::Usage { qualified_name, .. } => qualified_name,
                Symbol::Alias { qualified_name, .. } => qualified_name,
            };
            if symbol_qname == qualified_name {
                return Some(symbol);
            }
        }
        None
    }

    /// Resolves an import path to determine what symbols should be made visible
    ///
    /// Handles three types of imports:
    /// - Wildcard namespace: `Package::*` - makes all direct members visible
    /// - Specific member: `Package::Member` - makes only that member visible
    /// - Recursive wildcard: `Package::*::**` - makes all nested members visible (future)
    ///
    /// # Returns
    ///
    /// Returns a list of qualified names that should be imported into the current scope
    pub fn resolve_import(&self, import_path: &str) -> Vec<String> {
        if is_wildcard_import(import_path) {
            // Wildcard import: Package::* or *
            self.resolve_wildcard_import(import_path)
        } else {
            // Specific member import: Package::Member
            if self.resolve_qualified(import_path).is_some() {
                vec![import_path.to_string()]
            } else {
                vec![]
            }
        }
    }

    /// Resolves a wildcard import to all matching symbols
    ///
    /// For `Package::*`, returns all symbols whose qualified name starts with `Package::`
    /// and has no additional `::` separators (direct children only)
    fn resolve_wildcard_import(&self, import_path: &str) -> Vec<String> {
        // Handle bare * wildcard
        if import_path == "*" {
            return self
                .symbol_table
                .all_symbols()
                .into_iter()
                .filter_map(|(_, symbol)| {
                    let qname = self.get_qualified_name(symbol);
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
                let qname = self.get_qualified_name(symbol);

                // Check if this symbol is a direct child of prefix
                if let Some(remainder) = qname.strip_prefix(prefix) {
                    if let Some(remainder) = remainder.strip_prefix("::") {
                        // Only include direct children (no nested ::)
                        if !remainder.contains("::") {
                            return Some(qname.to_string());
                        }
                    }
                }
                None
            })
            .collect()
    }

    /// Helper to extract qualified name from any Symbol variant
    fn get_qualified_name<'b>(&self, symbol: &'b Symbol) -> &'b str {
        match symbol {
            Symbol::Package { qualified_name, .. } => qualified_name,
            Symbol::Classifier { qualified_name, .. } => qualified_name,
            Symbol::Feature { qualified_name, .. } => qualified_name,
            Symbol::Definition { qualified_name, .. } => qualified_name,
            Symbol::Usage { qualified_name, .. } => qualified_name,
            Symbol::Alias { qualified_name, .. } => qualified_name,
        }
    }
}

#[cfg(test)]
#[path = "resolver/tests.rs"]
mod tests;
