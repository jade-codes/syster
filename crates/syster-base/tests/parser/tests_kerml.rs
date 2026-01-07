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
fn assert_round_trip(rule: Rule, input: &str, desc: &str) {
    let result = KerMLParser::parse(rule, input)
        .unwrap_or_else(|e| panic!("Failed to parse {}: {}", desc, e));

    let parsed: String = result.into_iter().map(|p| p.as_str()).collect();

    assert_eq!(input, parsed, "Parsed output mismatch for {}", desc);
}

#[test]
fn test_parse_kerml_identifier() {
    let input = "myVar";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::identifier, input).unwrap();
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
#[case("readonly")]
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
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::keyword, keyword).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), keyword);
}

#[test]
fn test_parse_kerml_line_comment() {
    let input = "// this is a comment";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::line_comment, input).unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "// this is a comment");
}

#[test]
fn test_parse_kerml_block_comment() {
    let input = "/* block comment */";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::block_comment, input).unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "/* block comment */");
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
#[case("123", "123")]
#[case("0", "0")]
#[case("999999", "999999")]
fn test_parse_decimal(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::decimal, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("42", "42")]
#[case("3.14", "3.14")]
#[case(".5", ".5")]
fn test_parse_number(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::number, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("1.5e10", "1.5e10")]
#[case("2.0E-5", "2.0E-5")]
#[case("3e+2", "3e+2")]
fn test_parse_number_with_exponent(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::number, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("'simple'", "'simple'")]
#[case("'with space'", "'with space'")]
#[case("'with\\'quote'", "'with\\'quote'")]
fn test_parse_unrestricted_name(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::unrestricted_name, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("myName", "myName")]
#[case("'unrestricted name'", "'unrestricted name'")]
fn test_parse_name(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::name, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[test]
fn test_parse_string_value() {
    let input = r#""hello world""#;
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::string_value, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), r#""hello world""#);
}

// Identification Tests

#[rstest]
#[case("<shortName>", "<shortName>")]
#[case("<name123>", "<name123>")]
fn test_parse_short_name(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::short_name, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("regularName")]
#[case("'unrestricted name'")]
fn test_parse_regular_name(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::regular_name, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("<short> regular", "<short> regular")]
#[case("<short>", "<short>")]
#[case("regular", "regular")]
fn test_parse_identification(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::identification, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

// Relationship Token Tests

#[rstest]
#[case(":>", ":>")]
#[case("specializes", "specializes")]
fn test_parse_specializes_token(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::specializes_token, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case(":>>", ":>>")]
#[case("redefines", "redefines")]
fn test_parse_redefines_token(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::redefines_token, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case(":", ":")]
#[case("typed by", "typed by")]
fn test_parse_typed_by_token(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::typed_by_token, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("~", "~")]
#[case("conjugates", "conjugates")]
fn test_parse_conjugates_token(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::conjugates_token, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

// Common Fragment Tests

#[test]
fn test_parse_abstract_marker() {
    let input = "abstract";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::abstract_marker, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "abstract");
}

#[test]
fn test_parse_readonly() {
    let input = "readonly";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::readonly, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "readonly");
}

#[test]
fn test_parse_sufficient() {
    let input = "all";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::sufficient, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "all");
}

#[rstest]
#[case("ordered", "ordered")]
#[case("nonunique", "nonunique")]
#[case("ordered nonunique", "ordered nonunique")]
#[case("nonunique ordered", "nonunique ordered")]
fn test_parse_multiplicity_properties(#[case] input: &str, #[case] expected: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::multiplicity_properties, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("true", "true")]
#[case("false", "false")]
fn test_parse_literal_boolean(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::literal_boolean, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[test]
fn test_parse_literal_string() {
    let input = r#""test string""#;
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::literal_string, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), r#""test string""#);
}

#[rstest]
#[case("42")]
#[case("3.14")]
#[case("1.5e10")]
fn test_parse_literal_number(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::literal_number, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[test]
fn test_parse_literal_infinity() {
    let input = "*";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::literal_infinity, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "*");
}

#[rstest]
#[case("true")]
#[case(r#""string""#)]
#[case("42")]
#[case("*")]
fn test_parse_literal_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::literal_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("null", "null")]
#[case("()", "()")]
fn test_parse_null_expression(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::null_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("public")]
#[case("private")]
#[case("protected")]
fn test_parse_visibility_kind(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::visibility_kind, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("in")]
#[case("out")]
#[case("inout")]
fn test_parse_feature_direction_kind(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::feature_direction_kind, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("+", "+")]
#[case("-", "-")]
#[case("~", "~")]
#[case("not", "not")]
fn test_parse_unary_operator(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::unary_operator, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("hastype")]
#[case("istype")]
#[case("@")]
#[case("@@")]
fn test_parse_classification_test_operator(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::classification_test_operator,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("==", "==")]
#[case("!=", "!=")]
#[case("===", "===")]
#[case("!==", "!==")]
fn test_parse_equality_operator(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::equality_operator, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("<")]
#[case(">")]
#[case("<=")]
#[case(">=")]
fn test_parse_relational_operator(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::relational_operator, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("::*")]
#[case("::**")]
#[case("::*::**")]
fn test_parse_import_kind(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::import_kind, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Additional Common Fragment Tests

#[rstest]
#[case("public")]
#[case("private")]
#[case("protected")]
fn test_parse_visibility(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::visibility, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[test]
fn test_parse_derived() {
    let input = "derived";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::derived, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "derived");
}

#[test]
fn test_parse_end_marker() {
    let input = "end";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::end_marker, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "end");
}

#[test]
fn test_parse_standard() {
    let input = "standard";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::standard_marker, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "standard");
}

#[test]
fn test_parse_import_all() {
    let input = "all";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::import_all, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "all");
}

// Reference Tests

#[rstest]
#[case("Foo")]
#[case("Foo::Bar")]
#[case("Foo::Bar::Baz")]
fn test_parse_qualified_reference_chain(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::qualified_reference_chain,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("true")]
#[case(r#""test""#)]
#[case("42")]
#[case("null")]
fn test_parse_inline_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::inline_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Additional Token Tests
#[rstest]
#[case(":>", ":>")]
#[case("subsets", "subsets")]
fn test_parse_subsets_token(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::subsets_token, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("::>", "::>")]
#[case("references", "references")]
fn test_parse_references_token(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::references_token, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("=>", "=>")]
#[case("crosses", "crosses")]
fn test_parse_crosses_token(#[case] input: &str, #[case] expected: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::crosses_token, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), expected);
}

#[rstest]
#[case("myFeature")]
#[case("a.b")]
#[case("a.b.c")]
fn test_parse_feature_chain_expression(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::feature_chain_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("myArray")]
#[case("arr[0]")]
#[case("matrix[1][2]")]
fn test_parse_index_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::index_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Additional Expression and Metadata Tests

// Body Structure Tests

#[test]
fn test_parse_block_comment() {
    let input = "/* textual body */";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::block_comment, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), "/* textual body */");
}

#[rstest]
#[case(";")]
#[case("{}")]
fn test_parse_relationship_body(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::relationship_body, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Import and Filter Tests

#[rstest]
#[case("import")]
#[case("public import")]
#[case("private import")]
#[case("protected import")]
#[case("import all")]
#[case("private import all")]
fn test_parse_import_prefix(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::import_prefix, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("MyImport")]
#[case("MyImport::*")]
#[case("MyImport::**")]
#[case("MyImport::*::**")]
fn test_parse_imported_reference(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::imported_reference, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Relationship Declaration Tests

#[rstest]
#[case("BaseType")]
#[case("public BaseType")]
#[case("MyType::NestedType")]
fn test_parse_relationship(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::relationship, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("BaseType")]
#[case("private BaseClass")]
fn test_parse_inheritance(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::inheritance, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(":> BaseType")]
#[case("specializes BaseClass")]
#[case(":> public MyBase")]
fn test_parse_specialization(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::specialization, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(":> BaseType")]
#[case("subsets BaseClass")]
#[case(":> Base::MyType")]
#[case(":> Clock, Life")]
#[case(":> Type1, Type2, Type3")]
fn test_parse_subsetting(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::subsetting, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(":>> BaseType")]
#[case("redefines OldFeature")]
#[case(":>> Base::Type")]
#[case(":>> Collection::elements")]
#[case(":>> Feature1, Feature2")]
fn test_parse_redefinition(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::redefinition, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("::> RefType")]
#[case("references RefFeature")]
#[case("::> Ref::Feature")]
fn test_parse_reference_subsetting(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::reference_subsetting, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("=> CrossedType")]
#[case("crosses CrossedFeature")]
#[case("=> Cross::Type")]
fn test_parse_cross_subsetting(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::cross_subsetting, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("conjugates BaseType")]
#[case("conjugates public ConjugateType")]
fn test_parse_conjugation(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::conjugation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("unions Type1")]
#[case("unions public Type2")]
fn test_parse_unioning(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::unioning, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("differs Type1")]
#[case("differs private Type2")]
fn test_parse_differencing(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::differencing, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("intersects Type1")]
#[case("intersects public Type2")]
#[case("intersects VectorValue, Array")]
fn test_parse_intersecting(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::intersecting, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("chains feature1")]
#[case("chains public feature2")]
#[case("chains source.target")]
#[case("chains a.b.c")]
#[case("chains parent.child")]
fn test_parse_feature_chaining(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature_chaining, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("disjoint Type1")]
#[case("disjoint private Type2")]
fn test_parse_disjoining(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::disjoining, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("inverse feature1")]
#[case("inverse public feature2")]
fn test_parse_feature_inverting(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature_inverting, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("featured Type1")]
#[case("featured private Type2")]
fn test_parse_featuring(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::featuring, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("featuring featured Type1")]
#[case("featuring featured public Type2")]
fn test_parse_type_featuring(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::type_featuring, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("typed by :> BaseType")]
#[case(": specializes TypeSpec")]
#[case(": Complex[1]")]
#[case(": Boolean[1]")]
#[case(": Anything[2]")]
#[case(": String[0..*]")]
fn test_parse_feature_typing(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature_typing, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("subclassifier :> BaseClass")]
#[case("subclassifier specializes ClassSpec")]
fn test_parse_subclassification(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::subclassification, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("MyRef")]
#[case("public MyRef")]
#[case("alias MyRef")]
#[case("private alias")]
fn test_parse_membership(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::membership, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("MyRef")]
#[case("public alias MyRef")]
fn test_parse_owning_membership(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::owning_membership, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("= MyRef")]
#[case(":= public MyRef")]
#[case("= alias Target")]
fn test_parse_feature_value(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature_value, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("filter MyRef")]
#[case("filter public alias Target")]
fn test_parse_element_filter_membership(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::element_filter_membership,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("featured MyType MyRef")]
#[case("featured public BaseType alias Target")]
fn test_parse_feature_membership(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature_membership, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("end featured MyType MyRef")]
#[case("end featured public BaseType alias Target")]
fn test_parse_end_feature_membership(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::end_feature_membership, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("return featured MyType MyRef")]
#[case("return featured public BaseType alias Target")]
fn test_parse_result_expression_membership(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::result_expression_membership,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("import MyPackage;")]
#[case("public import MyLib;")]
#[case("import all MyNamespace;")]
#[case("private import all Base;")]
#[case("import MyPackage::*;")]
#[case("import MyPackage::**;")]
#[case("import MyPackage {}")]
fn test_parse_import(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::import, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("dependency Source to Target;")]
#[case("dependency MyDep from Source to Target;")]
#[case("dependency Source, Other to Target, Dest;")]
#[case("dependency <short> named from Source to Target {}")]
fn test_parse_dependency(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::dependency, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Element Declaration Tests

#[rstest]
#[case("namespace MyNamespace;")]
#[case("namespace MyNamespace {}")]
#[case("namespace <short> named {}")]
fn test_parse_namespace(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::namespace, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("package MyPackage;")]
#[case("package MyPackage {}")]
#[case("package <short> named {}")]
fn test_parse_package(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::package, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("library package LibPkg;")]
#[case("standard library package StdLib;")]
#[case("library package MyLib {}")]
fn test_parse_library_package(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::library_package, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("class MyClass;")]
#[case("class MyClass {}")]
#[case("abstract class MyClass;")]
#[case("class MyClass specializes Base {}")]
#[case("abstract class MyClass specializes Base, Other {}")]
fn test_parse_class(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::class, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("datatype MyData;")]
#[case("datatype MyData {}")]
#[case("abstract datatype ScalarValue specializes DataValue;")]
#[case("datatype Boolean specializes ScalarValue;")]
#[case("datatype String specializes ScalarValue;")]
fn test_parse_data_type(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::data_type, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("struct MyStruct;")]
#[case("struct MyStruct {}")]
#[case("struct MyStruct[1] :> Parent {}")]
#[case("private struct MyStruct[0..1] specializes Base {}")]
#[case("abstract struct MyStruct specializes Base, Other {}")]
fn test_parse_structure(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::structure, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("assoc MyAssoc;")]
#[case("assoc MyAssoc {}")]
#[case("abstract assoc Link specializes Anything {}")]
#[case("assoc MyAssoc specializes Base {}")]
fn test_parse_association(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::association, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("assoc struct MyAssocStruct;")]
#[case("assoc struct MyAssocStruct {}")]
fn test_parse_association_structure(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::association_structure, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("behavior MyBehavior;")]
#[case("behavior MyBehavior {}")]
#[case("abstract behavior DecisionPerformance specializes Performance {}")]
#[case("behavior MyBehavior specializes Base, Other {}")]
fn test_parse_behavior(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::behavior, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("function MyFunction;")]
#[case("function MyFunction {}")]
fn test_parse_function(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::function, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("predicate MyPredicate;")]
#[case("predicate MyPredicate {}")]
fn test_parse_predicate(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::predicate, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("interaction MyInteraction;")]
#[case("interaction MyInteraction {}")]
fn test_parse_interaction(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::interaction, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("metaclass MyMetaclass;")]
#[case("metaclass MyMetaclass {}")]
fn test_parse_metaclass(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::metaclass, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("connector MyConnector;")]
#[case("connector MyConnector {}")]
fn test_parse_connector(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::connector, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("binding MyBinding;")]
#[case("binding MyBinding {}")]
fn test_parse_binding_connector(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::binding_connector, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("succession MySuccession;")]
#[case("succession MySuccession {}")]
fn test_parse_succession(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::succession, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("step MyStep;")]
#[case("step MyStep {}")]
fn test_parse_step(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::step, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("expr MyExpr;")]
#[case("expr MyExpr {}")]
fn test_parse_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("inv MyInvariant;")]
#[case("inv not MyInvariant {}")]
fn test_parse_invariant(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::invariant, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Feature Tests

#[rstest]
#[case("feature MyFeature;")]
#[case("feature MyFeature {}")]
fn test_parse_feature_basic(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("in feature MyFeature;")]
#[case("out feature MyFeature;")]
#[case("inout feature MyFeature;")]
fn test_parse_feature_with_direction(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("abstract feature MyFeature;")]
#[case("composite feature MyFeature;")]
#[case("portion feature MyFeature;")]
fn test_parse_feature_with_composition(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("readonly feature MyFeature;")]
#[case("derived feature MyFeature;")]
#[case("end feature MyFeature;")]
fn test_parse_feature_with_property(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("feature MyFeature ordered;")]
#[case("feature MyFeature nonunique;")]
#[case("feature MyFeature ordered nonunique;")]
fn test_parse_feature_with_multiplicity_properties(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("in abstract readonly feature MyFeature ordered;")]
#[case("out composite derived feature MyFeature nonunique;")]
#[case("inout portion end feature MyFeature ordered nonunique;")]
fn test_parse_feature_combined_modifiers(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("feature elements[0..*] :>> Collection::elements {}")]
#[case("feature myFeature[1] :> BaseFeature;")]
#[case("feature items[*] : ItemType ordered;")]
fn test_parse_feature_with_multiplicity_and_relationships(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Annotation Element Tests

#[rstest]
#[case("comment /* simple comment */")]
#[case("comment myComment /* comment text */")]
fn test_parse_comment_basic(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::comment_annotation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(r#"comment locale "en-US" /* comment text */"#)]
#[case(r#"comment MyComment locale "fr-FR" /* texte */"#)]
fn test_parse_comment_with_locale(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::comment_annotation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("comment about Foo /* about Foo */")]
#[case("comment about Bar, Baz /* about multiple */")]
fn test_parse_comment_with_about(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::comment_annotation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("doc /* documentation */")]
#[case("doc MyDoc /* doc text */")]
fn test_parse_documentation_basic(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::documentation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(r#"doc locale "en-US" /* docs */"#)]
#[case(r#"doc MyDoc locale "ja-JP" /* text */"#)]
fn test_parse_documentation_with_locale(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::documentation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(r#"language "rust" /* code */"#)]
#[case(r#"rep language "python" /* code */"#)]
#[case(r#"rep MyRep language "java" /* code */"#)]
fn test_parse_textual_representation(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::textual_representation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Multiplicity tests
#[rstest]
#[case("feature;")]
#[case("feature myMultiplicity;")]
#[case("feature myMultiplicity : MyType;")]
fn test_parse_multiplicity(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::multiplicity, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// MultiplicityRange tests
#[rstest]
#[case("feature;")]
#[case("feature myRange;")]
#[case("feature myRange { feature bound; }")]
fn test_parse_multiplicity_range(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::multiplicity_range, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// MetadataFeature tests
#[rstest]
#[case("metadata type;")]
#[case("metadata type myMeta;")]
#[case("metadata type about Foo;")]
#[case("metadata type myMeta about Foo, Bar;")]
fn test_parse_metadata_feature(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::metadata_feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// ItemFeature tests
#[rstest]
#[case("feature;")]
#[case("feature myItem;")]
#[case("feature myItem : ItemType;")]
fn test_parse_item_feature(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::item_feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// ItemFlow tests
#[rstest]
#[case("flow connector;")]
#[case("flow connector myFlow;")]
fn test_parse_item_flow(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::item_flow, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// SuccessionItemFlow tests
#[rstest]
#[case("succession flow flow connector;")]
#[case("succession flow flow connector myFlow;")]
fn test_parse_succession_item_flow(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::succession_item_flow, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// BooleanExpression tests
#[rstest]
#[case("expr;")]
#[case("expr myBool;")]
fn test_parse_boolean_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::boolean_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Tests for missing critical rules

#[rstest]
#[case(Rule::file, "", "empty file")]
#[case(Rule::file, "   \n\t  \r\n  ", "file with whitespace")]
fn test_parse_file(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case("3.14")]
#[case(".5")]
#[case("0.0")]
fn test_parse_float(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::float, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(".5")]
#[case(".123")]
#[case(".0")]
fn test_parse_fraction(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::fraction, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("e10")]
#[case("E-5")]
#[case("e+3")]
fn test_parse_exponent(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::exponent, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("myElement")]
#[case("Base::Derived")]
#[case("Pkg::Sub::Element")]
fn test_parse_element_reference(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::element_reference, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("MyType")]
#[case("Base::MyType")]
fn test_parse_type_reference(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::type_reference, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("myFeature")]
#[case("Base::myFeature")]
fn test_parse_feature_reference(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature_reference, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("MyClassifier")]
#[case("Base::MyClassifier")]
fn test_parse_classifier_reference(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::classifier_reference, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("<shortName>")]
#[case("regularName")]
#[case("<shortName> regularName")]
fn test_parse_element(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::element, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("MyElement")]
fn test_parse_annotation(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::annotation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("comment /* text */")]
#[case("doc /* documentation */")]
fn test_parse_owned_annotation(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::owned_annotation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
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
#[case("namespace MyNamespace;")]
#[case("namespace MyNamespace {}")]
fn test_parse_namespace_body(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::namespace, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    // Verify namespace rule was matched and input was fully consumed
    assert_eq!(parsed.as_rule(), syster::parser::kerml::Rule::namespace);
    assert_eq!(parsed.as_str(), input);
}

// High-priority missing rules

#[rstest]
#[case("type MyType;")]
#[case("abstract type MyType {}")]
#[case("type MyType all {}")]
#[case("type MyType ordered {}")]
#[case("type MyType unions BaseType {}")]
#[case("type MyType differs BaseType {}")]
fn test_parse_type_def(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::type_def, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("classifier MyClassifier;")]
#[case("abstract classifier MyClassifier {}")]
#[case("classifier MyClassifier all {}")]
#[case("classifier MyClassifier unions BaseClassifier {}")]
fn test_parse_classifier(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::classifier, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("null")]
#[case("true")]
#[case("myFeature")]
fn test_parse_operator_expression(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::operator_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("obj.metadata")]
#[case("Base::Feature.metadata")]
fn test_parse_metadata_access_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::metadata_access_expression,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(Rule::root_namespace, "", "empty root namespace")]
fn test_parse_root_namespace(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[test]
fn test_parse_root_namespace_with_package() {
    let input = "package MyPackage;";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::root_namespace, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(
        parsed.as_rule(),
        syster::parser::kerml::Rule::root_namespace
    );
    // Verify the input was fully consumed
    assert_eq!(parsed.as_str(), input);
}

#[test]
fn test_parse_root_namespace_with_multiple_elements() {
    let input = "package Pkg1; package Pkg2;";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::root_namespace, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(
        parsed.as_rule(),
        syster::parser::kerml::Rule::root_namespace
    );
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
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::invocation_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("\"hello\"")]
#[case("\"hello\".toUpper")]
fn test_parse_collect_expression(#[case] input: &str) {
    // collect_expression is in inline_expression union
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::inline_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("\"world\"")]
#[case("myVar.property")]
fn test_parse_select_expression(#[case] input: &str) {
    // select_expression is in inline_expression union
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::inline_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test feature with ordered/nonunique after typing
#[rstest]
#[case("feature dimensions: Positive[0..*] ordered nonunique { }")]
#[case("feature x: Type ordered { }")]
#[case("feature y: T nonunique { }")]
#[case("feature z: T[1] ordered nonunique;")]
fn test_parse_feature_with_modifiers_after_typing(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test feature value with expressions
#[rstest]
#[case("feature rank: Natural[1] = size(dimensions);")]
#[case("feature x = 3;")]
#[case("feature y = foo();")]
fn test_parse_feature_value_with_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test documentation with block comments
#[rstest]
#[case("doc /* This is documentation */")]
#[case("doc /* Multi-line\n * documentation\n */")]
#[case("doc /* Simple */")]
fn test_parse_documentation(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::documentation, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test parameter membership (function parameters)
#[rstest]
#[case("in x: Anything[0..1];")]
#[case("in y: Boolean[1];")]
#[case("out result: Natural[1];")]
#[case("inout value: Complex[0..*];")]
#[case("in x: Anything[0..*] nonunique;")]
#[case("in x: Anything[0..*] ordered;")]
fn test_parse_parameter_membership(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::parameter_membership, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test return parameter membership
#[rstest]
#[case("return : Boolean[1];")]
#[case("return result: Natural[1];")]
#[case("return : Complex[1] = x + y;")]
fn test_parse_return_parameter_membership(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::return_parameter_membership,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test functions with quoted operator names
#[rstest]
#[case("function '==' { }")]
#[case("function '!=' { }")]
#[case("function '+' { }")]
#[case("abstract function '-' { }")]
fn test_parse_function_with_operator_name(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::function, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test complete function with parameters
#[rstest]
#[case("function '=='{ in x: Anything[0..1]; in y: Anything[0..1]; return : Boolean[1]; }")]
#[case("function add { in a: Natural[1]; in b: Natural[1]; return : Natural[1]; }")]
#[case(
    "abstract function compare { in x: Anything[0..1]; in y: Anything[0..1]; return : Boolean[1]; }"
)]
fn test_parse_function_with_parameters(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::function, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test quoted identifiers
#[rstest]
#[case("'=='")]
#[case("'!='")]
#[case("'+'")]
#[case("'-'")]
#[case("'*'")]
#[case("'/'")]
#[case("'<'")]
#[case("'>'")]
#[case("'<='")]
#[case("'>='")]
fn test_parse_quoted_identifier(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::identifier, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test qualified references with quoted identifiers
#[rstest]
#[case("ScalarFunctions::'not'")]
#[case("Base::'=='")]
#[case("Math::'+'")]
#[case("Ops::'*'::'nested'")]
fn test_parse_qualified_reference_with_quotes(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::qualified_reference_chain,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test function specialization with quoted names
#[rstest]
#[case("function 'not' specializes ScalarFunctions::'not' { }")]
#[case("function 'xor' specializes Base::'xor' { }")]
fn test_parse_function_specializes_quoted(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::function, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test invocation with numeric arguments
#[rstest]
#[case("rect(0.0, 1.0)")]
#[case("polar(1.0, 3.14)")]
#[case("add(42, 17)")]
fn test_parse_invocation_with_numbers(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::invocation_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test feature with invocation value
#[rstest]
#[case("feature i: Complex[1] = rect(0.0, 1.0);")]
#[case("feature x: Real[1] = sqrt(2.0);")]
fn test_parse_feature_with_invocation_value(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test top-level feature (namespace feature member)
#[rstest]
#[case("feature i: Complex[1] = rect(0.0, 1.0);")]
#[case("feature x: Natural[1] = 42;")]
fn test_parse_namespace_feature_with_value(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::namespace_feature_member, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test feature with chaining relationship
#[rstest]
#[case("feature chain chains source.target;")]
#[case("private feature chain chains source.target;")]
fn test_parse_feature_with_chaining(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::feature, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test return parameter with default value
#[rstest]
#[case("return : Integer[1] default sum0(collection, 0);")]
#[case("return : Boolean[1] default true;")]
#[case("return result: Natural[1] default 0;")]
fn test_parse_return_parameter_with_default(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::return_parameter_membership,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test function with return default
#[rstest]
#[case(
    "function sum { in collection: Integer[0..*]; return : Integer[1] default sum0(collection, 0); }"
)]
fn test_parse_function_with_return_default(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::function, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}
// Test binary operator expressions
#[rstest]
#[case("x == y")]
#[case("x != y")]
#[case("x === y")]
#[case("x < y")]
#[case("x <= y")]
#[case("x > y")]
#[case("x >= y")]
#[case("x + y")]
#[case("x - y")]
#[case("x * y")]
#[case("x / y")]
#[case("x and y")]
#[case("x or y")]
#[case("x xor y")]
#[case("a == b and c == d")]
fn test_parse_binary_expression(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::operator_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test return with binary expression
#[rstest]
#[case("return : Boolean[1] = x == y;")]
#[case("return : Boolean[1] = x != y;")]
#[case("return : Boolean[1] = x < y;")]
fn test_parse_return_with_binary_expression(#[case] input: &str) {
    let pairs = KerMLParser::parse(
        syster::parser::kerml::Rule::return_parameter_membership,
        input,
    )
    .unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test function with special operator names
#[rstest]
#[case("function '..' { in x: Integer[1]; return : Integer[1]; }")]
#[case("function test { return : Integer[0..*]; }")]
#[case(
    "abstract function '..' { in lower: DataValue[1]; in upper: DataValue[1]; return : DataValue[0..*] ordered; }"
)]
fn test_parse_function_with_range_operator(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::function, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test conditional expressions
#[rstest]
#[case("if true ? 1 else 0")]
#[case("if x > 5 ? 'yes' else 'no'")]
#[case("if isEmpty(seq)? 0 else size(tail(seq)) + 1")]
fn test_parse_conditional_expression(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::operator_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test tuple literals
#[rstest]
#[case("(a, b)")]
#[case("(1, 2, 3)")]
#[case("(seq1, seq2)")]
fn test_parse_tuple_expression(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::operator_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test null coalescing operator
#[rstest]
#[case("x ?? 0")]
#[case("dimensions->reduce '*' ?? 1")]
fn test_parse_null_coalescing(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::operator_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test arrow operator for collections
#[rstest]
#[case("col->reduce '+' ?? zero")]
#[case("collection->select {in x; x > 0}")]
#[case("col.elements->equals(other.elements)")]
#[case("coll->collect{in i : Positive; v#(i) + w#(i)}")]
fn test_parse_collection_operators(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::operator_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test as operator for type casting
#[rstest]
#[case("x as Integer")]
#[case("(col.elements as Anything)#(index)")]
fn test_parse_as_operator(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::operator_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test character literals
#[rstest]
#[case("'*'")]
#[case("'+'")]
#[case("'a'")]
fn test_parse_char_literal(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::literal_expression, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test parameters with default values
#[rstest]
#[case("in x: Integer[1] default 0;")]
#[case("in endIndex: Positive[1] default startIndex;")]
fn test_parse_parameter_with_default(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::parameter_membership, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

// Test expression parameters
#[rstest]
#[case("in expr thenValue[0..1] { return : Anything[0..*] ordered nonunique; }")]
#[case("in step myStep { in x: Integer[1]; }")]
fn test_parse_expression_parameters(#[case] input: &str) {
    let pairs =
        KerMLParser::parse(syster::parser::kerml::Rule::parameter_membership, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
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
    "comment about StructuredSurface, StructuredCurve, StructuredPoint",
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
    Rule::disjoining,
    "disjoint earlierOccurrence.successors from laterOccurrence.predecessors;",
    "disjoint feature chains from"
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

#[test]
fn test_parse_scalar_values_stdlib_file() {
    let content = r#"standard library package ScalarValues {
    private import Base::DataValue;
    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
}"#;

    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, content).unwrap();
    for pair in pairs.clone() {
        for inner in pair.into_inner() {
            for _inner2 in inner.into_inner() {}
        }
    }

    // Try to convert to KerMLFile
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, content).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();
    for _elem in file.elements.iter() {}

    assert!(!file.elements.is_empty(), "File should have elements!");
}

// ============================================================================
// AST Parsing Tests - Verify correct AST structure construction
// ============================================================================

#[test]
fn test_parse_classifier_with_specialization_ast() {
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "classifier Car specializes Vehicle;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "classifier SportsCar specializes Car, Vehicle;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "feature mass : Real;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "feature currentMass redefines mass;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "feature wheelMass subsets mass;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "feature currentMass : Real redefines mass;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "abstract classifier Vehicle;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
fn test_parse_readonly_feature_ast() {
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = r#"
        package Test {
            readonly feature id : String;
        }
    "#;
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    assert!(f.is_readonly, "Feature should be readonly");
}

#[test]
fn test_parse_datatype_ast() {
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "datatype Real;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = "function calculateArea;";
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
    use from_pest::FromPest;
    use syster::syntax::kerml::ast::KerMLFile;

    let input = r#"classifier Vehicle {
        feature mass : Real;
    }"#;
    let mut pairs = KerMLParser::parse(syster::parser::kerml::Rule::file, input).unwrap();
    let file = KerMLFile::from_pest(&mut pairs).unwrap();

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
