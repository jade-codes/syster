#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use pest::Parser;
use rstest::rstest;
use syster::parser::KerMLParser;
use syster::parser::kerml::Rule;
use syster::syntax::kerml::enums::*;
use syster::syntax::kerml::types::*;

/// Helper function to assert that parsing succeeds and the entire input is consumed.
/// This ensures the parser doesn't just match a prefix of the input.
///
/// The function verifies that:
/// 1. Parsing succeeds
/// 2. Exactly one top-level pair is produced (in most cases)
/// 3. The parsed output matches the original input exactly
fn assert_round_trip(rule: Rule, input: &str, desc: &str) {
    let result =
        KerMLParser::parse(rule, input).unwrap_or_else(|e| panic!("Failed to parse {desc}: {e}"));

    let pairs: Vec<_> = result.into_iter().collect();

    // Most parser rules should produce exactly one top-level pair
    // (the EOI rule is an exception that produces multiple pairs)
    if pairs.len() != 1 && rule != Rule::EOI {
        panic!(
            "Expected exactly one top-level pair for {}, but found {}",
            desc,
            pairs.len()
        );
    }

    let parsed: String = pairs.into_iter().map(|p| p.as_str()).collect();

    assert_eq!(input, parsed, "Parsed output mismatch for {desc}");
}

#[test]
fn test_parse_kerml_identifier() {
    assert_round_trip(Rule::identifier, "myVar", "simple identifier");
}

#[rstest]
#[case("about")]
#[case("abstract")]
#[case("alias")]
#[case("all")]
#[case("and")]
#[case("as")]
#[case("assoc")]
#[case("behavior")]
#[case("binding")]
#[case("bool")]
#[case("by")]
#[case("chains")]
#[case("class")]
#[case("classifier")]
#[case("comment")]
#[case("composite")]
#[case("conjugate")]
#[case("conjugates")]
#[case("conjugation")]
#[case("connector")]
#[case("crosses")]
#[case("datatype")]
#[case("default")]
#[case("dependency")]
#[case("derived")]
#[case("differences")]
#[case("disjoining")]
#[case("disjoint")]
#[case("doc")]
#[case("else")]
#[case("end")]
#[case("expr")]
#[case("false")]
#[case("feature")]
#[case("featured")]
#[case("featuring")]
#[case("filter")]
#[case("first")]
#[case("flow")]
#[case("for")]
#[case("from")]
#[case("function")]
#[case("hastype")]
#[case("if")]
#[case("implies")]
#[case("import")]
#[case("in")]
#[case("inout")]
#[case("interaction")]
#[case("intersects")]
#[case("inv")]
#[case("inverse")]
#[case("inverting")]
#[case("istype")]
#[case("language")]
#[case("library")]
#[case("locale")]
#[case("member")]
#[case("meta")]
#[case("metaclass")]
#[case("metadata")]
#[case("namespace")]
#[case("nonunique")]
#[case("not")]
#[case("null")]
#[case("of")]
#[case("or")]
#[case("ordered")]
#[case("out")]
#[case("package")]
#[case("portion")]
#[case("predicate")]
#[case("private")]
#[case("protected")]
#[case("public")]
#[case("const")]
#[case("redefinition")]
#[case("redefines")]
#[case("rep")]
#[case("return")]
#[case("specialization")]
#[case("specializes")]
#[case("standard")]
#[case("step")]
#[case("struct")]
#[case("subclassifier")]
#[case("subset")]
#[case("subsets")]
#[case("subtype")]
#[case("succession")]
#[case("then")]
#[case("to")]
#[case("true")]
#[case("type")]
#[case("typed")]
#[case("unions")]
#[case("xor")]
fn test_parse_kerml_keywords(#[case] keyword: &str) {
    assert_round_trip(Rule::keyword, keyword, keyword);
}

#[rstest]
#[case(Rule::line_comment, "// this is a comment", "line comment")]
#[case(Rule::block_comment, "/* block comment */", "block comment")]
fn test_parse_comments(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Enum Conversion Tests
#[rstest]
#[case("private", VisibilityKind::Private)]
#[case("protected", VisibilityKind::Protected)]
#[case("public", VisibilityKind::Public)]
fn test_visibility_kind_to_enum(#[case] input: &str, #[case] expected: VisibilityKind) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::visibility_kind, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    let result = match parsed.as_str() {
        "private" => VisibilityKind::Private,
        "protected" => VisibilityKind::Protected,
        "public" => VisibilityKind::Public,
        _ => panic!("Unknown visibility kind"),
    };

    assert_eq!(result, expected);
}

#[rstest]
#[case("+", UnaryOperator::Plus)]
#[case("-", UnaryOperator::Minus)]
#[case("not", UnaryOperator::Not)]
#[case("~", UnaryOperator::BitwiseNot)]
fn test_unary_operator_to_enum(#[case] input: &str, #[case] expected: UnaryOperator) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::unary_operator, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    let result = match parsed.as_str() {
        "+" => UnaryOperator::Plus,
        "-" => UnaryOperator::Minus,
        "not" => UnaryOperator::Not,
        "~" => UnaryOperator::BitwiseNot,
        _ => panic!("Unknown unary operator"),
    };

    assert_eq!(result, expected);
}

#[rstest]
#[case("@", ClassificationTestOperator::At)]
#[case("hastype", ClassificationTestOperator::HasType)]
#[case("istype", ClassificationTestOperator::IsType)]
fn test_classification_test_operator_to_enum(
    #[case] input: &str,
    #[case] expected: ClassificationTestOperator,
) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::classification_test_operator,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    let result = match parsed.as_str() {
        "@" => ClassificationTestOperator::At,
        "hastype" => ClassificationTestOperator::HasType,
        "istype" => ClassificationTestOperator::IsType,
        _ => panic!("Unknown classification test operator"),
    };

    assert_eq!(result, expected);
}

#[rstest]
#[case("!=", EqualityOperator::NotEqual)]
#[case("!==", EqualityOperator::NotIdentical)]
#[case("==", EqualityOperator::Equal)]
#[case("===", EqualityOperator::Identical)]
fn test_equality_operator_to_enum(#[case] input: &str, #[case] expected: EqualityOperator) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::equality_operator, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    let result = match parsed.as_str() {
        "!=" => EqualityOperator::NotEqual,
        "!==" => EqualityOperator::NotIdentical,
        "==" => EqualityOperator::Equal,
        "===" => EqualityOperator::Identical,
        _ => panic!("Unknown equality operator"),
    };

    assert_eq!(result, expected);
}

#[rstest]
#[case("::*", ImportKind::Members)]
#[case("::**", ImportKind::MembersRecursive)]
#[case("::*::**", ImportKind::AllRecursive)]
fn test_import_kind_to_enum(#[case] input: &str, #[case] expected: ImportKind) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::import_kind, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    let result = match parsed.as_str() {
        "::*" => ImportKind::Members,
        "::**" => ImportKind::MembersRecursive,
        "::*::**" => ImportKind::AllRecursive,
        _ => panic!("Unknown import kind"),
    };

    assert_eq!(result, expected);
}

#[rstest]
#[case("<", RelationalOperator::LessThan)]
#[case("<=", RelationalOperator::LessThanOrEqual)]
#[case(">", RelationalOperator::GreaterThan)]
#[case(">=", RelationalOperator::GreaterThanOrEqual)]
fn test_relational_operator_to_enum(#[case] input: &str, #[case] expected: RelationalOperator) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::relational_operator, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    let result = match parsed.as_str() {
        "<" => RelationalOperator::LessThan,
        "<=" => RelationalOperator::LessThanOrEqual,
        ">" => RelationalOperator::GreaterThan,
        ">=" => RelationalOperator::GreaterThanOrEqual,
        _ => panic!("Unknown relational operator"),
    };

    assert_eq!(result, expected);
}

// Test the grouped enum_type rule
#[rstest]
#[case("private")]
#[case("protected")]
#[case("public")]
#[case("in")]
#[case("out")]
#[case("+")]
#[case("-")]
#[case("@")]
#[case("==")]
#[case("::*")]
#[case("<")]
fn test_enum_type_parses_all_enums(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::enum_type, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Verify we got an enum_type node
    assert_eq!(parsed.as_rule(), syster::parser::kerml::Rule::enum_type);

    // The inner rule should be one of the specific enum types
    let inner = parsed.into_inner().next().unwrap();
    assert!(matches!(
        inner.as_rule(),
        syster::parser::kerml::Rule::visibility_kind
            | syster::parser::kerml::Rule::feature_direction_kind
            | syster::parser::kerml::Rule::unary_operator
            | syster::parser::kerml::Rule::classification_test_operator
            | syster::parser::kerml::Rule::equality_operator
            | syster::parser::kerml::Rule::import_kind
            | syster::parser::kerml::Rule::relational_operator
    ));
}

// Annotation type tests
#[test]
fn test_element_creation() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    assert_eq!(
        format!("{element:?}"),
        "Element { declared_name: None, declared_short_name: None }"
    );
}

#[test]
fn test_annotation_creation() {
    let annotation = Annotation {
        reference: "SomeElement".to_string(),
        span: None,
    };
    assert!(format!("{annotation:?}").contains("Annotation"));
    assert_eq!(annotation.reference, "SomeElement");
}

#[test]
fn test_annotating_element_empty() {
    let annotating = AnnotatingElement { about: vec![] };
    assert_eq!(annotating.about.len(), 0);
}

#[test]
fn test_annotating_element_with_annotations() {
    let annotation1 = Annotation {
        reference: "Element1".to_string(),
        span: None,
    };
    let annotation2 = Annotation {
        reference: "Element2".to_string(),
        span: None,
    };

    let annotating = AnnotatingElement {
        about: vec![annotation1, annotation2],
    };
    assert_eq!(annotating.about.len(), 2);
}

#[test]
fn test_textual_annotating_element() {
    let annotating_element = AnnotatingElement { about: vec![] };
    let textual = TextualAnnotatingElement {
        annotating_element,
        body: "Some text content".to_string(),
    };
    assert_eq!(textual.body, "Some text content");
}

#[test]
fn test_comment_without_locale() {
    let comment = Comment {
        content: "This is a comment".to_string(),
        about: vec![],
        locale: None,
        span: None,
    };
    assert!(comment.locale.is_none());
    assert_eq!(comment.content, "This is a comment");
}

#[test]
fn test_comment_with_locale() {
    let comment = Comment {
        content: "Ceci est un commentaire".to_string(),
        about: vec![],
        locale: Some("fr-FR".to_string()),
        span: None,
    };
    assert_eq!(comment.locale, Some("fr-FR".to_string()));
    assert_eq!(comment.content, "Ceci est un commentaire");
}

#[test]
fn test_documentation() {
    let comment = Comment {
        content: "Documentation text".to_string(),
        about: vec![],
        locale: Some("en-US".to_string()),
        span: None,
    };
    let doc = Documentation {
        comment,
        span: None,
    };
    assert_eq!(doc.comment.content, "Documentation text");
    assert_eq!(doc.comment.locale, Some("en-US".to_string()));
}

#[test]
fn test_textual_representation() {
    let textual = TextualAnnotatingElement {
        annotating_element: AnnotatingElement { about: vec![] },
        body: "fn main() {}".to_string(),
    };
    let representation = TextualRepresentation {
        textual_annotating_element: textual,
        language: "rust".to_string(),
    };
    assert_eq!(representation.language, "rust");
    assert_eq!(
        representation.textual_annotating_element.body,
        "fn main() {}"
    );
}

#[test]
fn test_clone_annotation() {
    let annotation = Annotation {
        reference: "TestElement".to_string(),
        span: None,
    };
    let cloned = annotation.clone();
    assert_eq!(annotation, cloned);
    assert_eq!(cloned.reference, "TestElement");
}

#[test]
fn test_equality_annotations() {
    let annotation1 = Annotation {
        reference: "Element".to_string(),
        span: None,
    };
    let annotation2 = Annotation {
        reference: "Element".to_string(),
        span: None,
    };
    assert_eq!(annotation1, annotation2);
}

// Relationship type tests
#[test]
fn test_relationship_with_element() {
    let element = Element {
        declared_name: Some("TestElement".to_string()),
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    assert!(relationship.element.declared_name.is_some());
}

#[test]
fn test_inheritance_from_relationship() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let inheritance = Inheritance { relationship };
    assert!(format!("{inheritance:?}").contains("Inheritance"));
}

#[test]
fn test_membership_with_alias() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let membership = Membership {
        relationship,
        is_alias: true,
    };
    assert!(membership.is_alias);
}

#[test]
fn test_import_with_flags() {
    let element = Element {
        declared_name: None,
        declared_short_name: None,
    };
    let relationship = Relationship {
        element,
        visibility: None,
        elements: vec![],
        source: None,
        source_ref: None,
        source_chain: None,
        target: None,
        target_ref: None,
        target_chain: None,
    };
    let import = Import {
        relationship,
        imports_all: true,
        is_recursive: false,
        is_namespace: Some(NamespaceMarker::Namespace),
    };
    assert!(import.imports_all);
    assert!(!import.is_recursive);
    assert!(import.is_namespace.is_some());
}

// Reference type tests
#[test]
fn test_element_reference_creation() {
    let element = Element {
        declared_name: Some("RefElement".to_string()),
        declared_short_name: None,
    };
    let reference = ElementReference {
        parts: vec![element],
    };
    assert_eq!(reference.parts.len(), 1);
    assert_eq!(
        reference.parts[0].declared_name,
        Some("RefElement".to_string())
    );
}

#[test]
fn test_namespace_reference() {
    let element_ref = ElementReference { parts: vec![] };
    let namespace_ref = NamespaceReference {
        element_reference: element_ref,
    };
    assert_eq!(namespace_ref.element_reference.parts.len(), 0);
}

#[test]
fn test_type_reference_hierarchy() {
    let element_ref = ElementReference { parts: vec![] };
    let namespace_ref = NamespaceReference {
        element_reference: element_ref,
    };
    let type_ref = TypeReference {
        namespace_reference: namespace_ref,
    };
    assert_eq!(
        type_ref.namespace_reference.element_reference.parts.len(),
        0
    );
}

#[test]
fn test_feature_reference() {
    let element_ref = ElementReference { parts: vec![] };
    let namespace_ref = NamespaceReference {
        element_reference: element_ref,
    };
    let type_ref = TypeReference {
        namespace_reference: namespace_ref,
    };
    let feature_ref = FeatureReference {
        type_reference: type_ref,
    };
    assert!(format!("{feature_ref:?}").contains("FeatureReference"));
}

#[rstest]
#[case(Rule::decimal, "123", "decimal integer")]
#[case(Rule::decimal, "0", "zero")]
#[case(Rule::decimal, "999999", "large decimal")]
#[case(Rule::number, "42", "simple number")]
#[case(Rule::number, "3.14", "decimal number")]
#[case(Rule::number, ".5", "decimal starting with dot")]
#[case(Rule::number, "1.5e10", "exponent notation")]
#[case(Rule::number, "2.0E-5", "exponent with negative")]
#[case(Rule::number, "3e+2", "exponent with positive")]
fn test_parse_numbers(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::unrestricted_name, "'simple'", "simple unrestricted name")]
#[case(
    Rule::unrestricted_name,
    "'with space'",
    "unrestricted name with space"
)]
#[case(
    Rule::unrestricted_name,
    "'with\\'quote'",
    "unrestricted name with quote"
)]
#[case(Rule::name, "myName", "regular name")]
#[case(Rule::name, "'unrestricted name'", "name as unrestricted")]
#[case(Rule::string_value, r#""hello world""#, "string value")]
fn test_parse_names(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Identification Tests

#[rstest]
#[case(Rule::short_name, "<shortName>", "short name")]
#[case(Rule::short_name, "<name123>", "short name with number")]
#[case(Rule::regular_name, "regularName", "regular name")]
#[case(Rule::regular_name, "'unrestricted name'", "regular unrestricted name")]
#[case(Rule::identification, "<short> regular", "identification with both")]
#[case(Rule::identification, "<short>", "identification short only")]
#[case(Rule::identification, "regular", "identification regular only")]
fn test_parse_identification(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Relationship Operator Tests

#[rstest]
#[case(Rule::specializes_operator, ":>", "specializes symbol")]
#[case(Rule::specializes_operator, "specializes", "specializes keyword")]
#[case(Rule::redefines_operator, ":>>", "redefines symbol")]
#[case(Rule::redefines_operator, "redefines", "redefines keyword")]
#[case(Rule::typed_by_operator, ":", "typed by colon")]
#[case(Rule::typed_by_operator, "typed by", "typed by keyword")]
#[case(Rule::conjugates_operator, "~", "conjugates symbol")]
#[case(Rule::conjugates_operator, "conjugates", "conjugates keyword")]
fn test_parse_relationship_operators(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Common Fragment Tests

#[rstest]
#[case(Rule::abstract_marker, "abstract", "abstract marker")]
#[case(Rule::const_modifier, "const", "const modifier")]
#[case(Rule::sufficient, "all", "sufficient")]
fn test_parse_common_fragments(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::multiplicity_properties, "ordered", "ordered")]
#[case(Rule::multiplicity_properties, "nonunique", "nonunique")]
#[case(
    Rule::multiplicity_properties,
    "ordered nonunique",
    "ordered nonunique"
)]
#[case(
    Rule::multiplicity_properties,
    "nonunique ordered",
    "nonunique ordered"
)]
fn test_parse_multiplicity_properties(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::literal_boolean, "true", "true literal")]
#[case(Rule::literal_boolean, "false", "false literal")]
#[case(Rule::literal_string, r#""test string""#, "string literal")]
#[case(Rule::literal_number, "42", "integer literal")]
#[case(Rule::literal_number, "3.14", "decimal literal")]
#[case(Rule::literal_number, "1.5e10", "exponent literal")]
#[case(Rule::literal_infinity, "*", "infinity literal")]
fn test_parse_literals(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::literal_expression, "true", "true expression")]
#[case(Rule::literal_expression, r#""string""#, "string expression")]
#[case(Rule::literal_expression, "42", "number expression")]
#[case(Rule::literal_expression, "*", "infinity expression")]
fn test_parse_literal_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::null_expression, "null", "null keyword")]
#[case(Rule::null_expression, "()", "empty parens")]
fn test_parse_null_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::visibility_kind, "public", "public visibility")]
#[case(Rule::visibility_kind, "private", "private visibility")]
#[case(Rule::visibility_kind, "protected", "protected visibility")]
#[case(Rule::feature_direction_kind, "in", "in direction")]
#[case(Rule::feature_direction_kind, "out", "out direction")]
#[case(Rule::feature_direction_kind, "inout", "inout direction")]
fn test_parse_visibility_and_direction(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::unary_operator, "+", "plus operator")]
#[case(Rule::unary_operator, "-", "minus operator")]
#[case(Rule::unary_operator, "~", "tilde operator")]
#[case(Rule::unary_operator, "not", "not operator")]
#[case(Rule::classification_test_operator, "hastype", "hastype operator")]
#[case(Rule::classification_test_operator, "istype", "istype operator")]
#[case(Rule::classification_test_operator, "@", "at operator")]
#[case(Rule::classification_test_operator, "@@", "double at operator")]
#[case(Rule::equality_operator, "==", "equal operator")]
#[case(Rule::equality_operator, "!=", "not equal operator")]
#[case(Rule::equality_operator, "===", "identical operator")]
#[case(Rule::equality_operator, "!==", "not identical operator")]
#[case(Rule::relational_operator, "<", "less than operator")]
#[case(Rule::relational_operator, ">", "greater than operator")]
#[case(Rule::relational_operator, "<=", "less than or equal operator")]
#[case(Rule::relational_operator, ">=", "greater than or equal operator")]
fn test_parse_operators(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::import_kind, "::*", "members import")]
#[case(Rule::import_kind, "::**", "recursive import")]
#[case(Rule::import_kind, "::*::**", "all recursive import")]
fn test_parse_import_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Additional Common Fragment Tests

#[rstest]
#[case(Rule::visibility, "public", "public visibility")]
#[case(Rule::visibility, "private", "private visibility")]
#[case(Rule::visibility, "protected", "protected visibility")]
#[case(Rule::derived, "derived", "derived marker")]
#[case(Rule::end_marker, "end", "end marker")]
#[case(Rule::standard_marker, "standard", "standard marker")]
#[case(Rule::import_all, "all", "import all")]
fn test_parse_additional_fragments(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Reference Tests

#[rstest]
#[case(Rule::qualified_reference_chain, "Foo", "simple reference")]
#[case(Rule::qualified_reference_chain, "Foo::Bar", "two-level reference")]
#[case(
    Rule::qualified_reference_chain,
    "Foo::Bar::Baz",
    "three-level reference"
)]
fn test_parse_qualified_reference_chain(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::inline_expression, "true", "boolean true")]
#[case(Rule::inline_expression, r#""test""#, "string literal")]
#[case(Rule::inline_expression, "42", "number")]
#[case(Rule::inline_expression, "null", "null literal")]
fn test_parse_inline_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Additional Token Tests
#[rstest]
#[case(Rule::subsets_operator, ":>", "symbol form")]
#[case(Rule::subsets_operator, "subsets", "keyword form")]
fn test_parse_subsets_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::references_operator, "::>", "symbol form")]
#[case(Rule::references_operator, "references", "keyword form")]
fn test_parse_references_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::crosses_operator, "=>", "symbol form")]
#[case(Rule::crosses_operator, "crosses", "keyword form")]
fn test_parse_crosses_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_chain_expression, "myFeature", "simple feature")]
#[case(Rule::feature_chain_expression, "a.b", "two-level chain")]
#[case(Rule::feature_chain_expression, "a.b.c", "three-level chain")]
fn test_parse_feature_chain_expression(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::index_expression, "myArray", "simple array")]
#[case(Rule::index_expression, "arr[0]", "indexed array")]
#[case(Rule::index_expression, "matrix[1][2]", "multi-indexed array")]
fn test_parse_index_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Additional Expression and Metadata Tests

// Body Structure Tests

#[test]
fn test_parse_block_comment() {
    assert_round_trip(Rule::block_comment, "/* textual body */", "block comment");
}

#[rstest]
#[case(Rule::relationship_body, ";", "semicolon form")]
#[case(Rule::relationship_body, "{}", "empty braces form")]
fn test_parse_relationship_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Import and Filter Tests

#[rstest]
#[case(Rule::import_prefix, "import", "simple import")]
#[case(Rule::import_prefix, "public import", "public import")]
#[case(Rule::import_prefix, "private import", "private import")]
#[case(Rule::import_prefix, "protected import", "protected import")]
#[case(Rule::import_prefix, "import all", "import all")]
#[case(Rule::import_prefix, "private import all", "private import all")]
fn test_parse_import_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::imported_reference, "MyImport", "simple import")]
#[case(Rule::imported_reference, "MyImport::*", "wildcard import")]
#[case(Rule::imported_reference, "MyImport::**", "recursive import")]
#[case(Rule::imported_reference, "MyImport::*::**", "all recursive import")]
fn test_parse_imported_reference(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Relationship Declaration Tests

#[rstest]
#[case(Rule::relationship, "BaseType", "simple relationship")]
#[case(Rule::relationship, "public BaseType", "relationship with visibility")]
#[case(Rule::relationship, "MyType::NestedType", "relationship qualified")]
fn test_parse_relationship(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::inheritance, "BaseType", "inheritance base")]
#[case(Rule::inheritance, "private BaseClass", "inheritance with visibility")]
#[case(Rule::specialization, ":> BaseType", "specialization symbol")]
#[case(
    Rule::specialization,
    "specializes BaseClass",
    "specialization keyword"
)]
#[case(
    Rule::specialization,
    ":> public MyBase",
    "specialization with visibility"
)]
fn test_parse_inheritance_specialization(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::subsetting, ":> BaseType", "subsetting symbol")]
#[case(Rule::subsetting, "subsets BaseClass", "subsetting keyword")]
#[case(Rule::subsetting, ":> Base::MyType", "subsetting qualified")]
#[case(Rule::subsetting, ":> Clock, Life", "subsetting multiple")]
#[case(Rule::subsetting, ":> Type1, Type2, Type3", "subsetting triple")]
#[case(Rule::redefinition, ":>> BaseType", "redefinition symbol")]
#[case(Rule::redefinition, "redefines OldFeature", "redefinition keyword")]
#[case(Rule::redefinition, ":>> Base::Type", "redefinition qualified")]
#[case(Rule::redefinition, ":>> Collection::elements", "redefinition path")]
#[case(Rule::redefinition, ":>> Feature1, Feature2", "redefinition multiple")]
#[case(
    Rule::reference_subsetting,
    "::> RefType",
    "reference subsetting symbol"
)]
#[case(
    Rule::reference_subsetting,
    "references RefFeature",
    "reference subsetting keyword"
)]
#[case(
    Rule::reference_subsetting,
    "::> Ref::Feature",
    "reference subsetting qualified"
)]
#[case(Rule::cross_subsetting, "=> CrossedType", "cross subsetting symbol")]
#[case(
    Rule::cross_subsetting,
    "crosses CrossedFeature",
    "cross subsetting keyword"
)]
#[case(Rule::cross_subsetting, "=> Cross::Type", "cross subsetting qualified")]
fn test_parse_subsetting_redefinition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::conjugation, "conjugates BaseType", "conjugation base")]
#[case(
    Rule::conjugation,
    "conjugates public ConjugateType",
    "conjugation with visibility"
)]
#[case(Rule::unioning, "unions Type1", "unioning base")]
#[case(Rule::unioning, "unions public Type2", "unioning with visibility")]
#[case(Rule::differencing, "differences Type1", "differencing base")]
#[case(
    Rule::differencing,
    "differences private Type2",
    "differencing with visibility"
)]
#[case(Rule::intersecting, "intersects Type1", "intersecting base")]
#[case(
    Rule::intersecting,
    "intersects public Type2",
    "intersecting with visibility"
)]
#[case(
    Rule::intersecting,
    "intersects VectorValue, Array",
    "intersecting multiple"
)]
fn test_parse_type_operations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_chaining, "chains feature1", "feature chaining")]
#[case(
    Rule::feature_chaining,
    "chains public feature2",
    "feature chaining with visibility"
)]
#[case(
    Rule::feature_chaining,
    "chains source.target",
    "feature chaining qualified"
)]
#[case(Rule::feature_chaining, "chains a.b.c", "feature chaining triple")]
#[case(
    Rule::feature_chaining,
    "chains parent.child",
    "feature chaining parent child"
)]
#[case(Rule::disjoining, "disjoint Type1", "disjoining base")]
#[case(
    Rule::disjoining,
    "disjoint private Type2",
    "disjoining with visibility"
)]
#[case(
    Rule::feature_inverting,
    "inverse feature1 of feature2;",
    "feature inverting base"
)]
#[case(
    Rule::feature_inverting,
    "inverse feature2 of other;",
    "feature inverting with target"
)]
#[case(Rule::featuring, "featured by Type1", "featuring base")]
#[case(Rule::featuring, "featured by Type2", "featuring second type")]
#[case(
    Rule::type_featuring,
    "featuring f by Type1;",
    "type featuring with ref"
)]
#[case(
    Rule::type_featuring,
    "featuring of f by Type1;",
    "type featuring standalone"
)]
fn test_parse_feature_operations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_typing, "typed by BaseType", "feature typing base")]
#[case(Rule::feature_typing, ": TypeSpec", "feature typing colon")]
#[case(Rule::feature_typing, ": Complex", "feature typing complex")]
#[case(Rule::feature_typing, ": Boolean", "feature typing boolean")]
#[case(Rule::feature_typing, ": Anything", "feature typing anything")]
#[case(Rule::feature_typing, ": String", "feature typing string")]
#[case(
    Rule::subclassification,
    "subclassifier Sub :> BaseClass;",
    "subclassification symbol"
)]
#[case(
    Rule::subclassification,
    "subclassifier Sub :> ClassSpec;",
    "subclassification keyword"
)]
#[case(Rule::membership, "MyRef", "membership simple")]
#[case(Rule::membership, "public MyRef", "membership with visibility")]
#[case(Rule::membership, "alias MyRef", "membership alias")]
#[case(Rule::membership, "private alias", "membership private alias")]
#[case(Rule::owning_membership, "MyRef", "owning membership simple")]
#[case(
    Rule::owning_membership,
    "public alias MyRef",
    "owning membership with visibility"
)]
fn test_parse_typing_and_membership(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// feature_value
#[case(Rule::feature_value, "= MyRef", "simple feature value")]
#[case(Rule::feature_value, ":= public MyRef", "bind with visibility")]
#[case(Rule::feature_value, "= alias Target", "alias feature value")]
// element_filter_membership
#[case(Rule::element_filter_membership, "filter MyRef;", "simple filter")]
#[case(
    Rule::element_filter_membership,
    "filter OtherRef;",
    "filter with expression"
)]
// feature_membership
#[case(
    Rule::feature_membership,
    "featured by MyType alias MyRef",
    "simple feature membership"
)]
#[case(
    Rule::feature_membership,
    "featured by BaseType public alias Target",
    "featured with visibility and alias"
)]
// end_feature_membership
#[case(
    Rule::end_feature_membership,
    "end x : MyType;",
    "simple end feature membership"
)]
#[case(
    Rule::end_feature_membership,
    "end y : BaseType[1];",
    "end featured with visibility and alias"
)]
// result_expression_membership
#[case(
    Rule::result_expression_membership,
    "return featured by MyType alias MyRef",
    "simple result expression"
)]
#[case(
    Rule::result_expression_membership,
    "return featured by BaseType public alias Target",
    "result expression with visibility and alias"
)]
fn test_parse_membership_constructs(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// import
#[case(Rule::import, "import MyPackage;", "simple import")]
#[case(Rule::import, "public import MyLib;", "public import")]
#[case(Rule::import, "import all MyNamespace;", "import all")]
#[case(Rule::import, "private import all Base;", "private import all")]
#[case(Rule::import, "import MyPackage::*;", "namespace import")]
#[case(Rule::import, "import MyPackage::**;", "recursive import")]
#[case(Rule::import, "import MyPackage {}", "import with body")]
// dependency
#[case(Rule::dependency, "dependency Source to Target;", "simple dependency")]
#[case(
    Rule::dependency,
    "dependency MyDep from Source to Target;",
    "named dependency"
)]
#[case(
    Rule::dependency,
    "dependency Source, Other to Target, Dest;",
    "multiple sources and targets"
)]
#[case(
    Rule::dependency,
    "dependency <short> named from Source to Target {}",
    "dependency with short name and body"
)]
fn test_parse_import_and_dependency(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Element Declaration Tests

#[rstest]
// namespace
#[case(Rule::namespace, "namespace MyNamespace;", "simple namespace")]
#[case(Rule::namespace, "namespace MyNamespace {}", "namespace with body")]
#[case(
    Rule::namespace,
    "namespace <short> named {}",
    "namespace with short name"
)]
// package
#[case(Rule::package, "package MyPackage;", "simple package")]
#[case(Rule::package, "package MyPackage {}", "package with body")]
#[case(Rule::package, "package <short> named {}", "package with short name")]
// library_package
#[case(
    Rule::library_package,
    "library package LibPkg;",
    "simple library package"
)]
#[case(
    Rule::library_package,
    "standard library package StdLib;",
    "standard library package"
)]
#[case(
    Rule::library_package,
    "library package MyLib {}",
    "library package with body"
)]
fn test_parse_namespaces_and_packages(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// class
#[case(Rule::class, "class MyClass;", "simple class")]
#[case(Rule::class, "class MyClass {}", "class with body")]
#[case(Rule::class, "abstract class MyClass;", "abstract class")]
#[case(
    Rule::class,
    "class MyClass specializes Base {}",
    "class with specialization"
)]
#[case(
    Rule::class,
    "abstract class MyClass specializes Base, Other {}",
    "abstract class with multiple specializations"
)]
// data_type
#[case(Rule::data_type, "datatype MyData;", "simple datatype")]
#[case(Rule::data_type, "datatype MyData {}", "datatype with body")]
#[case(
    Rule::data_type,
    "abstract datatype ScalarValue specializes DataValue;",
    "abstract datatype with specialization"
)]
#[case(
    Rule::data_type,
    "datatype Boolean specializes ScalarValue;",
    "Boolean datatype"
)]
#[case(
    Rule::data_type,
    "datatype String specializes ScalarValue;",
    "String datatype"
)]
// structure
#[case(Rule::structure, "struct MyStruct;", "simple struct")]
#[case(Rule::structure, "struct MyStruct {}", "struct with body")]
#[case(
    Rule::structure,
    "struct MyStruct[1] :> Parent {}",
    "struct with multiplicity and subclassification"
)]
#[case(
    Rule::structure,
    "private struct MyStruct[0..1] specializes Base {}",
    "private struct with range multiplicity"
)]
#[case(
    Rule::structure,
    "abstract struct MyStruct specializes Base, Other {}",
    "abstract struct with multiple specializations"
)]
// association
#[case(Rule::association, "assoc MyAssoc;", "simple association")]
#[case(Rule::association, "assoc MyAssoc {}", "association with body")]
#[case(
    Rule::association,
    "abstract assoc Link specializes Anything {}",
    "abstract association"
)]
#[case(
    Rule::association,
    "assoc MyAssoc specializes Base {}",
    "association with specialization"
)]
// association_structure
#[case(
    Rule::association_structure,
    "assoc struct MyAssocStruct;",
    "simple association struct"
)]
#[case(
    Rule::association_structure,
    "assoc struct MyAssocStruct {}",
    "association struct with body"
)]
fn test_parse_type_definitions(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// behavior
#[case(Rule::behavior, "behavior MyBehavior;", "simple behavior")]
#[case(Rule::behavior, "behavior MyBehavior {}", "behavior with body")]
#[case(
    Rule::behavior,
    "abstract behavior DecisionPerformance specializes Performance {}",
    "abstract behavior with specialization"
)]
#[case(
    Rule::behavior,
    "behavior MyBehavior specializes Base, Other {}",
    "behavior with multiple specializations"
)]
// function
#[case(Rule::function, "function MyFunction;", "simple function")]
#[case(Rule::function, "function MyFunction {}", "function with body")]
// predicate
#[case(Rule::predicate, "predicate MyPredicate;", "simple predicate")]
#[case(Rule::predicate, "predicate MyPredicate {}", "predicate with body")]
// interaction
#[case(Rule::interaction, "interaction MyInteraction;", "simple interaction")]
#[case(
    Rule::interaction,
    "interaction MyInteraction {}",
    "interaction with body"
)]
// metaclass
#[case(Rule::metaclass, "metaclass MyMetaclass;", "simple metaclass")]
#[case(Rule::metaclass, "metaclass MyMetaclass {}", "metaclass with body")]
fn test_parse_behavioral_types(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// connector
#[case(Rule::connector, "connector MyConnector;", "simple connector")]
#[case(Rule::connector, "connector MyConnector {}", "connector with body")]
// binding_connector
#[case(Rule::binding_connector, "binding MyBinding;", "simple binding")]
#[case(Rule::binding_connector, "binding MyBinding {}", "binding with body")]
// succession
#[case(Rule::succession, "succession MySuccession;", "simple succession")]
#[case(Rule::succession, "succession MySuccession {}", "succession with body")]
fn test_parse_connectors(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// step
#[case(Rule::step, "step MyStep;", "simple step")]
#[case(Rule::step, "step MyStep {}", "step with body")]
// expression
#[case(Rule::expression, "expr MyExpr;", "simple expression")]
#[case(Rule::expression, "expr MyExpr {}", "expression with body")]
// invariant
#[case(Rule::invariant, "inv MyInvariant;", "simple invariant")]
#[case(Rule::invariant, "inv not MyInvariant {}", "negated invariant")]
fn test_parse_step_expression_invariant(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Feature Tests

#[rstest]
// basic feature
#[case(Rule::feature, "feature MyFeature;", "simple feature")]
#[case(Rule::feature, "feature MyFeature {}", "feature with body")]
// with direction
#[case(Rule::feature, "in feature MyFeature;", "input feature")]
#[case(Rule::feature, "out feature MyFeature;", "output feature")]
#[case(Rule::feature, "inout feature MyFeature;", "inout feature")]
// with composition
#[case(Rule::feature, "abstract feature MyFeature;", "abstract feature")]
#[case(Rule::feature, "composite feature MyFeature;", "composite feature")]
#[case(Rule::feature, "portion feature MyFeature;", "portion feature")]
// with property
#[case(Rule::feature, "const feature MyFeature;", "const feature")]
#[case(Rule::feature, "derived feature MyFeature;", "derived feature")]
#[case(Rule::feature, "end feature MyFeature;", "end feature")]
// with multiplicity properties
#[case(Rule::feature, "feature MyFeature ordered;", "ordered feature")]
#[case(Rule::feature, "feature MyFeature nonunique;", "nonunique feature")]
#[case(
    Rule::feature,
    "feature MyFeature ordered nonunique;",
    "ordered nonunique feature"
)]
// combined modifiers
#[case(
    Rule::feature,
    "in abstract const feature MyFeature ordered;",
    "combined modifiers 1"
)]
#[case(
    Rule::feature,
    "out composite derived feature MyFeature nonunique;",
    "combined modifiers 2"
)]
#[case(
    Rule::feature,
    "inout portion end feature MyFeature ordered nonunique;",
    "combined modifiers 3"
)]
// with multiplicity and relationships
#[case(
    Rule::feature,
    "feature elements[0..*] :>> Collection::elements {}",
    "feature with multiplicity and redefinition"
)]
#[case(
    Rule::feature,
    "feature myFeature[1] :> BaseFeature;",
    "feature with multiplicity and subsetting"
)]
#[case(
    Rule::feature,
    "feature items[*] : ItemType ordered;",
    "feature with multiplicity and type"
)]
fn test_parse_feature_variations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Annotation Element Tests

#[rstest]
// comment_annotation basic
#[case(
    Rule::comment_annotation,
    "comment /* simple comment */",
    "simple comment"
)]
#[case(
    Rule::comment_annotation,
    "comment myComment /* comment text */",
    "named comment"
)]
// comment_annotation with locale
#[case(
    Rule::comment_annotation,
    r#"comment locale "en-US" /* comment text */"#,
    "comment with locale"
)]
#[case(
    Rule::comment_annotation,
    r#"comment MyComment locale "fr-FR" /* texte */"#,
    "named comment with locale"
)]
// comment_annotation with about
#[case(
    Rule::comment_annotation,
    "comment about Foo /* about Foo */",
    "comment about single element"
)]
#[case(
    Rule::comment_annotation,
    "comment about Bar, Baz /* about multiple */",
    "comment about multiple elements"
)]
// documentation basic
#[case(Rule::documentation, "doc /* documentation */", "simple documentation")]
#[case(Rule::documentation, "doc MyDoc /* doc text */", "named documentation")]
// documentation with locale
#[case(
    Rule::documentation,
    r#"doc locale "en-US" /* docs */"#,
    "doc with locale"
)]
#[case(
    Rule::documentation,
    r#"doc MyDoc locale "ja-JP" /* text */"#,
    "named doc with locale"
)]
// textual_representation
#[case(
    Rule::textual_representation,
    r#"language "rust" /* code */"#,
    "rust representation"
)]
#[case(
    Rule::textual_representation,
    r#"rep language "python" /* code */"#,
    "python representation"
)]
#[case(
    Rule::textual_representation,
    r#"rep MyRep language "java" /* code */"#,
    "named java representation"
)]
fn test_parse_annotations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Multiplicity and Feature tests
#[rstest]
// multiplicity
#[case(Rule::multiplicity, "feature;", "simple multiplicity")]
#[case(Rule::multiplicity, "feature myMultiplicity;", "named multiplicity")]
#[case(
    Rule::multiplicity,
    "feature myMultiplicity : MyType;",
    "typed multiplicity"
)]
// multiplicity_range
#[case(Rule::multiplicity_range, "feature;", "simple multiplicity range")]
#[case(
    Rule::multiplicity_range,
    "feature myRange;",
    "named multiplicity range"
)]
#[case(
    Rule::multiplicity_range,
    "feature myRange { feature bound; }",
    "multiplicity range with body"
)]
// metadata_feature
#[case(Rule::metadata_feature, "metadata MyType;", "simple metadata")]
#[case(Rule::metadata_feature, "metadata myMeta : MyType;", "named metadata")]
#[case(Rule::metadata_feature, "metadata MyType about Foo;", "metadata about")]
#[case(
    Rule::metadata_feature,
    "metadata myMeta : MyType about Foo, Bar;",
    "metadata about multiple"
)]
// item_feature
#[case(Rule::item_feature, "feature;", "simple item feature")]
#[case(Rule::item_feature, "feature myItem;", "named item feature")]
#[case(Rule::item_feature, "feature myItem : ItemType;", "typed item feature")]
// item_flow
#[case(Rule::item_flow, "flow myFlow;", "simple item flow")]
#[case(Rule::item_flow, "flow myFlow from a to b;", "named item flow")]
// succession_item_flow
#[case(
    Rule::succession_item_flow,
    "succession flow;",
    "simple succession flow"
)]
#[case(
    Rule::succession_item_flow,
    "succession flow myFlow;",
    "named succession flow"
)]
// boolean_expression
#[case(Rule::boolean_expression, "expr;", "simple boolean expression")]
#[case(Rule::boolean_expression, "expr myBool;", "named boolean expression")]
fn test_parse_multiplicity_and_features(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Tests for missing critical rules

#[rstest]
#[case(Rule::file, "", "empty file")]
#[case(Rule::file, "   \n\t  \r\n  ", "file with whitespace")]
fn test_parse_file(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// float
#[case(Rule::float, "3.14", "decimal float")]
#[case(Rule::float, ".5", "leading decimal")]
#[case(Rule::float, "0.0", "zero float")]
// fraction
#[case(Rule::fraction, ".5", "simple fraction")]
#[case(Rule::fraction, ".123", "multi-digit fraction")]
#[case(Rule::fraction, ".0", "zero fraction")]
// exponent
#[case(Rule::exponent, "e10", "positive exponent")]
#[case(Rule::exponent, "E-5", "negative exponent")]
#[case(Rule::exponent, "e+3", "explicit positive exponent")]
fn test_parse_numeric_components(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// element_reference
#[case(Rule::element_reference, "myElement", "simple element reference")]
#[case(
    Rule::element_reference,
    "Base::Derived",
    "qualified element reference"
)]
#[case(
    Rule::element_reference,
    "Pkg::Sub::Element",
    "nested element reference"
)]
// type_reference
#[case(Rule::type_reference, "MyType", "simple type reference")]
#[case(Rule::type_reference, "Base::MyType", "qualified type reference")]
// feature_reference
#[case(Rule::feature_reference, "myFeature", "simple feature reference")]
#[case(
    Rule::feature_reference,
    "Base::myFeature",
    "qualified feature reference"
)]
// classifier_reference
#[case(
    Rule::classifier_reference,
    "MyClassifier",
    "simple classifier reference"
)]
#[case(
    Rule::classifier_reference,
    "Base::MyClassifier",
    "qualified classifier reference"
)]
fn test_parse_references(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// element
#[case(Rule::element, "<shortName>", "short name only")]
#[case(Rule::element, "regularName", "regular name only")]
#[case(Rule::element, "<shortName> regularName", "short and regular name")]
// annotation
#[case(Rule::annotation, "MyElement", "simple annotation")]
// owned_annotation
#[case(Rule::owned_annotation, "comment /* text */", "owned comment")]
#[case(
    Rule::owned_annotation,
    "doc /* documentation */",
    "owned documentation"
)]
fn test_parse_element_and_annotation(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Functional tests for annotation properties (reference and span)
// These verify that parsing actually populates the Annotation struct fields

#[test]
fn test_annotation_reference_field_populated() {
    // Test that parsing an annotation creates an Annotation with correct reference field
    let source = "comment about MyElement /* This is about MyElement */";

    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Verify the annotation reference is captured
    // Find the element_reference in the parsed tree
    let mut found_reference = false;
    for inner in parsed.into_inner() {
        if inner.as_rule() == syster::parser::kerml::Rule::element_reference {
            assert_eq!(inner.as_str().trim(), "MyElement");
            found_reference = true;
        }
    }
    assert!(
        found_reference,
        "Should find element_reference 'MyElement' in parsed comment annotation"
    );
}

#[test]
fn test_annotation_reference_with_qualified_name() {
    // Test annotation with qualified reference like Package::Element
    let source = "comment about Base::Vehicle /* Reference to qualified name */";

    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Verify qualified reference is captured
    let mut found_reference = false;
    for inner in parsed.into_inner() {
        if inner.as_rule() == syster::parser::kerml::Rule::element_reference {
            assert_eq!(inner.as_str().trim(), "Base::Vehicle");
            found_reference = true;
        }
    }
    assert!(
        found_reference,
        "Should find qualified element_reference 'Base::Vehicle'"
    );
}

#[test]
fn test_annotation_multiple_references() {
    // Test comment with multiple "about" references
    let source = "comment about Element1, Element2, Element3 /* Multiple references */";

    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Collect all element references
    let mut references = Vec::new();
    for inner in parsed.into_inner() {
        if inner.as_rule() == syster::parser::kerml::Rule::element_reference {
            references.push(inner.as_str().trim().to_string());
        }
    }

    assert_eq!(references.len(), 3, "Should find 3 element references");
    assert_eq!(references, vec!["Element1", "Element2", "Element3"]);
}

#[test]
fn test_annotation_span_captured() {
    // Test that annotation reference location (span) is captured
    let source = "comment about MyElement /* comment text */";

    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Find element_reference and verify it has span information
    for inner in parsed.into_inner() {
        if inner.as_rule() == syster::parser::kerml::Rule::element_reference {
            let span = inner.as_span();
            // Verify span captures the reference position
            assert!(
                span.start() < span.end(),
                "Span should have valid start/end positions"
            );
            assert_eq!(inner.as_str().trim(), "MyElement");
        }
    }
}

#[rstest]
#[case(Rule::namespace, "namespace MyNamespace;", "semicolon body")]
#[case(Rule::namespace, "namespace MyNamespace {}", "braces body")]
fn test_parse_namespace_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// High-priority missing rules

#[rstest]
#[case(Rule::type_def, "type MyType;", "simple type")]
#[case(Rule::type_def, "abstract type MyType {}", "abstract type")]
#[case(Rule::type_def, "type all MyType {}", "type with all")]
#[case(Rule::type_def, "type MyType ordered {}", "ordered type")]
#[case(Rule::type_def, "type MyType unions BaseType {}", "type with unions")]
#[case(
    Rule::type_def,
    "type MyType differences BaseType {}",
    "type with differences"
)]
fn test_parse_type_def(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::classifier, "classifier MyClassifier;", "simple classifier")]
#[case(
    Rule::classifier,
    "abstract classifier MyClassifier {}",
    "abstract classifier"
)]
#[case(
    Rule::classifier,
    "classifier all MyClassifier {}",
    "classifier with all"
)]
#[case(
    Rule::classifier,
    "classifier MyClassifier unions BaseClassifier {}",
    "classifier with unions"
)]
fn test_parse_classifier(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::operator_expression, "null", "null expression")]
#[case(Rule::operator_expression, "true", "boolean expression")]
#[case(Rule::operator_expression, "myFeature", "feature expression")]
fn test_parse_operator_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::metadata_access_expression, "obj.metadata", "simple access")]
#[case(
    Rule::metadata_access_expression,
    "Base::Feature.metadata",
    "qualified access"
)]
fn test_parse_metadata_access_expression(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::root_namespace, "", "empty root namespace")]
#[case(Rule::root_namespace, "package MyPackage;", "single package")]
#[case(
    Rule::root_namespace,
    "package Pkg1; package Pkg2;",
    "multiple packages"
)]
fn test_parse_root_namespace(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// basic invocation
#[case(Rule::invocation_expression, "null", "null literal")]
#[case(Rule::invocation_expression, "123", "numeric literal")]
#[case(Rule::invocation_expression, "size(dimensions)", "single argument")]
#[case(Rule::invocation_expression, "foo()", "no arguments")]
#[case(Rule::invocation_expression, "max(a, b)", "two arguments")]
#[case(Rule::invocation_expression, "calculate(x, y, z)", "three arguments")]
#[case(
    Rule::invocation_expression,
    "NumericalFunctions::sum0(x, y)",
    "qualified function"
)]
#[case(
    Rule::invocation_expression,
    "Namespace::Nested::func(a)",
    "nested qualified function"
)]
// with numeric arguments
#[case(Rule::invocation_expression, "rect(0.0, 1.0)", "float arguments")]
#[case(Rule::invocation_expression, "polar(1.0, 3.14)", "polar with floats")]
#[case(Rule::invocation_expression, "add(42, 17)", "integer arguments")]
fn test_parse_invocation_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
// collect expressions
#[case(Rule::inline_expression, "\"hello\"", "string literal")]
#[case(Rule::inline_expression, "\"hello\".toUpper", "string with method")]
// select expressions
#[case(Rule::inline_expression, "\"world\"", "another string")]
#[case(Rule::inline_expression, "myVar.property", "property access")]
fn test_parse_inline_expressions(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test feature with ordered/nonunique after typing and values
#[rstest]
// with modifiers after typing
#[case(
    Rule::feature,
    "feature dimensions: Positive[0..*] ordered nonunique { }",
    "ordered nonunique with body"
)]
#[case(Rule::feature, "feature x: Type ordered { }", "ordered with body")]
#[case(Rule::feature, "feature y: T nonunique { }", "nonunique with body")]
#[case(
    Rule::feature,
    "feature z: T[1] ordered nonunique;",
    "ordered nonunique semicolon"
)]
// with expression values
#[case(
    Rule::feature,
    "feature rank: Natural[1] = size(dimensions);",
    "value with function call"
)]
#[case(Rule::feature, "feature x = 3;", "simple numeric value")]
#[case(Rule::feature, "feature y = foo();", "value with invocation")]
// with invocation value
#[case(
    Rule::feature,
    "feature i: Complex[1] = rect(0.0, 1.0);",
    "complex with rect"
)]
#[case(Rule::feature, "feature x: Real[1] = sqrt(2.0);", "real with sqrt")]
fn test_parse_advanced_features(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test documentation with block comments
#[rstest]
#[case(Rule::documentation, "doc /* This is documentation */", "simple doc")]
#[case(
    Rule::documentation,
    "doc /* Multi-line\n * documentation\n */",
    "multi-line doc"
)]
#[case(Rule::documentation, "doc /* Simple */", "short doc")]
fn test_parse_documentation(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test parameter membership (function parameters)
#[rstest]
// parameter_membership
#[case(Rule::parameter_membership, "in x: Anything[0..1];", "input parameter")]
#[case(Rule::parameter_membership, "in y: Boolean[1];", "boolean input")]
#[case(
    Rule::parameter_membership,
    "out result: Natural[1];",
    "output parameter"
)]
#[case(
    Rule::parameter_membership,
    "inout value: Complex[0..*];",
    "inout parameter"
)]
#[case(
    Rule::parameter_membership,
    "in x: Anything[0..*] nonunique;",
    "nonunique input"
)]
#[case(
    Rule::parameter_membership,
    "in x: Anything[0..*] ordered;",
    "ordered input"
)]
// return_parameter_membership
#[case(
    Rule::return_parameter_membership,
    "return : Boolean[1];",
    "return boolean"
)]
#[case(
    Rule::return_parameter_membership,
    "return result: Natural[1];",
    "named return"
)]
#[case(
    Rule::return_parameter_membership,
    "return : Complex[1] = x + y;",
    "return with expression"
)]
fn test_parse_parameter_memberships(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test functions with various features
#[rstest]
// with operator names
#[case(Rule::function, "function '==' { }", "equality operator function")]
#[case(Rule::function, "function '!=' { }", "inequality operator function")]
#[case(Rule::function, "function '+' { }", "plus operator function")]
#[case(Rule::function, "abstract function '-' { }", "abstract minus function")]
// with parameters
#[case(
    Rule::function,
    "function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; return : Boolean[1]; }",
    "function with params"
)]
#[case(
    Rule::function,
    "function add { in a: Natural[1]; in b: Natural[1]; return : Natural[1]; }",
    "add function"
)]
#[case(
    Rule::function,
    "abstract function compare { in x: Anything[0..1]; in y: Anything[0..1]; return : Boolean[1]; }",
    "abstract compare"
)]
// with specialization
#[case(
    Rule::function,
    "function 'not' specializes ScalarFunctions::'not' { }",
    "not specialization"
)]
#[case(
    Rule::function,
    "function 'xor' specializes Base::'xor' { }",
    "xor specialization"
)]
fn test_parse_function_variations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test quoted identifiers
#[rstest]
// simple quoted identifiers
#[case(Rule::unrestricted_name, "'=='", "equality")]
#[case(Rule::unrestricted_name, "'!='", "inequality")]
#[case(Rule::unrestricted_name, "'+'", "plus")]
#[case(Rule::unrestricted_name, "'-'", "minus")]
#[case(Rule::unrestricted_name, "'*'", "multiply")]
#[case(Rule::unrestricted_name, "'/'", "divide")]
#[case(Rule::unrestricted_name, "'<'", "less than")]
#[case(Rule::unrestricted_name, "'>'", "greater than")]
#[case(Rule::unrestricted_name, "'<='", "less or equal")]
#[case(Rule::unrestricted_name, "'>='", "greater or equal")]
fn test_parse_quoted_identifier(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test qualified references with quoted identifiers
#[rstest]
#[case(
    Rule::qualified_reference_chain,
    "ScalarFunctions::'not'",
    "qualified not"
)]
#[case(Rule::qualified_reference_chain, "Base::'=='", "qualified equality")]
#[case(Rule::qualified_reference_chain, "Math::'+'", "qualified plus")]
#[case(Rule::qualified_reference_chain, "Ops::'*'::'nested'", "nested quoted")]
fn test_parse_qualified_reference_with_quotes(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Test more feature variations (with invocation, namespace, chaining)
#[rstest]
// feature with invocation value (already tested in advanced_features but keeping namespace_feature_member version)
#[case(
    Rule::namespace_feature_member,
    "feature i: Complex[1] = rect(0.0, 1.0);",
    "namespace feature with invocation"
)]
#[case(
    Rule::namespace_feature_member,
    "feature x: Natural[1] = 42;",
    "namespace feature with literal"
)]
// feature with chaining
#[case(
    Rule::feature,
    "feature chain chains source.target;",
    "feature with chaining"
)]
#[case(
    Rule::feature,
    "private feature chain chains source.target;",
    "private feature with chaining"
)]
fn test_parse_more_feature_variations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test return parameter variations
#[rstest]
// with default value
#[case(
    Rule::return_parameter_membership,
    "return : Integer[1] default sum0(collection, 0);",
    "return with function default"
)]
#[case(
    Rule::return_parameter_membership,
    "return : Boolean[1] default true;",
    "return with literal default"
)]
#[case(
    Rule::return_parameter_membership,
    "return result: Natural[1] default 0;",
    "named return with default"
)]
// with binary expression
#[case(
    Rule::return_parameter_membership,
    "return : Boolean[1] = x == y;",
    "return with equality"
)]
#[case(
    Rule::return_parameter_membership,
    "return : Boolean[1] = x != y;",
    "return with inequality"
)]
#[case(
    Rule::return_parameter_membership,
    "return : Boolean[1] = x < y;",
    "return with less than"
)]
fn test_parse_return_parameter_variations(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Test function with return default
#[rstest]
#[case(
    Rule::function,
    "function sum { in collection: Integer[0..*]; return : Integer[1] default sum0(collection, 0); }",
    "function with return default"
)]
fn test_parse_function_with_return_default(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}
// Test binary operator expressions
#[rstest]
#[case(Rule::operator_expression, "x == y", "equality")]
#[case(Rule::operator_expression, "x != y", "inequality")]
#[case(Rule::operator_expression, "x === y", "identity")]
#[case(Rule::operator_expression, "x < y", "less than")]
#[case(Rule::operator_expression, "x <= y", "less or equal")]
#[case(Rule::operator_expression, "x > y", "greater than")]
#[case(Rule::operator_expression, "x >= y", "greater or equal")]
#[case(Rule::operator_expression, "x + y", "addition")]
#[case(Rule::operator_expression, "x - y", "subtraction")]
#[case(Rule::operator_expression, "x * y", "multiplication")]
#[case(Rule::operator_expression, "x / y", "division")]
#[case(Rule::operator_expression, "x and y", "logical and")]
#[case(Rule::operator_expression, "x or y", "logical or")]
#[case(Rule::operator_expression, "x xor y", "logical xor")]
#[case(Rule::operator_expression, "a == b and c == d", "compound expression")]
fn test_parse_binary_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test function with special operator names
#[rstest]
#[case(
    Rule::function,
    "function '..' { in x: Integer[1]; return : Integer[1]; }",
    "range operator function"
)]
#[case(
    Rule::function,
    "function test { return : Integer[0..*]; }",
    "function with multiplicity return"
)]
#[case(
    Rule::function,
    "abstract function '..' { in lower: DataValue[1]; in upper: DataValue[1]; return : DataValue[0..*] ordered; }",
    "abstract range function"
)]
fn test_parse_function_with_range_operator(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Test expression variations
#[rstest]
// conditional expressions
#[case(Rule::operator_expression, "if true ? 1 else 0", "simple conditional")]
#[case(
    Rule::operator_expression,
    "if x > 5 ? 'yes' else 'no'",
    "conditional with comparison"
)]
#[case(
    Rule::operator_expression,
    "if isEmpty(seq)? 0 else size(tail(seq)) + 1",
    "conditional with function calls"
)]
// tuple literals
#[case(Rule::operator_expression, "(a, b)", "simple tuple")]
#[case(Rule::operator_expression, "(1, 2, 3)", "numeric tuple")]
#[case(Rule::operator_expression, "(seq1, seq2)", "sequence tuple")]
// null coalescing
#[case(Rule::operator_expression, "x ?? 0", "simple null coalescing")]
#[case(
    Rule::operator_expression,
    "dimensions->reduce '*' ?? 1",
    "null coalescing with reduce"
)]
// collection operators
#[case(
    Rule::operator_expression,
    "col->reduce '+' ?? zero",
    "reduce with null coalescing"
)]
#[case(
    Rule::operator_expression,
    "collection->select {in x; x > 0}",
    "select with filter"
)]
#[case(
    Rule::operator_expression,
    "col.elements->equals(other.elements)",
    "equals on elements"
)]
#[case(
    Rule::operator_expression,
    "coll->collect{in i : Positive; v#(i) + w#(i)}",
    "collect with indexing"
)]
// as operator
#[case(Rule::operator_expression, "x as Integer", "simple cast")]
#[case(
    Rule::operator_expression,
    "(col.elements as Anything)#(index)",
    "cast with indexing"
)]
fn test_parse_expression_variations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test character literals
#[rstest]
#[case(Rule::literal_expression, "'*'", "asterisk char")]
#[case(Rule::literal_expression, "'+'", "plus char")]
#[case(Rule::literal_expression, "'a'", "letter char")]
fn test_parse_char_literal(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test parameter variations
#[rstest]
// with default values
#[case(
    Rule::parameter_membership,
    "in x: Integer[1] default 0;",
    "parameter with literal default"
)]
#[case(
    Rule::parameter_membership,
    "in endIndex: Positive[1] default startIndex;",
    "parameter with variable default"
)]
// expression parameters
#[case(
    Rule::parameter_membership,
    "in expr thenValue[0..1] { return : Anything[0..*] ordered nonunique; }",
    "expression parameter with return"
)]
#[case(
    Rule::parameter_membership,
    "in step myStep { in x: Integer[1]; }",
    "step parameter"
)]
fn test_parse_parameter_variations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test case_22 failure: shorthand feature with typing and redefinition
#[rstest]
#[case(
    Rule::namespace_body_element,
    "private thisClock : Clock :>> self;",
    "feature with typing and redefinition"
)]
#[case(
    Rule::operator_expression,
    "snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}",
    "lambda parameter no semicolon"
)]
#[case(
    Rule::invariant,
    r#"inv timeFlowConstraint {
        doc /* comment */
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#,
    "invariant with doc and expression"
)]
#[case(
    Rule::invariant,
    r#"inv timeFlowConstraint {
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#,
    "invariant with expression"
)]
#[case(
    Rule::operator_expression,
    "w == null or isZeroVector(w) implies u == w",
    "implies operator"
)]
#[case(
    Rule::invariant,
    "inv zeroAddition { w == null or isZeroVector(w) implies u == w }",
    "invariant with implies"
)]
#[case(
    Rule::feature,
    "abstract feature dataValues: DataValue[0..*] nonunique subsets things { }",
    "feature with multiplicity props before subsetting"
)]
#[case(
    Rule::parameter_membership,
    "in indexes: Positive[n] ordered nonunique;",
    "parameter with identifier multiplicity"
)]
#[case(
    Rule::return_parameter_membership,
    "return : NumericalVectorValue[1] { }",
    "return parameter with body"
)]
#[case(
    Rule::multiplicity,
    "multiplicity exactlyOne [1..1] { }",
    "multiplicity with identification and bounds"
)]
#[case(
    Rule::feature,
    "derived var feature annotatedElement : Element[1..*] ordered redefines annotatedElement;",
    "feature with var modifier"
)]
#[case(
    Rule::shorthand_feature_member,
    ":>> dimension = size(components);",
    "shorthand feature with redefines and default"
)]
#[case(
    Rule::parameter_membership,
    "in redefines ifTest;",
    "parameter with only redefines"
)]
#[case(
    Rule::succession,
    "succession [1] ifTest then [0..1] thenClause { }",
    "succession with multiplicity"
)]
#[case(
    Rule::binding_connector,
    "binding [1] whileDecision.ifTest = [1] whileTest { }",
    "binding with multiplicity and endpoints"
)]
#[case(
    Rule::binding_connector,
    "binding loopBack of [0..1] untilDecision.elseClause = [1] whileDecision { }",
    "binding with of keyword"
)]
#[case(
    Rule::return_parameter_membership,
    "return resultValues : Anything [*] nonunique redefines result redefines values;",
    "return parameter with multiple redefines"
)]
fn test_parse_complex_kerml_patterns(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test expression with visibility and typing
#[rstest]
#[case(
    Rule::expression,
    "protected expr monitoredOccurrence : Evaluation [1] redefines monitoredOccurrence { }",
    "expression with visibility and typing"
)]
#[case(
    Rule::parameter_membership,
    "in bool redefines onOccurrence { }",
    "parameter with bool type"
)]
#[case(
    Rule::parameter_membership,
    "in indexes: Positive[n] ordered nonunique;",
    "parameter with multiplicity props after type"
)]
#[case(
    Rule::typed_feature_member,
    "protected bool redefines monitoredOccurrence[1] { }",
    "typed feature member"
)]
#[case(
    Rule::collect_operation_args,
    "{in i; i > 0}",
    "lambda with inline parameter"
)]
#[case(Rule::collect_operation_args, "{i > 0}", "lambda no parameters")]
#[case(Rule::parameter_membership, "in x y { }", "simple parameter")]
#[case(
    Rule::feature,
    "end feature thisThing: Anything redefines source subsets sameThing crosses sameThing.self;",
    "cross subsetting with feature chain"
)]
#[case(
    Rule::end_feature,
    "end self2 [1] feature sameThing: Anything redefines target subsets thisThing;",
    "end feature with mult"
)]
#[case(
    Rule::step,
    "abstract step enactedPerformances: Performance[0..*] subsets involvingPerformances, timeEnclosedOccurrences { }",
    "step with multiple subsets"
)]
#[case(
    Rule::comment_annotation,
    "comment about StructuredSurface, StructuredCurve, StructuredPoint /* multi-element comment */",
    "comment with multiple about"
)]
#[case(
    Rule::class,
    "abstract class Occurrence specializes Anything disjoint from DataValue { }",
    "disjoining with from"
)]
#[case(
    Rule::subset_member,
    "subset laterOccurrence.successors subsets earlierOccurrence.successors;",
    "subset member"
)]
#[case(
    Rule::typed_feature_member,
    "bool guard[*] subsets enclosedPerformances;",
    "typed feature mult before relationships"
)]
#[case(
    Rule::binding_connector,
    "binding accept.receiver = triggerTarget;",
    "binding with feature chain"
)]
#[case(
    Rule::end_feature_membership,
    "end bool constrainedGuard;",
    "end typed feature"
)]
#[case(
    Rule::connector,
    "connector :HappensDuring from [1] shorterOccurrence references thisOccurrence to [1] longerOccurrence references thatOccurrence;",
    "connector from to endpoints"
)]
#[case(
    Rule::return_parameter_membership,
    "return feature changeSignal : ChangeSignal[1] = new ChangeSignal(condition, monitor) {}",
    "return feature parameter"
)]
#[case(
    Rule::end_feature,
    "end [1] feature transferSource references source;",
    "end feature mult first"
)]
fn test_parse_kerml_feature_patterns(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Test disjoint with feature chains (disjoining rule doesn't include semicolon)
#[test]
fn test_parse_disjoint_feature_chains_from() {
    assert_round_trip(
        Rule::disjoining,
        "disjoint earlierOccurrence.successors from laterOccurrence.predecessors",
        "disjoint feature chains",
    );
}

// Test abstract flow with typed feature pattern
#[rstest]
#[case(
    Rule::item_flow,
    "abstract flow flowTransfers: FlowTransfer[0..*] nonunique subsets transfers {}",
    "abstract flow"
)]
#[case(
    Rule::operator_expression,
    "subp istype StatePerformance",
    "istype operator"
)]
#[case(
    Rule::end_feature,
    "end happensWhile [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence redefines longerOccurrence;",
    "end feature with relationships before feature"
)]
#[case(
    Rule::collect_operation_args,
    "{in s : Clock; TimeOf(s, thisClock) == s.currentTime}",
    "collect args with in"
)]
#[case(
    Rule::namespace_body,
    r#"{
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#,
    "namespace body with expression"
)]
#[case(
    Rule::namespace_body,
    r#"{
        doc /* comment */
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#,
    "namespace body with doc and expression"
)]
#[case(Rule::annotating_member, "doc /* comment */", "annotating member doc")]
#[case(
    Rule::namespace_body_elements,
    r#"doc /* comment */
        x"#,
    "two namespace elements"
)]
#[case(
    Rule::namespace_body,
    r#"{
        doc /* comment */
        x
    }"#,
    "doc then simple expr"
)]
#[case(
    Rule::namespace_body,
    r#"{
        doc /* comment */
        x->y
    }"#,
    "doc then arrow expr"
)]
#[case(
    Rule::namespace_body_element,
    "snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}",
    "namespace body element expression"
)]
#[case(Rule::namespace_body_element, "x->y", "arrow expr as element")]
#[case(Rule::namespace_body, "{ x->y }", "arrow expr in body no doc")]
#[case(
    Rule::namespace_body_elements,
    r#"doc /* comment */
x->y"#,
    "elements doc then arrow"
)]
fn test_parse_kerml_namespace_patterns(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}
