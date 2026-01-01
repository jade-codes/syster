#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use crate::{
    semantic::{
        Resolver, SymbolTable, extract_imports, is_wildcard_import, parse_import_path,
        symbol_table::Symbol,
    },
    syntax::sysml::ast::{Element, Import, SysMLFile},
};

#[test]
fn test_resolve_simple_name() {
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

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("MyPackage");

    let Some(Symbol::Package { name, .. }) = result else {
        panic!("Expected Package symbol, got: {result:?}");
    };
    assert_eq!(name, "MyPackage");
}

#[test]
fn test_resolve_nonexistent() {
    let table = SymbolTable::new();
    let resolver = Resolver::new(&table);
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
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Child".to_string(),
                qualified_name: "Root::Child".to_string(),
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("Root::Child");

    let Some(Symbol::Package {
        scope_id: 0,
        source_file: None,
        span: None,
        name,
        qualified_name,
        references: _,
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
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "C".to_string(),
                qualified_name: "A::B::C".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("A::B::C");

    let Some(Symbol::Classifier {
        scope_id: 0,
        source_file: None,
        span: None,
        name,
        qualified_name,
        references: _,
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
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "Pkg::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("Pkg::MyClass");

    let Some(Symbol::Classifier { kind, .. }) = result else {
        panic!("Expected Classifier symbol, got: {result:?}");
    };
    assert_eq!(kind, "Class");
}

#[test]
fn test_resolve_invalid_qualified_name() {
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

    let resolver = Resolver::new(&table);
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
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "B".to_string(),
                qualified_name: "A::B".to_string(),
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
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
            "attr".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "attr".to_string(),
                qualified_name: "Pkg::attr".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("Pkg::attr");

    let Some(Symbol::Feature {
        scope_id: 0,
        source_file: None,
        span: None,
        name,
        feature_type,
        references: _,
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyPart".to_string(),
                qualified_name: "MyPart".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("MyPart");

    let Some(Symbol::Definition { name, kind, .. }) = result else {
        panic!("Expected Definition symbol, got: {result:?}");
    };
    assert_eq!(name, "MyPart");
    assert_eq!(kind, "Part");
}

#[test]
fn test_resolve_usage_symbol() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "System".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myPort".to_string(),
                qualified_name: "System::myPort".to_string(),
                kind: "Port".to_string(),
                semantic_role: None,
                usage_type: None,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("System::myPort");

    let Some(Symbol::Usage { name, kind, .. }) = result else {
        panic!("Expected Usage symbol, got: {result:?}");
    };
    assert_eq!(name, "myPort");
    assert_eq!(kind, "Port");
}

#[test]
fn test_resolve_mixed_symbol_path() {
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

    table.enter_scope();
    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "Root::MyClass".to_string(),
                kind: "Class".to_string(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature".to_string(),
                qualified_name: "Root::MyClass::feature".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("Root::MyClass::feature");

    let Some(Symbol::Feature {
        scope_id: 0,
        source_file: None,
        span: None,
        name,
        qualified_name,
        references: _,
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
    let resolver = Resolver::new(&table);
    let result = resolver.resolve("");

    assert!(result.is_none());
}

#[test]
fn test_resolve_only_separators() {
    let table = SymbolTable::new();
    let resolver = Resolver::new(&table);
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Package".to_string(),
                qualified_name: "Package".to_string(),
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
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
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "B".to_string(),
                qualified_name: "A::B".to_string(),
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
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
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "requirement".to_string(),
                qualified_name: "OuterPkg::InnerPkg::requirement".to_string(),
                kind: "Requirement".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
    let result = resolver.resolve("OuterPkg::InnerPkg::requirement");

    let Some(Symbol::Definition {
        name,
        qualified_name,
        kind,
        ..
    }) = result
    else {
        panic!("Expected Definition symbol, got: {result:?}");
    };
    assert_eq!(name, "requirement");
    assert_eq!(qualified_name, "OuterPkg::InnerPkg::requirement");
    assert_eq!(kind, "Requirement");
}

#[test]
fn test_resolve_abstract_classifier() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "AbstractClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "AbstractClass".to_string(),
                qualified_name: "AbstractClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: true,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);
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
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyBehavior".to_string(),
                qualified_name: "MyBehavior".to_string(),
                kind: "Behavior".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "MyFunction".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyFunction".to_string(),
                qualified_name: "MyFunction".to_string(),
                kind: "Function".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);

    let behavior_result = resolver.resolve("MyBehavior");
    let Some(Symbol::Classifier { kind, .. }) = behavior_result else {
        panic!("Expected Classifier symbol for Behavior, got: {behavior_result:?}");
    };
    assert_eq!(kind, "Behavior");

    let function_result = resolver.resolve("MyFunction");
    let Some(Symbol::Classifier { kind, .. }) = function_result else {
        panic!("Expected Classifier symbol for Function, got: {function_result:?}");
    };
    assert_eq!(kind, "Function");
}

#[test]
fn test_resolve_import_specific_member() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Base".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Base".to_string(),
                qualified_name: "Base".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "Base::Vehicle".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Vehicle".to_string(),
                qualified_name: "Base::Vehicle".to_string(),
                kind: "PartDef".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);

    // Specific import
    let imports = resolver.resolve_import("Base::Vehicle");
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "Base::Vehicle");
}

#[test]
fn test_resolve_import_wildcard() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Base".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Base".to_string(),
                qualified_name: "Base".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "Base::Vehicle".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Vehicle".to_string(),
                qualified_name: "Base::Vehicle".to_string(),
                kind: "PartDef".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "Base::Engine".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Engine".to_string(),
                qualified_name: "Base::Engine".to_string(),
                kind: "PartDef".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Nested symbol - should not be included
    table
        .insert(
            "Base::Vehicle::Wheel".to_string(),
            Symbol::Classifier {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Wheel".to_string(),
                qualified_name: "Base::Vehicle::Wheel".to_string(),
                kind: "PartDef".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);

    // Wildcard import
    let mut imports = resolver.resolve_import("Base::*");
    imports.sort(); // For deterministic ordering

    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0], "Base::Engine");
    assert_eq!(imports[1], "Base::Vehicle");
    // Wheel should not be included (nested)
}

#[test]
fn test_resolve_import_bare_wildcard() {
    let mut table = SymbolTable::new();

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

    table
        .insert(
            "PackageB".to_string(),
            Symbol::Package {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "PackageB".to_string(),
                qualified_name: "PackageB".to_string(),
            },
        )
        .unwrap();

    // Nested - should not be included
    table
        .insert(
            "PackageA::Nested".to_string(),
            Symbol::Package {
                scope_id: 2,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Nested".to_string(),
                qualified_name: "PackageA::Nested".to_string(),
            },
        )
        .unwrap();

    let resolver = Resolver::new(&table);

    // Bare wildcard
    let mut imports = resolver.resolve_import("*");
    imports.sort();

    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0], "PackageA");
    assert_eq!(imports[1], "PackageB");
}

#[test]
fn test_resolve_import_nonexistent() {
    let table = SymbolTable::new();
    let resolver = Resolver::new(&table);

    // Specific import that doesn't exist
    let imports = resolver.resolve_import("DoesNotExist::Member");
    assert_eq!(imports.len(), 0);

    // Wildcard import that doesn't match anything
    let imports = resolver.resolve_import("DoesNotExist::*");
    assert_eq!(imports.len(), 0);
}

#[test]
fn test_extract_no_imports() {
    // TDD: File with no imports returns empty vec
    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 0);
}

#[test]
fn test_extract_single_import() {
    // TDD: File with one import statement
    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Import(Import {
            path: "Base::Vehicle".to_string(),
            path_span: None,
            is_recursive: false,
            span: None,
        })],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "Base::Vehicle");
}

#[test]
fn test_extract_multiple_imports() {
    // TDD: File with multiple import statements
    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![
            Element::Import(Import {
                path: "Base::Vehicle".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Import(Import {
                path: "Systems::Engine".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Import(Import {
                path: "Utils::*".to_string(),
                path_span: None,
                is_recursive: true,
                span: None,
            }),
        ],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 3);
    assert!(imports.contains(&"Base::Vehicle".to_string()));
    assert!(imports.contains(&"Systems::Engine".to_string()));
    assert!(imports.contains(&"Utils::*".to_string()));
}

#[test]
fn test_extract_recursive_imports() {
    // TDD: Wildcard imports should be captured
    let file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Import(Import {
            path: "SysML::*".to_string(),
            path_span: None,
            is_recursive: true,
            span: None,
        })],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "SysML::*");
}

#[test]
fn test_extract_imports_mixed_elements() {
    // TDD: Should extract imports even with other elements present
    use crate::syntax::sysml::ast::types::NamespaceDeclaration;

    let file = SysMLFile {
        namespaces: vec![],
        namespace: Some(NamespaceDeclaration {
            name: "MyPackage".to_string(),
            span: None,
        }),
        elements: vec![
            Element::Import(Import {
                path: "Base::Vehicle".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
            Element::Comment(crate::syntax::sysml::ast::Comment {
                content: "Some comment".to_string(),
                span: None,
            }),
            Element::Import(Import {
                path: "Systems::Engine".to_string(),
                path_span: None,
                is_recursive: false,
                span: None,
            }),
        ],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 2);
}

#[test]
fn test_parse_namespace_path() {
    // TDD: Parse import path into components
    let path = "Base::Components::Vehicle";
    let parts = parse_import_path(path);

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "Base");
    assert_eq!(parts[1], "Components");
    assert_eq!(parts[2], "Vehicle");
}

#[test]
fn test_parse_wildcard_import() {
    // TDD: Wildcard imports parse correctly
    let path = "SysML::*";
    let parts = parse_import_path(path);

    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "SysML");
    assert_eq!(parts[1], "*");
}

#[test]
fn test_is_wildcard_import() {
    // TDD: Detect wildcard imports
    assert!(is_wildcard_import("SysML::*"));
    assert!(is_wildcard_import("Base::Components::*"));
    assert!(!is_wildcard_import("Base::Vehicle"));
    assert!(!is_wildcard_import("SysML::Items"));
}
