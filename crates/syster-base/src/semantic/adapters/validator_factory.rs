//! Validator Factory
//!
//! Creates the appropriate validator for a given file extension.
//! This allows the analyzer/workspace to get a validator without knowing
//! which specific language implementation to use.

use crate::core::constants::{KERML_EXT, SYSML_EXT};
use crate::semantic::analyzer::validation::RelationshipValidator;
use std::sync::Arc;

use super::kerml::validator::KermlValidator;
use super::sysml::validator::SysmlValidator;

/// Creates a validator based on a file extension.
///
/// # Arguments
/// * `extension` - The file extension (e.g., "sysml", "kerml")
///
/// # Returns
/// Returns an `Arc<dyn RelationshipValidator>` so it can be shared across threads
/// and used with the `SemanticAnalyzer`.
///
/// # Example
/// ```
/// use syster::semantic::adapters::create_validator;
///
/// let validator = create_validator("sysml");
/// // Use validator with analyzer
/// ```
pub fn create_validator(extension: &str) -> Arc<dyn RelationshipValidator> {
    match extension {
        SYSML_EXT => Arc::new(SysmlValidator::new()),
        KERML_EXT => Arc::new(KermlValidator::new()),
        _ => {
            // For unknown extensions, return no-op validator
            Arc::new(crate::semantic::analyzer::validation::NoOpValidator)
        }
    }
}
