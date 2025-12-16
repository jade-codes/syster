//! # Symbol Table
//!
//! Central registry of all named elements in a SysML/KerML model, providing fast lookup
//! and cross-file symbol resolution.
//!
//! ## Design Principles
//!
//! 1. **Qualified Names**: Every symbol has a unique qualified name (e.g., `Package::Class::Feature`)
//! 2. **Scope Hierarchy**: Symbols track their parent scope via `scope_id`
//! 3. **Source Tracking**: Each symbol records which file it came from
//! 4. **Import Support**: Symbol visibility affected by import statements
//!
//! ## Symbol Types
//!
//! The `Symbol` enum represents all named elements:
//!
//! - **Package**: Namespace container (e.g., `package Automotive { }`)
//! - **Classifier**: KerML types (class, struct, datatype, association, etc.)
//! - **Feature**: Properties and operations of classifiers
//! - **Definition**: SysML type definitions (part def, port def, action def, etc.)
//! - **Usage**: SysML instances (part, port, action, etc.)
//! - **Alias**: Import aliases (`import X as Y`)
//!
//! ## Qualified Names
//!
//! Qualified names uniquely identify symbols across files:
//!
//! ```text
//! package Automotive {
//!     package Engine {
//!         part def V8 {
//!             feature cylinders: Integer;
//!         }
//!     }
//! }
//!
//! Qualified names:
//! - "Automotive"                        (Package)
//! - "Automotive::Engine"                (Package)
//! - "Automotive::Engine::V8"            (Definition)
//! - "Automotive::Engine::V8::cylinders" (Feature)
//! ```
//!
//! ## Scope Hierarchy
//!
//! Scopes represent nested contexts (packages, classifiers):
//!
//! ```text
//! Scope 0 (root)
//!   └─ Scope 1 (package A)
//!       └─ Scope 2 (classifier B)
//!           └─ Scope 3 (feature c)
//! ```
//!
//! Name lookup walks the scope chain from innermost to outermost.
//!
//! ## Import System
//!
//! The `Import` struct tracks import statements, which are resolved in three passes:
//!
//! 1. **Namespace imports** (`Package::*`) - Import all members
//! 2. **Member imports** (`Package::Member`) - Import specific member
//! 3. **Recursive imports** (`Package::*::**`) - Import all nested members
//!
//! See [Import Resolution](../../docs/SEMANTIC_ANALYSIS.md#import-resolution) for algorithm.
//!
//! ## Usage Example
//!
//! ```rust
//! use syster::semantic::symbol_table::{Symbol, SymbolTable};
//!
//! let mut table = SymbolTable::new();
//!
//! // Insert a symbol
//! table.insert(
//!     "Automotive".to_string(),
//!     Symbol::Package {
//!         name: "Automotive".to_string(),
//!         qualified_name: "Automotive".to_string(),
//!         scope_id: 0,
//!         source_file: Some("auto.sysml".to_string()),
//!         span: None,
//!         references: Vec::new(),
//!     },
//! ).ok();
//!
//! // Lookup by name
//! let symbol = table.lookup("Automotive");
//! ```
use crate::core::Span;
use crate::core::events::EventEmitter;
use crate::core::operation::{EventBus, OperationResult};
use crate::semantic::events::SymbolTableEvent;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub is_recursive: bool,
    pub is_namespace: bool, // true for ::*, false for specific member
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Package {
        name: String,
        qualified_name: String,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<Span>,
    },
    Classifier {
        name: String,
        qualified_name: String,
        kind: String,
        is_abstract: bool,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<Span>,
    },
    Feature {
        name: String,
        qualified_name: String,
        feature_type: Option<String>,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<Span>,
    },
    Definition {
        name: String,
        qualified_name: String,
        kind: String,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<Span>,
    },
    Usage {
        name: String,
        qualified_name: String,
        kind: String,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<Span>,
    },
    Alias {
        name: String,
        qualified_name: String,
        target: String,
        scope_id: usize,
        source_file: Option<String>,
        span: Option<Span>,
        references: Vec<Span>,
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
            | Symbol::Usage { qualified_name, .. }
            | Symbol::Alias { qualified_name, .. } => qualified_name,
        }
    }

    /// Returns the simple name of this symbol
    pub fn name(&self) -> &str {
        match self {
            Symbol::Package { name, .. }
            | Symbol::Classifier { name, .. }
            | Symbol::Feature { name, .. }
            | Symbol::Definition { name, .. }
            | Symbol::Usage { name, .. }
            | Symbol::Alias { name, .. } => name,
        }
    }

    /// Returns the scope ID where this symbol was defined
    pub fn scope_id(&self) -> usize {
        match self {
            Symbol::Package { scope_id, .. }
            | Symbol::Classifier { scope_id, .. }
            | Symbol::Feature { scope_id, .. }
            | Symbol::Definition { scope_id, .. }
            | Symbol::Usage { scope_id, .. }
            | Symbol::Alias { scope_id, .. } => *scope_id,
        }
    }

    /// Returns the source file path where this symbol was defined
    pub fn source_file(&self) -> Option<&str> {
        match self {
            Symbol::Package { source_file, .. }
            | Symbol::Classifier { source_file, .. }
            | Symbol::Feature { source_file, .. }
            | Symbol::Definition { source_file, .. }
            | Symbol::Usage { source_file, .. }
            | Symbol::Alias { source_file, .. } => source_file.as_deref(),
        }
    }

    /// Returns the source span where this symbol was defined
    pub fn span(&self) -> Option<Span> {
        match self {
            Symbol::Package { span, .. }
            | Symbol::Classifier { span, .. }
            | Symbol::Feature { span, .. }
            | Symbol::Definition { span, .. }
            | Symbol::Usage { span, .. }
            | Symbol::Alias { span, .. } => *span,
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

    /// Returns all reference locations for this symbol
    pub fn references(&self) -> &[Span] {
        match self {
            Symbol::Package { references, .. }
            | Symbol::Classifier { references, .. }
            | Symbol::Feature { references, .. }
            | Symbol::Definition { references, .. }
            | Symbol::Usage { references, .. }
            | Symbol::Alias { references, .. } => references,
        }
    }

    /// Adds a reference location to this symbol (mutable access required)
    pub fn add_reference(&mut self, span: Span) {
        match self {
            Symbol::Package { references, .. }
            | Symbol::Classifier { references, .. }
            | Symbol::Feature { references, .. }
            | Symbol::Definition { references, .. }
            | Symbol::Usage { references, .. }
            | Symbol::Alias { references, .. } => references.push(span),
        }
    }
}

#[derive(Debug)]
struct Scope {
    parent: Option<usize>,
    symbols: HashMap<String, Symbol>,
    children: Vec<usize>,
    imports: Vec<Import>,
}

impl Scope {
    fn new(parent: Option<usize>) -> Self {
        Self {
            parent,
            symbols: HashMap::new(),
            children: Vec::new(),
            imports: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
    current_file: Option<String>,
    pub events: EventEmitter<SymbolTableEvent, SymbolTable>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(None)],
            current_scope: 0,
            current_file: None,
            events: EventEmitter::new(),
        }
    }

    /// Sets the current source file context for subsequently created symbols
    pub fn set_current_file(&mut self, file_path: Option<String>) {
        let _ = {
            self.current_file = file_path.clone();
            let event = file_path.map(|path| SymbolTableEvent::FileChanged { file_path: path });
            OperationResult::<(), String, SymbolTableEvent>::success((), event)
        }
        .publish(self);
    }

    /// Gets the current source file context
    pub fn current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
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
        {
            let qualified_name = symbol.qualified_name().to_string();
            let symbol_id = self.scopes.iter().map(|s| s.symbols.len()).sum::<usize>();

            let scope = &mut self.scopes[self.current_scope];
            if scope.symbols.contains_key(&name) {
                return OperationResult::failure(format!(
                    "Symbol '{}' already defined in this scope",
                    name
                ))
                .publish(self);
            }

            scope.symbols.insert(name, symbol);

            let event = SymbolTableEvent::SymbolInserted {
                qualified_name,
                symbol_id,
            };
            OperationResult::success((), Some(event))
        }
        .publish(self)
    }

    /// Adds an import to the current scope
    pub fn add_import(&mut self, path: String, is_recursive: bool) {
        let _ = {
            let is_namespace = path.ends_with("::*") || path.ends_with("::**");
            let import = Import {
                path: path.clone(),
                is_recursive,
                is_namespace,
            };
            self.scopes[self.current_scope].imports.push(import);

            let event = SymbolTableEvent::ImportAdded { import_path: path };
            OperationResult::<(), String, SymbolTableEvent>::success((), Some(event))
        }
        .publish(self);
    }
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        let mut current = self.current_scope;
        loop {
            // First check local symbols
            if let Some(symbol) = self.scopes[current].symbols.get(name) {
                // If it's an alias, resolve it
                if let Symbol::Alias { target, .. } = symbol {
                    return self.lookup(target);
                }
                return Some(symbol);
            }

            // Then check imported namespaces
            if let Some(symbol) = self.lookup_in_imports(name, current) {
                // If it's an alias, resolve it
                if let Symbol::Alias { target, .. } = symbol {
                    return self.lookup(target);
                }
                return Some(symbol);
            }

            match self.scopes[current].parent {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    /// Looks up a symbol in imported namespaces from a given scope
    fn lookup_in_imports(&self, name: &str, scope_id: usize) -> Option<&Symbol> {
        for import in &self.scopes[scope_id].imports {
            if import.is_namespace {
                // For namespace imports (::*), strip the ::* and look for name in that namespace
                let namespace = import.path.trim_end_matches("::*").trim_end_matches("::**");
                let qualified = format!("{}::{}", namespace, name);

                // Look through all scopes for the qualified name
                for scope in &self.scopes {
                    if let Some(symbol) = scope.symbols.get(&qualified) {
                        return Some(symbol);
                    }
                }

                // If recursive, also check nested namespaces
                if import.is_recursive
                    && let Some(symbol) = self.lookup_recursive_import(name, namespace)
                {
                    return Some(symbol);
                }
            } else {
                // For member imports, look for the exact path
                for scope in &self.scopes {
                    if let Some(symbol) = scope.symbols.get(&import.path) {
                        // Check if the name matches the last component of the path
                        if import.path.ends_with(&format!("::{}", name)) || import.path == name {
                            return Some(symbol);
                        }
                    }
                }
            }
        }
        None
    }

    /// Recursively searches for a symbol in nested namespaces
    fn lookup_recursive_import(&self, name: &str, namespace: &str) -> Option<&Symbol> {
        // Look for name in any sub-namespace of the given namespace
        let prefix = format!("{}::", namespace);
        for scope in &self.scopes {
            for (qualified_name, symbol) in &scope.symbols {
                if qualified_name.starts_with(&prefix)
                    && qualified_name.ends_with(&format!("::{}", name))
                {
                    return Some(symbol);
                }
            }
        }
        None
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

    pub fn current_scope_id(&self) -> usize {
        self.current_scope
    }

    pub fn all_symbols(&self) -> Vec<(&String, &Symbol)> {
        self.scopes
            .iter()
            .flat_map(|scope| scope.symbols.iter())
            .collect()
    }

    /// Removes all symbols from a specific source file
    ///
    /// Returns the number of symbols removed.
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

    /// Looks up a symbol by its qualified name across all scopes
    pub fn lookup_qualified(&self, qualified_name: &str) -> Option<&Symbol> {
        for scope in &self.scopes {
            for symbol in scope.symbols.values() {
                if symbol.qualified_name() == qualified_name {
                    return Some(symbol);
                }
            }
        }
        None
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus<SymbolTableEvent> for SymbolTable {
    fn publish(&mut self, event: &SymbolTableEvent) {
        let emitter = std::mem::take(&mut self.events);
        self.events = emitter.emit(event.clone(), self);
    }
}

#[cfg(test)]
#[path = "symbol_table/tests.rs"]
mod tests;
