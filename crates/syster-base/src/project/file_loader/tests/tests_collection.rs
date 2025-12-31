#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::super::collect_file_paths;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_collect_file_paths_empty_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let paths = collect_file_paths(&temp_dir.path().to_path_buf());

    assert!(paths.is_ok());
    assert_eq!(
        paths.unwrap().len(),
        0,
        "Empty directory should yield no files"
    );
}

#[test]
fn test_collect_file_paths_single_sysml_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.sysml");
    fs::write(&file_path, "part def Vehicle;").expect("Failed to write test file");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf());

    assert!(paths.is_ok());
    let paths = paths.unwrap();
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0], file_path);
}

#[test]
fn test_collect_file_paths_multiple_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create multiple supported files
    fs::write(temp_dir.path().join("file1.sysml"), "part def A;").expect("Failed to write");
    fs::write(temp_dir.path().join("file2.sysml"), "part def B;").expect("Failed to write");
    fs::write(temp_dir.path().join("file3.kerml"), "class C;").expect("Failed to write");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf());

    assert!(paths.is_ok());
    let paths = paths.unwrap();
    assert_eq!(paths.len(), 3);
}

#[test]
fn test_collect_file_paths_ignores_unsupported_extensions() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    fs::write(temp_dir.path().join("test.sysml"), "part def A;").expect("Failed to write");
    fs::write(temp_dir.path().join("readme.txt"), "Text file").expect("Failed to write");
    fs::write(temp_dir.path().join("data.json"), "{}").expect("Failed to write");
    fs::write(temp_dir.path().join("script.sh"), "#!/bin/bash").expect("Failed to write");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf());

    assert!(paths.is_ok());
    let paths = paths.unwrap();
    assert_eq!(paths.len(), 1, "Should only collect .sysml file");
    assert!(paths[0].ends_with("test.sysml"));
}

#[test]
fn test_collect_file_paths_recursive() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create nested directory structure
    let sub_dir1 = temp_dir.path().join("sub1");
    let sub_dir2 = temp_dir.path().join("sub1").join("sub2");
    fs::create_dir(&sub_dir1).expect("Failed to create sub1");
    fs::create_dir(&sub_dir2).expect("Failed to create sub2");

    // Create files at different levels
    fs::write(temp_dir.path().join("root.sysml"), "part def Root;").expect("Failed to write");
    fs::write(sub_dir1.join("level1.sysml"), "part def L1;").expect("Failed to write");
    fs::write(sub_dir2.join("level2.sysml"), "part def L2;").expect("Failed to write");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf());

    assert!(paths.is_ok());
    let paths = paths.unwrap();
    assert_eq!(paths.len(), 3, "Should recursively find all files");
}

#[test]
fn test_collect_file_paths_mixed_extensions_nested() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let sub_dir = temp_dir.path().join("models");
    fs::create_dir(&sub_dir).expect("Failed to create subdirectory");

    fs::write(temp_dir.path().join("main.sysml"), "part def Main;").expect("Failed to write");
    fs::write(sub_dir.join("base.kerml"), "class Base;").expect("Failed to write");
    fs::write(sub_dir.join("readme.md"), "# Docs").expect("Failed to write");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf());

    assert!(paths.is_ok());
    let paths = paths.unwrap();
    assert_eq!(paths.len(), 2, "Should collect .sysml and .kerml, not .md");
}

#[test]
fn test_collect_file_paths_nonexistent_directory() {
    let nonexistent = PathBuf::from("/nonexistent/path/that/does/not/exist");
    let result = collect_file_paths(&nonexistent);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to read directory"));
}

#[test]
fn test_collect_file_paths_file_not_directory() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.sysml");
    fs::write(&file_path, "part def Vehicle;").expect("Failed to write test file");

    // Try to collect from a file instead of directory
    let result = collect_file_paths(&file_path);

    assert!(
        result.is_err(),
        "Should fail when path is a file, not directory"
    );
}

#[test]
fn test_collect_file_paths_preserves_full_paths() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.sysml");
    fs::write(&file_path, "part def Vehicle;").expect("Failed to write test file");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf()).unwrap();

    assert_eq!(paths.len(), 1);
    assert!(paths[0].is_absolute(), "Should return absolute paths");
    assert_eq!(paths[0], file_path);
}

#[test]
fn test_collect_file_paths_empty_subdirectories() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    let sub_dir1 = temp_dir.path().join("empty1");
    let sub_dir2 = temp_dir.path().join("empty2");
    fs::create_dir(&sub_dir1).expect("Failed to create sub1");
    fs::create_dir(&sub_dir2).expect("Failed to create sub2");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf()).unwrap();

    assert_eq!(
        paths.len(),
        0,
        "Empty subdirectories should not affect result"
    );
}

#[test]
fn test_collect_file_paths_hidden_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    fs::write(temp_dir.path().join(".hidden.sysml"), "part def Hidden;")
        .expect("Failed to write hidden file");
    fs::write(temp_dir.path().join("visible.sysml"), "part def Visible;")
        .expect("Failed to write visible file");

    let paths = collect_file_paths(&temp_dir.path().to_path_buf()).unwrap();

    // Should include both hidden and visible files
    assert_eq!(paths.len(), 2, "Should collect hidden files too");
}
