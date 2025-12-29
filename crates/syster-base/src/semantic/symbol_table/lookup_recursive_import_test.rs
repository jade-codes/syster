#![allow(clippy::unwrap_used)]

use super::*;

/// Test finding a symbol in an immediately nested namespace
/// The function looks for HashMap keys that:
/// - Start with "namespace::"
/// - End with "::name"
///
/// Example: Looking for "Class1" in "PackageA" namespace
/// Will find key "PackageA::Class1"
#[test]
fn test_lookup_in_immediate_nested_namespace() {
    let mut table = SymbolTable::new();

    // Insert symbol with qualified name as HashMap key
    table
        .insert(
            "PackageA::Class1".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Class1".to_string(),
                qualified_name: "PackageA::Class1".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("Class1", "PackageA");

    assert!(result.is_some());
    assert_eq!(result.unwrap().qualified_name(), "PackageA::Class1");
}

/// Test finding a symbol in a deeply nested namespace
/// Looking for "DeepClass" in "Root" namespace
/// Will find key "Root::SubA::SubB::DeepClass"
#[test]
fn test_lookup_in_deeply_nested_namespace() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root::SubA::SubB::DeepClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "DeepClass".to_string(),
                qualified_name: "Root::SubA::SubB::DeepClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("DeepClass", "Root");

    assert!(result.is_some());
    assert_eq!(
        result.unwrap().qualified_name(),
        "Root::SubA::SubB::DeepClass"
    );
}

/// Test that returns None when symbol not found in namespace
/// Looking for "Class1" in "PackageX" but key is "PackageY::Class1"
#[test]
fn test_symbol_not_found_in_namespace() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "PackageY::Class1".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Class1".to_string(),
                qualified_name: "PackageY::Class1".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("Class1", "PackageX");

    assert!(result.is_none());
}

/// Test that returns None when symbol exists but not in specified namespace
#[test]
fn test_symbol_exists_but_wrong_namespace() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "WrongPackage::MySymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MySymbol".to_string(),
                qualified_name: "WrongPackage::MySymbol".to_string(),
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("MySymbol", "CorrectPackage");

    assert!(result.is_none());
}

/// Test finding correct symbol when multiple matches exist across different namespaces
#[test]
fn test_multiple_matches_different_namespaces() {
    let mut table = SymbolTable::new();

    // Insert "MyClass" in PackageA::Sub
    table
        .insert(
            "PackageA::Sub::MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "PackageA::Sub::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Insert "MyClass" in PackageB::Sub
    table
        .insert(
            "PackageB::Sub::MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "PackageB::Sub::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Look for MyClass in PackageA namespace
    let result_a = table.lookup_recursive_import("MyClass", "PackageA");
    assert!(result_a.is_some());
    assert_eq!(
        result_a.unwrap().qualified_name(),
        "PackageA::Sub::MyClass"
    );

    // Look for MyClass in PackageB namespace
    let result_b = table.lookup_recursive_import("MyClass", "PackageB");
    assert!(result_b.is_some());
    assert_eq!(
        result_b.unwrap().qualified_name(),
        "PackageB::Sub::MyClass"
    );
}

/// Test with different symbol types to ensure the function works with all variants
#[test]
fn test_with_different_symbol_types() {
    let mut table = SymbolTable::new();

    // Insert Package symbol
    table
        .insert(
            "Root::Nested::SubPackage".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SubPackage".to_string(),
                qualified_name: "Root::Nested::SubPackage".to_string(),
            },
        )
        .unwrap();

    // Insert Classifier symbol
    table
        .insert(
            "Root::Module::MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "Root::Module::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Insert Feature symbol
    table
        .insert(
            "Root::Class::MyFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyFeature".to_string(),
                qualified_name: "Root::Class::MyFeature".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    // Insert Definition symbol
    table
        .insert(
            "Root::Defs::MyDef".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyDef".to_string(),
                qualified_name: "Root::Defs::MyDef".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Insert Usage symbol
    table
        .insert(
            "Root::Usage::MyUsage".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                usage_type: None,
                semantic_role: None,
                references: Vec::new(),
                name: "MyUsage".to_string(),
                qualified_name: "Root::Usage::MyUsage".to_string(),
                kind: "Part".to_string(),
            },
        )
        .unwrap();

    // Test finding each symbol type
    let pkg_result = table.lookup_recursive_import("SubPackage", "Root");
    assert!(pkg_result.is_some());
    assert!(matches!(pkg_result.unwrap(), Symbol::Package { .. }));

    let class_result = table.lookup_recursive_import("MyClass", "Root");
    assert!(class_result.is_some());
    assert!(matches!(class_result.unwrap(), Symbol::Classifier { .. }));

    let feature_result = table.lookup_recursive_import("MyFeature", "Root");
    assert!(feature_result.is_some());
    assert!(matches!(feature_result.unwrap(), Symbol::Feature { .. }));

    let def_result = table.lookup_recursive_import("MyDef", "Root");
    assert!(def_result.is_some());
    assert!(matches!(def_result.unwrap(), Symbol::Definition { .. }));

    let usage_result = table.lookup_recursive_import("MyUsage", "Root");
    assert!(usage_result.is_some());
    assert!(matches!(usage_result.unwrap(), Symbol::Usage { .. }));
}

/// Test finding symbol at multiple nested levels under the same namespace
/// Should return the first match found
#[test]
fn test_symbol_at_multiple_levels_same_namespace() {
    let mut table = SymbolTable::new();

    // Insert "Name" at Root::Level1
    table
        .insert(
            "Root::Level1::Name".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Name".to_string(),
                qualified_name: "Root::Level1::Name".to_string(),
            },
        )
        .unwrap();

    // Insert another "Name" at Root::Level2::Sub
    table
        .insert(
            "Root::Level2::Sub::Name".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Name".to_string(),
                qualified_name: "Root::Level2::Sub::Name".to_string(),
            },
        )
        .unwrap();

    // lookup_recursive_import should find one of them
    let result = table.lookup_recursive_import("Name", "Root");
    assert!(result.is_some());

    let found_qname = result.unwrap().qualified_name();
    // Should be one of the two
    assert!(
        found_qname == "Root::Level1::Name" || found_qname == "Root::Level2::Sub::Name"
    );
}

/// Test that function returns None when looking for non-existent symbol
#[test]
fn test_nonexistent_symbol() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root::ExistingSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "ExistingSymbol".to_string(),
                qualified_name: "Root::ExistingSymbol".to_string(),
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("NonExistent", "Root");

    assert!(result.is_none());
}

/// Test with namespace that has no symbols
#[test]
fn test_empty_namespace() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "PackageA::Symbol1".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol1".to_string(),
                qualified_name: "PackageA::Symbol1".to_string(),
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("Symbol1", "EmptyNamespace");

    assert!(result.is_none());
}

/// Test that symbols in different scopes are all searched
#[test]
fn test_searches_across_all_scopes() {
    let mut table = SymbolTable::new();

    // Insert symbol in root scope (scope 0)
    table
        .insert(
            "NS::Sub1::Symbol1".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol1".to_string(),
                qualified_name: "NS::Sub1::Symbol1".to_string(),
            },
        )
        .unwrap();

    // Enter a new scope and insert another symbol
    table.enter_scope();
    table
        .insert(
            "NS::Sub2::Symbol2".to_string(),
            Symbol::Classifier {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol2".to_string(),
                qualified_name: "NS::Sub2::Symbol2".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // lookup_recursive_import should find symbols from both scopes
    let result1 = table.lookup_recursive_import("Symbol1", "NS");
    assert!(result1.is_some());
    assert_eq!(result1.unwrap().qualified_name(), "NS::Sub1::Symbol1");

    let result2 = table.lookup_recursive_import("Symbol2", "NS");
    assert!(result2.is_some());
    assert_eq!(result2.unwrap().qualified_name(), "NS::Sub2::Symbol2");
}

/// Test with symbol name containing colons
/// Ensures that the matching works with special characters
#[test]
fn test_symbol_name_with_colons() {
    let mut table = SymbolTable::new();

    // Insert symbol with special characters in name
    table
        .insert(
            "Root::Package::Special::Name".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Special::Name".to_string(),
                qualified_name: "Root::Package::Special::Name".to_string(),
            },
        )
        .unwrap();

    // Should find by the end of qualified name
    let result = table.lookup_recursive_import("Special::Name", "Root");
    assert!(result.is_some());
    assert_eq!(
        result.unwrap().qualified_name(),
        "Root::Package::Special::Name"
    );
}

/// Test that prefix and suffix matching is correct (not partial match)
/// Key "Package::MyClass" should not match when looking for "Class" in "Package"
/// because "MyClass" doesn't match "Class"
#[test]
fn test_exact_suffix_matching() {
    let mut table = SymbolTable::new();

    // Insert symbol "Package::MyClass"
    table
        .insert(
            "Package::MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "Package::MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Try to find "Class" (partial name) - should not match "MyClass"
    let result = table.lookup_recursive_import("Class", "Package");
    assert!(result.is_none());

    // Try to find "MyClass" - should match
    let result2 = table.lookup_recursive_import("MyClass", "Package");
    assert!(result2.is_some());
}

/// Test with very deeply nested namespace (5+ levels)
#[test]
fn test_very_deep_nesting() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "L1::L2::L3::L4::L5::L6::DeepSymbol".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "DeepSymbol".to_string(),
                qualified_name: "L1::L2::L3::L4::L5::L6::DeepSymbol".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("DeepSymbol", "L1");
    assert!(result.is_some());
    assert_eq!(
        result.unwrap().qualified_name(),
        "L1::L2::L3::L4::L5::L6::DeepSymbol"
    );
}

/// Test with symbol at the boundary: namespace immediately followed by symbol
/// Key "NS::Symbol" when looking for "Symbol" in "NS"
#[test]
fn test_immediate_child_no_intermediate_levels() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Namespace::DirectChild".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "DirectChild".to_string(),
                qualified_name: "Namespace::DirectChild".to_string(),
            },
        )
        .unwrap();

    let result = table.lookup_recursive_import("DirectChild", "Namespace");
    assert!(result.is_some());
    assert_eq!(
        result.unwrap().qualified_name(),
        "Namespace::DirectChild"
    );
}

/// Test that the function does not match if namespace appears elsewhere in the key
/// Key "Other::NS::Symbol" should not match when looking for "Symbol" in "NS"
/// because it doesn't start with "NS::"
#[test]
fn test_namespace_must_be_prefix() {
    let mut table = SymbolTable::new();

    // Insert symbol where namespace appears but not as prefix
    table
        .insert(
            "Other::NS::Symbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Symbol".to_string(),
                qualified_name: "Other::NS::Symbol".to_string(),
            },
        )
        .unwrap();

    // Should not find it because key doesn't start with "NS::"
    let result = table.lookup_recursive_import("Symbol", "NS");
    assert!(result.is_none());
}

/// Test that the function does not match if name appears elsewhere in the key
/// Key "NS::SymbolX" should not match when looking for "Symbol" in "NS"
/// because it doesn't end with "::Symbol"
#[test]
fn test_name_must_be_suffix() {
    let mut table = SymbolTable::new();

    // Insert symbol where name appears but not as suffix
    table
        .insert(
            "NS::SymbolX".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SymbolX".to_string(),
                qualified_name: "NS::SymbolX".to_string(),
            },
        )
        .unwrap();

    // Should not find it because key doesn't end with "::Symbol"
    let result = table.lookup_recursive_import("Symbol", "NS");
    assert!(result.is_none());
}

/// Test with empty string as name or namespace
#[test]
fn test_empty_strings() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "NS::Name".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Name".to_string(),
                qualified_name: "NS::Name".to_string(),
            },
        )
        .unwrap();

    // Empty name should not match
    let result1 = table.lookup_recursive_import("", "NS");
    assert!(result1.is_none());

    // Empty namespace should not match
    let result2 = table.lookup_recursive_import("Name", "");
    assert!(result2.is_none());
}
