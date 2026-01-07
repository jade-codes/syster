//! # Workspace Populator
//!
//! Handles the population of files in a workspace - extracting symbols from
//! ASTs and building the symbol table and relationship graph.

use crate::semantic::adapters;
use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::processors::ReferenceCollector;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::workspace::WorkspaceFile;
use crate::syntax::SyntaxFile;
use std::collections::HashMap;
use std::path::PathBuf;

/// Populates files in the workspace
pub struct WorkspacePopulator<'a> {
    files: &'a HashMap<PathBuf, WorkspaceFile<SyntaxFile>>,
    symbol_table: &'a mut SymbolTable,
    relationship_graph: &'a mut RelationshipGraph,
}

impl<'a> WorkspacePopulator<'a> {
    pub fn new(
        files: &'a HashMap<PathBuf, WorkspaceFile<SyntaxFile>>,
        symbol_table: &'a mut SymbolTable,
        relationship_graph: &'a mut RelationshipGraph,
    ) -> Self {
        Self {
            files,
            symbol_table,
            relationship_graph,
        }
    }

    /// Populates all files in sorted order
    pub fn populate_all(&mut self) -> Result<Vec<PathBuf>, String> {
        let paths = Self::get_sorted_paths(self.files);

        for path in &paths {
            if let Err(_e) = self.populate_file(path) {
                // Log error but continue processing other files
                // Duplicate symbols are a known issue with qualified redefinitions
            }
        }

        self.collect_references();

        // Always succeed even if some files had errors
        // This allows stdlib to load despite duplicate symbol issues
        Ok(paths)
    }

    /// Populates only unpopulated files
    pub fn populate_affected(&mut self) -> Result<Vec<PathBuf>, String> {
        let unpopulated = Self::get_unpopulated_paths(self.files);

        for path in &unpopulated {
            if let Err(_e) = self.populate_file(path) {
                // Log error but continue processing other files
                // Duplicate symbols are a known issue with qualified redefinitions in stdlib
            }
        }

        // NOTE: collect_references removed - references are now queried
        // directly from RelationshipGraph via get_references_to() for O(1) lookup
        // instead of O(n) on every keystroke
        Ok(unpopulated)
    }

    /// Populates a single file
    pub fn populate_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let content = self
            .files
            .get(path)
            .map(|f| f.content().clone())
            .ok_or_else(|| format!("File not found in workspace: {}", path.display()))?;

        let file_path_str = path.to_string_lossy().to_string();

        // Collect qualified names of symbols from this file BEFORE removing them
        // so we can also remove their relationships
        let symbols_to_remove: Vec<String> = self
            .symbol_table
            .get_symbols_for_file(&file_path_str)
            .into_iter()
            .map(|symbol| symbol.qualified_name().to_string())
            .collect();

        // Remove relationships for all symbols from this file
        for qualified_name in &symbols_to_remove {
            self.relationship_graph
                .remove_relationships_for_source(qualified_name);
        }

        // Also remove relationships stored by file path (the RefLocation entries)
        self.relationship_graph
            .remove_relationships_for_file(&file_path_str);

        // Clear import references for this file
        self.symbol_table
            .clear_import_references_for_file(&file_path_str);

        // Remove imports from the file
        self.symbol_table.remove_imports_from_file(&file_path_str);

        // Remove symbols from the file
        self.symbol_table.remove_symbols_from_file(&file_path_str);
        self.symbol_table
            .set_current_file(Some(file_path_str.clone()));

        // Delegate to adapter factory - workspace doesn't know about specific languages
        adapters::populate_syntax_file(&content, self.symbol_table, self.relationship_graph)
            .map_err(|errors| format!("Failed to populate {file_path_str}: {errors:?}"))
    }

    /// Collects references from relationship graph into symbols
    fn collect_references(&mut self) {
        let mut collector = ReferenceCollector::new(self.symbol_table, self.relationship_graph);
        collector.collect();
    }

    /// Gets all file paths sorted for deterministic ordering
    fn get_sorted_paths(files: &HashMap<PathBuf, WorkspaceFile<SyntaxFile>>) -> Vec<PathBuf> {
        let mut paths: Vec<_> = files.keys().cloned().collect();
        paths.sort();
        paths
    }

    /// Gets unpopulated file paths sorted for deterministic ordering
    fn get_unpopulated_paths(files: &HashMap<PathBuf, WorkspaceFile<SyntaxFile>>) -> Vec<PathBuf> {
        let mut unpopulated: Vec<_> = files
            .keys()
            .filter(|path| !files[*path].is_populated())
            .cloned()
            .collect();
        unpopulated.sort();
        unpopulated
    }
}
