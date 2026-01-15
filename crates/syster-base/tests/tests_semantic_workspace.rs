#![allow(clippy::unwrap_used)]
use syster::semantic::Workspace;
use syster::semantic::resolver::Resolver;
use syster::syntax::sysml::ast::SysMLFile;

use std::path::PathBuf;

use pest::Parser;
use syster::parser::SysMLParser;
use syster::parser::sysml::Rule;
use syster::syntax::SyntaxFile;
use syster::syntax::sysml::ast::parse_file;

#[test]
fn test_workspace_creation() {
    let workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_add_file() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file));

    assert_eq!(workspace.file_count(), 1);
    assert!(workspace.get_file(&path).is_some());
}

#[test]
fn test_populate_single_file() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

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
    let mut pairs1 = SysMLParser::parse(Rule::file, source1).unwrap();
    let file1 = parse_file(&mut pairs1).unwrap();

    // File 2: Derived definition
    let source2 = "part def Car :> Vehicle;";
    let mut pairs2 = SysMLParser::parse(Rule::file, source2).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

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
    // Car references Vehicle, so get_sources("Vehicle") should contain "Car"
    let sources = workspace.reference_index().get_sources("Vehicle");
    assert!(!sources.is_empty());
    assert!(sources.contains(&"Car"));
}

#[test]
fn test_update_file_content() {
    // TDD: Test LSP-style incremental updates
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Add initial file
    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::file, source1).unwrap();
    let file1 = parse_file(&mut pairs1).unwrap();

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
    let mut pairs2 = SysMLParser::parse(Rule::file, source2).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();

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
    let mut pairs3 = SysMLParser::parse(Rule::file, source3).unwrap();
    let file3 = parse_file(&mut pairs3).unwrap();

    let updated = workspace.update_file(&non_existent, syster::syntax::SyntaxFile::SysML(file3));
    assert!(!updated, "Updating non-existent file should return false");
}

#[test]
fn test_remove_file() {
    // TDD: Test file removal for LSP didClose
    let mut workspace = Workspace::<SyntaxFile>::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

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
    let mut pairs = SysMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();
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
    let mut pairs1 = SysMLParser::parse(Rule::file, source1).unwrap();
    let file1 = parse_file(&mut pairs1).unwrap();
    workspace.add_file(path.clone(), syster::syntax::SyntaxFile::SysML(file1));

    assert_eq!(workspace.get_file(&path).unwrap().version(), 0);

    // Update multiple times
    for i in 1..=5 {
        let source = format!("part def V{i};");
        let mut pairs = SysMLParser::parse(Rule::file, &source).unwrap();
        let file = parse_file(&mut pairs).unwrap();
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
    let mut pairs = SysMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

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
    let mut pairs2 = SysMLParser::parse(Rule::file, source2).unwrap();
    let file2 = parse_file(&mut pairs2).unwrap();
    workspace.update_file(&path, syster::syntax::SyntaxFile::SysML(file2));

    assert!(
        !workspace.get_file(&path).unwrap().is_populated(),
        "File should not be populated after content update"
    );
}

// Dependency tracking tests

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
    let mut pairs = SysMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

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
fn test_populate_affected_continues_on_error() {
    // Test that populate_affected continues processing files even when one has an error
    let mut workspace = Workspace::<SyntaxFile>::new();

    // Add a file with a duplicate symbol error
    let bad_file = r#"
        part def Car;
        part def Car;
    "#;
    let bad_path = PathBuf::from("bad.sysml");
    let mut pairs = SysMLParser::parse(Rule::file, bad_file).unwrap();
    let parsed_bad = parse_file(&mut pairs).unwrap();
    workspace.add_file(bad_path.clone(), SyntaxFile::SysML(parsed_bad));

    // Add a valid file
    let good_file = r#"
        part def Truck;
    "#;
    let good_path = PathBuf::from("good.sysml");
    let mut pairs = SysMLParser::parse(Rule::file, good_file).unwrap();
    let parsed_good = parse_file(&mut pairs).unwrap();
    workspace.add_file(good_path.clone(), SyntaxFile::SysML(parsed_good));

    // populate_affected should succeed even though one file has an error
    let result = workspace.populate_affected();
    assert!(
        result.is_ok(),
        "populate_affected should succeed even with errors in individual files"
    );

    // The good file should have been processed
    let truck_exists = workspace
        .symbol_table()
        .iter_symbols()
        .any(|sym| sym.name() == "Truck");
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
fn test_workspace_new_generic() {
    let workspace = Workspace::<SyntaxFile>::new();
    assert_eq!(workspace.file_count(), 0);
    assert_eq!(workspace.files().len(), 0);
    assert!(!workspace.has_stdlib());
}

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

/// Test that `meta` type annotations in expressions are indexed in a Workspace.
/// This is the full end-to-end test for Find References on types like `SysML::Usage`.
#[test]
fn test_workspace_meta_type_references() {
    let mut workspace = Workspace::<SyntaxFile>::new();

    // File with metadata def containing meta type reference
    let source = r#"
        metadata def TestMeta {
            ref :>> baseType = causations meta SysML::Usage;
        }
    "#;

    let mut pairs = SysMLParser::parse(Rule::file, source).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    let path = PathBuf::from("test.sysml");
    workspace.add_file(path.clone(), SyntaxFile::SysML(file));
    workspace.populate_file(&path).unwrap();

    // Verify the meta type reference is indexed
    let refs = workspace.reference_index().get_references("SysML::Usage");
    assert!(
        !refs.is_empty(),
        "Expected meta type reference to SysML::Usage to be indexed, got: {:?}",
        refs
    );

    // Verify the reference has correct span information
    let ref_info = refs.first().unwrap();
    assert_eq!(ref_info.file, path);
    assert!(ref_info.span.start.line > 0 || ref_info.span.start.column > 0);
}
