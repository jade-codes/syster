use crate::semantic::workspace::{populator::WorkspacePopulator, Workspace};
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

impl Workspace<SyntaxFile> {
    /// Populates the symbol table and reference index for all files
    pub fn populate_all(&mut self) -> Result<(), String> {
        let mut populator = WorkspacePopulator::new(
            &self.files,
            &mut self.symbol_table,
            &mut self.reference_index,
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
            &mut self.reference_index,
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
            &mut self.reference_index,
        );
        populator.populate_file(path)?;
        self.mark_file_populated(path);

        Ok(())
    }
}
