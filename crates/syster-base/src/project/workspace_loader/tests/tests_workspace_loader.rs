#![allow(unused_imports)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::super::WorkspaceLoader;
use crate::semantic::Workspace;
use crate::syntax::SyntaxFile;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_workspace_loader_creation() {
    let _loader = WorkspaceLoader::new();
    // Verify it implements Default
    let _default_loader = WorkspaceLoader;
}

#[test]
fn test_load_missing_file() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    let result = loader.load_file("/nonexistent/file.sysml", &mut workspace);
    assert!(result.is_err(), "Should fail when loading nonexistent file");
    if let Err(err) = result {
        assert!(err.contains("Failed to read"));
    }
}

#[test]
fn test_load_missing_directory() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    let result = loader.load_directory("/nonexistent/directory", &mut workspace);
    assert!(
        result.is_err(),
        "Should fail when loading nonexistent directory"
    );
    assert!(result.unwrap_err().contains("Directory not found"));
}

#[test]
fn test_load_valid_file() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Create temporary file with valid SysML content
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.sysml");
    fs::write(&file_path, "package Test {}").expect("Failed to write test file");

    let result = loader.load_file(&file_path, &mut workspace);
    assert!(result.is_ok(), "Should load valid SysML file");
    assert_eq!(
        workspace.file_paths().count(),
        1,
        "Should have one file in workspace"
    );
}

#[test]
fn test_load_invalid_syntax() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Create temporary file with invalid syntax
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("invalid.sysml");
    fs::write(&file_path, "this is not valid sysml syntax @#$%")
        .expect("Failed to write test file");

    let result = loader.load_file(&file_path, &mut workspace);
    assert!(result.is_err(), "Should fail on invalid syntax");
    assert!(result.unwrap_err().contains("Parse error"));
}

#[test]
fn test_load_unsupported_extension() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Create temporary file with unsupported extension
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "package Test {}").expect("Failed to write test file");

    let result = loader.load_file(&file_path, &mut workspace);
    assert!(result.is_err(), "Should reject unsupported file extension");
    assert!(result.unwrap_err().contains("Unsupported file extension"));
}

#[test]
fn test_load_directory_with_multiple_files() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Create temporary directory with multiple files
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("file1.sysml"), "package File1 {}")
        .expect("Failed to write test file");
    fs::write(temp_dir.path().join("file2.sysml"), "package File2 {}")
        .expect("Failed to write test file");
    fs::write(temp_dir.path().join("ignored.txt"), "ignored").expect("Failed to write test file");

    let result = loader.load_directory(temp_dir.path(), &mut workspace);
    assert!(result.is_ok(), "Should load valid directory");
    assert_eq!(
        workspace.file_paths().count(),
        2,
        "Should load only .sysml files"
    );
}

#[test]
fn test_load_directory_recursive() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Create nested directory structure
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).expect("Failed to create subdirectory");

    fs::write(temp_dir.path().join("root.sysml"), "package Root {}")
        .expect("Failed to write test file");
    fs::write(subdir.join("nested.sysml"), "package Nested {}").expect("Failed to write test file");

    let result = loader.load_directory(temp_dir.path(), &mut workspace);
    assert!(result.is_ok(), "Should load directory recursively");
    assert_eq!(
        workspace.file_paths().count(),
        2,
        "Should find files in subdirectories"
    );
}

#[test]
fn test_load_directory_filters_extensions() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Create directory with mixed file types
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    fs::write(temp_dir.path().join("valid.sysml"), "package Valid {}")
        .expect("Failed to write test file");
    fs::write(temp_dir.path().join("readme.md"), "# README").expect("Failed to write test file");
    fs::write(temp_dir.path().join("config.json"), "{}").expect("Failed to write test file");

    let result = loader.load_directory(temp_dir.path(), &mut workspace);
    assert!(result.is_ok());
    assert_eq!(
        workspace.file_paths().count(),
        1,
        "Should only load supported extensions"
    );
}

#[test]
fn test_load_kerml_file_handled() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Create temporary KerML file
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let file_path = temp_dir.path().join("test.kerml");
    fs::write(&file_path, "class TestClass;").expect("Failed to write test file");

    let result = loader.load_file(&file_path, &mut workspace);
    // KerML is not yet implemented, so file won't be added but shouldn't error
    assert!(result.is_ok(), "KerML files should be handled gracefully");
}

#[test]
fn test_load_directory_continues_on_parse_error() {
    // Tests that the loader continues processing files when one fails to parse.
    // Previously, the loader would stop at the first error.
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create a valid file
    let valid_file = temp_dir.path().join("valid.sysml");
    fs::write(&valid_file, "package Valid {}").expect("Failed to write valid file");

    // Create an invalid file with syntax errors
    let invalid_file = temp_dir.path().join("invalid.sysml");
    fs::write(&invalid_file, "package { invalid syntax !!!").expect("Failed to write invalid file");

    // Create another valid file
    let valid_file2 = temp_dir.path().join("valid2.sysml");
    fs::write(&valid_file2, "package Valid2 {}").expect("Failed to write valid2 file");

    let result = loader.load_directory(temp_dir.path(), &mut workspace);

    // Should return an error mentioning the failed file
    assert!(result.is_err(), "Should report errors for failed files");
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("Failed to load"),
        "Error should mention failed loading"
    );
    assert!(
        err_msg.contains("invalid.sysml"),
        "Error should mention the failing file"
    );

    // But valid files should still be loaded
    assert_eq!(
        workspace.file_paths().count(),
        2,
        "Both valid files should be loaded despite one invalid file"
    );
}

#[test]
fn test_load_directory_collects_multiple_errors() {
    let loader = WorkspaceLoader::new();
    let mut workspace = Workspace::<SyntaxFile>::new();

    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Create multiple invalid files
    let invalid1 = temp_dir.path().join("invalid1.sysml");
    fs::write(&invalid1, "this is not valid sysml").expect("Failed to write");

    let invalid2 = temp_dir.path().join("invalid2.sysml");
    fs::write(&invalid2, "also not valid {{{{").expect("Failed to write");

    // Create one valid file
    let valid = temp_dir.path().join("valid.sysml");
    fs::write(&valid, "package Valid {}").expect("Failed to write");

    let result = loader.load_directory(temp_dir.path(), &mut workspace);

    // Should report both errors
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("2 file(s)"),
        "Should report count of failed files"
    );

    // Valid file should still be loaded
    assert_eq!(workspace.file_paths().count(), 1);
}
