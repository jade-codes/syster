#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use crate::parser::sysml::{Rule, SysMLParser};
use crate::syntax::sysml::ast::utils::ref_from;
use pest::Parser;

// ============================================================================
// Direct Match Tests - Rules that directly match and return their text
// ============================================================================

#[test]
fn test_ref_from_identifier() {
    let source = "Vehicle";
    let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Vehicle".to_string()));
}

#[test]
fn test_ref_from_identifier_with_underscore() {
    let source = "my_vehicle";
    let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("my_vehicle".to_string()));
}

#[test]
fn test_ref_from_identifier_with_numbers() {
    let source = "vehicle123";
    let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("vehicle123".to_string()));
}

#[test]
fn test_ref_from_quoted_name() {
    // Quotes should be stripped from quoted names
    let source = "'My Vehicle'";
    let mut pairs = SysMLParser::parse(Rule::quoted_name, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("My Vehicle".to_string()));
}

#[test]
fn test_ref_from_quoted_name_with_special_chars() {
    // Quotes should be stripped from quoted names
    let source = "'Vehicle-123!@#'";
    let mut pairs = SysMLParser::parse(Rule::quoted_name, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Vehicle-123!@#".to_string()));
}

#[test]
fn test_ref_from_feature_reference() {
    let source = "myFeature";
    let mut pairs = SysMLParser::parse(Rule::feature_reference, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("myFeature".to_string()));
}

#[test]
fn test_ref_from_feature_reference_qualified() {
    let source = "Package::Class::feature";
    let mut pairs = SysMLParser::parse(Rule::feature_reference, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Package::Class::feature".to_string()));
}

#[test]
fn test_ref_from_classifier_reference() {
    let source = "Vehicle";
    let mut pairs = SysMLParser::parse(Rule::classifier_reference, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Vehicle".to_string()));
}

#[test]
fn test_ref_from_classifier_reference_qualified() {
    let source = "Package::Vehicle";
    let mut pairs = SysMLParser::parse(Rule::classifier_reference, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Package::Vehicle".to_string()));
}

// ============================================================================
// Trimming Tests - Verify whitespace is trimmed
// ============================================================================

#[test]
fn test_ref_from_identifier_trims_whitespace() {
    // Note: pest grammar typically handles whitespace, but we test trimming anyway
    let source = "Vehicle";
    let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    // Should not have leading/trailing whitespace
    assert_eq!(result.as_deref(), Some("Vehicle"));
    assert!(!result.as_ref().unwrap().starts_with(' '));
}

// ============================================================================
// Recursive Search Tests - Rules that require searching through children
// ============================================================================

#[test]
fn test_ref_from_typed_by_finds_reference() {
    let source = ": Vehicle";
    let mut pairs = SysMLParser::parse(Rule::typed_by, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Vehicle".to_string()));
}

#[test]
fn test_ref_from_typings_finds_reference() {
    let source = ": Vehicle";
    let mut pairs = SysMLParser::parse(Rule::typings, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Vehicle".to_string()));
}

#[test]
fn test_ref_from_subsets_finds_reference() {
    let source = ":> baseFeature";
    let mut pairs = SysMLParser::parse(Rule::subsets, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("baseFeature".to_string()));
}

#[test]
fn test_ref_from_subsettings_finds_reference() {
    let source = ":> baseFeature";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("baseFeature".to_string()));
}

#[test]
fn test_ref_from_redefines_finds_reference() {
    let source = ":>> originalFeature";
    let mut pairs = SysMLParser::parse(Rule::redefines, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("originalFeature".to_string()));
}

#[test]
fn test_ref_from_redefinitions_finds_reference() {
    let source = ":>> originalFeature";
    let mut pairs = SysMLParser::parse(Rule::redefinitions, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("originalFeature".to_string()));
}

#[test]
fn test_ref_from_feature_specialization_with_typing() {
    let source = ": Vehicle";
    let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Vehicle".to_string()));
}

#[test]
fn test_ref_from_feature_specialization_with_subsetting() {
    let source = ":> baseFeature";
    let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("baseFeature".to_string()));
}

#[test]
fn test_ref_from_feature_specialization_with_redefinition() {
    let source = ":>> originalFeature";
    let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("originalFeature".to_string()));
}

// ============================================================================
// Multiple References Tests - Verify only first is returned
// ============================================================================

#[test]
fn test_ref_from_returns_first_reference_only() {
    // When there are multiple references in a list, ref_from should return the first one
    let source = ":> base1, base2, base3";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    // Should return the first reference found during recursion
    assert_eq!(result, Some("base1".to_string()));
}

// ============================================================================
// Complex Nested Structure Tests
// ============================================================================

#[test]
fn test_ref_from_part_definition_finds_name() {
    // Test that ref_from can extract identifier from a part definition
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    // Should find the Vehicle identifier
    assert_eq!(result, Some("Vehicle".to_string()));
}

#[test]
fn test_ref_from_deeply_nested_reference() {
    // Test that recursion works through multiple levels
    let source = "part myPart : Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();
    let pair = pairs.next().unwrap();

    // ref_from should find the first reference it encounters (the part usage name "myPart")
    let result = ref_from(&pair);

    // Should find first identifier during depth-first search: the part usage name "myPart"
    assert_eq!(result, Some("myPart".to_string()));
}

// ============================================================================
// Complex Structure Tests
// ============================================================================

#[test]
fn test_ref_from_with_qualified_name_in_typing() {
    let source = ": Package::Vehicle";
    let mut pairs = SysMLParser::parse(Rule::typings, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("Package::Vehicle".to_string()));
}

#[test]
fn test_ref_from_with_quoted_name_in_subsetting() {
    // Quotes should be stripped from extracted references
    let source = ":> 'base feature'";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("base feature".to_string()));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_ref_from_single_letter_identifier() {
    let source = "x";
    let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("x".to_string()));
}

#[test]
fn test_ref_from_long_identifier() {
    let source = "veryLongIdentifierNameWithManyCharacters";
    let mut pairs = SysMLParser::parse(Rule::identifier, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(
        result,
        Some("veryLongIdentifierNameWithManyCharacters".to_string())
    );
}

#[test]
fn test_ref_from_feature_reference_all_keyword() {
    let source = "all";
    let mut pairs = SysMLParser::parse(Rule::feature_reference, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("all".to_string()));
}

// ============================================================================
// Subclassification Tests
// ============================================================================

#[test]
fn test_ref_from_subclassification_part() {
    let source = ":> BaseClass";
    let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("BaseClass".to_string()));
}

#[test]
fn test_ref_from_redefinition_part() {
    let source = ":>> BaseClass";
    let mut pairs = SysMLParser::parse(Rule::redefinition_part, source).unwrap();
    let pair = pairs.next().unwrap();

    let result = ref_from(&pair);

    assert_eq!(result, Some("BaseClass".to_string()));
}
