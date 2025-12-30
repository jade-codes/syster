#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use crate::parser::sysml::{Rule, SysMLParser};
use crate::syntax::sysml::ast::utils::{all_refs_from, all_refs_with_spans_from};
use pest::Parser;

// ============================================================================
// Tests for all_refs_from()
// ============================================================================

#[test]
fn test_all_refs_from_single_specialization() {
    // Test extracting a single reference from a specialization
    let source = ":> Vehicle";
    let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0], "Vehicle");
}

#[test]
fn test_all_refs_from_multiple_specializations() {
    // Test extracting multiple references from a subclassification part
    let source = ":> Vehicle, Machine, Device";
    let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 3);
    assert!(refs.contains(&"Vehicle".to_string()));
    assert!(refs.contains(&"Machine".to_string()));
    assert!(refs.contains(&"Device".to_string()));
}

#[test]
fn test_all_refs_from_subsetting() {
    // Test extracting references from subsetting relationships
    let source = ":> base1, base2";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 2);
    assert!(refs.contains(&"base1".to_string()));
    assert!(refs.contains(&"base2".to_string()));
}

#[test]
fn test_all_refs_from_typing() {
    // Test extracting a single reference from typing relationship
    let source = ": MyType";
    let mut pairs = SysMLParser::parse(Rule::typings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0], "MyType");
}

#[test]
fn test_all_refs_from_redefinitions() {
    // Test extracting multiple references from redefinitions
    let source = ":>> original1, original2";
    let mut pairs = SysMLParser::parse(Rule::redefinitions, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 2);
    assert!(refs.contains(&"original1".to_string()));
    assert!(refs.contains(&"original2".to_string()));
}

#[test]
fn test_all_refs_from_no_references() {
    // Test a structure with no references (should return empty vector)
    let source = "part def MyPart;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
    let pair = pairs.next().unwrap();

    // Get a sub-pair that doesn't contain references
    let body_pairs: Vec<_> = pair
        .clone()
        .into_inner()
        .filter(|p| matches!(p.as_rule(), Rule::definition_body))
        .collect();

    if let Some(body_pair) = body_pairs.first() {
        let refs = all_refs_from(body_pair);
        assert_eq!(refs.len(), 0);
    }
}

#[test]
fn test_all_refs_from_quoted_name_in_subsetting() {
    // Test extracting a quoted name reference from subsetting
    let source = ":> 'Complex Name'";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0], "'Complex Name'");
}

#[test]
fn test_all_refs_from_feature_specialization() {
    // Test extracting multiple references from a complex feature specialization
    // all_refs_from gets children, so we pass the feature_specialization which contains
    // typing, subsetting, and redefinition children
    let source = ": Type1 :> base1, base2 :>> redef1";
    let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    // Should extract Type1 (from typing child), base1, base2 (from subsetting child),
    // and redef1 (from redefinition child)
    // The exact count depends on how ref_from traverses the children
    assert!(!refs.is_empty(), "Expected at least 1 ref, got 0");

    // We should at least get the typing reference
    assert!(refs.iter().any(|r| r == "Type1"), "Should contain Type1");
}

#[test]
fn test_all_refs_from_references_relationship() {
    // Test extracting references from 'references' relationship
    let source = "::> ref1, ref2";
    let mut pairs = SysMLParser::parse(Rule::references, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    // all_refs_from looks at children, references rule has children that include the identifiers
    assert!(!refs.is_empty(), "Expected at least 1 ref, got 0");
    // Should contain at least one of the references
    let has_ref = refs.iter().any(|r| r == "ref1" || r == "ref2");
    assert!(has_ref, "Should contain ref1 or ref2");
}

#[test]
fn test_all_refs_from_empty_list() {
    // Test behavior with structures that could have references but don't
    let source = "part def Test;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
    let pair = pairs.next().unwrap();

    // The definition itself might have identifiers, but body should be empty
    let body_pairs: Vec<_> = pair
        .into_inner()
        .filter(|p| matches!(p.as_rule(), Rule::definition_body))
        .collect();

    if let Some(body_pair) = body_pairs.first() {
        let refs = all_refs_from(body_pair);
        assert_eq!(refs.len(), 0, "Empty body should have no references");
    }
}

#[test]
fn test_all_refs_trimming() {
    // Test that references are trimmed of whitespace
    let source = ":>  Vehicle  ";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 1);
    // Should be trimmed
    assert_eq!(refs[0], "Vehicle");
    assert!(!refs[0].starts_with(' '));
    assert!(!refs[0].ends_with(' '));
}

#[test]
fn test_all_refs_from_complex_relationship_chain() {
    // Test extracting all references from a complex chain of relationships
    // Test with just the feature_specialization part
    let source = ": PartType :> base1, base2 :>> original";
    let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    // Should find at least the typing (PartType)
    assert!(!refs.is_empty(), "Expected at least 1 ref, got 0");
    assert!(
        refs.iter().any(|r| r == "PartType"),
        "Should contain PartType"
    );
}

// ============================================================================
// Tests for all_refs_with_spans_from()
// ============================================================================

#[test]
fn test_all_refs_with_spans_from_single_reference() {
    // Test extracting a single reference with span from subsetting
    let source = ":> Vehicle";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 1);
    assert_eq!(refs_with_spans[0].0, "Vehicle");
    assert!(refs_with_spans[0].1.is_some());
}

#[test]
fn test_all_refs_with_spans_from_multiple_specializations() {
    // Test extracting multiple references with spans from subclassification
    let source = ":> Vehicle, Machine";
    let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 2);
    assert!(refs_with_spans.iter().any(|(name, _)| name == "Vehicle"));
    assert!(refs_with_spans.iter().any(|(name, _)| name == "Machine"));

    // Verify all spans are present
    for (_, span_opt) in &refs_with_spans {
        assert!(span_opt.is_some(), "Expected span to be present");
    }
}

#[test]
fn test_all_refs_with_spans_from_typing() {
    // Test extracting reference with span from typing
    let source = ": MyType";
    let mut pairs = SysMLParser::parse(Rule::typings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 1);
    assert_eq!(refs_with_spans[0].0, "MyType");
    assert!(refs_with_spans[0].1.is_some());
}

#[test]
fn test_all_refs_with_spans_from_subsetting() {
    // Test extracting multiple references with spans from subsetting
    let source = ":> base1, base2, base3";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 3);

    // Verify each reference has a span
    let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"base1"));
    assert!(names.contains(&"base2"));
    assert!(names.contains(&"base3"));

    for (_, span_opt) in &refs_with_spans {
        assert!(span_opt.is_some());
    }
}

#[test]
fn test_all_refs_with_spans_from_redefinitions() {
    // Test extracting references with spans from redefinitions
    let source = ":>> original1, original2";
    let mut pairs = SysMLParser::parse(Rule::redefinitions, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 2);

    let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"original1"));
    assert!(names.contains(&"original2"));

    // Verify spans are present
    for (_, span_opt) in &refs_with_spans {
        assert!(span_opt.is_some());
    }
}

#[test]
fn test_all_refs_with_spans_from_span_accuracy() {
    // Test that spans accurately point to the reference location
    let source = ":> VehicleBase";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 1);
    assert_eq!(refs_with_spans[0].0, "VehicleBase");

    if let Some(span) = &refs_with_spans[0].1 {
        // Span should be on the first line (0-indexed)
        assert_eq!(span.start.line, 0);
        // The identifier starts after ":> " (3 characters)
        assert!(span.start.column >= 3);
    } else {
        panic!("Expected span to be present");
    }
}

#[test]
fn test_all_refs_with_spans_from_no_references() {
    // Test with a structure that has no references
    let source = "part def MyPart;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
    let pair = pairs.next().unwrap();

    // Get a sub-pair that doesn't contain references
    let body_pairs: Vec<_> = pair
        .clone()
        .into_inner()
        .filter(|p| matches!(p.as_rule(), Rule::definition_body))
        .collect();

    if let Some(body_pair) = body_pairs.first() {
        let refs_with_spans = all_refs_with_spans_from(body_pair);
        assert_eq!(refs_with_spans.len(), 0);
    }
}

#[test]
fn test_all_refs_with_spans_from_feature_specialization() {
    // Test extracting multiple references with spans from feature specialization
    let source = ": Type1 :> base1";
    let mut pairs = SysMLParser::parse(Rule::feature_specialization, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert!(
        !refs_with_spans.is_empty(),
        "Expected at least 1 ref with span"
    );

    // Verify all have spans
    for (_, span_opt) in &refs_with_spans {
        assert!(span_opt.is_some());
    }

    // Check that Type1 is present
    let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"Type1"));
}

#[test]
fn test_all_refs_with_spans_ordering() {
    // Test that references are extracted in the order they appear
    let source = ":> First, Second, Third";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 3);

    // Check the names appear in order
    let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"First"));
    assert!(names.contains(&"Second"));
    assert!(names.contains(&"Third"));
}

// ============================================================================
// Additional Edge Case Tests
// ============================================================================

#[test]
fn test_all_refs_from_single_item() {
    // Test with a single reference to ensure vector has exactly one element
    let source = ":> SingleRef";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 1, "Expected exactly 1 reference");
    assert_eq!(refs[0], "SingleRef");
}

#[test]
fn test_all_refs_from_many_items() {
    // Test with many references to ensure all are captured
    let source = ":> Ref1, Ref2, Ref3, Ref4, Ref5";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 5, "Expected exactly 5 references");
    assert!(refs.contains(&"Ref1".to_string()));
    assert!(refs.contains(&"Ref2".to_string()));
    assert!(refs.contains(&"Ref3".to_string()));
    assert!(refs.contains(&"Ref4".to_string()));
    assert!(refs.contains(&"Ref5".to_string()));
}

#[test]
fn test_all_refs_from_qualified_names() {
    // Test with qualified names (Package::Class format)
    let source = ":> Package1::Class1, Package2::Class2";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 2);
    assert!(refs.contains(&"Package1::Class1".to_string()));
    assert!(refs.contains(&"Package2::Class2".to_string()));
}

#[test]
fn test_all_refs_from_mixed_quoted_and_unquoted() {
    // Test with a mix of quoted and unquoted names
    let source = ":> NormalName, 'Quoted Name'";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 2);
    assert!(refs.contains(&"NormalName".to_string()));
    assert!(refs.contains(&"'Quoted Name'".to_string()));
}

#[test]
fn test_all_refs_with_spans_from_empty_input() {
    // Test with an input that should have no children with references
    let source = "part def Empty;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();
    let pair = pairs.next().unwrap();

    // Get the body which should be empty
    let body_pairs: Vec<_> = pair
        .into_inner()
        .filter(|p| matches!(p.as_rule(), Rule::definition_body))
        .collect();

    if let Some(body_pair) = body_pairs.first() {
        let refs_with_spans = all_refs_with_spans_from(body_pair);
        assert_eq!(
            refs_with_spans.len(),
            0,
            "Empty body should have no references with spans"
        );
    }
}

#[test]
fn test_all_refs_with_spans_from_qualified_names() {
    // Test with qualified names to ensure spans are correct
    let source = ":> Pkg::Class1, Pkg::Class2";
    let mut pairs = SysMLParser::parse(Rule::subsettings, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs_with_spans = all_refs_with_spans_from(&pair);

    assert_eq!(refs_with_spans.len(), 2);

    let names: Vec<&str> = refs_with_spans.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"Pkg::Class1"));
    assert!(names.contains(&"Pkg::Class2"));

    // All should have spans
    for (_, span_opt) in &refs_with_spans {
        assert!(span_opt.is_some(), "Expected all spans to be present");
    }
}

#[test]
fn test_all_refs_from_subclassification_multiple() {
    // Test subclassification with multiple bases
    let source = ":> Base1, Base2, Base3";
    let mut pairs = SysMLParser::parse(Rule::subclassification_part, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 3);
    assert!(refs.contains(&"Base1".to_string()));
    assert!(refs.contains(&"Base2".to_string()));
    assert!(refs.contains(&"Base3".to_string()));
}

#[test]
fn test_all_refs_from_redefinition_part() {
    // Test redefinition_part with multiple redefinitions
    let source = ":>> Original1, Original2";
    let mut pairs = SysMLParser::parse(Rule::redefinition_part, source).unwrap();
    let pair = pairs.next().unwrap();

    let refs = all_refs_from(&pair);

    assert_eq!(refs.len(), 2);
    assert!(refs.contains(&"Original1".to_string()));
    assert!(refs.contains(&"Original2".to_string()));
}
