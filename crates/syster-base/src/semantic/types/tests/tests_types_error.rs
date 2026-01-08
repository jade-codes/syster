#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use crate::core::error_codes::{
    SEMANTIC_CIRCULAR_DEPENDENCY, SEMANTIC_CIRCULAR_DEPENDENCY_MSG, SEMANTIC_INVALID_TYPE,
    SEMANTIC_INVALID_TYPE_MSG, SEMANTIC_TYPE_MISMATCH, SEMANTIC_TYPE_MISMATCH_MSG,
    SEMANTIC_UNDEFINED_REFERENCE, SEMANTIC_UNDEFINED_REFERENCE_MSG,
};

// ============================================================================
// Tests for with_location method (issue #345)
// ============================================================================

#[test]
fn test_with_location_adds_location() {
    let error = SemanticError::undefined_reference("Symbol".to_string());
    let location = Location {
        file: Some("test.sysml".to_string()),
        line: Some(10),
        column: Some(5),
    };

    let error_with_loc = error.with_location(location.clone());

    assert_eq!(error_with_loc.location, Some(location));
}

#[test]
fn test_with_location_chains_properly() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("test.sysml".to_string()),
        line: Some(10),
        column: Some(5),
    });

    assert!(error.location.is_some());
    assert_eq!(
        error.location.as_ref().unwrap().file,
        Some("test.sysml".to_string())
    );
}

#[test]
fn test_with_location_with_file_only() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("file.sysml".to_string()),
        line: None,
        column: None,
    });

    assert!(error.location.is_some());
    assert_eq!(
        error.location.as_ref().unwrap().file,
        Some("file.sysml".to_string())
    );
    assert_eq!(error.location.as_ref().unwrap().line, None);
    assert_eq!(error.location.as_ref().unwrap().column, None);
}

#[test]
fn test_with_location_with_line_no_column() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("file.sysml".to_string()),
        line: Some(42),
        column: None,
    });

    assert!(error.location.is_some());
    assert_eq!(error.location.as_ref().unwrap().line, Some(42));
    assert_eq!(error.location.as_ref().unwrap().column, None);
}

#[test]
fn test_with_location_with_all_fields_none() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: None,
        line: None,
        column: None,
    });

    assert!(error.location.is_some());
    assert_eq!(error.location.as_ref().unwrap().file, None);
}

#[test]
fn test_with_location_preserves_error_fields() {
    let original_error = SemanticError::type_mismatch(
        "Int".to_string(),
        "String".to_string(),
        "assignment".to_string(),
    );
    let original_message = original_error.message.clone();
    let original_code = original_error.error_code;

    let error_with_loc = original_error.with_location(Location {
        file: Some("test.sysml".to_string()),
        line: Some(10),
        column: Some(5),
    });

    assert_eq!(error_with_loc.message, original_message);
    assert_eq!(error_with_loc.error_code, original_code);
}

// ============================================================================
// Tests for invalid_type method (issue #346)
// ============================================================================

#[test]
fn test_invalid_type_with_valid_name() {
    let error = SemanticError::invalid_type("UnknownType".to_string());

    assert_eq!(error.error_code, SEMANTIC_INVALID_TYPE);
    assert!(error.message.contains(SEMANTIC_INVALID_TYPE_MSG));
    assert!(error.message.contains("UnknownType"));
    assert!(matches!(error.kind, SemanticErrorKind::InvalidType { .. }));
}

#[test]
fn test_invalid_type_with_empty_name() {
    let error = SemanticError::invalid_type("".to_string());

    assert_eq!(error.error_code, SEMANTIC_INVALID_TYPE);
    assert_eq!(error.message, SEMANTIC_INVALID_TYPE_MSG);
    assert!(matches!(error.kind, SemanticErrorKind::InvalidType { .. }));
}

#[test]
fn test_invalid_type_message_format() {
    let error = SemanticError::invalid_type("MyType".to_string());

    let expected_message = format!("{SEMANTIC_INVALID_TYPE_MSG}: 'MyType'");
    assert_eq!(error.message, expected_message);
}

#[test]
fn test_invalid_type_kind_contains_name() {
    let error = SemanticError::invalid_type("CustomType".to_string());

    if let SemanticErrorKind::InvalidType { type_name } = &error.kind {
        assert_eq!(type_name, "CustomType");
    } else {
        panic!("Expected InvalidType kind");
    }
}

#[test]
fn test_invalid_type_with_special_characters() {
    let error = SemanticError::invalid_type("Type::With::Colons".to_string());

    assert!(error.message.contains("Type::With::Colons"));
}

#[test]
fn test_invalid_type_no_location_by_default() {
    let error = SemanticError::invalid_type("SomeType".to_string());

    assert_eq!(error.location, None);
}

// ============================================================================
// Tests for type_mismatch method (issue #347)
// ============================================================================

#[test]
fn test_type_mismatch_with_valid_inputs() {
    let error = SemanticError::type_mismatch(
        "Integer".to_string(),
        "String".to_string(),
        "assignment".to_string(),
    );

    assert_eq!(error.error_code, SEMANTIC_TYPE_MISMATCH);
    assert!(error.message.contains(SEMANTIC_TYPE_MISMATCH_MSG));
    assert!(error.message.contains("Integer"));
    assert!(error.message.contains("String"));
    assert!(error.message.contains("assignment"));
}

#[test]
fn test_type_mismatch_with_empty_expected() {
    let error =
        SemanticError::type_mismatch("".to_string(), "String".to_string(), "context".to_string());

    assert!(error.message.contains("expected ''"));
    assert!(error.message.contains("found 'String'"));
}

#[test]
fn test_type_mismatch_with_empty_found() {
    let error =
        SemanticError::type_mismatch("Integer".to_string(), "".to_string(), "context".to_string());

    assert!(error.message.contains("expected 'Integer'"));
    assert!(error.message.contains("found ''"));
}

#[test]
fn test_type_mismatch_with_empty_context() {
    let error = SemanticError::type_mismatch("Int".to_string(), "Bool".to_string(), "".to_string());

    assert!(error.message.contains("in "));
}

#[test]
fn test_type_mismatch_with_all_empty() {
    let error = SemanticError::type_mismatch("".to_string(), "".to_string(), "".to_string());

    assert!(error.message.contains(SEMANTIC_TYPE_MISMATCH_MSG));
    assert!(matches!(error.kind, SemanticErrorKind::TypeMismatch { .. }));
}

#[test]
fn test_type_mismatch_message_format() {
    let error = SemanticError::type_mismatch("A".to_string(), "B".to_string(), "test".to_string());

    let expected = format!("{SEMANTIC_TYPE_MISMATCH_MSG}: expected 'A', found 'B' in test");
    assert_eq!(error.message, expected);
}

#[test]
fn test_type_mismatch_kind_contains_all_fields() {
    let error = SemanticError::type_mismatch(
        "TypeA".to_string(),
        "TypeB".to_string(),
        "operation".to_string(),
    );

    if let SemanticErrorKind::TypeMismatch {
        expected,
        found,
        context,
    } = &error.kind
    {
        assert_eq!(expected, "TypeA");
        assert_eq!(found, "TypeB");
        assert_eq!(context, "operation");
    } else {
        panic!("Expected TypeMismatch kind");
    }
}

#[test]
fn test_type_mismatch_no_location_by_default() {
    let error = SemanticError::type_mismatch("A".to_string(), "B".to_string(), "ctx".to_string());

    assert_eq!(error.location, None);
}

// ============================================================================
// Tests for circular_dependency method (issue #349)
// ============================================================================

#[test]
fn test_circular_dependency_with_empty_cycle() {
    let error = SemanticError::circular_dependency(vec![]);

    assert_eq!(error.error_code, SEMANTIC_CIRCULAR_DEPENDENCY);
    assert_eq!(error.message, SEMANTIC_CIRCULAR_DEPENDENCY_MSG);
    assert!(matches!(
        error.kind,
        SemanticErrorKind::CircularDependency { .. }
    ));
}

#[test]
fn test_circular_dependency_with_single_element() {
    let error = SemanticError::circular_dependency(vec!["A".to_string()]);

    assert_eq!(error.error_code, SEMANTIC_CIRCULAR_DEPENDENCY);
    assert!(error.message.contains(SEMANTIC_CIRCULAR_DEPENDENCY_MSG));
    assert!(error.message.contains("A"));
}

#[test]
fn test_circular_dependency_with_two_elements() {
    let error = SemanticError::circular_dependency(vec!["A".to_string(), "B".to_string()]);

    assert!(error.message.contains("A -> B"));
}

#[test]
fn test_circular_dependency_with_multiple_elements() {
    let error = SemanticError::circular_dependency(vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
    ]);

    assert!(error.message.contains(SEMANTIC_CIRCULAR_DEPENDENCY_MSG));
    assert!(error.message.contains("A -> B -> C -> D"));
}

#[test]
fn test_circular_dependency_message_format() {
    let cycle = vec!["X".to_string(), "Y".to_string(), "Z".to_string()];
    let error = SemanticError::circular_dependency(cycle);

    let expected = format!("{SEMANTIC_CIRCULAR_DEPENDENCY_MSG}: X -> Y -> Z");
    assert_eq!(error.message, expected);
}

#[test]
fn test_circular_dependency_kind_contains_cycle() {
    let cycle = vec!["A".to_string(), "B".to_string()];
    let error = SemanticError::circular_dependency(cycle.clone());

    if let SemanticErrorKind::CircularDependency { cycle: error_cycle } = &error.kind {
        assert_eq!(error_cycle, &cycle);
    } else {
        panic!("Expected CircularDependency kind");
    }
}

#[test]
fn test_circular_dependency_with_qualified_names() {
    let error = SemanticError::circular_dependency(vec![
        "Package::A".to_string(),
        "Package::B".to_string(),
        "Package::A".to_string(),
    ]);

    assert!(
        error
            .message
            .contains("Package::A -> Package::B -> Package::A")
    );
}

#[test]
fn test_circular_dependency_no_location_by_default() {
    let error = SemanticError::circular_dependency(vec!["A".to_string()]);

    assert_eq!(error.location, None);
}

// ============================================================================
// Tests for undefined_reference method (issue #350)
// ============================================================================

#[test]
fn test_undefined_reference_with_valid_name() {
    let error = SemanticError::undefined_reference("MySymbol".to_string());

    assert_eq!(error.error_code, SEMANTIC_UNDEFINED_REFERENCE);
    assert!(error.message.contains(SEMANTIC_UNDEFINED_REFERENCE_MSG));
    assert!(error.message.contains("MySymbol"));
    assert!(matches!(
        error.kind,
        SemanticErrorKind::UndefinedReference { .. }
    ));
}

#[test]
fn test_undefined_reference_with_empty_name() {
    let error = SemanticError::undefined_reference("".to_string());

    assert_eq!(error.error_code, SEMANTIC_UNDEFINED_REFERENCE);
    assert_eq!(error.message, SEMANTIC_UNDEFINED_REFERENCE_MSG);
    assert!(matches!(
        error.kind,
        SemanticErrorKind::UndefinedReference { .. }
    ));
}

#[test]
fn test_undefined_reference_message_format() {
    let error = SemanticError::undefined_reference("Variable".to_string());

    let expected_message = format!("{SEMANTIC_UNDEFINED_REFERENCE_MSG}: 'Variable'");
    assert_eq!(error.message, expected_message);
}

#[test]
fn test_undefined_reference_kind_contains_name() {
    let error = SemanticError::undefined_reference("SomeSymbol".to_string());

    if let SemanticErrorKind::UndefinedReference { name } = &error.kind {
        assert_eq!(name, "SomeSymbol");
    } else {
        panic!("Expected UndefinedReference kind");
    }
}

#[test]
fn test_undefined_reference_with_qualified_name() {
    let error = SemanticError::undefined_reference("Package::Class::Member".to_string());

    assert!(error.message.contains("Package::Class::Member"));
}

#[test]
fn test_undefined_reference_no_location_by_default() {
    let error = SemanticError::undefined_reference("Symbol".to_string());

    assert_eq!(error.location, None);
}

// ============================================================================
// Tests for Display implementation (issue #351)
// ============================================================================

#[test]
fn test_display_with_error_code_only() {
    let error = SemanticError::undefined_reference("Symbol".to_string());
    let display = format!("{error}");

    assert!(display.starts_with("E002: "));
}

#[test]
fn test_display_without_location() {
    let error = SemanticError::invalid_type("Type".to_string());
    let display = format!("{error}");

    assert!(display.starts_with("E004: "));
    assert!(display.contains(SEMANTIC_INVALID_TYPE_MSG));
    assert!(display.contains("Type"));
    assert!(!display.contains(".sysml"));
}

#[test]
fn test_display_with_file_only() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("test.sysml".to_string()),
        line: None,
        column: None,
    });

    let display = format!("{error}");

    assert!(display.contains("E002: "));
    assert!(display.contains("test.sysml:"));
}

#[test]
fn test_display_with_file_and_line() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("test.sysml".to_string()),
        line: Some(42),
        column: None,
    });

    let display = format!("{error}");

    assert!(display.contains("test.sysml:42:"));
}

#[test]
fn test_display_with_all_location_fields() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("file.sysml".to_string()),
        line: Some(10),
        column: Some(25),
    });

    let display = format!("{error}");

    assert!(display.contains("E002: "));
    assert!(display.contains("file.sysml:10:25:"));
    assert!(display.contains("Symbol"));
}

#[test]
fn test_display_with_line_only() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: None,
        line: Some(15),
        column: None,
    });

    let display = format!("{error}");

    assert!(display.contains("15:"));
}

#[test]
fn test_display_with_line_and_column_no_file() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: None,
        line: Some(20),
        column: Some(30),
    });

    let display = format!("{error}");

    assert!(display.contains("20:30:"));
    assert!(!display.contains(".sysml"));
}

#[test]
fn test_display_format_includes_message() {
    let error = SemanticError::type_mismatch(
        "Integer".to_string(),
        "String".to_string(),
        "assignment".to_string(),
    );

    let display = format!("{error}");

    assert!(display.contains(error.message.as_str()));
}

#[test]
fn test_display_format_order() {
    let error = SemanticError::undefined_reference("MyVar".to_string()).with_location(Location {
        file: Some("main.sysml".to_string()),
        line: Some(5),
        column: Some(10),
    });

    let display = format!("{error}");

    // Should be: ERROR_CODE: file:line:column: message
    let parts: Vec<&str> = display.split(':').collect();
    assert!(parts.len() >= 4);
    assert_eq!(parts[0], "E002");
}

#[test]
fn test_display_circular_dependency_with_cycle() {
    let error =
        SemanticError::circular_dependency(vec!["A".to_string(), "B".to_string(), "A".to_string()]);

    let display = format!("{error}");

    assert!(display.starts_with("E005: "));
    assert!(display.contains("A -> B -> A"));
}

#[test]
fn test_display_type_mismatch_full_message() {
    let error = SemanticError::type_mismatch(
        "Int".to_string(),
        "Bool".to_string(),
        "comparison".to_string(),
    );

    let display = format!("{error}");

    assert!(display.starts_with("E003: "));
    assert!(display.contains("expected 'Int'"));
    assert!(display.contains("found 'Bool'"));
    assert!(display.contains("comparison"));
}

#[test]
fn test_display_consistency_across_error_types() {
    let errors = vec![
        SemanticError::undefined_reference("X".to_string()),
        SemanticError::invalid_type("Y".to_string()),
        SemanticError::circular_dependency(vec!["Z".to_string()]),
    ];

    for error in errors {
        let display = format!("{error}");
        // All should start with error code followed by colon
        assert!(display.contains(": "));
        let parts: Vec<&str> = display.split(':').collect();
        assert!(!parts[0].is_empty());
    }
}
