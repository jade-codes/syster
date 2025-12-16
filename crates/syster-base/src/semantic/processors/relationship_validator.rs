//! Trait for language-specific relationship validation.
#![allow(clippy::result_large_err)]

use crate::semantic::error::SemanticError;
use crate::semantic::symbol_table::Symbol;

pub trait RelationshipValidator: Send + Sync {
    /// # Errors
    /// Returns error if relationship violates language-specific constraints.
    fn validate_relationship(
        &self,
        relationship_type: &str,
        source: &Symbol,
        target: &Symbol,
    ) -> Result<(), SemanticError>;
}
pub struct NoOpValidator;

impl RelationshipValidator for NoOpValidator {
    fn validate_relationship(
        &self,
        _relationship_type: &str,
        _source: &Symbol,
        _target: &Symbol,
    ) -> Result<(), SemanticError> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "relationship_validator/tests.rs"]
mod tests;
