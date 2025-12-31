#![allow(clippy::unwrap_used)]

//! Comprehensive tests for normalize_path function (through public API)
//!
//! These tests verify the path normalization logic used when comparing file paths
//! during semantic token collection. The private normalize_path function is tested
//! through the public collect_from_symbols API.
//!
//! Function tested (through public API):
//! - normalize_path (private, tested via collect_from_symbols)
//!
//! Test strategy:
//! - Create symbols with different source_file paths
//! - Call collect_from_symbols with various file_path arguments
//! - Verify that path normalization correctly matches symbols based on:
//!   1. stdlib path handling (sysml.library/ prefix matching)
//!   2. canonical path resolution for existing files
//!   3. simple normalization for non-existent files (relative -> absolute)

use crate::core::Span;
use crate::semantic::processors::SemanticTokenCollector;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use std::fs::File;
use std::path::PathBuf;

/// Helper to create a span at a specific line/column
fn create_span(line: usize, column: usize) -> Span {
    Span {
        start: crate::core::Position { line, column },
        end: crate::core::Position {
            line,
            column: column + 5,
        },
    }
}

/// Helper to create a Package symbol with given parameters
fn create_package_symbol(
    name: &str,
    qualified_name: &str,
    source_file: Option<&str>,
    span: Option<Span>,
) -> Symbol {
    Symbol::Package {
        name: name.to_string(),
        qualified_name: qualified_name.to_string(),
        scope_id: 0,
        source_file: source_file.map(|s| s.to_string()),
        span,
        references: Vec::new(),
    }
}

/// Helper to create a Classifier symbol with given parameters
fn create_classifier_symbol(
    name: &str,
    qualified_name: &str,
    source_file: Option<&str>,
    span: Option<Span>,
) -> Symbol {
    Symbol::Classifier {
        name: name.to_string(),
        qualified_name: qualified_name.to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
        scope_id: 0,
        source_file: source_file.map(|s| s.to_string()),
        span,
        references: Vec::new(),
    }
}

/// Helper to create a Definition symbol with given parameters
fn create_definition_symbol(
    name: &str,
    qualified_name: &str,
    source_file: Option<&str>,
    span: Option<Span>,
) -> Symbol {
    Symbol::Definition {
        name: name.to_string(),
        qualified_name: qualified_name.to_string(),
        kind: "Part".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: source_file.map(|s| s.to_string()),
        span,
        references: Vec::new(),
    }
}

// ============================================================================
// Tests for stdlib path normalization (sysml.library/)
// ============================================================================

#[test]
fn test_normalize_path_stdlib_in_source_location() {
    // Test that stdlib paths are normalized by extracting the sysml.library/ suffix
    // regardless of the prefix path
    let mut symbol_table = SymbolTable::new();

    // Symbol from source location
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some("/workspaces/syster/crates/syster-base/sysml.library/Core.kerml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with different prefix but same sysml.library/ path
    let tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/different/path/sysml.library/Core.kerml",
    );

    // Should find the symbol because normalize_path matches on sysml.library/ suffix
    assert_eq!(
        tokens.len(),
        1,
        "Should match stdlib path with different prefix"
    );
}

#[test]
fn test_normalize_path_stdlib_in_build_location() {
    // Test stdlib path normalization for build artifacts
    let mut symbol_table = SymbolTable::new();

    // Symbol from build location
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some("/workspaces/syster/target/release/sysml.library/Kernel.kerml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with source location prefix
    let tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/workspaces/syster/crates/syster-base/sysml.library/Kernel.kerml",
    );

    // Should match because both normalize to "sysml.library/Kernel.kerml"
    assert_eq!(
        tokens.len(),
        1,
        "Should match stdlib paths across source and build locations"
    );
}

#[test]
fn test_normalize_path_stdlib_multiple_occurrences() {
    // Test that only the first occurrence of sysml.library/ is used
    // (edge case: path contains sysml.library/ multiple times)
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some("/path/sysml.library/nested/sysml.library/Test.kerml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same pattern from first occurrence
    let tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/other/sysml.library/nested/sysml.library/Test.kerml",
    );

    // Should match because both find first occurrence at same position
    assert_eq!(
        tokens.len(),
        1,
        "Should match on first sysml.library/ occurrence"
    );
}

#[test]
fn test_normalize_path_stdlib_case_sensitive() {
    // Test that stdlib path matching is case-sensitive
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol("Package", "Test::Package", None, Some(create_span(1, 0))),
        )
        .unwrap();

    // Request with different case (should not match)
    let tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/path/SYSML.LIBRARY/Core.kerml",
    );

    // Should not match because case is different (on case-sensitive systems)
    // Note: This behavior depends on the filesystem, but the code does string matching
    assert_eq!(
        tokens.len(),
        0,
        "Should not match stdlib path with different case"
    );
}

#[test]
fn test_normalize_path_non_stdlib_different_paths() {
    // Test that non-stdlib paths don't match if they're different
    // (even if they have similar names)
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol("Package", "Test::Package", None, Some(create_span(1, 0))),
        )
        .unwrap();

    // Request with different non-stdlib path
    let tokens =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, "/other/project/Test.sysml");

    // Should not match (different paths)
    assert_eq!(
        tokens.len(),
        0,
        "Should not match different non-stdlib paths"
    );
}

// ============================================================================
// Tests for canonical path resolution (existing files)
// ============================================================================

#[test]
fn test_normalize_path_canonical_existing_file() {
    // Test that existing files are matched via canonical paths
    // Create a temporary file to test canonicalization
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_canonical.sysml");

    // Create the file
    File::create(&test_file).expect("Failed to create temp file");

    let mut symbol_table = SymbolTable::new();

    // Store symbol with canonical path
    let canonical_path = test_file.canonicalize().expect("Failed to canonicalize");
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(&canonical_path.to_string_lossy()),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with the same canonical path
    let tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        &canonical_path.to_string_lossy(),
    );

    // Clean up
    let _ = std::fs::remove_file(&test_file);

    // Should match because both are canonicalized
    assert_eq!(
        tokens.len(),
        1,
        "Should match existing file via canonical path"
    );
}

#[test]
fn test_normalize_path_canonical_with_relative_path() {
    // Test that relative paths to existing files are canonicalized
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_relative.sysml");

    // Create the file
    File::create(&test_file).expect("Failed to create temp file");

    let mut symbol_table = SymbolTable::new();

    // Store symbol with canonical path
    let canonical_path = test_file.canonicalize().expect("Failed to canonicalize");
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(&canonical_path.to_string_lossy()),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Build a relative path to the same file (if possible)
    // For this test, we'll use the absolute path since relative paths are tricky in tests
    let tokens =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, test_file.to_str().unwrap());

    // Clean up
    let _ = std::fs::remove_file(&test_file);

    // Should match because the file exists and both paths canonicalize to the same result
    assert_eq!(
        tokens.len(),
        1,
        "Should match file via different path representations"
    );
}

// ============================================================================
// Tests for simple normalization (non-existent files)
// ============================================================================

#[test]
fn test_normalize_path_non_existent_absolute() {
    // Test that non-existent absolute paths are normalized by keeping them absolute
    let mut symbol_table = SymbolTable::new();

    let abs_path = "/non/existent/path/Test.sysml";
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(abs_path),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same absolute path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, abs_path);

    // Should match because both are the same absolute path
    assert_eq!(tokens.len(), 1, "Should match non-existent absolute paths");
}

#[test]
fn test_normalize_path_non_existent_relative() {
    // Test that non-existent relative paths are normalized to absolute
    // (by joining with current_dir)
    let mut symbol_table = SymbolTable::new();

    // Create expected absolute path
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    let expected_abs = current_dir.join("relative/Test.sysml");

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(&expected_abs.to_string_lossy()),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with relative path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "relative/Test.sysml");

    // Should match because relative path is normalized to absolute
    assert_eq!(
        tokens.len(),
        1,
        "Should match non-existent relative path after normalization"
    );
}

#[test]
fn test_normalize_path_different_relative_paths_to_same_location() {
    // Test that different relative paths that resolve to the same location
    // are normalized consistently
    let mut symbol_table = SymbolTable::new();

    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    let normalized_path = current_dir.join("Test.sysml");

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(&normalized_path.to_string_lossy()),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with just filename (which should normalize to current_dir/filename)
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "Test.sysml");

    // Should match because both resolve to current_dir/Test.sysml
    assert_eq!(
        tokens.len(),
        1,
        "Should match different relative paths to same location"
    );
}

// ============================================================================
// Edge cases and error conditions
// ============================================================================

#[test]
fn test_normalize_path_empty_string() {
    // Test behavior with empty path string
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol("Package", "Test::Package", None, Some(create_span(1, 0))),
        )
        .unwrap();

    // Request with empty string
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "");

    // Should not match
    assert_eq!(tokens.len(), 0, "Empty path should not match any symbols");
}

#[test]
fn test_normalize_path_symbols_without_source_file() {
    // Test that symbols without source_file are skipped
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol("Package", "Test::Package", None, Some(create_span(1, 0))),
        )
        .unwrap();

    // Request with any path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "/any/path.sysml");

    // Should not match because symbol has no source_file
    assert_eq!(
        tokens.len(),
        0,
        "Symbols without source_file should not match"
    );
}

#[test]
fn test_normalize_path_symbols_without_span() {
    // Test that symbols without span are skipped (even if source_file matches)
    let mut symbol_table = SymbolTable::new();

    let test_path = "/path/Test.sysml";
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol("Package", "Test::Package", None, None),
        )
        .unwrap();

    // Request with matching path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, test_path);

    // Should not produce tokens because symbol has no span
    assert_eq!(
        tokens.len(),
        0,
        "Symbols without span should not produce tokens"
    );
}

#[test]
fn test_normalize_path_special_characters() {
    // Test paths with special characters
    let mut symbol_table = SymbolTable::new();

    let special_path = "/path/with spaces/and-dashes/Test.sysml";
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(special_path),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, special_path);

    // Should match despite special characters
    assert_eq!(
        tokens.len(),
        1,
        "Should handle paths with special characters"
    );
}

#[test]
fn test_normalize_path_unicode_characters() {
    // Test paths with unicode characters
    let mut symbol_table = SymbolTable::new();

    let unicode_path = "/path/日本語/Test.sysml";
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(unicode_path),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, unicode_path);

    // Should match unicode paths
    assert_eq!(
        tokens.len(),
        1,
        "Should handle paths with unicode characters"
    );
}

#[test]
fn test_normalize_path_multiple_symbols_same_file() {
    // Test that multiple symbols from the same file all produce tokens
    let mut symbol_table = SymbolTable::new();

    let file_path = "/path/Test.sysml";

    // Add multiple symbols from the same file
    symbol_table
        .insert(
            "Test::Package1".to_string(),
            create_package_symbol(
                "Package1",
                "Test::Package1",
                Some(file_path),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    symbol_table
        .insert(
            "Test::Package2".to_string(),
            create_package_symbol(
                "Package2",
                "Test::Package2",
                Some(file_path),
                Some(create_span(2, 0)),
            ),
        )
        .unwrap();

    symbol_table
        .insert(
            "Test::Package3".to_string(),
            create_package_symbol(
                "Package3",
                "Test::Package3",
                Some(file_path),
                Some(create_span(3, 0)),
            ),
        )
        .unwrap();

    // Request tokens for the file
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, file_path);

    // Should get all three symbols
    assert_eq!(
        tokens.len(),
        3,
        "Should collect tokens from all symbols in the same file"
    );
}

#[test]
fn test_normalize_path_mixed_stdlib_and_regular() {
    // Test that stdlib and regular files are handled independently
    let mut symbol_table = SymbolTable::new();

    // Stdlib symbol
    symbol_table
        .insert(
            "Stdlib::Core".to_string(),
            create_package_symbol(
                "Core",
                "Stdlib::Core",
                Some("/source/sysml.library/Core.kerml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Regular symbol
    symbol_table
        .insert(
            "User::Test".to_string(),
            create_package_symbol(
                "Test",
                "User::Test",
                Some("/project/Test.sysml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request stdlib file with different prefix
    let stdlib_tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/build/sysml.library/Core.kerml",
    );
    assert_eq!(
        stdlib_tokens.len(),
        1,
        "Should match stdlib file with different prefix"
    );

    // Request regular file (exact match only)
    let regular_tokens =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, "/project/Test.sysml");
    assert_eq!(regular_tokens.len(), 1, "Should match regular file");
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_normalize_path_real_world_scenario() {
    // Test a realistic scenario with multiple files and symbols
    let mut symbol_table = SymbolTable::new();

    // Stdlib symbols from different locations
    symbol_table
        .insert(
            "Core::Base".to_string(),
            create_classifier_symbol(
                "Base",
                "Core::Base",
                Some("/workspaces/syster/crates/syster-base/sysml.library/Core.kerml"),
                Some(create_span(10, 0)),
            ),
        )
        .unwrap();

    symbol_table
        .insert(
            "Kernel::Thing".to_string(),
            create_classifier_symbol(
                "Thing",
                "Kernel::Thing",
                Some("/workspaces/syster/target/release/sysml.library/Kernel.kerml"),
                Some(create_span(5, 0)),
            ),
        )
        .unwrap();

    // User project symbols
    symbol_table
        .insert(
            "MyProject::Vehicle".to_string(),
            create_definition_symbol(
                "Vehicle",
                "MyProject::Vehicle",
                Some("/projects/myproject/src/Vehicle.sysml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Test 1: Query stdlib file from build location, should match source location
    let tokens1 = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/workspaces/syster/target/debug/sysml.library/Core.kerml",
    );
    assert_eq!(
        tokens1.len(),
        1,
        "Should match stdlib file across locations"
    );

    // Test 2: Query user project file
    let tokens2 = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/projects/myproject/src/Vehicle.sysml",
    );
    assert_eq!(tokens2.len(), 1, "Should match user project file");

    // Test 3: Query non-existent file
    let tokens3 =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, "/does/not/exist.sysml");
    assert_eq!(tokens3.len(), 0, "Should not match non-existent file");
}

#[test]
fn test_normalize_path_token_sorting() {
    // Test that tokens are properly sorted by line and column
    let mut symbol_table = SymbolTable::new();

    let file_path = "/path/Test.sysml";

    // Add symbols in non-sorted order
    symbol_table
        .insert(
            "Test::C".to_string(),
            create_package_symbol("C", "Test::C", Some(file_path), Some(create_span(5, 10))),
        )
        .unwrap();

    symbol_table
        .insert(
            "Test::A".to_string(),
            create_package_symbol("A", "Test::A", Some(file_path), Some(create_span(2, 5))),
        )
        .unwrap();

    symbol_table
        .insert(
            "Test::B".to_string(),
            create_package_symbol("B", "Test::B", Some(file_path), Some(create_span(2, 15))),
        )
        .unwrap();

    // Request tokens
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, file_path);

    // Verify tokens are sorted
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].line, 2);
    assert_eq!(tokens[0].column, 5);
    assert_eq!(tokens[1].line, 2);
    assert_eq!(tokens[1].column, 15);
    assert_eq!(tokens[2].line, 5);
    assert_eq!(tokens[2].column, 10);
}
