//! # Workspace
//!
//! Manages multi-file SysML/KerML projects with shared symbol table and relationship graphs.
//!
//! Coordinates multiple source files, cross-file symbol resolution, and incremental updates
//! with automatic dependency invalidation for LSP implementations.

use crate::core::events::EventEmitter;
use crate::core::operation::{EventBus, OperationResult};
use crate::language::sysml::syntax::SysMLFile;
use crate::semantic::dependency_graph::DependencyGraph;
use crate::semantic::events::WorkspaceEvent;
use crate::semantic::graph::RelationshipGraph;
use crate::semantic::import_extractor::extract_imports;
use crate::semantic::symbol_table::SymbolTable;
use std::collections::HashMap;
use std::path::PathBuf;

mod file;
mod populator;

pub use file::WorkspaceFile;
use populator::WorkspacePopulator;

/// A workspace manages multiple SysML files with a shared symbol table and relationship graph
pub struct Workspace {
    files: HashMap<PathBuf, WorkspaceFile>,
    symbol_table: SymbolTable,
    relationship_graph: RelationshipGraph,
    dependency_graph: DependencyGraph,
    file_imports: HashMap<PathBuf, Vec<String>>,
    stdlib_loaded: bool,
    pub events: EventEmitter<WorkspaceEvent, Workspace>,
}

impl Workspace {
    /// Creates a new empty workspace
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            symbol_table: SymbolTable::new(),
            relationship_graph: RelationshipGraph::new(),
            dependency_graph: DependencyGraph::new(),
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

    /// Adds a file to the workspace
    pub fn add_file(&mut self, path: PathBuf, content: SysMLFile) {
        let _ = {
            // Extract imports from the file
            let imports = extract_imports(&content);
            self.file_imports.insert(path.clone(), imports);

            let file = WorkspaceFile::new(path.clone(), content);
            self.files.insert(path.clone(), file);

            let event = WorkspaceEvent::FileAdded { path };
            OperationResult::<(), String, WorkspaceEvent>::success((), Some(event))
        }
        .publish(self);
    }

    /// Gets a reference to a file in the workspace
    pub fn get_file(&self, path: &PathBuf) -> Option<&WorkspaceFile> {
        self.files.get(path)
    }

    /// Updates an existing file's content (for LSP document sync)
    pub fn update_file(&mut self, path: &PathBuf, content: SysMLFile) -> bool {
        // Check if file exists first
        if !self.files.contains_key(path) {
            return false;
        }

        // Emit event BEFORE clearing dependencies so listeners can query the graph
        let _ = {
            let event = WorkspaceEvent::FileUpdated { path: path.clone() };
            OperationResult::<(), String, WorkspaceEvent>::success((), Some(event))
        }
        .publish(self);

        // Now update the file
        if let Some(file) = self.files.get_mut(path) {
            // Clear old dependencies
            self.dependency_graph.remove_file(path);

            // Extract new imports
            let imports = extract_imports(&content);
            self.file_imports.insert(path.clone(), imports);

            file.update_content(content);
            true
        } else {
            false
        }
    }

    /// Removes a file from the workspace
    pub fn remove_file(&mut self, path: &PathBuf) -> bool {
        let existed = self.files.remove(path).is_some();
        if existed {
            self.dependency_graph.remove_file(path);
            self.file_imports.remove(path);

            let _ = {
                let event = WorkspaceEvent::FileRemoved { path: path.clone() };
                OperationResult::<(), String, WorkspaceEvent>::success((), Some(event))
            }
            .publish(self);
        }
        existed
    }

    /// Populates the symbol table and relationship graph for all files
    pub fn populate_all(&mut self) -> Result<(), String> {
        let mut populator = WorkspacePopulator::new(
            &self.files,
            &mut self.symbol_table,
            &mut self.relationship_graph,
        );
        let populated_paths = populator.populate_all()?;

        for path in populated_paths {
            self.mark_file_populated(&path);
        }

        Ok(())
    }

    /// Populates only unpopulated files (for incremental updates)
    pub fn populate_affected(&mut self) -> Result<usize, String> {
        let mut populator = WorkspacePopulator::new(
            &self.files,
            &mut self.symbol_table,
            &mut self.relationship_graph,
        );
        let populated_paths = populator.populate_affected()?;
        let count = populated_paths.len();

        for path in populated_paths {
            self.mark_file_populated(&path);
        }

        Ok(count)
    }

    /// Populates a specific file
    pub fn populate_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let mut populator = WorkspacePopulator::new(
            &self.files,
            &mut self.symbol_table,
            &mut self.relationship_graph,
        );
        populator.populate_file(path)?;
        self.mark_file_populated(path);
        Ok(())
    }

    /// Marks a file as populated
    fn mark_file_populated(&mut self, path: &PathBuf) {
        if let Some(file) = self.files.get_mut(path) {
            file.set_populated(true);
        }
    }

    /// Returns a reference to the files map
    pub fn files(&self) -> &HashMap<PathBuf, WorkspaceFile> {
        &self.files
    }

    /// Returns a reference to the symbol table
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    /// Returns a mutable reference to the symbol table
    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    /// Returns a reference to the relationship graph
    pub fn relationship_graph(&self) -> &RelationshipGraph {
        &self.relationship_graph
    }

    /// Returns a mutable reference to the relationship graph
    pub fn relationship_graph_mut(&mut self) -> &mut RelationshipGraph {
        &mut self.relationship_graph
    }

    /// Returns the number of files in the workspace
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Returns an iterator over all file paths in the workspace
    pub fn file_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.keys()
    }

    /// Returns a reference to the dependency graph
    pub fn dependency_graph(&self) -> &DependencyGraph {
        &self.dependency_graph
    }

    /// Returns a mutable reference to the dependency graph
    pub fn dependency_graph_mut(&mut self) -> &mut DependencyGraph {
        &mut self.dependency_graph
    }

    /// Returns the list of files that depend on the given file
    pub fn get_file_dependents(&self, path: &PathBuf) -> Vec<PathBuf> {
        self.dependency_graph.get_dependents(path)
    }

    /// Enables automatic invalidation when files are updated (for LSP)
    pub fn enable_auto_invalidation(&mut self) {
        self.events.subscribe(|event, workspace| {
            if let WorkspaceEvent::FileUpdated { path } = event {
                // Invalidate the file itself and all its dependents
                let mut to_invalidate = vec![path.clone()];
                to_invalidate.extend(workspace.dependency_graph().get_all_affected(path));

                for file_path in to_invalidate {
                    workspace.mark_file_unpopulated(&file_path);
                }
            }
        });
    }

    /// Marks a file as unpopulated (needing re-population)
    fn mark_file_unpopulated(&mut self, path: &PathBuf) {
        if let Some(file) = self.files.get_mut(path) {
            file.set_populated(false);
        }
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus<WorkspaceEvent> for Workspace {
    fn publish(&mut self, event: &WorkspaceEvent) {
        let emitter = std::mem::take(&mut self.events);
        self.events = emitter.emit(event.clone(), self);
    }
}

#[cfg(test)]
#[path = "workspace/tests.rs"]
mod tests;
