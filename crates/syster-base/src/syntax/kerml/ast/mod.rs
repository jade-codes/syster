pub mod enums;
#[allow(clippy::module_inception)] // from_pest is not inception, it's trait implementations
pub mod from_pest;
pub mod types;

#[cfg(test)]
mod tests;

pub use enums::*;
pub use types::*;

/// Placeholder KerML file structure (KerML parser not yet implemented)
#[derive(Debug, Clone, PartialEq)]
pub struct KerMLFile {
    /// Placeholder for future KerML AST
    pub elements: Vec<()>,
}

impl KerMLFile {
    /// Creates an empty KerML file (placeholder)
    pub fn empty() -> Self {
        Self { elements: vec![] }
    }
}
