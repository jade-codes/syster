#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use pest::Parser;
use rstest::rstest;
use syster::parser::KerMLParser;
use syster::syntax::kerml::enums::*;
use syster::syntax::kerml::types::*;

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
        format!("{:?}", element),
        "Element { declared_name: None, declared_short_name: None }"
    );
}

#[test]
fn test_annotation_creation() {
    let annotation = Annotation {};
    assert!(format!("{:?}", annotation).contains("Annotation"));
}

#[test]
fn test_annotating_element_empty() {
    let annotating = AnnotatingElement { about: vec![] };
    assert_eq!(annotating.about.len(), 0);
}

#[test]
fn test_annotating_element_with_annotations() {
    let annotation1 = Annotation {};
    let annotation2 = Annotation {};

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
    let annotation = Annotation {};
    let cloned = annotation.clone();
    assert_eq!(annotation, cloned);
}

#[test]
fn test_equality_annotations() {
    let annotation1 = Annotation {};
    let annotation2 = Annotation {};
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
    assert!(format!("{:?}", inheritance).contains("Inheritance"));
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
    assert!(format!("{:?}", feature_ref).contains("FeatureReference"));
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
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::standard, input).unwrap();
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
fn test_parse_textual_body() {
    let input = "/* textual body */";
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::textual_body, input).unwrap();
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
fn test_parse_subsetting(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::subsetting, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(":>> BaseType")]
#[case("redefines OldFeature")]
#[case(":>> Base::Type")]
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
fn test_parse_intersecting(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::intersecting, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("chains feature1")]
#[case("chains public feature2")]
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
#[case("library standard package StdLib;")]
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
fn test_parse_class(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::class, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("datatype MyData;")]
#[case("datatype MyData {}")]
fn test_parse_data_type(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::data_type, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("struct MyStruct;")]
#[case("struct MyStruct {}")]
fn test_parse_structure(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::structure, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("assoc MyAssoc;")]
#[case("assoc MyAssoc {}")]
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
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::comment, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case(r#"comment locale "en-US" /* comment text */"#)]
#[case(r#"comment MyComment locale "fr-FR" /* texte */"#)]
fn test_parse_comment_with_locale(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::comment, input).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), input);
}

#[rstest]
#[case("comment about Foo /* about Foo */")]
#[case("comment about Bar, Baz /* about multiple */")]
fn test_parse_comment_with_about(#[case] input: &str) {
    let pairs = KerMLParser::parse(syster::parser::kerml::Rule::comment, input).unwrap();
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

#[test]
fn test_parse_file_empty() {
    let input = "";
    let result = KerMLParser::parse(syster::parser::kerml::Rule::file, input);
    assert!(
        result.is_ok(),
        "Failed to parse empty file: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_file_with_whitespace() {
    let input = "   \n\t  \r\n  ";
    let result = KerMLParser::parse(syster::parser::kerml::Rule::file, input);
    assert!(
        result.is_ok(),
        "Failed to parse file with whitespace: {:?}",
        result.err()
    );
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

#[test]
fn test_parse_root_namespace_empty() {
    let input = "";
    let result = KerMLParser::parse(syster::parser::kerml::Rule::root_namespace, input);
    assert!(
        result.is_ok(),
        "Failed to parse empty root namespace: {:?}",
        result.err()
    );
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
