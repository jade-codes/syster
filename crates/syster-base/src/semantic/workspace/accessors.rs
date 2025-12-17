use crate::semantic::graphs::{DependencyGraph, RelationshipGraph};
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::workspace::{ParsedFile, Workspace, WorkspaceFile};
use std::collections::HashMap;
use std::path::PathBuf;

impl<T: ParsedFile> Workspace<T> {
    /// Returns a reference to the files map
    pub fn files(&self) -> &HashMap<PathBuf, WorkspaceFile<T>> {
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
}
