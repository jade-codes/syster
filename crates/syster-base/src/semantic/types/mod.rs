pub mod diagnostic;
pub mod error;
pub mod events;
pub mod folding;
pub mod semantic_role;

pub use diagnostic::{Diagnostic, Location as DiagnosticLocation, Position, Range, Severity};
pub use error::{Location, SemanticError, SemanticErrorKind, SemanticResult};
pub use events::{DependencyEvent, SymbolTableEvent, WorkspaceEvent};
pub use folding::{FoldableRange, FoldingKind};
pub use semantic_role::SemanticRole;
