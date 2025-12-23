//! KerML-specific relationship validation.
//!
//! KerML is a foundational language with basic relationships like:
//! - Specialization (classifier specializes another)
//! - Typing (feature typed by a type)
//! - Redefinition (feature redefines another)
//! - Subsetting (feature subsets another)
//!
//! Unlike SysML, KerML doesn't have domain-specific relationships with
//! semantic constraints, so this validator is simpler.

#![allow(clippy::result_large_err)]

use crate::semantic::analyzer::validation::RelationshipValidator;
use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::SemanticError;

/// KerML relationship validator.
/// KerML has structural relationships but no domain-specific semantic constraints.
pub struct KermlValidator;

impl KermlValidator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for KermlValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl RelationshipValidator for KermlValidator {
    fn validate_relationship(
        &self,
        _relationship_type: &str,
        _source: &Symbol,
        _target: &Symbol,
    ) -> Result<(), SemanticError> {
        // KerML relationships are structural and don't have semantic constraints
        // like SysML's satisfy/perform/exhibit/include relationships.
        // All basic relationships (typing, specialization, redefinition, subsetting)
        // are valid as long as the symbols exist, which is checked elsewhere.
        Ok(())
    }
}
