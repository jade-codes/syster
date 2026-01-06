#![allow(clippy::unwrap_used)]
use crate::semantic::resolver::Resolver;

use crate::semantic::{SymbolTable, symbol_table::Symbol};

#[test]
fn test_symbol_table_creation() {
    let table = SymbolTable::new();
    assert_eq!(table.current_scope_id(), 0);
}

#[test]
fn test_insert_and_lookup() {
    let mut table = SymbolTable::new();
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MyPackage".to_string(),
        qualified_name: "MyPackage".to_string(),
    };

    table
        .insert("MyPackage".to_string(), symbol.clone())
        .unwrap();
    let resolver = Resolver::new(&table);
    let found = resolver.resolve("MyPackage");
    assert!(found.is_some());
    assert_eq!(found.unwrap(), &symbol);
}

#[test]
fn test_duplicate_symbol_error() {
    let mut table = SymbolTable::new();
    let symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MyPackage".to_string(),
        qualified_name: "MyPackage".to_string(),
    };

    table
        .insert("MyPackage".to_string(), symbol.clone())
        .unwrap();
    let result = table.insert("MyPackage".to_string(), symbol);
    assert!(result.is_err());
}

#[test]
fn test_scope_hierarchy() {
    let mut table = SymbolTable::new();

    let pkg_symbol = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Root".to_string(),
        qualified_name: "Root".to_string(),
    };
    table.insert("Root".to_string(), pkg_symbol).unwrap();

    table.enter_scope();
    let class_symbol = Symbol::Classifier {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "MyClass".to_string(),
        qualified_name: "Root::MyClass".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("MyClass".to_string(), class_symbol).unwrap();

    assert!(Resolver::new(&table).resolve("Root").is_some());
    assert!(Resolver::new(&table).resolve("MyClass").is_some());

    table.exit_scope();

    assert!(Resolver::new(&table).resolve("Root").is_some());
    assert!(Resolver::new(&table).resolve("MyClass").is_none());
}

#[test]
fn test_all_symbols() {
    let mut table = SymbolTable::new();

    let pkg = Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Pkg".to_string(),
        qualified_name: "Pkg".to_string(),
    };
    table.insert("Pkg".to_string(), pkg).unwrap();

    table.enter_scope();
    let class = Symbol::Classifier {
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
        name: "Class".to_string(),
        qualified_name: "Pkg::Class".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("Class".to_string(), class).unwrap();

    let all = table.all_symbols();
    assert_eq!(all.len(), 2);
}

#[test]
fn test_multiple_nested_scopes() {
    let mut table = SymbolTable::new();

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

    table.enter_scope();
    table
        .insert(
            "Level1".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Level1".to_string(),
                qualified_name: "Level0::Level1".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Level2".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Level2".to_string(),
                qualified_name: "Level0::Level1::Level2".to_string(),
            },
        )
        .unwrap();

    assert!(Resolver::new(&table).resolve("Level0").is_some());
    assert!(Resolver::new(&table).resolve("Level1").is_some());
    assert!(Resolver::new(&table).resolve("Level2").is_some());

    table.exit_scope();
    assert!(Resolver::new(&table).resolve("Level2").is_none());
    assert!(Resolver::new(&table).resolve("Level1").is_some());

    table.exit_scope();
    assert!(Resolver::new(&table).resolve("Level1").is_none());
    assert!(Resolver::new(&table).resolve("Level0").is_some());
}

#[test]
fn test_different_symbol_types() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "MyPackage".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyPackage".to_string(),
                qualified_name: "MyPackage".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "MyFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyFeature".to_string(),
                qualified_name: "MyClass::MyFeature".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    table
        .insert(
            "MyDef".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyDef".to_string(),
                qualified_name: "MyDef".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "MyUsage".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                usage_type: None,
                semantic_role: None,
                references: Vec::new(),
                name: "MyUsage".to_string(),
                qualified_name: "MyUsage".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    assert!(Resolver::new(&table).resolve("MyPackage").is_some());
    assert!(Resolver::new(&table).resolve("MyClass").is_some());
    assert!(Resolver::new(&table).resolve("MyFeature").is_some());
    assert!(Resolver::new(&table).resolve("MyDef").is_some());
    assert!(Resolver::new(&table).resolve("MyUsage").is_some());

    let all = table.all_symbols();
    assert_eq!(all.len(), 5);
}

#[test]
fn test_exit_scope_at_root() {
    let mut table = SymbolTable::new();
    let initial_scope = table.current_scope_id();

    table.exit_scope();

    assert_eq!(table.current_scope_id(), initial_scope);
}

#[test]
fn test_lookup_nonexistent_symbol() {
    let table = SymbolTable::new();
    assert!(Resolver::new(&table).resolve("DoesNotExist").is_none());
}

#[test]
fn test_remove_symbols_from_file() {
    let mut table = SymbolTable::new();

    // Add symbols from file1
    table.set_current_file(Some("file1.sysml".to_string()));
    table
        .insert(
            "Pkg1".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: Some("file1.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "Pkg1".to_string(),
                qualified_name: "Pkg1".to_string(),
            },
        )
        .unwrap();

    // Add symbols from file2
    table.set_current_file(Some("file2.sysml".to_string()));
    table
        .insert(
            "Pkg2".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: Some("file2.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "Pkg2".to_string(),
                qualified_name: "Pkg2".to_string(),
            },
        )
        .unwrap();

    // Add another symbol from file1
    table.set_current_file(Some("file1.sysml".to_string()));
    table.enter_scope();
    table
        .insert(
            "Class1".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: Some("file1.sysml".to_string()),
                span: None,
                references: Vec::new(),
                name: "Class1".to_string(),
                qualified_name: "Pkg1::Class1".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Verify all symbols exist
    assert!(Resolver::new(&table).resolve("Pkg1").is_some());
    assert!(Resolver::new(&table).resolve("Pkg2").is_some());
    assert!(Resolver::new(&table).resolve("Class1").is_some());

    // Remove file1 symbols
    let removed = table.remove_symbols_from_file("file1.sysml");

    // Should have removed 2 symbols (Pkg1 and Class1)
    assert_eq!(removed, 2);

    // Verify file1 symbols are gone
    assert!(Resolver::new(&table).resolve("Pkg1").is_none());
    assert!(Resolver::new(&table).resolve("Class1").is_none());

    // Verify file2 symbols remain
    assert!(Resolver::new(&table).resolve("Pkg2").is_some());
}
