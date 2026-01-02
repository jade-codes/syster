use crate::semantic::symbol_table::{Symbol, SymbolTable};

pub struct Resolver<'a> {
    pub(super) symbol_table: &'a SymbolTable,
}

impl<'a> Resolver<'a> {
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
                    let next_name = format!("{qualified_name}::{part}");
                    current_symbol = self.find_in_scope(&next_name)?;
                }
                Symbol::Classifier { qualified_name, .. } => {
                    let next_name = format!("{qualified_name}::{part}");
                    current_symbol = self.find_in_scope(&next_name)?;
                }
                _ => return None,
            }
        }

        Some(current_symbol)
    }

    pub(super) fn find_in_scope(&self, qualified_name: &str) -> Option<&Symbol> {
        for (_, symbol) in self.symbol_table.all_symbols() {
            let symbol_qname = match symbol {
                Symbol::Package { qualified_name, .. }
                | Symbol::Classifier { qualified_name, .. }
                | Symbol::Feature { qualified_name, .. }
                | Symbol::Definition { qualified_name, .. }
                | Symbol::Usage { qualified_name, .. }
                | Symbol::Alias { qualified_name, .. }
                | Symbol::Import { qualified_name, .. } => qualified_name,
            };
            if symbol_qname == qualified_name {
                return Some(symbol);
            }
        }
        None
    }
}
