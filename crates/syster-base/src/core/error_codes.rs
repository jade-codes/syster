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
//! use syster::semantic::SemanticError;
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

pub const SEMANTIC_DUPLICATE_DEFINITION: &str = "E001";
pub const SEMANTIC_DUPLICATE_DEFINITION_MSG: &str = "Symbol is already defined in this scope";

pub const SEMANTIC_UNDEFINED_REFERENCE: &str = "E002";
pub const SEMANTIC_UNDEFINED_REFERENCE_MSG: &str = "Cannot find symbol";

pub const SEMANTIC_TYPE_MISMATCH: &str = "E003";
pub const SEMANTIC_TYPE_MISMATCH_MSG: &str = "Type mismatch";

pub const SEMANTIC_INVALID_TYPE: &str = "E004";
pub const SEMANTIC_INVALID_TYPE_MSG: &str = "Invalid type";

pub const SEMANTIC_CIRCULAR_DEPENDENCY: &str = "E005";
pub const SEMANTIC_CIRCULAR_DEPENDENCY_MSG: &str = "Circular dependency detected";

pub const SEMANTIC_INVALID_SPECIALIZATION: &str = "E006";
pub const SEMANTIC_INVALID_SPECIALIZATION_MSG: &str = "Invalid specialization relationship";

pub const SEMANTIC_INVALID_REDEFINITION: &str = "E007";
pub const SEMANTIC_INVALID_REDEFINITION_MSG: &str = "Invalid redefinition";

pub const SEMANTIC_INVALID_SUBSETTING: &str = "E008";
pub const SEMANTIC_INVALID_SUBSETTING_MSG: &str = "Invalid subsetting relationship";

pub const SEMANTIC_CONSTRAINT_VIOLATION: &str = "E009";
pub const SEMANTIC_CONSTRAINT_VIOLATION_MSG: &str = "Constraint violation";

pub const SEMANTIC_INVALID_FEATURE_CONTEXT: &str = "E010";
pub const SEMANTIC_INVALID_FEATURE_CONTEXT_MSG: &str = "Feature used in invalid context";

pub const SEMANTIC_ABSTRACT_INSTANTIATION: &str = "E011";
pub const SEMANTIC_ABSTRACT_INSTANTIATION_MSG: &str = "Cannot instantiate abstract element";

pub const SEMANTIC_INVALID_IMPORT: &str = "E012";
pub const SEMANTIC_INVALID_IMPORT_MSG: &str = "Invalid import statement";

pub const SEMANTIC_UNSUPPORTED_LANGUAGE: &str = "E013";
pub const SEMANTIC_UNSUPPORTED_LANGUAGE_MSG: &str = "Unsupported language or file type";

// ============================================================================
// PARSER ERROR CODES (P001-P999)
// ============================================================================

pub const PARSER_SYNTAX_ERROR: &str = "P001";
pub const PARSER_SYNTAX_ERROR_MSG: &str = "Syntax error";

pub const PARSER_UNEXPECTED_TOKEN: &str = "P002";
pub const PARSER_UNEXPECTED_TOKEN_MSG: &str = "Unexpected token";

pub const PARSER_EXPECTED_TOKEN: &str = "P003";
pub const PARSER_EXPECTED_TOKEN_MSG: &str = "Expected token not found";

pub const PARSER_INVALID_IDENTIFIER: &str = "P004";
pub const PARSER_INVALID_IDENTIFIER_MSG: &str = "Invalid identifier";

pub const PARSER_INVALID_LITERAL: &str = "P005";
pub const PARSER_INVALID_LITERAL_MSG: &str = "Invalid literal value";

pub const PARSER_UNTERMINATED: &str = "P006";
pub const PARSER_UNTERMINATED_MSG: &str = "Unterminated string or comment";

pub const PARSER_INVALID_CHARACTER: &str = "P007";
pub const PARSER_INVALID_CHARACTER_MSG: &str = "Invalid character in input";

// ============================================================================
// FILE SYSTEM / IO ERROR CODES (IO001-IO999)
// ============================================================================

pub const IO_FILE_NOT_FOUND: &str = "IO001";
pub const IO_FILE_NOT_FOUND_MSG: &str = "File not found";

pub const IO_PERMISSION_DENIED: &str = "IO002";
pub const IO_PERMISSION_DENIED_MSG: &str = "Permission denied";

pub const IO_READ_FAILED: &str = "IO003";
pub const IO_READ_FAILED_MSG: &str = "Failed to read file";

pub const IO_WRITE_FAILED: &str = "IO004";
pub const IO_WRITE_FAILED_MSG: &str = "Failed to write file";

pub const IO_INVALID_PATH: &str = "IO005";
pub const IO_INVALID_PATH_MSG: &str = "Invalid file path";

pub const IO_FILE_EXISTS: &str = "IO006";
pub const IO_FILE_EXISTS_MSG: &str = "File already exists";

pub const IO_DIRECTORY_NOT_FOUND: &str = "IO007";
pub const IO_DIRECTORY_NOT_FOUND_MSG: &str = "Directory not found";

pub const IO_WORKSPACE_ERROR: &str = "IO008";
pub const IO_WORKSPACE_ERROR_MSG: &str = "Workspace error";

pub const IO_STDLIB_LOAD_FAILED: &str = "IO009";
pub const IO_STDLIB_LOAD_FAILED_MSG: &str = "Failed to load standard library";
