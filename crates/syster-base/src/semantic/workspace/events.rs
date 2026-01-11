use crate::core::operation::EventBus;
use crate::semantic::types::WorkspaceEvent;
use crate::semantic::workspace::{ParsedFile, Workspace};

impl<T: ParsedFile> Workspace<T> {
    /// Enables automatic invalidation when files are updated (for LSP)
    /// This clears old symbols and references before repopulation.
    pub fn enable_auto_invalidation(&mut self) {
        self.events.subscribe(|event, workspace| {
            if let WorkspaceEvent::FileUpdated { path } = event {
                let file_path_str = path.to_string_lossy().to_string();

                // Remove references from this file
                workspace
                    .reference_index
                    .remove_references_from_file(&file_path_str);

                // Remove imports from the file
                workspace
                    .symbol_table
                    .remove_imports_from_file(&file_path_str);

                // Remove symbols from the file
                workspace
                    .symbol_table
                    .remove_symbols_from_file(&file_path_str);

                // Mark this file as needing repopulation
                workspace.mark_file_unpopulated(path);
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
