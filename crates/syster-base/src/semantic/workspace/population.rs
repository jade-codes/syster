use crate::semantic::workspace::{Workspace, populator::WorkspacePopulator};
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

impl Workspace<SyntaxFile> {
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
}
