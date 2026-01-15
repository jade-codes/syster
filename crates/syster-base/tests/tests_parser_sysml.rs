#![allow(clippy::unwrap_used)]

use pest::Parser;
use rstest::rstest;
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_parse_simple_identifier() {
    let input = "myVar";
    let result = SysMLParser::parse(Rule::identifier, input);

    assert!(
        result.is_ok(),
        "Failed to parse identifier: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let identifier = pairs.into_iter().next().unwrap();
    assert_eq!(identifier.as_str(), "myVar");
}

#[rstest]
#[case("about")]
#[case("abstract")]
#[case("accept")]
#[case("action")]
#[case("actor")]
#[case("after")]
#[case("alias")]
#[case("all")]
#[case("allocate")]
#[case("allocation")]
#[case("analysis")]
#[case("and")]
#[case("as")]
#[case("assert")]
#[case("assign")]
#[case("assume")]
#[case("at")]
#[case("attribute")]
#[case("bind")]
#[case("binding")]
#[case("by")]
#[case("calc")]
#[case("case")]
#[case("comment")]
#[case("concern")]
#[case("connect")]
#[case("connection")]
#[case("constraint")]
#[case("crosses")]
#[case("decide")]
#[case("def")]
#[case("default")]
#[case("defined")]
#[case("dependency")]
#[case("derived")]
#[case("do")]
#[case("doc")]
#[case("else")]
#[case("end")]
#[case("entry")]
#[case("enum")]
#[case("event")]
#[case("exhibit")]
#[case("exit")]
#[case("expose")]
#[case("false")]
#[case("filter")]
#[case("first")]
#[case("flow")]
#[case("for")]
#[case("fork")]
#[case("frame")]
#[case("from")]
#[case("hastype")]
#[case("if")]
#[case("implies")]
#[case("import")]
#[case("in")]
#[case("include")]
#[case("individual")]
#[case("inout")]
#[case("interface")]
#[case("istype")]
#[case("item")]
#[case("join")]
#[case("language")]
#[case("library")]
#[case("locale")]
#[case("loop")]
#[case("merge")]
#[case("message")]
#[case("meta")]
#[case("metadata")]
#[case("nonunique")]
#[case("not")]
#[case("null")]
#[case("objective")]
#[case("occurrence")]
#[case("of")]
#[case("or")]
#[case("ordered")]
#[case("out")]
#[case("package")]
#[case("parallel")]
#[case("part")]
#[case("perform")]
#[case("port")]
#[case("private")]
#[case("protected")]
#[case("public")]
#[case("redefines")]
#[case("ref")]
#[case("references")]
#[case("render")]
#[case("rendering")]
#[case("rep")]
#[case("require")]
#[case("requirement")]
#[case("return")]
#[case("satisfy")]
#[case("send")]
#[case("snapshot")]
#[case("specializes")]
#[case("stakeholder")]
#[case("standard")]
#[case("state")]
#[case("subject")]
#[case("subsets")]
#[case("succession")]
#[case("terminate")]
#[case("then")]
#[case("timeslice")]
#[case("to")]
#[case("transition")]
#[case("true")]
#[case("until")]
#[case("use")]
#[case("variant")]
#[case("variation")]
#[case("verification")]
#[case("verify")]
#[case("via")]
#[case("view")]
#[case("viewpoint")]
#[case("when")]
#[case("while")]
#[case("xor")]
fn test_parse_keywords(#[case] keyword: &str) {
    let pairs = SysMLParser::parse(Rule::keyword, keyword).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), keyword);
}

#[test]
fn test_parse_line_comment() {
    let input = "// this is a comment";
    let result = SysMLParser::parse(Rule::line_comment, input);

    assert!(
        result.is_ok(),
        "Failed to parse line comment: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "// this is a comment");
}

#[test]
fn test_parse_block_comment() {
    let input = "/* block comment */";
    let result = SysMLParser::parse(Rule::block_comment, input);

    assert!(
        result.is_ok(),
        "Failed to parse block comment: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "/* block comment */");
}

#[test]
fn test_parse_multiline_block_comment() {
    let input = "/* line 1\nline 2\nline 3 */";
    let result = SysMLParser::parse(Rule::block_comment, input);

    assert!(
        result.is_ok(),
        "Failed to parse multiline block comment: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "/* line 1\nline 2\nline 3 */");
}

#[test]
fn test_parse_empty_file() {
    let input = "";
    let result = SysMLParser::parse(Rule::file, input);

    assert!(
        result.is_ok(),
        "Failed to parse empty file: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_file_with_whitespace() {
    let input = "   \n\t  \r\n  ";
    let result = SysMLParser::parse(Rule::file, input);

    assert!(
        result.is_ok(),
        "Failed to parse file with whitespace: {:?}",
        result.err()
    );
}

// Control Node Tests

#[rstest]
#[case("fork;", "fork node")]
#[case("fork myFork;", "fork with name")]
fn test_parse_fork_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::fork_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("merge;", "merge node")]
#[case("merge myMerge;", "merge with name")]
fn test_parse_merge_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::merge_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("join;", "join node")]
#[case("join myJoin;", "join with name")]
fn test_parse_join_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::join_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("decide;", "decision node")]
#[case("decide myDecision;", "decision with name")]
fn test_parse_decision_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::decision_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// State Subaction Membership Tests

#[test]
fn test_parse_entry_action() {
    let input = "entry myEntryAction;";
    let result = SysMLParser::parse(Rule::state_subaction_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse entry action: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_exit_action() {
    let input = "exit myExitAction;";
    let result = SysMLParser::parse(Rule::state_subaction_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse exit action: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_do_action() {
    let input = "do myDoAction;";
    let result = SysMLParser::parse(Rule::state_subaction_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse do action: {:?}",
        result.err()
    );
}

// Transition Feature Membership Tests

#[test]
fn test_parse_accept_feature() {
    let input = "accept myAcceptFeature;";
    let result = SysMLParser::parse(Rule::transition_feature_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse accept feature: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_if_feature() {
    let input = "if myCondition;";
    let result = SysMLParser::parse(Rule::transition_feature_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse if feature: {:?}",
        result.err()
    );
}

// Parameter Membership Tests

#[test]
fn test_parse_subject_membership() {
    let input = "subject mySubject;";
    let result = SysMLParser::parse(Rule::subject_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse subject membership: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_actor_membership() {
    let input = "actor myActor;";
    let result = SysMLParser::parse(Rule::actor_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse actor membership: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_stakeholder_membership() {
    let input = "stakeholder myStakeholder;";
    let result = SysMLParser::parse(Rule::stakeholder_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse stakeholder membership: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_objective_membership() {
    let input = "objective myObjective;";
    let result = SysMLParser::parse(Rule::objective_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse objective membership: {:?}",
        result.err()
    );
}

// Succession and Expose Tests

#[rstest]
#[case("first source then target;", "simple succession")]
#[case("first source then target { }", "succession with body")]
#[case(
    "succession mySuccession first source then target;",
    "succession with declaration"
)]
fn test_parse_succession_as_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::succession_as_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("succession", "succession keyword")]
fn test_parse_succession_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::succession_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[test]
fn test_parse_expose() {
    let input = "expose MyElement;";
    let result = SysMLParser::parse(Rule::expose, input);

    assert!(result.is_ok(), "Failed to parse expose: {:?}", result.err());
}

#[test]
fn test_parse_membership_expose() {
    let input = "expose MyElement::member;";
    let result = SysMLParser::parse(Rule::membership_expose, input);

    assert!(
        result.is_ok(),
        "Failed to parse membership expose: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_namespace_expose() {
    let input = "expose MyNamespace::*;";
    let result = SysMLParser::parse(Rule::namespace_expose, input);

    assert!(
        result.is_ok(),
        "Failed to parse namespace expose: {:?}",
        result.err()
    );
}

// Requirement Constraint Memberships

#[test]
fn test_parse_requirement_constraint_membership() {
    let input = "require myConstraint;";
    let result = SysMLParser::parse(Rule::requirement_constraint_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse requirement constraint membership: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_framed_concern_membership() {
    let input = "frame myConcern;";
    let result = SysMLParser::parse(Rule::framed_concern_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse framed concern membership: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_requirement_verification_membership() {
    let input = "verify myVerification;";
    let result = SysMLParser::parse(Rule::requirement_verification_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse requirement verification membership: {:?}",
        result.err()
    );
}

// Port and Conjugation Tests

#[test]
fn test_parse_conjugated_port_type_reference() {
    let input = "~MyPort";
    let result = SysMLParser::parse(Rule::owned_feature_typing, input);

    assert!(
        result.is_ok(),
        "Failed to parse conjugated port typing: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_variant_membership() {
    let input = "variant myVariant;";
    let result = SysMLParser::parse(Rule::variant_membership, input);

    assert!(
        result.is_ok(),
        "Failed to parse variant membership: {:?}",
        result.err()
    );
}

// Terminate Action

#[test]
fn test_parse_terminate_action() {
    let input = "terminate myOccurrence;";
    let result = SysMLParser::parse(Rule::terminate_action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse terminate action: {:?}",
        result.err()
    );
}

// Port Definition and Conjugation Tests

#[test]
fn test_parse_conjugated_port_definition() {
    let input = "port def ~MyConjugatedPort;";
    let result = SysMLParser::parse(Rule::conjugated_port_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse conjugated port definition: {:?}",
        result.err()
    );
}

// Test full port usage with conjugated type
#[test]
fn test_parse_port_usage_with_conjugated_type() {
    let input = "port myPort : ~ConjugatedPortType;";
    let result = SysMLParser::parse(Rule::port_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse port usage with conjugated type: {:?}",
        result.err()
    );
}

// Token Tests

#[rstest]
#[case(":")]
#[case("defined by")]
fn test_parse_defined_by_token(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::defined_by_token, input);
    assert!(
        result.is_ok(),
        "Failed to parse defined by token: {:?}",
        result.err()
    );
}

// Enum Tests

#[rstest]
#[case("timeslice")]
#[case("snapshot")]
fn test_parse_portion_kind(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::portion_kind, input);
    assert!(
        result.is_ok(),
        "Failed to parse portion kind: {:?}",
        result.err()
    );
}

#[rstest]
#[case("assume")]
#[case("require")]
fn test_parse_requirement_constraint_kind(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::requirement_constraint_kind, input);
    assert!(
        result.is_ok(),
        "Failed to parse requirement constraint kind: {:?}",
        result.err()
    );
}

// Fragment Tests

#[rstest]
#[case("variation")]
#[case("individual")]
fn test_parse_markers(#[case] input: &str) {
    let rule = if input == "variation" {
        Rule::variation_token
    } else {
        Rule::individual_marker
    };
    let result = SysMLParser::parse(rule, input);
    assert!(
        result.is_ok(),
        "Failed to parse {} marker: {:?}",
        input,
        result.err()
    );
}

// Model Entry Point Tests

#[rstest]
#[case("", "empty model")]
#[case("package MyPackage;", "model with simple package")]
#[case("package MyPackage { }", "model with package body")]
#[case("library package MyLibrary;", "model with library package")]
#[case(
    "standard library package MyLibrary;",
    "model with standard library package"
)]
#[case("package Pkg1; package Pkg2;", "model with multiple packages")]
#[case(
    "package MyPackage { part myPart; }",
    "model with package containing usage"
)]
fn test_parse_model(#[case] input: &str, #[case] description: &str) {
    let result = SysMLParser::parse(Rule::file, input);
    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        description,
        result.err()
    );
}

// Dependency Tests

#[rstest]
#[case("dependency from A to B;", "simple")]
#[case("dependency A to B;", "without from")]
#[case("dependency 'Service Layer' to 'Data Layer' { }", "with body")]
#[case("dependency from A, B, C to D;", "multiple clients")]
#[case("dependency from A to B, C, D;", "multiple suppliers")]
#[case("dependency myDep from A to B;", "with identification")]
#[case(
    "dependency from A to B { comment MyComment; }",
    "with comment in body"
)]
fn test_parse_dependency(#[case] input: &str, #[case] description: &str) {
    let result = SysMLParser::parse(Rule::dependency, input);
    assert!(
        result.is_ok(),
        "Failed to parse dependency ({}): {:?}",
        description,
        result.err()
    );
}

// Annotation Tests

#[test]
fn test_parse_comment() {
    let input = "comment MyComment about MyElement;";
    let result = SysMLParser::parse(Rule::comment_annotation, input);

    assert!(
        result.is_ok(),
        "Failed to parse comment: {:?}",
        result.err()
    );
}

#[rstest]
#[case(r#"comment locale "en-US" /* comment text */"#)]
#[case(r#"comment MyComment locale "fr-FR" /* texte */"#)]
#[case(r#"comment about Foo;"#)]
#[case(r#"comment about Foo, Bar;"#)]
#[case(r#"comment MyComment about Foo, Bar /* about multiple */"#)]
#[case(r#"comment locale "en-US" about Foo;"#)]
fn test_parse_comment_variants(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::comment_annotation, input);
    assert!(
        result.is_ok(),
        "Failed to parse comment '{}': {:?}",
        input,
        result.err()
    );
}

#[test]
fn test_parse_documentation() {
    let input = "doc MyDoc;";
    let result = SysMLParser::parse(Rule::documentation, input);

    assert!(
        result.is_ok(),
        "Failed to parse documentation: {:?}",
        result.err()
    );
}

#[rstest]
#[case(r#"doc locale "en-US" /* docs */"#)]
#[case(r#"doc MyDoc locale "ja-JP" /* text */"#)]
#[case(r#"doc /* inline doc */"#)]
#[case(r#"doc;"#)]
fn test_parse_documentation_variants(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::documentation, input);
    assert!(
        result.is_ok(),
        "Failed to parse documentation '{}': {:?}",
        input,
        result.err()
    );
}

#[rstest]
#[case(r#"locale "en_US" /* comment */"#, "locale with comment")]
#[case(r#"locale "ja-JP""#, "locale without comment")]
fn test_parse_locale_annotation(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::locale_annotation, input);
    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[test]
fn test_parse_textual_representation() {
    let input = "rep language \"Python\" /* code */";
    let result = SysMLParser::parse(Rule::textual_representation, input);

    assert!(
        result.is_ok(),
        "Failed to parse textual representation: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_metadata_usage() {
    let input = "#MyMetadata;";
    let result = SysMLParser::parse(Rule::metadata_usage_annotation, input);

    assert!(
        result.is_ok(),
        "Failed to parse metadata usage: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_annotating_element() {
    let input = "comment MyComment;";
    let result = SysMLParser::parse(Rule::annotating_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse annotating element: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_relationship_body_with_annotation() {
    let input = "{ comment MyComment; }";
    let result = SysMLParser::parse(Rule::relationship_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse relationship body with annotation: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_dependency_with_comment_in_body() {
    let input = "dependency from A to B { comment MyComment; }";
    let result = SysMLParser::parse(Rule::dependency, input);

    assert!(
        result.is_ok(),
        "Failed to parse dependency with comment in body: {:?}",
        result.err()
    );
}

// Metadata Tests

#[rstest]
#[case(
    "metadata def MyMetadata;",
    Rule::metadata_definition,
    "simple metadata definition"
)]
#[case(
    "abstract metadata def MyMetadata;",
    Rule::metadata_definition,
    "abstract metadata definition"
)]
#[case("#MyMetadata", Rule::prefix_metadata_usage, "prefix metadata usage")]
#[case("metadata MyMetadata;", Rule::metadata_usage, "simple metadata usage")]
#[case("@MyMetadata;", Rule::metadata_usage, "metadata usage with @")]
#[case(
    "metadata MyMetadata about A, B;",
    Rule::metadata_usage,
    "metadata usage with about"
)]
#[case(
    "metadata myMeta : MyMetadata;",
    Rule::metadata_usage,
    "metadata usage with defined by"
)]
#[case(
    "metadata MyMetadata { }",
    Rule::metadata_usage,
    "metadata usage with body"
)]
#[case(
    "ref :>> MyReference;",
    Rule::metadata_body_usage,
    "metadata body usage"
)]
#[case(
    "metadata InterfaceCompatibilityIssue : Issue about engineToTransmissionInterface { text = \"issue\"; }",
    Rule::metadata_usage,
    "metadata usage with name, typing, about, and body"
)]
fn test_parse_metadata(#[case] input: &str, #[case] rule: Rule, #[case] desc: &str) {
    let result = SysMLParser::parse(rule, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Package Tests

#[rstest]
#[case("package MyPackage;", Rule::package, "simple package")]
#[case("package MyPackage { }", Rule::package, "package with body")]
#[case("package;", Rule::package, "package without name")]
#[case("library package MyLibrary;", Rule::library_package, "library package")]
#[case(
    "standard library package MyLibrary;",
    Rule::library_package,
    "standard library package"
)]
fn test_parse_package(#[case] input: &str, #[case] rule: Rule, #[case] desc: &str) {
    let result = SysMLParser::parse(rule, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[test]
fn test_parse_cases_sysml_fragment() {
    // This is the exact fragment from Cases.sysml that fails to parse
    let input = r#"abstract case def Case {
                subject subj : Anything[1] { }
                objective obj : RequirementCheck[1] {
                        subject subj default Case::result;
                }
        }"#;

    let result = SysMLParser::parse(Rule::case_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse Cases.sysml fragment: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_simplified_case_with_objective() {
    // Simplified version without the first subject
    let input = r#"case def Case {
    objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }
}"#;

    let result = SysMLParser::parse(Rule::case_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse simplified case: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_requirement_body_with_subject() {
    // Test just the requirement_body portion
    let input = r#"{
        subject subj default Case::result;
    }"#;

    let result = SysMLParser::parse(Rule::requirement_body, input);
    assert!(
        result.is_ok(),
        "Failed to parse requirement_body with subject: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_objective_member_in_case_body() {
    // Test objective_member as it would appear in a case body
    let input = r#"objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }"#;
    let result = SysMLParser::parse(Rule::objective_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse objective_member: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_case_body_with_objective() {
    // Test case_body directly
    let input = r#"{
    objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }
}"#;
    let result = SysMLParser::parse(Rule::case_body, input);
    assert!(
        result.is_ok(),
        "Failed to parse case_body: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_objective_as_case_body_item() {
    // Test if objective can be parsed as a case_body_item
    let input = r#"objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }"#;
    let result = SysMLParser::parse(Rule::case_body_item, input);
    assert!(
        result.is_ok(),
        "Failed to parse objective as case_body_item: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_minimal_case_body() {
    // Minimal test - no whitespace issues
    let input = "{objective obj{subject subj;}}";
    let result = SysMLParser::parse(Rule::case_body, input);
    assert!(
        result.is_ok(),
        "Failed to parse minimal case_body: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_ref_state_usage() {
    // Test parsing "ref state" which appears in Parts.sysml
    let input = "ref state myState;";

    let result = SysMLParser::parse(Rule::state_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse ref state usage: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_abstract_ref_state_usage() {
    // Test parsing "abstract ref state" which appears in Parts.sysml
    let input = "abstract ref state exhibitedStates: StateAction[0..*] { }";

    let result = SysMLParser::parse(Rule::state_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse abstract ref state usage: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_ref_state_as_definition_body_item() {
    // Test if ref state can be parsed as a definition_body_item
    let input = "abstract ref state exhibitedStates: StateAction[0..*] { }";

    let result = SysMLParser::parse(Rule::definition_body_item, input);
    assert!(
        result.is_ok(),
        "Failed to parse ref state as definition_body_item: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_state_with_doc_comment() {
    // Test state with doc comment - this is what fails in Parts.sysml
    let input = r#"abstract ref state exhibitedStates: StateAction[0..*] {
        doc
        /*
         * StateActions that are exhibited by this Part.
         */
    }"#;

    let result = SysMLParser::parse(Rule::state_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse state with doc comment: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_constraint_with_doc_comment() {
    // Test constraint with doc comment - this is what fails in Items.sysml
    let input = r#"assert constraint {
        doc
        /*
         * Test constraint
         */
        innerSpaceDimension == value
    }"#;

    let result = SysMLParser::parse(Rule::assert_constraint_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse constraint with doc comment: {:?}",
        result.err()
    );
}

// Member Tests

#[rstest]
#[case("part myPart;", Rule::usage_member, "usage member")]
#[case(
    "public part myPart;",
    Rule::usage_member,
    "usage member with visibility"
)]
#[case(
    "filter myExpression;",
    Rule::element_filter_member,
    "element filter member"
)]
#[case(
    "alias MyAlias for MyElement;",
    Rule::alias_member_element,
    "alias member"
)]
#[case(
    "private alias MyAlias for MyElement;",
    Rule::alias_member_element,
    "alias member with visibility"
)]
fn test_parse_member(#[case] input: &str, #[case] rule: Rule, #[case] desc: &str) {
    let result = SysMLParser::parse(rule, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Import Tests

#[rstest]
#[case("import MyElement;", "simple import")]
#[case("public import MyElement;", "import with visibility")]
#[case("import all MyElement;", "import all")]
#[case("import MyElement::*;", "import namespace")]
#[case("import MyElement::*::**;", "import recursive")]
#[case("import MyElement [condition];", "import with filter")]
#[case("import MyElement [filter1][filter2];", "import with multiple filters")]
#[case("import MyElement { }", "import with body")]
fn test_parse_import(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::import, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Definition Element Tests

#[rstest]
#[case("attribute def MyAttribute;", "attribute definition")]
#[case("enum def MyEnum;", "enumeration definition")]
#[case("occurrence def MyOccurrence;", "occurrence definition")]
#[case("individual def MyIndividual;", "individual definition")]
#[case("item def MyItem;", "item definition")]
#[case("part def MyPart;", "part definition")]
#[case("connection def MyConnection;", "connection definition")]
// Note: "flow connection def" removed - not part of official SysML v2 grammar
// FlowConnectionDefinition is an internal abstract syntax element
#[case("interface def MyInterface;", "interface definition")]
#[case("allocation def MyAllocation;", "allocation definition")]
#[case("port def MyPort;", "port definition")]
#[case("action def MyAction;", "action definition")]
#[case("calc def MyCalc;", "calculation definition")]
#[case("state def MyState;", "state definition")]
#[case("constraint def MyConstraint;", "constraint definition")]
#[case("requirement def MyRequirement;", "requirement definition")]
#[case("concern def MyConcern;", "concern definition")]
#[case("case def MyCase;", "case definition")]
#[case("analysis case def MyAnalysisCase;", "analysis case definition")]
#[case(
    "verification case def MyVerificationCase;",
    "verification case definition"
)]
#[case("use case def MyUseCase;", "use case definition")]
#[case("view def MyView;", "view definition")]
#[case("viewpoint def MyViewpoint;", "viewpoint definition")]
#[case("rendering def MyRendering;", "rendering definition")]
fn test_parse_definition_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::definition_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Usage Element Tests

#[rstest]
#[case("attribute myAttr;", "attribute usage")]
#[case("part myPart;", "part usage")]
fn test_parse_usage_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::usage_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Classifier Tests

#[rstest]
#[case("specializes", "specializes keyword")]
fn test_parse_specializes_token(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::specializes_token, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(":>", "specializes symbol")]
#[case("specializes", "specializes keyword")]
fn test_parse_specializes_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::specializes_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("BaseClass", "simple classifier reference")]
#[case("'Quoted Classifier'", "quoted classifier reference")]
fn test_parse_classifier_reference(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::classifier_reference, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("BaseClass", "single subclassification")]
fn test_parse_owned_subclassification(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_subclassification, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("specializes Base", "single base")]
#[case(":> Base", "single base with symbol")]
#[case("specializes Base1, Base2", "multiple bases")]
#[case(":> Base1, Base2, Base3", "multiple bases with symbol")]
fn test_parse_subclassification_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::subclassification_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Feature Tests

#[rstest]
#[case(":", "colon")]
#[case("typed by", "typed by keyword")]
fn test_parse_typed_by_token(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::typed_by_token, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("subsets", "subsets keyword")]
fn test_parse_subsets_token(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::subsets_token, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(":>", "subsets symbol")]
#[case("subsets", "subsets keyword")]
fn test_parse_subsets_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::subsets_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("references", "references keyword")]
fn test_parse_references_token(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::references_token, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("::>", "references symbol")]
#[case("references", "references keyword")]
fn test_parse_references_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::references_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("redefines", "redefines keyword")]
fn test_parse_redefines_token(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::redefines_token, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(":>>", "redefines symbol")]
#[case("redefines", "redefines keyword")]
fn test_parse_redefines_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::redefines_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("[1]", "single bound")]
#[case("[0..*]", "range with star")]
#[case("[1..5]", "numeric range")]
#[case("[*]", "unbounded")]
fn test_parse_owned_multiplicity(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_multiplicity, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("ordered", "ordered")]
#[case("nonunique", "nonunique")]
#[case("ordered nonunique", "ordered nonunique")]
#[case("nonunique ordered", "nonunique ordered")]
fn test_parse_multiplicity_properties(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::multiplicity_properties, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("[1]", "multiplicity only")]
#[case("[1] ordered", "multiplicity with properties")]
#[case("ordered", "properties only")]
fn test_parse_multiplicity_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::multiplicity_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(": BaseType", "typed by")]
#[case(":> BaseFeature", "subsets")]
#[case("::> ReferencedFeature", "references")]
#[case(":>> RedefinedFeature", "redefines")]
#[case("crosses other.feature", "crosses")]
fn test_parse_feature_specialization(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_specialization, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(": BaseType", "single typing")]
#[case(": Type1 [1]", "typing with multiplicity")]
#[case("[0..*] ordered", "multiplicity with properties")]
#[case(": Type1 [1] :> Base", "typing, multiplicity, and subsetting")]
#[case(
    ": ShoppingCart[1] crosses selectedProduct.inCart",
    "typing, multiplicity, and crosses"
)]
fn test_parse_feature_specialization_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_specialization_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "cart: ShoppingCart[1] crosses selectedProduct.inCart",
    "identification with typing, multiplicity, and crosses"
)]
fn test_parse_feature_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "cart: ShoppingCart[1] crosses selectedProduct.inCart",
    "identification with typing, multiplicity, and crosses"
)]
fn test_parse_usage_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::usage_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );

    // Check how much was consumed
    let pair = result.clone().unwrap().next().unwrap();
    println!(
        "{}: consumed '{}' from input len {}",
        desc,
        pair.as_str(),
        input.len()
    );
}

#[rstest]
#[case("end", "end prefix")]
#[case("end item", "end followed by item - should only consume end")]
#[case(
    "end item cart: ShoppingCart[1] crosses selectedProduct.inCart;",
    "full end item usage"
)]
fn test_parse_occurrence_usage_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::occurrence_usage_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );

    // Check how much was consumed
    let pair = result.unwrap().next().unwrap();
    println!(
        "{}: consumed '{}' (expected to consume only prefix)",
        desc,
        pair.as_str()
    );
}

#[rstest]
#[case("myFeature", "simple feature reference")]
#[case("'Quoted Feature'", "quoted feature reference")]
fn test_parse_feature_reference(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_reference, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("a.b", "simple chain")]
#[case("a.b.c", "longer chain")]
#[case("vehicle.engine.cylinder", "descriptive chain")]
fn test_parse_owned_feature_chain(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_feature_chain, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("BaseFeature", "feature reference")]
#[case("a.b.c", "feature chain")]
fn test_parse_owned_subsetting(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_subsetting, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("RefFeature", "feature reference")]
#[case("parent.child", "feature chain")]
fn test_parse_owned_reference_subsetting(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_reference_subsetting, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("RedefinedFeature", "feature reference")]
#[case("base.feature", "feature chain")]
fn test_parse_owned_redefinition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_redefinition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Definition Structure Tests

#[rstest]
#[case("abstract", "abstract marker")]
#[case("variation", "variation marker")]
fn test_parse_basic_definition_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::basic_definition_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty prefix")]
#[case("abstract", "abstract only")]
#[case("variation", "variation only")]
#[case("#Meta", "with metadata")]
#[case("abstract #Meta", "abstract with metadata")]
fn test_parse_definition_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::definition_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("MyDef;", "simple declaration")]
#[case("MyDef { }", "declaration with body")]
#[case("MyDef :> Base;", "with subclassification")]
#[case("MyDef :> Base { }", "subclassification with body")]
fn test_parse_definition_suffix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::definition_suffix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("MyDef", "simple identification")]
#[case("MyDef :> Base", "with subclassification")]
#[case(":> Base", "subclassification only")]
fn test_parse_definition_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::definition_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon")]
#[case("{ }", "empty body")]
fn test_parse_definition_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::definition_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("part def MyPart;", "part definition")]
#[case("attribute def MyAttr;", "attribute definition")]
fn test_parse_definition_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::definition_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Usage Structure Tests

#[rstest]
#[case("constant", "constant")]
#[case("derived", "derived")]
fn test_parse_usage_modifiers(#[case] input: &str, #[case] desc: &str) {
    let constant_result = SysMLParser::parse(Rule::constant_token, input);
    let derived_result = SysMLParser::parse(Rule::derived_token, input);

    assert!(
        constant_result.is_ok() || derived_result.is_ok(),
        "Failed to parse {}: constant={:?}, derived={:?}",
        desc,
        constant_result.err(),
        derived_result.err()
    );
}

#[rstest]
#[case("in", "in direction")]
#[case("out", "out direction")]
#[case("inout", "inout direction")]
fn test_parse_feature_direction_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_direction_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// =============================================================================
// Issue #617: feature_direction_kind should not match prefix of other keywords
// =============================================================================

/// Tests that feature_direction_kind does NOT match when followed by alphanumeric chars
/// This prevents "in" from matching the start of "interface", "interaction", etc.
#[rstest]
#[case("interface")]
#[case("interaction")]
#[case("input")]
#[case("internal")]
#[case("inout_extended")]
#[case("output")]
#[case("outer")]
fn test_feature_direction_kind_rejects_prefixes(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::feature_direction_kind, input);
    // The parse should either fail or not consume the entire input
    if let Ok(pairs) = result {
        let parsed = pairs.as_str();
        assert_ne!(
            parsed, input,
            "'{input}' should not fully match as feature_direction_kind"
        );
    }
}

#[rstest]
#[case("", "empty")]
#[case("in", "with direction")]
#[case("abstract", "with abstract")]
#[case("derived", "with derived")]
#[case("constant", "with constant")]
// Per spec 8.2.2.6.2: FeatureDirection? derived? BasicDefinitionPrefix? constant?
#[case("derived constant", "derived constant (spec order)")]
#[case("in derived abstract constant", "all modifiers (spec order)")]
fn test_parse_ref_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::ref_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Issue #634: ref_prefix should NOT allow concatenated keywords without whitespace
#[rstest]
#[case("derivedconstant", "derivedconstant should not fully parse")]
#[case("abstractconstant", "abstractconstant should not fully parse")]
#[case("abstractderived", "abstractderived should not fully parse")]
fn test_ref_prefix_rejects_concatenated_keywords(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::ref_prefix, input);
    // The parse should either fail or not consume the entire input
    if let Ok(pairs) = result {
        let parsed = pairs.as_str();
        assert_ne!(
            parsed, input,
            "{desc}: '{input}' should not fully match as ref_prefix"
        );
    }
}

#[rstest]
#[case("", "without ref")]
#[case("ref", "with ref")]
#[case("in ref", "with direction and ref")]
#[case("constant ref", "with constant and ref")]
fn test_parse_basic_usage_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::basic_usage_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("= myValue", "assignment")]
#[case(":= myValue", "initial assignment")]
#[case("default myValue", "default without assignment")]
#[case("default = myValue", "default with assignment")]
#[case("default := myValue", "default with initial assignment")]
fn test_parse_feature_value(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("= value", "simple value part")]
#[case(":= initialValue", "initial value part")]
fn test_parse_value_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::value_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "simple body")]
#[case("{ }", "empty body")]
fn test_parse_usage_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::usage_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Reference Usage Tests

#[rstest]
#[case("ref myRef;", "simple reference")]
#[case("ref myRef { }", "reference with body")]
fn test_parse_reference_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::reference_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myDefault;", "simple default reference")]
#[case("end myEnd;", "end default reference")]
fn test_parse_default_reference_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::default_reference_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Body Element Tests

#[rstest]
#[case("attribute myAttr;", "attribute usage")]
#[case("ref myRef;", "reference usage")]
#[case("bind source = target;", "binding connector")]
fn test_parse_non_occurrence_usage_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::non_occurrence_usage_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("part myPart;", "part usage")]
#[case("item myItem;", "item usage")]
#[case("action myAction;", "action usage")]
fn test_parse_occurrence_usage_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::occurrence_usage_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("part myPart;", "part usage")]
#[case("item myItem;", "item usage")]
#[case("port myPort;", "port usage")]
#[case("connection myConn;", "connection usage")]
fn test_parse_structure_usage_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::structure_usage_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action myAction;", "action usage")]
#[case("calc myCalc;", "calculation usage")]
#[case("state myState;", "state usage")]
#[case("constraint myConstraint;", "constraint usage")]
fn test_parse_behavior_usage_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::behavior_usage_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Extended Definition and Usage Tests

#[rstest]
#[case("#meta def ExtendedDef;", "simple extended definition")]
#[case(
    "abstract #meta def ExtendedDef { }",
    "extended definition with prefix and body"
)]
#[case("#meta #meta2 def ExtendedDef :> Base;", "multiple extension keywords")]
fn test_parse_extended_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::extended_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("#meta extendedUsage;", "simple extended usage")]
#[case("ref #meta extendedUsage;", "extended usage with ref prefix")]
#[case("#meta #meta2 extendedUsage : Type;", "multiple extension keywords")]
fn test_parse_extended_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::extended_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Attribute Definition and Usage Tests

#[rstest]
#[case("attribute def Speed;", "simple attribute definition")]
#[case(
    "attribute def Speed :> Real;",
    "attribute definition with subclassification"
)]
#[case(
    "abstract attribute def Speed { }",
    "attribute definition with prefix and body"
)]
fn test_parse_attribute_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::attribute_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("attribute speed;", "simple attribute usage")]
#[case("attribute speed : Real;", "attribute usage with typing")]
#[case("ref attribute speed;", "attribute usage with ref prefix")]
fn test_parse_attribute_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Enumeration Definition and Usage Tests

#[rstest]
#[case("enum def Color;", "simple enumeration definition")]
#[case("enum def Color { }", "enumeration definition with empty body")]
#[case("#meta enum def Status { }", "enumeration with prefix metadata")]
fn test_parse_enumeration_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::enumeration_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "simple body")]
#[case("{ }", "empty body with braces")]
fn test_parse_enumeration_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::enumeration_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("red;", "simple enumerated value")]
#[case("public green;", "enumerated value with visibility")]
#[case("private blue;", "enumerated value with private visibility")]
fn test_parse_enumeration_usage_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::enumeration_usage_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("red;", "simple enumerated value")]
#[case("enum green;", "enumerated value with enum keyword")]
#[case("#meta blue;", "enumerated value with metadata")]
fn test_parse_enumerated_value(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::enumerated_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("enum status;", "simple enumeration usage")]
#[case("enum status : Status;", "enumeration usage with typing")]
#[case("ref enum myEnum;", "enumeration usage with ref prefix")]
fn test_parse_enumeration_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::enumeration_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Occurrence Definition and Individual Definition Tests

#[rstest]
#[case("occurrence def Occurrence1;", "simple occurrence definition")]
#[case("occurrence def Occurrence1 { }", "occurrence definition with body")]
#[case(
    "abstract occurrence def Occurrence1;",
    "occurrence definition with abstract prefix"
)]
#[case(
    "individual occurrence def UniqueOccurrence;",
    "occurrence definition with individual marker"
)]
fn test_parse_occurrence_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::occurrence_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("individual def Thing;", "simple individual definition")]
#[case("individual def Thing { }", "individual definition with body")]
#[case(
    "abstract individual def UniqueThing;",
    "individual definition with abstract prefix"
)]
fn test_parse_individual_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::individual_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("occurrence", "occurrence keyword")]
fn test_parse_occurrence_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::occurrence_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("occurrence def", "occurrence def keyword")]
fn test_parse_occurrence_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::occurrence_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Occurrence Usage Tests

#[rstest]
#[case("occurrence occ1;", "simple occurrence usage")]
#[case("occurrence occ1 { }", "occurrence usage with body")]
#[case(
    "ref individual occurrence uniqueOcc;",
    "occurrence usage with ref and individual marker"
)]
#[case("snapshot occurrence snap1;", "occurrence usage with portion kind")]
fn test_parse_occurrence_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::occurrence_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("ref individual thing;", "simple individual usage")]
#[case("ref individual thing { }", "individual usage with body")]
#[case("out individual thing : Type;", "individual usage with typing")]
fn test_parse_individual_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::individual_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("snapshot snap1;", "simple snapshot portion usage")]
#[case("timeslice slice1;", "simple timeslice portion usage")]
#[case("ref individual snapshot snap2;", "individual snapshot usage")]
#[case("snapshot snap3 { }", "portion usage with body")]
fn test_parse_portion_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::portion_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("event myEvent;", "simple event occurrence usage")]
#[case("event myEvent { }", "event occurrence usage with body")]
#[case("event myRef;", "event with owned reference subsetting")]
fn test_parse_event_occurrence_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::event_occurrence_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("occurrence", "occurrence usage keyword")]
fn test_parse_occurrence_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::occurrence_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Empty Succession Tests

#[rstest]
#[case("then", "simple empty succession")]
#[case("then [1]", "empty succession with multiplicity")]
#[case("then [0..*]", "empty succession with range multiplicity")]
fn test_parse_empty_succession(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::empty_succession, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty multiplicity source end")]
#[case("[1]", "multiplicity source end with multiplicity")]
#[case("[0..*]", "multiplicity source end with range")]
fn test_parse_multiplicity_source_end(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::multiplicity_source_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty target end")]
fn test_parse_empty_target_end(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::empty_target_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Item Definition and Usage Tests

#[rstest]
#[case("item def MyItem;", "simple item definition")]
#[case("item def MyItem { }", "item definition with body")]
#[case("abstract item def MyItem;", "item definition with abstract prefix")]
#[case(
    "individual item def UniqueItem;",
    "item definition with individual marker"
)]
fn test_parse_item_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::item_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("item myItem;", "simple item usage")]
#[case("item myItem { }", "item usage with body")]
#[case(
    "ref individual item uniqueItem;",
    "item usage with ref and individual marker"
)]
#[case("snapshot item snap1;", "item usage with portion kind")]
#[case(
    "end item cart: ShoppingCart[1] crosses selectedProduct.inCart;",
    "end item with crosses"
)]
fn test_parse_item_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::item_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("item", "item keyword")]
fn test_parse_item_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::item_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("item def", "item def keyword")]
fn test_parse_item_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::item_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("item", "item usage keyword")]
fn test_parse_item_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::item_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Part Definition and Usage Tests

#[rstest]
#[case("part def MyPart;", "simple part definition")]
#[case("part def MyPart { }", "part definition with body")]
#[case("abstract part def MyPart;", "part definition with abstract prefix")]
#[case(
    "individual part def UniquePart;",
    "part definition with individual marker"
)]
fn test_parse_part_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::part_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("part myPart;", "simple part usage")]
#[case("part myPart { }", "part usage with body")]
#[case(
    "ref individual part uniquePart;",
    "part usage with ref and individual marker"
)]
#[case("snapshot part snap1;", "part usage with portion kind")]
fn test_parse_part_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::part_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("part", "part keyword")]
fn test_parse_part_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::part_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("part def", "part def keyword")]
fn test_parse_part_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::part_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("part", "part usage keyword")]
fn test_parse_part_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::part_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Port Usage Tests

#[rstest]
#[case("port myPort;", "simple port usage")]
#[case("port myPort { }", "port usage with body")]
#[case(
    "ref individual port uniquePort;",
    "port usage with ref and individual marker"
)]
#[case("snapshot port snap1;", "port usage with portion kind")]
fn test_parse_port_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::port_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("port", "port keyword")]
fn test_parse_port_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::port_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("port", "port usage keyword")]
fn test_parse_port_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::port_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Connector Tests

#[rstest]
#[case("myRef", "simple connector end")]
#[case("[1] myRef", "connector end with cross multiplicity")]
#[case("endName references myRef", "connector end with name and references")]
#[case("[0..*] endName references myRef", "connector end with all elements")]
fn test_parse_connector_end(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connector_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("[1]", "owned cross multiplicity")]
#[case("[0..*]", "owned cross multiplicity with range")]
fn test_parse_owned_cross_multiplicity(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_cross_multiplicity, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Binding Connector Tests

#[rstest]
#[case("bind source = target;", "simple binding connector")]
#[case("bind source = target { }", "binding connector with body")]
#[case(
    "binding myBinding bind source = target;",
    "binding connector with declaration"
)]
fn test_parse_binding_connector_as_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::binding_connector_as_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("binding", "binding keyword")]
fn test_parse_binding_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::binding_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Connection Definition Tests

#[rstest]
#[case("connection def MyConnection;", "simple connection definition")]
#[case("connection def MyConnection { }", "connection definition with body")]
#[case(
    "abstract connection def MyConnection;",
    "connection definition with abstract prefix"
)]
fn test_parse_connection_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connection_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("connection", "connection keyword")]
fn test_parse_connection_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connection_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("connection def", "connection def keyword")]
fn test_parse_connection_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connection_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Connection Usage Tests

#[rstest]
#[case("connection myConn;", "simple connection usage")]
#[case("connection myConn { }", "connection usage with body")]
#[case("connect source to target;", "connection usage with connector")]
#[case(
    "connection myConn connect source to target;",
    "connection usage with declaration and connector"
)]
#[case("connect (a, b, c);", "connection usage with nary connector")]
fn test_parse_connection_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connection_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("connect", "connector keyword")]
fn test_parse_connector_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connector_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("connection", "connection usage keyword")]
fn test_parse_connection_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connection_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Connector Part Tests

#[rstest]
#[case("source to target", "binary connector part")]
fn test_parse_binary_connector_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::binary_connector_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("(a, b)", "nary connector with two ends")]
#[case("(a, b, c)", "nary connector with three ends")]
#[case("(x, y, z, w)", "nary connector with four ends")]
fn test_parse_nary_connector_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::nary_connector_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty source end")]
fn test_parse_empty_source_end(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::empty_source_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "interface def MyInterface;",
    "simple interface definition with semicolon"
)]
#[case(
    "interface def Vehicle { port driver; }",
    "interface definition with port"
)]
#[case(
    "abstract interface def DataInterface { ref data : DataType; }",
    "abstract interface with reference usage"
)]
fn test_parse_interface_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("interface", "interface keyword")]
fn test_parse_interface_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("interface def", "interface def keyword")]
fn test_parse_interface_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon body")]
#[case("{ port driver; }", "body with port")]
fn test_parse_interface_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("ref data : DataType;", "reference usage")]
#[case("attribute speed : Real;", "attribute usage")]
fn test_parse_interface_non_occurrence_usage_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_non_occurrence_usage_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("end driver;", "default interface end")]
#[case("port sensor;", "port usage")]
fn test_parse_interface_occurrence_usage_element(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_occurrence_usage_element, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("end driver;", "end with usage declaration")]
#[case("end;", "end without declaration")]
fn test_parse_default_interface_end(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::default_interface_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("interface", "interface usage keyword")]
fn test_parse_interface_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("portA to portB", "binary interface part only")]
fn test_parse_interface_usage_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_usage_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Issue #617: Ensure 'in' from 'interface' isn't consumed as a prefix
#[rstest]
#[case("interface connect left to right;", "simple interface usage")]
#[case("interface connect left to right{}", "interface with empty body")]
#[case(
    "constant interface connect left to right;",
    "interface with constant prefix"
)]
#[case(
    "constant interface interfaceA connect left to right;",
    "named interface with prefix"
)]
#[case(
    "interface leftFrontMount: Mounting connect frontAxle.leftMountingPoint to leftFrontWheel.hub;",
    "interface with typing"
)]
fn test_parse_interface_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("portA to portB", "binary interface part")]
#[case("(portA, portB)", "nary interface part with two ports")]
#[case("(portA, portB, portC)", "nary interface part with three ports")]
fn test_parse_interface_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("portA to portB", "binary interface part")]
fn test_parse_binary_interface_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::binary_interface_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("(portA, portB)", "nary with two ports")]
#[case("(portA, portB, portC)", "nary with three ports")]
fn test_parse_nary_interface_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::nary_interface_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("portA", "simple interface end member")]
fn test_parse_interface_end_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_end_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("portA", "simple interface end")]
#[case("myPort references BasePort", "interface end with references")]
fn test_parse_interface_end(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("flow def DataFlow;", "simple flow definition with semicolon")]
#[case("flow def FluidFlow { }", "flow definition with body")]
#[case("abstract flow def AbstractFlow;", "abstract flow definition")]
fn test_parse_flow_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("flow", "flow keyword")]
fn test_parse_flow_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("flow def", "flow def keyword")]
fn test_parse_flow_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("message", "message keyword")]
fn test_parse_message_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::message_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("eventA to eventB", "message declaration with two events")]
fn test_parse_message_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::message_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("dataRef", "payload feature member")]
fn test_parse_payload_feature_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::payload_feature_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("dataRef", "payload feature")]
fn test_parse_payload_feature(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::payload_feature, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("eventRef", "message event member")]
fn test_parse_message_event_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::message_event_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("eventRef", "message event")]
fn test_parse_message_event(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::message_event, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("dataRef", "payload - identifier")]
#[case("S", "payload - bare type name (OwnedFeatureTyping)")]
#[case("msg : Type", "payload - with typing")]
#[case("msg = value", "payload - with value")]
#[case("[1..*] S", "payload - multiplicity then type")]
fn test_parse_payload(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::payload, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(": DataType [1]", "payload feature specialization with multiplicity")]
fn test_parse_payload_feature_specialization_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::payload_feature_specialization_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myFlow", "flow end member")]
fn test_parse_flow_end_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_end_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("flowRef", "flow end")]
fn test_parse_flow_end(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_end, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("flowRef", "flow feature member")]
fn test_parse_flow_feature_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_feature_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("flowRef", "flow feature")]
fn test_parse_flow_feature(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_feature, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("BaseFlow", "flow redefinition")]
fn test_parse_flow_redefinition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_redefinition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action def Move;", "simple action definition with semicolon")]
#[case("action def Calculate { }", "action definition with body")]
#[case("abstract action def AbstractAction;", "abstract action definition")]
fn test_parse_action_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action", "action keyword")]
fn test_parse_action_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action def", "action def keyword")]
fn test_parse_action_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon body")]
#[case("{ }", "empty brace body")]
fn test_parse_action_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("first startNode;", "initial node member")]
fn test_parse_initial_node_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::initial_node_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty member prefix")]
fn test_parse_member_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::member_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("send myMsg;", "send action node")]
#[case("accept myAccept;", "accept action node")]
#[case("assign x := y;", "assignment action node")]
fn test_parse_action_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("send msg;", "action node member with send")]
#[case("accept acc;", "action node member with accept")]
fn test_parse_action_node_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_node_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("then nextNode;", "simple target succession")]
#[case("if true then nextNode;", "guarded target succession")]
#[case("else defaultNode;", "default target succession")]
fn test_parse_action_target_succession(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_target_succession, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("then nextNode;", "target succession member with then")]
fn test_parse_target_succession_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::target_succession_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("if x then y;", "guarded target succession")]
fn test_parse_guarded_target_succession(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::guarded_target_succession, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("if x then y;", "guarded succession member")]
fn test_parse_guarded_succession_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::guarded_succession_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action", "action usage keyword")]
fn test_parse_action_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty action usage declaration")]
fn test_parse_action_usage_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_usage_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("perform c.incr;", "perform with feature chain")]
#[case("perform myAction;", "perform with reference")]
fn test_parse_perform_action_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::perform_action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("actionRef", "perform action usage declaration with reference")]
#[case(
    "action myAction;",
    "perform action usage declaration with action keyword"
)]
fn test_parse_perform_action_usage_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::perform_action_usage_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action", "action node usage declaration")]
fn test_parse_action_node_usage_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_node_usage_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty action node prefix")]
fn test_parse_action_node_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_node_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("accept msg", "accept node declaration with identifier")]
#[case("accept S", "accept node declaration with bare type name")]
#[case(
    "accept signal : SignalType",
    "accept node declaration with typed payload"
)]
fn test_parse_accept_node_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::accept_node_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("accept msg;", "accept node with identifier")]
#[case("accept S;", "accept node with bare type name")]
#[case("accept signal : SignalType;", "accept node with typed payload")]
fn test_parse_accept_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::accept_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Test action body with then accept patterns - this is the failing case from ActionTest.sysml
#[rstest]
#[case(
    "action a { first start; then accept S; }",
    "action with then accept bare type"
)]
#[case(
    "action a { first start; then merge m; then accept S; }",
    "action with merge then accept"
)]
#[case(
    r#"action a1 {
                first start;
                then merge m;
                then accept S;
        }"#,
    "action from ActionTest.sysml lines 14-18"
)]
fn test_parse_action_body_with_accept(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Test package with action containing then accept - isolate ActionTest.sysml issue
#[rstest]
#[case(
    r#"package ActionTest {
        attribute def S;
        action a1 {
                first start;
                then merge m;
                then accept S;
        }
}"#,
    "package with action from ActionTest.sysml - minimal"
)]
#[case(
    r#"package ActionTest {
        action def A{ in x; }
        action a: A { 
                first start;
                action b { in y = x; }
                bind x = b.y;
        }
        attribute def S;
        action a1 {
                first start;
                then merge m;
                then accept S;
        }
}"#,
    "package with action from ActionTest.sysml - with action def"
)]
fn test_parse_package_with_accept_action(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::package, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Test full action a1 from ActionTest.sysml - this is the exact failing case
#[test]
fn test_parse_action_a1_full() {
    let input = r#"action a1 {
        first start;		
        then merge m;
        then accept S;
        then accept sig after 10[SI::s]; 
        then accept at new Time::Iso8601DateTime("2022-01-30T01:00:00Z");
        
        then send new S() to b;
        then accept when b.f;
        then decide;
            if true then m;
            else done;
    }"#;

    let result = SysMLParser::parse(Rule::action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse full action a1: {:?}",
        result.err()
    );
}

// Isolate failing pattern: accept sig after ...
#[rstest]
#[case("accept sig after 10[SI::s];", "accept with sig after time")]
#[case("accept S after 10;", "accept with S after simple number")]
#[case("accept after 10;", "accept with after (no name)")]
fn test_parse_accept_node_with_after(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::accept_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Test just the trigger value part in isolation
#[rstest]
#[case("after 10", "after with simple number")]
#[case("at 5", "at with simple number")]
fn test_parse_trigger_value_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::trigger_value_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Test payload_parameter with trigger
#[rstest]
#[case("after 10", "trigger only (no name)")]
#[case("sig after 10", "name + trigger")]
fn test_parse_payload_parameter_with_trigger(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::payload_parameter, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Test accept_parameter_part with trigger
#[rstest]
#[case("after 10", "trigger only (no name)")]
#[case("sig after 10", "name + trigger")]
fn test_parse_accept_parameter_part_with_trigger(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::accept_parameter_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Test accept_node_declaration with trigger
#[rstest]
#[case("accept after 10", "accept + trigger only")]
#[case("accept sig after 10", "accept + name + trigger")]
fn test_parse_accept_node_declaration_with_trigger(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::accept_node_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("msg", "accept parameter part")]
#[case("msg via port", "accept parameter part with via")]
fn test_parse_accept_parameter_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::accept_parameter_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("data", "payload parameter - identifier")]
#[case("S", "payload parameter - bare type name")]
#[case("msg : MessageType", "payload parameter - typed")]
#[case(
    "msg : MessageType = defaultValue",
    "payload parameter - typed with value"
)]
fn test_parse_payload_parameter(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::payload_parameter, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("at timeValue", "time trigger expression")]
#[case("when condition", "change trigger expression")]
fn test_parse_trigger_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::trigger_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("at", "at trigger")]
#[case("after", "after trigger")]
fn test_parse_time_trigger_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::time_trigger_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("when", "when trigger")]
fn test_parse_change_trigger_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::change_trigger_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("arg", "argument member")]
fn test_parse_argument_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::argument_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("expr", "argument expression")]
fn test_parse_argument_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::argument_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("exprRef", "owned expression reference")]
fn test_parse_owned_expression_reference(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_expression_reference, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("param", "node parameter")]
fn test_parse_node_parameter(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::node_parameter, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("expr", "feature binding")]
fn test_parse_feature_binding(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("expr", "owned expression")]
#[case("v.m", "feature chain expression")]
#[case("a.b.c", "longer feature chain")]
fn test_parse_owned_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("send msg;", "send node declaration")]
fn test_parse_send_node_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::send_node_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("via port1", "sender receiver part with via")]
#[case("via port1 to port2", "sender receiver part with via and to")]
fn test_parse_sender_receiver_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::sender_receiver_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("assign x:= y", "assignment node declaration")]
fn test_parse_assignment_node_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::assignment_node_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("target", "assignment target member")]
fn test_parse_assignment_target_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::assignment_target_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("feature", "target parameter without binding")]
#[case("binding.feature", "target parameter with binding")]
fn test_parse_target_parameter(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::target_parameter, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("target", "target binding with identifier")]
#[case("source.property", "target binding with feature chain")]
fn test_parse_target_binding(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::target_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("chain", "feature chain member")]
fn test_parse_feature_chain_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::feature_chain_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("expr", "base expression")]
fn test_parse_base_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::base_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("expr", "simple target expression")]
#[case("expr.member", "target expression with chain")]
#[case("expr[index]", "target expression with index")]
fn test_parse_target_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::target_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("seq", "sequence expression")]
fn test_parse_sequence_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::sequence_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("TypeRef", "reference typing")]
fn test_parse_reference_typing(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::reference_typing, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("{;}", "expression body member with empty body")]
fn test_parse_expression_body_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::expression_body_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("funcRef", "function reference member")]
fn test_parse_function_reference_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::function_reference_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("()", "empty argument list")]
#[case("(arg)", "argument list with one positional arg")]
#[case("(arg1, arg2)", "argument list with multiple positional args")]
#[case("(arg1, arg2, arg3)", "argument list with three positional args")]
#[case("(param1 = value1)", "argument list with one named arg")]
#[case(
    "(param1 = value1, param2 = value2)",
    "argument list with multiple named args"
)]
fn test_parse_argument_list(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::argument_list, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("arg1, arg2", "positional argument list")]
#[case("x, y, z", "positional argument list with three args")]
fn test_parse_positional_argument_list(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::positional_argument_list, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "argument")]
#[case("expression", "argument with expression")]
fn test_parse_argument(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::argument, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("param1 = value1, param2 = value2", "named argument list")]
#[case("x = a, y = b", "named argument list with two args")]
fn test_parse_named_argument_list(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::named_argument_list, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("param = value", "named argument member")]
#[case("x = y", "named argument member with simple assignment")]
fn test_parse_named_argument_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::named_argument_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("param = value", "named argument")]
#[case("x = expression", "named argument with expression")]
fn test_parse_named_argument(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::named_argument, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("param", "parameter redefinition")]
#[case("featureRef", "parameter redefinition with feature ref")]
fn test_parse_parameter_redefinition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::parameter_redefinition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "argument value")]
#[case("123", "argument value with number")]
#[case("\"string\"", "argument value with string")]
fn test_parse_argument_value(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::argument_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("MyType()", "invocation expression with empty args")]
#[case("MyType(arg)", "invocation expression with one arg")]
#[case("MyType(arg1, arg2)", "invocation expression with multiple args")]
#[case("MyType(param = value)", "invocation expression with named arg")]
fn test_parse_invocation_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::invocation_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("feature", "target feature")]
fn test_parse_target_feature(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::target_feature, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("accessed", "target accessed feature member")]
fn test_parse_target_accessed_feature_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::target_accessed_feature_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty usage")]
fn test_parse_empty_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::empty_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("expr", "expression parameter member")]
fn test_parse_expression_parameter_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::expression_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("{}", "empty action body parameter")]
fn test_parse_action_body_parameter(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_body_parameter, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("{}", "action body parameter member")]
fn test_parse_action_body_parameter_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_body_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("varName", "for variable declaration")]
fn test_parse_for_variable_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::for_variable_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "control node prefix")]
fn test_parse_control_node_prefix(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::control_node_prefix, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("if condition {}", "if with condition")]
#[case("if x {}", "if with simple condition")]
#[case("if x {} else {}", "if with else")]
fn test_parse_if_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::if_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("if condition {}", "if node parameter member")]
fn test_parse_if_node_parameter_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::if_node_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("while condition {}", "while loop")]
#[case("loop {}", "unconditional loop")]
#[case("while x {}", "while with simple condition")]
#[case("loop {} until result;", "loop with until")]
fn test_parse_while_loop_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::while_loop_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty parameter member")]
fn test_parse_empty_parameter_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::empty_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("for x in items {}", "for with simple vars")]
fn test_parse_for_loop_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::for_loop_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("terminate;", "terminate node")]
fn test_parse_terminate_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::terminate_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("merge;", "control node with merge")]
#[case("decide;", "control node with decide")]
#[case("join;", "control node with join")]
#[case("fork;", "control node with fork")]
fn test_parse_control_node(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::control_node, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// State Definition Tests

#[rstest]
#[case("state", "state keyword")]
fn test_parse_state_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("state def", "state def keyword")]
fn test_parse_state_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("state def MyState;", "simple state definition")]
#[case("state def MyState {}", "state definition with empty body")]
#[case("state def MyState parallel {}", "parallel state definition")]
fn test_parse_state_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon state def body")]
#[case("{}", "empty braces state def body")]
#[case("parallel {}", "parallel state def body")]
fn test_parse_state_def_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_def_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("entry", "entry action kind")]
fn test_parse_entry_action_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::entry_action_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("do", "do action kind")]
fn test_parse_do_action_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::do_action_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("exit", "exit action kind")]
fn test_parse_exit_action_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::exit_action_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action entryAction;", "full form with action keyword")]
#[case("action doAction: Action;", "typed state action usage")]
#[case("action exercise : Exercise { }", "state action with body")]
#[case(";", "empty action shorthand")]
#[case("monitorTemperature;", "reference subsetting shorthand")]
fn test_parse_state_action_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty action usage")]
fn test_parse_empty_action_usage_state(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::empty_action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("entry action entryAction;", "full form with action keyword")]
#[case("entry action warmup : WarmUp;", "typed entry action")]
#[case(
    "entry action entryAction :>> 'entry';",
    "entry action with redefinition"
)]
#[case("entry;", "empty entry action shorthand")]
#[case("entry setupSensor;", "entry reference subsetting shorthand")]
fn test_parse_entry_action_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::entry_action_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("do action doAction;", "full form with action keyword")]
#[case("do action exercise : Exercise;", "typed do action")]
#[case("do action doAction: Action :>> 'do';", "do action with redefinition")]
#[case("do action exercise : Exercise { }", "do action with body")]
#[case("do;", "empty do action shorthand")]
#[case("do monitorTemperature;", "do reference subsetting shorthand")]
fn test_parse_do_action_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::do_action_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("exit action exitAction;", "full form with action keyword")]
#[case("exit action cooldown : Cooldown;", "typed exit action")]
#[case(
    "exit action exitAction: Action :>> 'exit';",
    "exit action with redefinition"
)]
#[case("exit;", "empty exit action shorthand")]
#[case("exit cleanup;", "exit reference subsetting shorthand")]
fn test_parse_exit_action_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::exit_action_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// State Usage Tests

#[rstest]
#[case("state", "state usage keyword")]
fn test_parse_state_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("state;", "simple state usage")]
#[case("state {}", "state usage with empty body")]
#[case("state parallel {}", "parallel state usage")]
fn test_parse_state_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon state usage body")]
#[case("{}", "empty braces state usage body")]
#[case("parallel {}", "parallel state usage body")]
fn test_parse_state_usage_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_usage_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("exhibit state;", "simple exhibit state usage")]
#[case("exhibit myRef;", "exhibit with reference")]
#[case("exhibit state MyState;", "exhibit with state and identifier")]
fn test_parse_exhibit_state_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::exhibit_state_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Transition Usage Tests

#[rstest]
#[case("transition", "transition usage keyword")]
fn test_parse_transition_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::transition_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("accept", "trigger feature kind")]
fn test_parse_trigger_feature_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::trigger_feature_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("if", "guard feature kind")]
fn test_parse_guard_feature_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::guard_feature_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("do", "effect feature kind")]
fn test_parse_effect_feature_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::effect_feature_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("sourceRef", "transition source member with reference")]
fn test_parse_transition_source_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::transition_source_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("msg", "trigger action with type")]
#[case("msg via msg2", "trigger action with other msg")]
fn test_parse_trigger_action(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::trigger_action, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("accept msg", "trigger action member")]
fn test_parse_trigger_action_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::trigger_action_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("if condition", "guard expression member")]
fn test_parse_guard_expression_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::guard_expression_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("do {}", "effect behavior member with empty action")]
fn test_parse_effect_behavior_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::effect_behavior_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty effect behavior usage")]
fn test_parse_effect_behavior_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::effect_behavior_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("target", "transition succession")]
fn test_parse_transition_succession(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::transition_succession, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("target", "transition succession member")]
fn test_parse_transition_succession_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::transition_succession_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Full Transition Usage Tests

#[rstest]
#[case("transition first S1 then S2;", "anonymous transition")]
#[case("transition T first S1 then S2;", "named transition with first")]
#[case(
    "transition\n    first S1\n    then S2;",
    "anonymous transition with first on new line"
)]
#[case(
    "transition T\n    first S2.S3\n    accept s : Sig via p\n    if true\n    do send s to p\n    then S1;",
    "named transition with first on new line"
)]
fn test_parse_transition_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::transition_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Allocate Usage Tests

#[rstest]
#[case("allocate a to b;", "simple allocate")]
#[case(
    "allocate l.component to assembly.element;",
    "allocate with feature chains"
)]
fn test_parse_allocate_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::allocate_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Allocation Usage Tests (full form with allocation keyword)

#[rstest]
#[case("allocation myAlloc;", "simple allocation usage")]
#[case(
    "allocation allocation1 : Logical_to_Physical allocate l to p;",
    "allocation with name, type and allocate"
)]
#[case(
    "allocation allocation2 : Logical_to_Physical allocate (\n    logical ::> l,\n    physical ::> p\n);",
    "allocation with nary allocate"
)]
fn test_parse_allocation_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::allocation_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Calculation Definition Tests

#[rstest]
#[case("calc", "calculation keyword")]
fn test_parse_calculation_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("calc def", "calculation def keyword")]
fn test_parse_calculation_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("calc def MyCalc;", "simple calculation definition")]
#[case("calc def MyCalc {}", "calculation definition with empty body")]
#[case(
    "calc def Increment { in c : Counter; return : Counter; perform c.incr; c }",
    "full calculation definition"
)]
fn test_parse_calculation_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon calculation body")]
#[case("{}", "empty braces calculation body")]
#[case("{ perform c.incr; c }", "calculation body with perform and result")]
#[case(
    "{ in c : Counter; return : Counter; perform c.incr; c }",
    "full calculation body"
)]
fn test_parse_calculation_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty calculation body part")]
fn test_parse_calculation_body_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_body_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("return myValue;", "return parameter member")]
fn test_parse_return_parameter_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("result", "result expression member")]
#[case("v.m", "simple expression")]
#[case("a.b.c", "chained expression")]
fn test_parse_result_expression_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::result_expression_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Calculation Usage Tests

#[rstest]
#[case("calc", "calculation usage keyword")]
fn test_parse_calculation_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("calc;", "simple calculation usage")]
#[case("calc {}", "calculation usage with body")]
fn test_parse_calculation_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Constraint Definition Tests

#[rstest]
#[case("constraint", "constraint keyword")]
fn test_parse_constraint_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::constraint_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("constraint def", "constraint def keyword")]
fn test_parse_constraint_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::constraint_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("constraint def MyConstraint;", "simple constraint definition")]
#[case("constraint def MyConstraint {}", "constraint definition with body")]
fn test_parse_constraint_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::constraint_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Constraint Usage Tests

#[rstest]
#[case("constraint", "constraint usage keyword")]
fn test_parse_constraint_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::constraint_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("constraint;", "simple constraint usage")]
#[case("constraint {}", "constraint usage with body")]
fn test_parse_constraint_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::constraint_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("", "empty constraint usage declaration")]
fn test_parse_constraint_usage_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::constraint_usage_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("assert myRef;", "assert constraint usage with reference")]
#[case("assert not myRef;", "assert constraint usage with negation")]
#[case("assert constraint;", "assert constraint usage with keyword")]
fn test_parse_assert_constraint_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::assert_constraint_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Requirement Definition Tests

#[rstest]
#[case("requirement", "requirement keyword")]
fn test_parse_requirement_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("requirement def", "requirement def keyword")]
fn test_parse_requirement_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "requirement def SafetyRequirement;",
    "requirement definition with semicolon"
)]
#[case(
    "requirement def SafetyRequirement {}",
    "requirement definition with empty body"
)]
#[case(
    "requirement def SafetyRequirement { /* requirement doc */ }",
    "requirement definition with doc comment"
)]
fn test_parse_requirement_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon body")]
#[case("{}", "empty body")]
fn test_parse_requirement_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("subject mySubject;", "subject usage")]
#[case(
    "subject subj default Case::result;",
    "subject usage with default qualified value"
)]
#[case(
    "subject subj default myValue;",
    "subject usage with default simple value"
)]
#[case("subject subj : MyType;", "subject usage with typing")]
#[case(
    "subject subj : MyType default myValue;",
    "subject usage with typing and default"
)]
fn test_parse_subject_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::subject_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("subject mySubject;", "subject member")]
#[case(
    "subject subj default Case::result;",
    "subject member with default qualified value"
)]
fn test_parse_subject_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::subject_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myConstraint {}", "requirement constraint usage with reference")]
#[case("constraint {}", "requirement constraint usage with keyword")]
fn test_parse_requirement_constraint_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_constraint_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("assume myConstraint {}", "assume requirement constraint member")]
#[case("require constraint {}", "require requirement constraint member")]
fn test_parse_requirement_constraint_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_constraint_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("frame", "frame keyword")]
fn test_parse_framed_concern_kind(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::framed_concern_kind, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("concern", "concern usage keyword")]
fn test_parse_concern_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::concern_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myConcern;", "framed concern usage with reference")]
#[case("concern {}", "framed concern usage with keyword")]
fn test_parse_framed_concern_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::framed_concern_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("frame myConcern;", "framed concern member")]
fn test_parse_framed_concern_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::framed_concern_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("actor myActor;", "actor usage")]
fn test_parse_actor_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::actor_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("actor myActor;", "actor member")]
fn test_parse_actor_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::actor_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("stakeholder myStakeholder;", "stakeholder usage")]
fn test_parse_stakeholder_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::stakeholder_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("stakeholder myStakeholder;", "stakeholder member")]
fn test_parse_stakeholder_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::stakeholder_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myVerification;", "requirement verification usage with reference")]
#[case("requirement myReq {}", "requirement verification usage with keyword")]
fn test_parse_requirement_verification_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_verification_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("verify myVerification;", "requirement verification member")]
fn test_parse_requirement_verification_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_verification_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("concern myConcern {}", "concern usage")]
fn test_parse_concern_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::concern_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Concern Definition Tests

#[rstest]
#[case("concern", "concern keyword")]
fn test_parse_concern_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::concern_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("concern def", "concern def keyword")]
fn test_parse_concern_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::concern_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("concern def PerformanceConcern;", "concern definition with semicolon")]
#[case(
    "concern def PerformanceConcern {}",
    "concern definition with empty body"
)]
#[case(
    "concern def BrakingConcern { require constraint { /**/ } }",
    "concern definition with requirement constraint and doc"
)]
#[case(
    "concern def SafetyConcern { subject vehicle; stakeholder driver; }",
    "concern definition with subject and stakeholder"
)]
fn test_parse_concern_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::concern_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Case Definition Tests

#[rstest]
#[case("case", "case keyword")]
fn test_parse_case_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::case_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("case def", "case def keyword")]
fn test_parse_case_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::case_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("case def TestCase;", "case definition with semicolon")]
#[case("case def TestCase {}", "case definition with empty body")]
#[case(
    "case def TestCase { subject testSubject; }",
    "case definition with subject"
)]
fn test_parse_case_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::case_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("v;", "identifier with semicolon")]
#[case("v {}", "identifier with empty body")]
fn test_parse_case_body_item(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::case_body_item, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[test]
fn test_case_body_item_does_not_match_expression() {
    // v.m should NOT match case_body_item because it doesn't end with ; or {}
    let result = SysMLParser::parse(Rule::case_body_item, "v.m");
    if let Ok(pairs) = &result {
        for pair in pairs.clone() {
            println!("Matched: {:?} = '{}'", pair.as_rule(), pair.as_str());
            for inner in pair.into_inner() {
                println!("  Inner: {:?} = '{}'", inner.as_rule(), inner.as_str());
            }
        }
    }
    assert!(
        result.is_err(),
        "case_body_item should NOT match v.m, but got: {result:?}"
    );
}

#[rstest]
#[case(";", "semicolon body")]
#[case("{}", "empty body")]
#[case("{ v.m }", "body with result expression")]
#[case("{ subject v : V; v.m }", "body with subject and result expression")]
fn test_parse_case_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::case_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myObjective {}", "objective requirement usage with declaration")]
#[case("{}", "objective requirement usage with empty body")]
#[case(
    "obj : RequirementCheck[1] { subject subj default Case::result; }",
    "objective with subject and default value"
)]
fn test_parse_objective_requirement_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::objective_requirement_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("objective myObjective {}", "objective member")]
#[case(
    "objective obj : RequirementCheck[1] { subject subj default Case::result; }",
    "objective member with subject and default value"
)]
fn test_parse_objective_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::objective_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Case Usage Tests

#[rstest]
#[case("case", "case usage keyword")]
fn test_parse_case_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::case_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("case testCase;", "case usage with semicolon")]
#[case("case testCase {}", "case usage with empty body")]
#[case("case testCase { subject testSubject; }", "case usage with subject")]
fn test_parse_case_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::case_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Analysis Case Tests

#[rstest]
#[case("analysis", "analysis keyword")]
fn test_parse_analysis_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::analysis_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "analysis case def AnalysisTest;",
    "analysis case definition with semicolon"
)]
#[case(
    "analysis case def AnalysisTest {}",
    "analysis case definition with empty body"
)]
#[case(
    "analysis case def AnalysisTest { subject testSubject; }",
    "analysis case definition with subject"
)]
fn test_parse_analysis_case_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::analysis_case_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("analysis testAnalysis;", "analysis case usage with semicolon")]
#[case("analysis testAnalysis {}", "analysis case usage with empty body")]
#[case(
    "analysis testAnalysis { actor analyst; }",
    "analysis case usage with actor"
)]
fn test_parse_analysis_case_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::analysis_case_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Verification Case Tests

#[rstest]
#[case("verification", "verification keyword")]
fn test_parse_verification_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::verification_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "verification case def VerifyTest;",
    "verification case definition with semicolon"
)]
#[case(
    "verification case def VerifyTest {}",
    "verification case definition with empty body"
)]
#[case(
    "verification case def VerifyTest { objective myObj {} }",
    "verification case definition with objective"
)]
fn test_parse_verification_case_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::verification_case_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "verification testVerification;",
    "verification case usage with semicolon"
)]
#[case(
    "verification testVerification {}",
    "verification case usage with empty body"
)]
fn test_parse_verification_case_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::verification_case_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Use Case Tests

#[rstest]
#[case("use case def", "use case def keyword")]
fn test_parse_use_case_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::use_case_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("use case def TestUseCase;", "use case definition with semicolon")]
#[case("use case def TestUseCase {}", "use case definition with empty body")]
fn test_parse_use_case_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::use_case_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// View Definition Tests

#[rstest]
#[case("view", "view keyword")]
fn test_parse_view_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("view def", "view def keyword")]
fn test_parse_view_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("view def TestView;", "view definition with semicolon")]
#[case("view def TestView {}", "view definition with empty body")]
fn test_parse_view_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon body")]
#[case("{}", "empty body")]
fn test_parse_view_definition_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_definition_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myRendering;", "view rendering usage with reference")]
#[case("rendering myRender {}", "view rendering usage with keyword")]
fn test_parse_view_rendering_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_rendering_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("render myRendering;", "view rendering member")]
fn test_parse_view_rendering_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_rendering_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// View Usage Tests

#[rstest]
#[case("view", "view usage keyword")]
fn test_parse_view_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("view myView;", "view usage with semicolon")]
#[case("view myView {}", "view usage with empty body")]
fn test_parse_view_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(";", "semicolon body")]
#[case("{}", "empty body")]
fn test_parse_view_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::view_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Viewpoint Tests

#[rstest]
#[case("viewpoint", "viewpoint keyword")]
fn test_parse_viewpoint_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::viewpoint_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("viewpoint def", "viewpoint def keyword")]
fn test_parse_viewpoint_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::viewpoint_def_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("viewpoint def TestViewpoint;", "viewpoint definition with semicolon")]
#[case(
    "viewpoint def TestViewpoint {}",
    "viewpoint definition with empty body"
)]
#[case(
    "viewpoint def TestViewpoint { stakeholder user; }",
    "viewpoint definition with stakeholder"
)]
fn test_parse_viewpoint_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::viewpoint_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("viewpoint", "viewpoint usage keyword")]
fn test_parse_viewpoint_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::viewpoint_usage_keyword, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("viewpoint myViewpoint;", "viewpoint usage with semicolon")]
#[case("viewpoint myViewpoint {}", "viewpoint usage with empty body")]
fn test_parse_viewpoint_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::viewpoint_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Rendering Definition Tests

#[rstest]
#[case("rendering", "rendering keyword")]
fn test_parse_rendering_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::rendering_token, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("rendering def", "rendering def keyword")]
fn test_parse_rendering_def_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::rendering_def, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("rendering def TestRendering;", "rendering definition with semicolon")]
#[case(
    "rendering def TestRendering {}",
    "rendering definition with empty body"
)]
fn test_parse_rendering_definition(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::rendering_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Rendering Usage Tests

#[rstest]
#[case("rendering", "rendering usage keyword")]
fn test_parse_rendering_usage_keyword(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::rendering_token, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("rendering myRendering;", "rendering usage")]
fn test_parse_rendering_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::rendering_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Expression Tests

#[rstest]
#[case(";", "expression body with semicolon")]
#[case("{}", "expression body with empty body")]
fn test_parse_expression_body(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::expression_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myValue", "owned expression member with identifier")]
fn test_parse_owned_expression_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::owned_expression_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("myValue", "conditional expression with identifier")]
#[case("\"test\"", "conditional expression with string")]
#[case("123", "conditional expression with number")]
#[case("false", "conditional expression with boolean")]
#[case("if x ? a else b", "conditional expression with if-else")]
fn test_parse_conditional_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::conditional_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "if x ? a else b",
    "conditional expression with if-then-else (short names)"
)]
#[case(
    "if condition ? trueValue else falseValue",
    "conditional expression with if-then-else"
)]
fn test_parse_conditional_expression_ternary(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::conditional_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "null coalescing expression with single value")]
#[case("a ?? b", "null coalescing expression with operator")]
fn test_parse_null_coalescing_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::null_coalescing_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "implies expression with single value")]
#[case("a implies b", "implies expression with operator")]
fn test_parse_implies_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::implies_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "or expression with single value")]
#[case("a | b", "or expression with pipe operator")]
#[case("a or b", "or expression with or keyword")]
fn test_parse_or_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::or_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "xor expression with single value")]
#[case("a xor b", "xor expression with operator")]
fn test_parse_xor_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::xor_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "and expression with single value")]
#[case("a & b", "and expression with ampersand operator")]
#[case("a and b", "and expression with and keyword")]
fn test_parse_and_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::and_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "equality expression with identifier")]
#[case("42", "equality expression with number")]
#[case("a == b", "equality expression with == operator")]
#[case("a != b", "equality expression with != operator")]
#[case("a === b", "equality expression with === operator")]
#[case("a !== b", "equality expression with !== operator")]
fn test_parse_equality_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::equality_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("==", "equality operator ==")]
#[case("!=", "equality operator !=")]
#[case("===", "equality operator ===")]
#[case("!==", "equality operator !==")]
fn test_parse_equality_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::equality_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "classification expression with identifier")]
#[case("value hastype Type", "classification expression with hastype")]
#[case("value istype Type", "classification expression with istype")]
#[case("value @ Type", "classification expression with @ operator")]
#[case("value as Type", "classification expression with as")]
#[case("metadata @@ Type", "classification expression with @@")]
#[case("metadata meta Type", "classification expression with meta")]
fn test_parse_classification_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::classification_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("hastype", "classification test operator hastype")]
#[case("istype", "classification test operator istype")]
#[case("@", "classification test operator @")]
fn test_parse_classification_test_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::classification_test_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("MyType", "type reference member")]
fn test_parse_type_reference_member(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::type_reference_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("MyType", "type reference")]
fn test_parse_type_reference(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::type_reference, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "relational expression with identifier")]
#[case("42", "relational expression with number")]
#[case("a < b", "relational expression with < operator")]
#[case("a > b", "relational expression with > operator")]
#[case("a <= b", "relational expression with <= operator")]
#[case("a >= b", "relational expression with >= operator")]
fn test_parse_relational_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::relational_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("<", "relational operator <")]
#[case(">", "relational operator >")]
#[case("<=", "relational operator <=")]
#[case(">=", "relational operator >=")]
fn test_parse_relational_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::relational_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "range expression with single value")]
#[case("1..10", "range expression with range operator")]
fn test_parse_range_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::range_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "additive expression with single value")]
#[case("a + b", "additive expression with + operator")]
#[case("a - b", "additive expression with - operator")]
#[case("a + b - c", "additive expression with multiple operators")]
fn test_parse_additive_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::additive_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("+", "additive operator +")]
#[case("-", "additive operator -")]
fn test_parse_additive_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::additive_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "multiplicative expression with single value")]
#[case("a * b", "multiplicative expression with * operator")]
#[case("a / b", "multiplicative expression with / operator")]
#[case("a % b", "multiplicative expression with % operator")]
fn test_parse_multiplicative_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::multiplicative_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("*", "multiplicative operator *")]
#[case("/", "multiplicative operator /")]
#[case("%", "multiplicative operator %")]
fn test_parse_multiplicative_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::multiplicative_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "exponentiation expression with single value")]
#[case("a ** b", "exponentiation expression with ** operator")]
#[case("a ^ b", "exponentiation expression with ^ operator")]
fn test_parse_exponentiation_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::exponentiation_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("**", "exponentiation operator **")]
#[case("^", "exponentiation operator ^")]
fn test_parse_exponentiation_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::exponentiation_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "unary expression with identifier")]
#[case("+value", "unary expression with + operator")]
#[case("-value", "unary expression with - operator")]
#[case("~value", "unary expression with ~ operator")]
#[case("not value", "unary expression with not operator")]
fn test_parse_unary_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::unary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("+", "unary operator +")]
#[case("-", "unary operator -")]
#[case("~", "unary operator ~")]
#[case("not", "unary operator not")]
fn test_parse_unary_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::unary_operator, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("value", "extent expression with identifier")]
#[case("42", "extent expression with number")]
fn test_parse_extent_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::extent_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("\"hello world\"", "string literal")]
#[case("\"\"", "empty string")]
fn test_parse_string_value(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::string_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("42", "positive integer")]
#[case("-42", "negative integer")]
#[case("3.14", "decimal number")]
#[case("-3.14", "negative decimal")]
fn test_parse_numeric_value(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::numeric_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("true", "boolean true")]
#[case("false", "boolean false")]
fn test_parse_boolean_value(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::boolean_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("\"string\"", "literal with string")]
#[case("42", "literal with number")]
#[case("true", "literal with boolean")]
#[case("null", "literal with null")]
fn test_parse_literal(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::literal, input);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[test]
fn test_parse_attribute_def_from_stdlib() {
    use std::path::PathBuf;
    use syster::project::file_loader;
    use syster::syntax::sysml::ast::Element;
    use syster::syntax::sysml::ast::enums::DefinitionKind;

    // Test actual attribute def from MeasurementReferences.sysml
    let input = r#"
    package TestPackage {
        attribute def DimensionOneUnit {
        }
    }
    "#;

    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(input, &path);
    let language_file = parse_result.content.expect("Parse should succeed");
    let file = match language_file {
        syster::syntax::SyntaxFile::SysML(f) => f,
        _ => panic!("Expected SysML file"),
    };

    // Should have 1 element (the package)
    assert_eq!(file.elements.len(), 1);

    let package = match &file.elements[0] {
        Element::Package(p) => p,
        _ => panic!("Expected Package"),
    };

    // Package should have 1 member (the attribute def)
    assert_eq!(package.elements.len(), 1, "Package should have 1 member");

    // Check the attribute def
    let member = &package.elements[0];
    let Element::Definition(def) = member else {
        panic!("Expected Definition member, got {member:?}");
    };

    assert_eq!(
        def.kind,
        DefinitionKind::Attribute,
        "Should be Attribute definition"
    );
    assert_eq!(
        def.name,
        Some("DimensionOneUnit".to_string()),
        "Should have correct name"
    );
}

#[test]
fn test_parse_abstract_attribute_def() {
    use std::path::PathBuf;
    use syster::project::file_loader;
    use syster::syntax::sysml::ast::Element;
    use syster::syntax::sysml::ast::enums::DefinitionKind;

    // Test ABSTRACT attribute def like in stdlib
    let input = r#"
    package MeasurementReferences {
        abstract attribute def ScalarMeasurementReference {
        }
    }
    "#;

    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(input, &path);

    assert!(
        parse_result.content.is_some(),
        "Parse should succeed. Errors: {:?}",
        parse_result.errors
    );

    let language_file = parse_result.content.expect("Parse should succeed");
    let syster::syntax::SyntaxFile::SysML(file) = language_file else {
        panic!("Expected SysML file");
    };

    // Should have 1 element (the package)
    assert_eq!(file.elements.len(), 1, "Should have 1 package");

    let Element::Package(package) = &file.elements[0] else {
        panic!("Expected Package, got {:?}", file.elements[0]);
    };

    // Package should have 1 member (the attribute def)
    assert_eq!(package.elements.len(), 1, "Package should have 1 member");

    // Check the attribute def
    let member = &package.elements[0];
    let Element::Definition(def) = member else {
        panic!("Expected Definition member, got {member:?}");
    };
    assert_eq!(
        def.kind,
        DefinitionKind::Attribute,
        "Should be Attribute definition"
    );
    assert_eq!(
        def.name,
        Some("ScalarMeasurementReference".to_string()),
        "Should have correct name"
    );
    assert!(
        def.is_abstract,
        "Should be marked as abstract - THIS IS THE BUG!"
    );
}

// =============================================================================
// Issue #619: Support conjugated port type syntax (~TypeName)
// =============================================================================

/// Tests that owned_feature_typing supports conjugated types with ~ prefix
#[rstest]
#[case("~DriveIF", "DriveIF")]
#[case("~MyPort", "MyPort")]
#[case("~Qualified::Name", "Qualified::Name")]
fn test_conjugated_port_typing_in_owned_feature_typing(
    #[case] input: &str,
    #[case] _expected_type: &str,
) {
    let result = SysMLParser::parse(Rule::owned_feature_typing, input);
    assert!(
        result.is_ok(),
        "Failed to parse conjugated type '{}': {:?}",
        input,
        result.err()
    );
    let pairs = result.unwrap();
    assert_eq!(
        pairs.as_str(),
        input,
        "Should consume entire conjugated type"
    );
}

/// Tests that port usages can have conjugated types
#[rstest]
#[case("port drive: ~DriveIF;")]
#[case("port input: ~InputPort;")]
#[case("port p: ~Pkg::PortDef;")]
fn test_port_usage_with_conjugated_type(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::port_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse port with conjugated type '{}': {:?}",
        input,
        result.err()
    );
}

/// Tests a part definition containing a port with conjugated type
#[test]
fn test_part_def_with_conjugated_port() {
    let source = r#"
        part def Transmission {
            port drive: ~DriveIF;
        }
    "#;
    let result = SysMLParser::parse(Rule::part_definition, source.trim());
    assert!(
        result.is_ok(),
        "Failed to parse part def with conjugated port: {:?}",
        result.err()
    );
}

/// Tests that regular (non-conjugated) port types still work
#[rstest]
#[case("port drive: DriveIF;")]
#[case("port p: MyPort;")]
fn test_port_usage_with_regular_type(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::port_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse port with regular type '{}': {:?}",
        input,
        result.err()
    );
}

// =============================================================================
// Issue #624: Support ::> (references) operator in connector/interface ends
// =============================================================================

/// Tests that interface_end supports ::> syntax for reference subsetting
#[rstest]
#[case("transDrive ::> transmission.drive")]
#[case("axleDrive ::> rearAxle.drive")]
#[case("p ::> port1")]
fn test_interface_end_with_references_operator(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::interface_end, input);
    assert!(
        result.is_ok(),
        "Failed to parse interface end with ::> operator '{}': {:?}",
        input,
        result.err()
    );
}

/// Tests full interface usage with ::> syntax
#[test]
fn test_interface_usage_with_references_operator() {
    let source = r#"
        interface driveShaft connect 
            transDrive ::> transmission.drive to axleDrive ::> rearAxle.drive;
    "#;
    let result = SysMLParser::parse(Rule::interface_usage, source.trim());
    assert!(
        result.is_ok(),
        "Failed to parse interface usage with ::> operator: {:?}",
        result.err()
    );
}

/// Tests interface usage patterns from SysML v2 spec 8.2.2.14.2
#[rstest]
// Pattern 1: declaration + connect + ends
#[case("interface fuelLine : FuelingInterface connect fuelTank.fuelingPort to engine.fuelingPort;")]
// Pattern 2: shorthand - just ends (no connect keyword when declaration empty)
#[case("interface fuelTank.fuelingPort to engine.fuelingPort;")]
// Pattern 3: name + connect (no type specialization)
#[case("interface myInterface connect portA to portB;")]
// Pattern 4: with body
#[case("interface conn connect a.p to b.p { }")]
// Pattern 5: simple feature chains as ends
#[case("interface system.input to system.output;")]
fn test_interface_usage_patterns(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::interface_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse interface usage '{}': {:?}",
        input,
        result.err()
    );
}

/// Tests interface definition with end features (from spec)
#[rstest]
#[case(
    "interface def FuelingInterface { end fuelOutPort : FuelingPort; end fuelInPort : ~FuelingPort; }"
)]
#[case("interface def DataIF { end sender : DataPort; end receiver : ~DataPort; }")]
fn test_interface_definition_with_ends(#[case] input: &str) {
    let result = SysMLParser::parse(Rule::interface_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse interface definition with ends '{}': {:?}",
        input,
        result.err()
    );
}

/// Tests flow_end rule for parsing feature chain paths in flow connections
/// This is the core issue: `b.f.a` should parse as a flow end with chained features
#[rstest]
#[case("a", "simple identifier")]
#[case("a.b", "two-part feature chain")]
#[case("b.f.a", "three-part feature chain")]
#[case("x.y.z.w", "four-part feature chain")]
fn test_flow_end_feature_chains(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_end, input);
    assert!(
        result.is_ok(),
        "Failed to parse flow_end '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
    // Verify the entire input was consumed
    let pairs = result.unwrap();
    let consumed: String = pairs.map(|p| p.as_str()).collect();
    assert_eq!(
        consumed, input,
        "flow_end '{input}' ({desc}) only consumed '{consumed}', expected full input"
    );
}

/// Tests flow_part rule - source to target
#[rstest]
#[case("a to b", "simple identifiers")]
#[case("a.b to c", "two-part source")]
#[case("a to b.c", "two-part target")]
#[case("a.b to c.d", "two-part on both")]
#[case("b.f.a to c.aa", "three-part source, two-part target")]
fn test_flow_part(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_part, input);
    assert!(
        result.is_ok(),
        "Failed to parse flow_part '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests imported_membership syntax (single element imports like a::b)
/// Can include recursive marker (::**) to import all nested public members
#[rstest]
#[case("q", "single-char import")]
#[case("myPackage", "named import")]
#[case("A::B", "qualified import")]
#[case("A::B::C", "multi-level qualified import")]
#[case("q::**", "single-char recursive import")]
#[case("myPackage::**", "named recursive import")]
#[case("A::B::**", "qualified recursive import")]
fn test_imported_membership(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::imported_membership, input);
    assert!(
        result.is_ok(),
        "Failed to parse imported_membership '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
    // Verify full consumption
    let pairs = result.unwrap();
    let consumed: String = pairs.map(|p| p.as_str()).collect();
    assert_eq!(
        consumed, input,
        "imported_membership only consumed '{consumed}', expected '{input}'"
    );
}

/// Tests imported_namespace syntax (wildcard imports like pkg::*)
/// Requires ::* marker to import all direct members; can add ::* for recursive
#[rstest]
#[case("q::*", "single-char namespace import")]
#[case("myPackage::*", "named namespace import")]
#[case("A::B::*", "qualified namespace import")]
#[case("A::B::*::**", "namespace with recursive import")]
fn test_imported_namespace(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::imported_namespace, input);
    assert!(
        result.is_ok(),
        "Failed to parse imported_namespace '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
    // Verify full consumption
    let pairs = result.unwrap();
    let consumed: String = pairs.map(|p| p.as_str()).collect();
    assert_eq!(
        consumed, input,
        "imported_namespace only consumed '{consumed}', expected '{input}'"
    );
}

/// Tests flow_usage with feature chain paths (from FeaturePathTest.sysml)
#[rstest]
#[case("flow a to b;", "simple identifiers")]
#[case("flow a.b to c;", "two-part source chain")]
#[case("flow a to b.c;", "two-part target chain")]
#[case("flow a.b to c.d;", "two-part chains on both sides")]
#[case(
    "flow b.f.a to c.aa;",
    "three-part source, two-part target - from FeaturePathTest"
)]
fn test_flow_usage_with_feature_chains(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse flow_usage '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests state_action_usage patterns (for entry/do/exit actions)
#[rstest]
#[case("action myAction;", "action with identifier")]
#[case(";", "empty action")]
#[case("someAction;", "qualified name reference")]
fn test_state_action_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::state_action_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse state_action_usage '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests send_node_declaration for state entry actions
#[rstest]
#[case("send msg", "send with parameter")]
#[case("send msg via port", "send with via")]
fn test_send_node_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::send_node_declaration, input);
    assert!(
        result.is_ok(),
        "Failed to parse send_node_declaration '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests succession_flow_usage (succession flow x to y;)
#[rstest]
#[case("succession flow x.p to a.b;", "succession flow with chains")]
#[case("succession flow a to b;", "succession flow simple")]
#[case(
    "succession flow onOffCmdFlow from sendOnOffCmd.onOffCmd to produceDirectedLight.onOffCmd;",
    "succession flow with name and from-to"
)]
fn test_succession_flow_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::succession_flow_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse succession_flow_usage '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests textual_representation with optional name
#[rstest]
#[case("rep language \"ocl\" /* code */", "rep without name")]
#[case("rep inOCL language \"ocl\" /* code */", "rep with name")]
fn test_textual_representation(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::textual_representation, input);
    assert!(
        result.is_ok(),
        "Failed to parse textual_representation '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

// =============================================================================
// AssignmentTest.sysml patterns - assignments in various contexts
// =============================================================================

/// Tests parsing of the full AssignmentTest package with complex patterns
#[test]
fn test_parse_assignment_test_package_full() {
    let input = r#"package AssignmentTest {
	
	part def Counter {
		attribute count : ScalarValues::Integer := 0;
		
		action incr {
			assign count := count + 1;
		}
		
		action decr {
			assign count := count - 1;
		}
	}
	
	attribute def Incr;
	attribute def Decr;
	
	state def Counting {
		part counter : Counter;
		entry assign counter.count := 0;
		
		then state wait;
		accept Incr
			then increment;
		accept Decr
			then decrement;
		
		state increment {
			do assign counter.count := counter.count + 1;
		}
		then wait;
		
		state decrement {
			do assign counter.count := counter.count - 1;
		}
		then wait;
	}
	
	calc def Increment { 
		in c : Counter;
		return : Counter;
		
		perform c.incr;
		c
	}
	
	action a {
		state counting : Counting;
		assign counting.counter.count := counting.counter.count + 1;
		assign counting.counter.count := Increment(counting.counter).count;
	}
}"#;

    let result = SysMLParser::parse(Rule::package, input);

    assert!(
        result.is_ok(),
        "Failed to parse AssignmentTest package: {:?}",
        result.err()
    );
}

/// Tests assignment action usage with various patterns including feature chains and calc invocation
#[rstest]
#[case("assign count := count + 1;", "simple increment assignment")]
#[case("assign count := count - 1;", "simple decrement assignment")]
#[case("assign counter.count := 0;", "feature chain assignment")]
#[case(
    "assign counter.count := counter.count + 1;",
    "feature chain self-increment"
)]
#[case(
    "assign counting.counter.count := counting.counter.count + 1;",
    "nested feature chain assignment"
)]
#[case(
    "assign counting.counter.count := Increment(counting.counter).count;",
    "assignment with calc invocation"
)]
fn test_parse_assignment_with_feature_chains(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::assignment_node_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse assignment_node_declaration '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests part def with nested action definitions containing assignments
#[test]
fn test_parse_part_def_counter_with_actions() {
    let input = r#"part def Counter {
		attribute count : ScalarValues::Integer := 0;
		
		action incr {
			assign count := count + 1;
		}
		
		action decr {
			assign count := count - 1;
		}
	}"#;

    let result = SysMLParser::parse(Rule::part_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse Counter part def with actions: {:?}",
        result.err()
    );
}

/// Tests state def with entry assignment, accept transitions, and do assignments
#[test]
fn test_parse_state_def_with_entry_and_do_assignments() {
    let input = r#"state def Counting {
		part counter : Counter;
		entry assign counter.count := 0;
		
		then state wait;
		accept Incr
			then increment;
		accept Decr
			then decrement;
		
		state increment {
			do assign counter.count := counter.count + 1;
		}
		then wait;
		
		state decrement {
			do assign counter.count := counter.count - 1;
		}
		then wait;
	}"#;

    let result = SysMLParser::parse(Rule::state_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse Counting state def: {:?}",
        result.err()
    );
}

/// Tests calc def with perform action and expression result
#[test]
fn test_parse_calc_def_with_perform() {
    let input = r#"calc def Increment { 
		in c : Counter;
		return : Counter;
		
		perform c.incr;
		c
	}"#;

    let result = SysMLParser::parse(Rule::calculation_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse Increment calc def: {:?}",
        result.err()
    );
}

/// Tests action with state usage and assignments including calc invocation
#[test]
fn test_parse_action_with_state_and_calc_invocation() {
    let input = r#"action a {
		state counting : Counting;
		assign counting.counter.count := counting.counter.count + 1;
		assign counting.counter.count := Increment(counting.counter).count;
	}"#;

    let result = SysMLParser::parse(Rule::action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse action a with state and assignments: {:?}",
        result.err()
    );
}

// =============================================================================
// DecisionTest.sysml patterns - succession with guarded transitions
// =============================================================================

/// Tests succession_as_usage with guarded target succession (if ... then ...)
#[rstest]
#[case("first A1 then A2;", "simple succession")]
#[case("succession S first A1 then A2;", "named succession")]
#[case("first A1 if x == 0 then A2;", "succession with guard")]
#[case(
    "succession S first A1 if x == 0 then A2;",
    "named succession with guard"
)]
#[case("first A1 else A2;", "succession with default")]
fn test_parse_succession_with_guarded_transitions(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::succession_as_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse succession_as_usage '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests the full DecisionTest action def pattern
#[test]
fn test_parse_decision_test_action_def() {
    let input = r#"action def DecisionTest {
        attribute x = 1;

        decide 'test x';
        if x == 1 then A1; 
        if x > 1 then A2;
        else A3; 

        then decide D; 
        if true then A1;
        if false then A2;

        action A1;
        action A2;
        action A3;

        succession S first A1 
                if x == 0 then A2;

        first A3;
                if x > 0 then 'test x';
}"#;

    let result = SysMLParser::parse(Rule::action_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse DecisionTest action def: {:?}",
        result.err()
    );
}

// =============================================================================
// PictureTaking.sysml patterns - flow of Type from X to Y
// =============================================================================

/// Tests flow_usage_declaration with typed flow pattern
#[rstest]
#[case("of Exposure from focus.xrsl to shoot.xsf", "typed flow with from-to")]
#[case("from focus.xrsl to shoot.xsf", "flow with from-to")]
#[case("focus.xrsl to shoot.xsf", "flow with to only")]
#[case("myFlow : Exposure from x to y", "named typed flow")]
#[case(
    "publish_returnallitems of Publish from apspm.pub to mqget.APIS_MQTT.pub",
    "named typed flow with from-to"
)]
#[case(
    "call_getItems of CallGiveItems[1] from tlu.cll to apsph.cll",
    "named typed flow with multiplicity"
)]
fn test_parse_flow_usage_declaration(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_usage_declaration, input);

    assert!(
        result.is_ok(),
        "Failed to parse flow_usage_declaration '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests flow_usage with multi-line declaration
#[rstest]
#[case("flow publish of Publish from a.b to c.d;", "single line flow")]
#[case("flow publish of Publish\nfrom a.b to c.d;", "multi-line flow LF")]
#[case("flow publish of Publish\r\nfrom a.b to c.d;", "multi-line flow CRLF")]
fn test_parse_flow_usage(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::flow_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse flow_usage '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests interface usage with flows inside
#[test]
fn test_parse_interface_usage_with_flows() {
    let input = r#"package Test {
        occurrence def APIS_transfer_lifetime {
            interface APIS_transfer_interface : Interfaces::Interface connect (
                tlu ::> X.a, // port reference
                apsph ::> Y.b) {

                flow publish_returnallitems of Publish
                from apspm.pub to mqget.APIS_MQTT.pub;
                flow subscribe_returnallitems of Subscribe
                from apsc.subscr to mqgive.APIS_MQTT.subscr;
                flow call_getItems of CallGiveItems[1]
                from tlu.cll to apsph.cll;
            }
        }
    }"#;

    let result = SysMLParser::parse(Rule::file, input);

    assert!(
        result.is_ok(),
        "Failed to parse interface usage with flows: {:?}",
        result.err()
    );
}

/// Tests the full PictureTaking action pattern
#[test]
fn test_parse_picture_taking_action() {
    let input = r#"action takePicture {
        action focus: Focus[1];
        flow of Exposure from focus.xrsl to shoot.xsf;
        action shoot: Shoot[1];
    }"#;

    let result = SysMLParser::parse(Rule::action_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse takePicture action: {:?}",
        result.err()
    );
}

// =============================================================================
// CauseAndEffectExample.sysml patterns - connector with references operator
// =============================================================================

/// Tests connector_end_reference with references operator (::>)
#[rstest]
#[case("cause1 ::> causer1", "simple references")]
#[case("effect1 ::> effected1", "effect references")]
fn test_parse_connector_end_with_references_operator(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::connector_end_reference, input);

    assert!(
        result.is_ok(),
        "Failed to parse connector_end_reference '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests n-ary connector with metadata prefix and references operator
#[test]
fn test_parse_nary_connector_with_metadata() {
    let input = r#"#multicausation connection : MultiCauseEffect connect
                ( cause1 ::> causer1, cause2 ::> causer2,
                  effect1 ::> effected1, effect2 ::> effected2 );"#;

    let result = SysMLParser::parse(Rule::connection_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse n-ary connector with metadata: {:?}",
        result.err()
    );
}

// =============================================================================
// MassRollup.sysml patterns - select expression with specializes operator
// =============================================================================

/// Tests parameter_binding with specializes operator (:>) in addition to typed-by (:)
#[rstest]
#[case("in p : ISQ::mass", "typed-by with direction")]
#[case("in p :> ISQ::mass", "specializes with direction")]
#[case("p : ISQ::mass", "typed-by without direction")]
#[case("p :> ISQ::mass", "specializes without direction")]
fn test_parse_parameter_binding_with_specializes(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::parameter_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse parameter_binding '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests select expression body with specializes operator
#[test]
fn test_parse_select_expression_body() {
    let input = r#"{in p :> ISQ::mass; p > minMass}"#;

    let result = SysMLParser::parse(Rule::expression_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse select expression body: {:?}",
        result.err()
    );
}

// =============================================================================
// RequirementDerivationExample.sysml patterns - #derivation connection
// =============================================================================

/// Tests connection_usage with #derivation prefix and end elements inside block
#[test]
fn test_parse_derivation_connection_with_end_elements() {
    let input = r#"#derivation connection : Req1_Derivation {
        end r1 ::> req1;
        end r1_1 ::> req1_1;
        end r1_2 ::> req1_2;
    }"#;
    let result = SysMLParser::parse(Rule::connection_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse #derivation connection with end elements: {:?}",
        result.err()
    );
}

/// Tests part usage with connection inside (the actual context from RequirementDerivationExample)
#[test]
fn test_parse_part_with_derivation_connection() {
    let input = r#"part satisfactionContext {
        ref :>> system;
        
        satisfy requirement req1 : Req1 by system;
        
        #derivation connection : Req1_Derivation {
            end r1 ::> req1;
        }
    }"#;
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse part with #derivation connection: {:?}",
        result.err()
    );
}

/// Tests satisfy requirement pattern
#[test]
fn test_parse_satisfy_requirement() {
    let input = "satisfy requirement req1 : Req1 by system;";
    let result = SysMLParser::parse(Rule::satisfy_requirement_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse satisfy requirement: {:?}",
        result.err()
    );
}

/// Tests satisfy requirement with dotted feature chain in 'by' clause
#[test]
fn test_parse_satisfy_requirement_with_dot() {
    let input = "satisfy requirement req1_1 : Req1_1 by system.sub1;";
    let result = SysMLParser::parse(Rule::satisfy_requirement_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse satisfy requirement with dotted feature: {:?}",
        result.err()
    );
}

/// Tests #derivation connection def (definition, not usage)
#[test]
fn test_parse_derivation_connection_def() {
    let input = r#"#derivation connection def Req1_Derivation {
        end #original r1 : Req1;
        end #derive r1_1 : Req1_1;
        end #derive r1_2 : Req1_2;
    }"#;
    let result = SysMLParser::parse(Rule::connection_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse #derivation connection def: {:?}",
        result.err()
    );
}

/// Tests the full RequirementDerivationExample as a package
#[test]
fn test_parse_full_requirement_derivation_example() {
    let input = r#"package RequirementDerivationExample {
    private import RequirementDerivation::*;
    
    requirement def Req1;
    
    requirement def Req1_1;
    requirement def Req1_2;
    
    #derivation connection def Req1_Derivation {
        end #original r1 : Req1;
        end #derive r1_1 : Req1_1;
        end #derive r1_2 : Req1_2;
    }
    
    part def System;
    part def Subsystem1;
    part def Subsystem2;
    
    part system : System {
        part sub1 : Subsystem1;
        part sub2 : Subsystem2;
    }
    
    part satisfactionContext {
        ref :>> system;
        
        satisfy requirement req1 : Req1 by system;
        satisfy requirement req1_1 : Req1_1 by system.sub1;
        satisfy requirement req1_2 : Req1_2 by system.sub2;
        
        #derivation connection : Req1_Derivation {
            end r1 ::> req1;
            end r1_1 ::> req1_1;
            end r1_2 ::> req1_1;
        }
        
    }
    
}"#;
    let result = SysMLParser::parse(Rule::package, input);
    assert!(
        result.is_ok(),
        "Failed to parse full RequirementDerivationExample: {:?}",
        result.err()
    );
}

// =============================================================================
// TextualRepresentationTest.sysml patterns - language/rep textual representation
// =============================================================================

/// Tests action def with textual representation (language "alf")
#[test]
fn test_parse_action_with_textual_rep() {
    let input = r#"action def setX {
        in c : C;
        in newX : Real;
        
        language "alf" 
            /* c.x = newX; */
    }"#;
    let result = SysMLParser::parse(Rule::action_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse action def with textual representation: {:?}",
        result.err()
    );
}

// =============================================================================
// TimeVaryingAttribute.sysml patterns - anonymous features with redefinition
// =============================================================================

/// Tests parsing feature chain redefinition
#[test]
fn test_parse_feature_chain_in_redefinition() {
    let input = ":>> localClock.currentTime";
    let result = SysMLParser::parse(Rule::redefinitions, input);
    assert!(
        result.is_ok(),
        "Failed to parse feature chain redefinition: {:?}",
        result.err()
    );
}

/// Tests feature_specialization_part with redefinition
#[test]
fn test_parse_feature_specialization_with_redefinition() {
    let input = ":>> localClock.currentTime";
    let result = SysMLParser::parse(Rule::feature_specialization_part, input);
    assert!(
        result.is_ok(),
        "Failed to parse feature specialization part: {:?}",
        result.err()
    );
}

/// Tests attribute with redefinition (no value, just semicolon)
#[test]
fn test_parse_attribute_with_redefinition_no_value() {
    let input = "attribute :>> localClock.currentTime;";
    let result = SysMLParser::parse(Rule::attribute_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse attribute with redefinition (no value): {:?}",
        result.err()
    );
}

/// Tests usage_suffix with redefinition
#[test]
fn test_parse_usage_suffix_with_redefinition() {
    let input = ":>> localClock.currentTime;";
    let result = SysMLParser::parse(Rule::usage_suffix, input);
    assert!(
        result.is_ok(),
        "Failed to parse usage_suffix with redefinition: {:?}",
        result.err()
    );
}

/// Tests anonymous attribute with redefinition and value
#[test]
fn test_parse_anonymous_attribute_with_redefinition() {
    let input = "attribute :>> localClock.currentTime = startTime + elapseTime;";
    let result = SysMLParser::parse(Rule::attribute_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse anonymous attribute with redefinition: {:?}",
        result.err()
    );
}

/// Tests anonymous feature with redefinition in snapshot
#[test]
fn test_parse_anonymous_redefinition_in_snapshot() {
    let input = r#"snapshot :>> start {
        :>> elapseTime = 0 [s];
        :>> pwrCmd.pwrLevel = 0;
    }"#;
    let result = SysMLParser::parse(Rule::portion_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse snapshot with anonymous redefinitions: {:?}",
        result.err()
    );
}

// =============================================================================
// UseCaseTest.sysml patterns - include use case with typing
// =============================================================================

/// Tests include use case with typing
#[test]
fn test_parse_include_use_case_with_typing() {
    let input = "include use case uc1 : UC1;";
    let result = SysMLParser::parse(Rule::include_use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse include use case with typing: {:?}",
        result.err()
    );
}

/// Tests include use case with body
#[test]
fn test_parse_include_use_case_with_body() {
    let input = r#"include use case uc2 {
        subject = system;
        actor user = UseSystem::user;
    }"#;
    let result = SysMLParser::parse(Rule::include_use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse include use case with body: {:?}",
        result.err()
    );
}

/// Tests shortened include form
#[test]
fn test_parse_include_shorthand() {
    let input = "include uc2;";
    let result = SysMLParser::parse(Rule::include_use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse include shorthand: {:?}",
        result.err()
    );
}

/// Tests include with feature chain
#[test]
fn test_parse_include_with_feature_chain() {
    let input = "include system.uc1;";
    let result = SysMLParser::parse(Rule::include_use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse include with feature chain: {:?}",
        result.err()
    );
}

// =============================================================================
// VariabilityTest.sysml patterns - variation definitions and variant usages
// =============================================================================

/// Tests variation part definition
#[test]
fn test_parse_variation_part_def() {
    let input = r#"variation part def V :> P {
        variant part x : Q {
            attribute b : B;
        }
    }"#;
    let result = SysMLParser::parse(Rule::part_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse variation part definition: {:?}",
        result.err()
    );
}

/// Tests variation part usage
#[test]
fn test_parse_variation_part_usage() {
    let input = r#"variation part v : P {
        variant q {
            attribute b : B;
        }
    }"#;
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse variation part usage: {:?}",
        result.err()
    );
}

// =============================================================================
// VehicleGeometryAndCoordinateFrames.sysml patterns - forAll expressions
// =============================================================================

/// Tests forAll expression with parameter and attributes
#[test]
fn test_parse_forall_with_parameter() {
    let input = r#"(1..5)->forAll {
        in i : Natural;
        i > 0
    }"#;
    let result = SysMLParser::parse(Rule::owned_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse forAll with parameter: {:?}",
        result.err()
    );
}

/// Tests forAll expression with private attribute
#[test]
fn test_parse_forall_with_private_attribute() {
    let input = r#"(1..5)->forAll {
        in i : Natural;
        private attribute x = i * 2;
        x > 0
    }"#;
    let result = SysMLParser::parse(Rule::owned_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse forAll with private attribute: {:?}",
        result.err()
    );
}

/// Tests forAll with hash/metadata access
#[test]
fn test_parse_forall_with_hash_access() {
    let input = r#"(1..numberOfBolts)->forAll {
        in i : Natural;
        private attribute lbcf = lugBolts#(i).coordinateFrame;
        lbcf == null
    }"#;
    let result = SysMLParser::parse(Rule::owned_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse forAll with hash access: {:?}",
        result.err()
    );
}

/// Tests forAll with attribute containing body
#[test]
fn test_parse_forall_with_attribute_body() {
    let input = r#"(1..n)->forAll {
        in i : Natural;
        private attribute trs : TranslationRotationSequence {
            :>> source = wcf;
            :>> target = lbcf;
        }
        trs == null
    }"#;
    let result = SysMLParser::parse(Rule::owned_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse forAll with attribute body: {:?}",
        result.err()
    );
}

/// Tests new expression with subscript
#[test]
fn test_parse_new_with_subscript() {
    let input = "new Translation((x, y, z)[wcf])";
    let result = SysMLParser::parse(Rule::instantiation_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse new with subscript: {:?}",
        result.err()
    );
}

/// Tests parameter binding with in direction
#[test]
fn test_parse_parameter_binding_with_in() {
    let input = "in i : Natural";
    let result = SysMLParser::parse(Rule::parameter_binding, input);
    assert!(
        result.is_ok(),
        "Failed to parse parameter binding with in: {:?}",
        result.err()
    );
}

/// Tests forAll inside constraint body
#[test]
fn test_parse_forall_in_constraint() {
    let input = r#"assert constraint {
        (1..n)->forAll {
            in i : Natural;
            i > 0
        }
    }"#;
    let result = SysMLParser::parse(Rule::assert_constraint_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse forAll in constraint: {:?}",
        result.err()
    );
}

/// Tests full Wheel part def from VehicleGeometryAndCoordinateFrames
#[test]
fn test_parse_wheel_part_def_with_forall() {
    let input = r#"part def Wheel :> SpatialItem {
        item :>> shape : Cylinder {
            :>> radius = 22/2*25.4 + 110 [mm]; 
            :>> height = 220 [mm];
        }
        attribute <wcf> wheelCoordinateFrame : CoordinateFrame;
        
        attribute numberOfBolts : Natural = 5;	
        part lugBolts : LugBolt[1..numberOfBolts] :> subSpatialParts;
        
        attribute <lbpr> lugBoltPlacementRadius :>> radius default 60 [mm];
        private attribute lugBoltDistributionAngle :>> planeAngle = 360/numberOfBolts ['°'];
        private attribute lbda : Real = lugBoltDistributionAngle.num * (pi/180);
        assert constraint {
            (1..numberOfBolts)->forAll {
                in i : Natural;
                private attribute lbcf = lugBolts#(i).coordinateFrame; 
                private attribute trs : TranslationRotationSequence {
                    :>> source = wcf;
                    :>> target = lbcf;
                    :>> elements = new Translation((lbpr*cos((i-1)*lbda), lbpr*sin((i-1)*lbda), -8)[wcf]); 
                }
                lbcf.transformation == trs
            }
        }
    }"#;
    let result = SysMLParser::parse(Rule::part_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse Wheel part def with forAll: {:?}",
        result.err()
    );
}

// =============================================================================
// SysMLv2SpecAnnexA SimpleVehicleModel - transition from initial state
// =============================================================================

/// Tests transition from initial pseudostate
#[test]
fn test_parse_transition_from_initial() {
    let input = "transition initial then off;";
    let result = SysMLParser::parse(Rule::transition_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse transition from initial: {:?}",
        result.err()
    );
}

/// Tests entry action with name
#[test]
fn test_parse_entry_action_initial() {
    let input = "entry action initial;";
    let result = SysMLParser::parse(Rule::entry_action_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse entry action: {:?}",
        result.err()
    );
}

/// Tests multiple bind and interface statements in sequence
#[test]
fn test_parse_bind_and_interface_sequence() {
    let input = r#"
        part vehicle {
            bind engine.fuelCmdPort=fuelCmdPort;
            
            interface engineInterface:EngineInterface
                connect engine.port1 to trans.port2;
            
            interface fuelInterface:FuelInterface
                connect fuelTank.outPort to engine.inPort;
        }
    "#;
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse bind and interface sequence: {:?}",
        result.err()
    );
}

/// Tests bind, interface, and allocate in sequence (from SimpleVehicleModel line ~703)
#[test]
fn test_parse_bind_interface_allocate_sequence() {
    let input = r#"
        part vehicle_b : Vehicle {
            bind engine.fuelCmdPort=fuelCmdPort;

            interface engineToTransmissionInterface:EngineToTransmissionInterface
                connect engine.drivePwrPort to transmission.clutchPort;
        
            interface fuelInterface:FuelInterface
                connect fuelTank.fuelOutPort to engine.fuelInPort;

            allocate ActionTree::providePower.generateToAmplify to engineToTransmissionInterface;
        }
    "#;
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse bind/interface/allocate sequence: {:?}",
        result.err()
    );
}

/// Tests parsing feature chain for allocate
#[test]
fn test_parse_allocate_feature_chain() {
    let input = "ActionTree::providePower.generateToAmplify";
    let result = SysMLParser::parse(Rule::feature_reference, input);
    assert!(
        result.is_ok(),
        "Failed to parse feature_reference: {:?}",
        result.err()
    );
}

/// Tests parsing short feature reference
#[test]
fn test_parse_short_feature_reference() {
    let input = "ActionTree::providePower";
    let result = SysMLParser::parse(Rule::feature_reference, input);
    assert!(
        result.is_ok(),
        "Failed to parse short feature_reference: {:?}",
        result.err()
    );
}

/// Tests parsing connector_end_member for allocate
#[test]
fn test_parse_connector_end_for_allocate() {
    let input = "ActionTree::providePower.generateToAmplify";
    let result = SysMLParser::parse(Rule::connector_end_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse connector_end_member: {:?}",
        result.err()
    );
}

/// Tests parsing allocate_end_member
#[test]
fn test_parse_allocate_end_member() {
    let input = "ActionTree::providePower.generateToAmplify";
    let result = SysMLParser::parse(Rule::allocate_end_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse allocate_end_member: {:?}",
        result.err()
    );
}

/// Tests binary allocate with simple identifiers
#[test]
fn test_parse_binary_allocate_simple() {
    let input = "a to b";
    let result = SysMLParser::parse(Rule::binary_allocate_part, input);
    assert!(
        result.is_ok(),
        "Failed to parse simple binary_allocate_part: {:?}",
        result.err()
    );
}

/// Tests the exact allocate statement from line 705 of SimpleVehicleModel
#[test]
fn test_parse_allocate_from_line_705() {
    let input =
        "allocate ActionTree::providePower.generateToAmplify to engineToTransmissionInterface;";
    let result = SysMLParser::parse(Rule::allocate_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse allocate from line 705: {:?}",
        result.err()
    );
}

/// Tests allocate_end directly
#[test]
fn test_parse_allocate_end() {
    let input = "ActionTree::providePower.generateToAmplify";
    let result = SysMLParser::parse(Rule::allocate_end, input);
    assert!(
        result.is_ok(),
        "Failed to parse allocate_end: {:?}",
        result.err()
    );
}

/// Tests allocate from AllocationTest.sysml line 30 with nested feature access
#[test]
fn test_parse_allocate_nested_features() {
    let input = "allocate l.component to p.assembly.element;";
    let result = SysMLParser::parse(Rule::allocate_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse allocate with nested features: {:?}",
        result.err()
    );
}

/// Tests nary allocate with named ends using subsetting operator
#[test]
fn test_parse_nary_allocate_with_subsetting() {
    let input = "allocate (logical ::> l, physical ::> p);";
    let result = SysMLParser::parse(Rule::allocate_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse nary allocate with subsetting: {:?}",
        result.err()
    );
}

/// Tests allocate binary part
#[test]
fn test_parse_binary_allocate_part() {
    let input = "ActionTree::providePower.generateToAmplify to engineInterface";
    let result = SysMLParser::parse(Rule::binary_allocate_part, input);
    assert!(
        result.is_ok(),
        "Failed to parse binary_allocate_part: {:?}",
        result.err()
    );
}

/// Tests allocate with qualified name and feature chain
#[test]
fn test_parse_allocate_with_qualified_feature_chain() {
    let input = "allocate ActionTree::providePower.generateToAmplify to engineInterface;";
    let result = SysMLParser::parse(Rule::allocate_keyword_part, input);
    assert!(
        result.is_ok(),
        "Failed to parse allocate with qualified name + feature chain: {:?}",
        result.err()
    );
}

/// Tests interface with connect on indented next line (no braces)
#[test]
fn test_parse_interface_implicit_body() {
    let input = "interface fuelInterface:FuelInterface\n                        connect fuelTank.fuelOutPort to engine.fuelInPort;";
    let result = SysMLParser::parse(Rule::interface_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse interface with implicit body: {:?}",
        result.err()
    );
}

/// Tests dependency with prefix metadata
#[test]
fn test_parse_dependency_with_metadata() {
    let input = "#refinement dependency engine4Cyl to VehicleConfiguration_b::PartsTree::vehicle_b::engine;";
    let result = SysMLParser::parse(Rule::dependency, input);
    assert!(
        result.is_ok(),
        "Failed to parse dependency with metadata: {:?}",
        result.err()
    );
}

/// Tests dependency within package body
#[test]
fn test_parse_dependency_in_package() {
    let input = r#"package Test {
    #refinement dependency engine4Cyl to VehicleConfiguration_b::PartsTree::vehicle_b::engine;
}"#;
    let result = SysMLParser::parse(Rule::package_declaration, input);
    assert!(
        result.is_ok(),
        "Failed to parse dependency in package: {:?}",
        result.err()
    );
}

/// Tests interface declaration with connect keyword
#[test]
fn test_parse_interface_decl_with_connect_v2() {
    let input = "fuelInterface:FuelInterface connect fuelTank.fuelOutPort to engine.fuelInPort";
    let result = SysMLParser::parse(Rule::interface_usage_declaration, input);
    assert!(
        result.is_ok(),
        "Failed to parse interface declaration with connect: {:?}",
        result.err()
    );
}

/// Tests metadata usage with about clause and qualified name
#[test]
fn test_parse_metadata_with_about_qualified() {
    let input = r#"@Rationale about engineTradeOffAnalysis::vehicle_b_engine4cyl{
        explanation = VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis;          
        text = "the engine4cyl was evaluated";
    }"#;
    let result = SysMLParser::parse(Rule::metadata_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse metadata with about and qualified name: {:?}",
        result.err()
    );
}

/// Tests part with typed-by redefinition and multiplicity
#[test]
fn test_parse_part_typed_by_redefinition_with_multiplicity() {
    let input = "part cylinders :>> cylinders [4];";
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse part with :>> and multiplicity: {:?}",
        result.err()
    );
}

/// Tests attribute with redefines and unit
#[test]
fn test_parse_attribute_redefines_with_unit() {
    let input = "attribute mass redefines mass=180 [kg];";
    let result = SysMLParser::parse(Rule::attribute_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse attribute with redefines and unit: {:?}",
        result.err()
    );
}

/// Tests analysis with subject and parts from line 1165-1172
#[test]
fn test_parse_analysis_from_line_1165() {
    let input = r#"analysis engineTradeOffAnalysis:TradeStudy{
        subject vehicleAlternatives[2]:>vehicle_b;   
        
        part vehicle_b_engine4cyl:>vehicleAlternatives{   
            part engine redefines engine{
                part cylinders :>> cylinders [4];
                attribute mass redefines mass=180 [kg];
            }
        }
    }"#;
    let result = SysMLParser::parse(Rule::analysis_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse analysis from line 1165: {:?}",
        result.err()
    );
}

/// Tests parsing part body with two interfaces (from actual file)
#[test]
fn test_parse_part_with_two_interfaces() {
    let input = r#"part vehicle {
                    interface engineToTransmissionInterface:EngineToTransmissionInterface
                        connect engine.drivePwrPort to transmission.clutchPort;
                
                    interface fuelInterface:FuelInterface
                        connect fuelTank.fuelOutPort to engine.fuelInPort;
                }"#;
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse part with two interfaces: {:?}",
        result.err()
    );
}

/// Tests larger context from actual file (lines 690-715)
#[test]
fn test_parse_full_context_with_interfaces() {
    let input = r#"part vehicle {
                        part seatBelt[2] {@Safety{isMandatory = true;}}
                        part frontSeat[2];
                        part driverAirBag {@Safety{isMandatory = false;}}
                    }
                    
                    //connections
                    bind engine.fuelCmdPort=fuelCmdPort;

                    interface engineToTransmissionInterface:EngineToTransmissionInterface
                        connect engine.drivePwrPort to transmission.clutchPort;
                
                    interface fuelInterface:FuelInterface
                        connect fuelTank.fuelOutPort to engine.fuelInPort;

                    allocate ActionTree::providePower.generateToAmplify to engineToTransmissionInterface;
                    
                    bind engine.ignitionCmdPort=ignitionCmdPort;
                    connect starterMotor.gearPort to engine.flyWheelPort;
                    connect vehicleSoftware.vehicleController.controlPort to engine.engineControlPort;
                    bind vehicle_b.setSpeedPort = vehicleSoftware.vehicleController.cruiseController.setSpeedPort;
                    connect speedSensor.speedSensorPort to vehicleSoftware.vehicleController.cruiseController.speedSensorPort;
                    bind vehicleSoftware.vehicleController.cruiseController.cruiseControlPort = vehicleSoftware.vehicleController.controlPort;
                    connect transmission.shaftPort_a to driveshaft.shaftPort_b; 
                    connect driveshaft.shaftPort_c to rearAxleAssembly.shaftPort_d;
                    bind rearAxleAssembly.rearWheel1.wheelToRoadPort=vehicleToRoadPort.wheelToRoadPort1;
                    bind rearAxleAssembly.rearWheel2.wheelToRoadPort=vehicleToRoadPort.wheelToRoadPort2;
    }"#;
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse full context with interfaces: {:?}",
        result.err()
    );
}

// Test for AllocationTest.sysml line 24-27: allocation def with end features
#[test]
fn test_parse_allocation_def_with_end_features() {
    let input = r#"allocation def Logical_to_Physical :> A {
        end logical : Logical;
        end physical : Physical;
    }"#;
    let result = SysMLParser::parse(Rule::allocation_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse allocation def with end features: {:?}",
        result.err()
    );
}

// Test for SimpleVehicleModel.sysml line 925-938: #refinement dependency
#[test]
fn test_parse_refinement_dependency() {
    let input = r#"package Engine4Cyl_Variant{
            public import ModelingMetadata::*;
            part engine:Engine{
                part cylinders:Cylinder[4..8] ordered;
            }
            part engine4Cyl:>engine{
                part redefines cylinders [4];
                part cylinder1 subsets cylinders[1];
                part cylinder2 subsets cylinders[1];
                part cylinder3 subsets cylinders[1];
                part cylinder4 subsets cylinders[1];
            }
            #refinement dependency engine4Cyl to VehicleConfiguration_b::PartsTree::vehicle_b::engine;
        }"#;
    let result = SysMLParser::parse(Rule::package, input);
    assert!(
        result.is_ok(),
        "Failed to parse refinement dependency: {:?}",
        result.err()
    );
}

// Test for SimpleVehicleModel.sysml line 1117-1124: @Rationale about qualified::name
#[test]
fn test_parse_metadata_about_qualified_name() {
    let input = r#"package VehicleTradeOffAnalysis{
            @Rationale about engineTradeOffAnalysis::vehicle_b_engine4cyl{
                explanation = VehicleAnalysis::VehicleTradeOffAnalysis::engineTradeOffAnalysis;          
                text = "the engine4cyl was evaluated to have a higher objective function compared to the engine6cyl based on the trade-off analyiss"; 
            }
        }"#;
    let result = SysMLParser::parse(Rule::package, input);
    assert!(
        result.is_ok(),
        "Failed to parse metadata about qualified name: {:?}",
        result.err()
    );
}

// Test for SimpleVehicleModel.sysml line 1168-1176: redefines with inline value
#[test]
fn test_parse_redefines_with_inline_value() {
    let input = r#"part vehicle_b_engine4cyl:>vehicleAlternatives{   
                    part engine redefines engine{
                        part cylinders :>> cylinders [4];
                        attribute mass redefines mass=180 [kg];
                        attribute peakHorsePower redefines peakHorsePower = 180 [W];
                        attribute fuelEfficiency redefines fuelEfficiency=.6;
                        attribute cost redefines cost = 1000;                     
                    }
                }"#;
    let result = SysMLParser::parse(Rule::part_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse redefines with inline value: {:?}",
        result.err()
    );
}

// Test for PartTest.sysml line 6-7: part with quoted short name
#[test]
fn test_parse_part_with_quoted_short_name() {
    let input = r#"part def A {
		part <'1'> b: B;
	}"#;
    let result = SysMLParser::parse(Rule::part_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse part with quoted short name: {:?}",
        result.err()
    );
}

// Test for PartTest.sysml: public part def A with full body
#[test]
fn test_parse_part_test_file() {
    let input = r#"package PartTest {
	
	part f: A;

	public part def A {
		part <'1'> b: B;
		protected port c: C;
		constant attribute x[0..2];
		derived constant ref attribute y :> x;
		ref z : ScalarValues::Integer;
	}
}"#;
    let result = SysMLParser::parse(Rule::package, input);
    assert!(
        result.is_ok(),
        "Failed to parse PartTest file: {:?}",
        result.err()
    );
}

// Test for SimpleVehicleModel.sysml line 1215-1237: verification with actions
#[test]
fn test_parse_verification_with_actions() {
    let input = r#"verification massTests:MassTest {
                subject vehicle_uut :> vehicle_b;
                actor vehicleVerificationSubSystem_1 = verificationContext.massVerificationSystem;
                objective {
                    verify vehicleSpecification.vehicleMassRequirement{
                        redefines massActual=weighVehicle.massMeasured;
                    }
                }     
               @ VerificationMethod{
                    kind = (VerificationMethodKind::test, VerificationMethodKind::analyze);
                }
                action weighVehicle {
                    out massMeasured:>ISQ::mass;
                }
                then action evaluatePassFail {
                    in massMeasured:>ISQ::mass;
                    out verdict = PassIf(vehicleSpecification.vehicleMassRequirement(vehicle_uut));
                }
                flow from weighVehicle.massMeasured to evaluatePassFail.massMeasured;
                return :>> verdict = evaluatePassFail.verdict;
            }"#;
    let result = SysMLParser::parse(Rule::verification_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse verification with actions: {:?}",
        result.err()
    );
}

// Test for SimpleVehicleModel.sysml line 1232: nested function call
#[test]
fn test_parse_nested_function_call() {
    let input = "PassIf(vehicleSpecification.vehicleMassRequirement(vehicle_uut))";
    let result = SysMLParser::parse(Rule::owned_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse nested function call: {:?}",
        result.err()
    );
}

// Test for SimpleVehicleModel.sysml line 1232: out parameter with nested function call
#[test]
fn test_parse_out_param_nested_function_call() {
    let input = "out verdict = PassIf(vehicleSpecification.vehicleMassRequirement(vehicle_uut));";
    let result = SysMLParser::parse(Rule::directed_parameter_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse out param with nested function call: {:?}",
        result.err()
    );
}

// Test parsing the feature_value part
#[test]
fn test_parse_feature_value_nested_call() {
    let input = "= PassIf(vehicleSpecification.vehicleMassRequirement(vehicle_uut))";
    let result = SysMLParser::parse(Rule::feature_value, input);
    assert!(
        result.is_ok(),
        "Failed to parse feature_value with nested call: {:?}",
        result.err()
    );
}

// Test simple directed parameter
#[test]
fn test_parse_simple_directed_parameter() {
    let input = "out verdict = 5;";
    let result = SysMLParser::parse(Rule::directed_parameter_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse simple directed parameter: {:?}",
        result.err()
    );
}

// Test directed parameter with function call
#[test]
fn test_parse_directed_parameter_func_call() {
    let input = "out verdict = foo(x);";
    let result = SysMLParser::parse(Rule::directed_parameter_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse directed parameter with func call: {:?}",
        result.err()
    );
}

// Test directed parameter with dotted function call
#[test]
fn test_parse_directed_parameter_dotted_func_call() {
    let input = "out verdict = foo.bar(x);";
    let result = SysMLParser::parse(Rule::directed_parameter_member, input);
    assert!(
        result.is_ok(),
        "Failed to parse directed parameter with dotted func call: {:?}",
        result.err()
    );
}

// Test expression: foo.bar(x)
#[test]
fn test_parse_expression_dotted_func_call() {
    let input = "foo.bar(x)";
    let result = SysMLParser::parse(Rule::owned_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse dotted func call expression: {:?}",
        result.err()
    );
}

// Test use case usage inside package body
#[test]
fn test_parse_use_case_usage() {
    let input = "use case transportPassenger:TransportPassenger;";
    let result = SysMLParser::parse(Rule::use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse use case usage: {:?}",
        result.err()
    );
}

// Test use case usage in package body
#[test]
fn test_parse_use_case_in_package() {
    let input = "package P { use case transportPassenger:TransportPassenger; }";
    let result = SysMLParser::parse(Rule::package, input);
    assert!(
        result.is_ok(),
        "Failed to parse use case in package: {:?}",
        result.err()
    );
}

// Test action trigger with accept
#[test]
fn test_parse_action_trigger_accept() {
    let input = "then action trigger accept ignitionCmd:IgnitionCmd;";
    let result = SysMLParser::parse(Rule::action_body_item, input);
    assert!(
        result.is_ok(),
        "Failed to parse action trigger accept: {:?}",
        result.err()
    );
}

// Test use case with trigger accept inside
#[test]
fn test_parse_use_case_with_trigger_accept() {
    let input = "use case transportPassenger:TransportPassenger{
                first start;
                then action trigger accept ignitionCmd:IgnitionCmd;
            }";
    let result = SysMLParser::parse(Rule::use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse use case with trigger accept: {:?}",
        result.err()
    );
}

// Test "then done;" pattern in use case
#[test]
fn test_parse_then_done() {
    let input = "use case test:TestCase{
                first start;
                then done;
            }";
    let result = SysMLParser::parse(Rule::use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse then done: {:?}",
        result.err()
    );
}

// Test full use case with multiple then actions
#[test]
fn test_parse_use_case_full_flow() {
    let input = r#"use case transportPassenger:TransportPassenger{
                first start;
                then action a{
                    action driverGetInVehicle;
                }
                then action trigger accept ignitionCmd:IgnitionCmd;
                then action b{
                    action driveVehicleToDestination;
                }
                then action c{
                    action driverGetOutOfVehicle;
                }
                then done;
            }"#;
    let result = SysMLParser::parse(Rule::use_case_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse use case full flow: {:?}",
        result.err()
    );
}

#[test]
fn test_parse_package_with_nested_elements_issue() {
    use std::path::PathBuf;
    use syster::project::file_loader;
    use syster::syntax::sysml::ast::Element;

    // Use namespace syntax - file-level package declaration
    let input = r#"
package ScalarValues {
    attribute def Real;
}
package TestWithStdlib {

part def Calculator {
    attribute result : ScalarValues::Real;
}

part cal : Calculator;
}"#;

    let path = PathBuf::from("test.sysml");
    let parse_result = file_loader::parse_with_result(input, &path);

    println!("Parse errors: {:?}", parse_result.errors);
    assert!(
        parse_result.content.is_some(),
        "Parse should succeed. Errors: {:?}",
        parse_result.errors
    );

    let language_file = parse_result.content.expect("Parse should succeed");
    let syster::syntax::SyntaxFile::SysML(file) = language_file else {
        panic!("Expected SysML file");
    };

    println!(
        "file.namespace: {:?}",
        file.namespace.as_ref().map(|n| &n.name)
    );
    println!("file.elements.len: {}", file.elements.len());
    for (i, el) in file.elements.iter().enumerate() {
        match el {
            Element::Package(p) => println!("element[{}] = Package({:?})", i, p.name),
            Element::Definition(d) => println!("element[{}] = Definition({:?})", i, d.name),
            Element::Usage(u) => println!("element[{}] = Usage({:?})", i, u.name),
            Element::Comment(_) => println!("element[{i}] = Comment"),
            Element::Import(_) => println!("element[{i}] = Import"),
            Element::Alias(_) => println!("element[{i}] = Alias"),
        }
    }

    // Should have 2 top-level packages
    assert_eq!(file.elements.len(), 2, "Should have 2 top-level packages");

    // First package is ScalarValues
    let Element::Package(scalar_pkg) = &file.elements[0] else {
        panic!("Expected Package, got {:?}", file.elements[0]);
    };
    assert_eq!(scalar_pkg.name, Some("ScalarValues".to_string()));

    // Second package is TestWithStdlib
    let Element::Package(test_pkg) = &file.elements[1] else {
        panic!("Expected Package, got {:?}", file.elements[1]);
    };
    assert_eq!(test_pkg.name, Some("TestWithStdlib".to_string()));

    // TestWithStdlib should have 2 elements: part def Calculator and part cal
    assert_eq!(
        test_pkg.elements.len(),
        2,
        "TestWithStdlib should have 2 elements (definition and usage)"
    );

    // First element should be the part def
    let Element::Definition(def) = &test_pkg.elements[0] else {
        panic!("Expected Definition, got {:?}", test_pkg.elements[0]);
    };
    assert_eq!(def.name, Some("Calculator".to_string()));

    // Second element should be the part usage
    let Element::Usage(usage) = &test_pkg.elements[1] else {
        panic!("Expected Usage, got {:?}", test_pkg.elements[1]);
    };
    assert_eq!(usage.name, Some("cal".to_string()));
}

// Satisfy Statement with `by` Clause
#[rstest]
#[case(
    "satisfy Requirements::REQ1 by RoboticVacuumCleaner::battery;",
    "qualified name by clause"
)]
#[case("satisfy req by system.component.feature;", "feature chain by clause")]
#[case("assert satisfy r by q;", "assert satisfy")]
#[case("not satisfy r1 by p;", "not satisfy")]
#[case("assert not satisfy r1 by q;", "assert not satisfy")]
fn test_parse_satisfy_by_clause(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::satisfy_requirement_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Short Names using angle bracket syntax
#[rstest]
#[case("<'1'>", "angle bracket short name with quoted")]
#[case("<myId>", "angle bracket with identifier")]
fn test_parse_short_name_variants(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::short_name, input);
    assert!(
        result.is_ok(),
        "Failed to parse short_name {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "requirement def <'REQ-1'> SafetyReq { }",
    "requirement def with short name"
)]
fn test_parse_definition_with_short_name(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::requirement_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Measurement References `@[unit]`
#[rstest]
#[case("5@[SI::kg]", "integer with unit")]
#[case("120.5@[SI::min]", "decimal with unit")]
#[case("0@[ISQ::Length::meter]", "nested qualified unit")]
#[case("3.14159@[SI::rad]", "pi with radians")]
fn test_parse_measurement_reference(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::primary_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse measurement {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("mass <= 5@[SI::kg]", "comparison with unit")]
#[case("speed >= 100@[SI::m] / 1@[SI::s]", "expression with units")]
fn test_parse_measurement_in_expression(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::conditional_expression, input);
    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

// Interface End with Crosses Operator `=>`
#[rstest]
#[case("supplierPort => Source::output", "crosses with qualified name")]
fn test_parse_interface_end_crosses(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_end, input);
    assert!(
        result.is_ok(),
        "Failed to parse interface end {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "interface connect supplierPort => Source::FEnergy to consumerPort => Consumer::FEnergy;",
    "connect with crosses"
)]
fn test_parse_interface_with_crosses(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::interface_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse interface {}: {:?}",
        desc,
        result.err()
    );
}

// Function Definition with Parameters and Return Types
#[rstest]
#[case("(in a, out b, inout c)", "direction prefixes")]
#[case("(current:>SI::A, voltage:>ISQ::voltage)", "subsetting parameters")]
#[case("(x, y, z)", "simple identifiers")]
#[case("(in 'input data', out 'output data')", "quoted parameter names")]
fn test_parse_function_parameter_list(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::function_parameter_list, input);
    assert!(
        result.is_ok(),
        "Failed to parse parameter list {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(
    "calc def Torque (current:>SI::A, voltage:>ISQ::voltage):> ISQ::torque { }",
    "params with return type"
)]
#[case("calc def Compute (in x, in y, out result) { }", "in/out params")]
#[case("calc def Simple :> BaseCalc;", "specialization no params")]
#[case("calc def Mixed (in x) :> BaseCalc { }", "params and specialization")]
fn test_parse_calc_definition_variants(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::calculation_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse calc def {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case("action def Process (in data, out result) { }", "with params")]
#[case("action def Drive :> Action;", "specialization no params")]
#[case(
    "action def Combined (in x) :> BaseAction { }",
    "params and specialization"
)]
fn test_parse_action_definition_variants(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_definition, input);
    assert!(
        result.is_ok(),
        "Failed to parse action def {}: {:?}",
        desc,
        result.err()
    );
}

// Action Usage with Parameters
#[rstest]
#[case(
    "action convertEnergy:ConvertEnergy (in current, in voltage, out energy);",
    "typed with params"
)]
#[case("action doSomething;", "simple no params")]
#[case("action compute (in x, out y);", "with params no type")]
fn test_parse_action_usage_variants(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_usage, input);
    assert!(
        result.is_ok(),
        "Failed to parse action usage {}: {:?}",
        desc,
        result.err()
    );
}

/// Tests expose rule with namespace patterns (::*)
/// Regression test: namespace_expose must be tried before membership_expose
/// in the expose rule, otherwise qualified_name in membership_expose will
/// greedily consume the namespace prefix leaving ::* unconsumed
#[rstest]
#[case("expose MyNamespace::*;", "namespace expose with wildcard")]
#[case("expose A::B::*;", "qualified namespace expose")]
#[case("expose MyPackage::*::**;", "namespace expose with recursive")]
#[case("expose MyElement;", "simple membership expose")]
#[case("expose A::B::C;", "qualified membership expose")]
fn test_expose_namespace_before_membership(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::expose, input);
    assert!(
        result.is_ok(),
        "Failed to parse expose '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
    // Verify full consumption
    let pairs = result.unwrap();
    let consumed: String = pairs.map(|p| p.as_str()).collect();
    assert_eq!(
        consumed, input,
        "expose only consumed '{consumed}', expected '{input}'"
    );
}
