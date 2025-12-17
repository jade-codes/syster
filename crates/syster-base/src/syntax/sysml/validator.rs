//! SysML-specific relationship validation.

#![allow(clippy::result_large_err)]

use crate::semantic::processors::RelationshipValidator;
use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::SemanticError;
use crate::syntax::sysml::ast::constants::{
    SYSML_KIND_ACTION, SYSML_KIND_REQUIREMENT, SYSML_KIND_STATE, SYSML_KIND_USE_CASE,
};

pub struct SysMLRelationshipValidator;

impl SysMLRelationshipValidator {
    pub fn new() -> Self {
        Self
    }

    fn validate_satisfy(&self, target: &Symbol) -> Result<(), SemanticError> {
        let is_requirement = match target {
            Symbol::Definition {
                span: None, kind, ..
            } => kind == SYSML_KIND_REQUIREMENT,
            Symbol::Usage {
                span: None, kind, ..
            } => kind == SYSML_KIND_REQUIREMENT,
            _ => false,
        };

        if !is_requirement {
            return Err(SemanticError::invalid_type(format!(
                "satisfy relationship must target a requirement, but '{}' is not",
                target.qualified_name()
            )));
        }
        Ok(())
    }

    fn validate_perform(&self, target: &Symbol) -> Result<(), SemanticError> {
        if let Symbol::Definition {
            span: None, kind, ..
        } = target
            && kind != SYSML_KIND_ACTION
        {
            return Err(SemanticError::invalid_type(format!(
                "perform relationship must target an action, but '{}' is a {}",
                target.qualified_name(),
                kind
            )));
        }
        Ok(())
    }

    fn validate_exhibit(&self, target: &Symbol) -> Result<(), SemanticError> {
        if let Symbol::Definition {
            span: None, kind, ..
        } = target
            && kind != SYSML_KIND_STATE
        {
            return Err(SemanticError::invalid_type(format!(
                "exhibit relationship must target a state, but '{}' is a {}",
                target.qualified_name(),
                kind
            )));
        }
        Ok(())
    }

    fn validate_include(&self, target: &Symbol) -> Result<(), SemanticError> {
        if let Symbol::Definition {
            span: None, kind, ..
        } = target
            && kind != SYSML_KIND_USE_CASE
        {
            return Err(SemanticError::invalid_type(format!(
                "include relationship must target a use case, but '{}' is a {}",
                target.qualified_name(),
                kind
            )));
        }
        Ok(())
    }
}

impl RelationshipValidator for SysMLRelationshipValidator {
    fn validate_relationship(
        &self,
        relationship_type: &str,
        _source: &Symbol,
        target: &Symbol,
    ) -> Result<(), SemanticError> {
        match relationship_type {
            "satisfy" => self.validate_satisfy(target),
            "perform" => self.validate_perform(target),
            "exhibit" => self.validate_exhibit(target),
            "include" => self.validate_include(target),
            _ => Ok(()), // No constraints for other relationships
        }
    }
}

impl Default for SysMLRelationshipValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "validator/tests.rs"]
mod tests;
