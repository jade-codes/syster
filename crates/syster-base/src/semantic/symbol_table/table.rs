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
            let symbol_id = self.scopes.iter().map(|s| s.symbols.len()).sum::<usize>();

            let scope = &mut self.scopes[self.current_scope];
            if scope.symbols.contains_key(&name) {
                return OperationResult::failure(format!(
                    "Symbol '{name}' already defined in this scope"
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
                file,
            };
            self.scopes[self.current_scope].imports.push(import);

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
