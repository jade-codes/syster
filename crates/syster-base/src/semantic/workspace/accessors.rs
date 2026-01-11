use crate::semantic::graphs::ReferenceIndex;
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

    /// Returns a reference to the reference index
    pub fn reference_index(&self) -> &ReferenceIndex {
        &self.reference_index
    }

    /// Returns a mutable reference to the reference index
    pub fn reference_index_mut(&mut self) -> &mut ReferenceIndex {
        &mut self.reference_index
    }

    /// Returns the number of files in the workspace
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Returns an iterator over all file paths in the workspace
    pub fn file_paths(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.keys()
    }
}
