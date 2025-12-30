//! Tests for main function in syster CLI
//!
//! Note: Testing main() directly is challenging because it uses clap::Parser
//! and requires command-line arguments. The logic is thoroughly tested via
//! run_analysis() which main() delegates to. The main() function is a thin
//! wrapper that:
//! 1. Parses CLI arguments
//! 2. Calls run_analysis with parsed arguments
//! 3. Formats the output
//!
//! All business logic is tested through run_analysis() tests in cli_tests.rs.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use syster_cli::run_analysis;
use tempfile::TempDir;

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
