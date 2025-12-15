//! # Error Code System
//!
//! Centralized error codes for consistent error reporting across all layers.
//!
//! ## Error Code Ranges
//!
//! - **E001-E999**: Semantic analysis errors (symbol resolution, type checking, validation)
//! - **P001-P999**: Parser errors (syntax errors, malformed input)
//! - **IO001-IO999**: File system and workspace errors (file not found, read/write failures)
//!
//! ## Usage
//!
//! Error codes should be used consistently across error types to enable:
//! - User-friendly error messages with searchable codes
//! - IDE integration for quick error lookup
//! - Documentation generation
//! - Error analytics and tracking
//!
//! ## Example
//!
//! ```rust
//! use syster::core::error_codes::SEMANTIC_DUPLICATE_DEFINITION;
//! use syster::semantic::error::SemanticError;
//!
//! let error = SemanticError::duplicate_definition(
//!     "Vehicle".to_string(),
//!     None,
//! );
//! // Error will display as: "E001: Symbol 'Vehicle' is already defined in this scope"
//! ```

// ============================================================================
// SEMANTIC ERROR CODES (E001-E999)
// ============================================================================

/// Symbol is defined multiple times in the same scope
pub const SEMANTIC_DUPLICATE_DEFINITION: &str = "E001";

/// Referenced symbol cannot be found
pub const SEMANTIC_UNDEFINED_REFERENCE: &str = "E002";

/// Type mismatch between expected and actual types
pub const SEMANTIC_TYPE_MISMATCH: &str = "E003";

/// Invalid type reference (type doesn't exist or isn't a valid type)
pub const SEMANTIC_INVALID_TYPE: &str = "E004";

/// Circular dependency detected in specialization, imports, etc.
pub const SEMANTIC_CIRCULAR_DEPENDENCY: &str = "E005";

/// Invalid specialization relationship
pub const SEMANTIC_INVALID_SPECIALIZATION: &str = "E006";

/// Invalid redefinition of a feature
pub const SEMANTIC_INVALID_REDEFINITION: &str = "E007";

/// Invalid subsetting relationship
pub const SEMANTIC_INVALID_SUBSETTING: &str = "E008";

/// Constraint violation (multiplicity, cardinality, etc.)
pub const SEMANTIC_CONSTRAINT_VIOLATION: &str = "E009";

/// Feature used in invalid context
pub const SEMANTIC_INVALID_FEATURE_CONTEXT: &str = "E010";

/// Abstract element cannot be instantiated
pub const SEMANTIC_ABSTRACT_INSTANTIATION: &str = "E011";

/// Invalid import statement
pub const SEMANTIC_INVALID_IMPORT: &str = "E012";

// ============================================================================
// PARSER ERROR CODES (P001-P999)
// ============================================================================

/// Generic syntax error
pub const PARSER_SYNTAX_ERROR: &str = "P001";

/// Unexpected token encountered
pub const PARSER_UNEXPECTED_TOKEN: &str = "P002";

/// Expected token not found
pub const PARSER_EXPECTED_TOKEN: &str = "P003";

/// Invalid identifier
pub const PARSER_INVALID_IDENTIFIER: &str = "P004";

/// Invalid literal value
pub const PARSER_INVALID_LITERAL: &str = "P005";

/// Unterminated string or comment
pub const PARSER_UNTERMINATED: &str = "P006";

/// Invalid character in input
pub const PARSER_INVALID_CHARACTER: &str = "P007";

// ============================================================================
// FILE SYSTEM / IO ERROR CODES (IO001-IO999)
// ============================================================================

/// File not found
pub const IO_FILE_NOT_FOUND: &str = "IO001";

/// Permission denied when accessing file
pub const IO_PERMISSION_DENIED: &str = "IO002";

/// Failed to read file contents
pub const IO_READ_FAILED: &str = "IO003";

/// Failed to write file contents
pub const IO_WRITE_FAILED: &str = "IO004";

/// Invalid file path
pub const IO_INVALID_PATH: &str = "IO005";

/// File already exists (when creating new file)
pub const IO_FILE_EXISTS: &str = "IO006";

/// Directory not found
pub const IO_DIRECTORY_NOT_FOUND: &str = "IO007";

/// Workspace error (file not in workspace, invalid workspace state)
pub const IO_WORKSPACE_ERROR: &str = "IO008";

/// Standard library loading failed
pub const IO_STDLIB_LOAD_FAILED: &str = "IO009";

#[cfg(test)]
#[path = "error_codes/tests.rs"]
mod tests;
