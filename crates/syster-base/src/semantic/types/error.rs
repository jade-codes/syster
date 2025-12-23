use crate::core::error_codes::{
    SEMANTIC_CIRCULAR_DEPENDENCY, SEMANTIC_CIRCULAR_DEPENDENCY_MSG, SEMANTIC_DUPLICATE_DEFINITION,
    SEMANTIC_DUPLICATE_DEFINITION_MSG, SEMANTIC_INVALID_TYPE, SEMANTIC_INVALID_TYPE_MSG,
    SEMANTIC_TYPE_MISMATCH, SEMANTIC_TYPE_MISMATCH_MSG, SEMANTIC_UNDEFINED_REFERENCE,
    SEMANTIC_UNDEFINED_REFERENCE_MSG,
};
use std::fmt;

/// Represents semantic errors found during analysis
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticError {
    pub error_code: &'static str,
    pub kind: SemanticErrorKind,
    pub message: String,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticErrorKind {
    /// A symbol is defined multiple times in the same scope
    DuplicateDefinition {
        name: String,
        first_location: Option<Location>,
    },
    /// A referenced symbol cannot be found
    UndefinedReference { name: String },
    /// Type mismatch between expected and actual types
    TypeMismatch {
        expected: String,
        found: String,
        context: String,
    },
    /// Invalid type reference
    InvalidType { type_name: String },
    /// Invalid specialization (e.g., circular inheritance)
    InvalidSpecialization {
        child: String,
        parent: String,
        reason: String,
    },
    /// Invalid redefinition
    InvalidRedefinition {
        feature: String,
        redefined: String,
        reason: String,
    },
    /// Invalid subsetting
    InvalidSubsetting {
        feature: String,
        subset_of: String,
        reason: String,
    },
    /// Constraint violation (e.g., multiplicity, relationship rules)
    ConstraintViolation { constraint: String, reason: String },
    /// Feature used in invalid context
    InvalidFeatureContext { feature: String, context: String },
    /// Abstract element instantiated
    AbstractInstantiation { element: String },
    /// Invalid import
    InvalidImport { path: String, reason: String },
    /// Circular dependency detected
    CircularDependency { cycle: Vec<String> },
    /// Unsupported language or file type
    UnsupportedLanguage,
}

impl SemanticError {
    pub fn new(error_code: &'static str, kind: SemanticErrorKind, message: String) -> Self {
        Self {
            error_code,
            kind,
            message,
            location: None,
        }
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn duplicate_definition(name: String, first_location: Option<Location>) -> Self {
        let message = if name.is_empty() {
            SEMANTIC_DUPLICATE_DEFINITION_MSG.to_string()
        } else {
            format!("{SEMANTIC_DUPLICATE_DEFINITION_MSG}: '{name}'")
        };
        Self::new(
            SEMANTIC_DUPLICATE_DEFINITION,
            SemanticErrorKind::DuplicateDefinition {
                name,
                first_location,
            },
            message,
        )
    }

    pub fn undefined_reference(name: String) -> Self {
        let message = if name.is_empty() {
            SEMANTIC_UNDEFINED_REFERENCE_MSG.to_string()
        } else {
            format!("{SEMANTIC_UNDEFINED_REFERENCE_MSG}: '{name}'")
        };
        Self::new(
            SEMANTIC_UNDEFINED_REFERENCE,
            SemanticErrorKind::UndefinedReference { name },
            message,
        )
    }

    pub fn type_mismatch(expected: String, found: String, context: String) -> Self {
        let message = format!(
            "{SEMANTIC_TYPE_MISMATCH_MSG}: expected '{expected}', found '{found}' in {context}"
        );
        Self::new(
            SEMANTIC_TYPE_MISMATCH,
            SemanticErrorKind::TypeMismatch {
                expected,
                found,
                context,
            },
            message,
        )
    }

    pub fn invalid_type(type_name: String) -> Self {
        let message = if type_name.is_empty() {
            SEMANTIC_INVALID_TYPE_MSG.to_string()
        } else {
            format!("{SEMANTIC_INVALID_TYPE_MSG}: '{type_name}'")
        };
        Self::new(
            SEMANTIC_INVALID_TYPE,
            SemanticErrorKind::InvalidType { type_name },
            message,
        )
    }

    pub fn circular_dependency(cycle: Vec<String>) -> Self {
        let message = if cycle.is_empty() {
            SEMANTIC_CIRCULAR_DEPENDENCY_MSG.to_string()
        } else {
            format!(
                "{}: {}",
                SEMANTIC_CIRCULAR_DEPENDENCY_MSG,
                cycle.join(" -> ")
            )
        };
        Self::new(
            SEMANTIC_CIRCULAR_DEPENDENCY,
            SemanticErrorKind::CircularDependency { cycle },
            message,
        )
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write error code first
        write!(f, "{}: ", self.error_code)?;

        // Write location if available
        if let Some(loc) = &self.location {
            if let Some(file) = &loc.file {
                write!(f, "{file}:")?;
            }
            if let Some(line) = loc.line {
                write!(f, "{line}:")?;
                if let Some(col) = loc.column {
                    write!(f, "{col}:")?;
                }
            }
            write!(f, " ")?;
        }

        // Write message
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SemanticError {}

/// Result type for semantic analysis operations
pub type SemanticResult<T> = Result<T, Vec<SemanticError>>;
