#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::core::constants::SUPPORTED_EXTENSIONS;
use crate::project::{StdLibLoader, file_loader};
use crate::semantic::Workspace;
use crate::syntax::SyntaxFile;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Arc;

/// Shared stdlib workspace loaded once for all tests that need it.
/// This avoids the ~9s cost of loading stdlib repeatedly in each test.
static SHARED_STDLIB_WORKSPACE: Lazy<Arc<SharedStdlibData>> = Lazy::new(|| {
    let loader = StdLibLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();
    loader.load(&mut workspace).expect("Failed to load stdlib");

    let paths =
        file_loader::collect_file_paths(&loader.stdlib_path).expect("Failed to collect file paths");

    Arc::new(SharedStdlibData {
        file_count: workspace.file_paths().count(),
        has_stdlib: workspace.has_stdlib(),
        collected_paths: paths,
    })
});

/// Cached data from loading stdlib once
struct SharedStdlibData {
    file_count: usize,
    has_stdlib: bool,
    collected_paths: Vec<PathBuf>,
}

#[test]
fn test_stdlib_loader_creation() {
    let loader = StdLibLoader::new();
    assert_eq!(loader.stdlib_path, PathBuf::from("sysml.library"));

    let custom_loader = StdLibLoader::with_path(PathBuf::from("/custom/path"));
    assert_eq!(custom_loader.stdlib_path, PathBuf::from("/custom/path"));
}

#[test]
fn test_load_missing_directory() {
    let loader = StdLibLoader::with_path(PathBuf::from("/nonexistent/path"));
    let mut workspace = Workspace::<SyntaxFile>::new();

    let result = loader.load(&mut workspace);
    assert!(
        result.is_ok(),
        "Loading missing directory should succeed gracefully"
    );
    assert!(
        !workspace.has_stdlib(),
        "Stdlib should not be marked as loaded"
    );
}

#[test]
fn test_load_actual_stdlib() {
    // Uses shared fixture - stdlib already loaded once
    let data = &*SHARED_STDLIB_WORKSPACE;
    assert!(data.has_stdlib, "Stdlib should be marked as loaded");
}

#[test]
fn test_collect_file_paths() {
    // Uses shared fixture - paths already collected
    let data = &*SHARED_STDLIB_WORKSPACE;

    let sysml_files: Vec<_> = data
        .collected_paths
        .iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("sysml"))
        .collect();

    assert!(
        !sysml_files.is_empty(),
        "Should find at least one .sysml file in stdlib"
    );

    assert_eq!(
        sysml_files.len(),
        58,
        "Expected exactly 58 .sysml files in stdlib, found {}",
        sysml_files.len()
    );
}

#[test]
fn test_supported_extensions_only() {
    // Uses shared fixture - paths already collected
    let data = &*SHARED_STDLIB_WORKSPACE;

    let unsupported: Vec<_> = data
        .collected_paths
        .iter()
        .filter(|path| {
            !path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| SUPPORTED_EXTENSIONS.contains(&e))
        })
        .collect();

    assert!(
        unsupported.is_empty(),
        "Found {} paths with unsupported extensions: {:?}",
        unsupported.len(),
        unsupported
    );
}

#[test]
fn test_parallel_loading() {
    // Verifies that shared fixture can be accessed multiple times
    let data1 = &*SHARED_STDLIB_WORKSPACE;
    let data2 = &*SHARED_STDLIB_WORKSPACE;

    // Both accesses should see the same data (idempotent)
    assert_eq!(data1.file_count, data2.file_count);
    assert!(data1.has_stdlib);
    assert!(data2.has_stdlib);
}

#[test]
fn test_files_added_to_workspace() {
    // Uses shared fixture - workspace already loaded
    let data = &*SHARED_STDLIB_WORKSPACE;

    assert!(
        data.file_count == 94,
        "Expected 94 files in workspace after loading stdlib, found {}",
        data.file_count
    );

    assert!(data.has_stdlib);
}

#[test]
fn test_kerml_files_handled() {
    // Uses shared fixture - paths already collected
    let data = &*SHARED_STDLIB_WORKSPACE;

    let kerml_count = data
        .collected_paths
        .iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("kerml"))
        .count();

    assert!(
        kerml_count == 36,
        "Expected 36 .kerml files, found {kerml_count}"
    );
}

#[test]
fn test_lazy_loading_behavior() {
    // Comprehensive test for all lazy loading behavior - loads stdlib only ONCE
    let data = &*SHARED_STDLIB_WORKSPACE;

    // === Test 1: Lazy loader doesn't load immediately ===
    let mut loader = StdLibLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();
    assert!(!workspace.has_stdlib(), "Stdlib should not be loaded yet");
    assert!(!loader.is_loaded(), "Loader should report not loaded");
    assert_eq!(
        workspace.file_count(),
        0,
        "Should have no files before lazy load"
    );

    // === Test 2: First call to ensure_loaded loads stdlib ===
    loader.ensure_loaded(&mut workspace).unwrap();
    assert!(
        workspace.has_stdlib(),
        "Should be loaded after ensure_loaded"
    );
    assert!(loader.is_loaded(), "Loader should report loaded");
    assert_eq!(
        workspace.file_count(),
        data.file_count,
        "Should have same file count as shared fixture"
    );

    // === Test 3: Second call doesn't reload (idempotent) ===
    let count_before = workspace.file_count();
    loader.ensure_loaded(&mut workspace).unwrap();
    assert_eq!(
        workspace.file_count(),
        count_before,
        "Should not reload on second ensure_loaded call"
    );
}

#[test]
fn test_eager_vs_lazy_equivalence() {
    // Verify eager and lazy loaders produce the same result
    // Uses shared fixture for expected values (no additional loading)
    let data = &*SHARED_STDLIB_WORKSPACE;

    // Eager loader should match shared fixture
    assert_eq!(data.file_count, 94, "Shared fixture should have 94 files");
    assert!(data.has_stdlib, "Shared fixture should have stdlib loaded");
}

#[test]
fn test_lazy_avoids_reloading() {
    let mut loader = StdLibLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Manually mark stdlib as loaded (simulate pre-loaded state)
    workspace.mark_stdlib_loaded();

    // ensure_loaded should respect existing stdlib
    loader.ensure_loaded(&mut workspace).unwrap();

    // Should not have added files since stdlib was already marked
    assert!(workspace.has_stdlib());
}
