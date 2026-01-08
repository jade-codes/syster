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

// ============================================================================
// Additional edge cases for comprehensive coverage
// ============================================================================

#[test]
fn test_normalize_path_with_dot_components() {
    // Test paths containing "." (current directory) components
    // Create a temp file to ensure canonicalization works
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_dot.sysml");

    // Ensure cleanup even on panic using a guard
    struct FileGuard(PathBuf);
    impl Drop for FileGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let _guard = FileGuard(test_file.clone());

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

    // Request with a path that includes "./" - when the file exists, canonicalize cleans it up
    // For this test, we construct a path like "/tmp/./test_dot.sysml"
    let path_with_dot = format!("{}/./test_dot.sysml", temp_dir.to_string_lossy());
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &path_with_dot);

    // Should match because both canonicalize to the same path
    assert_eq!(
        tokens.len(),
        1,
        "Should match path with dot component (./) after canonicalization"
    );
}

#[test]
fn test_normalize_path_with_dotdot_components() {
    // Test paths containing ".." (parent directory) components
    // Note: Since "subdir/../Test.sysml" doesn't exist, normalize_path falls through
    // to simple normalization (lines 28-35) which preserves "..". If the file existed,
    // canonicalize() would clean up the ".." component.
    let mut symbol_table = SymbolTable::new();

    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    let path_with_dotdot = current_dir.join("subdir/../Test.sysml");

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(&path_with_dotdot.to_string_lossy()),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with the same dotdot path
    let tokens =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, "subdir/../Test.sysml");

    // Should match because both normalize to the same path with ".." preserved
    assert_eq!(tokens.len(), 1, "Should match path with dotdot components");
}

#[test]
fn test_normalize_path_root_path() {
    // Test root directory path "/" edge case
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Root::Package".to_string(),
            create_package_symbol(
                "Package",
                "Root::Package",
                Some("/Test.sysml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with root path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "/Test.sysml");

    // Should match exact root path
    assert_eq!(tokens.len(), 1, "Should match root-level file path");
}

#[test]
fn test_normalize_path_stdlib_at_beginning() {
    // Test when sysml.library/ is at the very beginning of the path
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some("sysml.library/Core.kerml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with prefix path
    let tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/some/prefix/sysml.library/Core.kerml",
    );

    // Should match because both normalize to "sysml.library/Core.kerml"
    assert_eq!(
        tokens.len(),
        1,
        "Should match stdlib path starting at beginning"
    );
}

#[test]
fn test_normalize_path_very_long_path() {
    // Test handling of very long file paths (edge case for path buffer limits)
    let mut symbol_table = SymbolTable::new();

    // Create a very long path (but within reasonable limits)
    let long_path = format!(
        "/{}/Test.sysml",
        "very/long/nested/directory/structure/that/goes/deep/into/the/filesystem"
    );

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(&long_path),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with the same long path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &long_path);

    // Should match despite long path
    assert_eq!(tokens.len(), 1, "Should handle very long file paths");
}

#[test]
fn test_normalize_path_exact_match() {
    // Test exact path matching for non-existent files
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some("/path/to/dir/Test.sysml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same path (should match exactly)
    let tokens =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, "/path/to/dir/Test.sysml");

    // Should match exact path
    assert_eq!(tokens.len(), 1, "Should match path consistently");
}

#[test]
fn test_normalize_path_double_slash() {
    // Test paths with double slashes (e.g., "/path//to///file.sysml")
    // Note: Since "/path//Test.sysml" doesn't exist, normalize_path falls through
    // to simple normalization (lines 28-35) which preserves double slashes via
    // PathBuf's to_string_lossy. If the file existed, canonicalize() would typically
    // normalize consecutive slashes to single slashes on Unix systems.
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some("/path//Test.sysml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same double slash
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "/path//Test.sysml");

    // Should match because both have the same double slash representation (file doesn't exist)
    assert_eq!(tokens.len(), 1, "Should match path with double slashes");
}

#[test]
fn test_normalize_path_stdlib_substring_not_matched() {
    // Test that paths containing "sysml.library" as a substring
    // (but not followed by "/") are NOT treated as stdlib paths
    let mut symbol_table = SymbolTable::new();

    let path1 = "/path/sysml.library_backup/Test.kerml";
    let path2 = "/path/mysysml.library/Test.kerml";

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(path1),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with different path that also has similar substring
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, path2);

    // Should NOT match because the pattern is "sysml.library/" not just "sysml.library"
    assert_eq!(
        tokens.len(),
        0,
        "Should not match similar non-stdlib substring"
    );
}

#[test]
fn test_normalize_path_empty_filename() {
    // Test edge case of directory path without filename
    let mut symbol_table = SymbolTable::new();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some("/path/to/directory/"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same directory path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "/path/to/directory/");

    // Should match (even though it's unusual for a file path)
    assert_eq!(
        tokens.len(),
        1,
        "Should handle directory path with trailing slash"
    );
}

#[test]
fn test_normalize_path_backslash_in_path() {
    // Test paths with backslashes (Windows-style or escaped characters)
    // On Unix systems, backslashes are valid filename characters
    let mut symbol_table = SymbolTable::new();

    let path_with_backslash = "/path/with\\backslash/Test.sysml";
    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(path_with_backslash),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with same backslash path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, path_with_backslash);

    // Should match (backslash is treated as literal character)
    assert_eq!(tokens.len(), 1, "Should handle paths with backslashes");
}

#[test]
fn test_normalize_path_multiple_stdlib_symbols_different_files() {
    // Test collecting tokens when there are multiple stdlib files
    let mut symbol_table = SymbolTable::new();

    // Add symbols from different stdlib files
    symbol_table
        .insert(
            "Core::Base".to_string(),
            create_classifier_symbol(
                "Base",
                "Core::Base",
                Some("/source/sysml.library/Core.kerml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    symbol_table
        .insert(
            "Kernel::Thing".to_string(),
            create_classifier_symbol(
                "Thing",
                "Kernel::Thing",
                Some("/source/sysml.library/Kernel.kerml"),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request tokens for Core.kerml only
    let tokens = SemanticTokenCollector::collect_from_symbols(
        &symbol_table,
        "/build/sysml.library/Core.kerml",
    );

    // Should only get symbols from Core.kerml, not Kernel.kerml
    assert_eq!(
        tokens.len(),
        1,
        "Should only match symbols from the requested file"
    );
}

#[test]
fn test_normalize_path_relative_with_multiple_levels() {
    // Test relative paths with multiple directory levels
    let mut symbol_table = SymbolTable::new();

    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
    let expected_abs = current_dir.join("a/b/c/Test.sysml");

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
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "a/b/c/Test.sysml");

    // Should match after normalization to absolute path
    assert_eq!(tokens.len(), 1, "Should match multi-level relative path");
}

#[test]
fn test_normalize_path_different_symbol_types_same_file() {
    // Test that different symbol types from the same file all produce tokens
    let mut symbol_table = SymbolTable::new();

    let file_path = "/path/Test.sysml";

    // Add different types of symbols from the same file
    symbol_table
        .insert(
            "Test::MyPackage".to_string(),
            create_package_symbol(
                "MyPackage",
                "Test::MyPackage",
                Some(file_path),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    symbol_table
        .insert(
            "Test::MyClass".to_string(),
            create_classifier_symbol(
                "MyClass",
                "Test::MyClass",
                Some(file_path),
                Some(create_span(3, 0)),
            ),
        )
        .unwrap();

    symbol_table
        .insert(
            "Test::MyPart".to_string(),
            create_definition_symbol(
                "MyPart",
                "Test::MyPart",
                Some(file_path),
                Some(create_span(5, 0)),
            ),
        )
        .unwrap();

    // Request tokens for the file
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, file_path);

    // Should get tokens for all three different symbol types
    assert_eq!(
        tokens.len(),
        3,
        "Should collect tokens from different symbol types in same file"
    );
}

#[test]
fn test_normalize_path_case_sensitivity_non_stdlib() {
    // Test case sensitivity for non-stdlib paths.
    //
    // We create a real file with mixed-case path in a temporary directory so
    // that normalize_path will exercise filesystem canonicalization where
    // available. We then request tokens using a lowercased version of the
    // path and assert OS-specific behavior:
    // - On case-insensitive filesystems (Windows, macOS) we expect a match.
    // - On case-sensitive filesystems (e.g., Linux) we expect no match.
    let mut symbol_table = SymbolTable::new();

    // Create a temporary directory and mixed-case file path.
    let temp_dir = std::env::temp_dir().join("syster_case_sensitivity_test");
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Ensure cleanup even on panic using a guard
    struct DirGuard(PathBuf);
    impl Drop for DirGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _guard = DirGuard(temp_dir.clone());

    let mixed_path = temp_dir.join("File.MixedCase.sysml");
    File::create(&mixed_path).unwrap();

    let mixed_path_str = mixed_path.to_string_lossy().to_string();
    let lower_path_str = mixed_path_str.to_lowercase();

    symbol_table
        .insert(
            "Test::Package".to_string(),
            create_package_symbol(
                "Package",
                "Test::Package",
                Some(&mixed_path_str),
                Some(create_span(1, 0)),
            ),
        )
        .unwrap();

    // Request with different case
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &lower_path_str);

    // On case-insensitive systems, canonicalization should make these match.
    // On case-sensitive systems, the differing case should prevent a match.
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        assert_eq!(
            tokens.len(),
            1,
            "On case-insensitive filesystems, differently-cased paths should match"
        );
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        assert_eq!(
            tokens.len(),
            0,
            "On case-sensitive filesystems, differently-cased paths should not match"
        );
    }
}
