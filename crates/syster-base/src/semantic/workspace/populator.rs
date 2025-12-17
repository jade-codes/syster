//! # Workspace Populator
//!
//! Handles the population of files in a workspace - extracting symbols from
//! ASTs and building the symbol table and relationship graph.

use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::processors::ReferenceCollector;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::workspace::WorkspaceFile;
use crate::syntax::sysml::SymbolTablePopulator;
use crate::syntax::sysml::ast::SysMLFile;
use std::collections::HashMap;
use std::path::PathBuf;

/// Populates files in the workspace
pub struct WorkspacePopulator<'a> {
    files: &'a HashMap<PathBuf, WorkspaceFile>,
    symbol_table: &'a mut SymbolTable,
    relationship_graph: &'a mut RelationshipGraph,
}

impl<'a> WorkspacePopulator<'a> {
    pub fn new(
        files: &'a HashMap<PathBuf, WorkspaceFile>,
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
            self.populate_file(path)?;
        }

        self.collect_references();
        Ok(paths)
    }

    /// Populates only unpopulated files
    pub fn populate_affected(&mut self) -> Result<Vec<PathBuf>, String> {
        let unpopulated = Self::get_unpopulated_paths(self.files);

        for path in &unpopulated {
            self.populate_file(path)?;
        }

        self.collect_references();
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

        // Only populate SysML files for now
        if let crate::syntax::SyntaxFile::SysML(sysml_file) = &content {
            self.populate_file_content(&file_path_str, sysml_file)?;
        }

        Ok(())
    }

    /// Populates the symbol table with file content
    fn populate_file_content(
        &mut self,
        file_path: &str,
        content: &SysMLFile,
    ) -> Result<(), String> {
        self.symbol_table.remove_symbols_from_file(file_path);
        self.symbol_table
            .set_current_file(Some(file_path.to_string()));

        let mut populator =
            SymbolTablePopulator::with_relationships(self.symbol_table, self.relationship_graph);

        populator
            .populate(content)
            .map_err(|e| format!("Failed to populate {}: {:?}", file_path, e))
    }

    /// Collects references from relationship graph into symbols
    fn collect_references(&mut self) {
        let mut collector = ReferenceCollector::new(self.symbol_table, self.relationship_graph);
        collector.collect();
    }

    /// Gets all file paths sorted for deterministic ordering
    fn get_sorted_paths(files: &HashMap<PathBuf, WorkspaceFile>) -> Vec<PathBuf> {
        let mut paths: Vec<_> = files.keys().cloned().collect();
        paths.sort();
        paths
    }

    /// Gets unpopulated file paths sorted for deterministic ordering
    fn get_unpopulated_paths(files: &HashMap<PathBuf, WorkspaceFile>) -> Vec<PathBuf> {
        let mut unpopulated: Vec<_> = files
            .keys()
            .filter(|path| !files[*path].is_populated())
            .cloned()
            .collect();
        unpopulated.sort();
        unpopulated
    }
}
