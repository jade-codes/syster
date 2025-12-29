pub mod diagnostic;
pub mod error;
pub mod events;
pub mod inlay_hint;
pub mod semantic_role;

pub use diagnostic::{Diagnostic, Location as DiagnosticLocation, Position, Range, Severity};
pub use error::{Location, SemanticError, SemanticErrorKind, SemanticResult};
pub use events::{DependencyEvent, SymbolTableEvent, WorkspaceEvent};
pub use inlay_hint::{InlayHint, InlayHintKind};
pub use semantic_role::SemanticRole;

#[cfg(test)]
mod types_error_test;
#[cfg(test)]
mod types_semantic_role_test;
