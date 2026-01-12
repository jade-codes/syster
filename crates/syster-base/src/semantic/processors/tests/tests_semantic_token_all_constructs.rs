//! Comprehensive tests for semantic tokens across ALL SysML constructs.
//!
//! This file ensures every SysML construct that should produce semantic tokens
//! is properly covered. It serves as a living specification of what gets
//! semantic highlighting.
//!
//! ## Token Types
//! - `Namespace` = packages, imports
//! - `Type` = definitions, classifiers, type references
//! - `Property` = usages, features
//! - `Variable` = aliases
//! - `Keyword` = (reserved for future)
//!
//! ## Test Categories
//! 1. Definition types (all `xxx def` constructs)
//! 2. Usage types (all `xxx` constructs that create instances)
//! 3. Relationship references (specialization, typing, subsetting, etc.)
//! 4. Metadata constructs (metadata def, metadata body usages)
//! 5. Import statements
//! 6. Aliases

#![allow(clippy::unwrap_used)]

use crate::semantic::Workspace;
use crate::semantic::processors::{SemanticTokenCollector, TokenType};
use crate::syntax::SyntaxFile;
use crate::syntax::parser::parse_content;
use std::path::PathBuf;

/// Helper to parse SysML content and create workspace
fn create_workspace(source: &str) -> Workspace<SyntaxFile> {
    let path = PathBuf::from("test.sysml");
    let syntax_file = parse_content(source, &path).expect("Parse should succeed");

    let mut workspace = Workspace::<SyntaxFile>::new();
    workspace.add_file(path.clone(), syntax_file);
    workspace.populate_file(&path).expect("Failed to populate");

    workspace
}

/// Count tokens by type
fn count_tokens_by_type(workspace: &Workspace<SyntaxFile>, token_type: TokenType) -> usize {
    SemanticTokenCollector::collect_from_workspace(workspace, "test.sysml")
        .iter()
        .filter(|t| t.token_type == token_type)
        .count()
}

// ============================================================================
// DEFINITION TYPES
// ============================================================================

#[test]
fn test_part_definition_produces_type_token() {
    let workspace = create_workspace("part def Vehicle;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_action_definition_produces_type_token() {
    let workspace = create_workspace("action def Drive;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_requirement_definition_produces_type_token() {
    let workspace = create_workspace("requirement def Safety;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_port_definition_produces_type_token() {
    let workspace = create_workspace("port def DataPort;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_item_definition_produces_type_token() {
    let workspace = create_workspace("item def Fuel;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_attribute_definition_produces_type_token() {
    let workspace = create_workspace("attribute def Speed;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_concern_definition_produces_type_token() {
    let workspace = create_workspace("concern def Maintainability;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_case_definition_produces_type_token() {
    let workspace = create_workspace("case def TestCase;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_view_definition_produces_type_token() {
    let workspace = create_workspace("view def SystemView;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_viewpoint_definition_produces_type_token() {
    let workspace = create_workspace("viewpoint def EngineeringView;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_constraint_definition_produces_type_token() {
    let workspace = create_workspace("constraint def PowerConstraint;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_calculation_definition_produces_type_token() {
    let workspace = create_workspace("calc def ComputeSpeed;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_state_definition_produces_type_token() {
    let workspace = create_workspace("state def Running;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_connection_definition_produces_type_token() {
    let workspace = create_workspace("connection def DataFlow;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_interface_definition_produces_type_token() {
    let workspace = create_workspace("interface def USB;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_allocation_definition_produces_type_token() {
    let workspace = create_workspace("allocation def Hardware;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_enumeration_definition_produces_type_token() {
    let workspace = create_workspace("enum def Color;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_flow_definition_produces_type_token() {
    let workspace = create_workspace("flow def EnergyFlow;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_metadata_definition_produces_type_token() {
    let workspace = create_workspace("metadata def CustomMeta;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

// ============================================================================
// USAGE TYPES (produce Property tokens)
// ============================================================================

#[test]
fn test_part_usage_produces_property_token() {
    let workspace = create_workspace("part engine;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_attribute_usage_produces_property_token() {
    let workspace = create_workspace("attribute mass;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_port_usage_produces_property_token() {
    let workspace = create_workspace("port dataIn;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_action_usage_produces_property_token() {
    let workspace = create_workspace("action drive;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_item_usage_produces_property_token() {
    let workspace = create_workspace("item fuel;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_requirement_usage_produces_property_token() {
    let workspace = create_workspace("requirement safety;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_constraint_usage_produces_property_token() {
    let workspace = create_workspace("constraint power;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_state_usage_produces_property_token() {
    let workspace = create_workspace("state running;");
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

// ============================================================================
// RELATIONSHIP REFERENCES (produce Type tokens for the target)
// ============================================================================

#[test]
fn test_specialization_produces_type_token_for_target() {
    let workspace = create_workspace("part def ElectricCar :> Car;");
    // Should have 2 Type tokens: ElectricCar (def) + Car (specialization target)
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 2);
}

#[test]
fn test_redefinition_produces_type_token_for_target() {
    let workspace = create_workspace("part def SportsCar :>> Car;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 2);
}

#[test]
fn test_subsetting_produces_property_token_for_target() {
    // Subsetting on a usage (`part wheels :> components;`) targets another usage,
    // so the target `components` should be highlighted as Property, not Type.
    let workspace = create_workspace("part wheels :> components;");
    // wheels (usage) + components (subsetting target, which is also a usage)
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 2);
}

#[test]
fn test_typing_produces_type_token_for_target() {
    let workspace = create_workspace("part engine : Engine;");
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}

#[test]
fn test_multiple_specializations_produce_type_tokens() {
    let workspace = create_workspace("part def HybridCar :> Car, Electric;");
    // Should have 3 Type tokens: HybridCar (def) + Car + Electric
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 3);
}

// ============================================================================
// METADATA CONSTRUCTS
// ============================================================================

#[test]
fn test_metadata_body_usage_produces_tokens() {
    let workspace = create_workspace(
        r#"metadata def TestMeta {
            ref :>> annotatedElement : Usage;
        }"#,
    );
    // Should have Type token for TestMeta (def) and Usage (type ref)
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 2);
}

#[test]
fn test_metadata_body_usage_with_value_produces_tokens() {
    let workspace = create_workspace(
        r#"metadata def TestMeta {
            ref :>> baseType = causes as Usage;
        }"#,
    );
    // Should have Type token for TestMeta and Usage
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 2);
}

/// Tests that anonymous usages with redefinitions get correct semantic tokens.
/// Anonymous usages (like `ref :>> annotatedElement`) derive their name from
/// the first redefinition target and should get Property tokens.
#[test]
fn test_anonymous_usage_with_redefinition_gets_property_token() {
    let workspace = create_workspace(
        r#"metadata def TestMeta {
    ref :>> annotatedElement : Usage;
    ref :>> baseType = causes as Usage;
}"#,
    );

    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // We expect:
    // - TestMeta (Type) - the metadata def name
    // - annotatedElement (Property) - the redefined feature name
    // - Usage (Type) - the type reference after ":"
    // - baseType (Property) - the redefined feature name
    // - Usage (Type) - the type reference after "as"

    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();
    let property_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Property)
        .collect();

    // TestMeta + Usage (line 2) + Usage (line 3) = 3 Type tokens
    assert_eq!(
        type_tokens.len(),
        3,
        "Expected 3 Type tokens (TestMeta, Usage x2), got {}",
        type_tokens.len()
    );

    // annotatedElement + baseType = 2 Property tokens
    assert_eq!(
        property_tokens.len(),
        2,
        "Expected 2 Property tokens (annotatedElement, baseType), got {}",
        property_tokens.len()
    );

    // Verify the Property tokens are at the expected positions
    assert!(
        property_tokens
            .iter()
            .any(|t| t.line == 1 && t.column == 12),
        "annotatedElement should have Property token"
    );
    assert!(
        property_tokens
            .iter()
            .any(|t| t.line == 2 && t.column == 12),
        "baseType should have Property token"
    );
}

/// Tests that anonymous usages with subsettings (:>) get correct semantic tokens.
/// Anonymous usages (like `ref :> annotatedElement`) derive their name from
/// the first subsetting target and should get Property tokens.
#[test]
fn test_anonymous_usage_with_subsetting_gets_property_token() {
    let workspace = create_workspace(
        r#"metadata def TestMeta {
    ref :> annotatedElement : ConnectionDefinition;
    ref :> baseType : ConnectionUsage;
}"#,
    );

    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    let property_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Property)
        .collect();

    // annotatedElement + baseType = 2 Property tokens
    assert_eq!(
        property_tokens.len(),
        2,
        "Expected 2 Property tokens (annotatedElement, baseType), got {}",
        property_tokens.len()
    );

    // Verify the Property tokens are at the expected positions
    assert!(
        property_tokens
            .iter()
            .any(|t| t.line == 1 && t.column == 11),
        "annotatedElement should have Property token"
    );
    assert!(
        property_tokens
            .iter()
            .any(|t| t.line == 2 && t.column == 11),
        "baseType should have Property token"
    );
}

/// Tests that qualified type references in metadata body get correct semantic tokens.
/// e.g., `ref :> annotatedElement : SysML::ConnectionDefinition;`
/// Both `annotatedElement` (Property) and `SysML::ConnectionDefinition` (Type) should be highlighted.
#[test]
fn test_metadata_body_qualified_type_reference() {
    let workspace = create_workspace(
        r#"metadata def CausationMetadata {
    ref :> annotatedElement : SysML::ConnectionDefinition;
    ref :> annotatedElement : SysML::ConnectionUsage;
}"#,
    );

    let tokens = SemanticTokenCollector::collect_from_workspace(&workspace, "test.sysml");

    // Debug output - symbols
    println!("\n=== Symbol Table ===");
    for sym in workspace.symbol_table().iter_symbols() {
        println!(
            "  {} -> {:?} @ {:?}",
            sym.qualified_name(),
            std::mem::discriminant(sym),
            sym.span()
        );
    }

    // Debug output - reference index
    println!("\n=== Reference Index ===");
    for target in workspace.reference_index().targets() {
        for ref_info in workspace.reference_index().get_references(target) {
            println!(
                "  {} <- {} @ {:?}",
                target, ref_info.source_qname, ref_info.span
            );
        }
    }

    // Debug output - tokens
    println!("\n=== All Semantic Tokens ===");
    for t in &tokens {
        println!(
            "Line {}, Col {}, Len {}: {:?}",
            t.line, t.column, t.length, t.token_type
        );
    }

    let type_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Type)
        .collect();
    let property_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| t.token_type == TokenType::Property)
        .collect();

    println!("\n=== Type tokens ({}) ===", type_tokens.len());
    for t in &type_tokens {
        println!("  Line {}, Col {}, Len {}", t.line, t.column, t.length);
    }

    println!("\n=== Property tokens ({}) ===", property_tokens.len());
    for t in &property_tokens {
        println!("  Line {}, Col {}, Len {}", t.line, t.column, t.length);
    }

    // We expect:
    // - CausationMetadata (Type) - the metadata def name
    // - annotatedElement (Property) - the subsetted feature name (line 1)
    // - SysML::ConnectionDefinition (Type) - the qualified type reference (line 1)
    // - annotatedElement (Property) - the subsetted feature name (line 2)
    // - SysML::ConnectionUsage (Type) - the qualified type reference (line 2)

    // CausationMetadata + SysML::ConnectionDefinition + SysML::ConnectionUsage = at least 3 Type tokens
    assert!(
        type_tokens.len() >= 3,
        "Expected at least 3 Type tokens (CausationMetadata, SysML::ConnectionDefinition, SysML::ConnectionUsage), got {}",
        type_tokens.len()
    );

    // Both annotatedElement references should be Property tokens now
    // (first from symbol table, second from reference index with Property token type)
    assert!(
        property_tokens.len() >= 2,
        "Expected at least 2 Property tokens (annotatedElement x2), got {}",
        property_tokens.len()
    );
}

#[test]
fn test_metadata_def_with_specialization() {
    let workspace = create_workspace("metadata def CustomMeta :> SemanticMetadata;");
    // CustomMeta (def) + SemanticMetadata (specialization)
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 2);
}

// ============================================================================
// PACKAGES (produce Namespace tokens)
// ============================================================================

#[test]
fn test_package_produces_namespace_token() {
    let workspace = create_workspace("package MyPackage;");
    assert!(count_tokens_by_type(&workspace, TokenType::Namespace) >= 1);
}

#[test]
fn test_nested_packages_produce_namespace_tokens() {
    let workspace = create_workspace(
        r#"package Outer {
            package Inner;
        }"#,
    );
    assert!(count_tokens_by_type(&workspace, TokenType::Namespace) >= 2);
}

// ============================================================================
// IMPORTS (produce Namespace tokens)
// ============================================================================

#[test]
fn test_import_produces_namespace_token() {
    let workspace = create_workspace("import ScalarValues::*;");
    assert!(count_tokens_by_type(&workspace, TokenType::Namespace) >= 1);
}

#[test]
fn test_member_import_produces_namespace_token() {
    let workspace = create_workspace("import ScalarValues::Real;");
    assert!(count_tokens_by_type(&workspace, TokenType::Namespace) >= 1);
}

// ============================================================================
// ALIASES (produce Variable tokens)
// ============================================================================

#[test]
fn test_alias_produces_variable_token() {
    let workspace = create_workspace("alias Car for Vehicle;");
    assert!(count_tokens_by_type(&workspace, TokenType::Variable) >= 1);
}

// ============================================================================
// COMPLEX/NESTED CONSTRUCTS
// ============================================================================

#[test]
fn test_definition_with_nested_usages() {
    let workspace = create_workspace(
        r#"part def Vehicle {
            attribute mass : Real;
            part engine : Engine;
        }"#,
    );
    // Vehicle (Type), Real (Type), Engine (Type)
    // mass (Property), engine (Property)
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 3);
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 2);
}

#[test]
fn test_package_with_definitions_and_usages() {
    let workspace = create_workspace(
        r#"package Test {
            part def Car;
            part myCar : Car;
        }"#,
    );
    assert!(count_tokens_by_type(&workspace, TokenType::Namespace) >= 1);
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 2); // Car def + Car ref
    assert!(count_tokens_by_type(&workspace, TokenType::Property) >= 1);
}

#[test]
fn test_qualified_type_reference() {
    let workspace = create_workspace(r#"part engine : Automotive::Engine;"#);
    // The qualified name Automotive::Engine should produce a Type token
    assert!(count_tokens_by_type(&workspace, TokenType::Type) >= 1);
}
