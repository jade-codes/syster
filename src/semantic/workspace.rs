//! # Workspace
//!
//! Manages multi-file SysML/KerML projects with shared symbol table and relationship graphs.
//!
//! ## Purpose
//!
//! A `Workspace` coordinates:
//! - Multiple source files (`.sysml`, `.kerml`)
//! - Shared symbol table (cross-file symbol resolution)
//! - Shared relationship graphs (cross-file relationships)
//! - Optional standard library loading
//!
//! ## Workflow
//!
//! ```text
//! 1. Create workspace
//! 2. Add files (parsed ASTs)
//! 3. Populate all files → builds symbol table + graphs
//! 4. Query the model
//! ```
//!
//! ## Usage Example
//!
//! ```rust
//! use syster::semantic::Workspace;
//! use syster::language::sysml::syntax::SysMLFile;
//! use std::path::PathBuf;
//!
//! let mut workspace = Workspace::new();
//!
//! // Enable automatic invalidation for LSP use
//! workspace.enable_auto_invalidation();
//!
//! // Add files (with parsed content)
//! let parsed_base = SysMLFile { namespace: None, elements: vec![] };
//! let parsed_app = SysMLFile { namespace: None, elements: vec![] };
//! workspace.add_file(PathBuf::from("base.sysml"), parsed_base);
//! workspace.add_file(PathBuf::from("app.sysml"), parsed_app);
//!
//! // Populate symbol table and resolve imports
//! workspace.populate_all().ok();
//!
//! // Update a file - dependents automatically invalidated
//! let new_content = SysMLFile { namespace: None, elements: vec![] };
//! workspace.update_file(&PathBuf::from("base.sysml"), new_content);
//!
//! // Query the model
//! let symbol = workspace.symbol_table().lookup("App::myCar");
//! ```
//!
//! ## Standard Library Support
//!
//! The workspace can optionally load the SysML standard library:
//!
//! ```rust
//! use syster::semantic::Workspace;
//!
//! // Create workspace with stdlib
//! let mut workspace = Workspace::with_stdlib();
//!
//! // Or mark stdlib as loaded after manual loading
//! workspace.mark_stdlib_loaded();
//!
//! // Check if stdlib is available
//! if workspace.has_stdlib() {
//!     // Can reference stdlib types
//! }
//! ```
//!
//! The standard library includes:
//! - `sysml.library/Kernel Libraries/` - Core KerML types
//! - `sysml.library/Systems Library/` - SysML v2 types (Parts, Ports, etc.)
//! - `sysml.library/Domain Libraries/` - Domain-specific libraries
//!
//! ## File Management
//!
//! Files are indexed by `PathBuf` and stored as `WorkspaceFile` entries:
//!
//! ```rust
//! use std::path::PathBuf;
//! use syster::language::sysml::syntax::SysMLFile;
//!
//! pub struct WorkspaceFile {
//!     path: PathBuf,         // File path
//!     content: SysMLFile,    // Parsed AST
//! }
//! ```
//!
//! ## Symbol Table Population
//!
//! `populate_all()` processes files in deterministic order (sorted by path):
//!
//! 1. For each file:
//!    - Walk AST with `SymbolTablePopulator`
//!    - Add symbols to shared symbol table
//!    - Track relationships in shared graphs
//! 2. Resolve imports (three-pass algorithm)
//! 3. Validate semantic rules
//!
//! **Key invariant**: All symbols from all files visible in global symbol table.
//!
//! ## Cross-File References
//!
//! The workspace enables cross-file references via imports:
//!
//! **File: base.sysml**
//! ```sysml
//! package Base {
//!     part def Vehicle;
//! }
//! ```
//!
//! **File: app.sysml**
//! ```sysml
//! package App {
//!     import Base::*;
//!     part myCar: Vehicle;  // References Base::Vehicle
//! }
//! ```
//!
//! After `populate_all()`, `Vehicle` is visible in `App` scope.
//!
//! ## Incremental Updates
//!
//! The workspace supports smart invalidation via an event system:
//!
//! ```rust
//! use syster::semantic::Workspace;
//! use syster::language::sysml::syntax::SysMLFile;
//! use std::path::PathBuf;
//!
//! let mut workspace = Workspace::new();
//! workspace.enable_auto_invalidation();
//!
//! let path = PathBuf::from("file.sysml");
//! let content = SysMLFile { namespace: None, elements: vec![] };
//! workspace.add_file(path.clone(), content);
//!
//! // When file changes, dependents are automatically marked
//! let new_content = SysMLFile { namespace: None, elements: vec![] };
//! workspace.update_file(&path, new_content);
//!
//! // Only re-populate files that need it
//! let file_paths: Vec<_> = workspace.file_paths().cloned().collect();
//! for file_path in file_paths {
//!     if !workspace.get_file(&file_path).unwrap().is_populated() {
//!         workspace.populate_file(&file_path).unwrap();
//!     }
//! }
//! ```
//!
//! This enables efficient incremental analysis for LSP implementations where
//! only changed files and their dependents need re-analysis.
//!
//! ## Performance Considerations
//!
//! - **File ordering**: Sorted by path for deterministic builds
//! - **Parallel parsing**: Files can be parsed independently (not yet implemented)
//! - **Symbol indexing**: Pre-computed indexes for fast lookup (partial implementation)
//!
//! See [Workspace Management](../../docs/SEMANTIC_ANALYSIS.md#workspace-management) for details.

use crate::core::events::EventEmitter;
use crate::core::operation::{EventBus, OperationResult};
use crate::language::sysml::SymbolTablePopulator;
use crate::language::sysml::syntax::SysMLFile;
use crate::semantic::dependency_graph::DependencyGraph;
use crate::semantic::events::WorkspaceEvent;
use crate::semantic::graph::RelationshipGraph;
use crate::semantic::import_extractor::extract_imports;
use crate::semantic::symbol_table::SymbolTable;
use std::collections::HashMap;
use std::path::PathBuf;

mod file;
pub use file::WorkspaceFile;

/// A workspace manages multiple SysML files with a shared symbol table and relationship graph
pub struct Workspace {
    files: HashMap<PathBuf, WorkspaceFile>,
    symbol_table: SymbolTable,
    relationship_graph: RelationshipGraph,
    dependency_graph: DependencyGraph,
    file_imports: HashMap<PathBuf, Vec<String>>,
    stdlib_loaded: bool,
    symbol_index: HashMap<String, Vec<usize>>,
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
            symbol_index: HashMap::new(),
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
    ///
    /// Returns true if updated, false if file not found.
    /// Emits a FileUpdated event for invalidation listeners.
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
    ///
    /// Returns true if the file was found and removed, false otherwise.
    /// Note: This does not remove symbols from the symbol table - use `repopulate_all()` after.
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

    /// Populates the symbol table and relationship graph for all files in the workspace
    ///
    /// # Errors
    ///
    /// Returns an error if any file in the workspace fails to populate due to invalid syntax
    /// or semantic errors in the SysML content.
    pub fn populate_all(&mut self) -> Result<(), String> {
        // Sort files by path for deterministic ordering
        let mut paths: Vec<_> = self.files.keys().cloned().collect();
        paths.sort();

        for path in paths {
            self.populate_file(&path)?;
        }

        Ok(())
    }

    /// Populates only files that are marked as unpopulated (needing re-population)
    ///
    /// This is more efficient than `populate_all()` for incremental updates in LSP scenarios
    /// where only a subset of files have been modified or invalidated.
    ///
    /// # Returns
    ///
    /// Returns the number of files that were repopulated.
    ///
    /// # Errors
    ///
    /// Returns an error if any unpopulated file fails to populate due to invalid syntax
    /// or semantic errors in the SysML content.
    pub fn populate_affected(&mut self) -> Result<usize, String> {
        // Collect unpopulated files (sorted for determinism)
        let mut unpopulated: Vec<_> = self
            .files
            .keys()
            .filter(|path| !self.files[*path].is_populated())
            .cloned()
            .collect();
        unpopulated.sort();

        let count = unpopulated.len();

        unpopulated
            .into_iter()
            .try_for_each(|path| self.populate_file(&path))?;

        Ok(count)
    }

    /// Populates the symbol table and relationship graph for a specific file
    ///
    /// # Errors
    ///
    /// Returns an error if the file is not found in the workspace or if the file fails
    /// to populate due to invalid syntax or semantic errors in the SysML content.
    pub fn populate_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let file = self
            .files
            .get(path)
            .ok_or_else(|| format!("File not found in workspace: {}", path.display()))?;

        // Set the current file context in the symbol table
        self.symbol_table
            .set_current_file(Some(path.to_string_lossy().to_string()));

        // Populate the symbol table and relationship graph
        let mut populator = SymbolTablePopulator::with_relationships(
            &mut self.symbol_table,
            &mut self.relationship_graph,
        );

        populator
            .populate(file.content())
            .map_err(|e| format!("Failed to populate {}: {:?}", path.display(), e))?;

        // Mark file as populated
        if let Some(file) = self.files.get_mut(path) {
            file.set_populated(true);
        }

        // Rebuild symbol index after population
        self.rebuild_symbol_index();

        Ok(())
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

    /// Checks if a file exists in the workspace
    pub fn contains_file(&self, path: &PathBuf) -> bool {
        self.files.contains_key(path)
    }

    /// Rebuilds the symbol index from the symbol table
    pub fn rebuild_symbol_index(&mut self) {
        self.symbol_index = self.symbol_table.all_qualified_names();
    }

    /// Looks up a symbol by qualified name using the index (O(1))
    pub fn lookup_qualified(&self, qualified_name: &str) -> Option<Vec<usize>> {
        self.symbol_index.get(qualified_name).cloned()
    }

    /// Returns a reference to the dependency graph
    pub fn dependency_graph(&self) -> &DependencyGraph {
        &self.dependency_graph
    }

    /// Returns a mutable reference to the dependency graph
    pub fn dependency_graph_mut(&mut self) -> &mut DependencyGraph {
        &mut self.dependency_graph
    }

    /// Returns the list of import paths for a file
    pub fn get_file_imports(&self, path: &PathBuf) -> Vec<String> {
        self.file_imports.get(path).cloned().unwrap_or_default()
    }

    /// Returns the list of files that depend on the given file
    pub fn get_file_dependents(&self, path: &PathBuf) -> Vec<PathBuf> {
        self.dependency_graph.get_dependents(path)
    }

    /// Enables automatic invalidation of dependent files when files are updated
    ///
    /// Recommended for LSP implementations where file changes trigger re-analysis.
    pub fn enable_auto_invalidation(&mut self) {
        self.subscribe(|event, workspace| {
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

    /// Subscribes a listener to workspace events for custom side effects
    pub fn subscribe<F>(&mut self, listener: F)
    where
        F: Fn(&WorkspaceEvent, &mut Workspace) + Send + Sync + 'static,
    {
        self.events.subscribe(listener);
    }

    /// Marks a file as unpopulated (needing re-population)
    pub fn mark_file_unpopulated(&mut self, path: &PathBuf) {
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
