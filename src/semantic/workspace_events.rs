//! Event system for workspace file changes
//!
//! Provides a pub/sub mechanism for reacting to workspace changes like file updates,
//! additions, and removals. This enables decoupled invalidation and notification logic.

use crate::core::events::Event;
use std::path::PathBuf;

/// Events emitted by the workspace during file operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceEvent {
    /// A file was added to the workspace
    FileAdded { path: PathBuf },

    /// A file's content was updated
    FileUpdated { path: PathBuf },

    /// A file was removed from the workspace
    FileRemoved { path: PathBuf },
}

impl Event for WorkspaceEvent {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let path = PathBuf::from("test.sysml");

        let added = WorkspaceEvent::FileAdded { path: path.clone() };
        let updated = WorkspaceEvent::FileUpdated { path: path.clone() };
        let removed = WorkspaceEvent::FileRemoved { path: path.clone() };

        assert!(matches!(added, WorkspaceEvent::FileAdded { .. }));
        assert!(matches!(updated, WorkspaceEvent::FileUpdated { .. }));
        assert!(matches!(removed, WorkspaceEvent::FileRemoved { .. }));
    }

    #[test]
    fn test_event_equality() {
        let path = PathBuf::from("test.sysml");

        let event1 = WorkspaceEvent::FileUpdated { path: path.clone() };
        let event2 = WorkspaceEvent::FileUpdated { path: path.clone() };

        assert_eq!(event1, event2);
    }
}
