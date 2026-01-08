use crate::core::operation::EventBus;
use crate::semantic::types::WorkspaceEvent;
use crate::semantic::workspace::{ParsedFile, Workspace};

impl<T: ParsedFile> Workspace<T> {
    /// Enables automatic invalidation when files are updated (for LSP)
    /// This clears old symbols and relationships before repopulation.
    pub fn enable_auto_invalidation(&mut self) {
        self.events.subscribe(|event, workspace| {
            if let WorkspaceEvent::FileUpdated { path } = event {
                let file_path_str = path.to_string_lossy().to_string();

                // Clear old symbols and relationships for this file BEFORE repopulation
                // This prevents duplicates from accumulating on each edit

                // Use the symbols_by_file index directly - it's faster and more reliable
                let symbols_to_remove: Vec<String> = workspace
                    .symbol_table
                    .get_qualified_names_for_file(&file_path_str);

                // Remove relationships for all symbols from this file
                for qualified_name in &symbols_to_remove {
                    workspace
                        .relationship_graph
                        .remove_relationships_for_source(qualified_name);
                }

                // Remove relationships stored by file path (RefLocation entries)
                workspace
                    .relationship_graph
                    .remove_relationships_for_file(&file_path_str);

                // Remove imports from the file
                workspace
                    .symbol_table
                    .remove_imports_from_file(&file_path_str);

                // Remove symbols from the file
                workspace
                    .symbol_table
                    .remove_symbols_from_file(&file_path_str);

                // Invalidate the file itself and all its dependents
                let mut to_invalidate = vec![path.clone()];
                to_invalidate.extend(workspace.dependency_graph().get_all_affected(path));

                for file_path in to_invalidate {
                    workspace.mark_file_unpopulated(&file_path);
                }
            }
        });
    }
}

impl<T: ParsedFile> EventBus<WorkspaceEvent> for Workspace<T> {
    fn publish(&mut self, event: &WorkspaceEvent) {
        let emitter = std::mem::take(&mut self.events);
        self.events = emitter.emit(event.clone(), self);
    }
}
