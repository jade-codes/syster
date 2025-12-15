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
//! use std::path::PathBuf;
//!
//! let mut workspace = Workspace::new();
//!
//! // Add files
//! workspace.add_file(PathBuf::from("base.sysml"), parsed_base);
//! workspace.add_file(PathBuf::from("app.sysml"), parsed_app);
//!
//! // Populate symbol table and resolve imports
//! workspace.populate_all().unwrap();
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
//! ## Incremental Updates (Future)
//!
//! Currently, the entire workspace is re-populated when files change.
//! Future work will support incremental updates:
//! - Re-populate only changed files
//! - Invalidate dependent imports
//! - Preserve unchanged symbols
//!
//! ## Performance Considerations
//!
//! - **File ordering**: Sorted by path for deterministic builds
//! - **Parallel parsing**: Files can be parsed independently (not yet implemented)
//! - **Symbol indexing**: Pre-computed indexes for fast lookup (partial implementation)
//!
//! See [Workspace Management](../../docs/SEMANTIC_ANALYSIS.md#workspace-management) for details.

use crate::language::sysml::SymbolTablePopulator;
use crate::language::sysml::syntax::SysMLFile;
use crate::semantic::RelationshipGraph;
use crate::semantic::symbol_table::SymbolTable;
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a file in the workspace with its path and parsed content
#[derive(Debug)]
pub struct WorkspaceFile {
    path: PathBuf,
    content: SysMLFile,
    version: u32,
    populated: bool,
}

impl WorkspaceFile {
    pub fn new(path: PathBuf, content: SysMLFile) -> Self {
        Self {
            path,
            content,
            version: 0,
            populated: false,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn content(&self) -> &SysMLFile {
        &self.content
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn is_populated(&self) -> bool {
        self.populated
    }

    fn set_populated(&mut self, populated: bool) {
        self.populated = populated;
    }

    fn update_content(&mut self, content: SysMLFile) {
        self.content = content;
        self.version += 1;
        self.populated = false; // Need to re-populate after content change
    }
}

/// A workspace manages multiple SysML files with a shared symbol table and relationship graph
#[derive(Debug)]
pub struct Workspace {
    files: HashMap<PathBuf, WorkspaceFile>,
    symbol_table: SymbolTable,
    relationship_graph: RelationshipGraph,
    stdlib_loaded: bool,
    symbol_index: HashMap<String, Vec<usize>>,
}

impl Workspace {
    /// Creates a new empty workspace
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            symbol_table: SymbolTable::new(),
            relationship_graph: RelationshipGraph::new(),
            stdlib_loaded: false,
            symbol_index: HashMap::new(),
        }
    }

    /// Creates a new workspace with the standard library pre-loaded
    ///
    /// Note: This is a placeholder. In a complete implementation, use a separate
    /// `LibraryLoader` or `WorkspaceBuilder` to load standard library files from
    /// `/sysml.lib/` before creating the workspace.
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
        let file = WorkspaceFile::new(path.clone(), content);
        self.files.insert(path, file);
    }

    /// Gets a reference to a file in the workspace
    pub fn get_file(&self, path: &PathBuf) -> Option<&WorkspaceFile> {
        self.files.get(path)
    }

    /// Updates an existing file's content (for LSP document sync)
    ///
    /// Returns true if the file was found and updated, false otherwise.
    /// The file will need to be re-populated after this call.
    pub fn update_file(&mut self, path: &PathBuf, content: SysMLFile) -> bool {
        if let Some(file) = self.files.get_mut(path) {
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
        self.files.remove(path).is_some()
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
        self.symbol_index.clear();
        for (qualified_name, scope_ids) in self.symbol_table.all_qualified_names() {
            self.symbol_index.insert(qualified_name, scope_ids);
        }
    }

    /// Looks up a symbol by qualified name using the index (O(1))
    pub fn lookup_qualified(&self, qualified_name: &str) -> Option<Vec<usize>> {
        self.symbol_index.get(qualified_name).cloned()
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "workspace/tests.rs"]
mod tests;
