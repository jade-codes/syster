use super::symbol::{Symbol, SymbolId};
use super::table::SymbolTable;
use crate::semantic::types::normalize_path;

impl SymbolTable {
    // ============================================================
    // Mutable Lookups (required for population)
    // ============================================================

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let scope_chain = self.build_scope_chain(self.current_scope);
        self.find_in_scope_chain_mut(name, &scope_chain)
    }

    fn build_scope_chain(&self, scope_id: usize) -> Vec<usize> {
        let mut chain = Vec::new();
        let mut current = scope_id;
        loop {
            chain.push(current);
            current = match self.scopes[current].parent {
                Some(parent) => parent,
                None => break,
            };
        }
        chain
    }

    fn find_in_scope_chain_mut(&mut self, name: &str, chain: &[usize]) -> Option<&mut Symbol> {
        for &scope_id in chain {
            if let Some(id) = self.scopes[scope_id].symbols.get(name).copied() {
                return self.arena.get_mut(id.index());
            }
        }
        None
    }

    pub fn lookup_global_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        for scope in &self.scopes {
            if let Some(id) = scope.symbols.get(name).copied() {
                return self.arena.get_mut(id.index());
            }
        }
        None
    }

    // ============================================================
    // Enumeration
    // ============================================================

    /// Returns the count of active symbols (O(1))
    pub fn symbol_count(&self) -> usize {
        self.symbols_by_qname.len()
    }

    /// Returns an iterator over all active symbols (lazy, no allocation)
    pub fn iter_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols_by_qname
            .values()
            .filter_map(|id| self.arena.get(id.index()))
    }

    /// Returns all symbols as a Vec (for backward compatibility, prefer iter_symbols())
    #[deprecated(note = "Use iter_symbols() for iteration or targeted index methods")]
    pub fn all_symbols(&self) -> Vec<&Symbol> {
        self.iter_symbols().collect()
    }

    // ============================================================
    // File-based Operations
    // ============================================================

    pub fn remove_symbols_from_file(&mut self, file_path: &str) -> usize {
        let normalized = normalize_path(file_path);

        // Get the SymbolIds to remove
        let ids_to_remove: Vec<SymbolId> = self
            .symbols_by_file
            .get(&normalized)
            .cloned()
            .unwrap_or_default();

        if ids_to_remove.is_empty() {
            return 0;
        }

        // Remove from qname index
        for id in &ids_to_remove {
            if let Some(symbol) = self.arena.get(id.index()) {
                self.symbols_by_qname.remove(symbol.qualified_name());
            }
        }

        // Remove from scope symbol maps
        for scope in &mut self.scopes {
            scope.symbols.retain(|_, id| !ids_to_remove.contains(id));
        }

        // Remove from file index
        self.symbols_by_file.remove(&normalized);

        // Note: We don't actually remove from arena (would invalidate other IDs)
        // The symbols become orphaned but unreachable. This is a trade-off.
        // For a proper solution, we'd need tombstones or a generational arena.

        ids_to_remove.len()
    }

    pub fn remove_imports_from_file(&mut self, file_path: &str) {
        let normalized = normalize_path(file_path);
        self.imports_by_file.remove(&normalized);

        for scope in &mut self.scopes {
            scope
                .imports
                .retain(|import| import.file.as_deref() != Some(file_path));
        }
    }
}
