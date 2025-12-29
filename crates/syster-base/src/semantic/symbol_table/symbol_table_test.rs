#![allow(clippy::unwrap_used)]

use super::*;

// ============================================================================
// Tests for lookup_mut (which internally tests build_scope_chain)
// ============================================================================

#[test]
fn test_lookup_mut_finds_symbol_in_current_scope() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "CurrentSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "CurrentSymbol".to_string(),
                qualified_name: "CurrentSymbol".to_string(),
            },
        )
        .unwrap();

    let found = table.lookup_mut("CurrentSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "CurrentSymbol");
}

#[test]
fn test_lookup_mut_finds_symbol_in_parent_scope() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "ParentSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "ParentSymbol".to_string(),
                qualified_name: "ParentSymbol".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    let found = table.lookup_mut("ParentSymbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().name(), "ParentSymbol");
}

#[test]
fn test_lookup_mut_returns_none_for_nonexistent() {
    let mut table = SymbolTable::new();
    let found = table.lookup_mut("DoesNotExist");
    assert!(found.is_none());
}

#[test]
fn test_lookup_mut_with_shadowing() {
    let mut table = SymbolTable::new();

    // Parent scope symbol
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: Some("parent.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Symbol".to_string(),
            },
        )
        .unwrap();

    // Child scope symbol with same name
    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: Some("child.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Child::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Should find child scope symbol (nearest)
    let found = table.lookup_mut("Symbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().source_file(), Some("child.sysml"));
}

#[test]
fn test_lookup_mut_in_deeply_nested_scopes() {
    let mut table = SymbolTable::new();

    // Level 0
    table
        .insert(
            "Level0".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Level0".to_string(),
                qualified_name: "Level0".to_string(),
            },
        )
        .unwrap();

    // Create nested scopes
    for i in 1..=5 {
        table.enter_scope();
        table
            .insert(
                format!("Level{}", i),
                Symbol::Package {
                    scope_id: i,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: format!("Level{}", i),
                    qualified_name: format!("Level{}", i),
                },
            )
            .unwrap();
    }

    // From deepest level, should find all ancestors
    for i in 0..=5 {
        let found = table.lookup_mut(&format!("Level{}", i));
        assert!(found.is_some(), "Should find Level{}", i);
    }

    // Exit to middle level
    table.exit_scope();
    table.exit_scope();
    table.exit_scope();

    // Should only find up to current level
    assert!(table.lookup_mut("Level0").is_some());
    assert!(table.lookup_mut("Level1").is_some());
    assert!(table.lookup_mut("Level2").is_some());
    assert!(table.lookup_mut("Level3").is_none());
}

#[test]
fn test_lookup_mut_allows_mutation() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "Mutable".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Mutable".to_string(),
                qualified_name: "Mutable".to_string(),
            },
        )
        .unwrap();

    // Add reference through mutable access
    {
        let symbol = table.lookup_mut("Mutable").unwrap();
        assert_eq!(symbol.references().len(), 0);
        symbol.add_reference(SymbolReference {
            file: "test.sysml".to_string(),
            span: crate::core::Span::from_coords(1, 0, 1, 10),
        });
    }

    // Verify mutation persisted
    let symbol = table.lookup_mut("Mutable").unwrap();
    assert_eq!(symbol.references().len(), 1);
}

#[test]
fn test_lookup_mut_different_symbol_types() {
    let mut table = SymbolTable::new();

    // Add various symbol types
    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Pkg".to_string(),
                qualified_name: "Pkg".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Class".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Class".to_string(),
                qualified_name: "Pkg::Class".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Feature".to_string(),
            Symbol::Feature {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Feature".to_string(),
                qualified_name: "Pkg::Class::Feature".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    // All should be findable
    assert!(matches!(
        table.lookup_mut("Pkg").unwrap(),
        Symbol::Package { .. }
    ));
    assert!(matches!(
        table.lookup_mut("Class").unwrap(),
        Symbol::Classifier { .. }
    ));
    assert!(matches!(
        table.lookup_mut("Feature").unwrap(),
        Symbol::Feature { .. }
    ));
}

#[test]
fn test_lookup_mut_after_scope_changes() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    let scope1 = table.enter_scope();
    table
        .insert(
            "Child1".to_string(),
            Symbol::Package {
                scope_id: scope1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Child1".to_string(),
                qualified_name: "Root::Child1".to_string(),
            },
        )
        .unwrap();

    assert!(table.lookup_mut("Root").is_some());
    assert!(table.lookup_mut("Child1").is_some());

    table.exit_scope();
    assert!(table.lookup_mut("Root").is_some());
    assert!(table.lookup_mut("Child1").is_none());
}

// ============================================================================
// Tests for lookup_namespace_import (tested via lookup with imports)
// ============================================================================

#[test]
fn test_lookup_namespace_import_basic() {
    let mut table = SymbolTable::new();

    // Create a package with a symbol
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Package::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Exit and create another scope with import
    table.exit_scope();
    table.enter_scope();
    table.add_import("Package::*".to_string(), false, None, None);

    // Should find Symbol through namespace import
    let found = table.lookup("Symbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().qualified_name(), "Package::Symbol");
}

#[test]
fn test_lookup_namespace_import_does_not_find_non_member() {
    let mut table = SymbolTable::new();

    // Create a package with a symbol
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Package::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Exit and import
    table.exit_scope();
    table.enter_scope();
    table.add_import("Package::*".to_string(), false, None, None);

    // Should NOT find a symbol that doesn't exist
    let found = table.lookup("NonExistent");
    assert!(found.is_none());
}

#[test]
fn test_lookup_namespace_import_multiple_imports() {
    let mut table = SymbolTable::new();

    // Package A with SymbolA
    table
        .insert(
            "PackageA".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "PackageA".to_string(),
                qualified_name: "PackageA".to_string(),
            },
        )
        .unwrap();

    let scope_a = table.enter_scope();
    table
        .insert(
            "SymbolA".to_string(),
            Symbol::Classifier {
                scope_id: scope_a,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SymbolA".to_string(),
                qualified_name: "PackageA::SymbolA".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.exit_scope();

    // Package B with SymbolB
    table
        .insert(
            "PackageB".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "PackageB".to_string(),
                qualified_name: "PackageB".to_string(),
            },
        )
        .unwrap();

    let scope_b = table.enter_scope();
    table
        .insert(
            "SymbolB".to_string(),
            Symbol::Classifier {
                scope_id: scope_b,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SymbolB".to_string(),
                qualified_name: "PackageB::SymbolB".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.exit_scope();

    // New scope with both imports
    table.enter_scope();
    table.add_import("PackageA::*".to_string(), false, None, None);
    table.add_import("PackageB::*".to_string(), false, None, None);

    // Should find both symbols
    assert!(table.lookup("SymbolA").is_some());
    assert!(table.lookup("SymbolB").is_some());
}

#[test]
fn test_lookup_namespace_import_with_local_shadowing() {
    let mut table = SymbolTable::new();

    // Create package with Symbol
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: Some("imported.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Package::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.exit_scope();

    // New scope with import and local symbol
    table.enter_scope();
    table.add_import("Package::*".to_string(), false, None, None);
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Package {
                scope_id: 2,
                source_file: Some("local.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Local::Symbol".to_string(),
            },
        )
        .unwrap();

    // Local symbol should take precedence
    let found = table.lookup("Symbol");
    assert!(found.is_some());
    assert_eq!(found.unwrap().source_file(), Some("local.sysml"));
}

// ============================================================================
// Tests for lookup_recursive_import (tested via lookup with recursive imports)
// NOTE: The current implementation has a bug where it searches HashMap keys
// (simple names) instead of symbol qualified names, so recursive imports
// don't work as intended. These tests verify the current (buggy) behavior.
// ============================================================================

#[test]
fn test_lookup_recursive_import_does_not_find_nested_due_to_bug() {
    let mut table = SymbolTable::new();

    // Create Package::Nested::Symbol
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Nested".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Nested".to_string(),
                qualified_name: "Package::Nested".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Package::Nested::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.exit_scope();
    table.exit_scope();

    // New scope with recursive import
    table.enter_scope();
    table.add_import("Package::**".to_string(), true, None, None);

    // BUG: Should find deeply nested symbol through recursive import, but doesn't
    // because lookup_recursive_import checks HashMap keys (simple names) instead
    // of symbol qualified names
    let found = table.lookup("Symbol");
    assert!(found.is_none()); // Current buggy behavior
}

#[test]
fn test_lookup_recursive_import_current_behavior() {
    let mut table = SymbolTable::new();

    // Create Package::DirectSymbol and Package::Nested::DeepSymbol
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "DirectSymbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "DirectSymbol".to_string(),
                qualified_name: "Package::DirectSymbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "Nested".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Nested".to_string(),
                qualified_name: "Package::Nested".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "DeepSymbol".to_string(),
            Symbol::Classifier {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "DeepSymbol".to_string(),
                qualified_name: "Package::Nested::DeepSymbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.exit_scope();
    table.exit_scope();

    // Non-recursive import should find direct child
    table.enter_scope();
    table.add_import("Package::*".to_string(), false, None, None);
    assert!(table.lookup("DirectSymbol").is_some());
    assert!(table.lookup("DeepSymbol").is_none());
    table.exit_scope();

    // BUG: Recursive import should find nested symbols but doesn't due to
    // implementation bug (searches HashMap keys instead of qualified names)
    table.enter_scope();
    table.add_import("Package::**".to_string(), true, None, None);
    assert!(table.lookup("DirectSymbol").is_some()); // Direct child still found via namespace import
    assert!(table.lookup("DeepSymbol").is_none()); // Should find but doesn't due to bug
}

#[test]
fn test_lookup_recursive_import_does_not_find_sibling() {
    let mut table = SymbolTable::new();

    // Create Package::Nested::Symbol and OtherPackage::Symbol
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Package::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();
    table.exit_scope();

    table
        .insert(
            "OtherPackage".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "OtherPackage".to_string(),
                qualified_name: "OtherPackage".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "OtherSymbol".to_string(),
            Symbol::Classifier {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "OtherSymbol".to_string(),
                qualified_name: "OtherPackage::OtherSymbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();
    table.exit_scope();

    // Import only Package recursively
    table.enter_scope();
    table.add_import("Package::**".to_string(), true, None, None);

    // Should find Package::Symbol but not OtherPackage::OtherSymbol
    assert!(table.lookup("Symbol").is_some());
    assert!(table.lookup("OtherSymbol").is_none());
}

#[test]
fn test_lookup_recursive_import_verifies_bug() {
    let mut table = SymbolTable::new();

    // Create A::B::C::D::E::Symbol
    let mut qualified = "A".to_string();
    table
        .insert(
            "A".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "A".to_string(),
                qualified_name: "A".to_string(),
            },
        )
        .unwrap();

    for (i, letter) in ["B", "C", "D", "E"].iter().enumerate() {
        table.enter_scope();
        qualified = format!("{}::{}", qualified, letter);
        table
            .insert(
                letter.to_string(),
                Symbol::Package {
                    scope_id: i + 1,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: letter.to_string(),
                    qualified_name: qualified.clone(),
                },
            )
            .unwrap();
    }

    // Add the deep symbol
    table.enter_scope();
    table
        .insert(
            "Symbol".to_string(),
            Symbol::Classifier {
                scope_id: 5,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "A::B::C::D::E::Symbol".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Exit all scopes
    for _ in 0..6 {
        table.exit_scope();
    }

    // Import recursively from A
    table.enter_scope();
    table.add_import("A::**".to_string(), true, None, None);

    // BUG: Should find the deeply nested symbol but doesn't
    let found = table.lookup("Symbol");
    assert!(found.is_none()); // Current buggy behavior
}

#[test]
fn test_lookup_recursive_import_empty_namespace() {
    let mut table = SymbolTable::new();

    // Create empty package
    table
        .insert(
            "EmptyPackage".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "EmptyPackage".to_string(),
                qualified_name: "EmptyPackage".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    // No symbols in this package
    table.exit_scope();

    // Import recursively from empty package
    table.enter_scope();
    table.add_import("EmptyPackage::**".to_string(), true, None, None);

    // Should not find anything
    assert!(table.lookup("AnySymbol").is_none());
}

// ============================================================================
// Additional edge case tests for lookup_global_mut
// ============================================================================

#[test]
fn test_lookup_global_mut_with_multiple_branches() {
    let mut table = SymbolTable::new();

    // Root with two branches
    table.enter_scope(); // Branch 1
    table
        .insert(
            "Branch1Symbol".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Branch1Symbol".to_string(),
                qualified_name: "Branch1Symbol".to_string(),
            },
        )
        .unwrap();
    table.exit_scope();

    table.enter_scope(); // Branch 2
    table
        .insert(
            "Branch2Symbol".to_string(),
            Symbol::Package {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Branch2Symbol".to_string(),
                qualified_name: "Branch2Symbol".to_string(),
            },
        )
        .unwrap();

    // From Branch 2, lookup_global_mut should find Branch 1's symbol
    assert!(table.lookup_global_mut("Branch1Symbol").is_some());
    assert!(table.lookup_global_mut("Branch2Symbol").is_some());
}

#[test]
fn test_lookup_global_mut_with_special_characters() {
    let mut table = SymbolTable::new();

    let special_names = vec![
        "name-with-dash",
        "name_with_underscore",
        "name::with::colons",
        "name.with.dots",
    ];

    for name in &special_names {
        table
            .insert(
                name.to_string(),
                Symbol::Package {
                    scope_id: 0,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: name.to_string(),
                    qualified_name: name.to_string(),
                },
            )
            .unwrap();
    }

    // All should be findable globally
    for name in special_names {
        assert!(
            table.lookup_global_mut(name).is_some(),
            "Should find {}",
            name
        );
    }
}

#[test]
fn test_lookup_global_mut_consistent_across_scope_changes() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "GlobalSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "GlobalSymbol".to_string(),
                qualified_name: "GlobalSymbol".to_string(),
            },
        )
        .unwrap();

    // Should find from root
    assert!(table.lookup_global_mut("GlobalSymbol").is_some());

    // Should find from nested scope
    table.enter_scope();
    assert!(table.lookup_global_mut("GlobalSymbol").is_some());

    table.enter_scope();
    assert!(table.lookup_global_mut("GlobalSymbol").is_some());

    // Should find after exiting
    table.exit_scope();
    assert!(table.lookup_global_mut("GlobalSymbol").is_some());

    table.exit_scope();
    assert!(table.lookup_global_mut("GlobalSymbol").is_some());
}

// ============================================================================
// Tests for build_scope_chain (implicitly tested through lookup_mut behavior)
// ============================================================================

#[test]
fn test_build_scope_chain_includes_all_ancestors() {
    let mut table = SymbolTable::new();

    // Create hierarchy: L0 -> L1 -> L2 -> L3
    for i in 0..=3 {
        if i > 0 {
            table.enter_scope();
        }
        table
            .insert(
                format!("L{}", i),
                Symbol::Package {
                    scope_id: i,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: format!("L{}", i),
                    qualified_name: format!("L{}", i),
                },
            )
            .unwrap();
    }

    // From L3, build_scope_chain should include all ancestors
    // We test this by verifying lookup_mut can find all of them
    for i in 0..=3 {
        assert!(
            table.lookup_mut(&format!("L{}", i)).is_some(),
            "Should find L{} from L3",
            i
        );
    }
}

#[test]
fn test_build_scope_chain_stops_at_root() {
    let mut table = SymbolTable::new();

    // Add symbol at root
    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    // Create deeply nested scope
    for _ in 0..10 {
        table.enter_scope();
    }

    // Should still find root (chain includes root)
    assert!(table.lookup_mut("Root").is_some());
}

#[test]
fn test_build_scope_chain_with_sibling_branches() {
    let mut table = SymbolTable::new();

    // Create first branch
    table.enter_scope();
    table
        .insert(
            "Branch1".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Branch1".to_string(),
                qualified_name: "Branch1".to_string(),
            },
        )
        .unwrap();
    table.exit_scope();

    // Create second branch
    table.enter_scope();
    table
        .insert(
            "Branch2".to_string(),
            Symbol::Package {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Branch2".to_string(),
                qualified_name: "Branch2".to_string(),
            },
        )
        .unwrap();

    // From Branch2, should NOT find Branch1 (different branch)
    assert!(table.lookup_mut("Branch2").is_some());
    assert!(table.lookup_mut("Branch1").is_none());
}

// ============================================================================
// Integration tests combining multiple features
// ============================================================================

#[test]
fn test_integration_imports_and_scopes() {
    let mut table = SymbolTable::new();

    // Create Library::Utils::Helper
    table
        .insert(
            "Library".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Library".to_string(),
                qualified_name: "Library".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Utils".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Utils".to_string(),
                qualified_name: "Library::Utils".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Helper".to_string(),
            Symbol::Classifier {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Helper".to_string(),
                qualified_name: "Library::Utils::Helper".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.exit_scope();
    table.exit_scope();

    // Create App scope with various imports
    table
        .insert(
            "App".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "App".to_string(),
                qualified_name: "App".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();

    // Non-recursive import should not find Helper
    table.add_import("Library::*".to_string(), false, None, None);
    assert!(table.lookup("Utils").is_some());
    assert!(table.lookup("Helper").is_none());

    // BUG: Recursive import should find Helper but doesn't due to implementation bug
    table.add_import("Library::**".to_string(), true, None, None);
    assert!(table.lookup("Helper").is_none()); // Current buggy behavior
}

#[test]
fn test_integration_mutation_across_lookups() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "SharedSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SharedSymbol".to_string(),
                qualified_name: "SharedSymbol".to_string(),
            },
        )
        .unwrap();

    // Add reference via lookup_mut
    {
        let symbol = table.lookup_mut("SharedSymbol").unwrap();
        symbol.add_reference(SymbolReference {
            file: "file1.sysml".to_string(),
            span: crate::core::Span::from_coords(1, 0, 1, 5),
        });
    }

    // Add another reference via lookup_global_mut
    {
        let symbol = table.lookup_global_mut("SharedSymbol").unwrap();
        symbol.add_reference(SymbolReference {
            file: "file2.sysml".to_string(),
            span: crate::core::Span::from_coords(2, 0, 2, 5),
        });
    }

    // Verify both references exist
    let symbol = table.lookup_mut("SharedSymbol").unwrap();
    assert_eq!(symbol.references().len(), 2);
}

#[test]
fn test_integration_complex_scope_hierarchy_with_imports() {
    let mut table = SymbolTable::new();

    // Create complex hierarchy:
    // Root
    //   -> PackageA
    //       -> ClassA
    //   -> PackageB (imports PackageA::*)
    //       -> ClassB

    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    table.enter_scope(); // Scope 1
    table
        .insert(
            "PackageA".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "PackageA".to_string(),
                qualified_name: "Root::PackageA".to_string(),
            },
        )
        .unwrap();

    table.enter_scope(); // Scope 2
    table
        .insert(
            "ClassA".to_string(),
            Symbol::Classifier {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "ClassA".to_string(),
                qualified_name: "Root::PackageA::ClassA".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table.exit_scope(); // Back to scope 1
    table.exit_scope(); // Back to scope 0

    table.enter_scope(); // Scope 3
    table
        .insert(
            "PackageB".to_string(),
            Symbol::Package {
                scope_id: 3,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "PackageB".to_string(),
                qualified_name: "Root::PackageB".to_string(),
            },
        )
        .unwrap();

    table.enter_scope(); // Scope 4
    table.add_import("Root::PackageA::*".to_string(), false, None, None);

    table
        .insert(
            "ClassB".to_string(),
            Symbol::Classifier {
                scope_id: 4,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "ClassB".to_string(),
                qualified_name: "Root::PackageB::ClassB".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // From PackageB::scope, should find ClassA via import
    assert!(table.lookup("ClassA").is_some());
    assert!(table.lookup("ClassB").is_some());
    assert!(table.lookup("Root").is_some());
}
