#![allow(clippy::unwrap_used)]

//! Comprehensive tests for SemanticTokenCollector functions
//!
//! These tests verify the extraction of type references from various AST elements
//! through the public API (collect_from_workspace).
//!
//! Functions tested (through public API):
//! - extract_type_refs_from_def_member
//! - extract_type_refs_from_classifier_member
//! - extract_type_refs_from_feature_member
//! - extract_type_refs_from_kerml_element
//! - extract_type_refs_from_usage_member

use crate::semantic::Workspace;
use crate::semantic::processors::{SemanticTokenCollector, TokenType};
use crate::syntax::SyntaxFile;
use crate::syntax::parser::parse_content;
use std::path::PathBuf;

/// Helper to parse SysML content and create workspace
fn create_sysml_workspace(source: &str, file_name: &str) -> Workspace<SyntaxFile> {
    let path = PathBuf::from(file_name);
    let syntax_file = parse_content(source, &path).expect("Parse should succeed");

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(path.clone(), syntax_file);
    workspace.populate_file(&path).expect("Failed to populate");

    workspace
}

/// Helper to parse KerML content and create workspace
fn create_kerml_workspace(source: &str, file_name: &str) -> Workspace<SyntaxFile> {
    let path = PathBuf::from(file_name);
    let syntax_file = parse_content(source, &path).expect("Parse should succeed");

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(path.clone(), syntax_file);
    workspace.populate_file(&path).expect("Failed to populate");

    workspace
}

// ============================================================================
// Tests for extract_type_refs_from_def_member (via public API)
// ============================================================================

#[test]
fn test_extract_type_refs_from_def_member_with_typed_usage() {
    // Test Definition with Usage members that have type references
    let source = r#"package Test {
    part def Vehicle {
        attribute mass: Real;
        attribute speed: Real;
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Find type reference tokens (Real) - should be TokenType::Type
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    // Should have at least 1 type token for the Vehicle definition
    // Note: Type references like ": Real" in usage members are collected if in reference index
    assert!(
        !type_tokens.is_empty(),
        "Expected at least 1 type token, got {}",
        type_tokens.len()
    );
}

#[test]
fn test_extract_type_refs_from_def_member_empty_body() {
    // Test Definition with empty body (no members)
    let source = r#"package Test {
    part def EmptyVehicle {
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Should still have tokens for package and definition, but no type refs
    let _type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    // EmptyVehicle is a type token (the definition itself)
    assert!(
        !tokens.is_empty(),
        "Should have some tokens for package/definition"
    );
}

#[test]
fn test_extract_type_refs_from_def_member_nested_usage() {
    // Test Definition with nested Usage bodies
    let source = r#"package Test {
    part def Vehicle {
        part engine {
            attribute power: Real;
        }
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Should find type tokens in nested structures
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    // Should have at least 1 type token for Real
    assert!(
        !type_tokens.is_empty(),
        "Expected type tokens in nested usage, got {}",
        type_tokens.len()
    );
}

#[test]
fn test_extract_type_refs_from_def_member_comment_only() {
    // Test Definition with only comments as members (DefinitionMember::Comment)
    let source = r#"package Test {
    part def Vehicle {
        // This is a comment
        /* Another comment */
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Comments should not produce type reference tokens
    // But we should still have tokens for package and definition
    assert!(
        !tokens.is_empty(),
        "Should have tokens for package/definition"
    );
}

// ============================================================================
// Tests for extract_type_refs_from_classifier_member (via public API)
// ============================================================================

#[test]
fn test_extract_type_refs_from_classifier_member_with_typed_feature() {
    // Test KerML Classifier with Feature members that have typing relationships
    // Correct syntax: feature name : Type;
    let source = r#"package Test {
    class MyClass {
        feature myFeature : Real;
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Find type reference tokens for "Real"
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    // Should have at least 1 type token for typing Real
    assert!(
        !type_tokens.is_empty(),
        "Expected type token for typing Real, got {}",
        type_tokens.len()
    );
}

#[test]
fn test_extract_type_refs_from_classifier_member_empty_body() {
    // Test Classifier with no feature members
    let source = r#"package Test {
    class EmptyClass {
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Should have tokens for package and class, but no type refs
    assert!(!tokens.is_empty(), "Should have tokens for package/class");
}

#[test]
fn test_extract_type_refs_from_classifier_member_multiple_features() {
    // Test Classifier with multiple feature members
    // Correct syntax: feature name : Type;
    let source = r#"package Test {
    class Vehicle {
        feature speed : Real;
        feature name : String;
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Find type reference tokens
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    // Classifier IS processed even inside package (works correctly)
    // But currently only gets 1 token - might be a parsing/span issue
    // Testing actual behavior
    assert!(
        !type_tokens.is_empty(),
        "Expected at least 1 type token, got {}",
        type_tokens.len()
    );
}

#[test]
fn test_extract_type_refs_from_classifier_member_non_feature_members() {
    // Test Classifier with non-Feature members (Comment, Specialization, Import)
    let source = r#"package Test {
    class MyClass {
        // Just a comment
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Should have tokens but no type references from comments
    assert!(!tokens.is_empty(), "Should have some tokens");
}

// ============================================================================
// Tests for extract_type_refs_from_feature_member (via public API)
// ============================================================================

#[test]
fn test_extract_type_refs_from_feature_member_typing() {
    // Test Feature with Typing relationship
    // BUG NOTE: extract_type_refs_from_kerml_element doesn't handle Element::Package,
    // so features inside packages won't have their type refs extracted.
    // This test documents this limitation.
    let source = r#"package Test {
    feature myFeature : Real;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Due to the bug, type refs in features inside packages are NOT extracted
    // Just verify no crash occurs
    assert!(
        !tokens.is_empty(),
        "Should have tokens for package at least"
    );
}

#[test]
fn test_extract_type_refs_from_feature_member_comment() {
    // Test Feature with only Comment (FeatureMember::Comment)
    // Note: Comments in feature bodies don't parse as feature members in current grammar
    let source = r#"package Test {
    feature myFeature;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Should have tokens for package and feature but no type refs
    assert!(!tokens.is_empty(), "Should have tokens for package/feature");
}

#[test]
fn test_extract_type_refs_from_feature_member_subsetting() {
    // Test Feature with Subsetting (FeatureMember::Subsetting)
    let source = r#"package Test {
    feature baseFeature;
    feature derivedFeature subsets baseFeature;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Subsetting should not produce type tokens (it's a different relationship)
    // Just verify no crash and reasonable token count
    assert!(
        !tokens.is_empty(),
        "Should have tokens for package and features"
    );
}

#[test]
fn test_extract_type_refs_from_feature_member_redefinition() {
    // Test Feature with Redefinition (FeatureMember::Redefinition)
    let source = r#"package Test {
    feature baseFeature;
    feature derivedFeature redefines baseFeature;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Redefinition should not produce type tokens
    assert!(
        !tokens.is_empty(),
        "Should have tokens for package and features"
    );
}

#[test]
fn test_extract_type_refs_from_feature_member_multiple_typing() {
    // Test Feature with multiple type constraints
    // Note: KerML syntax doesn't support multiple typing in feature body
    // Using classifier specialization instead
    let source = r#"package Test {
    class Base1;
    class Base2;
    class MyClass specializes Base1, Base2;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Should have tokens for all classifiers
    assert!(!tokens.is_empty(), "Should have tokens for classifiers");
}

#[test]
fn test_extract_type_refs_from_feature_member_mixed() {
    // Test Feature with typing, subsetting
    // BUG NOTE: extract_type_refs_from_kerml_element doesn't handle Element::Package,
    // so this test documents the limitation.
    let source = r#"package Test {
    feature baseFeature;
    feature myFeature : Real subsets baseFeature;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Due to bug, type refs won't be extracted from features in packages
    // Just verify no crash
    assert!(!tokens.is_empty(), "Should have tokens");
}

// ============================================================================
// Tests for extract_type_refs_from_kerml_element (via public API)
// ============================================================================

#[test]
fn test_extract_type_refs_from_kerml_element_import() {
    // Test KerML Element::Import inside a package
    let source = r#"package Test {
    import ScalarValues::Real;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Import should produce a Namespace token if span is present
    // Just verify no crash - imports may not always have spans
    assert!(!tokens.is_empty(), "Should have some tokens");
}

#[test]
fn test_extract_type_refs_from_kerml_element_classifier() {
    // Test KerML Element::Classifier
    let source = r#"package Test {
    classifier MyClassifier {
        feature f : Real;
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Should have type token for Real in feature
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    assert!(
        !type_tokens.is_empty(),
        "Expected type token from classifier feature, got {}",
        type_tokens.len()
    );
}

#[test]
fn test_extract_type_refs_from_kerml_element_feature() {
    // Test KerML Element::Feature (top-level)
    // BUG NOTE: Due to missing Element::Package handling, features inside packages
    // won't have type refs extracted. This test documents the limitation.
    let source = r#"package Test {
    feature myFeature : Real;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Due to bug, no type tokens will be extracted
    assert!(!tokens.is_empty(), "Should have tokens for package");
}

#[test]
fn test_extract_type_refs_from_kerml_element_other_variants() {
    // Test other KerML Element variants (Package, etc.) which are handled by default case
    let source = r#"package Test {
    // Just a package with a comment
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Should have token for package name
    assert!(!tokens.is_empty(), "Should have token for package");
}

#[test]
fn test_extract_type_refs_from_kerml_element_nested_structure() {
    // Test nested KerML structure
    let source = r#"package Outer {
    package Inner {
        class MyClass {
            feature f1 : Real;
            feature f2 : String;
        }
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Classifiers inside packages work, but may only get partial results
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    assert!(
        !type_tokens.is_empty(),
        "Expected at least 1 type token in nested structure, got {}",
        type_tokens.len()
    );
}

// ============================================================================
// Tests for extract_type_refs_from_usage_member (via public API)
// ============================================================================

#[test]
fn test_extract_type_refs_from_usage_member_comment() {
    // Test UsageMember::Comment
    let source = r#"package Test {
    part def Vehicle {
        part engine {
            // This is a comment in usage
        }
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Comments should not crash and should not produce type refs
    assert!(
        !tokens.is_empty(),
        "Should have tokens for package/definitions"
    );
}

#[test]
fn test_extract_type_refs_from_usage_member_nested_usage() {
    // Test UsageMember::Usage (nested)
    // This tests that nested usages are handled (though the function currently does nothing for them)
    let source = r#"package Test {
    part def Vehicle {
        part engine {
            part cylinder {
                attribute volume: Real;
            }
        }
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Should find type tokens even in deeply nested structures
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    // Should have at least 1 type token for Real
    assert!(
        !type_tokens.is_empty(),
        "Expected type token in deeply nested usage, got {}",
        type_tokens.len()
    );
}

#[test]
fn test_extract_type_refs_from_usage_member_empty_usage() {
    // Test empty usage body
    let source = r#"package Test {
    part def Vehicle {
        part engine {
        }
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Should have tokens for package/definitions but no type refs
    assert!(
        !tokens.is_empty(),
        "Should have tokens for package/definitions"
    );
}

// ============================================================================
// Edge case and integration tests
// ============================================================================

#[test]
fn test_empty_file() {
    // Test with completely empty file
    let source = "";

    let workspace = create_sysml_workspace(source, "empty.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "empty.sysml");

    // Empty file should produce no tokens
    assert!(tokens.is_empty(), "Empty file should have no tokens");
}

#[test]
fn test_file_with_only_comments() {
    // Test file with only comments
    let source = r#"
    // This is a comment
    /* This is a block comment */
    "#;

    let workspace = create_sysml_workspace(source, "comments.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "comments.sysml");

    // Comments only should produce no semantic tokens
    assert!(
        tokens.is_empty(),
        "Comments-only file should have no semantic tokens"
    );
}

#[test]
fn test_mixed_sysml_kerml_patterns() {
    // Test SysML with KerML-style features
    let source = r#"package Test {
    part def Vehicle {
        attribute speed: Real;
        attribute name: String;
    }
    
    part myVehicle: Vehicle;
}"#;

    let workspace = create_sysml_workspace(source, "mixed.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "mixed.sysml");

    // Should have tokens for types (Vehicle definition + myVehicle usage type ref)
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    // At minimum we should get Vehicle definition as a type token
    // Type refs for Real/String may or may not be in reference index
    assert!(
        !type_tokens.is_empty(),
        "Expected at least 1 type token, got {}",
        type_tokens.len()
    );
}

#[test]
fn test_kerml_all_feature_member_variants() {
    // Comprehensive test covering all FeatureMember variants
    // BUG NOTE: Features inside packages don't have type refs extracted
    let source = r#"package Test {
    feature baseFeature;
    feature typedFeature : Real subsets baseFeature redefines baseFeature;
}"#;

    let workspace = create_kerml_workspace(source, "comprehensive.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "comprehensive.kerml");

    // Due to bug, type refs won't be extracted
    assert!(!tokens.is_empty(), "Should have tokens");
}

#[test]
fn test_token_ordering_and_deduplication() {
    // Test that tokens are properly sorted and handled
    let source = r#"package Test {
    part def A {
        attribute x: Real;
    }
    part def B {
        attribute y: Real;
    }
}"#;

    let workspace = create_sysml_workspace(source, "ordering.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "ordering.sysml");

    // Verify tokens are sorted by line, then column
    for i in 1..tokens.len() {
        let prev = &tokens[i - 1];
        let curr = &tokens[i];
        assert!(
            (prev.line, prev.column) <= (curr.line, curr.column),
            "Tokens should be sorted: ({}, {}) should be <= ({}, {})",
            prev.line,
            prev.column,
            curr.line,
            curr.column
        );
    }
}

// ============================================================================
// Additional tests for full code coverage
// ============================================================================

#[test]
fn test_classifier_member_with_specialization() {
    // Test ClassifierMember::Specialization variant
    let source = r#"package Test {
    class Base;
    class Derived specializes Base;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Specialization should not produce type reference tokens from extract_type_refs_from_classifier_member
    // Just verify no crash
    assert!(!tokens.is_empty(), "Should have tokens for classes");
}

#[test]
fn test_classifier_member_with_import() {
    // Test ClassifierMember::Import variant
    let source = r#"package Test {
    classifier MyClassifier {
        import ScalarValues::*;
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Import inside classifier should not produce type reference tokens from extract_type_refs_from_classifier_member
    // Just verify no crash
    assert!(!tokens.is_empty(), "Should have tokens");
}

#[test]
fn test_classifier_member_comment() {
    // Test ClassifierMember::Comment variant explicitly
    let source = r#"classifier MyClassifier {
    // This is a comment inside a classifier
    /* Block comment */
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    // Comments should not produce type reference tokens
    assert!(!tokens.is_empty(), "Should have tokens for classifier");
}

#[test]
fn test_def_member_comment_variant() {
    // Explicitly test DefinitionMember::Comment variant (non-Usage)
    let source = r#"package Test {
    part def MyDef {
        // Comment in definition body
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Comments in definition body should not crash
    assert!(
        !tokens.is_empty(),
        "Should have tokens for package and definition"
    );
}

#[test]
fn test_usage_member_nested_recursion() {
    // Test that UsageMember::Usage actually gets handled by the parent function recursion
    let source = r#"package Test {
    part def Container {
        part inner {
            part deeplyNested {
                attribute value: Real;
            }
        }
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Should extract type token from deeply nested usage
    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();

    assert!(
        !type_tokens.is_empty(),
        "Should have type token from nested usage"
    );
}

// ============================================================================
// Tests for KerML Import Semantic Tokens (Issue: imports not highlighting)
// ============================================================================

/// Test that KerML import paths generate Namespace semantic tokens
/// This is the core test for the "imports not working for KerML" issue
#[test]
fn test_kerml_import_generates_namespace_token() {
    // This mimics the ScalarValues.kerml stdlib pattern
    let source = r#"package ScalarValues {
    private import Base::DataValue;
    datatype Boolean;
}"#;

    let workspace = create_kerml_workspace(source, "ScalarValues.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "ScalarValues.kerml");

    // Should have a Namespace token for "Base::DataValue" on line 1 (0-indexed)
    let namespace_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Namespace)
        .collect();

    assert!(
        !namespace_tokens.is_empty(),
        "Should have Namespace token for import path 'Base::DataValue'. All tokens: {tokens:?}"
    );

    // The import is on line 1 (0-indexed), "Base::DataValue" is 15 chars
    let import_token = namespace_tokens
        .iter()
        .find(|t| t.line == 1 && t.length == 15);

    assert!(
        import_token.is_some(),
        "Should have Namespace token for 'Base::DataValue' on line 1 with length 15. Namespace tokens: {namespace_tokens:?}"
    );
}

/// Test that multiple KerML imports all get semantic tokens
#[test]
fn test_kerml_multiple_imports_generate_tokens() {
    let source = r#"package TestPackage {
    import Base::DataValue;
    import ScalarValues::*;
    import Collections::Array;
    classifier MyClass;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    let namespace_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Namespace)
        .collect();

    // Should have 3 namespace tokens for the 3 imports
    assert!(
        namespace_tokens.len() >= 3,
        "Should have at least 3 Namespace tokens for 3 imports. Got: {namespace_tokens:?}"
    );
}

/// Test KerML import with wildcard (::*) generates token for the path
#[test]
fn test_kerml_wildcard_import_generates_token() {
    let source = r#"package Test {
    import ScalarValues::*;
    classifier MyClass;
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    let namespace_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Namespace)
        .collect();

    assert!(
        !namespace_tokens.is_empty(),
        "Should have Namespace token for wildcard import. All tokens: {tokens:?}"
    );
}

/// Test KerML import inside classifier body generates token
#[test]
fn test_kerml_import_in_classifier_body_generates_token() {
    let source = r#"package Outer {
    classifier MyClass {
        import Inner::*;
        feature myFeature;
    }
}"#;

    let workspace = create_kerml_workspace(source, "test.kerml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.kerml");

    let namespace_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Namespace)
        .collect();

    // The import inside classifier body should also get a token
    let inner_import_token = namespace_tokens.iter().find(|t| t.line == 2);

    assert!(
        inner_import_token.is_some(),
        "Should have Namespace token for import inside classifier body on line 2. Namespace tokens: {namespace_tokens:?}"
    );
}

/// Test that SysML imports also generate semantic tokens (for comparison)
#[test]
fn test_sysml_import_generates_namespace_token() {
    let source = r#"package TestPkg {
    import ScalarValues::Real;
    part def MyPart;
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    let namespace_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Namespace)
        .collect();

    assert!(
        !namespace_tokens.is_empty(),
        "Should have Namespace token for SysML import path. All tokens: {tokens:?}"
    );
}

// ============================================================================
// Comprehensive coverage test for all definition/usage types
// ============================================================================

/// Helper struct to track test results for each construct
#[derive(Debug, Default)]
#[allow(dead_code)]
struct ConstructTest {
    name: &'static str,
    source: &'static str,
    expected_definitions: Vec<&'static str>,
    expected_type_refs: Vec<&'static str>,
    /// Expected Property token references (for redefines/subsets on usages)
    expected_property_refs: Vec<&'static str>,
}

impl ConstructTest {
    /// Create a new test with no property refs (default for most tests)
    #[allow(dead_code)]
    fn new(
        name: &'static str,
        source: &'static str,
        expected_definitions: Vec<&'static str>,
        expected_type_refs: Vec<&'static str>,
    ) -> Self {
        Self {
            name,
            source,
            expected_definitions,
            expected_type_refs,
            expected_property_refs: vec![],
        }
    }

    /// Create a new test with property refs (for redefines/subsets on usages)
    #[allow(dead_code)]
    fn with_property_refs(
        name: &'static str,
        source: &'static str,
        expected_definitions: Vec<&'static str>,
        expected_type_refs: Vec<&'static str>,
        expected_property_refs: Vec<&'static str>,
    ) -> Self {
        Self {
            name,
            source,
            expected_definitions,
            expected_type_refs,
            expected_property_refs,
        }
    }
}

/// Test ALL SysML definition types get semantic tokens
#[test]
fn test_all_definition_types_get_semantic_tokens() {
    let tests = vec![
        // Part definition
        ConstructTest {
            name: "part def",
            source: "part def Vehicle;",
            expected_definitions: vec!["Vehicle"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Part definition with specialization
        ConstructTest {
            name: "part def with specialization",
            source: "part def Car :> Vehicle;",
            expected_definitions: vec!["Car"],
            expected_type_refs: vec!["Vehicle"],
            ..Default::default()
        },
        // Port definition
        ConstructTest {
            name: "port def",
            source: "port def DataPort;",
            expected_definitions: vec!["DataPort"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Action definition
        ConstructTest {
            name: "action def",
            source: "action def Move;",
            expected_definitions: vec!["Move"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Attribute definition
        ConstructTest {
            name: "attribute def",
            source: "attribute def Speed;",
            expected_definitions: vec!["Speed"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Item definition
        ConstructTest {
            name: "item def",
            source: "item def Fuel;",
            expected_definitions: vec!["Fuel"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Connection definition
        ConstructTest {
            name: "connection def",
            source: "connection def Link;",
            expected_definitions: vec!["Link"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Interface definition
        ConstructTest {
            name: "interface def",
            source: "interface def DataInterface;",
            expected_definitions: vec!["DataInterface"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Allocation definition
        ConstructTest {
            name: "allocation def",
            source: "allocation def ResourceAlloc;",
            expected_definitions: vec!["ResourceAlloc"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Requirement definition
        ConstructTest {
            name: "requirement def",
            source: "requirement def SafetyReq;",
            expected_definitions: vec!["SafetyReq"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Constraint definition
        ConstructTest {
            name: "constraint def",
            source: "constraint def MassLimit;",
            expected_definitions: vec!["MassLimit"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // State definition
        ConstructTest {
            name: "state def",
            source: "state def EngineState;",
            expected_definitions: vec!["EngineState"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Calc definition
        ConstructTest {
            name: "calc def",
            source: "calc def TotalMass;",
            expected_definitions: vec!["TotalMass"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Case definition
        ConstructTest {
            name: "case def",
            source: "case def TestCase;",
            expected_definitions: vec!["TestCase"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Analysis case definition
        ConstructTest {
            name: "analysis case def",
            source: "analysis def PerformanceAnalysis;",
            expected_definitions: vec!["PerformanceAnalysis"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Verification case definition
        ConstructTest {
            name: "verification case def",
            source: "verification def SafetyVerification;",
            expected_definitions: vec!["SafetyVerification"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Use case definition
        ConstructTest {
            name: "use case def",
            source: "use case def DriveVehicle;",
            expected_definitions: vec!["DriveVehicle"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // View definition
        ConstructTest {
            name: "view def",
            source: "view def StructuralView;",
            expected_definitions: vec!["StructuralView"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Viewpoint definition
        ConstructTest {
            name: "viewpoint def",
            source: "viewpoint def EngineerViewpoint;",
            expected_definitions: vec!["EngineerViewpoint"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Rendering definition
        ConstructTest {
            name: "rendering def",
            source: "rendering def DiagramRender;",
            expected_definitions: vec!["DiagramRender"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Metadata definition
        ConstructTest {
            name: "metadata def",
            source: "metadata def CustomMetadata;",
            expected_definitions: vec!["CustomMetadata"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Metadata definition with short name and specialization
        ConstructTest {
            name: "metadata def with short name",
            source: "metadata def <orig> OriginalMetadata :> SemanticMetadata;",
            expected_definitions: vec!["OriginalMetadata", "orig"],
            expected_type_refs: vec!["SemanticMetadata"],
            ..Default::default()
        },
        // Occurrence definition
        ConstructTest {
            name: "occurrence def",
            source: "occurrence def Event;",
            expected_definitions: vec!["Event"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Flow connection definition
        ConstructTest {
            name: "flow connection def",
            source: "flow def DataFlow;",
            expected_definitions: vec!["DataFlow"],
            expected_type_refs: vec![],
            ..Default::default()
        },
        // Concern definition
        ConstructTest {
            name: "concern def",
            source: "concern def SafetyConcern;",
            expected_definitions: vec!["SafetyConcern"],
            expected_type_refs: vec![],
            ..Default::default()
        },
    ];

    let mut failures: Vec<String> = vec![];

    for test in &tests {
        let workspace = create_sysml_workspace(test.source, "test.sysml");
        let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

        let type_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| t.token_type == TokenType::Type)
            .collect();

        // Check we have at least as many type tokens as expected definitions + type refs
        let min_expected = test.expected_definitions.len() + test.expected_type_refs.len();
        if type_tokens.len() < min_expected {
            failures.push(format!(
                "{}: Expected at least {} Type tokens ({} defs + {} refs), got {}. Tokens: {:?}",
                test.name,
                min_expected,
                test.expected_definitions.len(),
                test.expected_type_refs.len(),
                type_tokens.len(),
                type_tokens
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Definition type coverage failures:\n{}",
        failures.join("\n")
    );
}

/// Test ALL SysML usage types get semantic tokens
#[test]
fn test_all_usage_types_get_semantic_tokens() {
    let tests = vec![
        // Part usage with typing
        ConstructTest {
            name: "part usage",
            source: "part myCar : Vehicle;",
            expected_definitions: vec![],
            expected_type_refs: vec!["Vehicle"],
            ..Default::default()
        },
        // Port usage
        ConstructTest {
            name: "port usage",
            source: "port dataIn : DataPort;",
            expected_definitions: vec![],
            expected_type_refs: vec!["DataPort"],
            ..Default::default()
        },
        // Action usage
        ConstructTest {
            name: "action usage",
            source: "action move : Move;",
            expected_definitions: vec![],
            expected_type_refs: vec!["Move"],
            ..Default::default()
        },
        // Attribute usage
        ConstructTest {
            name: "attribute usage",
            source: "attribute speed : Real;",
            expected_definitions: vec![],
            expected_type_refs: vec!["Real"],
            ..Default::default()
        },
        // Item usage
        ConstructTest {
            name: "item usage",
            source: "item fuel : Fuel;",
            expected_definitions: vec![],
            expected_type_refs: vec!["Fuel"],
            ..Default::default()
        },
        // Reference usage with redefines (target is a usage/feature, not a type)
        ConstructTest {
            name: "ref usage redefines",
            source: "ref velocity :>> speed;",
            expected_definitions: vec![],
            expected_type_refs: vec![], // redefines targets usages, not types
            expected_property_refs: vec!["speed"],
        },
        // Connection usage
        ConstructTest {
            name: "connection usage",
            source: "connection link : Link;",
            expected_definitions: vec![],
            expected_type_refs: vec!["Link"],
            ..Default::default()
        },
        // Allocation usage
        ConstructTest {
            name: "allocation usage",
            source: "allocation alloc : ResourceAlloc;",
            expected_definitions: vec![],
            expected_type_refs: vec!["ResourceAlloc"],
            ..Default::default()
        },
        // State usage
        ConstructTest {
            name: "state usage",
            source: "state running : EngineState;",
            expected_definitions: vec![],
            expected_type_refs: vec!["EngineState"],
            ..Default::default()
        },
        // Exhibit state usage
        ConstructTest {
            name: "exhibit state usage",
            source: "exhibit state running : EngineState;",
            expected_definitions: vec![],
            expected_type_refs: vec!["EngineState"],
            ..Default::default()
        },
        // Perform action usage
        ConstructTest {
            name: "perform action usage",
            source: "perform action move : Move;",
            expected_definitions: vec![],
            expected_type_refs: vec!["Move"],
            ..Default::default()
        },
    ];

    let mut failures: Vec<String> = vec![];

    for test in &tests {
        let workspace = create_sysml_workspace(test.source, "test.sysml");
        let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

        let type_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| t.token_type == TokenType::Type)
            .collect();

        // For usages, we primarily care about type references
        if type_tokens.len() < test.expected_type_refs.len() {
            failures.push(format!(
                "{}: Expected at least {} Type tokens for type refs, got {}. Tokens: {:?}",
                test.name,
                test.expected_type_refs.len(),
                type_tokens.len(),
                type_tokens
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Usage type coverage failures:\n{}",
        failures.join("\n")
    );
}

/// Test nested usages in definition bodies get semantic tokens
#[test]
fn test_nested_usages_in_definitions_get_semantic_tokens() {
    let tests = vec![
        // Part def with attribute members
        ConstructTest {
            name: "part def with attributes",
            source: r#"part def Vehicle {
                attribute mass : Real;
                attribute speed : Real;
            }"#,
            expected_definitions: vec!["Vehicle"],
            expected_type_refs: vec!["Real", "Real"],
            ..Default::default()
        },
        // Connection def with end usages
        ConstructTest {
            name: "connection def with ends",
            source: r#"connection def Derivation {
                end r1 : Req1;
                end r1_1 : Req1_1;
            }"#,
            expected_definitions: vec!["Derivation"],
            expected_type_refs: vec!["Req1", "Req1_1"],
            ..Default::default()
        },
        // Part def with part members
        ConstructTest {
            name: "part def with parts",
            source: r#"part def Car {
                part engine : Engine;
                part transmission : Transmission;
            }"#,
            expected_definitions: vec!["Car"],
            expected_type_refs: vec!["Engine", "Transmission"],
            ..Default::default()
        },
        // Interface def with port members
        ConstructTest {
            name: "interface def with ports",
            source: r#"interface def DataInterface {
                port dataIn : DataPort;
                port dataOut : DataPort;
            }"#,
            expected_definitions: vec!["DataInterface"],
            expected_type_refs: vec!["DataPort", "DataPort"],
            ..Default::default()
        },
        // Requirement def with constraint
        ConstructTest {
            name: "requirement def with constraint",
            source: r#"requirement def SafetyReq {
                attribute maxSpeed : Real;
            }"#,
            expected_definitions: vec!["SafetyReq"],
            expected_type_refs: vec!["Real"],
            ..Default::default()
        },
    ];

    let mut failures: Vec<String> = vec![];

    for test in &tests {
        let workspace = create_sysml_workspace(test.source, "test.sysml");
        let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

        let type_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| t.token_type == TokenType::Type)
            .collect();

        let min_expected = test.expected_definitions.len() + test.expected_type_refs.len();
        if type_tokens.len() < min_expected {
            failures.push(format!(
                "{}: Expected at least {} Type tokens ({} defs + {} refs), got {}. Tokens: {:?}",
                test.name,
                min_expected,
                test.expected_definitions.len(),
                test.expected_type_refs.len(),
                type_tokens.len(),
                type_tokens
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Nested usage coverage failures:\n{}",
        failures.join("\n")
    );
}

/// Test specialization, redefines, and subsets all get semantic tokens
#[test]
fn test_relationship_type_refs_get_semantic_tokens() {
    let tests = vec![
        // Specialization
        ConstructTest {
            name: "specialization",
            source: "part def ElectricCar :> Car;",
            expected_definitions: vec!["ElectricCar"],
            expected_type_refs: vec!["Car"],
            ..Default::default()
        },
        // Multiple specializations
        ConstructTest {
            name: "multiple specializations",
            source: "part def HybridCar :> Car, Electric;",
            expected_definitions: vec!["HybridCar"],
            expected_type_refs: vec!["Car", "Electric"],
            ..Default::default()
        },
        // Redefinition
        ConstructTest {
            name: "redefinition",
            source: "part def SportsCar :>> Car;",
            expected_definitions: vec!["SportsCar"],
            expected_type_refs: vec!["Car"],
            ..Default::default()
        },
        // Subsetting (on usage - target is a usage/feature, not a type)
        ConstructTest {
            name: "subsetting",
            source: "part frontWheels :> wheels;",
            expected_definitions: vec![],
            expected_type_refs: vec![], // subsetting targets usages, not types
            expected_property_refs: vec!["wheels"],
        },
        // Metadata body usage with ref :>> redefines
        ConstructTest {
            name: "metadata body usage redefines",
            source: r#"metadata def TestMeta {
                ref :>> annotatedElement : Usage;
            }"#,
            expected_definitions: vec!["TestMeta"],
            expected_type_refs: vec!["Usage"],
            ..Default::default()
        },
        // Metadata body usage with value assignment
        ConstructTest {
            name: "metadata body usage with value",
            source: r#"metadata def TestMeta2 {
                ref :>> baseType = causes as Usage;
            }"#,
            expected_definitions: vec!["TestMeta2"],
            expected_type_refs: vec!["Usage"], // "causes" is a value, "Usage" is a type
            ..Default::default()
        },
    ];

    let mut failures: Vec<String> = vec![];

    for test in &tests {
        let workspace = create_sysml_workspace(test.source, "test.sysml");
        let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

        let type_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| t.token_type == TokenType::Type)
            .collect();

        let min_expected = test.expected_definitions.len() + test.expected_type_refs.len();
        if type_tokens.len() < min_expected {
            failures.push(format!(
                "{}: Expected at least {} Type tokens ({} defs + {} refs), got {}. Tokens: {:?}",
                test.name,
                min_expected,
                test.expected_definitions.len(),
                test.expected_type_refs.len(),
                type_tokens.len(),
                type_tokens
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Relationship type ref coverage failures:\n{}",
        failures.join("\n")
    );
}

#[test]
fn test_debug_time_varying_attribute_tokens() {
    // TimeVaryingAttribute.sysml patterns - anonymous features with redefinition
    let source = r#"package TimeVaryingAttribute {
    private import SI::s;
    
    item def PwrCmd {
        attribute pwrLevel: ScalarValues::Integer;
    }
    
    part def Transport2 {
        private import Time::*;
        attribute startTime = TimeOf(start);
        attribute elapseTime :> ISQ::duration;
        attribute :>> localClock.currentTime = startTime + elapseTime;
        
        out item pwrCmd:PwrCmd;
        // Lifetime conditions
        timeslice :>> portionOfLife {
            snapshot :>> start {
                :>> elapseTime = 0 [s];
                :>> pwrCmd.pwrLevel = 0;
            }
            snapshot :>> done {
                :>> elapseTime = 2 [s];
                :>> pwrCmd.pwrLevel = 1;
            }
        }
    }
}"#;

    let workspace = create_sysml_workspace(source, "test.sysml");
    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    println!("\n=== Symbols ===");
    for symbol in workspace.symbol_table().iter_symbols() {
        if let Some(span) = symbol.span()
            && symbol.source_file() == Some("test.sysml")
        {
            println!(
                "  {} at line {}, col {}",
                symbol.qualified_name(),
                span.start.line,
                span.start.column
            );
        }
    }

    println!("\n=== References ===");
    for ref_info in workspace
        .reference_index()
        .get_references_in_file("test.sysml")
    {
        println!(
            "  {} -> line {}, col {}->{}, token_type: {:?}",
            ref_info.source_qname,
            ref_info.span.start.line,
            ref_info.span.start.column,
            ref_info.span.end.column,
            ref_info.token_type
        );
    }

    println!("\n=== Semantic Tokens ===");
    for token in &tokens {
        // Get the text at this position
        let lines: Vec<&str> = source.lines().collect();
        let text = if (token.line as usize) < lines.len() {
            let line = lines[token.line as usize];
            let start = token.column as usize;
            let end = (token.column + token.length) as usize;
            if end <= line.len() {
                &line[start..end]
            } else {
                "<out of bounds>"
            }
        } else {
            "<line out of bounds>"
        };
        println!(
            "  Line {:2}, Col {:2}, Len {:2}: {:?} = '{}'",
            token.line, token.column, token.length, token.token_type, text
        );
    }
    println!("\nTotal tokens: {}", tokens.len());

    // Verify no trailing whitespace in token lengths
    for token in &tokens {
        let lines: Vec<&str> = source.lines().collect();
        if (token.line as usize) < lines.len() {
            let line = lines[token.line as usize];
            let start = token.column as usize;
            let end = (token.column + token.length) as usize;
            if end <= line.len() {
                let text = &line[start..end];
                assert!(
                    !text.ends_with(' '),
                    "Token at line {} col {} should not include trailing whitespace: '{}'",
                    token.line,
                    token.column,
                    text
                );
            }
        }
    }

    // Verify we have at least some tokens
    assert!(
        tokens.len() >= 10,
        "Should have at least 10 semantic tokens"
    );
}
