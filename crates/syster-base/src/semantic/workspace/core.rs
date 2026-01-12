use crate::core::events::EventEmitter;
use crate::semantic::graphs::ReferenceIndex;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::WorkspaceEvent;
use crate::semantic::workspace::{ParsedFile, WorkspaceFile};
use std::collections::HashMap;
use std::path::PathBuf;

/// A workspace manages multiple SysML files with a shared symbol table and reference index
pub struct Workspace<T: ParsedFile> {
    pub(super) files: HashMap<PathBuf, WorkspaceFile<T>>,
    pub(super) symbol_table: SymbolTable,
    pub(super) reference_index: ReferenceIndex,
    pub(super) file_imports: HashMap<PathBuf, Vec<String>>,
    pub(super) stdlib_loaded: bool,
    pub events: EventEmitter<WorkspaceEvent, Workspace<T>>,
}

impl<T: ParsedFile> Workspace<T> {
    /// Creates a new empty workspace
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            symbol_table: SymbolTable::new(),
            reference_index: ReferenceIndex::new(),
            file_imports: HashMap::new(),
            stdlib_loaded: false,
            events: EventEmitter::new(),
        }
    }

    /// Creates a new workspace with the standard library pre-loaded
    pub fn with_stdlib() -> Self {
        let mut workspace = Self::new();
        workspace.stdlib_loaded = true;
        workspace
    }

    /// Marks the standard library as loaded (used by library loaders)
    pub fn mark_stdlib_loaded(&mut self) {
        self.stdlib_loaded = true;
    }

    /// Returns whether the standard library has been loaded
    pub fn has_stdlib(&self) -> bool {
        self.stdlib_loaded
    }
}

impl<T: ParsedFile> Default for Workspace<T> {
    fn default() -> Self {
        Self::new()
    }
}
