#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use std::path::PathBuf;

use crate::{
    core::error_codes::{
        SEMANTIC_ABSTRACT_INSTANTIATION, SEMANTIC_CIRCULAR_DEPENDENCY,
        SEMANTIC_CIRCULAR_DEPENDENCY_MSG, SEMANTIC_CONSTRAINT_VIOLATION,
        SEMANTIC_DUPLICATE_DEFINITION, SEMANTIC_DUPLICATE_DEFINITION_MSG,
        SEMANTIC_INVALID_FEATURE_CONTEXT, SEMANTIC_INVALID_IMPORT, SEMANTIC_INVALID_REDEFINITION,
        SEMANTIC_INVALID_SPECIALIZATION, SEMANTIC_INVALID_SUBSETTING, SEMANTIC_INVALID_TYPE,
        SEMANTIC_INVALID_TYPE_MSG, SEMANTIC_TYPE_MISMATCH, SEMANTIC_TYPE_MISMATCH_MSG,
        SEMANTIC_UNDEFINED_REFERENCE,
    },
    semantic::{
        DependencyEvent, SemanticError, SemanticErrorKind, SymbolTableEvent, WorkspaceEvent,
        types::diagnostic::{Diagnostic, Location as DiagLocation, Position, Range, Severity},
        types::error::Location,
    },
};

#[test]
fn test_error_has_code() {
    let error = SemanticError::undefined_reference("MySymbol".to_string());
    assert_eq!(error.error_code, SEMANTIC_UNDEFINED_REFERENCE);
}

#[test]
fn test_duplicate_definition_has_code() {
    let error = SemanticError::duplicate_definition("Test".to_string(), None);
    assert_eq!(error.error_code, SEMANTIC_DUPLICATE_DEFINITION);
}

#[test]
fn test_type_mismatch_has_code() {
    let error = SemanticError::type_mismatch(
        "Integer".to_string(),
        "String".to_string(),
        "assignment".to_string(),
    );
    assert_eq!(error.error_code, SEMANTIC_TYPE_MISMATCH);
}

#[test]
fn test_invalid_type_has_code() {
    let error = SemanticError::invalid_type("Unknown".to_string());
    assert_eq!(error.error_code, SEMANTIC_INVALID_TYPE);
}

#[test]
fn test_circular_dependency_has_code() {
    let error = SemanticError::circular_dependency(vec!["A".to_string(), "B".to_string()]);
    assert_eq!(error.error_code, SEMANTIC_CIRCULAR_DEPENDENCY);
}

#[test]
fn test_error_display_includes_code() {
    let error = SemanticError::undefined_reference("MySymbol".to_string());
    let display = format!("{error}");
    assert!(display.starts_with("E002:"));
}

#[test]
fn test_error_display_with_location() {
    let error =
        SemanticError::undefined_reference("MySymbol".to_string()).with_location(Location {
            file: Some("test.sysml".to_string()),
            line: Some(10),
            column: Some(5),
        });

    let display = format!("{error}");
    assert!(display.contains("test.sysml"));
    assert!(display.contains("10"));
    assert!(display.contains("MySymbol"));
}

#[test]
fn test_error_display_without_location() {
    let error = SemanticError::undefined_reference("MySymbol".to_string());
    let display = format!("{error}");
    assert!(display.contains("MySymbol"));
    assert!(!display.contains(".sysml"));
}

#[test]
fn test_duplicate_definition_error() {
    let error = SemanticError::duplicate_definition("Test".to_string(), None);
    assert!(
        matches!(error.kind, SemanticErrorKind::DuplicateDefinition { .. }),
        "Expected DuplicateDefinition error kind"
    );
    assert!(error.message.contains("Test"));
    assert!(error.message.contains(SEMANTIC_DUPLICATE_DEFINITION_MSG));
}

#[test]
fn test_type_mismatch_error() {
    let error = SemanticError::type_mismatch(
        "Integer".to_string(),
        "String".to_string(),
        "assignment".to_string(),
    );
    assert!(
        matches!(error.kind, SemanticErrorKind::TypeMismatch { .. }),
        "Expected TypeMismatch error kind"
    );
    assert!(error.message.contains(SEMANTIC_TYPE_MISMATCH_MSG));
    assert!(error.message.contains("Integer"));
    assert!(error.message.contains("String"));
    assert!(error.message.contains("assignment"));
}

#[test]
fn test_circular_dependency_error() {
    let cycle = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "A".to_string(),
    ];
    let error = SemanticError::circular_dependency(cycle);
    assert!(
        matches!(error.kind, SemanticErrorKind::CircularDependency { .. }),
        "Expected CircularDependency error kind"
    );
    assert!(error.message.contains(SEMANTIC_CIRCULAR_DEPENDENCY_MSG));
    assert!(error.message.contains("A -> B -> C -> A"));
}

#[test]
fn test_invalid_type_error() {
    let error = SemanticError::invalid_type("UnknownType".to_string());
    assert!(
        matches!(error.kind, SemanticErrorKind::InvalidType { .. }),
        "Expected InvalidType error kind"
    );
    assert!(error.message.contains("UnknownType"));
    assert!(error.message.contains(SEMANTIC_INVALID_TYPE_MSG));
}

#[test]
fn test_location_with_file_only() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("file.sysml".to_string()),
        line: None,
        column: None,
    });

    let display = format!("{error}");
    assert!(display.contains("file.sysml"));
}

#[test]
fn test_location_with_line_only() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: None,
        line: Some(42),
        column: None,
    });

    let display = format!("{error}");
    assert!(display.contains("42"));
}

#[test]
fn test_location_with_all_fields() {
    let error = SemanticError::undefined_reference("Symbol".to_string()).with_location(Location {
        file: Some("test.sysml".to_string()),
        line: Some(15),
        column: Some(20),
    });

    let display = format!("{error}");
    assert!(display.contains("test.sysml"));
    assert!(display.contains("15"));
    assert!(display.contains("20"));
}

#[test]
fn test_duplicate_definition_with_first_location() {
    let first_loc = Location {
        file: Some("first.sysml".to_string()),
        line: Some(5),
        column: Some(10),
    };
    let error = SemanticError::duplicate_definition("MyClass".to_string(), Some(first_loc));

    let SemanticErrorKind::DuplicateDefinition {
        name,
        first_location,
    } = &error.kind
    else {
        panic!("Expected DuplicateDefinition, got: {:?}", error.kind);
    };

    assert_eq!(name, "MyClass");
    assert!(first_location.is_some());
    assert_eq!(
        first_location.as_ref().unwrap().file,
        Some("first.sysml".to_string())
    );
}

#[test]
fn test_invalid_specialization_error() {
    let error = SemanticError::new(
        SEMANTIC_INVALID_SPECIALIZATION,
        SemanticErrorKind::InvalidSpecialization {
            child: "Derived".to_string(),
            parent: "Base".to_string(),
            reason: "incompatible types".to_string(),
        },
        "Cannot specialize Base with Derived: incompatible types".to_string(),
    );

    assert!(
        matches!(error.kind, SemanticErrorKind::InvalidSpecialization { .. }),
        "Expected InvalidSpecialization error kind"
    );
    assert!(error.message.contains("Derived"));
    assert!(error.message.contains("Base"));
}

#[test]
fn test_invalid_redefinition_error() {
    let error = SemanticError::new(
        SEMANTIC_INVALID_REDEFINITION,
        SemanticErrorKind::InvalidRedefinition {
            feature: "myFeature".to_string(),
            redefined: "baseFeature".to_string(),
            reason: "type mismatch".to_string(),
        },
        "Invalid redefinition of baseFeature by myFeature: type mismatch".to_string(),
    );

    assert!(
        matches!(error.kind, SemanticErrorKind::InvalidRedefinition { .. }),
        "Expected InvalidRedefinition error kind"
    );
}

#[test]
fn test_invalid_subsetting_error() {
    let error = SemanticError::new(
        SEMANTIC_INVALID_SUBSETTING,
        SemanticErrorKind::InvalidSubsetting {
            feature: "subFeature".to_string(),
            subset_of: "superFeature".to_string(),
            reason: "incompatible multiplicity".to_string(),
        },
        "Invalid subsetting: subFeature cannot subset superFeature".to_string(),
    );

    assert!(
        matches!(error.kind, SemanticErrorKind::InvalidSubsetting { .. }),
        "Expected InvalidSubsetting error kind"
    );
}

#[test]
fn test_constraint_violation_error() {
    let error = SemanticError::new(
        SEMANTIC_CONSTRAINT_VIOLATION,
        SemanticErrorKind::ConstraintViolation {
            constraint: "multiplicity".to_string(),
            reason: "expected 1..*, got 0..1".to_string(),
        },
        "Constraint violation: multiplicity - expected 1..*, got 0..1".to_string(),
    );

    assert!(
        matches!(error.kind, SemanticErrorKind::ConstraintViolation { .. }),
        "Expected ConstraintViolation error kind"
    );
}

#[test]
fn test_invalid_feature_context_error() {
    let error = SemanticError::new(
        SEMANTIC_INVALID_FEATURE_CONTEXT,
        SemanticErrorKind::InvalidFeatureContext {
            feature: "attribute".to_string(),
            context: "function".to_string(),
        },
        "Feature 'attribute' cannot be used in function context".to_string(),
    );

    assert!(
        matches!(error.kind, SemanticErrorKind::InvalidFeatureContext { .. }),
        "Expected InvalidFeatureContext error kind"
    );
}

#[test]
fn test_abstract_instantiation_error() {
    let error = SemanticError::new(
        SEMANTIC_ABSTRACT_INSTANTIATION,
        SemanticErrorKind::AbstractInstantiation {
            element: "AbstractClass".to_string(),
        },
        "Cannot instantiate abstract element 'AbstractClass'".to_string(),
    );

    assert!(
        matches!(error.kind, SemanticErrorKind::AbstractInstantiation { .. }),
        "Expected AbstractInstantiation error kind"
    );
}

#[test]
fn test_invalid_import_error() {
    let error = SemanticError::new(
        SEMANTIC_INVALID_IMPORT,
        SemanticErrorKind::InvalidImport {
            path: "NonExistent::Package".to_string(),
            reason: "package not found".to_string(),
        },
        "Invalid import 'NonExistent::Package': package not found".to_string(),
    );

    assert!(
        matches!(error.kind, SemanticErrorKind::InvalidImport { .. }),
        "Expected InvalidImport error kind"
    );
}

#[test]
fn test_empty_cycle_error() {
    let error = SemanticError::circular_dependency(vec![]);
    assert!(
        matches!(error.kind, SemanticErrorKind::CircularDependency { .. }),
        "Expected CircularDependency error kind"
    );
}

#[test]
fn test_single_element_cycle() {
    let cycle = vec!["A".to_string(), "A".to_string()];
    let error = SemanticError::circular_dependency(cycle);
    assert!(error.message.contains(SEMANTIC_CIRCULAR_DEPENDENCY_MSG));
    assert!(error.message.contains("A -> A"));
}

#[test]
fn test_error_equality() {
    let error1 = SemanticError::undefined_reference("Test".to_string());
    let error2 = SemanticError::undefined_reference("Test".to_string());
    assert_eq!(error1, error2);
}

#[test]
fn test_error_clone() {
    let error1 = SemanticError::undefined_reference("Test".to_string());
    let error2 = error1.clone();
    assert_eq!(error1, error2);
}

#[test]
fn test_location_equality() {
    let loc1 = Location {
        file: Some("test.sysml".to_string()),
        line: Some(10),
        column: Some(5),
    };
    let loc2 = Location {
        file: Some("test.sysml".to_string()),
        line: Some(10),
        column: Some(5),
    };
    assert_eq!(loc1, loc2);
}

#[test]
fn test_type_mismatch_with_empty_strings() {
    let error = SemanticError::type_mismatch("".to_string(), "".to_string(), "".to_string());
    assert!(
        matches!(error.kind, SemanticErrorKind::TypeMismatch { .. }),
        "Expected TypeMismatch error kind"
    );
}

#[test]
fn test_error_kind_variants_coverage() {
    // Test all SemanticErrorKind variants are constructible
    let _undefined = SemanticErrorKind::UndefinedReference {
        name: "test".to_string(),
    };
    let _duplicate = SemanticErrorKind::DuplicateDefinition {
        name: "test".to_string(),
        first_location: None,
    };
    let _type_mismatch = SemanticErrorKind::TypeMismatch {
        expected: "A".to_string(),
        found: "B".to_string(),
        context: "test".to_string(),
    };
    let _invalid_type = SemanticErrorKind::InvalidType {
        type_name: "T".to_string(),
    };
    let _invalid_spec = SemanticErrorKind::InvalidSpecialization {
        child: "C".to_string(),
        parent: "P".to_string(),
        reason: "R".to_string(),
    };
    let _invalid_redef = SemanticErrorKind::InvalidRedefinition {
        feature: "F".to_string(),
        redefined: "R".to_string(),
        reason: "reason".to_string(),
    };
    let _invalid_subset = SemanticErrorKind::InvalidSubsetting {
        feature: "F".to_string(),
        subset_of: "S".to_string(),
        reason: "reason".to_string(),
    };
    let _constraint = SemanticErrorKind::ConstraintViolation {
        constraint: "C".to_string(),
        reason: "R".to_string(),
    };
    let _invalid_context = SemanticErrorKind::InvalidFeatureContext {
        feature: "F".to_string(),
        context: "C".to_string(),
    };
    let _abstract_inst = SemanticErrorKind::AbstractInstantiation {
        element: "E".to_string(),
    };
    let _invalid_import = SemanticErrorKind::InvalidImport {
        path: "P".to_string(),
        reason: "R".to_string(),
    };
    let _circular = SemanticErrorKind::CircularDependency {
        cycle: vec!["A".to_string()],
    };
}

#[test]
fn test_workspace_event_creation() {
    let path = PathBuf::from("test.sysml");

    let added = WorkspaceEvent::FileAdded { path: path.clone() };
    let updated = WorkspaceEvent::FileUpdated { path: path.clone() };
    let removed = WorkspaceEvent::FileRemoved { path: path.clone() };

    assert!(matches!(added, WorkspaceEvent::FileAdded { .. }));
    assert!(matches!(updated, WorkspaceEvent::FileUpdated { .. }));
    assert!(matches!(removed, WorkspaceEvent::FileRemoved { .. }));
}

#[test]
fn test_workspace_event_equality() {
    let path = PathBuf::from("test.sysml");

    let event1 = WorkspaceEvent::FileUpdated { path: path.clone() };
    let event2 = WorkspaceEvent::FileUpdated { path: path.clone() };

    assert_eq!(event1, event2);
}

#[test]
fn test_dependency_event_creation() {
    let from = PathBuf::from("app.sysml");
    let to = PathBuf::from("base.sysml");

    let added = DependencyEvent::DependencyAdded {
        from: from.clone(),
        to: to.clone(),
    };
    let removed = DependencyEvent::FileRemoved { path: from.clone() };

    assert!(matches!(added, DependencyEvent::DependencyAdded { .. }));
    assert!(matches!(removed, DependencyEvent::FileRemoved { .. }));
}

#[test]
fn test_dependency_event_equality() {
    let from = PathBuf::from("app.sysml");
    let to = PathBuf::from("base.sysml");

    let event1 = DependencyEvent::DependencyAdded {
        from: from.clone(),
        to: to.clone(),
    };
    let event2 = DependencyEvent::DependencyAdded {
        from: from.clone(),
        to: to.clone(),
    };

    assert_eq!(event1, event2);
}

#[test]
fn test_symbol_table_event_creation() {
    let inserted = SymbolTableEvent::SymbolInserted {
        qualified_name: "Package::Vehicle".to_string(),
        symbol_id: 42,
    };
    let import_added = SymbolTableEvent::ImportAdded {
        import_path: "Base::*".to_string(),
    };
    let file_changed = SymbolTableEvent::FileChanged {
        file_path: "test.sysml".to_string(),
    };

    assert!(matches!(inserted, SymbolTableEvent::SymbolInserted { .. }));
    assert!(matches!(import_added, SymbolTableEvent::ImportAdded { .. }));
    assert!(matches!(file_changed, SymbolTableEvent::FileChanged { .. }));
}

#[test]
fn test_symbol_table_event_equality() {
    let event1 = SymbolTableEvent::SymbolInserted {
        qualified_name: "Package::Vehicle".to_string(),
        symbol_id: 42,
    };
    let event2 = SymbolTableEvent::SymbolInserted {
        qualified_name: "Package::Vehicle".to_string(),
        symbol_id: 42,
    };

    assert_eq!(event1, event2);
}

#[test]
fn test_diagnostic_creation() {
    let location = DiagLocation::new(
        "test.sysml",
        Range::new(Position::new(0, 5), Position::new(0, 10)),
    );

    let diag = Diagnostic::error("Undefined symbol", location.clone());

    assert_eq!(diag.severity, Severity::Error);
    assert_eq!(diag.message, "Undefined symbol");
    assert_eq!(diag.location.file, "test.sysml");
    assert_eq!(diag.location.range.start.line, 0);
    assert_eq!(diag.location.range.start.column, 5);
    assert_eq!(diag.code, None);
}

#[test]
fn test_diagnostic_with_code() {
    let location = DiagLocation::new("test.sysml", Range::single(0, 5));

    let diag = Diagnostic::error("Parse error", location).with_code("E001");

    assert_eq!(diag.code, Some("E001".to_string()));
}

#[test]
fn test_warning_diagnostic() {
    let location = DiagLocation::new("test.sysml", Range::single(2, 10));

    let diag = Diagnostic::warning("Unused variable", location);

    assert_eq!(diag.severity, Severity::Warning);
}

#[test]
fn test_single_char_range() {
    let range = Range::single(5, 10);

    assert_eq!(range.start.line, 5);
    assert_eq!(range.start.column, 10);
    assert_eq!(range.end.line, 5);
    assert_eq!(range.end.column, 11);
}

#[test]
fn test_diagnostic_display() {
    let location = DiagLocation::new("test.sysml", Range::single(0, 5));
    let diag = Diagnostic::error("Test error", location);

    let display = format!("{diag}");

    assert!(display.contains("test.sysml:1:6")); // 1-indexed display
    assert!(display.contains("Error"));
    assert!(display.contains("Test error"));
}

#[test]
fn test_multiline_range() {
    let range = Range::new(Position::new(0, 5), Position::new(2, 10));

    assert_eq!(range.start.line, 0);
    assert_eq!(range.end.line, 2);
}
