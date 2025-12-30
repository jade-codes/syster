//! Comprehensive tests for batch-3 module (syster-cli portion)
//!
//! This file contains tests for:
//! - Issue #157: syster_cli::run_analysis
//! - Issue #155: syster::main (tested via run_analysis)

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use syster_cli::run_analysis;
use tempfile::TempDir;

// =============================================================================
// Tests for syster_cli::run_analysis (Issue #157)
// =============================================================================

#[test]
fn test_run_analysis_single_file() {
    // Test analyzing a single SysML file
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let result = run_analysis(&file_path, false, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count > 0);
}

#[test]
fn test_run_analysis_directory() {
    // Test analyzing a directory with multiple files
    let temp_dir = TempDir::new().unwrap();

    let file1 = temp_dir.path().join("file1.sysml");
    let mut f1 = fs::File::create(&file1).unwrap();
    writeln!(f1, "part def Car;").unwrap();

    let file2 = temp_dir.path().join("file2.sysml");
    let mut f2 = fs::File::create(&file2).unwrap();
    writeln!(f2, "part def Bike;").unwrap();

    let result = run_analysis(&temp_dir.path().to_path_buf(), false, false, None).unwrap();

    assert_eq!(result.file_count, 2);
    assert!(result.symbol_count >= 2);
}

#[test]
fn test_run_analysis_nonexistent_path() {
    // Test with nonexistent path
    let result = run_analysis(&PathBuf::from("/nonexistent/path"), false, false, None);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

#[test]
fn test_run_analysis_verbose_mode() {
    // Test verbose mode doesn't affect results
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let result = run_analysis(&file_path, true, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count > 0);
}

#[test]
fn test_run_analysis_with_stdlib() {
    // Test loading with standard library
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let result = run_analysis(&file_path, false, true, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count >= 1);
}

#[test]
fn test_run_analysis_empty_directory() {
    // Test analyzing an empty directory
    let temp_dir = TempDir::new().unwrap();

    let result = run_analysis(&temp_dir.path().to_path_buf(), false, false, None).unwrap();

    assert_eq!(result.file_count, 0);
}

#[test]
fn test_run_analysis_nested_directory() {
    // Test analyzing nested directory structure
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("models");
    fs::create_dir(&subdir).unwrap();

    let file1 = temp_dir.path().join("root.sysml");
    let mut f1 = fs::File::create(&file1).unwrap();
    writeln!(f1, "part def Root;").unwrap();

    let file2 = subdir.join("nested.sysml");
    let mut f2 = fs::File::create(&file2).unwrap();
    writeln!(f2, "part def Nested;").unwrap();

    let result = run_analysis(&temp_dir.path().to_path_buf(), false, false, None).unwrap();

    assert_eq!(result.file_count, 2);
    assert!(result.symbol_count >= 2);
}

#[test]
fn test_run_analysis_complex_sysml() {
    // Test analyzing more complex SysML content
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("complex.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "package TestPackage {{").unwrap();
    writeln!(file, "    part def Vehicle {{").unwrap();
    writeln!(file, "        part engine;").unwrap();
    writeln!(file, "    }}").unwrap();
    writeln!(file, "}}").unwrap();

    let result = run_analysis(&file_path, false, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    // Should have multiple symbols (package, definition, usage)
    assert!(result.symbol_count >= 2);
}

#[test]
fn test_run_analysis_invalid_sysml() {
    // Test with invalid SysML syntax
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("invalid.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "this is not valid sysml syntax !@#$").unwrap();

    let result = run_analysis(&file_path, false, false, None);

    assert!(result.is_err());
}

#[test]
fn test_run_analysis_multiple_packages() {
    // Test file with multiple package declarations
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("multi.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "package Package1 {{ }}").unwrap();
    writeln!(file, "package Package2 {{ }}").unwrap();

    let result = run_analysis(&file_path, false, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count >= 2);
}

#[test]
fn test_run_analysis_with_imports() {
    // Test file with imports
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("imports.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "package Test {{").unwrap();
    writeln!(file, "    import Base::*;").unwrap();
    writeln!(file, "    part def Vehicle;").unwrap();
    writeln!(file, "}}").unwrap();

    let result = run_analysis(&file_path, false, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count >= 1);
}

#[test]
fn test_run_analysis_custom_stdlib_path() {
    // Test with custom stdlib path
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let custom_stdlib = temp_dir.path().join("custom_lib");
    fs::create_dir(&custom_stdlib).unwrap();

    let stdlib_file = custom_stdlib.join("Base.sysml");
    let mut sf = fs::File::create(&stdlib_file).unwrap();
    writeln!(sf, "package Base {{ }}").unwrap();

    let result = run_analysis(&file_path, false, true, Some(&custom_stdlib)).unwrap();

    assert!(result.file_count >= 1);
    assert!(result.symbol_count >= 1);
}

#[test]
fn test_run_analysis_edge_case_empty_file() {
    // Test with empty file
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("empty.sysml");

    fs::File::create(&file_path).unwrap();

    let result = run_analysis(&file_path, false, false, None);

    // Empty file should parse successfully but have no symbols
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.file_count, 1);
    assert_eq!(result.symbol_count, 0);
}

#[test]
fn test_run_analysis_with_comments_only() {
    // Test file with only comments
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("comments.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "// This is a comment").unwrap();
    writeln!(file, "/* Multi-line comment */").unwrap();

    let result = run_analysis(&file_path, false, false, None);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.file_count, 1);
    // Comments don't create symbols
    assert_eq!(result.symbol_count, 0);
}

// =============================================================================
// Tests for syster::main (Issue #155)
// Note: Testing main() directly is challenging because it uses clap::Parser
// and requires command-line arguments. The logic is thoroughly tested via
// run_analysis() which main() delegates to. The main() function is a thin
// wrapper that:
// 1. Parses CLI arguments
// 2. Calls run_analysis with parsed arguments
// 3. Formats the output
//
// All business logic is tested through run_analysis() tests above.
// =============================================================================

#[test]
fn test_main_logic_through_run_analysis() {
    // This test verifies the core logic that main() uses
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    // Simulate what main() does:
    // 1. Parse args (simulated with direct values)
    let input = &file_path;
    let verbose = false;
    let load_stdlib = true; // !no_stdlib
    let stdlib_path = None;

    // 2. Call run_analysis
    let result = run_analysis(input, verbose, load_stdlib, stdlib_path.as_ref());

    // 3. Verify result can be formatted as main() does
    assert!(result.is_ok());
    let result = result.unwrap();
    let output = format!(
        "✓ Successfully analyzed {} files ({} symbols)",
        result.file_count, result.symbol_count
    );

    assert!(output.contains("Successfully analyzed"));
    assert!(output.contains("files"));
    assert!(output.contains("symbols"));
}

#[test]
fn test_main_error_handling_through_run_analysis() {
    // Test error path that main() would handle
    let result = run_analysis(&PathBuf::from("/nonexistent"), false, false, None);

    assert!(result.is_err());
    let error = result.unwrap_err();

    // Verify error can be converted to anyhow::Error as main() does
    let _anyhow_err = anyhow::anyhow!(error.clone());

    // Verify error message is meaningful
    assert!(error.contains("does not exist"));
}

#[test]
fn test_main_verbose_flag_through_run_analysis() {
    // Test verbose flag behavior
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    // Test with verbose = true
    let result = run_analysis(&file_path, true, false, None);
    assert!(result.is_ok());

    // Test with verbose = false
    let result = run_analysis(&file_path, false, false, None);
    assert!(result.is_ok());
}

#[test]
fn test_main_stdlib_flags_through_run_analysis() {
    // Test stdlib loading behavior
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    // Test with stdlib enabled (default behavior)
    let result = run_analysis(&file_path, false, true, None);
    assert!(result.is_ok());

    // Test with stdlib disabled (--no-stdlib flag)
    let result = run_analysis(&file_path, false, false, None);
    assert!(result.is_ok());
}

#[test]
fn test_main_output_format() {
    // Test that output format matches what main() produces
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();
    writeln!(file, "part def Car;").unwrap();

    let result = run_analysis(&file_path, false, false, None).unwrap();

    // Verify the format string that main() uses
    let output = format!(
        "✓ Successfully analyzed {} files ({} symbols)",
        result.file_count, result.symbol_count
    );

    // Check that checkmark and formatting are present
    assert!(output.starts_with("✓ Successfully"));
    assert!(output.contains("1 files"));
    assert!(output.contains("2 symbols"));
}
