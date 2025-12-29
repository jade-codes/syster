#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::*;
use crate::parser::SysMLParser;
use crate::parser::sysml::Rule;
use crate::syntax::SyntaxFile;
use crate::syntax::sysml::ast::SysMLFile;
use from_pest::FromPest;
use pest::Parser;

// Tests for WorkspaceFile::path()

#[test]
fn test_workspace_file_path_returns_correct_path() {
    // Create a workspace and add a file to test path() through public API
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("test/path/vehicle.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file));

    let workspace_file = workspace.get_file(&path).unwrap();
    assert_eq!(workspace_file.path(), &path);
}

#[test]
fn test_workspace_file_path_with_relative_path() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Car;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("car.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file));

    let workspace_file = workspace.get_file(&path).unwrap();
    assert_eq!(workspace_file.path(), &path);
}

#[test]
fn test_workspace_file_path_with_nested_directory() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Truck;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("models/vehicles/truck.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file));

    let workspace_file = workspace.get_file(&path).unwrap();
    assert_eq!(workspace_file.path(), &path);
    assert_eq!(
        workspace_file.path().to_str().unwrap(),
        "models/vehicles/truck.sysml"
    );
}

#[test]
fn test_workspace_file_path_immutable() {
    // Verify that path remains consistent after updates
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("constant.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    let original_path = workspace.get_file(&path).unwrap().path().clone();

    // Update the file content
    let source2 = "part def V2;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    // Path should remain unchanged
    assert_eq!(workspace.get_file(&path).unwrap().path(), &original_path);
    assert_eq!(workspace.get_file(&path).unwrap().path(), &path);
}

// Tests for WorkspaceFile::version()

#[test]
fn test_workspace_file_version_initial_value() {
    // New files should have version 0
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file));

    let workspace_file = workspace.get_file(&path).unwrap();
    assert_eq!(workspace_file.version(), 0);
}

#[test]
fn test_workspace_file_version_increments_on_update() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    assert_eq!(workspace.get_file(&path).unwrap().version(), 0);

    // First update
    let source2 = "part def V2;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    assert_eq!(workspace.get_file(&path).unwrap().version(), 1);
}

#[test]
fn test_workspace_file_version_multiple_updates() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Update multiple times and verify version increments correctly
    for expected_version in 1..=10 {
        let source = format!("part def V{expected_version};");
        let mut pairs = SysMLParser::parse(Rule::model, &source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();
        workspace.update_file(&path, SyntaxFile::SysML(file));

        assert_eq!(
            workspace.get_file(&path).unwrap().version(),
            expected_version
        );
    }
}

#[test]
fn test_workspace_file_version_independent_across_files() {
    // Verify that different files maintain independent version numbers
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path1 = PathBuf::from("file1.sysml");
    let path2 = PathBuf::from("file2.sysml");

    // Add first file
    let source1 = "part def F1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();
    workspace.add_file(path1.clone(), SyntaxFile::SysML(file1));

    // Add second file
    let source2 = "part def F2;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.add_file(path2.clone(), SyntaxFile::SysML(file2));

    // Both should start at version 0
    assert_eq!(workspace.get_file(&path1).unwrap().version(), 0);
    assert_eq!(workspace.get_file(&path2).unwrap().version(), 0);

    // Update first file twice
    for i in 1..=2 {
        let source = format!("part def F1_V{i};");
        let mut pairs = SysMLParser::parse(Rule::model, &source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();
        workspace.update_file(&path1, SyntaxFile::SysML(file));
    }

    // Update second file once
    let source = "part def F2_V1;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.update_file(&path2, SyntaxFile::SysML(file));

    // Verify independent version tracking
    assert_eq!(workspace.get_file(&path1).unwrap().version(), 2);
    assert_eq!(workspace.get_file(&path2).unwrap().version(), 1);
}

#[test]
fn test_workspace_file_version_large_number() {
    // Verify version can handle large update counts
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Perform many updates
    let update_count = 100u32;
    for _ in 0..update_count {
        let source = "part def Updated;";
        let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();
        workspace.update_file(&path, SyntaxFile::SysML(file));
    }

    assert_eq!(workspace.get_file(&path).unwrap().version(), update_count);
}

// Tests for WorkspaceFile::update_content()

#[test]
fn test_workspace_file_update_content_changes_content() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Verify initial content exists
    let initial_file = workspace.get_file(&path).unwrap();
    assert!(initial_file.content().as_sysml().is_some());

    // Update content
    let source2 = "part def Car;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    let updated = workspace.update_file(&path, SyntaxFile::SysML(file2));

    assert!(updated);
    assert!(
        workspace
            .get_file(&path)
            .unwrap()
            .content()
            .as_sysml()
            .is_some()
    );
}

#[test]
fn test_workspace_file_update_content_increments_version() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    let version_before = workspace.get_file(&path).unwrap().version();

    // Update content
    let source2 = "part def V2;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    let version_after = workspace.get_file(&path).unwrap().version();
    assert_eq!(version_after, version_before + 1);
}

#[test]
fn test_workspace_file_update_content_resets_populated_flag() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Populate the file
    workspace.populate_file(&path).unwrap();
    assert!(workspace.get_file(&path).unwrap().is_populated());

    // Update content
    let source2 = "part def Car;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    // Populated flag should be reset to false
    assert!(!workspace.get_file(&path).unwrap().is_populated());
}

#[test]
fn test_workspace_file_update_content_with_empty_file() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Update with empty file
    let empty_source = "";
    let mut pairs_empty = SysMLParser::parse(Rule::model, empty_source).unwrap();
    let empty_file = SysMLFile::from_pest(&mut pairs_empty).unwrap();
    let updated = workspace.update_file(&path, SyntaxFile::SysML(empty_file));

    assert!(updated);
    assert_eq!(workspace.get_file(&path).unwrap().version(), 1);
}

#[test]
fn test_workspace_file_update_content_preserves_path() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("important/path/file.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    let path_before = workspace.get_file(&path).unwrap().path().clone();

    // Update content multiple times
    for i in 2..=5 {
        let source = format!("part def V{i};");
        let mut pairs = SysMLParser::parse(Rule::model, &source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();
        workspace.update_file(&path, SyntaxFile::SysML(file));
    }

    // Path should remain unchanged
    let path_after = workspace.get_file(&path).unwrap().path().clone();
    assert_eq!(path_after, path_before);
    assert_eq!(path_after, path);
}

#[test]
fn test_workspace_file_update_content_with_syntax_file_sysml() {
    // Test update_content specifically with SyntaxFile::SysML variant
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def Original;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    workspace.populate_file(&path).unwrap();
    assert_eq!(workspace.get_file(&path).unwrap().version(), 0);
    assert!(workspace.get_file(&path).unwrap().is_populated());

    // Update with new SysML content
    let source2 = "part def Updated;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    // Verify all expected changes
    assert_eq!(workspace.get_file(&path).unwrap().version(), 1);
    assert!(!workspace.get_file(&path).unwrap().is_populated());
    assert!(
        workspace
            .get_file(&path)
            .unwrap()
            .content()
            .as_sysml()
            .is_some()
    );
}

#[test]
fn test_workspace_file_update_content_consecutive_updates() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Perform consecutive updates without populating in between
    let source2 = "part def V2;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    let source3 = "part def V3;";
    let mut pairs3 = SysMLParser::parse(Rule::model, source3).unwrap();
    let file3 = SysMLFile::from_pest(&mut pairs3).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file3));

    // Version should be 2, populated should still be false
    assert_eq!(workspace.get_file(&path).unwrap().version(), 2);
    assert!(!workspace.get_file(&path).unwrap().is_populated());
}

#[test]
fn test_workspace_file_update_content_after_repopulate() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Initial populate
    workspace.populate_file(&path).unwrap();
    assert!(workspace.get_file(&path).unwrap().is_populated());
    assert_eq!(workspace.get_file(&path).unwrap().version(), 0);

    // Update
    let source2 = "part def V2;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    assert!(!workspace.get_file(&path).unwrap().is_populated());
    assert_eq!(workspace.get_file(&path).unwrap().version(), 1);

    // Re-populate
    workspace.populate_file(&path).unwrap();
    assert!(workspace.get_file(&path).unwrap().is_populated());
    assert_eq!(workspace.get_file(&path).unwrap().version(), 1);

    // Another update
    let source3 = "part def V3;";
    let mut pairs3 = SysMLParser::parse(Rule::model, source3).unwrap();
    let file3 = SysMLFile::from_pest(&mut pairs3).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file3));

    assert!(!workspace.get_file(&path).unwrap().is_populated());
    assert_eq!(workspace.get_file(&path).unwrap().version(), 2);
}

#[test]
fn test_workspace_file_update_content_with_complex_content() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let simple_source = "part def Simple;";
    let mut pairs1 = SysMLParser::parse(Rule::model, simple_source).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file1));

    // Update with more complex content
    let complex_source = r#"
        package Vehicles {
            part def Vehicle {
                attribute mass : Real;
            }
            part def Car :> Vehicle;
        }
    "#;
    let mut pairs2 = SysMLParser::parse(Rule::model, complex_source).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, SyntaxFile::SysML(file2));

    assert_eq!(workspace.get_file(&path).unwrap().version(), 1);
    assert!(!workspace.get_file(&path).unwrap().is_populated());
}
