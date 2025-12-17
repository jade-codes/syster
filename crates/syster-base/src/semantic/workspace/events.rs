use crate::core::operation::EventBus;
use crate::semantic::types::WorkspaceEvent;
use crate::semantic::workspace::{ParsedFile, Workspace};

impl<T: ParsedFile> Workspace<T> {
    /// Enables automatic invalidation when files are updated (for LSP)
    pub fn enable_auto_invalidation(&mut self) {
        self.events.subscribe(|event, workspace| {
            if let WorkspaceEvent::FileUpdated { path } = event {
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
