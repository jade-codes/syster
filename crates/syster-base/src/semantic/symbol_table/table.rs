use std::collections::HashMap;

use crate::core::Span;
use crate::core::events::EventEmitter;
use crate::core::operation::{EventBus, OperationResult};
use crate::semantic::SymbolTableEvent;
use crate::semantic::types::normalize_path;

use super::scope::{Import, Scope};
use super::symbol::Symbol;

use super::symbol::SymbolId;

pub struct SymbolTable {
    /// Arena storage for all symbols - single source of truth
    pub(super) arena: Vec<Symbol>,
    pub(super) scopes: Vec<Scope>,
    pub(super) current_scope: usize,
    current_file: Option<String>,
    pub events: EventEmitter<SymbolTableEvent, SymbolTable>,
    /// Index mapping file paths to SymbolIds of symbols defined in that file
    pub(super) symbols_by_file: HashMap<String, Vec<SymbolId>>,
    /// Index mapping file paths to imports originating from that file
    pub(super) imports_by_file: HashMap<String, Vec<Import>>,
    /// Index for O(1) qualified name lookups: qname -> SymbolId
    pub(super) symbols_by_qname: HashMap<String, SymbolId>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            arena: Vec::new(),
            scopes: vec![Scope::new(None)],
            current_scope: 0,
            current_file: None,
            events: EventEmitter::new(),
            symbols_by_file: HashMap::new(),
            imports_by_file: HashMap::new(),
            symbols_by_qname: HashMap::new(),
        }
    }

    pub fn set_current_file(&mut self, file_path: Option<String>) {
        let _ = {
            self.current_file = file_path.clone();
            let event = file_path.map(|path| SymbolTableEvent::FileChanged { file_path: path });
            OperationResult::<(), String, SymbolTableEvent>::success((), event)
        }
        .publish(self);
    }

    pub fn current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
    }

    pub fn get_current_scope(&self) -> usize {
        self.current_scope
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

    pub fn insert(&mut self, name: String, symbol: Symbol) -> Result<(), String> {
        {
            let qualified_name = symbol.qualified_name().to_string();
            let source_file = symbol.source_file().map(normalize_path);

            // Check for duplicate in scope
            let scope = &self.scopes[self.current_scope];
            if scope.symbols.contains_key(&name) {
                return OperationResult::failure(format!(
                    "Symbol '{name}' already defined in this scope"
                ))
                .publish(self);
            }

            // Add symbol to arena and get its ID
            let symbol_id = SymbolId::new(self.arena.len());
            self.arena.push(symbol);

            // Store SymbolId in scope
            self.scopes[self.current_scope]
                .symbols
                .insert(name, symbol_id);

            // Update the qname -> SymbolId index for O(1) lookup
            self.symbols_by_qname
                .insert(qualified_name.clone(), symbol_id);

            // Update the file -> SymbolIds index (using normalized path)
            if let Some(file_path) = source_file {
                self.symbols_by_file
                    .entry(file_path)
                    .or_default()
                    .push(symbol_id);
            }

            let event = SymbolTableEvent::SymbolInserted {
                qualified_name,
                symbol_id: symbol_id.index(),
            };
            OperationResult::success((), Some(event))
        }
        .publish(self)
    }

    pub fn add_import(
        &mut self,
        path: String,
        is_recursive: bool,
        is_public: bool,
        span: Option<crate::core::Span>,
        file: Option<String>,
    ) {
        let _ = {
            let is_namespace = path.ends_with("::*") || path.ends_with("::**");
            let import = Import {
                path: path.clone(),
                is_recursive,
                is_namespace,
                is_public,
                span,
                file: file.clone(),
            };
            self.scopes[self.current_scope].imports.push(import.clone());

            // Update the file -> imports index (using normalized path)
            if let Some(file_path) = file {
                let normalized = normalize_path(&file_path);
                self.imports_by_file
                    .entry(normalized)
                    .or_default()
                    .push(import);
            }

            let event = SymbolTableEvent::ImportAdded { import_path: path };
            OperationResult::<(), String, SymbolTableEvent>::success((), Some(event))
        }
        .publish(self);
    }

    pub fn current_scope_id(&self) -> usize {
        self.current_scope
    }

    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    // ============================================================
    // Data Access Methods (for Resolver)
    // ============================================================

    /// Get read-only access to all scopes.
    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }

    /// Get a symbol directly from a specific scope (no chain walking).
    pub fn get_symbol_in_scope(&self, scope_id: usize, name: &str) -> Option<&Symbol> {
        let id = self.scopes.get(scope_id)?.symbols.get(name)?;
        self.get_symbol(*id)
    }

    /// Get a SymbolId from a specific scope
    pub fn get_symbol_id_in_scope(&self, scope_id: usize, name: &str) -> Option<SymbolId> {
        self.scopes.get(scope_id)?.symbols.get(name).copied()
    }

    /// Get the parent of a scope.
    pub fn get_scope_parent(&self, scope_id: usize) -> Option<usize> {
        self.scopes.get(scope_id)?.parent
    }

    /// Get the scope ID for a file that contains its imports.
    ///
    /// This returns the scope where imports are registered for the given file,
    /// which is typically the scope of the top-level package body.
    /// Falls back to the scope of the first symbol if no imports are found.
    pub fn get_scope_for_file(&self, file_path: &str) -> Option<usize> {
        // First, try to find a scope with imports from this file
        for (scope_id, scope) in self.scopes.iter().enumerate() {
            if scope
                .imports
                .iter()
                .any(|import| import.file.as_deref() == Some(file_path))
            {
                return Some(scope_id);
            }
        }

        // Fall back to the scope of the first symbol defined in the file
        self.get_symbols_for_file(file_path)
            .first()
            .map(|symbol| symbol.scope_id())
    }

    pub fn get_scope_imports(&self, scope_id: usize) -> Vec<super::scope::Import> {
        self.scopes
            .get(scope_id)
            .map(|scope| scope.imports.clone())
            .unwrap_or_default()
    }

    /// Get all imports from a specific file
    ///
    /// Returns a vector of (import_path, span) tuples for all imports in the given file.
    /// Uses an internal index for O(1) file lookup. Paths are normalized.
    pub fn get_file_imports(&self, file_path: &str) -> Vec<(String, Span)> {
        let normalized = normalize_path(file_path);
        self.imports_by_file
            .get(&normalized)
            .into_iter()
            .flatten()
            .filter_map(|import| import.span.map(|span| (import.path.clone(), span)))
            .collect()
    }

    /// Get all symbols defined in a specific file
    ///
    /// Returns a Vec of symbols whose source_file matches the given path.
    /// Uses an internal index for O(1) file lookup instead of iterating all symbols.
    /// Paths are normalized before lookup (handles stdlib path variations).
    pub fn get_symbols_for_file(&self, file_path: &str) -> Vec<&Symbol> {
        let normalized = normalize_path(file_path);
        self.symbols_by_file
            .get(&normalized)
            .into_iter()
            .flatten()
            .filter_map(|id| self.get_symbol(*id))
            .collect()
    }

    /// Get a symbol by its SymbolId (O(1) arena lookup)
    pub fn get_symbol(&self, id: SymbolId) -> Option<&Symbol> {
        self.arena.get(id.index())
    }

    /// Get a mutable symbol by its SymbolId (O(1) arena lookup)
    pub fn get_symbol_mut(&mut self, id: SymbolId) -> Option<&mut Symbol> {
        self.arena.get_mut(id.index())
    }

    /// Find a symbol by its exact qualified name (data access, not resolution)
    /// Uses O(1) index lookup.
    pub fn find_by_qualified_name(&self, qualified_name: &str) -> Option<&Symbol> {
        let id = self.symbols_by_qname.get(qualified_name)?;
        self.get_symbol(*id)
    }

    /// Find a SymbolId by qualified name (for callers that need the ID)
    pub fn find_id_by_qualified_name(&self, qualified_name: &str) -> Option<SymbolId> {
        self.symbols_by_qname.get(qualified_name).copied()
    }

    /// Get qualified names of all symbols defined in a specific file
    ///
    /// Returns a Vec of qualified names for the file.
    /// Used by enable_auto_invalidation to know which symbols to remove.
    /// Paths are normalized.
    pub fn get_qualified_names_for_file(&self, file_path: &str) -> Vec<String> {
        let normalized = normalize_path(file_path);
        self.symbols_by_file
            .get(&normalized)
            .into_iter()
            .flatten()
            .filter_map(|id| self.get_symbol(*id).map(|s| s.qualified_name().to_string()))
            .collect()
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
