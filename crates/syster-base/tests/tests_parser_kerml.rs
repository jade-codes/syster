#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use pest::Parser;
use rstest::rstest;
use syster::parser::KerMLParser;
use syster::parser::kerml::Rule;
use syster::syntax::kerml::enums::*;
use syster::syntax::kerml::types::*;
// For AST parsing tests - import with alias to avoid conflict with model::Element
use syster::syntax::kerml::ast::{
    ClassifierKind, ClassifierMember, Element as AstElement, FeatureMember,
};

/// Helper function to assert that parsing succeeds and the entire input is consumed.
/// This ensures the parser doesn't just match a prefix of the input.
///
/// The function verifies that:
/// 1. Parsing succeeds
/// 2. Exactly one top-level pair is produced (in most cases)
/// 3. The parsed output matches the original input exactly
fn assert_round_trip(rule: Rule, input: &str, desc: &str) {
    let result = KerMLParser::parse(rule, input)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", desc, e));

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

    assert_eq!(input, parsed, "Parsed output mismatch for {}", desc);
}

#[test]
fn test_parse_kerml_identifier() {
    let input = "myVar";
    let pairs = KerMLParser::parse(Rule::identifier, input).unwrap();
    let identifier = pairs.into_iter().next().unwrap();
    assert_eq!(identifier.as_str(), "myVar");
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

#[test]
fn test_parse_kerml_line_comment() {
    assert_round_trip(Rule::line_comment, "// this is a comment", "line comment");
}

#[test]
fn test_parse_kerml_block_comment() {
    assert_round_trip(Rule::block_comment, "/* block comment */", "block comment");
}

// Enum Conversion Tests
#[rstest]
#[case("private", VisibilityKind::Private)]
#[case("protected", VisibilityKind::Protected)]
#[case("public", VisibilityKind::Public)]
fn test_visibility_kind_to_enum(#[case] input: &str, #[case] expected: VisibilityKind) {
    let pairs = KerMLParser::parse(Rule::visibility_kind, input).unwrap();
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
    let pairs = KerMLParser::parse(Rule::unary_operator, input).unwrap();
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
    let pairs = KerMLParser::parse(Rule::classification_test_operator, input).unwrap();
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
    let pairs = KerMLParser::parse(Rule::equality_operator, input).unwrap();
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
    let pairs = KerMLParser::parse(Rule::import_kind, input).unwrap();
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
    let pairs = KerMLParser::parse(Rule::relational_operator, input).unwrap();
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
    let pairs = KerMLParser::parse(Rule::enum_type, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Verify we got an enum_type node
    assert_eq!(parsed.as_rule(), Rule::enum_type);

    // The inner rule should be one of the specific enum types
    let inner = parsed.into_inner().next().unwrap();
    assert!(matches!(
        inner.as_rule(),
        Rule::visibility_kind
            | Rule::feature_direction_kind
            | Rule::unary_operator
            | Rule::classification_test_operator
            | Rule::equality_operator
            | Rule::import_kind
            | Rule::relational_operator
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
#[case("myName", "myName")]
#[case("'unrestricted name'", "'unrestricted name'")]
fn test_parse_name(#[case] input: &str, #[case] expected: &str) {
    let _pairs = KerMLParser::parse(Rule::name, input).unwrap();
    assert_round_trip(Rule::name, input, expected);
}

#[test]
fn test_parse_string_value() {
    assert_round_trip(Rule::string_value, r#""hello world""#, "string value");
}

// Identification Tests

// Relationship Token Tests

// Common Fragment Tests

#[test]
fn test_parse_abstract_marker() {
    assert_round_trip(Rule::abstract_marker, "abstract", "abstract marker");
}

#[test]
fn test_parse_const_modifier() {
    assert_round_trip(Rule::const_modifier, "const", "const modifier");
}

/// Tests that keyword modifiers require whitespace before the next token.
/// "constfeature" should NOT parse as "const feature".
#[rstest]
#[case("constfeature MyFeature;")]
#[case("derivedfeature MyFeature;")]
fn test_modifier_requires_space_before_feature(#[case] input: &str) {
    let result = KerMLParser::parse(Rule::feature, input);
    assert!(
        result.is_err(),
        "Should reject '{}' - modifier must have space before 'feature'",
        input
    );
}

/// Tests that valid modifier + feature syntax is accepted
#[rstest]
#[case("const feature MyFeature;")]
#[case("derived feature MyFeature;")]
#[case("const derived feature MyFeature;")]
fn test_modifier_with_space_before_feature(#[case] input: &str) {
    let result = KerMLParser::parse(Rule::feature, input);
    assert!(
        result.is_ok(),
        "Should accept '{}': {:?}",
        input,
        result.err()
    );
}

#[test]
fn test_parse_sufficient() {
    assert_round_trip(Rule::sufficient, "all", "sufficient");
}

#[rstest]
#[case("true", "true")]
#[case("false", "false")]
fn test_parse_literal_boolean(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(Rule::literal_boolean, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[test]
fn test_parse_literal_string() {
    assert_round_trip(Rule::literal_string, r#""test string""#, "literal string");
}

#[test]
fn test_parse_literal_infinity() {
    assert_round_trip(Rule::literal_infinity, "*", "literal infinity");
}

#[rstest]
#[case("null", "null")]
#[case("()", "()")]
fn test_parse_null_expression(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::null_expression, input, expected);
}

// TDD: Test shorthand collect `.{` and select `.?{` expressions
#[rstest]
#[case("x.{in xx; xx + 1}", "x.{in xx; xx + 1}")] // shorthand collect
#[case("x.?{in xx; xx != null}", "x.?{in xx; xx != null}")] // shorthand select
#[case("x->collect {in xx; xx + 1}", "x->collect {in xx; xx + 1}")] // explicit collect
#[case("x->select {in xx; xx != null}", "x->select {in xx; xx != null}")] // explicit select
fn test_shorthand_collect_select_expressions(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::operator_expression, input, expected);
}

// TDD: Test standalone redefinition with feature chains
#[rstest]
#[case("redefinition b.f redefines b.a;", "redefinition b.f redefines b.a;")]
#[case("redefinition a :>> b;", "redefinition a :>> b;")]
#[case(
    "specialization id redefinition a.b redefines c.d { }",
    "specialization id redefinition a.b redefines c.d { }"
)]
fn test_standalone_redefinition(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::relationship_element, input, expected);
}

// TDD: Test standalone subtype (specialization) with feature chains
#[rstest]
#[case("subtype g.g specializes b.f.a;", "subtype g.g specializes b.f.a;")]
#[case("subtype A :> B;", "subtype A :> B;")]
#[case(
    "specialization id subtype A specializes B { }",
    "specialization id subtype A specializes B { }"
)]
fn test_standalone_subtype(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::relationship_element, input, expected);
}

// TDD: Test type_featuring with full syntax: featuring (Identification? of)? feature by type
#[rstest]
#[case("featuring F of y by C;", "featuring F of y by C;")]
#[case("featuring y by C;", "featuring y by C;")]
#[case("featuring of x by T;", "featuring of x by T;")]
fn test_type_featuring_full_syntax(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::relationship_element, input, expected);
}

// TDD: Test standalone feature typing: ('specialization' id?)? 'typing' feature (':' | 'typed by') Type
#[rstest]
#[case(
    "specialization t1 typing f typed by B;",
    "specialization t1 typing f typed by B;"
)]
#[case("typing x : A;", "typing x : A;")]
#[case("typing f typed by T;", "typing f typed by T;")]
fn test_standalone_feature_typing(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::relationship_element, input, expected);
}

// TDD: Test classification expression with @ prefix (implicit self operand)
#[rstest]
#[case("@Structure", "@Structure")] // implicit self istype Structure
#[case("@@MetaClass", "@@MetaClass")] // meta classification test
#[case("hastype T", "hastype T")] // explicit hastype
#[case("istype T", "istype T")] // explicit istype
#[case("x @ T", "x @ T")] // explicit operand with @ operator
fn test_classification_expression_prefix(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::operator_expression, input, expected);
}

// TDD: Test standalone feature inverting: ('inverting' id?)? 'inverse' feature 'of' feature
#[rstest]
#[case("inverse B::g of A::f;", "inverse B::g of A::f;")]
#[case(
    "inverting Invert inverse B::g.f of A::h;",
    "inverting Invert inverse B::g.f of A::h;"
)]
#[case("inverse a of b;", "inverse a of b;")]
fn test_standalone_feature_inverting(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::relationship_element, input, expected);
}

// TDD: Test inline inverse of in feature declarations
#[rstest]
#[case("inverse of B::g", "inverse of B::g")]
#[case("inverse of a.b", "inverse of a.b")]
fn test_owned_feature_inverting(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::owned_feature_inverting, input, expected);
}

// TDD: Test class with all modifier and multiplicity
#[rstest]
#[case(
    "class all JohnLife[0..1] specializes John;",
    "class all JohnLife[0..1] specializes John;"
)]
#[case("class MyClass[1] :> Base { }", "class MyClass[1] :> Base { }")]
fn test_class_with_all_and_multiplicity(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::class, input, expected);
}

// TDD: Test feature with prefix metadata instead of feature keyword
#[rstest]
#[case("abstract #Classified z2;", "abstract #Classified z2;")]
#[case("#Security feature z;", "#Security feature z;")]
#[case(
    "private #Classified #Security feature z1;",
    "private #Classified #Security feature z1;"
)]
fn test_feature_with_prefix_metadata(#[case] input: &str, #[case] expected: &str) {
    assert_round_trip(Rule::namespace_body_element, input, expected);
}

#[rstest]
#[case("in")]
#[case("out")]
#[case("inout")]
fn test_parse_feature_direction_kind(#[case] input: &str) {
    assert_round_trip(Rule::feature_direction_kind, input, input);
}

// Additional Common Fragment Tests

#[test]
fn test_parse_derived() {
    assert_round_trip(Rule::derived, "derived", "derived");
}

#[test]
fn test_parse_end_marker() {
    assert_round_trip(Rule::end_marker, "end", "end marker");
}

#[test]
fn test_parse_standard() {
    assert_round_trip(Rule::standard_marker, "standard", "standard marker");
}

#[test]
fn test_parse_import_all() {
    assert_round_trip(Rule::import_all, "all", "import all");
}

// Reference Tests

// Additional Token Tests

// Additional Expression and Metadata Tests

// Body Structure Tests

#[test]
fn test_parse_block_comment() {
    assert_round_trip(Rule::block_comment, "/* textual body */", "block comment");
}

// Import and Filter Tests

// Relationship Declaration Tests

// Element Declaration Tests

// Feature Tests

// Annotation Element Tests

// Multiplicity tests

// MultiplicityRange tests

// MetadataFeature tests

// ItemFeature tests

// ItemFlow tests

// SuccessionItemFlow tests

// BooleanExpression tests

// Tests for missing critical rules

#[test]
fn test_parse_file_empty() {
    let input = "";
    let result = KerMLParser::parse(Rule::file, input);
    assert!(
        result.is_ok(),
        "Failed to parse empty file: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_file_with_whitespace() {
    let input = "   \n\t  \r\n  ";
    let result = KerMLParser::parse(Rule::file, input);
    assert!(
        result.is_ok(),
        "Failed to parse file with whitespace: {:?}",
        result.err()
    );
}

// Functional tests for annotation properties (reference and span)
// These verify that parsing actually populates the Annotation struct fields

#[test]
fn test_annotation_reference_field_populated() {
    // Test that parsing an annotation creates an Annotation with correct reference field
    let source = "comment about MyElement /* This is about MyElement */";

    let pairs = KerMLParser::parse(Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Verify the annotation reference is captured
    // Find the element_reference in the parsed tree
    let mut found_reference = false;
    for inner in parsed.into_inner() {
        if inner.as_rule() == Rule::element_reference {
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

    let pairs = KerMLParser::parse(Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Verify qualified reference is captured
    let mut found_reference = false;
    for inner in parsed.into_inner() {
        if inner.as_rule() == Rule::element_reference {
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

    let pairs = KerMLParser::parse(Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Collect all element references
    let mut references = Vec::new();
    for inner in parsed.into_inner() {
        if inner.as_rule() == Rule::element_reference {
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

    let pairs = KerMLParser::parse(Rule::comment_annotation, source).unwrap();
    let parsed = pairs.into_iter().next().unwrap();

    // Find element_reference and verify it has span information
    for inner in parsed.into_inner() {
        if inner.as_rule() == Rule::element_reference {
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
#[case("namespace MyNamespace;")]
#[case("namespace MyNamespace {}")]
fn test_parse_namespace_body(#[case] input: &str) {
    let pairs = KerMLParser::parse(Rule::namespace, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    // Verify namespace rule was matched and input was fully consumed
    assert_eq!(parsed.as_rule(), Rule::namespace);
    assert_eq!(parsed.as_str(), input);
}

// High-priority missing rules

#[test]
fn test_parse_root_namespace_empty() {
    let input = "";
    let result = KerMLParser::parse(Rule::root_namespace, input);
    assert!(
        result.is_ok(),
        "Failed to parse empty root namespace: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_root_namespace_with_package() {
    let input = "package MyPackage;";
    let pairs = KerMLParser::parse(Rule::root_namespace, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_rule(), Rule::root_namespace);
    // Verify the input was fully consumed
    assert_eq!(parsed.as_str(), input);
}

#[test]
fn test_parse_root_namespace_with_multiple_elements() {
    let input = "package Pkg1; package Pkg2;";
    let pairs = KerMLParser::parse(Rule::root_namespace, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_rule(), Rule::root_namespace);
    // Verify the entire input with multiple packages was parsed
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("null")]
#[case("123")]
#[case("size(dimensions)")]
#[case("foo()")]
#[case("max(a, b)")]
#[case("calculate(x, y, z)")]
#[case("NumericalFunctions::sum0(x, y)")]
#[case("Namespace::Nested::func(a)")]
fn test_parse_invocation_expression(#[case] input: &str) {
    assert_round_trip(Rule::invocation_expression, input, input);
}

#[rstest]
#[case("\"hello\"")]
#[case("\"hello\".toUpper")]
fn test_parse_collect_expression(#[case] input: &str) {
    // collect_expression is in inline_expression union
    assert_round_trip(Rule::inline_expression, input, input);
}

#[rstest]
#[case("\"world\"")]
#[case("myVar.property")]
fn test_parse_select_expression(#[case] input: &str) {
    // select_expression is in inline_expression union
    assert_round_trip(Rule::inline_expression, input, input);
}

// Test feature with ordered/nonunique after typing

// Test feature value with expressions
#[rstest]
#[case("feature rank: Natural[1] = size(dimensions);")]
#[case("feature x = 3;")]
#[case("feature y = foo();")]
fn test_parse_feature_value_with_expression(#[case] input: &str) {
    assert_round_trip(Rule::feature, input, input);
}

// Test documentation with block comments

// Test parameter membership (function parameters)

// Test return parameter membership

// Test functions with quoted operator names

// Test complete function with parameters

// Test quoted identifiers (unrestricted names)

// Test qualified references with quoted identifiers

// Test function specialization with quoted names

// Test invocation with numeric arguments
#[rstest]
#[case("rect(0.0, 1.0)")]
#[case("polar(1.0, 3.14)")]
#[case("add(42, 17)")]
fn test_parse_invocation_with_numbers(#[case] input: &str) {
    assert_round_trip(Rule::invocation_expression, input, input);
}

// Test feature with invocation value
#[rstest]
#[case("feature i: Complex[1] = rect(0.0, 1.0);")]
#[case("feature x: Real[1] = sqrt(2.0);")]
fn test_parse_feature_with_invocation_value(#[case] input: &str) {
    assert_round_trip(Rule::feature, input, input);
}

// Test top-level feature (namespace feature member)
#[rstest]
#[case("feature i: Complex[1] = rect(0.0, 1.0);")]
#[case("feature x: Natural[1] = 42;")]
fn test_parse_namespace_feature_with_value(#[case] input: &str) {
    assert_round_trip(Rule::namespace_feature_member, input, input);
}

// Test feature with chaining relationship

// Test return parameter with default value
#[rstest]
#[case("return : Integer[1] default sum0(collection, 0);")]
#[case("return : Boolean[1] default true;")]
#[case("return result: Natural[1] default 0;")]
fn test_parse_return_parameter_with_default(#[case] input: &str) {
    assert_round_trip(Rule::return_parameter_membership, input, input);
}

// Test function with return default
#[rstest]
#[case(
    "function sum { in collection: Integer[0..*]; return : Integer[1] default sum0(collection, 0); }"
)]
fn test_parse_function_with_return_default(#[case] input: &str) {
    assert_round_trip(Rule::function, input, input);
}
// Test binary operator expressions

// Test return with binary expression

// Test function with special operator names

// Test conditional expressions
#[rstest]
#[case("if true ? 1 else 0")]
#[case("if x > 5 ? 'yes' else 'no'")]
#[case("if isEmpty(seq)? 0 else size(tail(seq)) + 1")]
fn test_parse_conditional_expression(#[case] input: &str) {
    assert_round_trip(Rule::operator_expression, input, input);
}

// Test tuple literals
#[rstest]
#[case("(a, b)")]
#[case("(1, 2, 3)")]
#[case("(seq1, seq2)")]
fn test_parse_tuple_expression(#[case] input: &str) {
    assert_round_trip(Rule::operator_expression, input, input);
}

// Test null coalescing operator

// Test arrow operator for collections
#[rstest]
#[case("col->reduce '+' ?? zero")]
#[case("collection->select {in x; x > 0}")]
#[case("col.elements->equals(other.elements)")]
#[case("coll->collect{in i : Positive; v#(i) + w#(i)}")]
fn test_parse_collection_operators(#[case] input: &str) {
    assert_round_trip(Rule::operator_expression, input, input);
}

// Test as operator for type casting
#[rstest]
#[case("x as Integer")]
#[case("(col.elements as Anything)#(index)")]
fn test_parse_as_operator(#[case] input: &str) {
    assert_round_trip(Rule::operator_expression, input, input);
}

// Test character literals

// Test parameters with default values

// Test expression parameters

// Test case_22 failure: shorthand feature with typing and redefinition
#[test]
fn test_parse_feature_with_typing_and_redefinition() {
    let input = "private thisClock : Clock :>> self;";
    // This should parse as a namespace_body_element
    let result = KerMLParser::parse(Rule::namespace_body_element, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test lambda parameter without trailing semicolon
#[test]
fn test_parse_lambda_parameter_no_semicolon() {
    let input = "snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}";
    let result = KerMLParser::parse(Rule::operator_expression, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test invariant with doc and expression body
#[test]
fn test_parse_invariant_with_doc_and_expression() {
    let input = r#"inv timeFlowConstraint {
        doc /* comment */
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#;
    let result = KerMLParser::parse(Rule::invariant, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test invariant with doc and expression body
#[test]
fn test_parse_invariant_with_expression() {
    let input = r#"inv timeFlowConstraint {
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#;
    let result = KerMLParser::parse(Rule::invariant, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test implies operator
#[test]
fn test_parse_implies_operator() {
    let input = "w == null or isZeroVector(w) implies u == w";
    let result = KerMLParser::parse(Rule::operator_expression, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test invariant with implies in body
#[test]
fn test_parse_invariant_with_implies() {
    let input = "inv zeroAddition { w == null or isZeroVector(w) implies u == w }";
    let result = KerMLParser::parse(Rule::invariant, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test feature with ordered/nonunique before subsetting
#[test]
fn test_parse_feature_with_multiplicity_props_before_subsetting() {
    let input = "abstract feature dataValues: DataValue[0..*] nonunique subsets things { }";
    let result = KerMLParser::parse(Rule::feature, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test parameter with identifier in multiplicity bounds and ordered/nonunique after
#[test]
fn test_parse_parameter_with_identifier_multiplicity() {
    let input = "in indexes: Positive[n] ordered nonunique;";
    let result = KerMLParser::parse(Rule::parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test return parameter with body
#[test]
fn test_parse_return_parameter_with_body() {
    let input = "return : NumericalVectorValue[1] { }";
    let result = KerMLParser::parse(Rule::return_parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test multiplicity with identification and bounds
#[test]
fn test_parse_multiplicity_with_identification_and_bounds() {
    let input = "multiplicity exactlyOne [1..1] { }";
    let result = KerMLParser::parse(Rule::multiplicity, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test feature with var modifier
#[test]
fn test_parse_feature_with_var_modifier() {
    let input =
        "derived var feature annotatedElement : Element[1..*] ordered redefines annotatedElement;";
    let result = KerMLParser::parse(Rule::feature, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test shorthand feature with redefinition and default value
#[test]
fn test_parse_shorthand_feature_with_redefines_and_default() {
    let input = ":>> dimension = size(components);";
    let result = KerMLParser::parse(Rule::shorthand_feature_member, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test parameter with only redefinition, no identifier
#[test]
fn test_parse_parameter_with_only_redefines() {
    let input = "in redefines ifTest;";
    let result = KerMLParser::parse(Rule::parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test succession with multiplicity on succession and endpoints
#[test]
fn test_parse_succession_with_multiplicity() {
    let input = "succession [1] ifTest then [0..1] thenClause { }";
    let result = KerMLParser::parse(Rule::succession, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test binding with multiplicity and endpoints
#[test]
fn test_parse_binding_with_multiplicity_and_endpoints() {
    let input = "binding [1] whileDecision.ifTest = [1] whileTest { }";
    let result = KerMLParser::parse(Rule::binding_connector, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test binding with "of" keyword (type featuring)
#[test]
fn test_parse_binding_with_of_keyword() {
    let input = "binding loopBack of [0..1] untilDecision.elseClause = [1] whileDecision { }";
    let result = KerMLParser::parse(Rule::binding_connector, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test return parameter with multiple redefines after multiplicity properties
#[test]
fn test_parse_return_parameter_with_multiple_redefines() {
    let input = "return resultValues : Anything [*] nonunique redefines result redefines values;";
    let result = KerMLParser::parse(Rule::return_parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test expression with visibility and typing
#[test]
fn test_parse_expression_with_visibility_and_typing() {
    let input =
        "protected expr monitoredOccurrence : Evaluation [1] redefines monitoredOccurrence { }";
    let result = KerMLParser::parse(Rule::expression, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test parameter with bool type and only redefines
#[test]
fn test_parse_parameter_with_bool_type() {
    let input = "in bool redefines onOccurrence { }";
    let result = KerMLParser::parse(Rule::parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test parameter with ordered/nonunique after type
#[test]
fn test_parse_parameter_with_multiplicity_props_after_type() {
    let input = "in indexes: Positive[n] ordered nonunique;";
    let result = KerMLParser::parse(Rule::parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test typed feature shorthand: bool redefines x[1] { }
#[test]
fn test_parse_typed_feature_member() {
    let input = "protected bool redefines monitoredOccurrence[1] { }";
    let result = KerMLParser::parse(Rule::typed_feature_member, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test lambda expression with inline parameter: {in i; body}
#[test]
fn test_parse_lambda_with_inline_parameter() {
    let input = "{in i; i > 0}";
    let result = KerMLParser::parse(Rule::collect_operation_args, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test lambda without parameters
#[test]
fn test_parse_lambda_no_parameters() {
    let input = "{i > 0}";
    let result = KerMLParser::parse(Rule::collect_operation_args, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test simple parameter: in i;
#[test]
fn test_parse_simple_parameter() {
    let input = "in x y { }";
    let result = KerMLParser::parse(Rule::parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test feature with crosses and feature chain: crosses sameThing.self
#[test]
fn test_parse_cross_subsetting_with_feature_chain() {
    let input = "end feature thisThing: Anything redefines source subsets sameThing crosses sameThing.self;";
    let result = KerMLParser::parse(Rule::feature, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test end feature with identification and multiplicity
#[test]
fn test_parse_end_feature_with_mult() {
    let input = "end self2 [1] feature sameThing: Anything redefines target subsets thisThing;";
    let result = KerMLParser::parse(Rule::end_feature, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test step with multiple subsetting targets
#[test]
fn test_parse_step_with_multiple_subsets() {
    let input = "abstract step enactedPerformances: Performance[0..*] subsets involvingPerformances, timeEnclosedOccurrences { }";
    let result = KerMLParser::parse(Rule::step, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test comment with multiple about targets
#[test]
fn test_parse_comment_with_multiple_about() {
    let input =
        "comment about StructuredSurface, StructuredCurve, StructuredPoint /* comment body */";
    let result = KerMLParser::parse(Rule::comment_annotation, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test disjoint from syntax
#[test]
fn test_parse_disjoining_with_from() {
    let input = "abstract class Occurrence specializes Anything disjoint from DataValue { }";
    let result = KerMLParser::parse(Rule::class, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test subset member shorthand
#[test]
fn test_parse_subset_member() {
    let input = "subset laterOccurrence.successors subsets earlierOccurrence.successors;";
    let result = KerMLParser::parse(Rule::subset_member, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test typed feature with multiplicity before relationships
#[test]
fn test_parse_typed_feature_mult_before_relationships() {
    let input = "bool guard[*] subsets enclosedPerformances;";
    let result = KerMLParser::parse(Rule::typed_feature_member, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test binding with feature chain
#[test]
fn test_parse_binding_with_feature_chain() {
    let input = "binding accept.receiver = triggerTarget;";
    let result = KerMLParser::parse(Rule::binding_connector, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
// Test end with typed feature member: end bool name;
#[test]
fn test_parse_end_typed_feature() {
    let input = "end bool constrainedGuard;";
    let result = KerMLParser::parse(Rule::end_feature_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
// Test disjoint with feature chains and from: disjoint a.b from c.d
#[test]
fn test_parse_disjoint_feature_chains_from() {
    let input = "disjoint earlierOccurrence.successors from laterOccurrence.predecessors;";
    let result = KerMLParser::parse(Rule::disjoining, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
// Test connector with from/to endpoints
#[test]
fn test_parse_connector_from_to_endpoints() {
    let input = "connector :HappensDuring from [1] shorterOccurrence references thisOccurrence to [1] longerOccurrence references thatOccurrence;";
    let result = KerMLParser::parse(Rule::connector, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
// Test return feature parameter
#[test]
fn test_parse_return_feature_parameter() {
    let input =
        "return feature changeSignal : ChangeSignal[1] = new ChangeSignal(condition, monitor) {}";
    let result = KerMLParser::parse(Rule::return_parameter_membership, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
// Test end feature with multiplicity first: end [1] feature name ...
#[test]
fn test_parse_end_feature_mult_first() {
    let input = "end [1] feature transferSource ::> source;";
    let result = KerMLParser::parse(Rule::end_feature, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
// Test abstract flow with typed feature pattern
#[test]
fn test_parse_abstract_flow() {
    let input = "abstract flow flowTransfers: FlowTransfer[0..*] nonunique subsets transfers {}";
    let result = KerMLParser::parse(Rule::item_flow, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}
// Test istype operator in expression
#[test]
fn test_parse_istype_operator() {
    let input = "subp istype StatePerformance";
    let result = KerMLParser::parse(Rule::operator_expression, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// Test end feature with relationships before feature keyword
#[test]
fn test_parse_end_feature_with_index_before_feature() {
    let input = "end happensWhile [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence redefines longerOccurrence;";
    let result = KerMLParser::parse(Rule::end_feature, input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
}

// TEMPORARY DEBUG TESTS
#[test]
fn test_collect_args_with_in() {
    let input = "{in s : Clock; TimeOf(s, thisClock) == s.currentTime}";
    let result = KerMLParser::parse(Rule::collect_operation_args, input);
    assert!(
        result.is_ok(),
        "collect_operation_args failed: {:?}",
        result.err()
    );
}

#[test]
fn test_namespace_body_with_expression() {
    let input = r#"{
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#;
    let result = KerMLParser::parse(Rule::namespace_body, input);
    assert!(result.is_ok(), "namespace_body failed: {:?}", result.err());
}

#[test]
fn test_namespace_body_with_doc_and_expression() {
    let input = r#"{
        doc /* comment */
        snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}
    }"#;
    let result = KerMLParser::parse(Rule::namespace_body, input);
    assert!(
        result.is_ok(),
        "namespace_body with doc failed: {:?}",
        result.err()
    );
}

#[test]
fn test_annotating_member_doc() {
    let input = "doc /* comment */";
    let result = KerMLParser::parse(Rule::annotating_member, input);
    assert!(
        result.is_ok(),
        "annotating_member failed: {:?}",
        result.err()
    );
}

#[test]
fn test_two_namespace_elements() {
    let input = r#"doc /* comment */
        x"#;
    let result = KerMLParser::parse(Rule::namespace_body_elements, input);
    assert!(result.is_ok(), "two elements failed: {:?}", result.err());
}

#[test]
fn test_doc_then_simple_expr() {
    let input = r#"{
        doc /* comment */
        x
    }"#;
    let result = KerMLParser::parse(Rule::namespace_body, input);
    assert!(
        result.is_ok(),
        "doc then simple expr failed: {:?}",
        result.err()
    );
}

#[test]
fn test_doc_then_arrow_expr() {
    let input = r#"{
        doc /* comment */
        x->y
    }"#;
    let result = KerMLParser::parse(Rule::namespace_body, input);
    assert!(
        result.is_ok(),
        "doc then arrow expr failed: {:?}",
        result.err()
    );
}

#[test]
fn test_namespace_body_element_expression() {
    let input = "snapshots->forAll{in s : Clock; TimeOf(s, thisClock) == s.currentTime}";
    let result = KerMLParser::parse(Rule::namespace_body_element, input);
    assert!(
        result.is_ok(),
        "namespace_body_element failed: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_expr_as_element() {
    let input = "x->y";
    let result = KerMLParser::parse(Rule::namespace_body_element, input);
    assert!(
        result.is_ok(),
        "arrow expr as element failed: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_expr_in_body_no_doc() {
    let input = "{ x->y }";
    let result = KerMLParser::parse(Rule::namespace_body, input);
    assert!(
        result.is_ok(),
        "arrow expr in body no doc failed: {:?}",
        result.err()
    );
}

#[test]
fn test_elements_doc_then_arrow() {
    let input = r#"doc /* comment */
x->y"#;
    let result = KerMLParser::parse(Rule::namespace_body_elements, input);
    assert!(
        result.is_ok(),
        "elements doc then arrow failed: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_scalar_values_stdlib_file() {
    let content = r#"standard library package ScalarValues {
    private import Base::DataValue;
    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
}"#;

    let pairs = KerMLParser::parse(Rule::file, content).unwrap();
    for pair in pairs.clone() {
        for inner in pair.into_inner() {
            for _inner2 in inner.into_inner() {}
        }
    }

    // Try to convert to KerMLFile
    use syster::syntax::kerml::ast::parse_file;

    let mut pairs = KerMLParser::parse(Rule::file, content).unwrap();
    let file = parse_file(&mut pairs).unwrap();
    for _elem in file.elements.iter() {}

    assert!(!file.elements.is_empty(), "File should have elements!");
}

// ============================================================================
// AST Parsing Tests - Verify correct AST structure construction
// ============================================================================

#[test]
fn test_parse_classifier_with_specialization_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "classifier Car specializes Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Car".to_string()));
            assert_eq!(c.body.len(), 1, "Classifier should have 1 body member");
            match &c.body[0] {
                ClassifierMember::Specialization(s) => {
                    assert_eq!(s.general, "Vehicle");
                }
                _ => panic!("Expected Specialization"),
            }
        }
        _ => panic!("Expected Classifier"),
    }
}

#[test]
fn test_parse_classifier_with_multiple_specializations_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "classifier SportsCar specializes Car, Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("SportsCar".to_string()));
            assert_eq!(c.body.len(), 2, "Should have 2 specializations");

            let generals: Vec<String> = c
                .body
                .iter()
                .filter_map(|m| match m {
                    ClassifierMember::Specialization(s) => Some(s.general.clone()),
                    _ => None,
                })
                .collect();

            assert!(generals.contains(&"Car".to_string()));
            assert!(generals.contains(&"Vehicle".to_string()));
        }
        _ => panic!("Expected Classifier"),
    }
}

#[test]
fn test_parse_feature_with_typing_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "feature mass : Real;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("mass".to_string()));
            assert_eq!(f.body.len(), 1, "Feature should have 1 body member");
            match &f.body[0] {
                FeatureMember::Typing(t) => {
                    assert_eq!(t.typed, "Real");
                }
                _ => panic!("Expected Typing"),
            }
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_feature_with_redefinition_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "feature currentMass redefines mass;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("currentMass".to_string()));
            assert_eq!(f.body.len(), 1, "Feature should have 1 body member");
            match &f.body[0] {
                FeatureMember::Redefinition(r) => {
                    assert_eq!(r.redefined, "mass");
                }
                _ => panic!("Expected Redefinition"),
            }
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_feature_with_subsetting_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "feature wheelMass subsets mass;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("wheelMass".to_string()));
            assert_eq!(f.body.len(), 1, "Feature should have 1 body member");
            match &f.body[0] {
                FeatureMember::Subsetting(s) => {
                    assert_eq!(s.subset, "mass");
                }
                _ => panic!("Expected Subsetting"),
            }
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_feature_with_typing_and_redefinition_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "feature currentMass : Real redefines mass;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Feature(f) => {
            assert_eq!(f.name, Some("currentMass".to_string()));
            assert_eq!(f.body.len(), 2, "Feature should have 2 body members");

            let has_typing = f
                .body
                .iter()
                .any(|m| matches!(m, FeatureMember::Typing(t) if t.typed == "Real"));
            let has_redef = f
                .body
                .iter()
                .any(|m| matches!(m, FeatureMember::Redefinition(r) if r.redefined == "mass"));

            assert!(has_typing, "Should have typing relationship");
            assert!(has_redef, "Should have redefinition relationship");
        }
        _ => panic!("Expected Feature"),
    }
}

#[test]
fn test_parse_abstract_classifier_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "abstract classifier Vehicle;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Vehicle".to_string()));
            assert!(c.is_abstract, "Classifier should be abstract");
        }
        _ => panic!("Expected Classifier"),
    }
}

#[test]
fn test_parse_const_feature_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = r#"
        package Test {
            const feature id : String;
        }
    "#;
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    // Extract the package and feature directly with assertions
    assert_eq!(file.elements.len(), 1, "Should have exactly one package");
    let AstElement::Package(pkg) = &file.elements[0] else {
        panic!("Expected Package, got {:?}", file.elements[0]);
    };

    assert_eq!(
        pkg.elements.len(),
        1,
        "Package should have exactly one feature"
    );
    let AstElement::Feature(f) = &pkg.elements[0] else {
        panic!("Expected Feature, got {:?}", pkg.elements[0]);
    };

    assert_eq!(f.name, Some("id".to_string()));
    assert!(f.is_const, "Feature should be const");
}

#[test]
fn test_parse_datatype_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "datatype Real;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Real".to_string()));
            assert_eq!(c.kind, ClassifierKind::DataType);
        }
        _ => panic!("Expected Classifier (DataType)"),
    }
}

#[test]
fn test_parse_function_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = "function calculateArea;";
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("calculateArea".to_string()));
            assert_eq!(c.kind, ClassifierKind::Function);
        }
        _ => panic!("Expected Classifier (Function)"),
    }
}

#[test]
fn test_parse_classifier_with_nested_feature_ast() {
    use syster::syntax::kerml::ast::parse_file;

    let input = r#"classifier Vehicle {
        feature mass : Real;
    }"#;
    let mut pairs = KerMLParser::parse(Rule::file, input).unwrap();
    let file = parse_file(&mut pairs).unwrap();

    assert_eq!(file.elements.len(), 1);
    match &file.elements[0] {
        AstElement::Classifier(c) => {
            assert_eq!(c.name, Some("Vehicle".to_string()));
            assert_eq!(c.body.len(), 1, "Classifier should have 1 nested feature");
            match &c.body[0] {
                ClassifierMember::Feature(f) => {
                    assert_eq!(f.name, Some("mass".to_string()));
                    assert_eq!(f.body.len(), 1, "Feature should have typing");
                    match &f.body[0] {
                        FeatureMember::Typing(t) => {
                            assert_eq!(t.typed, "Real");
                        }
                        _ => panic!("Expected Typing"),
                    }
                }
                _ => panic!("Expected Feature"),
            }
        }
        _ => panic!("Expected Classifier"),
    }
}

// =============================================================================
// Issue #617: feature_direction_kind should not match prefix of other keywords
// =============================================================================

/// Tests that feature_direction_kind matches standalone direction keywords
#[rstest]
#[case("in")]
#[case("out")]
#[case("inout")]
fn test_feature_direction_kind_standalone(#[case] input: &str) {
    let result = KerMLParser::parse(Rule::feature_direction_kind, input);
    assert!(
        result.is_ok(),
        "Should parse '{input}' as feature_direction_kind"
    );
    assert_eq!(result.unwrap().as_str(), input);
}

/// Tests that feature_direction_kind does NOT match when followed by alphanumeric chars
/// This prevents "in" from matching the start of "interaction", "interface", etc.
#[rstest]
#[case("interaction")]
#[case("interface")]
#[case("input")]
#[case("internal")]
#[case("inout_extended")]
#[case("output")]
#[case("outer")]
fn test_feature_direction_kind_rejects_prefixes(#[case] input: &str) {
    let result = KerMLParser::parse(Rule::feature_direction_kind, input);
    // The parse should either fail or not consume the entire input
    if let Ok(pairs) = result {
        let parsed = pairs.as_str();
        assert_ne!(
            parsed, input,
            "'{input}' should not fully match as feature_direction_kind"
        );
    }
}

// =============================================================================
// Consolidated Round-Trip Tests
// Generated by scripts/consolidate_parser_tests.py
// =============================================================================

#[rstest]
#[case("regularName", "regularName", Rule::regular_name, "regular_name")]
#[case(
    "'unrestricted name'",
    "'unrestricted name'",
    Rule::regular_name,
    "regular_name"
)]
#[case("42", "42", Rule::literal_number, "literal_number")]
#[case("3.14", "3.14", Rule::literal_number, "literal_number")]
#[case("1.5e10", "1.5e10", Rule::literal_number, "literal_number")]
#[case("true", "true", Rule::literal_expression, "literal_expression")]
#[case("42", "42", Rule::literal_expression, "literal_expression")]
#[case("*", "*", Rule::literal_expression, "literal_expression")]
#[case("public", "public", Rule::visibility_kind, "visibility_kind")]
#[case("private", "private", Rule::visibility_kind, "visibility_kind")]
#[case("protected", "protected", Rule::visibility_kind, "visibility_kind")]
#[case(
    "hastype",
    "hastype",
    Rule::classification_test_operator,
    "classification_test_operator"
)]
#[case(
    "istype",
    "istype",
    Rule::classification_test_operator,
    "classification_test_operator"
)]
#[case(
    "@",
    "@",
    Rule::classification_test_operator,
    "classification_test_operator"
)]
#[case(
    "@@",
    "@@",
    Rule::classification_test_operator,
    "classification_test_operator"
)]
#[case("<", "<", Rule::relational_operator, "relational_operator")]
#[case(">", ">", Rule::relational_operator, "relational_operator")]
#[case("<=", "<=", Rule::relational_operator, "relational_operator")]
#[case(">=", ">=", Rule::relational_operator, "relational_operator")]
#[case("::*", "::*", Rule::import_kind, "import_kind")]
#[case("::**", "::**", Rule::import_kind, "import_kind")]
#[case("::*::**", "::*::**", Rule::import_kind, "import_kind")]
#[case("public", "public", Rule::visibility, "visibility")]
#[case("private", "private", Rule::visibility, "visibility")]
#[case("protected", "protected", Rule::visibility, "visibility")]
#[case(
    "Foo",
    "Foo",
    Rule::qualified_reference_chain,
    "qualified_reference_chain"
)]
#[case(
    "Foo::Bar",
    "Foo::Bar",
    Rule::qualified_reference_chain,
    "qualified_reference_chain"
)]
#[case(
    "Foo::Bar::Baz",
    "Foo::Bar::Baz",
    Rule::qualified_reference_chain,
    "qualified_reference_chain"
)]
#[case("true", "true", Rule::inline_expression, "inline_expression")]
#[case("42", "42", Rule::inline_expression, "inline_expression")]
#[case("null", "null", Rule::inline_expression, "inline_expression")]
#[case(
    "myFeature",
    "myFeature",
    Rule::feature_chain_expression,
    "feature_chain_expression"
)]
#[case(
    "a.b",
    "a.b",
    Rule::feature_chain_expression,
    "feature_chain_expression"
)]
#[case(
    "a.b.c",
    "a.b.c",
    Rule::feature_chain_expression,
    "feature_chain_expression"
)]
#[case("myArray", "myArray", Rule::index_expression, "index_expression")]
#[case("arr[0]", "arr[0]", Rule::index_expression, "index_expression")]
#[case(
    "matrix[1][2]",
    "matrix[1][2]",
    Rule::index_expression,
    "index_expression"
)]
#[case(";", ";", Rule::relationship_body, "relationship_body")]
#[case("{}", "{}", Rule::relationship_body, "relationship_body")]
#[case("import", "import", Rule::import_prefix, "import_prefix")]
#[case("public import", "public import", Rule::import_prefix, "import_prefix")]
#[case(
    "private import",
    "private import",
    Rule::import_prefix,
    "import_prefix"
)]
#[case(
    "protected import",
    "protected import",
    Rule::import_prefix,
    "import_prefix"
)]
#[case("import all", "import all", Rule::import_prefix, "import_prefix")]
#[case(
    "private import all",
    "private import all",
    Rule::import_prefix,
    "import_prefix"
)]
#[case("MyImport", "MyImport", Rule::imported_reference, "imported_reference")]
#[case(
    "MyImport::*",
    "MyImport::*",
    Rule::imported_reference,
    "imported_reference"
)]
#[case(
    "MyImport::**",
    "MyImport::**",
    Rule::imported_reference,
    "imported_reference"
)]
#[case(
    "MyImport::*::**",
    "MyImport::*::**",
    Rule::imported_reference,
    "imported_reference"
)]
#[case("BaseType", "BaseType", Rule::relationship, "relationship")]
#[case(
    "public BaseType",
    "public BaseType",
    Rule::relationship,
    "relationship"
)]
#[case(
    "MyType::NestedType",
    "MyType::NestedType",
    Rule::relationship,
    "relationship"
)]
#[case("BaseType", "BaseType", Rule::inheritance, "inheritance")]
#[case(
    "private BaseClass",
    "private BaseClass",
    Rule::inheritance,
    "inheritance"
)]
#[case(":> BaseType", ":> BaseType", Rule::specialization, "specialization")]
#[case(
    "specializes BaseClass",
    "specializes BaseClass",
    Rule::specialization,
    "specialization"
)]
#[case(
    ":> public MyBase",
    ":> public MyBase",
    Rule::specialization,
    "specialization"
)]
#[case(":> BaseType", ":> BaseType", Rule::subsetting, "subsetting")]
#[case(
    "subsets BaseClass",
    "subsets BaseClass",
    Rule::subsetting,
    "subsetting"
)]
#[case(":> Base::MyType", ":> Base::MyType", Rule::subsetting, "subsetting")]
#[case(":> Clock, Life", ":> Clock, Life", Rule::subsetting, "subsetting")]
#[case(
    ":> Type1, Type2, Type3",
    ":> Type1, Type2, Type3",
    Rule::subsetting,
    "subsetting"
)]
#[case(":>> BaseType", ":>> BaseType", Rule::redefinition, "redefinition")]
#[case(
    "redefines OldFeature",
    "redefines OldFeature",
    Rule::redefinition,
    "redefinition"
)]
#[case(":>> Base::Type", ":>> Base::Type", Rule::redefinition, "redefinition")]
#[case(
    ":>> Collection::elements",
    ":>> Collection::elements",
    Rule::redefinition,
    "redefinition"
)]
#[case(
    ":>> Feature1, Feature2",
    ":>> Feature1, Feature2",
    Rule::redefinition,
    "redefinition"
)]
#[case(
    "::> RefType",
    "::> RefType",
    Rule::reference_subsetting,
    "reference_subsetting"
)]
#[case(
    "references RefFeature",
    "references RefFeature",
    Rule::reference_subsetting,
    "reference_subsetting"
)]
#[case(
    "::> Ref::Feature",
    "::> Ref::Feature",
    Rule::reference_subsetting,
    "reference_subsetting"
)]
#[case(
    "=> CrossedType",
    "=> CrossedType",
    Rule::cross_subsetting,
    "cross_subsetting"
)]
#[case(
    "crosses CrossedFeature",
    "crosses CrossedFeature",
    Rule::cross_subsetting,
    "cross_subsetting"
)]
#[case(
    "=> Cross::Type",
    "=> Cross::Type",
    Rule::cross_subsetting,
    "cross_subsetting"
)]
#[case(
    "conjugates BaseType",
    "conjugates BaseType",
    Rule::conjugation,
    "conjugation"
)]
#[case(
    "conjugates public ConjugateType",
    "conjugates public ConjugateType",
    Rule::conjugation,
    "conjugation"
)]
#[case("unions Type1", "unions Type1", Rule::unioning, "unioning")]
#[case(
    "unions public Type2",
    "unions public Type2",
    Rule::unioning,
    "unioning"
)]
#[case(
    "differences Type1",
    "differences Type1",
    Rule::differencing,
    "differencing"
)]
#[case(
    "differences private Type2",
    "differences private Type2",
    Rule::differencing,
    "differencing"
)]
#[case(
    "intersects Type1",
    "intersects Type1",
    Rule::intersecting,
    "intersecting"
)]
#[case(
    "intersects public Type2",
    "intersects public Type2",
    Rule::intersecting,
    "intersecting"
)]
#[case(
    "intersects VectorValue, Array",
    "intersects VectorValue, Array",
    Rule::intersecting,
    "intersecting"
)]
#[case(
    "chains feature1",
    "chains feature1",
    Rule::feature_chaining,
    "feature_chaining"
)]
#[case(
    "chains public feature2",
    "chains public feature2",
    Rule::feature_chaining,
    "feature_chaining"
)]
#[case(
    "chains source.target",
    "chains source.target",
    Rule::feature_chaining,
    "feature_chaining"
)]
#[case(
    "chains a.b.c",
    "chains a.b.c",
    Rule::feature_chaining,
    "feature_chaining"
)]
#[case(
    "chains parent.child",
    "chains parent.child",
    Rule::feature_chaining,
    "feature_chaining"
)]
#[case("disjoint Type1", "disjoint Type1", Rule::disjoining, "disjoining")]
#[case(
    "disjoint private Type2",
    "disjoint private Type2",
    Rule::disjoining,
    "disjoining"
)]
#[case(
    "inverse feature1 of feature2;",
    "inverse feature1 of feature2;",
    Rule::feature_inverting,
    "feature_inverting"
)]
#[case(
    "inverse feature2 of other;",
    "inverse feature2 of other;",
    Rule::feature_inverting,
    "feature_inverting"
)]
#[case("featured by Type1", "featured by Type1", Rule::featuring, "featuring")]
#[case("featured by Type2", "featured by Type2", Rule::featuring, "featuring")]
#[case(
    "featuring feature1 by Type1 ;",
    "featuring feature1 by Type1 ;",
    Rule::type_featuring,
    "type_featuring"
)]
#[case(
    "featuring of f by Type1 ;",
    "featuring of f by Type1 ;",
    Rule::type_featuring,
    "type_featuring"
)]
#[case(": BaseType", ": BaseType", Rule::feature_typing, "feature_typing")]
#[case(
    "typed by TypeSpec",
    "typed by TypeSpec",
    Rule::feature_typing,
    "feature_typing"
)]
#[case(": Complex", ": Complex", Rule::feature_typing, "feature_typing")]
#[case(
    ": Boolean, String",
    ": Boolean, String",
    Rule::feature_typing,
    "feature_typing"
)]
#[case(": Anything", ": Anything", Rule::feature_typing, "feature_typing")]
#[case(
    ": String, Integer",
    ": String, Integer",
    Rule::feature_typing,
    "feature_typing"
)]
#[case(
    "subclassifier SubClass :> BaseClass;",
    "subclassifier SubClass :> BaseClass;",
    Rule::subclassification,
    "subclassification"
)]
#[case(
    "subclassifier MyClass specializes ClassSpec;",
    "subclassifier MyClass specializes ClassSpec;",
    Rule::subclassification,
    "subclassification"
)]
#[case("MyRef", "MyRef", Rule::membership, "membership")]
#[case("public MyRef", "public MyRef", Rule::membership, "membership")]
#[case("alias MyRef", "alias MyRef", Rule::membership, "membership")]
#[case("private alias", "private alias", Rule::membership, "membership")]
#[case("MyRef", "MyRef", Rule::owning_membership, "owning_membership")]
#[case(
    "public alias MyRef",
    "public alias MyRef",
    Rule::owning_membership,
    "owning_membership"
)]
#[case("= MyRef", "= MyRef", Rule::feature_value, "feature_value")]
#[case(
    ":= public MyRef",
    ":= public MyRef",
    Rule::feature_value,
    "feature_value"
)]
#[case(
    "= alias Target",
    "= alias Target",
    Rule::feature_value,
    "feature_value"
)]
#[case(
    "filter MyRef;",
    "filter MyRef;",
    Rule::element_filter_membership,
    "element_filter_membership"
)]
#[case(
    "filter OtherRef;",
    "filter OtherRef;",
    Rule::element_filter_membership,
    "element_filter_membership"
)]
#[case(
    "featured by MyType alias MyRef",
    "featured by MyType alias MyRef",
    Rule::feature_membership,
    "feature_membership"
)]
#[case(
    "featured by BaseType public alias Target",
    "featured by BaseType public alias Target",
    Rule::feature_membership,
    "feature_membership"
)]
#[case(
    "end x : MyType;",
    "end x : MyType;",
    Rule::end_feature_membership,
    "end_feature_membership"
)]
#[case(
    "end y : BaseType[1];",
    "end y : BaseType[1];",
    Rule::end_feature_membership,
    "end_feature_membership"
)]
#[case(
    "return featured by MyType alias MyRef",
    "return featured by MyType alias MyRef",
    Rule::result_expression_membership,
    "result_expression_membership"
)]
#[case(
    "return featured by BaseType public alias Target",
    "return featured by BaseType public alias Target",
    Rule::result_expression_membership,
    "result_expression_membership"
)]
#[case("import MyPackage;", "import MyPackage;", Rule::import, "import")]
#[case("public import MyLib;", "public import MyLib;", Rule::import, "import")]
#[case(
    "import all MyNamespace;",
    "import all MyNamespace;",
    Rule::import,
    "import"
)]
#[case(
    "private import all Base;",
    "private import all Base;",
    Rule::import,
    "import"
)]
#[case("import MyPackage::*;", "import MyPackage::*;", Rule::import, "import")]
#[case(
    "import MyPackage::**;",
    "import MyPackage::**;",
    Rule::import,
    "import"
)]
#[case("import MyPackage {}", "import MyPackage {}", Rule::import, "import")]
#[case(
    "dependency Source to Target;",
    "dependency Source to Target;",
    Rule::dependency,
    "dependency"
)]
#[case(
    "dependency MyDep from Source to Target;",
    "dependency MyDep from Source to Target;",
    Rule::dependency,
    "dependency"
)]
#[case(
    "dependency Source, Other to Target, Dest;",
    "dependency Source, Other to Target, Dest;",
    Rule::dependency,
    "dependency"
)]
#[case(
    "dependency <short> named from Source to Target {}",
    "dependency <short> named from Source to Target {}",
    Rule::dependency,
    "dependency"
)]
#[case(
    "namespace MyNamespace;",
    "namespace MyNamespace;",
    Rule::namespace,
    "namespace"
)]
#[case(
    "namespace MyNamespace {}",
    "namespace MyNamespace {}",
    Rule::namespace,
    "namespace"
)]
#[case(
    "namespace <short> named {}",
    "namespace <short> named {}",
    Rule::namespace,
    "namespace"
)]
#[case("package MyPackage;", "package MyPackage;", Rule::package, "package")]
#[case(
    "package MyPackage {}",
    "package MyPackage {}",
    Rule::package,
    "package"
)]
#[case(
    "package <short> named {}",
    "package <short> named {}",
    Rule::package,
    "package"
)]
#[case(
    "library package LibPkg;",
    "library package LibPkg;",
    Rule::library_package,
    "library_package"
)]
#[case(
    "standard library package StdLib;",
    "standard library package StdLib;",
    Rule::library_package,
    "library_package"
)]
#[case(
    "library package MyLib {}",
    "library package MyLib {}",
    Rule::library_package,
    "library_package"
)]
#[case("class MyClass;", "class MyClass;", Rule::class, "class")]
#[case("class MyClass {}", "class MyClass {}", Rule::class, "class")]
#[case(
    "abstract class MyClass;",
    "abstract class MyClass;",
    Rule::class,
    "class"
)]
#[case(
    "class MyClass specializes Base {}",
    "class MyClass specializes Base {}",
    Rule::class,
    "class"
)]
#[case(
    "abstract class MyClass specializes Base, Other {}",
    "abstract class MyClass specializes Base, Other {}",
    Rule::class,
    "class"
)]
#[case("datatype MyData;", "datatype MyData;", Rule::data_type, "data_type")]
#[case(
    "datatype MyData {}",
    "datatype MyData {}",
    Rule::data_type,
    "data_type"
)]
#[case(
    "abstract datatype ScalarValue specializes DataValue;",
    "abstract datatype ScalarValue specializes DataValue;",
    Rule::data_type,
    "data_type"
)]
#[case(
    "datatype Boolean specializes ScalarValue;",
    "datatype Boolean specializes ScalarValue;",
    Rule::data_type,
    "data_type"
)]
#[case(
    "datatype String specializes ScalarValue;",
    "datatype String specializes ScalarValue;",
    Rule::data_type,
    "data_type"
)]
#[case("struct MyStruct;", "struct MyStruct;", Rule::structure, "structure")]
#[case(
    "struct MyStruct {}",
    "struct MyStruct {}",
    Rule::structure,
    "structure"
)]
#[case(
    "struct MyStruct[1] :> Parent {}",
    "struct MyStruct[1] :> Parent {}",
    Rule::structure,
    "structure"
)]
#[case(
    "private struct MyStruct[0..1] specializes Base {}",
    "private struct MyStruct[0..1] specializes Base {}",
    Rule::structure,
    "structure"
)]
#[case(
    "abstract struct MyStruct specializes Base, Other {}",
    "abstract struct MyStruct specializes Base, Other {}",
    Rule::structure,
    "structure"
)]
#[case("assoc MyAssoc;", "assoc MyAssoc;", Rule::association, "association")]
#[case(
    "assoc MyAssoc {}",
    "assoc MyAssoc {}",
    Rule::association,
    "association"
)]
#[case(
    "abstract assoc Link specializes Anything {}",
    "abstract assoc Link specializes Anything {}",
    Rule::association,
    "association"
)]
#[case(
    "assoc MyAssoc specializes Base {}",
    "assoc MyAssoc specializes Base {}",
    Rule::association,
    "association"
)]
#[case(
    "assoc struct MyAssocStruct;",
    "assoc struct MyAssocStruct;",
    Rule::association_structure,
    "association_structure"
)]
#[case(
    "assoc struct MyAssocStruct {}",
    "assoc struct MyAssocStruct {}",
    Rule::association_structure,
    "association_structure"
)]
#[case(
    "behavior MyBehavior;",
    "behavior MyBehavior;",
    Rule::behavior,
    "behavior"
)]
#[case(
    "behavior MyBehavior {}",
    "behavior MyBehavior {}",
    Rule::behavior,
    "behavior"
)]
#[case(
    "abstract behavior DecisionPerformance specializes Performance {}",
    "abstract behavior DecisionPerformance specializes Performance {}",
    Rule::behavior,
    "behavior"
)]
#[case(
    "behavior MyBehavior specializes Base, Other {}",
    "behavior MyBehavior specializes Base, Other {}",
    Rule::behavior,
    "behavior"
)]
#[case(
    "function MyFunction;",
    "function MyFunction;",
    Rule::function,
    "function"
)]
#[case(
    "function MyFunction {}",
    "function MyFunction {}",
    Rule::function,
    "function"
)]
#[case(
    "predicate MyPredicate;",
    "predicate MyPredicate;",
    Rule::predicate,
    "predicate"
)]
#[case(
    "predicate MyPredicate {}",
    "predicate MyPredicate {}",
    Rule::predicate,
    "predicate"
)]
#[case(
    "interaction MyInteraction;",
    "interaction MyInteraction;",
    Rule::interaction,
    "interaction"
)]
#[case(
    "interaction MyInteraction {}",
    "interaction MyInteraction {}",
    Rule::interaction,
    "interaction"
)]
#[case(
    "metaclass MyMetaclass;",
    "metaclass MyMetaclass;",
    Rule::metaclass,
    "metaclass"
)]
#[case(
    "metaclass MyMetaclass {}",
    "metaclass MyMetaclass {}",
    Rule::metaclass,
    "metaclass"
)]
#[case(
    "connector MyConnector;",
    "connector MyConnector;",
    Rule::connector,
    "connector"
)]
#[case(
    "connector MyConnector {}",
    "connector MyConnector {}",
    Rule::connector,
    "connector"
)]
#[case(
    "binding MyBinding;",
    "binding MyBinding;",
    Rule::binding_connector,
    "binding_connector"
)]
#[case(
    "binding MyBinding {}",
    "binding MyBinding {}",
    Rule::binding_connector,
    "binding_connector"
)]
#[case(
    "succession MySuccession;",
    "succession MySuccession;",
    Rule::succession,
    "succession"
)]
#[case(
    "succession MySuccession {}",
    "succession MySuccession {}",
    Rule::succession,
    "succession"
)]
#[case("step MyStep;", "step MyStep;", Rule::step, "step")]
#[case("step MyStep {}", "step MyStep {}", Rule::step, "step")]
#[case("expr MyExpr;", "expr MyExpr;", Rule::expression, "expression")]
#[case("expr MyExpr {}", "expr MyExpr {}", Rule::expression, "expression")]
#[case("inv MyInvariant;", "inv MyInvariant;", Rule::invariant, "invariant")]
#[case(
    "inv not MyInvariant {}",
    "inv not MyInvariant {}",
    Rule::invariant,
    "invariant"
)]
#[case(
    "feature MyFeature;",
    "feature MyFeature;",
    Rule::feature,
    "feature_basic"
)]
#[case(
    "feature MyFeature {}",
    "feature MyFeature {}",
    Rule::feature,
    "feature_basic"
)]
#[case(
    "in feature MyFeature;",
    "in feature MyFeature;",
    Rule::feature,
    "feature_with_direction"
)]
#[case(
    "out feature MyFeature;",
    "out feature MyFeature;",
    Rule::feature,
    "feature_with_direction"
)]
#[case(
    "inout feature MyFeature;",
    "inout feature MyFeature;",
    Rule::feature,
    "feature_with_direction"
)]
#[case(
    "abstract feature MyFeature;",
    "abstract feature MyFeature;",
    Rule::feature,
    "feature_with_composition"
)]
#[case(
    "composite feature MyFeature;",
    "composite feature MyFeature;",
    Rule::feature,
    "feature_with_composition"
)]
#[case(
    "portion feature MyFeature;",
    "portion feature MyFeature;",
    Rule::feature,
    "feature_with_composition"
)]
#[case(
    "const feature MyFeature;",
    "const feature MyFeature;",
    Rule::feature,
    "feature_with_property"
)]
#[case(
    "derived feature MyFeature;",
    "derived feature MyFeature;",
    Rule::feature,
    "feature_with_property"
)]
#[case(
    "end feature MyFeature;",
    "end feature MyFeature;",
    Rule::feature,
    "feature_with_property"
)]
#[case(
    "feature MyFeature ordered;",
    "feature MyFeature ordered;",
    Rule::feature,
    "feature_with_multiplicity_properties"
)]
#[case(
    "feature MyFeature nonunique;",
    "feature MyFeature nonunique;",
    Rule::feature,
    "feature_with_multiplicity_properties"
)]
#[case(
    "feature MyFeature ordered nonunique;",
    "feature MyFeature ordered nonunique;",
    Rule::feature,
    "feature_with_multiplicity_properties"
)]
#[case(
    "in abstract const feature MyFeature ordered;",
    "in abstract const feature MyFeature ordered;",
    Rule::feature,
    "feature_combined_modifiers"
)]
#[case(
    "out composite derived feature MyFeature nonunique;",
    "out composite derived feature MyFeature nonunique;",
    Rule::feature,
    "feature_combined_modifiers"
)]
#[case(
    "inout portion end feature MyFeature ordered nonunique;",
    "inout portion end feature MyFeature ordered nonunique;",
    Rule::feature,
    "feature_combined_modifiers"
)]
#[case(
    "feature elements[0..*] :>> Collection::elements {}",
    "feature elements[0..*] :>> Collection::elements {}",
    Rule::feature,
    "feature_with_multiplicity_and_relationships"
)]
#[case(
    "feature myFeature[1] :> BaseFeature;",
    "feature myFeature[1] :> BaseFeature;",
    Rule::feature,
    "feature_with_multiplicity_and_relationships"
)]
#[case(
    "feature items[*] : ItemType ordered;",
    "feature items[*] : ItemType ordered;",
    Rule::feature,
    "feature_with_multiplicity_and_relationships"
)]
#[case(
    "comment /* simple comment */",
    "comment /* simple comment */",
    Rule::comment_annotation,
    "comment_basic"
)]
#[case(
    "comment myComment /* comment text */",
    "comment myComment /* comment text */",
    Rule::comment_annotation,
    "comment_basic"
)]
#[case(
    "comment about Foo /* about Foo */",
    "comment about Foo /* about Foo */",
    Rule::comment_annotation,
    "comment_with_about"
)]
#[case(
    "comment about Bar, Baz /* about multiple */",
    "comment about Bar, Baz /* about multiple */",
    Rule::comment_annotation,
    "comment_with_about"
)]
#[case(
    "doc /* documentation */",
    "doc /* documentation */",
    Rule::documentation,
    "documentation_basic"
)]
#[case(
    "doc MyDoc /* doc text */",
    "doc MyDoc /* doc text */",
    Rule::documentation,
    "documentation_basic"
)]
#[case("feature;", "feature;", Rule::multiplicity, "multiplicity")]
#[case(
    "feature myMultiplicity;",
    "feature myMultiplicity;",
    Rule::multiplicity,
    "multiplicity"
)]
#[case(
    "feature myMultiplicity : MyType;",
    "feature myMultiplicity : MyType;",
    Rule::multiplicity,
    "multiplicity"
)]
#[case("feature;", "feature;", Rule::multiplicity_range, "multiplicity_range")]
#[case(
    "feature myRange;",
    "feature myRange;",
    Rule::multiplicity_range,
    "multiplicity_range"
)]
#[case(
    "feature myRange { feature bound; }",
    "feature myRange { feature bound; }",
    Rule::multiplicity_range,
    "multiplicity_range"
)]
#[case(
    "metadata MyType;",
    "metadata MyType;",
    Rule::metadata_feature,
    "metadata_feature"
)]
#[case(
    "metadata myMeta : MyType;",
    "metadata myMeta : MyType;",
    Rule::metadata_feature,
    "metadata_feature"
)]
#[case(
    "metadata MyType about Foo;",
    "metadata MyType about Foo;",
    Rule::metadata_feature,
    "metadata_feature"
)]
#[case(
    "metadata myMeta : MyType about Foo, Bar;",
    "metadata myMeta : MyType about Foo, Bar;",
    Rule::metadata_feature,
    "metadata_feature"
)]
#[case("feature;", "feature;", Rule::item_feature, "item_feature")]
#[case(
    "feature myItem;",
    "feature myItem;",
    Rule::item_feature,
    "item_feature"
)]
#[case(
    "feature myItem : ItemType;",
    "feature myItem : ItemType;",
    Rule::item_feature,
    "item_feature"
)]
#[case("flow myFlow;", "flow myFlow;", Rule::item_flow, "item_flow")]
#[case(
    "flow myFlow from a to b;",
    "flow myFlow from a to b;",
    Rule::item_flow,
    "item_flow"
)]
#[case(
    "succession flow;",
    "succession flow;",
    Rule::succession_item_flow,
    "succession_item_flow"
)]
#[case(
    "succession flow myFlow;",
    "succession flow myFlow;",
    Rule::succession_item_flow,
    "succession_item_flow"
)]
#[case("expr;", "expr;", Rule::boolean_expression, "boolean_expression")]
#[case(
    "expr myBool;",
    "expr myBool;",
    Rule::boolean_expression,
    "boolean_expression"
)]
#[case("3.14", "3.14", Rule::float, "float")]
#[case(".5", ".5", Rule::float, "float")]
#[case("0.0", "0.0", Rule::float, "float")]
#[case(".5", ".5", Rule::fraction, "fraction")]
#[case(".123", ".123", Rule::fraction, "fraction")]
#[case(".0", ".0", Rule::fraction, "fraction")]
#[case("e10", "e10", Rule::exponent, "exponent")]
#[case("E-5", "E-5", Rule::exponent, "exponent")]
#[case("e+3", "e+3", Rule::exponent, "exponent")]
#[case("myElement", "myElement", Rule::element_reference, "element_reference")]
#[case(
    "Base::Derived",
    "Base::Derived",
    Rule::element_reference,
    "element_reference"
)]
#[case(
    "Pkg::Sub::Element",
    "Pkg::Sub::Element",
    Rule::element_reference,
    "element_reference"
)]
#[case("MyType", "MyType", Rule::type_reference, "type_reference")]
#[case("Base::MyType", "Base::MyType", Rule::type_reference, "type_reference")]
#[case("myFeature", "myFeature", Rule::feature_reference, "feature_reference")]
#[case(
    "Base::myFeature",
    "Base::myFeature",
    Rule::feature_reference,
    "feature_reference"
)]
#[case(
    "MyClassifier",
    "MyClassifier",
    Rule::classifier_reference,
    "classifier_reference"
)]
#[case(
    "Base::MyClassifier",
    "Base::MyClassifier",
    Rule::classifier_reference,
    "classifier_reference"
)]
#[case("<shortName>", "<shortName>", Rule::element, "element")]
#[case("regularName", "regularName", Rule::element, "element")]
#[case(
    "<shortName> regularName",
    "<shortName> regularName",
    Rule::element,
    "element"
)]
#[case("MyElement", "MyElement", Rule::annotation, "annotation")]
#[case(
    "comment /* text */",
    "comment /* text */",
    Rule::owned_annotation,
    "owned_annotation"
)]
#[case(
    "doc /* documentation */",
    "doc /* documentation */",
    Rule::owned_annotation,
    "owned_annotation"
)]
#[case("type MyType;", "type MyType;", Rule::type_def, "type_def")]
#[case(
    "abstract type MyType {}",
    "abstract type MyType {}",
    Rule::type_def,
    "type_def"
)]
#[case("type all MyType {}", "type all MyType {}", Rule::type_def, "type_def")]
#[case(
    "type MyType ordered {}",
    "type MyType ordered {}",
    Rule::type_def,
    "type_def"
)]
#[case(
    "type MyType unions BaseType {}",
    "type MyType unions BaseType {}",
    Rule::type_def,
    "type_def"
)]
#[case(
    "type MyType differences BaseType {}",
    "type MyType differences BaseType {}",
    Rule::type_def,
    "type_def"
)]
#[case(
    "classifier MyClassifier;",
    "classifier MyClassifier;",
    Rule::classifier,
    "classifier"
)]
#[case(
    "abstract classifier MyClassifier {}",
    "abstract classifier MyClassifier {}",
    Rule::classifier,
    "classifier"
)]
#[case(
    "classifier all MyClassifier {}",
    "classifier all MyClassifier {}",
    Rule::classifier,
    "classifier"
)]
#[case(
    "classifier MyClassifier unions BaseClassifier {}",
    "classifier MyClassifier unions BaseClassifier {}",
    Rule::classifier,
    "classifier"
)]
#[case("null", "null", Rule::operator_expression, "operator_expression")]
#[case("true", "true", Rule::operator_expression, "operator_expression")]
#[case(
    "myFeature",
    "myFeature",
    Rule::operator_expression,
    "operator_expression"
)]
#[case(
    "obj.metadata",
    "obj.metadata",
    Rule::metadata_access_expression,
    "metadata_access_expression"
)]
#[case(
    "Base::Feature.metadata",
    "Base::Feature.metadata",
    Rule::metadata_access_expression,
    "metadata_access_expression"
)]
#[case(
    "feature dimensions: Positive[0..*] ordered nonunique { }",
    "feature dimensions: Positive[0..*] ordered nonunique { }",
    Rule::feature,
    "feature_with_modifiers_after_typing"
)]
#[case(
    "feature x: Type ordered { }",
    "feature x: Type ordered { }",
    Rule::feature,
    "feature_with_modifiers_after_typing"
)]
#[case(
    "feature y: T nonunique { }",
    "feature y: T nonunique { }",
    Rule::feature,
    "feature_with_modifiers_after_typing"
)]
#[case(
    "feature z: T[1] ordered nonunique;",
    "feature z: T[1] ordered nonunique;",
    Rule::feature,
    "feature_with_modifiers_after_typing"
)]
#[case(
    "doc /* This is documentation */",
    "doc /* This is documentation */",
    Rule::documentation,
    "documentation"
)]
#[case(
    "doc /* Multi-line\\n * documentation\\n */",
    "doc /* Multi-line\\n * documentation\\n */",
    Rule::documentation,
    "documentation"
)]
#[case(
    "doc /* Simple */",
    "doc /* Simple */",
    Rule::documentation,
    "documentation"
)]
#[case(
    "in x: Anything[0..1];",
    "in x: Anything[0..1];",
    Rule::parameter_membership,
    "parameter_membership"
)]
#[case(
    "in y: Boolean[1];",
    "in y: Boolean[1];",
    Rule::parameter_membership,
    "parameter_membership"
)]
#[case(
    "out result: Natural[1];",
    "out result: Natural[1];",
    Rule::parameter_membership,
    "parameter_membership"
)]
#[case(
    "inout value: Complex[0..*];",
    "inout value: Complex[0..*];",
    Rule::parameter_membership,
    "parameter_membership"
)]
#[case(
    "in x: Anything[0..*] nonunique;",
    "in x: Anything[0..*] nonunique;",
    Rule::parameter_membership,
    "parameter_membership"
)]
#[case(
    "in x: Anything[0..*] ordered;",
    "in x: Anything[0..*] ordered;",
    Rule::parameter_membership,
    "parameter_membership"
)]
#[case(
    "return : Boolean[1];",
    "return : Boolean[1];",
    Rule::return_parameter_membership,
    "return_parameter_membership"
)]
#[case(
    "return result: Natural[1];",
    "return result: Natural[1];",
    Rule::return_parameter_membership,
    "return_parameter_membership"
)]
#[case(
    "return : Complex[1] = x + y;",
    "return : Complex[1] = x + y;",
    Rule::return_parameter_membership,
    "return_parameter_membership"
)]
#[case(
    "function '==' { }",
    "function '==' { }",
    Rule::function,
    "function_with_operator_name"
)]
#[case(
    "function '!=' { }",
    "function '!=' { }",
    Rule::function,
    "function_with_operator_name"
)]
#[case(
    "function '+' { }",
    "function '+' { }",
    Rule::function,
    "function_with_operator_name"
)]
#[case(
    "abstract function '-' { }",
    "abstract function '-' { }",
    Rule::function,
    "function_with_operator_name"
)]
#[case(
    "function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; return : Boolean[1]; }",
    "function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; return : Boolean[1]; }",
    Rule::function,
    "function_with_parameters"
)]
#[case(
    "function add { in a: Natural[1]; in b: Natural[1]; return : Natural[1]; }",
    "function add { in a: Natural[1]; in b: Natural[1]; return : Natural[1]; }",
    Rule::function,
    "function_with_parameters"
)]
#[case("'=='", "'=='", Rule::unrestricted_name, "quoted_identifier")]
#[case("'!='", "'!='", Rule::unrestricted_name, "quoted_identifier")]
#[case("'+'", "'+'", Rule::unrestricted_name, "quoted_identifier")]
#[case("'-'", "'-'", Rule::unrestricted_name, "quoted_identifier")]
#[case("'*'", "'*'", Rule::unrestricted_name, "quoted_identifier")]
#[case("'/'", "'/'", Rule::unrestricted_name, "quoted_identifier")]
#[case("'<'", "'<'", Rule::unrestricted_name, "quoted_identifier")]
#[case("'>'", "'>'", Rule::unrestricted_name, "quoted_identifier")]
#[case("'<='", "'<='", Rule::unrestricted_name, "quoted_identifier")]
#[case("'>='", "'>='", Rule::unrestricted_name, "quoted_identifier")]
#[case(
    "ScalarFunctions::'not'",
    "ScalarFunctions::'not'",
    Rule::qualified_reference_chain,
    "qualified_reference_with_quotes"
)]
#[case(
    "Base::'=='",
    "Base::'=='",
    Rule::qualified_reference_chain,
    "qualified_reference_with_quotes"
)]
#[case(
    "Math::'+'",
    "Math::'+'",
    Rule::qualified_reference_chain,
    "qualified_reference_with_quotes"
)]
#[case(
    "Ops::'*'::'nested'",
    "Ops::'*'::'nested'",
    Rule::qualified_reference_chain,
    "qualified_reference_with_quotes"
)]
#[case(
    "function 'not' specializes ScalarFunctions::'not' { }",
    "function 'not' specializes ScalarFunctions::'not' { }",
    Rule::function,
    "function_specializes_quoted"
)]
#[case(
    "function 'xor' specializes Base::'xor' { }",
    "function 'xor' specializes Base::'xor' { }",
    Rule::function,
    "function_specializes_quoted"
)]
#[case("x == y", "x == y", Rule::operator_expression, "binary_expression")]
#[case("x != y", "x != y", Rule::operator_expression, "binary_expression")]
#[case("x === y", "x === y", Rule::operator_expression, "binary_expression")]
#[case("x < y", "x < y", Rule::operator_expression, "binary_expression")]
#[case("x <= y", "x <= y", Rule::operator_expression, "binary_expression")]
#[case("x > y", "x > y", Rule::operator_expression, "binary_expression")]
#[case("x >= y", "x >= y", Rule::operator_expression, "binary_expression")]
#[case("x + y", "x + y", Rule::operator_expression, "binary_expression")]
#[case("x - y", "x - y", Rule::operator_expression, "binary_expression")]
#[case("x * y", "x * y", Rule::operator_expression, "binary_expression")]
#[case("x / y", "x / y", Rule::operator_expression, "binary_expression")]
#[case("x and y", "x and y", Rule::operator_expression, "binary_expression")]
#[case("x or y", "x or y", Rule::operator_expression, "binary_expression")]
#[case("x xor y", "x xor y", Rule::operator_expression, "binary_expression")]
#[case(
    "a == b and c == d",
    "a == b and c == d",
    Rule::operator_expression,
    "binary_expression"
)]
#[case(
    "return : Boolean[1] = x == y;",
    "return : Boolean[1] = x == y;",
    Rule::return_parameter_membership,
    "return_with_binary_expression"
)]
#[case(
    "return : Boolean[1] = x != y;",
    "return : Boolean[1] = x != y;",
    Rule::return_parameter_membership,
    "return_with_binary_expression"
)]
#[case(
    "return : Boolean[1] = x < y;",
    "return : Boolean[1] = x < y;",
    Rule::return_parameter_membership,
    "return_with_binary_expression"
)]
#[case("x ?? 0", "x ?? 0", Rule::operator_expression, "null_coalescing")]
#[case(
    "dimensions->reduce '*' ?? 1",
    "dimensions->reduce '*' ?? 1",
    Rule::operator_expression,
    "null_coalescing"
)]
#[case("'*'", "'*'", Rule::literal_expression, "char_literal")]
#[case("'+'", "'+'", Rule::literal_expression, "char_literal")]
#[case("'a'", "'a'", Rule::literal_expression, "char_literal")]
#[case(
    "in x: Integer[1] default 0;",
    "in x: Integer[1] default 0;",
    Rule::parameter_membership,
    "parameter_with_default"
)]
#[case(
    "in endIndex: Positive[1] default startIndex;",
    "in endIndex: Positive[1] default startIndex;",
    Rule::parameter_membership,
    "parameter_with_default"
)]
#[case(
    "in expr thenValue[0..1] { return : Anything[0..*] ordered nonunique; }",
    "in expr thenValue[0..1] { return : Anything[0..*] ordered nonunique; }",
    Rule::parameter_membership,
    "expression_parameters"
)]
#[case(
    "in step myStep { in x: Integer[1]; }",
    "in step myStep { in x: Integer[1]; }",
    Rule::parameter_membership,
    "expression_parameters"
)]
#[case("123", "123", Rule::decimal, "decimal")]
#[case("0", "0", Rule::decimal, "decimal")]
#[case("999999", "999999", Rule::decimal, "decimal")]
#[case("42", "42", Rule::number, "number")]
#[case("3.14", "3.14", Rule::number, "number")]
#[case(".5", ".5", Rule::number, "number")]
#[case("1.5e10", "1.5e10", Rule::number, "number_with_exponent")]
#[case("2.0E-5", "2.0E-5", Rule::number, "number_with_exponent")]
#[case("3e+2", "3e+2", Rule::number, "number_with_exponent")]
#[case("'simple'", "'simple'", Rule::unrestricted_name, "unrestricted_name")]
#[case(
    "'with space'",
    "'with space'",
    Rule::unrestricted_name,
    "unrestricted_name"
)]
#[case(
    "'with\\'quote'",
    "'with\\'quote'",
    Rule::unrestricted_name,
    "unrestricted_name"
)]
#[case("<shortName>", "<shortName>", Rule::short_name, "short_name")]
#[case("<name123>", "<name123>", Rule::short_name, "short_name")]
#[case(
    "<short> regular",
    "<short> regular",
    Rule::identification,
    "identification"
)]
#[case("<short>", "<short>", Rule::identification, "identification")]
#[case("regular", "regular", Rule::identification, "identification")]
#[case(":>", ":>", Rule::specializes_operator, "specializes_operator")]
#[case(
    "specializes",
    "specializes",
    Rule::specializes_operator,
    "specializes_operator"
)]
#[case(":>>", ":>>", Rule::redefines_operator, "redefines_operator")]
#[case(
    "redefines",
    "redefines",
    Rule::redefines_operator,
    "redefines_operator"
)]
#[case(":", ":", Rule::typed_by_operator, "typed_by_operator")]
#[case("typed by", "typed by", Rule::typed_by_operator, "typed_by_operator")]
#[case("~", "~", Rule::conjugates_operator, "conjugates_operator")]
#[case(
    "conjugates",
    "conjugates",
    Rule::conjugates_operator,
    "conjugates_operator"
)]
#[case(
    "ordered",
    "ordered",
    Rule::multiplicity_properties,
    "multiplicity_properties"
)]
#[case(
    "nonunique",
    "nonunique",
    Rule::multiplicity_properties,
    "multiplicity_properties"
)]
#[case(
    "ordered nonunique",
    "ordered nonunique",
    Rule::multiplicity_properties,
    "multiplicity_properties"
)]
#[case(
    "nonunique ordered",
    "nonunique ordered",
    Rule::multiplicity_properties,
    "multiplicity_properties"
)]
#[case("+", "+", Rule::unary_operator, "unary_operator")]
#[case("-", "-", Rule::unary_operator, "unary_operator")]
#[case("~", "~", Rule::unary_operator, "unary_operator")]
#[case("not", "not", Rule::unary_operator, "unary_operator")]
#[case("==", "==", Rule::equality_operator, "equality_operator")]
#[case("!=", "!=", Rule::equality_operator, "equality_operator")]
#[case("===", "===", Rule::equality_operator, "equality_operator")]
#[case("!==", "!==", Rule::equality_operator, "equality_operator")]
#[case(":>", ":>", Rule::subsets_operator, "subsets_operator")]
#[case("subsets", "subsets", Rule::subsets_operator, "subsets_operator")]
#[case("::>", "::>", Rule::references_operator, "references_operator")]
#[case(
    "references",
    "references",
    Rule::references_operator,
    "references_operator"
)]
#[case("=>", "=>", Rule::crosses_operator, "crosses_operator")]
#[case("crosses", "crosses", Rule::crosses_operator, "crosses_operator")]
#[case(
    "feature chain chains source.target;",
    "feature chain chains source.target;",
    Rule::feature,
    "feature_with_chaining"
)]
#[case(
    "private feature chain chains source.target;",
    "private feature chain chains source.target;",
    Rule::feature,
    "feature_with_chaining"
)]
#[case(
    "function '..' { in x: Integer[1]; return : Integer[1]; }",
    "function '..' { in x: Integer[1]; return : Integer[1]; }",
    Rule::function,
    "function_with_range_operator"
)]
#[case(
    "function test { return : Integer[0..*]; }",
    "function test { return : Integer[0..*]; }",
    Rule::function,
    "function_with_range_operator"
)]
fn test_parse_round_trip(
    #[case] input: &str,
    #[case] expected: &str,
    #[case] rule: Rule,
    #[case] desc: &str,
) {
    let result = KerMLParser::parse(rule, input);
    assert!(
        result.is_ok(),
        "Failed to parse '{}' as {:?} ({}): {:?}",
        input,
        rule,
        desc,
        result.err()
    );
    #[allow(clippy::useless_conversion)]
    let parsed: String = result.unwrap().into_iter().map(|p| p.as_str()).collect();
    assert_eq!(
        parsed, expected,
        "Round-trip mismatch for '{}' ({:?}, {})",
        input, rule, desc
    );
}

// ============================================================================
// Connector grammar tests - binary connector patterns from official examples
// ============================================================================

/// Test connector_endpoint rule with various patterns from official grammar:
/// ConnectorEnd = [mult]? [Name '::>']? (QualifiedName | FeatureChain)
#[rstest]
#[case("a.x", "simple feature chain")]
#[case("b", "simple name")]
#[case("self", "self reference")]
#[case("occ", "simple identifier")]
#[case("a ::> a.x", "named reference subsetting")]
#[case("[1] x", "multiplicity then reference")]
#[case("[0..1] self", "range multiplicity then self")]
fn test_connector_endpoint_patterns(#[case] input: &str, #[case] desc: &str) {
    let result = KerMLParser::parse(Rule::connector_endpoint, input);
    assert!(
        result.is_ok(),
        "connector_endpoint failed for '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Test binary connector patterns from official grammar:
/// BinaryConnectorDeclaration = ( FeatureDeclaration? 'from' | 'all' 'from'? )? ConnectorEnd 'to' ConnectorEnd
#[rstest]
#[case("connector a ::> a.x to b;", "named endpoint with ref subsetting")]
#[case("connector from self to occ;", "from keyword without declaration")]
#[case("connector x to y;", "simple binary without from")]
#[case(
    "connector all during: HappensDuring[0..1] from self to occ;",
    "all + declaration + from"
)]
#[case("connector all from x to y;", "all + from")]
#[case("connector all x to y;", "all without from")]
#[case("connector myConn: Type from a to b;", "typed declaration with from")]
#[case(
    "connector redefines fixWheel : Type [2] from [1] x to [1] y;",
    "redefines with multiplicity"
)]
fn test_binary_connector_patterns(#[case] input: &str, #[case] desc: &str) {
    let result = KerMLParser::parse(Rule::connector, input);
    assert!(
        result.is_ok(),
        "binary connector failed for '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

// ============================================================================
// Metadata annotation tests - @M inside feature bodies
// ============================================================================

/// Test metadata annotation inside end feature body
#[test]
fn test_metadata_annotation_in_end_feature_body() {
    // From Associations.kerml: end feature with @M; inside body
    let input = r#"end [0..1] feature x : X {
        @M;
    }"#;
    let result = KerMLParser::parse(Rule::end_feature, input);
    assert!(
        result.is_ok(),
        "end_feature with metadata annotation failed: {:?}",
        result.err()
    );
}

// ============================================================================
// Flow tests - from/to with feature chains
// ============================================================================

/// Test flow with feature chains: flow a.y to b.x1
#[test]
fn test_flow_with_feature_chains() {
    // From Behaviors.kerml: flow with feature chain endpoints
    let input = "flow a.y to b.x1;";
    let result = KerMLParser::parse(Rule::item_flow, input);
    assert!(
        result.is_ok(),
        "item_flow with feature chains failed: {:?}",
        result.err()
    );
}

// ============================================================================
// Multi-type feature tests - comma-separated types
// ============================================================================

/// Test feature with multiple types: y: A, '2'[0..*]
#[test]
fn test_feature_with_multiple_types() {
    // From Classes.kerml: feature typed by multiple types
    let input = "private y: A, '2'[0..*];";
    let result = KerMLParser::parse(Rule::namespace_body_element, input);
    assert!(
        result.is_ok(),
        "feature with multiple types failed: {:?}",
        result.err()
    );
}

// ============================================================================
// All-type expression tests - (all T) syntax
// ============================================================================

/// Test expression with all-type syntax: (all T)#(3)
#[test]
fn test_all_type_expression() {
    // First test: just "all T" extent expression
    let input = "all T";
    let result = KerMLParser::parse(Rule::extent_expression, input);
    assert!(
        result.is_ok(),
        "extent_expression 'all T' failed: {:?}",
        result.err()
    );

    // Second test: all T through operator_expression
    let input2 = "all T";
    let result2 = KerMLParser::parse(Rule::operator_expression, input2);
    assert!(
        result2.is_ok(),
        "operator_expression 'all T' failed: {:?}",
        result2.err()
    );

    // Third test: (all T) parenthesized - use operator_expression (inline_expression)
    let input3 = "(all T)";
    let result3 = KerMLParser::parse(Rule::operator_expression, input3);
    assert!(
        result3.is_ok(),
        "operator_expression '(all T)' failed: {:?}",
        result3.err()
    );

    // Fourth test: (all T)#(3) - parenthesized then indexed
    let input4 = "(all T)#(3)";
    let result4 = KerMLParser::parse(Rule::operator_expression, input4);
    assert!(
        result4.is_ok(),
        "operator_expression '(all T)#(3)' failed: {:?}",
        result4.err()
    );
}

// ============================================================================
// Specialization with subclassifier tests
// ============================================================================

/// Test specialization with subclassifier keyword
#[test]
fn test_specialization_with_subclassifier() {
    // From Classifiers.kerml: specialization Super subclassifier A specializes B;
    let input = "specialization Super subclassifier A specializes B;";
    let result = KerMLParser::parse(Rule::namespace_body_element, input);
    assert!(
        result.is_ok(),
        "specialization with subclassifier failed: {:?}",
        result.err()
    );
}

// ============================================================================
// Root namespace reference tests
// ============================================================================

/// Test root namespace reference with $:: prefix (Scoping.kerml)
#[rstest]
#[case("$::Objects::Object", "global qualified name")]
#[case("$::Root::Sub::Item", "deeper global qualified name")]
#[case("$::Root", "simple global qualified name")]
fn test_root_namespace_reference(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::qualified_reference_chain, input, desc);
}

/// Test root reference in class heritage (Scoping.kerml)
#[rstest]
#[case("class E :> $::Objects::Object;", "class with global heritage")]
#[case("class E :> '$'::Objects::Object;", "class with quoted $ heritage")]
fn test_class_with_root_reference(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::namespace_body_element, input, desc);
}

// ============================================================================
// Connector with featured by (TimeVaryingCarDriver.kerml)
// ============================================================================

/// Test connector with featured by before from/to endpoints
#[rstest]
#[case(
    "connector drive featured by Car from engine to transmission;",
    "connector featured by with from/to"
)]
#[case(
    "connector c featured by X from a to b { }",
    "connector featured by with body"
)]
#[case(
    "connector featured by Y from x to y;",
    "anonymous connector featured by"
)]
#[case(
    "member connector drive featured by Car from engine to transmission;",
    "member connector featured by"
)]
fn test_connector_with_featured_by(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::namespace_body_element, input, desc);
}

// ============================================================================
// Type with 'all' suffix before identification (Types.kerml)
// ============================================================================

/// Test type with 'all' keyword (sufficient type)
#[rstest]
#[case("type all x specializes A;", "type all with name")]
#[case("type all x specializes A, B;", "type all with multiple supertypes")]
#[case("classifier all C :> Base { }", "classifier all with body")]
fn test_type_all_sufficient(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::namespace_body_element, input, desc);
}

// ============================================================================
// Standalone conjugation (Types.kerml)
// ============================================================================

/// Test standalone conjugation syntax
#[rstest]
#[case(
    "conjugation c1 conjugate Conjugate1 conjugates Original;",
    "conjugation with conjugates"
)]
#[case(
    "conjugation c2 conjugate Conjugate2 ~ Original;",
    "conjugation with tilde"
)]
#[case("conjugate A ~ B;", "conjugate without conjugation prefix")]
fn test_standalone_conjugation(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::namespace_body_element, input, desc);
}

// ============================================================================
// Shorthand feature with ordered/nonunique after multiplicity (VehicleTanks.kerml)
// ============================================================================

/// Test shorthand features with modifiers and ordered/nonunique
#[rstest]
#[case("composite tanks: Tank[1..*] ordered;", "composite with ordered")]
#[case("portion items: Item[*] nonunique;", "portion with nonunique")]
#[case(
    "composite parts: Part[1..10] ordered nonunique;",
    "composite with both"
)]
fn test_shorthand_feature_with_multiplicity_props(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::namespace_body_element, input, desc);
}

// ============================================================================
// Unified grammar rules (any_relationship, feature_or_chain)
// ============================================================================

/// Test any_relationship - unified rule for heritage, type, and feature relationships
#[rstest]
#[case(":> Base", "heritage specialization")]
#[case("subsets parent", "heritage subsetting")]
#[case(":>> original", "heritage redefinition")]
#[case("~ Conjugate", "heritage conjugation")]
#[case("unions A", "type relationship unioning")]
#[case("differences B", "type relationship differencing")]
#[case(": Type", "feature typing")]
#[case("chains a.b", "feature chaining")]
fn test_any_relationship(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::any_relationship, input, desc);
}

/// Test feature_or_chain - unified rule for feature chain or element reference
#[rstest]
#[case("name", "simple identifier")]
#[case("Package::Element", "qualified name")]
#[case("a.b.c", "feature chain")]
#[case("$::Root::Element", "global qualified name")]
fn test_feature_or_chain(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::feature_or_chain, input, desc);
}

/// Test classifier_relationships - unified rule for classifier inheritance patterns
#[rstest]
#[case(":> Base", "heritage specialization")]
#[case(":> A, B", "multiple specialization")]
#[case(":> Base unions Other", "heritage with unioning")]
#[case("specializes Parent differences Child", "specializes with differences")]
fn test_classifier_relationships(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::classifier_relationships, input, desc);
}

/// Test ordering_modifiers - unified rule for ordered/nonunique
#[rstest]
#[case("ordered", "ordered only")]
#[case("nonunique", "nonunique only")]
#[case("ordered nonunique", "both ordered first")]
#[case("nonunique ordered", "both nonunique first")]
#[case("", "empty is valid")]
fn test_ordering_modifiers(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::ordering_modifiers, input, desc);
}

/// Test feature_prefix_modifiers - unified rule for feature modifiers
#[rstest]
#[case("abstract", "abstract only")]
#[case("composite", "composite only")]
#[case("abstract const", "abstract const")]
#[case("composite derived", "composite derived")]
#[case("portion var", "portion with var")]
#[case("", "empty is valid")]
fn test_feature_prefix_modifiers(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::feature_prefix_modifiers, input, desc);
}

/// Test connector_feature_modifiers - unified prefix for connector-like features
#[rstest]
#[case("abstract", "abstract only")]
#[case("composite", "composite only")]
#[case("abstract const", "abstract then const")]
#[case("const", "const only")]
#[case("", "empty is valid")]
fn test_connector_feature_modifiers(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::connector_feature_modifiers, input, desc);
}

/// Test connector_body_suffix - unified suffix for connector rules
#[rstest]
#[case("{}", "empty body")]
#[case(";", "semicolon terminator")]
fn test_connector_body_suffix(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::connector_body_suffix, input, desc);
}

/// Test specialization_prefix - optional specialization identification
#[rstest]
#[case("specialization", "just specialization")]
#[case("specialization mySpec", "specialization with name")]
#[case("", "empty is valid")]
fn test_specialization_prefix(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::specialization_prefix, input, desc);
}

/// Test optional_specialization_part - optional relationships with multiplicity
#[rstest]
#[case(":> Base", "just heritage")]
#[case(":> Base [1..*]", "heritage with multiplicity")]
#[case("[1..*]", "just multiplicity")]
#[case("[1..*] :> Base", "multiplicity then heritage")]
#[case(":> A, B [0..1] :> C", "mixed heritage and multiplicity")]
#[case("", "empty is valid")]
fn test_optional_specialization_part(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::optional_specialization_part, input, desc);
}

/// Test type_body - namespace body or semicolon terminator
#[rstest]
#[case("{}", "empty body")]
#[case(";", "semicolon terminator")]
fn test_type_body(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::type_body, input, desc);
}

/// Test feature_declaration - optional identification with optional specialization
#[rstest]
#[case("myFeature", "just name")]
#[case("myFeature :> Base", "name with specialization")]
#[case(":> Base", "just specialization")]
#[case("myFeature [1..*]", "name with multiplicity")]
#[case("", "empty is valid")]
fn test_feature_declaration(#[case] input: &str, #[case] desc: &str) {
    assert_round_trip(Rule::feature_declaration, input, desc);
}

// ============================================================================
// Locale documentation tests
// ============================================================================

/// Test locale with block comment
#[test]
fn test_locale_documentation() {
    // From Comments.kerml: locale "en_US" /* ... */
    let input = r#"locale "en_US" /* localized comment */"#;
    let result = KerMLParser::parse(Rule::namespace_body_element, input);
    assert!(
        result.is_ok(),
        "locale documentation failed: {:?}",
        result.err()
    );
}
