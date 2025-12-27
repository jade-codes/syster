#![allow(clippy::unwrap_used)]

use pest::Parser;
use rstest::rstest;
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_chained_member_access() {
    let input = "fn.samples.domainValue";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse chained member access: {:?}",
        result.err()
    );
}

#[test]
fn test_instantiation_expression_with_args() {
    let input = "new SampledFunction(samples = values)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse instantiation expression: {:?}",
        result.err()
    );
}

#[test]
fn test_instantiation_expression_positional() {
    let input = "new SamplePair(x, y)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse instantiation with positional args: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_invocation_with_block() {
    let input = "list->select { in i; true }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse arrow invocation with block: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_invocation_with_block_then_index() {
    let input = "list->select { in i; true }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse arrow invocation with block followed by indexing: {:?}",
        result.err()
    );
}

#[test]
fn test_typed_parameter_in_lambda() {
    let input = "list->select { in i : Positive; domainValues#(i) <= value }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse lambda with typed parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_typed_parameter_in_lambda_then_index() {
    let input = "list->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse lambda with typed parameter followed by indexing: {:?}",
        result.err()
    );
}

#[test]
fn test_full_sampled_functions_expression() {
    let input =
        "(1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse full SampledFunctions expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_instantiation_in_collect() {
    let input = "domainValues->collect { in x; new SamplePair(x, calculation(x)) }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse collect with nested instantiation: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_complex_initializer() {
    let input = "attribute index : Positive[0..1] = (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with complex initializer: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_without_type() {
    let input = "attribute index = (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute without type: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_in_package_context() {
    let input =
        "package Test { attribute index = list->select { in i : Positive; vals#(i) <= v }#(1); }";
    let result = SysMLParser::parse(Rule::package, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute in package: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_simple_type() {
    let input = "attribute index : Positive[0..1];";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with simple type: {:?}",
        result.err()
    );
}

#[test]
fn test_feature_value_with_lambda() {
    use pest::Parser;
    use syster::parser::{SysMLParser, sysml::Rule};

    let input =
        "= (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::feature_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse feature_value: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_minimal() {
    let input = "{ vals#(i) }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse minimal calculation body: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_parameter_binding() {
    let input = "{ in i; vals#(i) }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation body with parameter binding: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_typed_parameter() {
    let input = "{ in i : Positive; vals#(i) }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation body with typed parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_parameter_declaration() {
    // This is the failing case from SampledFunctions.sysml line 53
    let input = "{ in fn : SampledFunction; }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation body with parameter declaration: {:?}",
        result.err()
    );
}

#[test]
fn test_expression_body_with_parameter() {
    let input = "{ in i; vals#(i) }";
    let result = SysMLParser::parse(Rule::expression_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse expression body with parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_parameter_binding_simple() {
    let input = "in i";
    let result = SysMLParser::parse(Rule::parameter_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse simple parameter binding: {:?}",
        result.err()
    );
}

#[test]
fn test_parameter_binding_typed() {
    let input = "in fn : SampledFunction";
    let result = SysMLParser::parse(Rule::parameter_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse typed parameter binding: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_param_and_return() {
    // Exact pattern from SampledFunctions.sysml Domain calc def
    let input = "{ in fn : SampledFunction; return : Anything[0..*] = fn.samples.domainValue; }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calc body with param and return: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_def_domain() {
    // Full Domain calc def from SampledFunctions.sysml
    let input = r#"calc def Domain {
        in fn : SampledFunction;
        return : Anything[0..*] = fn.samples.domainValue;
    }"#;
    let result = SysMLParser::parse(Rule::calculation_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse Domain calc def: {:?}",
        result.err()
    );
}

#[test]
fn test_return_parameter_member() {
    let input = "return : Anything[0..*] = fn.samples.domainValue";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return_parameter_member: {:?}",
        result.err()
    );
}

#[test]
fn test_return_parameter_member_with_semicolon() {
    let input = "return : Anything[0..*] = fn.samples.domainValue;";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    // Should consume everything EXCEPT the semicolon
    match result {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            let consumed = pair.as_str();
            assert_eq!(consumed, "return : Anything[0..*] = fn.samples.domainValue");
        }
        Err(e) => panic!("Failed to parse: {e:?}"),
    }
}

#[test]
fn test_calculation_body_item_return() {
    let input = "return : Anything[0..*] = fn.samples.domainValue;";
    let result = SysMLParser::parse(Rule::calculation_body_item, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body_item with return: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_part_with_param_and_return() {
    let input = "in fn : SampledFunction; return : Anything[0..*] = fn.samples.domainValue;";
    let result = SysMLParser::parse(Rule::calculation_body_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body_part: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_braces() {
    let input = "{ in fn : SampledFunction; return : Anything[0..*] = fn.samples.domainValue; }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body with braces: {:?}",
        result.err()
    );
}

#[rstest]
#[case("in")]
#[case("out")]
#[case("return")]
#[case("attribute")]
#[case("calc")]
#[case("action")]
fn test_identifier_excludes_keywords(#[case] keyword: &str) {
    let result = SysMLParser::parse(Rule::identifier, keyword);
    assert!(
        result.is_err(),
        "Keyword '{keyword}' should not parse as identifier"
    );
}

#[rstest]
#[case("myVar")]
#[case("calculation1")]
#[case("result_value")]
#[case("InCamelCase")]
fn test_identifier_allows_valid_names(#[case] ident: &str) {
    let result = SysMLParser::parse(Rule::identifier, ident);
    assert!(
        result.is_ok(),
        "Valid identifier '{}' should parse: {:?}",
        ident,
        result.err()
    );
}

#[test]
fn test_calculation_body_item_without_semicolon() {
    // Calculation usage without trailing semicolon should parse
    let input = "in calc calculation { in x; }";
    let result = SysMLParser::parse(Rule::calculation_body_item, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body_item without trailing semicolon: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_item_attribute_with_semicolon() {
    // Attribute usage with trailing semicolon should parse
    let input = "in attribute domainValues [0..*];";
    let result = SysMLParser::parse(Rule::calculation_body_item, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute usage in calculation_body_item: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_mixed_items() {
    // Test a complete calculation body with mixed items
    let input = "{ in calc calculation { in x; } in attribute domainValues [0..*]; return sampling = value; }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body with mixed items: {:?}",
        result.err()
    );
}

#[test]
fn test_return_parameter_member_with_name() {
    // Return with identifier name
    let input = "return sampling = new SampledFunction()";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return_parameter_member with name: {:?}",
        result.err()
    );
}

#[test]
fn test_return_parameter_member_with_name_and_type() {
    // Return with identifier, type, and value
    let input = "return result: StateSpace = value";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return_parameter_member with name and type: {:?}",
        result.err()
    );
}

#[test]
fn test_return_attribute_member() {
    // Return attribute without value
    let input = "return attribute result";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return attribute member: {:?}",
        result.err()
    );
}

#[test]
fn test_return_attribute_member_with_type() {
    // Return attribute with type and body
    let input = "return attribute result : ScalarValue[1]";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return attribute member with type: {:?}",
        result.err()
    );
}

#[rstest]
#[case("a <= b", "<=")]
#[case("a >= b", ">=")]
#[case("a < b", "<")]
#[case("a > b", ">")]
fn test_relational_operators_ordering(#[case] input: &str, #[case] operator: &str) {
    let result = SysMLParser::parse(Rule::relational_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {} operator: {:?}",
        operator,
        result.err()
    );
}

#[test]
fn test_return_attribute_with_body() {
    let input = r#"return attribute result : ScalarValue[1] {
        doc
        /*
         * A comment
         */
    }"#;
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return attribute with body: {:?}",
        result.err()
    );
}

#[test]
fn test_complex_relational_expression() {
    // Test complex expression with collection indexing and relational operator
    let input = "domainValues#(i) <= value";
    let result = SysMLParser::parse(Rule::relational_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse complex relational expression: {:?}",
        result.err()
    );
}

#[rstest]
#[case("a == b", "==")]
#[case("a === b", "===")]
#[case("a != b", "!=")]
#[case("a !== b", "!==")]
fn test_equality_operators(#[case] input: &str, #[case] operator: &str) {
    let result = SysMLParser::parse(Rule::equality_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {} operator: {:?}",
        operator,
        result.err()
    );
}

#[test]
fn test_equality_expression_with_member_access() {
    let input = "stateSpace.order == order";
    let result = SysMLParser::parse(Rule::equality_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse equality expression with member access: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_usage_with_expression() {
    let input = "constraint { stateSpace.order == order }";
    let result = SysMLParser::parse(Rule::constraint_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse constraint usage with expression: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_part_simple_expression() {
    let input = "a == b";
    let result = SysMLParser::parse(Rule::calculation_body_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse simple expression as calculation_body_part: {:?}",
        result.err()
    );
}

#[test]
fn test_action_body_item_identifier() {
    // This should fail or match as a usage
    let input = "a";
    let _result = SysMLParser::parse(Rule::action_body_item, input);
}

#[test]
fn test_calculation_body_item_identifier() {
    // This should fail or match as a usage
    let input = "a";
    let _result = SysMLParser::parse(Rule::calculation_body_item, input);
}

#[test]
fn test_calculation_body_with_simple_expression() {
    let input = "{ a == b }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation body with simple expression: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_def_with_expression_body() {
    let input = "calc def Test { a == b }";
    let result = SysMLParser::parse(Rule::calculation_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse calc def with expression body: {:?}",
        result.err()
    );
}

#[test]
fn test_conditional_expression_simple_equality() {
    let input = "a == b";
    let result = SysMLParser::parse(Rule::conditional_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse simple equality as conditional_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_owned_expression_simple_equality() {
    let input = "a == b";
    let result = SysMLParser::parse(Rule::owned_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse simple equality as owned_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_result_expression_member_simple_equality() {
    let input = "a == b";
    let result = SysMLParser::parse(Rule::result_expression_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse simple equality as result_expression_member: {:?}",
        result.err()
    );
}

#[test]
fn test_return_with_equality_expression() {
    let input = "return a == b";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return with equality expression: {:?}",
        result.err()
    );
}

#[test]
fn test_expression_body_with_doc() {
    let input = r#"{
        doc
        /*
         * Some documentation
         */
        in x; eval(x)
    }"#;
    let result = SysMLParser::parse(Rule::expression_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse expression body with doc: {:?}",
        result.err()
    );
}

#[test]
fn test_expression_body_with_ref_parameter() {
    let input = r#"{in ref a {
        doc
        /* The alternative */
    }; 
    a
    }"#;
    let result = SysMLParser::parse(Rule::expression_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse expression body with ref parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_case_body_with_doc() {
    let input = r#"{
        doc
        /*
         * A TradeStudy documentation
         */
    }"#;
    let result = SysMLParser::parse(Rule::case_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse case body with doc: {:?}",
        result.err()
    );
}

#[test]
fn test_multiplicity_with_expression() {
    let input = "[nCauses]";
    let result = SysMLParser::parse(Rule::owned_multiplicity, input);

    assert!(
        result.is_ok(),
        "Failed to parse multiplicity with expression: {:?}",
        result.err()
    );
}

#[test]
fn test_multiplicity_with_range_expressions() {
    let input = "[0..size(items)]";
    let result = SysMLParser::parse(Rule::owned_multiplicity, input);

    assert!(
        result.is_ok(),
        "Failed to parse multiplicity with expression range: {:?}",
        result.err()
    );
}

#[test]
fn test_connector_end_with_multiplicity_and_chain() {
    let input = "[nCauses] causes.startShot";
    let result = SysMLParser::parse(Rule::connector_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse connector end with multiplicity and feature chain: {:?}",
        result.err()
    );
}

#[test]
fn test_connector_end_with_multiplicity_and_identifier() {
    let input = "[1] endpoint";
    let result = SysMLParser::parse(Rule::connector_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse connector end with multiplicity and identifier: {:?}",
        result.err()
    );
}

#[test]
fn test_connector_end_with_name_references() {
    let input = "myEnd references source.port";
    let result = SysMLParser::parse(Rule::connector_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse connector end with name and references: {:?}",
        result.err()
    );
}

#[test]
fn test_succession_with_multiplicity() {
    let input = r#"succession causalOrdering first [nCauses] causes.startShot then [nEffects] effects {
        doc /* test */
    }"#;
    let result = SysMLParser::parse(Rule::succession_as_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse succession with multiplicities: {:?}",
        result.err()
    );
}

#[test]
fn test_as_expression_with_qualified_name() {
    let input = "causes as SysML::Usage";
    let result = SysMLParser::parse(Rule::classification_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse as expression with qualified name: {:?}",
        result.err()
    );
}

#[test]
fn test_hastype_with_qualified_name() {
    let input = "value hastype Domain::ItemType";
    let result = SysMLParser::parse(Rule::classification_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse hastype with qualified name: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_qualified_type_and_as_cast() {
    let input = "ref :>> baseType = causes as SysML::Usage;";
    let result = SysMLParser::parse(Rule::reference_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with qualified type in as expression: {:?}",
        result.err()
    );
}

#[test]
fn test_meta_expression_with_qualified_name() {
    let input = "multicausations meta SysML::Usage";
    let result = SysMLParser::parse(Rule::classification_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse meta expression with qualified name: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_meta_expression() {
    let input = "ref :>> baseType = multicausations meta SysML::Usage;";
    let result = SysMLParser::parse(Rule::reference_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with meta expression: {:?}",
        result.err()
    );
}

#[test]
fn test_metadata_access_with_qualified_name() {
    let input = "myMetadata @@ SysML::Metadata";
    let result = SysMLParser::parse(Rule::classification_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse @@ expression with qualified name: {:?}",
        result.err()
    );
}

#[test]
fn test_type_reference_with_qualified_name() {
    let input = "Domain::Library::Type";
    let result = SysMLParser::parse(Rule::type_reference, input);

    assert!(
        result.is_ok(),
        "Failed to parse type_reference with qualified name: {:?}",
        result.err()
    );
}

#[test]
fn test_type_result_with_qualified_name() {
    let input = "SysML::Usage";
    let result = SysMLParser::parse(Rule::type_result, input);

    assert!(
        result.is_ok(),
        "Failed to parse type_result with qualified name: {:?}",
        result.err()
    );
}

#[test]
fn test_metadata_reference_with_qualified_name() {
    let input = "MyPackage::MyMetadata";
    let result = SysMLParser::parse(Rule::metadata_reference, input);

    assert!(
        result.is_ok(),
        "Failed to parse metadata_reference with qualified name: {:?}",
        result.err()
    );
}

#[test]
fn test_expression_body_with_typed_parameter_no_direction() {
    let input = "{p2 : Point; p1 != p2}";
    let result = SysMLParser::parse(Rule::expression_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse expression body with typed parameter (no direction): {:?}",
        result.err()
    );
}

#[test]
fn test_parameter_binding_without_direction() {
    let input = "p : Point";
    let result = SysMLParser::parse(Rule::parameter_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse parameter binding without direction: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_default_value_and_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute x default foo { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with default value and body: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_redefinition_default_and_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute :>> x default foo { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with redefinition, default, and body: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_multiplicity_and_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute x[1] { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with multiplicity and body: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_multiplicity_and_default() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute x[1] default foo;");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with multiplicity and default: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_multiplicity_default_no_redef_with_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute x[1] default foo { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with multiplicity, default, and body (no redef): {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_redef_multiplicity_and_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute :>> x[1] { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with redef, multiplicity, and body: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_redef_multiplicity_and_default() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute :>> x[1] default foo;");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with redef, multiplicity, and default: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_long_name_multiplicity_default_body() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute transformation[1] default nullTransformation { }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with long name, multiplicity, default, body: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_y_multiplicity_default_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute y[1] default z { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute y[1] default z with body: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_x_multiplicity_default_null_x_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute x[1] default nullX { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute x[1] default nullX with body: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_x_multiplicity_default_abc_body() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute x[1] default abcdef { }");
    assert!(
        result.is_ok(),
        "Failed to parse attribute x[1] default abcdef with body: {:?}",
        result.err()
    );
}

#[test]
fn test_qualified_name_with_unicode_theta_simple() {
    let result = SysMLParser::parse(Rule::qualified_name, "isq.'Θ'");
    assert!(
        result.is_ok(),
        "Failed to parse simple qualified name with Unicode theta: {:?}",
        result.err()
    );
}

#[test]
fn test_qualified_name_with_unicode_theta_in_expression() {
    let result = SysMLParser::parse(Rule::primary_expression, "isq.'Θ'");
    assert!(
        result.is_ok(),
        "Failed to parse qualified name with Unicode theta as expression: {:?}",
        result.err()
    );
}

#[test]
fn test_unrestricted_name_with_unicode_theta() {
    let result = SysMLParser::parse(Rule::quoted_name, "'Θ'");
    assert!(
        result.is_ok(),
        "Failed to parse quoted name with Unicode theta: {:?}",
        result.err()
    );
}

#[test]
fn test_qualified_name_with_regular_identifiers_in_attribute_body() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute pf { :>> quantity = isq.theta; }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with regular qualified name in body assignment: {:?}",
        result.err()
    );
}

#[test]
fn test_qualified_name_with_quoted_name_in_attribute_body() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute pf { :>> quantity = isq.'z'; }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with quoted name in body assignment: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_short_name() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute <x> myAttr;");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with short name: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_short_name_and_quoted_full_name() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute <isq> 'International System of Quantities';",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with short name and quoted full name: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_short_name_typed_and_body() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute <isq> 'International System of Quantities': SystemOfQuantities { }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with short name, type, and body: {:?}",
        result.err()
    );
}

#[test]
fn test_empty_tuple_expression() {
    let result = SysMLParser::parse(Rule::owned_expression, "()");
    assert!(
        result.is_ok(),
        "Failed to parse empty tuple expression: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_empty_tuple_value() {
    let result = SysMLParser::parse(Rule::attribute_usage, "attribute :>> dimensions = ();");
    assert!(
        result.is_ok(),
        "Failed to parse attribute with empty tuple value: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_with_in_parameter() {
    let result = SysMLParser::parse(
        Rule::assert_constraint_usage,
        "assert constraint c { in x = y; }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse constraint with in parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_scientific_notation_negative_exponent() {
    let result = SysMLParser::parse(Rule::owned_expression, "1E-24");
    assert!(
        result.is_ok(),
        "Failed to parse scientific notation with negative exponent: {:?}",
        result.err()
    );
}

#[test]
fn test_scientific_notation_positive_exponent() {
    let result = SysMLParser::parse(Rule::owned_expression, "1E24");
    assert!(
        result.is_ok(),
        "Failed to parse scientific notation with positive exponent: {:?}",
        result.err()
    );
}

#[test]
fn test_scientific_notation_lowercase() {
    let result = SysMLParser::parse(Rule::owned_expression, "1e-24");
    assert!(
        result.is_ok(),
        "Failed to parse scientific notation lowercase e: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_scientific_notation_value() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute yocto { :>> conversionFactor = 1E-24; }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with scientific notation value: {:?}",
        result.err()
    );
}

#[test]
fn test_function_call_simple() {
    let result = SysMLParser::parse(Rule::owned_expression, "foo()");
    assert!(
        result.is_ok(),
        "Failed to parse simple function call: {:?}",
        result.err()
    );
}

#[test]
fn test_invocation_expression_direct() {
    let result = SysMLParser::parse(Rule::invocation_expression, "allTrue(x)");
    assert!(
        result.is_ok(),
        "Failed to parse invocation expression directly: {:?}",
        result.err()
    );
}

#[test]
fn test_function_call_with_argument() {
    let result = SysMLParser::parse(Rule::owned_expression, "allTrue(x)");
    assert!(
        result.is_ok(),
        "Failed to parse function call with argument: {:?}",
        result.err()
    );
}

#[test]
fn test_function_call_with_dotted_argument() {
    let result = SysMLParser::parse(
        Rule::owned_expression,
        "allTrue(derivedRequirements.result)",
    );
    assert!(
        result.is_ok(),
        "Failed to parse function call with dotted argument: {:?}",
        result.err()
    );
}

#[test]
fn test_implies_expression_with_function_call() {
    let result = SysMLParser::parse(
        Rule::owned_expression,
        "originalRequirement.result implies allTrue(derivedRequirements.result)",
    );
    assert!(
        result.is_ok(),
        "Failed to parse implies expression with function call: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_with_simple_expression() {
    let result = SysMLParser::parse(
        Rule::assert_constraint_usage,
        "assert constraint c { x implies y }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse constraint with simple implies: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_with_function_on_right() {
    let result = SysMLParser::parse(
        Rule::assert_constraint_usage,
        "assert constraint c { x implies f() }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse constraint with function on right: {:?}",
        result.err()
    );
}

#[test]
fn test_all_true_as_identifier() {
    let result = SysMLParser::parse(Rule::identifier, "allTrue");
    assert!(
        result.is_ok(),
        "Failed to parse 'allTrue' as identifier: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_with_actual_names() {
    let result = SysMLParser::parse(
        Rule::assert_constraint_usage,
        "assert constraint c { x implies all(y) }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse constraint with 'all' function: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_with_all_true_function() {
    let result = SysMLParser::parse(
        Rule::assert_constraint_usage,
        "assert constraint c { x implies allTrue(y) }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse constraint with 'allTrue' function: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_without_doc() {
    let result = SysMLParser::parse(
        Rule::assert_constraint_usage,
        r#"assert constraint originalImpliesDerived {
            originalRequirement.result implies allTrue(derivedRequirements.result)
        }"#,
    );
    assert!(
        result.is_ok(),
        "Failed to parse constraint without doc: {:?}",
        result.err()
    );
}

#[test]
fn test_constraint_with_doc_and_expression() {
    let result = SysMLParser::parse(
        Rule::assert_constraint_usage,
        r#"assert constraint originalImpliesDerived {
            doc 
            /* comment */
            originalRequirement.result implies allTrue(derivedRequirements.result)
        }"#,
    );
    assert!(
        result.is_ok(),
        "Failed to parse constraint with doc and expression: {:?}",
        result.err()
    );
}

#[test]
fn test_qualified_name_with_unicode_theta_as_owned_expression() {
    let result = SysMLParser::parse(Rule::owned_expression, "isq.'Θ'");
    assert!(
        result.is_ok(),
        "Failed to parse qualified name with Unicode theta as owned_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_qualified_name_with_unicode_theta_assignment() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute pf { :>> quantity = isq.'Θ'; }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with Unicode theta in body assignment: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_multiplicity_default_and_body() {
    let result = SysMLParser::parse(
        Rule::attribute_usage,
        "attribute :>> transformation[1] default nullTransformation { attribute :>> source; }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse attribute with multiplicity, default, and body: {:?}",
        result.err()
    );
}

#[test]
fn test_interface_usage_with_nonunique() {
    let result = SysMLParser::parse(
        Rule::interface_usage,
        "abstract interface interfaces: Interface[0..*] nonunique :> connections { }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse interface usage with nonunique: {:?}",
        result.err()
    );
}

#[test]
fn test_flow_usage_with_nonunique() {
    let result = SysMLParser::parse(
        Rule::occurrence_usage_element,
        "abstract flow flows: Flow[0..*] nonunique :> messages, flowTransfers { }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse flow usage with nonunique: {:?}",
        result.err()
    );
}

#[test]
fn test_in_parameter_with_default_block() {
    let result = SysMLParser::parse(
        Rule::definition_body_item,
        "in whileTest default {true} { }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse in parameter with default block: {:?}",
        result.err()
    );
}

#[test]
fn test_ref_with_feature_chain_subsetting() {
    let result = SysMLParser::parse(
        Rule::definition_body_item,
        "ref :>> outgoingTransfersFromSelf :> interfacingPorts.incomingTransfersToSelf { }",
    );
    assert!(
        result.is_ok(),
        "Failed to parse ref with feature chain subsetting: {:?}",
        result.err()
    );
}

#[test]
fn test_end_ref_with_name_only() {
    let result = SysMLParser::parse(Rule::reference_usage, "end ref source;");
    assert!(
        result.is_ok(),
        "Failed to parse end ref with name only: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_invocation_expression() {
    // Test at classification_expression level
    let result = SysMLParser::parse(Rule::classification_expression, "allTrue(assumptions())");
    assert!(
        result.is_ok(),
        "Failed at classification_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_invocation_equality() {
    // Test at equality_expression level
    let result = SysMLParser::parse(Rule::equality_expression, "allTrue(assumptions())");
    assert!(
        result.is_ok(),
        "Failed at equality_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_invocation_classification() {
    // Test at classification_expression level
    let result = SysMLParser::parse(Rule::classification_expression, "assumptions()");
    assert!(
        result.is_ok(),
        "Failed at classification_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_invocation_relational() {
    // Test at relational_expression level
    let result = SysMLParser::parse(Rule::relational_expression, "assumptions()");
    assert!(
        result.is_ok(),
        "Failed at relational_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_invocation_base() {
    // Test at base_expression level
    let result = SysMLParser::parse(Rule::base_expression, "assumptions()");
    assert!(
        result.is_ok(),
        "Failed at base_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_invocation_after_constraint() {
    // Test with constraint context - this should reveal the issue
    let result = SysMLParser::parse(Rule::owned_expression, "assumptions()");
    assert!(
        result.is_ok(),
        "Failed with owned_expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_function_call() {
    // Test nested invocation at argument level
    let result = SysMLParser::parse(Rule::argument, "assumptions()");
    assert!(
        result.is_ok(),
        "Failed to parse as argument: {:?}",
        result.err()
    );
}

#[test]
fn test_outer_function_with_inner_call() {
    // Test outer function with inner invocation
    let result = SysMLParser::parse(Rule::owned_expression, "allTrue(assumptions())");
    assert!(result.is_ok(), "Failed nested call: {:?}", result.err());
}

#[test]
fn test_argument_value_with_invocation() {
    // Test at argument_value level (this is where it fails in practice)
    let result = SysMLParser::parse(Rule::argument_value, "assumptions()");
    assert!(
        result.is_ok(),
        "Failed at argument_value: {:?}",
        result.err()
    );
}

#[test]
fn test_invocation_in_calc_body_with_constraints() {
    // Test the exact failing scenario - calculation_body with prior constraint declarations
    let input = r#"{
        constraint assumptions[0..*] :> constraintChecks, subperformances { }
        constraint constraints[0..*] :> constraintChecks, subperformances { }
        return result = allTrue(assumptions()) implies allTrue(constraints()) { }
    }"#;
    let result = SysMLParser::parse(Rule::calculation_body, input);
    assert!(
        result.is_ok(),
        "Failed with constraint declarations before return: {:?}",
        result.err()
    );
}

// Test identifiers starting with "as" keyword
#[test]
fn test_invocations_starting_with_as_keyword() {
    // Issue: "as" keyword was greedily matching prefixes in identifiers
    let cases = vec![
        ("allTrue(assumptions())", "assumptions starts with 'as'"),
        ("allTrue(assertion())", "assertion starts with 'as'"),
        ("allTrue(asdf())", "asdf starts with 'as'"),
        (
            "foo(assumptions(), assertion(), asdf())",
            "multiple args starting with 'as'",
        ),
    ];

    for (input, description) in cases {
        let result = SysMLParser::parse(Rule::owned_expression, input);
        assert!(
            result.is_ok(),
            "Failed to parse {} - {}: {:?}",
            description,
            input,
            result.err()
        );
    }
}

// Test "as" operator with proper word boundary
#[test]
fn test_as_operator_with_qualified_names() {
    // "as" should work as cast operator when followed by space
    let cases = vec![
        ("x as Int", "simple cast"),
        ("x as SysML::Usage", "cast to qualified name"),
        ("foo() as MyType::SubType", "invocation result cast"),
        ("causes as SysML::Usage", "exact case"),
    ];

    for (input, description) in cases {
        let result = SysMLParser::parse(Rule::owned_expression, input);
        assert!(
            result.is_ok(),
            "Failed to parse {} - {}: {:?}",
            description,
            input,
            result.err()
        );
    }
}

// Test "meta" operator with proper word boundary
#[test]
fn test_meta_operator_with_qualified_names() {
    // "meta" should work as operator when followed by space
    let cases = vec![
        ("x meta Usage", "simple meta"),
        ("x meta SysML::Usage", "meta with qualified name"),
        (
            "multicausations meta SysML::Usage",
            "identifier starting with similar pattern",
        ),
    ];

    for (input, description) in cases {
        let result = SysMLParser::parse(Rule::owned_expression, input);
        assert!(
            result.is_ok(),
            "Failed to parse {} - {}: {:?}",
            description,
            input,
            result.err()
        );
    }
}
