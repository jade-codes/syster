pub mod diagnostic;
pub mod error;
pub mod events;
pub mod folding_range;
pub mod inlay_hint;
pub mod path_utils;
pub mod semantic_role;
pub mod token_type;

pub use diagnostic::{Diagnostic, Location as DiagnosticLocation, Position, Range, Severity};
pub use error::{Location, SemanticError, SemanticErrorKind, SemanticResult};
pub use events::{DependencyEvent, SymbolTableEvent, WorkspaceEvent};
pub use folding_range::FoldingRangeInfo;
pub use inlay_hint::{InlayHint, InlayHintKind};
pub use path_utils::{normalize_path, normalize_pathbuf};
pub use semantic_role::SemanticRole;
pub use token_type::TokenType;

#[cfg(test)]
mod tests;
