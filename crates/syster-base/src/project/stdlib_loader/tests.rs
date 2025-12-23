#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::StdLibLoader;
use crate::core::constants::SUPPORTED_EXTENSIONS;
use crate::project::file_loader;
use crate::semantic::Workspace;
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

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
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let mut workspace = Workspace::<SyntaxFile>::new();
    let result = loader.load(&mut workspace);

    assert!(result.is_ok(), "Loading stdlib should succeed");
    assert!(workspace.has_stdlib(), "Stdlib should be marked as loaded");
}

#[test]
fn test_collect_file_paths() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let paths = file_loader::collect_file_paths(&loader.stdlib_path);
    assert!(paths.is_ok(), "Should collect paths successfully");

    let paths = paths.unwrap();
    // Should find at least some .sysml files
    let sysml_files: Vec<_> = paths
        .iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("sysml"))
        .collect();

    assert!(
        !sysml_files.is_empty(),
        "Should find at least one .sysml file in stdlib"
    );

    // Verify we're finding the exact number of files (stdlib has 58 .sysml files)
    assert_eq!(
        sysml_files.len(),
        58,
        "Expected exactly 58 .sysml files in stdlib, found {}",
        sysml_files.len()
    );
}

#[test]
fn test_supported_extensions_only() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let paths = file_loader::collect_file_paths(&loader.stdlib_path).unwrap();

    // All collected paths should have supported extensions
    let unsupported: Vec<_> = paths
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
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let mut workspace = Workspace::<SyntaxFile>::new();
    // Load once
    let result1 = loader.load(&mut workspace);
    assert!(result1.is_ok());

    // Should be able to load multiple times (idempotent)
    let mut workspace2 = Workspace::<SyntaxFile>::new();
    let result2 = loader.load(&mut workspace2);
    assert!(result2.is_ok());
}

#[test]
fn test_files_added_to_workspace() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let mut workspace = Workspace::<SyntaxFile>::new();
    let result = loader.load(&mut workspace);
    assert!(result.is_ok());

    let file_count = workspace.file_paths().count();

    assert!(
        file_count == 94,
        "Expected 94 files in workspace after loading stdlib, found {file_count}"
    );

    assert!(workspace.has_stdlib());
}

#[test]
fn test_kerml_files_handled() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let paths = file_loader::collect_file_paths(&loader.stdlib_path).unwrap();

    let kerml_count = paths
        .iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("kerml"))
        .count();

    assert!(
        kerml_count == 36,
        "Expected 36 .kerml files, found {kerml_count}"
    );
}

#[test]
fn test_lazy_loader_does_not_load_immediately() {
    let _loader = StdLibLoader::lazy();
    let workspace = Workspace::<SyntaxFile>::new();

    // Should not have stdlib loaded yet
    assert!(!workspace.has_stdlib(), "Stdlib should not be loaded yet");
    assert_eq!(
        workspace.file_count(),
        0,
        "Should have no files before lazy load"
    );
}

#[test]
fn test_lazy_load_on_demand() {
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::<SyntaxFile>::new();

    assert!(!workspace.has_stdlib(), "Should not be loaded initially");

    // First call to ensure_loaded should load the stdlib
    loader.ensure_loaded(&mut workspace).unwrap();

    assert!(
        workspace.has_stdlib(),
        "Should be loaded after ensure_loaded"
    );
    assert!(
        workspace.file_count() > 0,
        "Should have stdlib files loaded"
    );
}

#[test]
fn test_lazy_load_only_once() {
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // First load
    loader.ensure_loaded(&mut workspace).unwrap();
    let first_count = workspace.file_count();

    // Second request - should not reload (file count stays the same)
    loader.ensure_loaded(&mut workspace).unwrap();
    assert_eq!(
        workspace.file_count(),
        first_count,
        "Should not reload on second ensure_loaded call"
    );
}

#[test]
fn test_can_check_if_stdlib_loaded() {
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::<SyntaxFile>::new();

    assert!(!workspace.has_stdlib(), "Initially not loaded");
    assert!(!loader.is_loaded(), "Loader should report not loaded");

    // After loading
    loader.ensure_loaded(&mut workspace).unwrap();
    assert!(
        workspace.has_stdlib(),
        "Should be loaded after ensure_loaded"
    );
    assert!(loader.is_loaded(), "Loader should report loaded");
}

#[test]
fn test_eager_load_still_works() {
    let loader = StdLibLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Eager load immediately loads
    loader.load(&mut workspace).unwrap();

    assert!(
        workspace.has_stdlib(),
        "Should be loaded after eager load()"
    );
    assert!(
        workspace.file_count() > 0,
        "Should have stdlib files loaded"
    );
}

#[test]
fn test_lazy_avoids_reloading() {
    let mut loader = StdLibLoader::lazy();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Manually mark stdlib as loaded (simulate pre-loaded state)
    workspace.mark_stdlib_loaded();

    // ensure_loaded should respect existing stdlib
    loader.ensure_loaded(&mut workspace).unwrap();

    // Should not have added files since stdlib was already marked
    assert!(workspace.has_stdlib());
}
