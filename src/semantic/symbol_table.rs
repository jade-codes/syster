use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Package {
        name: String,
        qualified_name: String,
        scope_id: usize,
    },
    Classifier {
        name: String,
        qualified_name: String,
        kind: String,
        is_abstract: bool,
        scope_id: usize,
    },
    Feature {
        name: String,
        qualified_name: String,
        feature_type: Option<String>,
        scope_id: usize,
    },
    Definition {
        name: String,
        qualified_name: String,
        kind: String,
        scope_id: usize,
    },
    Usage {
        name: String,
        qualified_name: String,
        kind: String,
        scope_id: usize,
    },
}

impl Symbol {
    /// Returns the qualified name of this symbol
    pub fn qualified_name(&self) -> &str {
        match self {
            Symbol::Package { qualified_name, .. }
            | Symbol::Classifier { qualified_name, .. }
            | Symbol::Feature { qualified_name, .. }
            | Symbol::Definition { qualified_name, .. }
            | Symbol::Usage { qualified_name, .. } => qualified_name,
        }
    }

    /// Returns the simple name of this symbol
    pub fn name(&self) -> &str {
        match self {
            Symbol::Package { name, .. }
            | Symbol::Classifier { name, .. }
            | Symbol::Feature { name, .. }
            | Symbol::Definition { name, .. }
            | Symbol::Usage { name, .. } => name,
        }
    }

    /// Returns the scope ID where this symbol was defined
    pub fn scope_id(&self) -> usize {
        match self {
            Symbol::Package { scope_id, .. }
            | Symbol::Classifier { scope_id, .. }
            | Symbol::Feature { scope_id, .. }
            | Symbol::Definition { scope_id, .. }
            | Symbol::Usage { scope_id, .. } => *scope_id,
        }
    }

    /// Returns true if this symbol can be used as a type
    pub fn is_type(&self) -> bool {
        matches!(self, Symbol::Classifier { .. } | Symbol::Definition { .. })
    }

    /// Returns the type reference for Features that have one
    pub fn type_reference(&self) -> Option<&str> {
        match self {
            Symbol::Feature { feature_type, .. } => feature_type.as_deref(),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Scope {
    parent: Option<usize>,
    symbols: HashMap<String, Symbol>,
    children: Vec<usize>,
}

impl Scope {
    fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(None)],
            current_scope: 0,
        }
    }

    pub fn enter_scope(&mut self) -> usize {
        let parent = self.current_scope;
        let scope_id = self.scopes.len();
        self.scopes.push(Scope::new(Some(parent)));
        self.scopes[parent].children.push(scope_id);
        self.current_scope = scope_id;
        scope_id
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    /// Inserts a symbol into the current scope.
    ///
    /// # Errors
    ///
    /// Returns an error if a symbol with the same name already exists in the current scope.
    pub fn insert(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        let scope = &mut self.scopes[self.current_scope];
        if scope.symbols.contains_key(&name) {
            return Err(format!("Symbol '{}' already defined in this scope", name));
        }
        scope.symbols.insert(name, symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current = self.current_scope;
        loop {
            if let Some(symbol) = self.scopes[current].symbols.get(name) {
                return Some(symbol);
            }
            match self.scopes[current].parent {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    pub fn lookup_from_scope(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        let mut current = scope_id;
        loop {
            if let Some(symbol) = self.scopes[current].symbols.get(name) {
                return Some(symbol);
            }
            match self.scopes[current].parent {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.scopes[self.current_scope].symbols.get(name)
    }

    pub fn current_scope_id(&self) -> usize {
        self.current_scope
    }

    pub fn symbols_in_scope(&self, scope_id: usize) -> Option<&HashMap<String, Symbol>> {
        self.scopes.get(scope_id).map(|s| &s.symbols)
    }

    pub fn all_symbols(&self) -> Vec<(&String, &Symbol)> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.symbols.iter())
            .collect()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "symbol_table/tests.rs"]
mod tests;
