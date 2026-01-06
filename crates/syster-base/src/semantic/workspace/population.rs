use crate::core::constants::REL_TYPING;
use crate::semantic::resolver::Resolver;
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

        // Resolve relationship targets after all symbols are populated
        self.resolve_relationship_targets();

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

        // Resolve relationship targets after population
        self.resolve_relationship_targets();

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

        // Resolve relationship targets after population
        self.resolve_relationship_targets();

        Ok(())
    }

    /// Resolves unqualified targets in relationships to their fully qualified names.
    /// This runs after population when all symbols are available.
    fn resolve_relationship_targets(&mut self) {
        let resolver = Resolver::new(&self.symbol_table);

        self.relationship_graph
            .resolve_targets(REL_TYPING, |source, target| {
                // Get the scope of the source symbol to resolve in the correct context
                let scope_id = self
                    .symbol_table
                    .find_by_qualified_name(source)
                    .map(|sym| sym.scope_id())
                    .unwrap_or(0);

                // Resolve the target name in that scope (handles imports, scope chain)
                resolver
                    .resolve_in_scope(target, scope_id)
                    .map(|sym| sym.qualified_name().to_string())
            });
    }
}
