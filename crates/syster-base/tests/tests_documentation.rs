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
use syster::semantic::{Resolver, SymbolTable, Workspace};
use syster::syntax::SyntaxFile;

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
    let _reference_index = workspace.reference_index();

    // Verify they work correctly
    assert!(
        Resolver::new(symbol_table).resolve("NonExistent").is_none(),
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
    use syster::semantic::graphs;
    use syster::semantic::resolver;
    use syster::semantic::symbol_table;
    use syster::semantic::workspace;

    // Verify key types are public as documented
    let _: SymbolTable;
    let _: Resolver;
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
    };

    let classifier = Symbol::Classifier {
        name: "Test".to_string(),
        qualified_name: "Test".to_string(),
        kind: "class".to_string(),
        is_abstract: false,
        scope_id: 0,
        source_file: None,
        span: None,
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

/// Verify reference index methods exist
#[test]
fn test_reference_index_api_matches_docs() {
    use std::path::PathBuf;
    use syster::core::Span;
    use syster::semantic::graphs::ReferenceIndex;

    let mut index = ReferenceIndex::new();

    // Add a reference with span information
    let file = PathBuf::from("test.sysml");
    let span = Span::from_coords(0, 0, 0, 10);
    index.add_reference("Vehicle", "Car", Some(&file), Some(span));

    // Get sources that reference a target
    let sources = index.get_sources("Car");
    assert_eq!(sources.len(), 1);
    assert!(sources.contains(&"Vehicle"));

    // Check if has references
    assert!(index.has_references("Car"));
    assert!(!index.has_references("Unknown"));
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
