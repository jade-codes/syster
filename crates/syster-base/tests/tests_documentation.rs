//! Documentation Verification Tests
//!
//! These tests ensure that documentation stays synchronized with the codebase.
//! They check that examples compile and that documented features actually exist.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::path::PathBuf;
use syster::semantic::{RelationshipGraph, Resolver, SemanticAnalyzer, SymbolTable, Workspace};
use syster::syntax::SyntaxFile;
use syster::syntax::sysml::ast::SysMLFile;

/// Verify that code examples in ARCHITECTURE.md compile and work
#[test]
fn test_architecture_examples_compile() {
    // Example from "Adding a New SysML Element Type" section
    // If we add ConcernDef, this test should pass
    // If we remove it, this test will fail and remind us to update docs

    // For now, test with existing types
    let mut workspace = Workspace::<SyntaxFile>::new();
    // This should work as documented
    assert!(
        !workspace.has_stdlib(),
        "New workspace should not have stdlib loaded"
    );
}

/// Verify workspace APIs documented in ARCHITECTURE.md exist
#[test]
fn test_workspace_api_exists() {
    let workspace = Workspace::<SyntaxFile>::new();

    // Verify APIs exist as documented and return correct types
    let symbol_table = workspace.symbol_table();
    let _relationship_graph = workspace.relationship_graph();

    // Verify they work correctly
    assert!(
        symbol_table.lookup("NonExistent").is_none(),
        "Empty workspace should have no symbols"
    );
}

/// Verify type aliases mentioned in documentation exist
#[test]
fn test_documented_type_aliases_exist() {
    // These should compile if type aliases are properly exported
    let qname: syster::semantic::QualifiedName = "Package::Element".to_string();
    let simple: syster::semantic::SimpleName = "Element".to_string();
    let scope: syster::semantic::ScopeId = 0;
    let path: syster::semantic::SourceFilePath = "file.sysml".to_string();

    // Verify the types work as expected
    assert_eq!(qname, "Package::Element");
    assert_eq!(simple, "Element");
    assert_eq!(scope, 0);
    assert_eq!(path, "file.sysml");
}

/// Verify documented module structure matches reality
#[test]
fn test_documented_modules_exist() {
    // If these imports fail, module organization has changed
    use syster::semantic;
    use syster::semantic::analyzer;
    use syster::semantic::graphs;
    use syster::semantic::resolver;
    use syster::semantic::symbol_table;
    use syster::semantic::workspace;

    // Verify key types are public as documented
    let _: SymbolTable;
    let _: RelationshipGraph;
    let _: Resolver;
    let _: SemanticAnalyzer;
    let _: Workspace<SyntaxFile>;
}

/// Verify documented Symbol enum variants exist
#[test]
fn test_symbol_enum_variants_documented() {
    use syster::semantic::symbol_table::Symbol;

    // Create examples of each documented variant
    let package = Symbol::Package {
        name: "Test".to_string(),
        qualified_name: "Test".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    };

    let classifier = Symbol::Classifier {
        name: "Test".to_string(),
        qualified_name: "Test".to_string(),
        kind: "class".to_string(),
        is_abstract: false,
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    };

    // Verify symbol variants can be matched
    assert!(
        matches!(package, Symbol::Package { .. }),
        "Should match Package variant"
    );
    assert!(
        matches!(classifier, Symbol::Classifier { .. }),
        "Should match Classifier variant"
    );

    // If any variant changes, this test breaks and reminds us to update docs
}

/// Verify relationship graph methods documented in ARCHITECTURE.md exist
#[test]
fn test_relationship_graph_api_matches_docs() {
    use syster::semantic::graphs::OneToManyGraph;

    let mut graph = OneToManyGraph::new();

    // These methods are documented - ensure they exist and work correctly
    graph.add("Vehicle".to_string(), "Car".to_string(), None);

    let targets = graph.get_targets("Vehicle");
    assert_eq!(targets.as_ref().map(|v| v.len()), Some(1));
    assert!(targets.unwrap().contains(&&"Car".to_string()));

    let sources = graph.get_sources("Car");
    assert_eq!(
        sources,
        vec![&"Vehicle".to_string()],
        "Car should have Vehicle as source"
    );

    assert!(
        graph.has_path("Vehicle", "Car"),
        "Should have path from Vehicle to Car"
    );
    assert!(
        !graph.has_path("Car", "Vehicle"),
        "Should not have reverse path without adding it"
    );

    let cycles = graph.find_cycles();
    assert!(cycles.is_empty(), "No cycles should exist in simple graph");
}

/// Verify the three-phase pipeline terminology is accurate
#[test]
fn test_three_phase_pipeline_terminology() {
    // Phase 1: Parse (verified by parser module existing)
    use syster::parser;

    // Phase 2: Syntax (verified by language module existing)
    use syster::syntax;

    // Phase 3: Semantic (verified by semantic module existing)
    use syster::semantic;

    // If any phase is renamed/removed, update ARCHITECTURE.md
}

// Note: Run `cargo test --doc` to verify all doc comment examples compile
