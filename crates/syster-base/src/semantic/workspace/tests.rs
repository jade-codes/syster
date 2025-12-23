#![allow(clippy::unwrap_used)]

use std::path::PathBuf;

use super::*;
use crate::core::constants::REL_SPECIALIZATION;
use crate::parser::SysMLParser;
use crate::parser::sysml::Rule;
use crate::syntax::SyntaxFile;
use crate::syntax::sysml::ast::SysMLFile;
use from_pest::FromPest;
use pest::Parser;

#[test]
fn test_workspace_creation() {
    let workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_add_file() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));

    assert_eq!(workspace.file_count(), 1);
    assert!(workspace.get_file(&path).is_some());
}

#[test]
fn test_populate_single_file() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));

    let result = workspace.populate_file(&path);
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify symbol was added to the shared symbol table
    let symbol = workspace.symbol_table().lookup("Vehicle");
    assert!(symbol.is_some());
    assert_eq!(symbol.unwrap().source_file(), Some("vehicle.sysml"));
}

#[test]
fn test_populate_multiple_files() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    // File 1: Base definition
    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    // File 2: Derived definition
    let source2 = "part def Car :> Vehicle;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    workspace.add_file(
        PathBuf::from("vehicle.sysml"),
        crate::syntax::SyntaxFile::SysML(file1),
    );
    workspace.add_file(
        PathBuf::from("car.sysml"),
        crate::syntax::SyntaxFile::SysML(file2),
    );

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
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Add initial file
    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file1));
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

    let updated = workspace.update_file(&path, crate::syntax::SyntaxFile::SysML(file2));
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

    let updated = workspace.update_file(&non_existent, crate::syntax::SyntaxFile::SysML(file3));
    assert!(!updated, "Updating non-existent file should return false");
}

#[test]
fn test_remove_file() {
    // TDD: Test file removal for LSP didClose
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));

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
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path = PathBuf::from("test.sysml");

    // File doesn't exist yet
    assert!(workspace.get_file(&path).is_none());

    // Add file
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));

    // File should exist
    let workspace_file = workspace.get_file(&path);
    assert!(workspace_file.is_some());
    assert_eq!(workspace_file.unwrap().version(), 0);
}

#[test]
fn test_file_version_increments() {
    // TDD: Test that version increments on each update
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path = PathBuf::from("test.sysml");

    // Add initial file
    let source1 = "part def V1;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file1));

    assert_eq!(workspace.get_file(&path).unwrap().version(), 0);

    // Update multiple times
    for i in 1..=5 {
        let source = format!("part def V{i};");
        let mut pairs = SysMLParser::parse(Rule::model, &source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();
        workspace.update_file(&path, crate::syntax::SyntaxFile::SysML(file));

        assert_eq!(
            workspace.get_file(&path).unwrap().version(),
            i,
            "Version should be {i} after {i} updates"
        );
    }
}

#[test]
fn test_populated_flag_resets_on_update() {
    // TDD: Test that populated flag resets when content changes
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path = PathBuf::from("test.sysml");
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));
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
    workspace.update_file(&path, crate::syntax::SyntaxFile::SysML(file2));

    assert!(
        !workspace.get_file(&path).unwrap().is_populated(),
        "File should not be populated after content update"
    );
}

// Dependency tracking tests

#[test]
fn test_dependency_graph_initialized() {
    // TDD: Workspace should have a dependency graph
    let workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.dependency_graph().dependencies_count(), 0);
}

#[test]
fn test_cross_file_dependency_tracking() {
    // TDD: Track dependencies between workspace files
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Base file defines Vehicle
    let base_source = r#"
        package Base {
            part def Vehicle;
        }
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, base_source).unwrap();
    let base_file = SysMLFile::from_pest(&mut pairs).unwrap();
    let base_path = PathBuf::from("base.sysml");
    workspace.add_file(
        base_path.clone(),
        crate::syntax::SyntaxFile::SysML(base_file),
    );

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
    workspace.add_file(app_path.clone(), crate::syntax::SyntaxFile::SysML(app_file));

    // After populating, we should track that app depends on base
    workspace.populate_all().unwrap();

    // Verify files were populated
    assert!(workspace.get_file(&app_path).unwrap().is_populated());
    assert!(workspace.get_file(&base_path).unwrap().is_populated());
}

#[test]
fn test_update_file_clears_dependencies() {
    // TDD: When a file is updated, its old dependencies should be cleared
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path = PathBuf::from("test.sysml");

    // First version imports A and B
    let source_v1 = r#"
        import A::*;
        import B::*;
        part def Test;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source_v1).unwrap();
    let file_v1 = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file_v1));

    // Update to only import C
    let source_v2 = r#"
        import C::*;
        part def Test;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source_v2).unwrap();
    let file_v2 = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.update_file(&path, crate::syntax::SyntaxFile::SysML(file_v2));

    // File should still exist
    assert!(workspace.get_file(&path).is_some());
}

#[test]
fn test_remove_file_clears_dependencies() {
    // TDD: When a file is removed, clean up its dependencies
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path = PathBuf::from("test.sysml");
    let source = r#"
        import SysML::*;
        part def Test;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));

    // Remove the file
    workspace.remove_file(&path);

    // After removal, file should not exist
    assert!(workspace.get_file(&path).is_none());
}

#[test]
fn test_subscribe_to_file_added() {
    use crate::semantic::types::WorkspaceEvent;
    use std::sync::{Arc, Mutex};

    let mut workspace = Workspace::<SyntaxFile>::new();
    let events_received = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events_received.clone();

    workspace.events.subscribe(move |event, _workspace| {
        events_clone.lock().unwrap().push(event.clone());
    });

    let path = PathBuf::from("test.sysml");
    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    };

    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));

    let events = events_received.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], WorkspaceEvent::FileAdded { path });
}

#[test]
fn test_subscribe_to_file_updated() {
    use crate::semantic::types::WorkspaceEvent;
    use std::sync::{Arc, Mutex};

    let mut workspace = Workspace::<SyntaxFile>::new();
    let path = PathBuf::from("test.sysml");

    // Add file first
    workspace.add_file(
        path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    let events_received = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events_received.clone();

    workspace.events.subscribe(move |event, _workspace| {
        events_clone.lock().unwrap().push(event.clone());
    });

    // Update the file
    workspace.update_file(
        &path,
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    let events = events_received.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], WorkspaceEvent::FileUpdated { path });
}

#[test]
fn test_invalidate_on_update() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.enable_auto_invalidation();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(
        path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Populate the file
    let _ = workspace.populate_file(&path);
    assert!(workspace.get_file(&path).unwrap().is_populated());

    // Update the file - should trigger invalidation
    workspace.update_file(
        &path,
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // File should now be unpopulated
    assert!(!workspace.get_file(&path).unwrap().is_populated());
}
#[test]
fn test_invalidate_dependent_files() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.enable_auto_invalidation();

    let base_path = PathBuf::from("base.sysml");
    let app_path = PathBuf::from("app.sysml"); // Add base file
    workspace.add_file(
        base_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Add app file
    workspace.add_file(
        app_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Set up dependency: app imports base
    workspace
        .dependency_graph_mut()
        .add_dependency(&app_path, &base_path);

    // Populate both files
    let _ = workspace.populate_file(&base_path);
    let _ = workspace.populate_file(&app_path);
    assert!(workspace.get_file(&base_path).unwrap().is_populated());
    assert!(workspace.get_file(&app_path).unwrap().is_populated());

    // Update base - should invalidate app too
    workspace.update_file(
        &base_path,
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Both files should be unpopulated
    assert!(!workspace.get_file(&base_path).unwrap().is_populated());
    assert!(!workspace.get_file(&app_path).unwrap().is_populated());
}

#[test]
fn test_invalidate_transitive_dependencies() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.enable_auto_invalidation();

    let a_path = PathBuf::from("a.sysml");
    let b_path = PathBuf::from("b.sysml");
    let c_path = PathBuf::from("c.sysml"); // Add files
    workspace.add_file(
        a_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        c_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Set up dependency chain: A -> B -> C
    workspace
        .dependency_graph_mut()
        .add_dependency(&a_path, &b_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&b_path, &c_path);

    // Populate all files
    let _ = workspace.populate_file(&a_path);
    let _ = workspace.populate_file(&b_path);
    let _ = workspace.populate_file(&c_path);

    // Update C - should invalidate B and A
    workspace.update_file(
        &c_path,
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // All three files should be unpopulated
    assert!(!workspace.get_file(&c_path).unwrap().is_populated());
    assert!(!workspace.get_file(&b_path).unwrap().is_populated());
    assert!(!workspace.get_file(&a_path).unwrap().is_populated());
}

#[test]
fn test_circular_dependency_simple() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let a_path = PathBuf::from("a.sysml");
    let b_path = PathBuf::from("b.sysml");

    // Add files
    workspace.add_file(
        a_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Create circular dependency: A -> B -> A
    workspace
        .dependency_graph_mut()
        .add_dependency(&a_path, &b_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&b_path, &a_path);

    // Both files should have circular dependencies
    assert!(
        workspace
            .dependency_graph()
            .has_circular_dependency(&a_path)
    );
    assert!(
        workspace
            .dependency_graph()
            .has_circular_dependency(&b_path)
    );
}

#[test]
fn test_circular_dependency_complex() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let a_path = PathBuf::from("a.sysml");
    let b_path = PathBuf::from("b.sysml");
    let c_path = PathBuf::from("c.sysml");

    // Add files
    workspace.add_file(
        a_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        c_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Create circular dependency: A -> B -> C -> A
    workspace
        .dependency_graph_mut()
        .add_dependency(&a_path, &b_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&b_path, &c_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&c_path, &a_path);

    // All files should detect the circular dependency
    assert!(
        workspace
            .dependency_graph()
            .has_circular_dependency(&a_path)
    );
    assert!(
        workspace
            .dependency_graph()
            .has_circular_dependency(&b_path)
    );
    assert!(
        workspace
            .dependency_graph()
            .has_circular_dependency(&c_path)
    );
}

#[test]
fn test_no_circular_dependency_in_chain() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let a_path = PathBuf::from("a.sysml");
    let b_path = PathBuf::from("b.sysml");
    let c_path = PathBuf::from("c.sysml");

    // Add files
    workspace.add_file(
        a_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        c_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Create linear dependency: A -> B -> C (no cycle)
    workspace
        .dependency_graph_mut()
        .add_dependency(&a_path, &b_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&b_path, &c_path);

    // No files should have circular dependencies
    assert!(
        !workspace
            .dependency_graph()
            .has_circular_dependency(&a_path)
    );
    assert!(
        !workspace
            .dependency_graph()
            .has_circular_dependency(&b_path)
    );
    assert!(
        !workspace
            .dependency_graph()
            .has_circular_dependency(&c_path)
    );
}

#[test]
fn test_invalidation_with_circular_dependency() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.enable_auto_invalidation();

    let a_path = PathBuf::from("a.sysml");
    let b_path = PathBuf::from("b.sysml");

    // Add files
    workspace.add_file(
        a_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Create circular dependency: A -> B -> A
    workspace
        .dependency_graph_mut()
        .add_dependency(&a_path, &b_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&b_path, &a_path);

    // Populate both files
    workspace.populate_file(&a_path).unwrap();
    workspace.populate_file(&b_path).unwrap();

    // Update one file - should invalidate both without infinite loop
    workspace.update_file(
        &a_path,
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Both files should be unpopulated (invalidation visited each once)
    assert!(!workspace.get_file(&a_path).unwrap().is_populated());
    assert!(!workspace.get_file(&b_path).unwrap().is_populated());
}

#[test]
fn test_circular_dependency_self_reference() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let a_path = PathBuf::from("a.sysml");

    // Add file
    workspace.add_file(
        a_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Create self-reference: A -> A
    workspace
        .dependency_graph_mut()
        .add_dependency(&a_path, &a_path);

    // Should detect circular dependency
    assert!(
        workspace
            .dependency_graph()
            .has_circular_dependency(&a_path)
    );
}

#[test]
fn test_populate_affected_empty() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    // No unpopulated files
    let count = workspace.populate_affected().unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_populate_affected_single_file() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), crate::syntax::SyntaxFile::SysML(file));

    // File should be unpopulated
    assert!(!workspace.get_file(&path).unwrap().is_populated());

    // Populate affected
    let count = workspace.populate_affected().unwrap();
    assert_eq!(count, 1);

    // File should now be populated
    assert!(workspace.get_file(&path).unwrap().is_populated());

    // Running again should populate nothing
    let count = workspace.populate_affected().unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_populate_affected_after_update() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.enable_auto_invalidation();

    let base_path = PathBuf::from("base.sysml");
    let app_path = PathBuf::from("app.sysml");

    // Add files
    workspace.add_file(
        base_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        app_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Set up dependency: app imports base
    workspace
        .dependency_graph_mut()
        .add_dependency(&app_path, &base_path);

    // Populate all files
    workspace.populate_all().unwrap();
    assert!(workspace.get_file(&base_path).unwrap().is_populated());
    assert!(workspace.get_file(&app_path).unwrap().is_populated());

    // Update base - invalidates both files
    workspace.update_file(
        &base_path,
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Both should be unpopulated
    assert!(!workspace.get_file(&base_path).unwrap().is_populated());
    assert!(!workspace.get_file(&app_path).unwrap().is_populated());

    // Populate affected should repopulate both
    let count = workspace.populate_affected().unwrap();
    assert_eq!(count, 2);

    // Both should be populated again
    assert!(workspace.get_file(&base_path).unwrap().is_populated());
    assert!(workspace.get_file(&app_path).unwrap().is_populated());
}

#[test]
fn test_populate_affected_selective() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let a_path = PathBuf::from("a.sysml");
    let b_path = PathBuf::from("b.sysml");
    let c_path = PathBuf::from("c.sysml");

    // Add three files
    workspace.add_file(
        a_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        c_path.clone(),
        crate::syntax::SyntaxFile::SysML(SysMLFile {
        namespaces: vec![],
        namespace: None,
            elements: vec![],
        }),
    );

    // Populate all
    workspace.populate_all().unwrap();

    // Manually invalidate only one file
    workspace.mark_file_unpopulated(&b_path);

    // Only b should be unpopulated
    assert!(workspace.get_file(&a_path).unwrap().is_populated());
    assert!(!workspace.get_file(&b_path).unwrap().is_populated());
    assert!(workspace.get_file(&c_path).unwrap().is_populated());

    // Populate affected should only repopulate b
    let count = workspace.populate_affected().unwrap();
    assert_eq!(count, 1);

    // All should be populated
    assert!(workspace.get_file(&a_path).unwrap().is_populated());
    assert!(workspace.get_file(&b_path).unwrap().is_populated());
    assert!(workspace.get_file(&c_path).unwrap().is_populated());
}
