use super::symbol::Symbol;
use super::table::SymbolTable;

impl SymbolTable {
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current = self.current_scope;
        loop {
            if let Some(symbol) = self.scopes[current].symbols.get(name) {
                return self.resolve_alias(symbol);
            }

            if let Some(symbol) = self.lookup_in_imports(name, current) {
                return self.resolve_alias(symbol);
            }

            current = self.scopes[current].parent?;
        }
    }

    fn resolve_alias<'a>(&'a self, symbol: &'a Symbol) -> Option<&'a Symbol> {
        match symbol {
            Symbol::Alias { target, .. } => self.lookup(target),
            _ => Some(symbol),
        }
    }

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

    pub(super) fn lookup_in_imports(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        self.scopes[scope_id].imports.iter().find_map(|import| {
            if import.is_namespace {
                self.lookup_namespace_import(name, &import.path, import.is_recursive)
            } else {
                self.lookup_member_import(name, &import.path)
            }
        })
    }

    fn lookup_namespace_import(
        &self,
        name: &str,
        import_path: &str,
        is_recursive: bool,
    ) -> Option<&Symbol> {
        let namespace = import_path.trim_end_matches("::*").trim_end_matches("::**");
        let qualified = format!("{namespace}::{name}");

        self.find_by_qualified(&qualified)
            .or_else(|| is_recursive.then(|| self.lookup_recursive_import(name, namespace))?)
    }

    fn lookup_member_import(&self, name: &str, import_path: &str) -> Option<&Symbol> {
        (import_path.ends_with(&format!("::{name}")) || import_path == name)
            .then(|| self.find_by_qualified(import_path))?
    }

    fn find_by_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
        self.scopes
            .iter()
            .find_map(|scope| scope.symbols.get(qualified_name))
    }

    fn lookup_recursive_import(&self, name: &str, namespace: &str) -> Option<&Symbol> {
        let prefix = format!("{namespace}::");
        let suffix = format!("::{name}");

        self.scopes.iter().find_map(|scope| {
            scope
                .symbols
                .iter()
                .find(|(qname, _)| qname.starts_with(&prefix) && qname.ends_with(&suffix))
                .map(|(_, symbol)| symbol)
        })
    }

    pub fn lookup_from_scope(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        let mut current = scope_id;
        loop {
            if let Some(symbol) = self.scopes[current].symbols.get(name) {
                return Some(symbol);
            }
            current = self.scopes[current].parent?;
        }
    }

    pub fn lookup_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
        self.scopes.iter().find_map(|scope| {
            scope
                .symbols
                .values()
                .find(|symbol| symbol.qualified_name() == qualified_name)
        })
    }

    pub fn all_symbols(&self) -> Vec<(&String, &Symbol)> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.symbols.iter())
            .collect()
    }

    pub fn remove_symbols_from_file(&mut self, file_path: &str) -> usize {
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
}
