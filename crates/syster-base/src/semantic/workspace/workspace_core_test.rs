#![allow(clippy::unwrap_used)]

use super::*;
use crate::semantic::graphs::RelationshipGraph;
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

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
    workspace.dependency_graph_mut().add_dependency(&path_a, &path_b);
    workspace.dependency_graph_mut().add_dependency(&path_b, &path_c);
    
    assert_eq!(workspace.dependency_graph().dependencies_count(), 2);
}

// Tests for relationship_graph() - Issue #384
#[test]
fn test_relationship_graph() {
    let workspace = Workspace::<SyntaxFile>::new();
    let _rel_graph = workspace.relationship_graph();
    
    // Verify we get a reference to the relationship graph
    // The graph should be initialized for a new workspace
}

#[test]
fn test_relationship_graph_immutable_reference() {
    let workspace = Workspace::<SyntaxFile>::new();
    let _rel_graph_ref1 = workspace.relationship_graph();
    let _rel_graph_ref2 = workspace.relationship_graph();
    
    // Multiple immutable references should be allowed
}

// Tests for dependency_graph() - Issue #382
#[test]
fn test_dependency_graph() {
    let workspace = Workspace::<SyntaxFile>::new();
    let dep_graph = workspace.dependency_graph();
    
    // Verify we get a reference to an empty dependency graph
    assert_eq!(dep_graph.dependencies_count(), 0);
}

#[test]
fn test_dependency_graph_immutable_reference() {
    let mut workspace = Workspace::<SyntaxFile>::new();
    
    let path_a = PathBuf::from("a.sysml");
    let path_b = PathBuf::from("b.sysml");
    
    workspace.dependency_graph_mut().add_dependency(&path_a, &path_b);
    
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
    use crate::syntax::sysml::ast::SysMLFile;
    
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
    
    use crate::syntax::sysml::ast::SysMLFile;
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
fn test_file_count_with_files() {
    use crate::syntax::sysml::ast::SysMLFile;
    
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
    use crate::syntax::sysml::ast::SysMLFile;
    
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
    // Verify type-specific behavior for SyntaxFile workspace
    let workspace = Workspace::<SyntaxFile>::new();
    let _rel_graph: &RelationshipGraph = workspace.relationship_graph();
    
    // This test ensures the method works with the concrete SyntaxFile type
}

// Tests for relationship_graph_mut() - Issue #378
#[test]
fn test_relationship_graph_mut() {
    use crate::core::constants::REL_SPECIALIZATION;
    
    let mut workspace = Workspace::<SyntaxFile>::new();
    let rel_graph = workspace.relationship_graph_mut();
    
    rel_graph.add_one_to_many(REL_SPECIALIZATION, "Car".to_string(), "Vehicle".to_string(), None);
    
    // Verify the relationship was added
    let specializes = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car");
    assert!(specializes.is_some());
}

#[test]
fn test_relationship_graph_mut_allows_modifications() {
    use crate::core::constants::REL_SPECIALIZATION;
    
    let mut workspace = Workspace::<SyntaxFile>::new();
    
    // Add multiple relationships
    workspace
        .relationship_graph_mut()
        .add_one_to_many(REL_SPECIALIZATION, "Car".to_string(), "Vehicle".to_string(), None);
    workspace
        .relationship_graph_mut()
        .add_one_to_many(REL_SPECIALIZATION, "Truck".to_string(), "Vehicle".to_string(), None);
    
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
fn test_symbol_table_mut() {
    use crate::semantic::symbol_table::Symbol;
    
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
    symbol_table.insert("TestPackage".to_string(), symbol).unwrap();
    
    // Verify it was added
    let lookup = workspace.symbol_table().lookup("TestPackage");
    assert!(lookup.is_some());
}

#[test]
fn test_symbol_table_mut_allows_modifications() {
    use crate::semantic::symbol_table::Symbol;
    
    let mut workspace = Workspace::<SyntaxFile>::new();
    
    // Add multiple symbols
    workspace
        .symbol_table_mut()
        .insert("Symbol1".to_string(), Symbol::Package {
            name: "Symbol1".to_string(),
            qualified_name: "Symbol1".to_string(),
            scope_id: 0,
            source_file: None,
            span: None,
            references: Vec::new(),
        })
        .unwrap();
    workspace
        .symbol_table_mut()
        .insert("Symbol2".to_string(), Symbol::Package {
            name: "Symbol2".to_string(),
            qualified_name: "Symbol2".to_string(),
            scope_id: 0,
            source_file: None,
            span: None,
            references: Vec::new(),
        })
        .unwrap();
    
    // Verify both exist
    assert!(workspace.symbol_table().lookup("Symbol1").is_some());
    assert!(workspace.symbol_table().lookup("Symbol2").is_some());
}

#[test]
fn test_symbol_table_mut_independent_from_immutable() {
    use crate::semantic::symbol_table::Symbol;
    
    let mut workspace = Workspace::<SyntaxFile>::new();
    
    // Add symbol via mutable reference
    workspace
        .symbol_table_mut()
        .insert("Test".to_string(), Symbol::Package {
            name: "Test".to_string(),
            qualified_name: "Test".to_string(),
            scope_id: 0,
            source_file: None,
            span: None,
            references: Vec::new(),
        })
        .unwrap();
    
    // Access via immutable reference
    let symbol = workspace.symbol_table().lookup("Test");
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
    use crate::syntax::sysml::ast::SysMLFile;
    
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
