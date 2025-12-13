#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::semantic::symbol_table::{ClassifierKind, DefinitionKind, Symbol, UsageKind};

#[test]
fn test_resolve_simple_name() {
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

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("MyPackage");

    let Some(Symbol::Package { name, .. }) = result else {
        panic!("Expected Package symbol, got: {result:?}");
    };
    assert_eq!(name, "MyPackage");
}

#[test]
fn test_resolve_nonexistent() {
    let table = SymbolTable::new();
    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("DoesNotExist");

    assert!(result.is_none());
}

#[test]
fn test_resolve_qualified_name() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Child".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Child".to_string(),
                qualified_name: "Root::Child".to_string(),
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("Root::Child");

    let Some(Symbol::Package {
        scope_id: 0,
        name,
        qualified_name,
    }) = result
    else {
        panic!("Expected Package symbol, got: {result:?}");
    };
    assert_eq!(name, "Child");
    assert_eq!(qualified_name, "Root::Child");
}

#[test]
fn test_resolve_deeply_nested_qualified_name() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "A".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "A".to_string(),
                qualified_name: "A".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "B".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "B".to_string(),
                qualified_name: "A::B".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "C".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                name: "C".to_string(),
                qualified_name: "A::B::C".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("A::B::C");

    let Some(Symbol::Classifier {
        scope_id: 0,
        name,
        qualified_name,
        ..
    }) = result
    else {
        panic!("Expected Classifier symbol, got: {result:?}");
    };
    assert_eq!(name, "C");
    assert_eq!(qualified_name, "A::B::C");
}

#[test]
fn test_resolve_classifier_in_package() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Pkg".to_string(),
                qualified_name: "Pkg".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                name: "MyClass".to_string(),
                qualified_name: "Pkg::MyClass".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("Pkg::MyClass");

    let Some(Symbol::Classifier { kind, .. }) = result else {
        panic!("Expected Classifier symbol, got: {result:?}");
    };
    assert_eq!(kind, &ClassifierKind::Class);
}

#[test]
fn test_resolve_invalid_qualified_name() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("Root::DoesNotExist");

    assert!(result.is_none());
}

#[test]
fn test_resolve_partial_qualified_name() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "A".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "A".to_string(),
                qualified_name: "A".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "B".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "B".to_string(),
                qualified_name: "A::B".to_string(),
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("A::B::C");

    assert!(result.is_none());
}

#[test]
fn test_resolve_feature_symbol() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Pkg".to_string(),
                qualified_name: "Pkg".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "attr".to_string(),
            Symbol::Feature {
                scope_id: 0,
                name: "attr".to_string(),
                qualified_name: "Pkg::attr".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("Pkg::attr");

    let Some(Symbol::Feature {
        scope_id: 0,
        name,
        feature_type,
        ..
    }) = result
    else {
        panic!("Expected Feature symbol, got: {result:?}");
    };
    assert_eq!(name, "attr");
    assert_eq!(feature_type.as_deref(), Some("Integer"));
}

#[test]
fn test_resolve_definition_symbol() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "MyPart".to_string(),
            Symbol::Definition {
                scope_id: 0,
                name: "MyPart".to_string(),
                qualified_name: "MyPart".to_string(),
                kind: DefinitionKind::Part,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("MyPart");

    let Some(Symbol::Definition { name, kind, .. }) = result else {
        panic!("Expected Definition symbol, got: {result:?}");
    };
    assert_eq!(name, "MyPart");
    assert_eq!(kind, &DefinitionKind::Part);
}

#[test]
fn test_resolve_usage_symbol() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "System".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "System".to_string(),
                qualified_name: "System".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "myPort".to_string(),
            Symbol::Usage {
                scope_id: 0,
                name: "myPort".to_string(),
                qualified_name: "System::myPort".to_string(),
                kind: UsageKind::Port,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("System::myPort");

    let Some(Symbol::Usage { name, kind, .. }) = result else {
        panic!("Expected Usage symbol, got: {result:?}");
    };
    assert_eq!(name, "myPort");
    assert_eq!(kind, &UsageKind::Port);
}

#[test]
fn test_resolve_mixed_symbol_path() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                name: "MyClass".to_string(),
                qualified_name: "Root::MyClass".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: false,
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "feature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                name: "feature".to_string(),
                qualified_name: "Root::MyClass::feature".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("Root::MyClass::feature");

    let Some(Symbol::Feature {
        scope_id: 0,
        name,
        qualified_name,
        ..
    }) = result
    else {
        panic!("Expected Feature symbol, got: {result:?}");
    };
    assert_eq!(name, "feature");
    assert_eq!(qualified_name, "Root::MyClass::feature");
}

#[test]
fn test_resolve_empty_string() {
    let table = SymbolTable::new();
    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("");

    assert!(result.is_none());
}

#[test]
fn test_resolve_only_separators() {
    let table = SymbolTable::new();
    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("::");

    assert!(result.is_none());
}

#[test]
fn test_resolve_leading_separator() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("::Package");

    assert!(result.is_none());
}

#[test]
fn test_resolve_trailing_separator() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "Package".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("Package::");

    assert!(result.is_none());
}

#[test]
fn test_resolve_multiple_consecutive_separators() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "A".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "A".to_string(),
                qualified_name: "A".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "B".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "B".to_string(),
                qualified_name: "A::B".to_string(),
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("A::::B");

    assert!(result.is_none());
}

#[test]
fn test_resolve_definition_in_nested_scopes() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "OuterPkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "OuterPkg".to_string(),
                qualified_name: "OuterPkg".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "InnerPkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                name: "InnerPkg".to_string(),
                qualified_name: "OuterPkg::InnerPkg".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "requirement".to_string(),
            Symbol::Definition {
                scope_id: 0,
                name: "requirement".to_string(),
                qualified_name: "OuterPkg::InnerPkg::requirement".to_string(),
                kind: DefinitionKind::Requirement,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("OuterPkg::InnerPkg::requirement");

    let Some(Symbol::Definition {
        name,
        qualified_name,
        kind,
        scope_id: _,
    }) = result
    else {
        panic!("Expected Definition symbol, got: {result:?}");
    };
    assert_eq!(name, "requirement");
    assert_eq!(qualified_name, "OuterPkg::InnerPkg::requirement");
    assert_eq!(kind, &DefinitionKind::Requirement);
}

#[test]
fn test_resolve_abstract_classifier() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "AbstractClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                name: "AbstractClass".to_string(),
                qualified_name: "AbstractClass".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: true,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);
    let result = resolver.resolve("AbstractClass");

    let Some(Symbol::Classifier { is_abstract, .. }) = result else {
        panic!("Expected Classifier symbol, got: {result:?}");
    };
    assert!(is_abstract);
}

#[test]
fn test_resolve_different_classifier_kinds() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "MyBehavior".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                name: "MyBehavior".to_string(),
                qualified_name: "MyBehavior".to_string(),
                kind: ClassifierKind::Behavior,
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "MyFunction".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                name: "MyFunction".to_string(),
                qualified_name: "MyFunction".to_string(),
                kind: ClassifierKind::Function,
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = NameResolver::new(&table);

    let behavior_result = resolver.resolve("MyBehavior");
    let Some(Symbol::Classifier { kind, .. }) = behavior_result else {
        panic!("Expected Classifier symbol for Behavior, got: {behavior_result:?}");
    };
    assert_eq!(kind, &ClassifierKind::Behavior);

    let function_result = resolver.resolve("MyFunction");
    let Some(Symbol::Classifier { kind, .. }) = function_result else {
        panic!("Expected Classifier symbol for Function, got: {function_result:?}");
    };
    assert_eq!(kind, &ClassifierKind::Function);
}
