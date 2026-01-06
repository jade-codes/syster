use super::symbol::Symbol;
use super::table::SymbolTable;

impl SymbolTable {
    // ============================================================
    // Mutable Lookups (required for population)
    // ============================================================

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let scope_chain = self.build_scope_chain(self.current_scope);
        self.find_in_scope_chain(name, &scope_chain)
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

    fn find_in_scope_chain(&mut self, name: &str, chain: &[usize]) -> Option<&mut Symbol> {
        for &scope_id in chain {
            if self.scopes[scope_id].symbols.contains_key(name) {
                return self.scopes[scope_id].symbols.get_mut(name);
            }
        }
        None
    }

    pub fn lookup_global_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        self.scopes
            .iter_mut()
            .find_map(|scope| scope.symbols.get_mut(name))
    }

    // ============================================================
    // Enumeration
    // ============================================================

    pub fn all_symbols(&self) -> Vec<(&String, &Symbol)> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.symbols.iter())
            .collect()
    }

    // ============================================================
    // File-based Operations
    // ============================================================

    pub fn remove_symbols_from_file(&mut self, file_path: &str) -> usize {
        self.symbols_by_file.remove(file_path);

        self.scopes
            .iter_mut()
            .map(|scope| {
                let before = scope.symbols.len();
                scope
                    .symbols
                    .retain(|_, symbol| symbol.source_file() != Some(file_path));
                before - scope.symbols.len()
            })
            .sum()
    }

    pub fn remove_imports_from_file(&mut self, file_path: &str) {
        self.imports_by_file.remove(file_path);

        for scope in &mut self.scopes {
            scope
                .imports
                .retain(|import| import.file.as_deref() != Some(file_path));
        }
    }
}
