//! Trait for abstracting over parsed file types without depending on syntax layer

/// A trait for parsed files that can provide import information
///
/// This trait allows the semantic layer to work with parsed files
/// without directly depending on the syntax layer's concrete types.
pub trait ParsedFile: std::fmt::Debug {
    /// Extracts import statements from the file
    fn extract_imports(&self) -> Vec<String>;
}
