use crate::semantic::resolver::Resolver;
use crate::semantic::workspace::{Workspace, populator::WorkspacePopulator};
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

        // Re-resolve reference targets after population
        // This handles cross-file references that used simple names during population
        self.resolve_reference_targets();

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

        // Re-resolve reference targets after population
        self.resolve_reference_targets();

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

        // Re-resolve reference targets for the updated file
        self.resolve_reference_targets();

        Ok(())
    }

    /// Re-resolve reference targets from simple names to qualified names.
    ///
    /// After population, some cross-file references may have been stored with simple names
    /// because import resolution hadn't run yet. This method uses the Resolver to look up
    /// the correct qualified names based on each file's scope.
    fn resolve_reference_targets(&mut self) {
        // Build a map of file path -> scope ID for resolution
        let file_scopes: std::collections::HashMap<PathBuf, usize> = self
            .symbol_table
            .iter_symbols()
            .filter_map(|sym| {
                sym.source_file()
                    .and_then(|f| self.symbol_table.get_scope_for_file(f))
                    .map(|scope| (PathBuf::from(sym.source_file().unwrap()), scope))
            })
            .collect();

        self.reference_index.resolve_targets(|simple_name, file| {
            // Get the scope for this file
            let scope_id = file_scopes.get(file).copied()?;

            // Use the Resolver to look up the symbol in scope
            let resolver = Resolver::new(&self.symbol_table);
            let symbol = resolver.resolve_in_scope(simple_name, scope_id)?;

            Some(symbol.qualified_name().to_string())
        });
    }
}
