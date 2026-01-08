use crate::semantic::symbol_table::{Symbol, SymbolTable};

/// Resolver provides symbol resolution algorithms.
///
/// All resolution logic lives here, keeping SymbolTable as a pure data structure.
pub struct Resolver<'a> {
    symbol_table: &'a SymbolTable,
}

impl<'a> Resolver<'a> {
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self { symbol_table }
    }

    pub fn symbol_table(&self) -> &SymbolTable {
        self.symbol_table
    }

    // ============================================================
    // Primary Resolution API
    // ============================================================

    /// Resolve a name (qualified or simple) using current scope.
    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.resolve_qualified(name)
            .or_else(|| self.walk_scope_chain(name, self.symbol_table.current_scope_id()))
    }

    /// Resolve a name within a specific scope (for file-context-aware lookups).
    /// Handles both simple names and relative qualified names like `Inner::Type`.
    pub fn resolve_in_scope(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        // First try as a fully qualified name
        if let Some(symbol) = self.resolve_qualified(name) {
            return Some(symbol);
        }

        // For relative qualified names like "Inner::Vehicle", resolve the first segment
        // in the scope chain, then look up the rest as a suffix
        if let Some(colon_pos) = name.find("::") {
            let first_segment = &name[..colon_pos];
            let rest = &name[colon_pos + 2..];

            // Resolve the first segment in the scope chain
            if let Some(first_symbol) = self.walk_scope_chain(first_segment, scope_id) {
                // Build the full qualified name and resolve it
                let full_qualified = format!("{}::{}", first_symbol.qualified_name(), rest);
                return self.resolve_qualified(&full_qualified);
            }
        }

        // Fall back to simple name resolution via scope chain
        self.walk_scope_chain(name, scope_id)
    }

    /// Resolve a fully qualified name (e.g., "Package::Type").
    ///
    /// This first tries a direct lookup. If that fails and the name contains `::`,
    /// it also checks public re-exports from the target namespace.
    pub fn resolve_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
        // First, try direct lookup
        if let Some(symbol) = self.symbol_table.find_by_qualified_name(qualified_name) {
            return Some(symbol);
        }

        // If not found and contains "::", try resolving via public re-exports
        // e.g., "ISQ::MassValue" where ISQ has "public import ISQBase::*"
        if let Some(colon_pos) = qualified_name.rfind("::") {
            let namespace = &qualified_name[..colon_pos];
            let member_name = &qualified_name[colon_pos + 2..];

            // Look up the namespace symbol (e.g., ISQ)
            if let Some(ns_symbol) = self.symbol_table.find_by_qualified_name(namespace) {
                // The namespace symbol's scope_id is where it's *defined* (parent scope)
                // We need to check the *body* scope which is a child of the definition scope
                let definition_scope_id = ns_symbol.scope_id();

                // Check child scopes for public imports using the import resolver
                if let Some(scope) = self.symbol_table.scopes().get(definition_scope_id) {
                    for &child_scope_id in &scope.children {
                        if let Some(symbol) =
                            self.resolve_via_public_imports(member_name, child_scope_id)
                        {
                            return Some(symbol);
                        }
                    }
                }
            }
        }

        None
    }

    // ============================================================
    // Scope Chain Resolution
    // ============================================================

    /// Walk the scope chain looking for a symbol, checking imports at each level.
    fn walk_scope_chain(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        let mut current = scope_id;
        loop {
            // Direct lookup in scope
            if let Some(symbol) = self.symbol_table.get_symbol_in_scope(current, name) {
                return self.resolve_alias(symbol);
            }

            // Check imports in this scope (defined in import_resolver.rs)
            if let Some(symbol) = self.resolve_via_imports(name, current) {
                return self.resolve_alias(symbol);
            }

            // Walk to parent scope
            current = self.symbol_table.get_scope_parent(current)?;
        }
    }

    /// Walk scope chain without checking imports
    pub fn resolve_from_scope_direct(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        let mut current = scope_id;
        loop {
            if let Some(symbol) = self.symbol_table.get_symbol_in_scope(current, name) {
                return Some(symbol);
            }
            current = self.symbol_table.get_scope_parent(current)?;
        }
    }

    // ============================================================
    // Alias Resolution
    // ============================================================

    /// Resolve an alias to its target symbol.
    /// For non-alias symbols, returns the symbol itself.
    fn resolve_alias<'b>(&self, symbol: &'b Symbol) -> Option<&'b Symbol>
    where
        'a: 'b,
    {
        match symbol {
            // For aliases, resolve the target by qualified name
            Symbol::Alias { target, .. } => self.symbol_table.find_by_qualified_name(target),
            // For non-aliases, return the symbol directly
            _ => Some(symbol),
        }
    }
}
