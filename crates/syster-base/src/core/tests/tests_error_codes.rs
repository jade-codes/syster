use crate::core::error_codes::*;

#[test]
fn test_error_code_format() {
    // Semantic codes should start with E
    assert!(SEMANTIC_DUPLICATE_DEFINITION.starts_with('E'));
    assert!(SEMANTIC_UNDEFINED_REFERENCE.starts_with('E'));
    assert!(SEMANTIC_CIRCULAR_DEPENDENCY.starts_with('E'));

    // Parser codes should start with P
    assert!(PARSER_SYNTAX_ERROR.starts_with('P'));
    assert!(PARSER_UNEXPECTED_TOKEN.starts_with('P'));

    // IO codes should start with IO
    assert!(IO_FILE_NOT_FOUND.starts_with("IO"));
    assert!(IO_WORKSPACE_ERROR.starts_with("IO"));
}

#[test]
fn test_error_codes_unique() {
    let semantic_codes = vec![
        SEMANTIC_DUPLICATE_DEFINITION,
        SEMANTIC_UNDEFINED_REFERENCE,
        SEMANTIC_TYPE_MISMATCH,
        SEMANTIC_INVALID_TYPE,
        SEMANTIC_CIRCULAR_DEPENDENCY,
        SEMANTIC_INVALID_SPECIALIZATION,
        SEMANTIC_INVALID_REDEFINITION,
        SEMANTIC_INVALID_SUBSETTING,
        SEMANTIC_CONSTRAINT_VIOLATION,
        SEMANTIC_INVALID_FEATURE_CONTEXT,
        SEMANTIC_ABSTRACT_INSTANTIATION,
        SEMANTIC_INVALID_IMPORT,
    ];

    // Check all codes are unique
    let mut unique = semantic_codes.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), semantic_codes.len());
}

#[test]
fn test_error_code_ranges() {
    // Semantic codes should be E001-E999
    assert_eq!(SEMANTIC_DUPLICATE_DEFINITION, "E001");
    assert_eq!(SEMANTIC_UNDEFINED_REFERENCE, "E002");

    // Parser codes should be P001-P999
    assert_eq!(PARSER_SYNTAX_ERROR, "P001");
    assert_eq!(PARSER_UNEXPECTED_TOKEN, "P002");

    // IO codes should be IO001-IO999
    assert_eq!(IO_FILE_NOT_FOUND, "IO001");
    assert_eq!(IO_PERMISSION_DENIED, "IO002");
}
