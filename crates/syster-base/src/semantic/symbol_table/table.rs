use std::collections::HashMap;

use crate::core::Span;
use crate::core::events::EventEmitter;
use crate::core::operation::{EventBus, OperationResult};
use crate::semantic::SymbolTableEvent;

use super::scope::{Import, Scope};
use super::symbol::Symbol;

pub struct SymbolTable {
    pub(super) scopes: Vec<Scope>,
    pub(super) current_scope: usize,
    current_file: Option<String>,
    pub events: EventEmitter<SymbolTableEvent, SymbolTable>,
    /// Index mapping file paths to qualified names of symbols defined in that file
    pub(super) symbols_by_file: HashMap<String, Vec<String>>,
    /// Index mapping file paths to imports originating from that file
    pub(super) imports_by_file: HashMap<String, Vec<Import>>,
    /// Reverse index: target_qname -> [(file, span)] for O(1) import reference lookups
    import_references: HashMap<String, Vec<(String, Span)>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new(None)],
            current_scope: 0,
            current_file: None,
            events: EventEmitter::new(),
            symbols_by_file: HashMap::new(),
            imports_by_file: HashMap::new(),
            import_references: HashMap::new(),
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
            let source_file = symbol.source_file().map(|s| s.to_string());
            let symbol_id = self.scopes.iter().map(|s| s.symbols.len()).sum::<usize>();

            let scope = &mut self.scopes[self.current_scope];
            if scope.symbols.contains_key(&name) {
                return OperationResult::failure(format!(
                    "Symbol '{name}' already defined in this scope"
                ))
                .publish(self);
            }

            scope.symbols.insert(name, symbol);

            // Update the file -> symbols index
            if let Some(file_path) = source_file {
                self.symbols_by_file
                    .entry(file_path)
                    .or_default()
                    .push(qualified_name.clone());
            }

            let event = SymbolTableEvent::SymbolInserted {
                qualified_name,
                symbol_id,
            };
            OperationResult::success((), Some(event))
        }
        .publish(self)
    }

    pub fn add_import(
        &mut self,
        path: String,
        is_recursive: bool,
        span: Option<crate::core::Span>,
        file: Option<String>,
    ) {
        let _ = {
            let is_namespace = path.ends_with("::*") || path.ends_with("::**");
            let import = Import {
                path: path.clone(),
                is_recursive,
                is_namespace,
                span,
                file: file.clone(),
            };
            self.scopes[self.current_scope].imports.push(import.clone());

            // Update the file -> imports index
            if let Some(file_path) = file {
                self.imports_by_file
                    .entry(file_path)
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

    pub fn get_scope_imports(&self, scope_id: usize) -> Vec<super::scope::Import> {
        self.scopes
            .get(scope_id)
            .map(|scope| scope.imports.clone())
            .unwrap_or_default()
    }

    /// Add references to a symbol identified by its qualified name
    pub fn add_references_to_symbol(
        &mut self,
        qualified_name: &str,
        references: Vec<super::symbol::SymbolReference>,
    ) {
        for scope in &mut self.scopes {
            for symbol in scope.symbols.values_mut() {
                if symbol.qualified_name() == qualified_name {
                    for reference in references.clone() {
                        symbol.add_reference(reference);
                    }
                    return;
                }
            }
        }
    }

    /// Get all imports that reference a given target (for "Find References")
    /// Returns (file, span) pairs for each import of the target.
    /// O(1) lookup using reverse index.
    pub fn get_import_references(&self, target_qname: &str) -> Vec<(&str, &Span)> {
        self.import_references
            .get(target_qname)
            .map(|refs| {
                refs.iter()
                    .map(|(file, span)| (file.as_str(), span))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Register an import reference for reverse lookup.
    /// Called during import resolution when we know the target.
    pub fn add_import_reference(&mut self, target_qname: String, file: String, span: Span) {
        self.import_references
            .entry(target_qname)
            .or_default()
            .push((file, span));
    }

    /// Clear import references for a file (called when file is reparsed)
    pub fn clear_import_references_for_file(&mut self, file_path: &str) {
        for refs in self.import_references.values_mut() {
            refs.retain(|(file, _)| file != file_path);
        }
        // Clean up empty entries
        self.import_references.retain(|_, refs| !refs.is_empty());
    }

    /// Get all imports from a specific file
    ///
    /// Returns a vector of (import_path, span) tuples for all imports in the given file.
    /// Uses an internal index for O(1) file lookup.
    pub fn get_file_imports(&self, file_path: &str) -> Vec<(String, Span)> {
        self.imports_by_file
            .get(file_path)
            .into_iter()
            .flatten()
            .filter_map(|import| import.span.map(|span| (import.path.clone(), span)))
            .collect()
    }

    /// Get all symbols defined in a specific file
    ///
    /// Returns an iterator of symbols whose source_file matches the given path.
    /// Uses an internal index for O(1) file lookup instead of iterating all symbols.
    pub fn get_symbols_for_file(&self, file_path: &str) -> impl Iterator<Item = &Symbol> {
        self.symbols_by_file
            .get(file_path)
            .into_iter()
            .flatten()
            .filter_map(|qname| self.lookup_qualified(qname))
    }

    /// Get qualified names of all symbols defined in a specific file
    ///
    /// Returns a cloned Vec of qualified names for the file.
    /// Used by enable_auto_invalidation to know which symbols to remove.
    pub fn get_qualified_names_for_file(&self, file_path: &str) -> Vec<String> {
        self.symbols_by_file
            .get(file_path)
            .cloned()
            .unwrap_or_default()
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
