#![allow(clippy::unwrap_used)]
use syster::semantic::Workspace;
use syster::semantic::resolver::Resolver;

use std::path::PathBuf;

use from_pest::FromPest;
use pest::Parser;
use syster::core::constants::REL_SPECIALIZATION;
use syster::parser::SysMLParser;
use syster::parser::sysml::Rule;
use syster::syntax::SyntaxFile;
use syster::syntax::sysml::ast::SysMLFile;

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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

    let result = workspace.populate_file(&path);
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify symbol was added to the shared symbol table
    let resolver = Resolver::new(workspace.symbol_table());
    let symbol = resolver.resolve("Vehicle");
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
        syster::syntax::SyntaxFile::SysML(file1),
    );
    workspace.add_file(
        PathBuf::from("car.sysml"),
        syster::syntax::SyntaxFile::SysML(file2),
    );

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify both symbols are in the shared symbol table
    let resolver = Resolver::new(workspace.symbol_table());
    let vehicle = resolver.resolve("Vehicle");
    assert!(vehicle.is_some());
    assert_eq!(vehicle.unwrap().source_file(), Some("vehicle.sysml"));

    let resolver = Resolver::new(workspace.symbol_table());
    let car = resolver.resolve("Car");
    assert!(car.is_some());
    assert_eq!(car.unwrap().source_file(), Some("car.sysml"));

    // Verify the specialization relationship was captured
    let specializes = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car");
    assert_eq!(specializes.as_ref().map(|v| v.len()), Some(1));
    assert!(specializes.unwrap().contains(&"Vehicle"));
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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file1));
    workspace.populate_file(&path).unwrap();

    // Verify initial content
    let resolver = Resolver::new(workspace.symbol_table());
    let symbol = resolver.resolve("Vehicle");
    assert!(symbol.is_some());

    // Get initial version
    let file = workspace.get_file(&path).unwrap();
    assert_eq!(file.version(), 0, "Initial version should be 0");
    assert!(file.is_populated(), "File should be populated");

    // Update file content (simulating LSP didChange)
    let source2 = "part def Car;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    let updated = workspace.update_file(&path, syster::syntax::SyntaxFile::SysML(file2));
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

    let updated = workspace.update_file(&non_existent, syster::syntax::SyntaxFile::SysML(file3));
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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file1));

    assert_eq!(workspace.get_file(&path).unwrap().version(), 0);

    // Update multiple times
    for i in 1..=5 {
        let source = format!("part def V{i};");
        let mut pairs = SysMLParser::parse(Rule::model, &source).unwrap();
        let file = SysMLFile::from_pest(&mut pairs).unwrap();
        workspace.update_file(&path, syster::syntax::SyntaxFile::SysML(file));

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

    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));
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
    workspace.update_file(&path, syster::syntax::SyntaxFile::SysML(file2));

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
        syster::syntax::SyntaxFile::SysML(base_file),
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
    workspace.add_file(
        app_path.clone(),
        syster::syntax::SyntaxFile::SysML(app_file),
    );

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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file_v1));

    // Update to only import C
    let source_v2 = r#"
        import C::*;
        part def Test;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source_v2).unwrap();
    let file_v2 = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.update_file(&path, syster::syntax::SyntaxFile::SysML(file_v2));

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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

    // Remove the file
    workspace.remove_file(&path);

    // After removal, file should not exist
    assert!(workspace.get_file(&path).is_none());
}

#[test]
fn test_subscribe_to_file_added() {
    use std::sync::{Arc, Mutex};
    use syster::semantic::types::WorkspaceEvent;

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

    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

    let events = events_received.lock().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0], WorkspaceEvent::FileAdded { path });
}

#[test]
fn test_subscribe_to_file_updated() {
    use std::sync::{Arc, Mutex};
    use syster::semantic::types::WorkspaceEvent;

    let mut workspace = Workspace::<SyntaxFile>::new();
    let path = PathBuf::from("test.sysml");

    // Add file first
    workspace.add_file(
        path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );

    // Add app file
    workspace.add_file(
        app_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        c_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        c_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        c_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        b_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        app_path.clone(),
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
        syster::syntax::SyntaxFile::SysML(SysMLFile {
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
fn test_populate_affected_continues_on_error() {
    // Test that populate_affected continues processing files even when one has an error
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Add a file with a duplicate symbol error
    let bad_file = r#"
        part def Car;
        part def Car;
    "#;
    let bad_path = PathBuf::from("bad.sysml");
    let mut pairs = SysMLParser::parse(Rule::model, bad_file).unwrap();
    let parsed_bad = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(bad_path.clone(), SyntaxFile::SysML(parsed_bad));

    // Add a valid file
    let good_file = r#"
        part def Truck;
    "#;
    let good_path = PathBuf::from("good.sysml");
    let mut pairs = SysMLParser::parse(Rule::model, good_file).unwrap();
    let parsed_good = SysMLFile::from_pest(&mut pairs).unwrap();
    workspace.add_file(good_path.clone(), SyntaxFile::SysML(parsed_good));

    // populate_affected should succeed even though one file has an error
    let result = workspace.populate_affected();
    assert!(
        result.is_ok(),
        "populate_affected should succeed even with errors in individual files"
    );

    // The good file should have been processed
    let symbols = workspace.symbol_table().all_symbols();
    let truck_exists = symbols.iter().any(|(name, _)| *name == "Truck");
    assert!(truck_exists, "Valid file should have been processed");
}

// ============================================================================
// WORKSPACE CORE API TESTS (Issues #392, #391, #389, #388, #386, #384, #382, #381, #380, #379, #378, #377, #376)
// ============================================================================

// Tests for Workspace::default() - Issue #392
#[test]
fn test_workspace_default() {
    let workspace = Workspace::<SyntaxFile>::default();
    assert_eq!(workspace.file_count(), 0);
    assert!(!workspace.has_stdlib());
}

#[test]
fn test_workspace_default_initializes_empty_structures() {
    let workspace = Workspace::<SyntaxFile>::default();
    assert_eq!(workspace.files().len(), 0);
    assert_eq!(workspace.dependency_graph().dependencies_count(), 0);
}

// Tests for Workspace::<T>::new() - Issue #391
#[test]
fn test_workspace_new_generic() {
    let workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.file_count(), 0);
    assert_eq!(workspace.files().len(), 0);
    assert!(!workspace.has_stdlib());
}

#[test]
fn test_workspace_new_initializes_all_fields() {
    let workspace = Workspace::<SyntaxFile>::new();
    // Verify all internal structures are initialized
    assert_eq!(workspace.file_count(), 0);
    assert_eq!(workspace.symbol_table().all_symbols().len(), 0);
    assert_eq!(workspace.dependency_graph().dependencies_count(), 0);
    assert!(!workspace.has_stdlib());
}

// Tests for Workspace::<SyntaxFile>::new() - Issue #389
#[test]
fn test_workspace_syntax_file_new() {
    let workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_workspace_syntax_file_new_type_safety() {
    // Verify that Workspace can be created with SyntaxFile type
    let _workspace: Workspace<SyntaxFile> = Workspace::new();
    // If this compiles, the type constraint is satisfied
}

// Tests for Workspace::with_stdlib() - Issue #388
#[test]
fn test_workspace_with_stdlib() {
    let workspace = Workspace::<SyntaxFile>::with_stdlib();
    assert!(workspace.has_stdlib());
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_workspace_with_stdlib_vs_new() {
    let workspace_no_stdlib = Workspace::<SyntaxFile>::new();
    let workspace_with_stdlib = Workspace::<SyntaxFile>::with_stdlib();

    assert!(!workspace_no_stdlib.has_stdlib());
    assert!(workspace_with_stdlib.has_stdlib());
}

// Tests for dependency_graph_mut() - Issue #386
#[test]
fn test_dependency_graph_mut() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    let dep_graph = workspace.dependency_graph_mut();

    let path_a = PathBuf::from("a.sysml");
    let path_b = PathBuf::from("b.sysml");

    dep_graph.add_dependency(&path_a, &path_b);

    // Verify the dependency was added
    assert_eq!(workspace.dependency_graph().dependencies_count(), 1);
}

#[test]
fn test_dependency_graph_mut_allows_modifications() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path_a = PathBuf::from("a.sysml");
    let path_b = PathBuf::from("b.sysml");
    let path_c = PathBuf::from("c.sysml");

    // Add multiple dependencies
    workspace
        .dependency_graph_mut()
        .add_dependency(&path_a, &path_b);
    workspace
        .dependency_graph_mut()
        .add_dependency(&path_b, &path_c);

    assert_eq!(workspace.dependency_graph().dependencies_count(), 2);
}

// Tests for relationship_graph() - Issue #384
#[test]
fn test_relationship_graph_immutable() {
    let workspace = Workspace::<SyntaxFile>::new();
    let _rel_graph = workspace.relationship_graph();

    // Verify we get a reference to the relationship graph
    // The graph should be initialized for a new workspace
}

#[test]
fn test_relationship_graph_immutable_multiple_references() {
    let workspace = Workspace::<SyntaxFile>::new();
    let _rel_graph_ref1 = workspace.relationship_graph();
    let _rel_graph_ref2 = workspace.relationship_graph();

    // Multiple immutable references should be allowed
}

// Tests for dependency_graph() - Issue #382
#[test]
fn test_dependency_graph_immutable() {
    let workspace = Workspace::<SyntaxFile>::new();
    let dep_graph = workspace.dependency_graph();

    // Verify we get a reference to an empty dependency graph
    assert_eq!(dep_graph.dependencies_count(), 0);
}

#[test]
fn test_dependency_graph_immutable_after_mutations() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path_a = PathBuf::from("a.sysml");
    let path_b = PathBuf::from("b.sysml");

    workspace
        .dependency_graph_mut()
        .add_dependency(&path_a, &path_b);

    // Get immutable reference and verify data
    let dep_graph = workspace.dependency_graph();
    assert_eq!(dep_graph.dependencies_count(), 1);
}

// Tests for file_paths() - Issue #381
#[test]
fn test_file_paths_empty() {
    let workspace = Workspace::<SyntaxFile>::new();
    let paths: Vec<&PathBuf> = workspace.file_paths().collect();

    assert_eq!(paths.len(), 0);
}

#[test]
fn test_file_paths_with_files() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path1 = PathBuf::from("file1.sysml");
    let path2 = PathBuf::from("file2.sysml");
    let path3 = PathBuf::from("dir/file3.sysml");

    workspace.add_file(
        path1.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        path2.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        path3.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );

    let paths: Vec<&PathBuf> = workspace.file_paths().collect();
    assert_eq!(paths.len(), 3);

    // Verify all paths are present
    assert!(paths.contains(&&path1));
    assert!(paths.contains(&&path2));
    assert!(paths.contains(&&path3));
}

#[test]
fn test_file_paths_iterator_trait() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path1 = PathBuf::from("a.sysml");
    let path2 = PathBuf::from("b.sysml");

    workspace.add_file(
        path1.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        path2.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );

    // Test iterator operations
    assert_eq!(workspace.file_paths().count(), 2);

    // Test that we can iterate multiple times
    let count1 = workspace.file_paths().count();
    let count2 = workspace.file_paths().count();
    assert_eq!(count1, count2);
}

// Tests for file_count() - Issue #380
#[test]
fn test_file_count_empty() {
    let workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_file_count_increments() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.file_count(), 0);

    workspace.add_file(
        PathBuf::from("file1.sysml"),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    assert_eq!(workspace.file_count(), 1);

    workspace.add_file(
        PathBuf::from("file2.sysml"),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    assert_eq!(workspace.file_count(), 2);
}

#[test]
fn test_file_count_after_removal() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    let path = PathBuf::from("file.sysml");

    workspace.add_file(
        path.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    assert_eq!(workspace.file_count(), 1);

    workspace.remove_file(&path);
    assert_eq!(workspace.file_count(), 0);
}

// Tests for Workspace::<SyntaxFile>::relationship_graph() - Issue #379
#[test]
fn test_syntax_file_workspace_relationship_graph() {
    let workspace: Workspace<SyntaxFile> = Workspace::new();
    let _rel_graph = workspace.relationship_graph();

    // Verify the specific type works correctly
}

#[test]
fn test_syntax_file_workspace_relationship_graph_type_specific() {
    use syster::semantic::graphs::RelationshipGraph;

    // Verify type-specific behavior for SyntaxFile workspace
    let workspace = Workspace::<SyntaxFile>::new();
    let _rel_graph: &RelationshipGraph = workspace.relationship_graph();

    // This test ensures the method works with the concrete SyntaxFile type
}

// Tests for relationship_graph_mut() - Issue #378
#[test]
fn test_relationship_graph_mut_basic() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    let rel_graph = workspace.relationship_graph_mut();

    rel_graph.add_one_to_many(REL_SPECIALIZATION, "Car", "Vehicle", None, None);

    // Verify the relationship was added
    let specializes = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car");
    assert!(specializes.is_some());
}

#[test]
fn test_relationship_graph_mut_allows_modifications() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Add multiple relationships
    workspace.relationship_graph_mut().add_one_to_many(
        REL_SPECIALIZATION,
        "Car",
        "Vehicle",
        None,
        None,
    );
    workspace.relationship_graph_mut().add_one_to_many(
        REL_SPECIALIZATION,
        "Truck",
        "Vehicle",
        None,
        None,
    );

    // Verify both relationships exist
    let car_specializes = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car");
    let truck_specializes = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Truck");

    assert!(car_specializes.is_some());
    assert!(truck_specializes.is_some());
}

// Tests for get_file_dependents() - Issue #377
#[test]
fn test_get_file_dependents_no_dependents() {
    let workspace = Workspace::<SyntaxFile>::new();
    let path = PathBuf::from("file.sysml");

    let dependents = workspace.get_file_dependents(&path);
    assert_eq!(dependents.len(), 0);
}

#[test]
fn test_get_file_dependents_with_single_dependent() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let base_path = PathBuf::from("base.sysml");
    let app_path = PathBuf::from("app.sysml");

    // app depends on base
    workspace
        .dependency_graph_mut()
        .add_dependency(&app_path, &base_path);

    // Get dependents of base
    let dependents = workspace.get_file_dependents(&base_path);
    assert_eq!(dependents.len(), 1);
    assert!(dependents.contains(&app_path));
}

#[test]
fn test_get_file_dependents_with_multiple_dependents() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let base_path = PathBuf::from("base.sysml");
    let app1_path = PathBuf::from("app1.sysml");
    let app2_path = PathBuf::from("app2.sysml");
    let app3_path = PathBuf::from("app3.sysml");

    // Multiple apps depend on base
    workspace
        .dependency_graph_mut()
        .add_dependency(&app1_path, &base_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&app2_path, &base_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&app3_path, &base_path);

    // Get dependents of base
    let dependents = workspace.get_file_dependents(&base_path);
    assert_eq!(dependents.len(), 3);
    assert!(dependents.contains(&app1_path));
    assert!(dependents.contains(&app2_path));
    assert!(dependents.contains(&app3_path));
}

#[test]
fn test_get_file_dependents_chain() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let a_path = PathBuf::from("a.sysml");
    let b_path = PathBuf::from("b.sysml");
    let c_path = PathBuf::from("c.sysml");

    // Chain: C -> B -> A
    workspace
        .dependency_graph_mut()
        .add_dependency(&c_path, &b_path);
    workspace
        .dependency_graph_mut()
        .add_dependency(&b_path, &a_path);

    // Get immediate dependents only
    let a_dependents = workspace.get_file_dependents(&a_path);
    assert_eq!(a_dependents.len(), 1);
    assert!(a_dependents.contains(&b_path));

    let b_dependents = workspace.get_file_dependents(&b_path);
    assert_eq!(b_dependents.len(), 1);
    assert!(b_dependents.contains(&c_path));
}

// Tests for symbol_table_mut() - Issue #376
#[test]
fn test_symbol_table_mut_basic() {
    use syster::semantic::symbol_table::Symbol;

    let mut workspace = Workspace::<SyntaxFile>::new();
    let symbol_table = workspace.symbol_table_mut();

    // Add a symbol directly
    let symbol = Symbol::Package {
        name: "TestPackage".to_string(),
        qualified_name: "TestPackage".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    };
    symbol_table
        .insert("TestPackage".to_string(), symbol)
        .unwrap();

    // Verify it was added
    let resolver = Resolver::new(workspace.symbol_table());
    let lookup = resolver.resolve("TestPackage");
    assert!(lookup.is_some());
}

#[test]
fn test_symbol_table_mut_allows_modifications() {
    use syster::semantic::symbol_table::Symbol;

    let mut workspace = Workspace::<SyntaxFile>::new();

    // Add multiple symbols
    workspace
        .symbol_table_mut()
        .insert(
            "Symbol1".to_string(),
            Symbol::Package {
                name: "Symbol1".to_string(),
                qualified_name: "Symbol1".to_string(),
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();
    workspace
        .symbol_table_mut()
        .insert(
            "Symbol2".to_string(),
            Symbol::Package {
                name: "Symbol2".to_string(),
                qualified_name: "Symbol2".to_string(),
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Verify both exist
    assert!(
        Resolver::new(workspace.symbol_table())
            .resolve("Symbol1")
            .is_some()
    );
    assert!(
        Resolver::new(workspace.symbol_table())
            .resolve("Symbol2")
            .is_some()
    );
}

#[test]
fn test_symbol_table_mut_independent_from_immutable() {
    use syster::semantic::symbol_table::Symbol;

    let mut workspace = Workspace::<SyntaxFile>::new();

    // Add symbol via mutable reference
    workspace
        .symbol_table_mut()
        .insert(
            "Test".to_string(),
            Symbol::Package {
                name: "Test".to_string(),
                qualified_name: "Test".to_string(),
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Access via immutable reference
    let resolver = Resolver::new(workspace.symbol_table());
    let symbol = resolver.resolve("Test");
    assert!(symbol.is_some());
    assert_eq!(symbol.unwrap().name(), "Test");
}

// Edge case tests
#[test]
fn test_workspace_default_equals_new() {
    let workspace1 = Workspace::<SyntaxFile>::default();
    let workspace2 = Workspace::<SyntaxFile>::new();

    // Both should have identical initial state
    assert_eq!(workspace1.file_count(), workspace2.file_count());
    assert_eq!(workspace1.has_stdlib(), workspace2.has_stdlib());
}

#[test]
fn test_file_paths_empty_after_all_removed() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let path1 = PathBuf::from("file1.sysml");
    let path2 = PathBuf::from("file2.sysml");

    workspace.add_file(
        path1.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );
    workspace.add_file(
        path2.clone(),
        SyntaxFile::SysML(SysMLFile {
            namespaces: vec![],
            namespace: None,
            elements: vec![],
        }),
    );

    assert_eq!(workspace.file_count(), 2);

    workspace.remove_file(&path1);
    workspace.remove_file(&path2);

    assert_eq!(workspace.file_count(), 0);
    assert_eq!(workspace.file_paths().count(), 0);
}

#[test]
fn test_get_file_dependents_empty_graph() {
    let workspace = Workspace::<SyntaxFile>::new();
    let path = PathBuf::from("nonexistent.sysml");

    let dependents = workspace.get_file_dependents(&path);
    assert!(dependents.is_empty());
}

// =============================================================================
// Tests for file update clearing relationships (Issue: hover shows duplicates)
// =============================================================================

#[test]
fn test_update_file_clears_relationships() {
    // BUG: When a file is updated, old relationships should be cleared
    // before reparsing. Otherwise, duplicates accumulate.
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.enable_auto_invalidation();

    let path = PathBuf::from("/test/file.sysml");
    let source = r#"
        part def Vehicle;
        part def Car :> Vehicle;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    // Add and populate the file
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));
    workspace.populate_file(&path).unwrap();

    // Verify initial state - one specialization relationship
    let refs = workspace.relationship_graph().get_references_to("Vehicle");
    assert_eq!(
        refs.len(),
        1,
        "Should have 1 reference to Vehicle initially"
    );

    // Now update the file with the same content (simulating a save/edit)
    let source2 = r#"
        part def Vehicle;
        part def Car :> Vehicle;
    "#;
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    workspace.update_file(&path, syster::syntax::SyntaxFile::SysML(file2));
    workspace.populate_affected().unwrap();

    // After update + repopulate, should still have exactly 1 reference, not 2
    let refs_after = workspace.relationship_graph().get_references_to("Vehicle");
    assert_eq!(
        refs_after.len(),
        1,
        "Should still have 1 reference after update, not duplicates. Got: {}",
        refs_after.len()
    );
}

#[test]
fn test_update_file_clears_multiple_relationship_types() {
    // Test that all relationship types are cleared on update
    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.enable_auto_invalidation();

    let path = PathBuf::from("/test/file.sysml");
    let source = r#"
        part def Base;
        part def Derived :> Base;
        part instance : Base;
    "#;
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));
    workspace.populate_file(&path).unwrap();

    // Should have 2 references: specialization + typing
    let refs = workspace.relationship_graph().get_references_to("Base");
    assert_eq!(refs.len(), 2, "Should have 2 references initially");

    // Update 3 times (simulating multiple edits)
    for _ in 0..3 {
        let source2 = r#"
            part def Base;
            part def Derived :> Base;
            part instance : Base;
        "#;
        let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
        let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

        workspace.update_file(&path, syster::syntax::SyntaxFile::SysML(file2));
        workspace.populate_affected().unwrap();
    }

    // Should still have exactly 2 references, not 8 (2 * 4)
    let refs_after = workspace.relationship_graph().get_references_to("Base");
    assert_eq!(
        refs_after.len(),
        2,
        "Should still have 2 references after 3 updates, not duplicates. Got: {}",
        refs_after.len()
    );
}
