use std::fs;
use std::io::Write;
use std::path::PathBuf;
use syster_cli::run_analysis;
use tempfile::TempDir;

#[test]
fn test_analyze_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let result = run_analysis(&file_path, false, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count > 0);
}

#[test]
fn test_analyze_directory() {
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
fn test_nonexistent_path() {
    let result = run_analysis(&PathBuf::from("/nonexistent/path"), false, false, None);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

#[test]
fn test_invalid_sysml() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("invalid.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "this is not valid sysml !@#$%").unwrap();

    let result = run_analysis(&file_path, false, false, None);
    assert!(result.is_err());
}

#[test]
fn test_with_stdlib() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let result_with_stdlib = run_analysis(&file_path, false, true, None).unwrap();
    let result_without_stdlib = run_analysis(&file_path, false, false, None).unwrap();

    assert_eq!(result_with_stdlib.file_count, 1);
    assert_eq!(result_without_stdlib.file_count, 1);
    // Both should have at least 1 symbol from our file
    assert!(result_with_stdlib.symbol_count >= 1);
    assert!(result_without_stdlib.symbol_count >= 1);
}

#[test]
fn test_without_stdlib() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let result = run_analysis(&file_path, false, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert_eq!(result.symbol_count, 1);
}

#[test]
#[ignore = "KerML is now supported - test is outdated"]
fn test_kerml_file() {
    // KerML is not yet supported - should return error
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.kerml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "classifier Vehicle;").unwrap();

    let result = run_analysis(&file_path, false, false, None);

    // Should fail with unsupported language error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("KerML") || err.contains("not yet supported"));
}

#[test]
#[ignore = "KerML is now supported - test is outdated"]
fn test_mixed_directory() {
    // KerML is not yet supported - directory with mixed files should fail
    let temp_dir = TempDir::new().unwrap();

    let sysml_file = temp_dir.path().join("file1.sysml");
    let mut f1 = fs::File::create(&sysml_file).unwrap();
    writeln!(f1, "part def Car;").unwrap();

    let kerml_file = temp_dir.path().join("file2.kerml");
    let mut f2 = fs::File::create(&kerml_file).unwrap();
    writeln!(f2, "classifier Vehicle;").unwrap();

    let result = run_analysis(&temp_dir.path().to_path_buf(), false, false, None);

    // Should fail when trying to process KerML file
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("KerML") || err.contains("not yet supported"));
}

#[test]
fn test_verbose_mode() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    // Verbose mode should still succeed, just with more output
    let result = run_analysis(&file_path, true, false, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count > 0);
}

#[test]
fn test_empty_directory() {
    let temp_dir = TempDir::new().unwrap();

    let result = run_analysis(&temp_dir.path().to_path_buf(), false, false, None).unwrap();

    assert_eq!(result.file_count, 0);
}

#[test]
fn test_nested_directory_structure() {
    let temp_dir = TempDir::new().unwrap();

    // Create nested directory structure mimicking a workspace
    let subdir1 = temp_dir.path().join("models");
    let subdir2 = temp_dir.path().join("models/vehicles");
    fs::create_dir_all(&subdir2).unwrap();

    // Create files in different directories
    let file1 = temp_dir.path().join("root.sysml");
    let mut f1 = fs::File::create(&file1).unwrap();
    writeln!(f1, "package RootPackage;").unwrap();

    let file2 = subdir1.join("model.sysml");
    let mut f2 = fs::File::create(&file2).unwrap();
    writeln!(f2, "part def Component;").unwrap();

    let file3 = subdir2.join("vehicle.sysml");
    let mut f3 = fs::File::create(&file3).unwrap();
    writeln!(f3, "part def Car;").unwrap();

    let result = run_analysis(&temp_dir.path().to_path_buf(), false, false, None).unwrap();

    // Should load all 3 files recursively
    assert_eq!(result.file_count, 3);
    assert!(result.symbol_count >= 3);
}

#[test]
#[ignore = "KerML is now supported - test is outdated"]
fn test_workspace_with_mixed_extensions() {
    // KerML is not yet supported - mixed workspace should fail
    let temp_dir = TempDir::new().unwrap();

    // Create files with different extensions
    let sysml = temp_dir.path().join("file.sysml");
    let mut f1 = fs::File::create(&sysml).unwrap();
    writeln!(f1, "part def Vehicle;").unwrap();

    let kerml = temp_dir.path().join("base.kerml");
    let mut f2 = fs::File::create(&kerml).unwrap();
    writeln!(f2, "classifier Base;").unwrap();

    // Add a non-model file that should be ignored
    let txt = temp_dir.path().join("readme.txt");
    let mut f3 = fs::File::create(&txt).unwrap();
    writeln!(f3, "This is not a model file").unwrap();

    let result = run_analysis(&temp_dir.path().to_path_buf(), false, false, None);

    // Should fail when trying to process KerML file
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("KerML") || err.contains("not yet supported"));
}

#[test]
fn test_stdlib_path_resolution() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    let result = run_analysis(&file_path, false, true, None).unwrap();

    assert_eq!(result.file_count, 1);
    assert!(result.symbol_count >= 1);
}

#[test]
fn test_custom_stdlib_path() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.sysml");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "part def Vehicle;").unwrap();

    // Create a custom stdlib directory
    let custom_stdlib = temp_dir.path().join("custom_stdlib");
    fs::create_dir_all(&custom_stdlib).unwrap();

    let stdlib_file = custom_stdlib.join("Base.sysml");
    let mut sf = fs::File::create(&stdlib_file).unwrap();
    writeln!(sf, "package Base {{ }}").unwrap();

    let result = run_analysis(&file_path, false, true, Some(&custom_stdlib)).unwrap();

    // File count includes both the input file AND custom stdlib files loaded
    assert!(result.file_count >= 1);
    assert!(result.symbol_count >= 1);
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
