#![allow(clippy::unwrap_used)]

use crate::project::file_loader;
use crate::semantic::Workspace;
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

#[test]
fn test_performances_kerml_no_duplicate_symbols() {
    // Test that Performances.kerml loads without duplicate symbol errors
    // This file has "feature redefines dispatchScope default thisPerformance;"
    // where "thisPerformance" in the default clause should NOT be treated as a symbol definition

    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sysml.library/Kernel Libraries/Kernel Semantic Library/Performances.kerml");

    if !stdlib_path.exists() {
        panic!("Performances.kerml not found at {stdlib_path:?}");
    }

    let content = std::fs::read_to_string(&stdlib_path).expect("Failed to read Performances.kerml");

    let parse_result = file_loader::parse_with_result(&content, &stdlib_path);

    // Should parse successfully
    assert!(
        parse_result.content.is_some(),
        "Failed to parse Performances.kerml: {:?}",
        parse_result.errors
    );

    // Now try to populate it in a workspace
    let mut workspace = Workspace::<SyntaxFile>::new();

    if let SyntaxFile::KerML(kerml_file) = parse_result.content.unwrap() {
        workspace.add_file(stdlib_path.clone(), SyntaxFile::KerML(kerml_file));
    } else {
        panic!("Expected KerML file");
    }

    let result = workspace.populate_affected();

    // Check for duplicate symbol errors
    let has_duplicate_this_performance =
        result.is_err() && result.as_ref().err().unwrap().contains("thisPerformance");

    assert!(
        !has_duplicate_this_performance,
        "Performances.kerml should not have duplicate 'thisPerformance' symbol. Error: {:?}",
        result.err()
    );

    // Verify thisPerformance exists exactly once
    let this_perf_count = workspace
        .symbol_table()
        .iter_symbols()
        .filter(|sym| sym.name() == "thisPerformance")
        .count();

    assert_eq!(
        this_perf_count, 1,
        "Should have exactly one 'thisPerformance' definition, got {this_perf_count}"
    );
}

#[test]
fn test_observation_kerml_no_duplicate_symbols() {
    // Test that Observation.kerml loads without duplicate "observations" symbol

    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sysml.library/Kernel Libraries/Kernel Semantic Library/Observation.kerml");

    if !stdlib_path.exists() {
        panic!("Observation.kerml not found at {stdlib_path:?}");
    }

    let content = std::fs::read_to_string(&stdlib_path).expect("Failed to read Observation.kerml");

    let parse_result = file_loader::parse_with_result(&content, &stdlib_path);

    assert!(
        parse_result.content.is_some(),
        "Failed to parse Observation.kerml: {:?}",
        parse_result.errors
    );

    let mut workspace = Workspace::<SyntaxFile>::new();

    if let SyntaxFile::KerML(kerml_file) = parse_result.content.unwrap() {
        workspace.add_file(stdlib_path.clone(), SyntaxFile::KerML(kerml_file));
    } else {
        panic!("Expected KerML file");
    }

    let result = workspace.populate_affected();

    let has_duplicate_observations =
        result.is_err() && result.as_ref().err().unwrap().contains("observations");

    assert!(
        !has_duplicate_observations,
        "Observation.kerml should not have duplicate 'observations' symbol. Error: {:?}",
        result.err()
    );
}

#[test]
fn test_objects_kerml_no_duplicate_symbols() {
    // Test that Objects.kerml loads without duplicate "StructuredSpaceObject" symbol

    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sysml.library/Kernel Libraries/Kernel Semantic Library/Objects.kerml");

    if !stdlib_path.exists() {
        panic!("Objects.kerml not found at {stdlib_path:?}");
    }

    let content = std::fs::read_to_string(&stdlib_path).expect("Failed to read Objects.kerml");

    let parse_result = file_loader::parse_with_result(&content, &stdlib_path);

    assert!(
        parse_result.content.is_some(),
        "Failed to parse Objects.kerml: {:?}",
        parse_result.errors
    );

    let mut workspace = Workspace::<SyntaxFile>::new();

    if let SyntaxFile::KerML(kerml_file) = parse_result.content.unwrap() {
        workspace.add_file(stdlib_path.clone(), SyntaxFile::KerML(kerml_file));
    } else {
        panic!("Expected KerML file");
    }

    let result = workspace.populate_affected();

    let has_duplicate_structured = result.is_err()
        && result
            .as_ref()
            .err()
            .unwrap()
            .contains("StructuredSpaceObject");

    assert!(
        !has_duplicate_structured,
        "Objects.kerml should not have duplicate 'StructuredSpaceObject' symbol. Error: {:?}",
        result.err()
    );
}

#[test]
fn test_measurement_references_sysml_no_duplicate_package() {
    // Test that MeasurementReferences.sysml doesn't register the package symbol twice
    // The file has: standard library package MeasurementReferences { ... }
    // This should only create ONE symbol, not be duplicated

    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("sysml.library/Domain Libraries/Quantities and Units/MeasurementReferences.sysml");

    if !stdlib_path.exists() {
        panic!("MeasurementReferences.sysml not found at {stdlib_path:?}");
    }

    let content =
        std::fs::read_to_string(&stdlib_path).expect("Failed to read MeasurementReferences.sysml");

    let parse_result = file_loader::parse_with_result(&content, &stdlib_path);

    assert!(
        parse_result.content.is_some(),
        "Failed to parse MeasurementReferences.sysml: {:?}",
        parse_result.errors
    );

    let mut workspace = Workspace::<SyntaxFile>::new();

    if let SyntaxFile::SysML(sysml_file) = parse_result.content.unwrap() {
        workspace.add_file(stdlib_path.clone(), SyntaxFile::SysML(sysml_file));
    } else {
        panic!("Expected SysML file");
    }

    let result = workspace.populate_affected();

    let has_duplicate_package = result.is_err()
        && result
            .as_ref()
            .err()
            .unwrap()
            .contains("MeasurementReferences");

    assert!(
        !has_duplicate_package,
        "MeasurementReferences.sysml should not have duplicate package symbol. Error: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_feature_scopes() {
    // Test that features with the same name in different nested scopes don't conflict
    // This simulates the pattern in Observation.kerml where "observations" appears
    // both in the parent struct and in a nested step

    let input = r#"
        package NestedScopeTest {
            struct ChangeMonitor {
                private composite feature observations[0..*];
                
                step cancelObservation {
                    private feature observations[0..*];
                }
            }
        }
    "#;

    let path = PathBuf::from("test_nested_scopes.kerml");
    let parse_result = file_loader::parse_with_result(input, &path);

    assert!(
        parse_result.content.is_some(),
        "Failed to parse test input: {:?}",
        parse_result.errors
    );

    let mut workspace = Workspace::<SyntaxFile>::new();

    if let SyntaxFile::KerML(kerml_file) = parse_result.content.unwrap() {
        workspace.add_file(path.clone(), SyntaxFile::KerML(kerml_file));
    } else {
        panic!("Expected KerML file");
    }

    let result = workspace.populate_affected();

    // Should succeed - these are in different scopes
    // But currently fails due to scope handling bug
    let has_duplicate_observations =
        result.is_err() && result.as_ref().err().unwrap().contains("observations");

    assert!(
        !has_duplicate_observations,
        "Nested features with same name in different scopes should be allowed. Error: {:?}",
        result.err()
    );
}
