//! Tests for the Rowan-based formatter

use super::{FormatOptions, format_async};
use tokio_util::sync::CancellationToken;

/// Synchronous format helper for tests
fn format(source: &str, options: &FormatOptions) -> String {
    format_async(source, options, &CancellationToken::new()).unwrap_or_default()
}

/// Assert that formatting produces the expected output (single line)
fn assert_format(input: &str, expected: &str) {
    let result = format(input, &FormatOptions::default());
    assert_eq!(
        result.trim(),
        expected,
        "\nInput:    |{}|\nExpected: |{}|\nGot:      |{}|",
        input,
        expected,
        result.trim()
    );
}

/// Assert multiline formatting produces expected output
fn assert_format_multiline(input: &str, expected: &str) {
    let result = format(input, &FormatOptions::default());
    assert_eq!(
        result.trim(),
        expected.trim(),
        "\n=== Input ===\n{}\n=== Expected ===\n{}\n=== Got ===\n{}",
        input,
        expected.trim(),
        result.trim()
    );
}

// ============================================================================
// Basic formatting tests
// ============================================================================

#[test]
fn test_format_simple_package() {
    assert_format("package Test { }", "package Test { }");
}

#[test]
fn test_format_normalizes_spaces() {
    assert_format("package   Test   {   }", "package Test { }");
}

#[test]
fn test_format_part_definition() {
    assert_format("part def Vehicle { }", "part def Vehicle { }");
    assert_format("part def   Vehicle   {  }", "part def Vehicle { }");
}

#[test]
fn test_format_part_usage() {
    assert_format("part myPart ;", "part myPart ;");
    assert_format("part   myPart  ;", "part myPart ;");
}

#[test]
fn test_format_empty_input() {
    assert_format("", "");
}

#[test]
fn test_format_whitespace_only() {
    let result = format("   \n\n   ", &FormatOptions::default());
    assert!(result.trim().is_empty());
}

// ============================================================================
// Definition tests - verify exact formatting
// ============================================================================

#[test]
fn test_format_action_definition() {
    assert_format("action def   Drive  { }", "action def Drive { }");
}

#[test]
fn test_format_action_usage() {
    assert_format("action   drive  ;", "action drive ;");
}

#[test]
fn test_format_attribute_with_type() {
    assert_format(
        "attribute   speed :   Integer  ;",
        "attribute speed : Integer ;",
    );
}

#[test]
fn test_format_attribute_with_default_value() {
    assert_format(
        "attribute count : Integer = 5 ;",
        "attribute count : Integer = 5 ;",
    );
}

#[test]
fn test_format_readonly_attribute() {
    assert_format(
        "readonly  attribute   id : String ;",
        "readonly attribute id : String ;",
    );
}

#[test]
fn test_format_port_definition() {
    assert_format("port def   FuelPort  { }", "port def FuelPort { }");
}

#[test]
fn test_format_port_usage_in() {
    assert_format("in port   fuel :  FuelPort  ;", "in port fuel : FuelPort ;");
}

#[test]
fn test_format_port_usage_out() {
    assert_format(
        "out   port   exhaust : ExhaustPort ;",
        "out port exhaust : ExhaustPort ;",
    );
}

#[test]
fn test_format_port_usage_inout() {
    assert_format(
        "inout  port  data  : DataPort ;",
        "inout port data : DataPort ;",
    );
}

#[test]
fn test_format_state_definition() {
    assert_format(
        "state def   VehicleState  { }",
        "state def VehicleState { }",
    );
}

#[test]
fn test_format_connection_definition() {
    assert_format(
        "connection def   FuelConnection  { }",
        "connection def FuelConnection { }",
    );
}

#[test]
fn test_format_flow_connection_definition() {
    assert_format(
        "flow connection def   FuelFlow  { }",
        "flow connection def FuelFlow { }",
    );
}

#[test]
fn test_format_requirement_definition() {
    assert_format(
        "requirement def   SafetyReq  { }",
        "requirement def SafetyReq { }",
    );
}

#[test]
fn test_format_constraint_definition() {
    assert_format(
        "constraint def   SpeedLimit  { }",
        "constraint def SpeedLimit { }",
    );
}

#[test]
fn test_format_interface_definition() {
    assert_format(
        "interface def   FuelInterface  { }",
        "interface def FuelInterface { }",
    );
}

#[test]
fn test_format_allocation_definition() {
    assert_format(
        "allocation def   SoftwareAllocation  { }",
        "allocation def SoftwareAllocation { }",
    );
}

#[test]
fn test_format_item_definition() {
    assert_format("item def   Fuel  { }", "item def Fuel { }");
}

#[test]
fn test_format_item_usage() {
    assert_format("item   fuel  :  Fuel  ;", "item fuel : Fuel ;");
}

#[test]
fn test_format_enum_definition() {
    assert_format("enum def   Color  { }", "enum def Color { }");
}

#[test]
fn test_format_use_case_definition() {
    assert_format(
        "use case def   DriveVehicle  { }",
        "use case def DriveVehicle { }",
    );
}

#[test]
fn test_format_view_definition() {
    assert_format("view def   SystemView  { }", "view def SystemView { }");
}

#[test]
fn test_format_viewpoint_definition() {
    assert_format(
        "viewpoint def   SafetyViewpoint  { }",
        "viewpoint def SafetyViewpoint { }",
    );
}

#[test]
fn test_format_rendering_definition() {
    assert_format(
        "rendering def   DiagramRendering  { }",
        "rendering def DiagramRendering { }",
    );
}

#[test]
fn test_format_metadata_definition() {
    assert_format(
        "metadata def   ToolVariable  { }",
        "metadata def ToolVariable { }",
    );
}

#[test]
fn test_format_analysis_definition() {
    assert_format(
        "analysis def   TradeStudy  { }",
        "analysis def TradeStudy { }",
    );
}

#[test]
fn test_format_verification_definition() {
    assert_format(
        "verification def   SafetyTest  { }",
        "verification def SafetyTest { }",
    );
}

#[test]
fn test_format_concern_definition() {
    assert_format("concern def   Safety  { }", "concern def Safety { }");
}

#[test]
fn test_format_calc_definition() {
    assert_format("calc def   Speed  { }", "calc def Speed { }");
}

#[test]
fn test_format_individual_definition() {
    assert_format("individual def   MyCar  { }", "individual def MyCar { }");
}

#[test]
fn test_format_occurrence_definition() {
    assert_format("occurrence def   Event  { }", "occurrence def Event { }");
}

// ============================================================================
// Modifier tests
// ============================================================================

#[test]
fn test_format_abstract_part_def() {
    assert_format(
        "abstract   part def   Vehicle  { }",
        "abstract part def Vehicle { }",
    );
}

#[test]
fn test_format_ref_part() {
    assert_format(
        "ref   part   engine  :  Engine  ;",
        "ref part engine : Engine ;",
    );
}

// ============================================================================
// Relationship tests
// ============================================================================

#[test]
fn test_format_specializes() {
    assert_format(
        "part def Car   specializes   Vehicle  { }",
        "part def Car specializes Vehicle { }",
    );
}

#[test]
fn test_format_subsets() {
    assert_format(
        "part frontWheel   subsets   wheels  ;",
        "part frontWheel subsets wheels ;",
    );
}

#[test]
fn test_format_redefines() {
    assert_format(
        "attribute speed   redefines   baseSpeed  ;",
        "attribute speed redefines baseSpeed ;",
    );
}

// ============================================================================
// Import tests
// ============================================================================

#[test]
fn test_format_import_wildcard() {
    assert_format("import   Pkg :: *  ;", "import Pkg :: * ;");
}

#[test]
fn test_format_import_specific() {
    assert_format("import   Pkg :: Element  ;", "import Pkg :: Element ;");
}

#[test]
fn test_format_import_nested_path() {
    assert_format("import   A :: B :: C :: *  ;", "import A :: B :: C :: * ;");
}

#[test]
fn test_format_import_already_formatted() {
    assert_format("import Pkg :: * ;", "import Pkg :: * ;");
}

// ============================================================================
// Alias tests
// ============================================================================

#[test]
fn test_format_alias() {
    assert_format("alias   V   for   Vehicle  ;", "alias V for Vehicle ;");
}

// ============================================================================
// Comment preservation tests
// ============================================================================

#[test]
fn test_format_preserves_line_comments() {
    assert_format_multiline(
        "// This is a comment\npackage Test { }",
        "// This is a comment\npackage Test { }",
    );
}

#[test]
fn test_format_preserves_block_comments() {
    assert_format_multiline(
        "/* Block comment */\npackage Test { }",
        "/* Block comment */\npackage Test { }",
    );
}

#[test]
fn test_format_preserves_inline_comments() {
    assert_format_multiline(
        "package Test { // inline comment\n}",
        "package Test { // inline comment\n}",
    );
}

#[test]
fn test_format_comment_only() {
    assert_format("// Just a comment", "// Just a comment");
}

#[test]
fn test_format_comment_between_tokens() {
    assert_format(
        "part /* inline */ def Vehicle { }",
        "part /* inline */ def Vehicle { }",
    );
}

#[test]
fn test_format_multiple_line_comments() {
    assert_format_multiline(
        "// Comment 1\n// Comment 2\n// Comment 3\npackage Test { }",
        "// Comment 1\n// Comment 2\n// Comment 3\npackage Test { }",
    );
}

#[test]
fn test_format_trailing_comment_on_brace() {
    assert_format_multiline(
        "package Test { // opening brace comment\n}",
        "package Test { // opening brace comment\n}",
    );
}

#[test]
fn test_format_comment_after_semicolon() {
    assert_format("part x ; // end of part", "part x ; // end of part");
}

#[test]
fn test_format_doc_comment() {
    assert_format_multiline(
        "doc /* Documentation */\npackage Test { }",
        "doc /* Documentation */\npackage Test { }",
    );
}

#[test]
fn test_format_multiline_block_comment() {
    assert_format_multiline(
        "/*\n * Multiline\n * comment\n */\npackage Test { }",
        "/*\n * Multiline\n * comment\n */\npackage Test { }",
    );
}

// ============================================================================
// Brace formatting tests
// ============================================================================

#[test]
fn test_format_brace_on_next_line() {
    assert_format(
        "metadata def ToolVariable\n{",
        "metadata def ToolVariable {",
    );
}

#[test]
fn test_format_brace_with_body() {
    assert_format_multiline(
        "metadata def ToolVariable\n\t{\n\t\tattribute name : String ;\n\t}",
        "metadata def ToolVariable {\n    attribute name : String ;\n}",
    );
}

// ============================================================================
// Multiline formatting tests
// ============================================================================

#[test]
fn test_format_multiline_package_with_parts() {
    assert_format_multiline(
        "package A {\n    part x ;\n    part y ;\n}",
        "package A {\n    part x ;\n    part y ;\n}",
    );
}

#[test]
fn test_format_multiline_enum_with_values() {
    assert_format_multiline(
        "enum def Color {\n    enum Red ;\n    enum Green ;\n    enum Blue ;\n}",
        "enum def Color {\n    enum Red ;\n    enum Green ;\n    enum Blue ;\n}",
    );
}

#[test]
fn test_format_multiline_nested_package() {
    assert_format_multiline(
        "package Outer {\n    package Inner {\n        part x ;\n    }\n}",
        "package Outer {\n    package Inner {\n        part x ;\n    }\n}",
    );
}

#[test]
fn test_format_multiline_part_def_with_attributes() {
    assert_format_multiline(
        "part def Car {\n    attribute wheels : Integer ;\n    attribute color : String ;\n}",
        "part def Car {\n    attribute wheels : Integer ;\n    attribute color : String ;\n}",
    );
}

#[test]
fn test_format_multiline_normalizes_inner_spaces() {
    assert_format_multiline(
        "package A {\n    part   x  :  Type  ;\n}",
        "package A {\n    part x : Type ;\n}",
    );
}

#[test]
fn test_format_multiline_requirement_with_doc() {
    assert_format_multiline(
        "requirement def SafetyReq {\n    doc /* The system shall be safe. */\n}",
        "requirement def SafetyReq {\n    doc /* The system shall be safe. */\n}",
    );
}

// ============================================================================
// Options tests
// ============================================================================

#[test]
fn test_format_options_default() {
    let options = FormatOptions::default();
    assert_eq!(options.tab_size, 4);
    assert!(options.insert_spaces);
    assert_eq!(options.print_width, 80);
}

#[test]
fn test_format_with_tabs_option() {
    let source = "package Test { part x ; }";
    let options = FormatOptions {
        tab_size: 4,
        insert_spaces: false,
        print_width: 80,
    };
    let result = format(source, &options);
    assert_eq!(result.trim(), "package Test { part x ; }");
}

#[test]
fn test_format_multiline_uses_spaces() {
    let source = "package Test {\n    part x ;\n}";
    let result = format(source, &FormatOptions::default());
    // Find indented line
    let indented: Option<&str> = result.lines().find(|l| l.starts_with(' '));
    if let Some(line) = indented {
        assert!(!line.starts_with('\t'), "Should use spaces, not tabs");
    }
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_format_package_semicolon() {
    assert_format("package Test ;", "package Test ;");
}

#[test]
fn test_format_preserves_identifier_case() {
    assert_format(
        "part def MyPart_v2_FINAL { }",
        "part def MyPart_v2_FINAL { }",
    );
}

#[test]
fn test_format_unicode_identifiers() {
    assert_format("part def Véhicule { }", "part def Véhicule { }");
}

#[test]
fn test_format_multiplicity() {
    assert_format(
        "attribute count [ 0 .. 10 ] ;",
        "attribute count [ 0 .. 10 ] ;",
    );
}

#[test]
fn test_format_idempotent_single_line() {
    let sources = [
        "package Test { part x ; }",
        "part def Vehicle { }",
        "import Pkg :: * ;",
        "attribute speed : Integer ;",
    ];

    for source in sources {
        let first = format(source, &FormatOptions::default());
        let second = format(&first, &FormatOptions::default());
        assert_eq!(
            first, second,
            "Formatting should be idempotent for: {source}\nFirst:  |{first}|\nSecond: |{second}|"
        );
    }
}

#[test]
fn test_format_idempotent_multiline() {
    let sources = [
        "package A {\n    part x ;\n}",
        "enum def Color {\n    enum Red ;\n}",
        "part def Car {\n    attribute wheels : Integer ;\n}",
    ];

    for source in sources {
        let first = format(source, &FormatOptions::default());
        let second = format(&first, &FormatOptions::default());
        assert_eq!(
            first, second,
            "Formatting should be idempotent for multiline:\n{source}\nFirst:\n{first}\nSecond:\n{second}"
        );
    }
}

// ============================================================================
// Complex/real-world tests
// ============================================================================

#[test]
fn test_format_complex_file() {
    let input = "// File header comment\npackage Vehicle {\n    // Part comment\n    part def Car {\n        attribute wheels : Integer ;\n    }\n    \n    part myCar : Car ;\n}";
    let expected = "// File header comment\npackage Vehicle {\n    // Part comment\n    part def Car {\n        attribute wheels : Integer ;\n    }\n\n    part myCar : Car ;\n}";
    assert_format_multiline(input, expected);
}

#[test]
fn test_format_real_world_example() {
    // Note: formatter normalizes whitespace-only lines to empty lines
    let input = r#"package VehicleSystem {
    import SI :: * ;
    
    // Base vehicle definition
    abstract part def Vehicle {
        attribute mass : MassValue ;
        attribute maxSpeed : SpeedValue ;
        
        port fuelIn : FuelPort ;
    }
    
    part def Car specializes Vehicle {
        attribute wheels : Integer = 4 ;
        part engine : Engine ;
    }
}"#;
    let expected = "package VehicleSystem {\n    import SI :: * ;\n\n    // Base vehicle definition\n    abstract part def Vehicle {\n        attribute mass : MassValue ;\n        attribute maxSpeed : SpeedValue ;\n\n        port fuelIn : FuelPort ;\n    }\n\n    part def Car specializes Vehicle {\n        attribute wheels : Integer = 4 ;\n        part engine : Engine ;\n    }\n}";
    assert_format_multiline(input, expected);
}
