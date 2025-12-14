#![allow(clippy::unwrap_used)]

use super::*;

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
        name: "MyPackage".to_string(),
        qualified_name: "MyPackage".to_string(),
    };

    table
        .insert("MyPackage".to_string(), symbol.clone())
        .unwrap();
    let found = table.lookup("MyPackage");
    assert!(found.is_some());
    assert_eq!(found.unwrap(), &symbol);
}

#[test]
fn test_duplicate_symbol_error() {
    let mut table = SymbolTable::new();
    let symbol = Symbol::Package {
        scope_id: 0,
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
        name: "Root".to_string(),
        qualified_name: "Root".to_string(),
    };
    table.insert("Root".to_string(), pkg_symbol).unwrap();

    table.enter_scope();
    let class_symbol = Symbol::Classifier {
        scope_id: 0,
        name: "MyClass".to_string(),
        qualified_name: "Root::MyClass".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("MyClass".to_string(), class_symbol).unwrap();

    assert!(table.lookup("Root").is_some());
    assert!(table.lookup("MyClass").is_some());

    table.exit_scope();

    assert!(table.lookup("Root").is_some());
    assert!(table.lookup("MyClass").is_none());
}

#[test]
fn test_local_lookup() {
    let mut table = SymbolTable::new();

    let root_symbol = Symbol::Package {
        scope_id: 0,
        name: "Root".to_string(),
        qualified_name: "Root".to_string(),
    };
    table.insert("Root".to_string(), root_symbol).unwrap();

    table.enter_scope();
    let nested_symbol = Symbol::Classifier {
        scope_id: 0,
        name: "Nested".to_string(),
        qualified_name: "Root::Nested".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("Nested".to_string(), nested_symbol).unwrap();

    assert!(table.lookup_local("Nested").is_some());
    assert!(table.lookup_local("Root").is_none());

    assert!(table.lookup("Root").is_some());
}

#[test]
fn test_symbols_in_scope() {
    let mut table = SymbolTable::new();

    let pkg = Symbol::Package {
        scope_id: 0,
        name: "Pkg".to_string(),
        qualified_name: "Pkg".to_string(),
    };
    table.insert("Pkg".to_string(), pkg).unwrap();

    let root_symbols = table.symbols_in_scope(0).unwrap();
    assert_eq!(root_symbols.len(), 1);
    assert!(root_symbols.contains_key("Pkg"));

    table.enter_scope();
    let class = Symbol::Classifier {
        scope_id: 0,
        name: "Class".to_string(),
        qualified_name: "Pkg::Class".to_string(),
        kind: "Class".to_string(),
        is_abstract: false,
    };
    table.insert("Class".to_string(), class).unwrap();

    let nested_symbols = table.symbols_in_scope(1).unwrap();
    assert_eq!(nested_symbols.len(), 1);
    assert!(nested_symbols.contains_key("Class"));
}

#[test]
fn test_all_symbols() {
    let mut table = SymbolTable::new();

    let pkg = Symbol::Package {
        scope_id: 0,
        name: "Pkg".to_string(),
        qualified_name: "Pkg".to_string(),
    };
    table.insert("Pkg".to_string(), pkg).unwrap();

    table.enter_scope();
    let class = Symbol::Classifier {
        scope_id: 0,
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
                name: "Level2".to_string(),
                qualified_name: "Level0::Level1::Level2".to_string(),
            },
        )
        .unwrap();

    assert!(table.lookup("Level0").is_some());
    assert!(table.lookup("Level1").is_some());
    assert!(table.lookup("Level2").is_some());

    table.exit_scope();
    assert!(table.lookup("Level2").is_none());
    assert!(table.lookup("Level1").is_some());

    table.exit_scope();
    assert!(table.lookup("Level1").is_none());
    assert!(table.lookup("Level0").is_some());
}

#[test]
fn test_different_symbol_types() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "MyPackage".to_string(),
            Symbol::Package {
                scope_id: 0,
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
                name: "MyDef".to_string(),
                qualified_name: "MyDef".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "MyUsage".to_string(),
            Symbol::Usage {
                scope_id: 0,
                name: "MyUsage".to_string(),
                qualified_name: "MyUsage".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    assert!(table.lookup("MyPackage").is_some());
    assert!(table.lookup("MyClass").is_some());
    assert!(table.lookup("MyFeature").is_some());
    assert!(table.lookup("MyDef").is_some());
    assert!(table.lookup("MyUsage").is_some());

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
    assert!(table.lookup("DoesNotExist").is_none());
    assert!(table.lookup_local("DoesNotExist").is_none());
}

#[test]
fn test_symbols_in_invalid_scope() {
    let table = SymbolTable::new();
    assert!(table.symbols_in_scope(999).is_none());
}
