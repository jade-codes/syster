use crate::server::helpers::format_rich_hover;
use syster::core::{Position, Span};
use syster::semantic::Workspace;
use syster::semantic::symbol_table::Symbol;
use syster::syntax::SyntaxFile;

/// Helper to create a span for testing
fn test_span() -> Span {
    Span {
        start: Position { line: 0, column: 0 },
        end: Position {
            line: 0,
            column: 10,
        },
    }
}

// Tests for Symbol::Alias formatting
#[test]
fn test_format_alias_basic() {
    let symbol = Symbol::Alias {
        name: "MyAlias".to_string(),
        qualified_name: "Package::MyAlias".to_string(),
        target: "TargetType".to_string(),
        target_span: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Should contain the alias declaration format: "alias MyAlias for TargetType"
    assert!(
        result.contains("alias MyAlias for TargetType"),
        "Expected alias format 'alias MyAlias for TargetType', got: {result}"
    );
}

#[test]
fn test_format_alias_with_qualified_target() {
    let symbol = Symbol::Alias {
        name: "ShortName".to_string(),
        qualified_name: "Package::ShortName".to_string(),
        target: "Other::Package::LongQualifiedName".to_string(),
        target_span: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("alias ShortName for Other::Package::LongQualifiedName"),
        "Expected qualified target in alias, got: {result}"
    );
}

// Tests for Symbol::Package formatting
#[test]
fn test_format_package_basic() {
    let symbol = Symbol::Package {
        name: "MyPackage".to_string(),
        qualified_name: "MyPackage".to_string(),
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("package MyPackage"),
        "Expected package format 'package MyPackage', got: {result}"
    );
}

#[test]
fn test_format_package_nested() {
    let symbol = Symbol::Package {
        name: "InnerPackage".to_string(),
        qualified_name: "Outer::InnerPackage".to_string(),
        scope_id: 1,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Declaration shows simple name
    assert!(
        result.contains("package InnerPackage"),
        "Expected simple name in package declaration, got: {result}"
    );
    // But qualified name should appear elsewhere in hover
    assert!(
        result.contains("Outer::InnerPackage"),
        "Expected qualified name in hover info, got: {result}"
    );
}

// Tests for Symbol::Classifier formatting
#[test]
fn test_format_classifier_basic() {
    let symbol = Symbol::Classifier {
        name: "Vehicle".to_string(),
        qualified_name: "Package::Vehicle".to_string(),
        kind: "class".to_string(),
        is_abstract: false,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("classifier Vehicle"),
        "Expected classifier format 'classifier Vehicle', got: {result}"
    );
}

#[test]
fn test_format_classifier_abstract() {
    let symbol = Symbol::Classifier {
        name: "AbstractBase".to_string(),
        qualified_name: "Package::AbstractBase".to_string(),
        kind: "abstract class".to_string(),
        is_abstract: true,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Note: format_symbol_declaration shows name only, not kind or abstract flag
    assert!(
        result.contains("classifier AbstractBase"),
        "Expected classifier format with name, got: {result}"
    );
}

// Tests for Symbol::Definition formatting
#[test]
fn test_format_definition_part() {
    let symbol = Symbol::Definition {
        name: "Vehicle".to_string(),
        qualified_name: "Package::Vehicle".to_string(),
        kind: "Part".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Part def Vehicle"),
        "Expected definition format 'Part def Vehicle', got: {result}"
    );
}

#[test]
fn test_format_definition_attribute() {
    let symbol = Symbol::Definition {
        name: "Speed".to_string(),
        qualified_name: "Package::Speed".to_string(),
        kind: "Attribute".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Attribute def Speed"),
        "Expected definition format 'Attribute def Speed', got: {result}"
    );
}

#[test]
fn test_format_definition_requirement() {
    let symbol = Symbol::Definition {
        name: "SafetyRequirement".to_string(),
        qualified_name: "Requirements::SafetyRequirement".to_string(),
        kind: "Requirement".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Requirement def SafetyRequirement"),
        "Expected definition format 'Requirement def SafetyRequirement', got: {result}"
    );
}

#[test]
fn test_format_definition_action() {
    let symbol = Symbol::Definition {
        name: "StartEngine".to_string(),
        qualified_name: "Actions::StartEngine".to_string(),
        kind: "Action".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Action def StartEngine"),
        "Expected definition format 'Action def StartEngine', got: {result}"
    );
}

#[test]
fn test_format_definition_port() {
    let symbol = Symbol::Definition {
        name: "DataPort".to_string(),
        qualified_name: "Ports::DataPort".to_string(),
        kind: "Port".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Port def DataPort"),
        "Expected definition format 'Port def DataPort', got: {result}"
    );
}

// Tests for Symbol::Usage formatting
#[test]
fn test_format_usage_part() {
    let symbol = Symbol::Usage {
        name: "engine".to_string(),
        qualified_name: "Car::engine".to_string(),
        kind: "Part".to_string(),
        semantic_role: None,
        usage_type: Some("Engine".to_string()),
        scope_id: 1,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Part engine"),
        "Expected usage format 'Part engine', got: {result}"
    );
}

#[test]
fn test_format_usage_attribute() {
    let symbol = Symbol::Usage {
        name: "speed".to_string(),
        qualified_name: "Vehicle::speed".to_string(),
        kind: "Attribute".to_string(),
        semantic_role: None,
        usage_type: Some("Real".to_string()),
        scope_id: 1,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Attribute speed"),
        "Expected usage format 'Attribute speed', got: {result}"
    );
}

#[test]
fn test_format_usage_action() {
    let symbol = Symbol::Usage {
        name: "startAction".to_string(),
        qualified_name: "System::startAction".to_string(),
        kind: "Action".to_string(),
        semantic_role: None,
        usage_type: Some("StartEngine".to_string()),
        scope_id: 1,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Action startAction"),
        "Expected usage format 'Action startAction', got: {result}"
    );
}

#[test]
fn test_format_usage_port() {
    let symbol = Symbol::Usage {
        name: "inputPort".to_string(),
        qualified_name: "Component::inputPort".to_string(),
        kind: "Port".to_string(),
        semantic_role: None,
        usage_type: Some("DataPort".to_string()),
        scope_id: 1,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Port inputPort"),
        "Expected usage format 'Port inputPort', got: {result}"
    );
}

#[test]
fn test_format_usage_requirement() {
    let symbol = Symbol::Usage {
        name: "req1".to_string(),
        qualified_name: "System::req1".to_string(),
        kind: "Requirement".to_string(),
        semantic_role: None,
        usage_type: Some("SafetyRequirement".to_string()),
        scope_id: 1,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Requirement req1"),
        "Expected usage format 'Requirement req1', got: {result}"
    );
}

// Tests for Symbol::Feature formatting
#[test]
fn test_format_feature_without_type() {
    let symbol = Symbol::Feature {
        name: "property1".to_string(),
        qualified_name: "Class::property1".to_string(),
        scope_id: 1,
        feature_type: None,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("feature property1"),
        "Expected feature format without type 'feature property1', got: {result}"
    );
    // Should NOT have a colon when there's no type
    assert!(
        !result.contains("feature property1:"),
        "Feature without type should not have colon, got: {result}"
    );
}

#[test]
fn test_format_feature_with_type() {
    let symbol = Symbol::Feature {
        name: "property2".to_string(),
        qualified_name: "Class::property2".to_string(),
        scope_id: 1,
        feature_type: Some("Integer".to_string()),
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("feature property2: Integer"),
        "Expected feature format with type 'feature property2: Integer', got: {result}"
    );
}

#[test]
fn test_format_feature_with_qualified_type() {
    let symbol = Symbol::Feature {
        name: "speed".to_string(),
        qualified_name: "Vehicle::speed".to_string(),
        scope_id: 1,
        feature_type: Some("Types::Real".to_string()),
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("feature speed: Types::Real"),
        "Expected feature with qualified type, got: {result}"
    );
}

// Edge case tests
#[test]
fn test_format_with_empty_name() {
    let symbol = Symbol::Package {
        name: "".to_string(),
        qualified_name: "".to_string(),
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Should handle empty names gracefully
    assert!(
        result.contains("package "),
        "Should format empty package name, got: {result}"
    );
}

#[test]
fn test_format_with_special_characters() {
    let symbol = Symbol::Definition {
        name: "My_Special-Name123".to_string(),
        qualified_name: "Package::My_Special-Name123".to_string(),
        kind: "Part".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("Part def My_Special-Name123"),
        "Should handle special characters in names, got: {result}"
    );
}

#[test]
fn test_format_with_very_long_name() {
    let long_name = "VeryLongNameThatExceedsNormalLimitsAndMightCauseIssuesInFormattingOrDisplay";
    let symbol = Symbol::Usage {
        name: long_name.to_string(),
        qualified_name: format!("Package::{long_name}"),
        kind: "Part".to_string(),
        semantic_role: None,
        usage_type: Some("SomeType".to_string()),
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains(&format!("Part {long_name}")),
        "Should handle very long names, got: {result}"
    );
}

#[test]
fn test_format_with_unicode_characters() {
    let symbol = Symbol::Package {
        name: "包裹".to_string(), // Chinese characters for "package"
        qualified_name: "包裹".to_string(),
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("package 包裹"),
        "Should handle Unicode characters, got: {result}"
    );
}

#[test]
fn test_format_feature_type_with_spaces() {
    let symbol = Symbol::Feature {
        name: "value".to_string(),
        qualified_name: "Class::value".to_string(),
        scope_id: 1,
        feature_type: Some("Some Complex Type".to_string()),
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("feature value: Some Complex Type"),
        "Should handle types with spaces, got: {result}"
    );
}

#[test]
fn test_format_alias_with_empty_target() {
    let symbol = Symbol::Alias {
        name: "BrokenAlias".to_string(),
        qualified_name: "Package::BrokenAlias".to_string(),
        target: "".to_string(),
        target_span: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Should handle empty target gracefully
    assert!(
        result.contains("alias BrokenAlias for "),
        "Should format alias with empty target, got: {result}"
    );
}

#[test]
fn test_format_hover_includes_qualified_name() {
    let symbol = Symbol::Definition {
        name: "Vehicle".to_string(),
        qualified_name: "Automotive::Models::Vehicle".to_string(),
        kind: "Part".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Hover should include qualified name section
    assert!(
        result.contains("Qualified Name"),
        "Hover should include qualified name section, got: {result}"
    );
    assert!(
        result.contains("Automotive::Models::Vehicle"),
        "Hover should show full qualified name, got: {result}"
    );
}

#[test]
fn test_format_hover_includes_source_file() {
    let symbol = Symbol::Package {
        name: "TestPackage".to_string(),
        qualified_name: "TestPackage".to_string(),
        scope_id: 0,
        source_file: Some("/path/to/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Hover should include source file section
    assert!(
        result.contains("Defined in"),
        "Hover should include 'Defined in' section, got: {result}"
    );
    assert!(
        result.contains("/path/to/test.sysml"),
        "Hover should show source file path, got: {result}"
    );
}

#[test]
fn test_format_hover_without_source_file() {
    let symbol = Symbol::Package {
        name: "GeneratedPackage".to_string(),
        qualified_name: "GeneratedPackage".to_string(),
        scope_id: 0,
        source_file: None,
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Should handle missing source file gracefully
    assert!(
        result.contains("package GeneratedPackage"),
        "Should still format declaration without source file, got: {result}"
    );
}

#[test]
fn test_format_hover_markdown_structure() {
    let symbol = Symbol::Definition {
        name: "TestDef".to_string(),
        qualified_name: "TestDef".to_string(),
        kind: "Part".to_string(),
        semantic_role: None,
        scope_id: 0,
        source_file: Some("/test.sysml".to_string()),
        span: Some(test_span()),
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    // Verify markdown structure
    assert!(
        result.contains("```sysml"),
        "Hover should have SysML code block start, got: {result}"
    );
    assert!(
        result.contains("```"),
        "Hover should have code block end, got: {result}"
    );
    assert!(
        result.contains("**Qualified Name:**"),
        "Hover should have bold qualified name header, got: {result}"
    );
    assert!(
        result.contains("**Defined in:**"),
        "Hover should have bold source file header, got: {result}"
    );
}

#[test]
fn test_format_all_symbol_variants_produce_output() {
    // This test ensures all symbol variants produce non-empty output
    let workspace = Workspace::<SyntaxFile>::new();

    let symbols = vec![
        Symbol::Alias {
            name: "A".to_string(),
            qualified_name: "A".to_string(),
            target: "B".to_string(),
            target_span: None,
            scope_id: 0,
            source_file: None,
            span: None,
        },
        Symbol::Package {
            name: "P".to_string(),
            qualified_name: "P".to_string(),
            scope_id: 0,
            source_file: None,
            span: None,
        },
        Symbol::Classifier {
            name: "C".to_string(),
            qualified_name: "C".to_string(),
            kind: "class".to_string(),
            is_abstract: false,
            scope_id: 0,
            source_file: None,
            span: None,
        },
        Symbol::Definition {
            name: "D".to_string(),
            qualified_name: "D".to_string(),
            kind: "Part".to_string(),
            semantic_role: None,
            scope_id: 0,
            source_file: None,
            span: None,
        },
        Symbol::Usage {
            name: "U".to_string(),
            qualified_name: "U".to_string(),
            kind: "Part".to_string(),
            semantic_role: None,
            usage_type: None,
            scope_id: 0,
            source_file: None,
            span: None,
        },
        Symbol::Feature {
            name: "F".to_string(),
            qualified_name: "F".to_string(),
            scope_id: 0,
            feature_type: None,
            source_file: None,
            span: None,
        },
    ];

    for symbol in symbols {
        let result = format_rich_hover(&symbol, &workspace);
        assert!(
            !result.is_empty(),
            "Hover for symbol {symbol:?} should not be empty"
        );
        assert!(
            result.len() > 10,
            "Hover for symbol {symbol:?} should have substantial content, got: {result}"
        );
    }
}

#[test]
fn test_format_definition_lowercase_kind() {
    // Test that kinds are used as-is (the function doesn't change case)
    let symbol = Symbol::Definition {
        name: "test".to_string(),
        qualified_name: "test".to_string(),
        kind: "part".to_string(), // lowercase
        semantic_role: None,
        scope_id: 0,
        source_file: None,
        span: None,
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("part def test"),
        "Should use kind as-is (lowercase), got: {result}"
    );
}

#[test]
fn test_format_usage_lowercase_kind() {
    let symbol = Symbol::Usage {
        name: "test".to_string(),
        qualified_name: "test".to_string(),
        kind: "attribute".to_string(), // lowercase
        semantic_role: None,
        usage_type: None,
        scope_id: 0,
        source_file: None,
        span: None,
    };

    let workspace = Workspace::<SyntaxFile>::new();
    let result = format_rich_hover(&symbol, &workspace);

    assert!(
        result.contains("attribute test"),
        "Should use kind as-is (lowercase), got: {result}"
    );
}
