mod diagnostic_publisher;
pub mod file_loader;
pub mod stdlib_loader;
pub mod workspace_loader;

pub use diagnostic_publisher::DiagnosticPublisher;
pub use stdlib_loader::StdLibLoader;
pub use workspace_loader::WorkspaceLoader;

// Re-export parse types from core for backwards compatibility
pub use crate::core::{ParseError, ParseErrorKind, ParseResult};

// Re-export language parsing convenience function
pub use crate::syntax::parser::parse_with_result;
