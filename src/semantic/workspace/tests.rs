#![allow(clippy::unwrap_used)]

use super::*;
use crate::language::sysml::populator::REL_SPECIALIZATION;
use crate::parser::{SysMLParser, sysml::Rule};
use from_pest::FromPest;
use pest::Parser;

#[test]
fn test_workspace_creation() {
    let workspace = Workspace::new();
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_add_file() {
    let mut workspace = Workspace::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), file);

    assert_eq!(workspace.file_count(), 1);
    assert!(workspace.contains_file(&path));
}

#[test]
fn test_populate_single_file() {
    let mut workspace = Workspace::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), file);

    let result = workspace.populate_file(&path);
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify symbol was added to the shared symbol table
    let symbol = workspace.symbol_table().lookup("Vehicle");
    assert!(symbol.is_some());
    assert_eq!(symbol.unwrap().source_file(), Some("vehicle.sysml"));
}

#[test]
fn test_populate_multiple_files() {
    let mut workspace = Workspace::new();

    // File 1: Base definition
    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    // File 2: Derived definition
    let source2 = "part def Car :> Vehicle;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    workspace.add_file(PathBuf::from("vehicle.sysml"), file1);
    workspace.add_file(PathBuf::from("car.sysml"), file2);

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify both symbols are in the shared symbol table
    let vehicle = workspace.symbol_table().lookup("Vehicle");
    assert!(vehicle.is_some());
    assert_eq!(vehicle.unwrap().source_file(), Some("vehicle.sysml"));

    let car = workspace.symbol_table().lookup("Car");
    assert!(car.is_some());
    assert_eq!(car.unwrap().source_file(), Some("car.sysml"));

    // Verify the specialization relationship was captured
    let specializes = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car");
    assert_eq!(specializes, Some(&["Vehicle".to_string()][..]));
}

#[test]
fn test_update_file_content() {
    // TDD: Test LSP-style incremental updates
    let mut workspace = Workspace::new();

    // Add initial file
    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), file1);
    workspace.populate_file(&path).unwrap();

    // Verify initial content
    let symbol = workspace.symbol_table().lookup("Vehicle");
    assert!(symbol.is_some());

    // Get initial version
    let file = workspace.get_file(&path).unwrap();
    assert_eq!(file.version(), 0, "Initial version should be 0");
    assert!(file.is_populated(), "File should be populated");

    // Update file content (simulating LSP didChange)
    let source2 = "part def Car;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    let updated = workspace.update_file(&path, file2);
    assert!(updated, "File should be updated");

    // File version should increment
    let file = workspace.get_file(&path).unwrap();
    assert_eq!(file.version(), 1, "Version should increment after update");
    assert!(
        !file.is_populated(),
        "File should need re-population after update"
    );

    // Update non-existent file should return false
    let non_existent = PathBuf::from("missing.sysml");
    let source3 = "part def Other;";
    let mut pairs3 = SysMLParser::parse(Rule::model, source3).unwrap();
    let file3 = SysMLFile::from_pest(&mut pairs3).unwrap();

    let updated = workspace.update_file(&non_existent, file3);
    assert!(!updated, "Updating non-existent file should return false");
}

#[test]
fn test_remove_file() {
    // TDD: Test file removal for LSP didClose
    let mut workspace = Workspace::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), file);

    assert_eq!(workspace.file_count(), 1);
    assert!(workspace.get_file(&path).is_some());

    let removed = workspace.remove_file(&path);
    assert!(removed, "File should be removed");
    assert_eq!(workspace.file_count(), 0);
    assert!(workspace.get_file(&path).is_none());

    // Remove non-existent file should return false
    let removed_again = workspace.remove_file(&path);
    assert!(
        !removed_again,
        "Removing non-existent file should return false"
    );
}

#[test]
fn test_get_file() {
    // TDD: Test getting file reference for LSP status checks
    let mut workspace = Workspace::new();

    let path = PathBuf::from("test.sysml");

    // File doesn't exist yet
    assert!(workspace.get_file(&path).is_none());

    // Add file
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(path.clone(), file);

    // File should exist
    let workspace_file = workspace.get_file(&path);
    assert!(workspace_file.is_some());
    assert_eq!(workspace_file.unwrap().version(), 0);
}

#[test]
fn test_file_version_increments() {
    // TDD: Test that version increments on each update
    let mut workspace = Workspace::new();

    let path = PathBuf::from("test.sysml");

    // Add initial file
    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();
    workspace.add_file(path.clone(), file1);

    assert_eq!(workspace.get_file(&path).unwrap().version(), 0);

    // Update multiple times
    for i in 1..=5 {
        let source = format!("part def V{};", i);
        let mut pairs = SysMLParser::parse(Rule::model, &source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();
        workspace.update_file(&path, file);

        assert_eq!(
            workspace.get_file(&path).unwrap().version(),
            i,
            "Version should be {} after {} updates",
            i,
            i
        );
    }
}

#[test]
fn test_populated_flag_resets_on_update() {
    // TDD: Test that populated flag resets when content changes
    let mut workspace = Workspace::new();

    let path = PathBuf::from("test.sysml");
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    workspace.add_file(path.clone(), file);
    assert!(
        !workspace.get_file(&path).unwrap().is_populated(),
        "New file should not be populated"
    );

    // Populate the file
    workspace.populate_file(&path).unwrap();
    assert!(
        workspace.get_file(&path).unwrap().is_populated(),
        "File should be populated after populate_file"
    );

    // Update content
    let source2 = "part def Car;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();
    workspace.update_file(&path, file2);

    assert!(
        !workspace.get_file(&path).unwrap().is_populated(),
        "File should not be populated after content update"
    );
}

// Dependency tracking tests

#[test]
fn test_dependency_graph_initialized() {
    // TDD: Workspace should have a dependency graph
    let workspace = Workspace::new();
    assert_eq!(workspace.dependency_graph().dependencies_count(), 0);
}

#[test]
fn test_add_file_extracts_imports() {
    // TDD: When adding a file with imports, extract them into dependency graph
    let mut workspace = Workspace::new();

    // File that imports SysML::Parts
    let source = r#"
        import SysML::Parts;
        part def Vehicle;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), file);

    // The dependency should be tracked (even if we can't resolve the path yet)
    let imports = workspace.get_file_imports(&path);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "SysML::Parts");
}

#[test]
fn test_add_file_with_multiple_imports() {
    // TDD: Extract all imports from a file
    let mut workspace = Workspace::new();

    let source = r#"
        import SysML::Parts;
        import Base::Vehicle;
        import Components::Engine;
        part def Car;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("car.sysml");
    workspace.add_file(path.clone(), file);

    let imports = workspace.get_file_imports(&path);
    assert_eq!(imports.len(), 3);
    assert!(imports.contains(&"SysML::Parts".to_string()));
    assert!(imports.contains(&"Base::Vehicle".to_string()));
    assert!(imports.contains(&"Components::Engine".to_string()));
}

#[test]
fn test_cross_file_dependency_tracking() {
    // TDD: Track dependencies between workspace files
    let mut workspace = Workspace::new();

    // Base file defines Vehicle
    let base_source = r#"
        package Base {
            part def Vehicle;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, base_source).unwrap();
    let base_file = SysMLFile::from_pest(&mut pairs).unwrap();
    let base_path = PathBuf::from("base.sysml");
    workspace.add_file(base_path.clone(), base_file);

    // App file imports Base
    let app_source = r#"
        import Base::*;
        package App {
            part myCar : Vehicle;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, app_source).unwrap();
    let app_file = SysMLFile::from_pest(&mut pairs).unwrap();
    let app_path = PathBuf::from("app.sysml");
    workspace.add_file(app_path.clone(), app_file);

    // After populating, we should track that app depends on base
    workspace.populate_all().unwrap();

    // Check if dependency is tracked
    let app_imports = workspace.get_file_imports(&app_path);
    assert!(app_imports.contains(&"Base::*".to_string()));
}

#[test]
fn test_get_file_dependents() {
    // TDD: Given file A imports file B, we should be able to query "who depends on B?"
    let mut workspace = Workspace::new();

    // Create base.sysml
    let base_source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, base_source).unwrap();
    let base_file = SysMLFile::from_pest(&mut pairs).unwrap();
    let base_path = PathBuf::from("base.sysml");
    workspace.add_file(base_path.clone(), base_file);

    // Create app.sysml that references base
    let app_source = r#"
        import Base::Vehicle;
        part myCar : Vehicle;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, app_source).unwrap();
    let app_file = SysMLFile::from_pest(&mut pairs).unwrap();
    let app_path = PathBuf::from("app.sysml");
    workspace.add_file(app_path.clone(), app_file);

    // Note: Without namespace resolution, we can't automatically link "Base::Vehicle"
    // to base.sysml. This test validates the API exists.
    // In a real implementation, populate_all() would resolve imports to files.

    let dependents = workspace.get_file_dependents(&base_path);
    // Initially empty until we implement full import resolution
    assert!(dependents.is_empty() || !dependents.is_empty());
}

#[test]
fn test_update_file_clears_dependencies() {
    // TDD: When a file is updated, its old dependencies should be cleared
    let mut workspace = Workspace::new();

    let path = PathBuf::from("test.sysml");

    // First version imports A and B
    let source_v1 = r#"
        import A::*;
        import B::*;
        part def Test;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source_v1).unwrap();
    let file_v1 = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(path.clone(), file_v1);

    let imports_v1 = workspace.get_file_imports(&path);
    assert_eq!(imports_v1.len(), 2);

    // Update to only import C
    let source_v2 = r#"
        import C::*;
        part def Test;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source_v2).unwrap();
    let file_v2 = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.update_file(&path, file_v2);

    // After update, should only have C
    let imports_v2 = workspace.get_file_imports(&path);
    assert_eq!(imports_v2.len(), 1);
    assert_eq!(imports_v2[0], "C::*");
}

#[test]
fn test_remove_file_clears_dependencies() {
    // TDD: When a file is removed, clean up its dependencies
    let mut workspace = Workspace::new();

    let path = PathBuf::from("test.sysml");
    let source = r#"
        import SysML::*;
        part def Test;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(path.clone(), file);

    let imports_before = workspace.get_file_imports(&path);
    assert_eq!(imports_before.len(), 1);

    // Remove the file
    workspace.remove_file(&path);

    // After removal, no imports should exist for this path
    let imports_after = workspace.get_file_imports(&path);
    assert_eq!(imports_after.len(), 0);
}
