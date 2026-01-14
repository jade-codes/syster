#![allow(clippy::unwrap_used)]

use pest::Parser;
use rstest::rstest;
use syster::parser::{SysMLParser, sysml::Rule};

/// Helper function to assert that parsing succeeds and the entire input is consumed.
/// This ensures the parser doesn't just match a prefix of the input.
///
/// The function verifies that:
/// 1. Parsing succeeds
/// 2. Exactly one top-level pair is produced (in most cases)
/// 3. The parsed output matches the original input exactly
fn assert_round_trip(rule: Rule, input: &str, desc: &str) {
    let result =
        SysMLParser::parse(rule, input).unwrap_or_else(|e| panic!("Failed to parse {desc}: {e}"));

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

/// Helper function to assert that parsing succeeds.
/// Use this for tests where the parser may not consume the entire input.
fn assert_parse_succeeds(rule: Rule, input: &str, desc: &str) {
    let result = SysMLParser::parse(rule, input);
    assert!(
        result.is_ok(),
        "Failed to parse {}: {:?}",
        desc,
        result.err()
    );
}

#[rstest]
#[case(Rule::identifier, "myVar", "simple identifier")]
fn test_parse_simple_identifier(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
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
    assert_round_trip(Rule::keyword, keyword, keyword);
}

#[rstest]
#[case(Rule::line_comment, "// this is a comment", "line comment")]
#[case(Rule::block_comment, "/* block comment */", "block comment")]
#[case(
    Rule::block_comment,
    "/* line 1\nline 2\nline 3 */",
    "multiline block comment"
)]
fn test_parse_comments(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::file, "", "empty file")]
#[case(Rule::file, "   \n\t  \r\n  ", "file with whitespace")]
fn test_parse_file(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Control Node Tests

#[rstest]
#[case(Rule::fork_node, "fork;", "fork node")]
#[case(Rule::fork_node, "fork myFork;", "fork with name")]
#[case(Rule::merge_node, "merge;", "merge node")]
#[case(Rule::merge_node, "merge myMerge;", "merge with name")]
#[case(Rule::join_node, "join;", "join node")]
#[case(Rule::join_node, "join myJoin;", "join with name")]
#[case(Rule::decision_node, "decide;", "decision node")]
#[case(Rule::decision_node, "decide myDecision;", "decision with name")]
fn test_parse_control_nodes(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// State Subaction Membership Tests
#[rstest]
#[case(
    Rule::state_subaction_membership,
    "entry myEntryAction;",
    "entry action"
)]
#[case(Rule::state_subaction_membership, "exit myExitAction;", "exit action")]
#[case(Rule::state_subaction_membership, "do myDoAction;", "do action")]
fn test_parse_state_subaction(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Transition Feature Membership Tests
#[rstest]
#[case(
    Rule::transition_feature_membership,
    "accept myAcceptFeature;",
    "accept feature"
)]
#[case(Rule::transition_feature_membership, "if myCondition;", "if feature")]
fn test_parse_transition_feature(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Parameter Membership Tests

#[rstest]
#[case(Rule::subject_membership, "subject mySubject;", "subject membership")]
#[case(Rule::actor_membership, "actor myActor;", "actor membership")]
#[case(
    Rule::stakeholder_membership,
    "stakeholder myStakeholder;",
    "stakeholder membership"
)]
#[case(
    Rule::objective_membership,
    "objective myObjective;",
    "objective membership"
)]
fn test_parse_parameter_memberships(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Succession and Expose Tests

#[rstest]
#[case(
    Rule::succession_as_usage,
    "first source then target;",
    "simple succession"
)]
#[case(
    Rule::succession_as_usage,
    "first source then target { }",
    "succession with body"
)]
#[case(
    Rule::succession_as_usage,
    "succession mySuccession first source then target;",
    "succession with declaration"
)]
fn test_parse_succession_as_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::succession_keyword, "succession", "succession keyword")]
fn test_parse_succession_keyword(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::expose, "expose MyElement;", "expose")]
#[case(
    Rule::expose,
    "expose MyElement::member;",
    "membership expose (via expose rule)"
)]
#[case(
    Rule::expose,
    "expose MyNamespace::*;",
    "namespace expose (via expose rule)"
)]
#[case(
    Rule::membership_expose,
    "expose MyElement::member",
    "membership expose sub-rule"
)]
#[case(
    Rule::namespace_expose,
    "expose MyNamespace::*",
    "namespace expose sub-rule"
)]
fn test_parse_expose(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Requirement Constraint Memberships

#[rstest]
#[case(
    Rule::requirement_constraint_membership,
    "require myConstraint;",
    "requirement constraint membership"
)]
#[case(
    Rule::framed_concern_membership,
    "frame myConcern;",
    "framed concern membership"
)]
#[case(
    Rule::requirement_verification_membership,
    "verify myVerification;",
    "requirement verification membership"
)]
fn test_parse_requirement_memberships(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Port and Conjugation Tests

#[rstest]
#[case(
    Rule::owned_feature_typing,
    "~MyPort",
    "conjugated port type reference"
)]
#[case(Rule::variant_membership, "variant myVariant;", "variant membership")]
fn test_parse_port_and_variant(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Terminate Action

#[rstest]
#[case(
    Rule::terminate_action_usage,
    "terminate myOccurrence;",
    "terminate action"
)]
fn test_parse_terminate_action(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Port Definition and Conjugation Tests

// Updated to match official SysML v2 grammar structure:
// - Full port usage with conjugated type uses port_usage rule
#[rstest]
#[case(
    Rule::port_usage,
    "port myPort : ~ConjugatedPortType;",
    "port usage with conjugated type"
)]
fn test_parse_conjugated_port_definitions(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Token Tests

#[rstest]
#[case(Rule::defined_by_token, ":", "colon")]
#[case(Rule::defined_by_token, "defined by", "defined by keyword")]
fn test_parse_defined_by_token(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Enum Tests

#[rstest]
#[case(Rule::portion_kind, "timeslice", "timeslice")]
#[case(Rule::portion_kind, "snapshot", "snapshot")]
fn test_parse_portion_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::requirement_constraint_kind, "assume", "assume")]
#[case(Rule::requirement_constraint_kind, "require", "require")]
fn test_parse_requirement_constraint_kind(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Fragment Tests

#[rstest]
#[case(Rule::variation_token, "variation", "variation marker")]
#[case(Rule::individual_marker, "individual", "individual marker")]
fn test_parse_markers(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Model Entry Point Tests

#[rstest]
#[case(Rule::model, "", "empty model")]
#[case(Rule::model, "package MyPackage;", "model with simple package")]
#[case(
    Rule::model,
    "library package MyLibrary;",
    "model with library package"
)]
#[case(
    Rule::model,
    "standard library package MyLibrary;",
    "model with standard library package"
)]
#[case(
    Rule::model,
    "package Pkg1; package Pkg2;",
    "model with multiple packages"
)]
#[case(
    Rule::model,
    "package MyPackage { part myPart; }",
    "model with package containing usage"
)]
fn test_parse_model(#[case] rule: Rule, #[case] input: &str, #[case] description: &str) {
    assert_round_trip(rule, input, description);
}

// Dependency Tests

#[rstest]
#[case(Rule::dependency, "dependency from A to B;", "simple")]
#[case(Rule::dependency, "dependency A to B;", "without from")]
#[case(
    Rule::dependency,
    "dependency 'Service Layer' to 'Data Layer' { }",
    "with body"
)]
#[case(Rule::dependency, "dependency from A, B, C to D;", "multiple clients")]
#[case(
    Rule::dependency,
    "dependency from A to B, C, D;",
    "multiple suppliers"
)]
#[case(
    Rule::dependency,
    "dependency myDep from A to B;",
    "with identification"
)]
#[case(
    Rule::dependency,
    "dependency from A to B { comment MyComment; }",
    "with comment in body"
)]
fn test_parse_dependency(#[case] rule: Rule, #[case] input: &str, #[case] description: &str) {
    assert_round_trip(rule, input, description);
}

// Annotation Tests

#[rstest]
#[case(
    Rule::comment_annotation,
    "comment MyComment about MyElement;",
    "comment"
)]
#[case(Rule::documentation, "doc MyDoc;", "documentation")]
#[case(
    Rule::textual_representation,
    r#"rep language "Python" /* code */"#,
    "textual representation"
)]
#[case(Rule::metadata_usage_annotation, "#MyMetadata;", "metadata usage")]
#[case(Rule::annotating_element, "comment MyComment;", "annotating element")]
#[case(
    Rule::relationship_body,
    "{ comment MyComment; }",
    "relationship body with annotation"
)]
fn test_parse_annotations(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::comment_annotation,
    r#"comment locale "en-US" /* comment text */"#,
    "comment with locale"
)]
#[case(
    Rule::comment_annotation,
    r#"comment MyComment locale "fr-FR" /* texte */"#,
    "comment with name and locale"
)]
#[case(Rule::comment_annotation, r#"comment about Foo;"#, "comment about")]
#[case(
    Rule::comment_annotation,
    r#"comment about Foo, Bar;"#,
    "comment about multiple"
)]
#[case(
    Rule::comment_annotation,
    r#"comment MyComment about Foo, Bar /* about multiple */"#,
    "comment named about multiple"
)]
#[case(
    Rule::comment_annotation,
    r#"comment locale "en-US" about Foo;"#,
    "comment locale about"
)]
fn test_parse_comment_variants(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(
    Rule::documentation,
    r#"doc locale "en-US" /* docs */"#,
    "doc with locale"
)]
#[case(
    Rule::documentation,
    r#"doc MyDoc locale "ja-JP" /* text */"#,
    "doc with name and locale"
)]
#[case(Rule::documentation, r#"doc /* inline doc */"#, "doc inline")]
#[case(Rule::documentation, r#"doc;"#, "doc semicolon")]
fn test_parse_documentation_variants(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(
    Rule::dependency,
    "dependency from A to B { comment MyComment; }",
    "dependency with comment in body"
)]
fn test_parse_dependency_with_comment_in_body(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Metadata Tests

#[rstest]
#[case(
    Rule::metadata_definition,
    "metadata def MyMetadata;",
    "simple metadata definition"
)]
#[case(
    Rule::metadata_definition,
    "abstract metadata def MyMetadata;",
    "abstract metadata definition"
)]
#[case(Rule::prefix_metadata_usage, "#MyMetadata", "prefix metadata usage")]
#[case(Rule::metadata_usage, "metadata MyMetadata;", "simple metadata usage")]
#[case(Rule::metadata_usage, "@MyMetadata;", "metadata usage with @")]
#[case(
    Rule::metadata_usage,
    "metadata MyMetadata about A, B;",
    "metadata usage with about"
)]
#[case(
    Rule::metadata_usage,
    "metadata myMeta : MyMetadata;",
    "metadata usage with defined by"
)]
#[case(
    Rule::metadata_usage,
    "metadata MyMetadata { }",
    "metadata usage with body"
)]
#[case(
    Rule::metadata_body_usage,
    "ref :>> MyReference;",
    "metadata body usage"
)]
fn test_parse_metadata(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Package Tests

#[rstest]
#[case(Rule::package, "package MyPackage;", "simple package")]
#[case(Rule::package, "package MyPackage { }", "package with body")]
#[case(Rule::package, "package;", "package without name")]
#[case(Rule::library_package, "library package MyLibrary;", "library package")]
#[case(
    Rule::library_package,
    "standard library package MyLibrary;",
    "standard library package"
)]
fn test_parse_package(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::case_definition,
    r#"abstract case def Case {
                subject subj : Anything[1] { }
                objective obj : RequirementCheck[1] {
                        subject subj default Case::result;
                }
        }"#,
    "Cases.sysml fragment"
)]
#[case(
    Rule::case_definition,
    r#"case def Case {
    objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }
}"#,
    "simplified case with objective"
)]
#[case(
    Rule::requirement_body,
    r#"{
        subject subj default Case::result;
    }"#,
    "requirement_body with subject"
)]
#[case(
    Rule::objective_member,
    r#"objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }"#,
    "objective_member in case body"
)]
#[case(
    Rule::case_body,
    r#"{
    objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }
}"#,
    "case_body with objective"
)]
#[case(
    Rule::case_body_item,
    r#"objective obj : RequirementCheck[1] {
        subject subj default Case::result;
    }"#,
    "objective as case_body_item"
)]
#[case(Rule::case_body, "{objective obj{subject subj;}}", "minimal case_body")]
fn test_parse_case_and_requirement_patterns(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::state_usage, "ref state myState;", "ref state usage")]
#[case(
    Rule::state_usage,
    "abstract ref state exhibitedStates: StateAction[0..*] { }",
    "abstract ref state usage"
)]
#[case(
    Rule::definition_body_item,
    "abstract ref state exhibitedStates: StateAction[0..*] { }",
    "ref state as definition_body_item"
)]
#[case(
    Rule::state_usage,
    r#"abstract ref state exhibitedStates: StateAction[0..*] {
        doc
        /*
         * StateActions that are exhibited by this Part.
         */
    }"#,
    "state with doc comment"
)]
fn test_parse_state_patterns(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::assert_constraint_usage,
    r#"assert constraint {
        doc
        /*
         * Test constraint
         */
        innerSpaceDimension == value
    }"#,
    "constraint with doc comment"
)]
fn test_parse_constraint_patterns(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Member Tests

#[rstest]
#[case(Rule::usage_member, "part myPart;", "usage member")]
#[case(
    Rule::usage_member,
    "public part myPart;",
    "usage member with visibility"
)]
#[case(
    Rule::element_filter_member,
    "filter myExpression;",
    "element filter member"
)]
#[case(
    Rule::alias_member_element,
    "alias MyAlias for MyElement;",
    "alias member"
)]
#[case(
    Rule::alias_member_element,
    "private alias MyAlias for MyElement;",
    "alias member with visibility"
)]
fn test_parse_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Import Tests

#[rstest]
#[case(Rule::import, "import MyElement;", "simple import")]
#[case(Rule::import, "public import MyElement;", "import with visibility")]
#[case(Rule::import, "import all MyElement;", "import all")]
#[case(Rule::import, "import MyElement::*;", "import namespace")]
#[case(Rule::import, "import MyElement::*::**;", "import recursive")]
#[case(Rule::import, "import MyElement [condition];", "import with filter")]
#[case(
    Rule::import,
    "import MyElement [filter1][filter2];",
    "import with multiple filters"
)]
#[case(Rule::import, "import MyElement { }", "import with body")]
fn test_parse_import(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Definition Element Tests

#[rstest]
#[case(
    Rule::definition_element,
    "attribute def MyAttribute;",
    "attribute definition"
)]
#[case(Rule::definition_element, "enum def MyEnum;", "enumeration definition")]
#[case(
    Rule::definition_element,
    "occurrence def MyOccurrence;",
    "occurrence definition"
)]
#[case(
    Rule::definition_element,
    "individual def MyIndividual;",
    "individual definition"
)]
#[case(Rule::definition_element, "item def MyItem;", "item definition")]
#[case(Rule::definition_element, "part def MyPart;", "part definition")]
#[case(
    Rule::definition_element,
    "connection def MyConnection;",
    "connection definition"
)]
// Note: flow connection def was removed as it's not part of official SysML v2 grammar
// FlowConnectionDefinition is an internal abstract syntax element
#[case(
    Rule::definition_element,
    "interface def MyInterface;",
    "interface definition"
)]
#[case(
    Rule::definition_element,
    "allocation def MyAllocation;",
    "allocation definition"
)]
#[case(Rule::definition_element, "port def MyPort;", "port definition")]
#[case(Rule::definition_element, "action def MyAction;", "action definition")]
#[case(Rule::definition_element, "calc def MyCalc;", "calculation definition")]
#[case(Rule::definition_element, "state def MyState;", "state definition")]
#[case(
    Rule::definition_element,
    "constraint def MyConstraint;",
    "constraint definition"
)]
#[case(
    Rule::definition_element,
    "requirement def MyRequirement;",
    "requirement definition"
)]
#[case(
    Rule::definition_element,
    "concern def MyConcern;",
    "concern definition"
)]
#[case(Rule::definition_element, "case def MyCase;", "case definition")]
#[case(
    Rule::definition_element,
    "analysis case def MyAnalysisCase;",
    "analysis case definition"
)]
#[case(
    Rule::definition_element,
    "verification case def MyVerificationCase;",
    "verification case definition"
)]
#[case(
    Rule::definition_element,
    "use case def MyUseCase;",
    "use case definition"
)]
#[case(Rule::definition_element, "view def MyView;", "view definition")]
#[case(
    Rule::definition_element,
    "viewpoint def MyViewpoint;",
    "viewpoint definition"
)]
#[case(
    Rule::definition_element,
    "rendering def MyRendering;",
    "rendering definition"
)]
fn test_parse_definition_element(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Usage Element Tests

#[rstest]
#[case(Rule::usage_element, "attribute myAttr;", "attribute usage")]
#[case(Rule::usage_element, "part myPart;", "part usage")]
fn test_parse_usage_element(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Classifier Tests

#[rstest]
#[case(Rule::specializes_token, "specializes", "specializes token")]
#[case(Rule::specializes_operator, ":>", "specializes symbol")]
#[case(Rule::specializes_operator, "specializes", "specializes keyword")]
fn test_parse_specializes(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::classifier_reference, "BaseClass", "simple classifier reference")]
#[case(
    Rule::classifier_reference,
    "'Quoted Classifier'",
    "quoted classifier reference"
)]
fn test_parse_classifier_reference(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_subclassification, "BaseClass", "single subclassification")]
fn test_parse_owned_subclassification(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::subclassification_part, "specializes Base", "single base")]
#[case(Rule::subclassification_part, ":> Base", "single base with symbol")]
#[case(
    Rule::subclassification_part,
    "specializes Base1, Base2",
    "multiple bases"
)]
#[case(
    Rule::subclassification_part,
    ":> Base1, Base2, Base3",
    "multiple bases with symbol"
)]
fn test_parse_subclassification_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Feature Tests

#[rstest]
#[case(Rule::typed_by_token, ":", "colon")]
#[case(Rule::typed_by_token, "typed by", "typed by keyword")]
fn test_parse_typed_by_token(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::subsets_token, "subsets", "subsets token")]
#[case(Rule::subsets_operator, ":>", "subsets symbol")]
#[case(Rule::subsets_operator, "subsets", "subsets keyword")]
fn test_parse_subsets(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::references_token, "references", "references token")]
#[case(Rule::references_operator, "::>", "references symbol")]
#[case(Rule::references_operator, "references", "references keyword")]
fn test_parse_references(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::redefines_token, "redefines", "redefines token")]
#[case(Rule::redefines_operator, ":>>", "redefines symbol")]
#[case(Rule::redefines_operator, "redefines", "redefines keyword")]
fn test_parse_redefines(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_multiplicity, "[1]", "single bound")]
#[case(Rule::owned_multiplicity, "[0..*]", "range with star")]
#[case(Rule::owned_multiplicity, "[1..5]", "numeric range")]
#[case(Rule::owned_multiplicity, "[*]", "unbounded")]
fn test_parse_owned_multiplicity(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
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
#[case(Rule::multiplicity_part, "[1]", "multiplicity only")]
#[case(Rule::multiplicity_part, "[1] ordered", "multiplicity with properties")]
#[case(Rule::multiplicity_part, "ordered", "properties only")]
fn test_parse_multiplicity_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_specialization, ": BaseType", "typed by")]
#[case(Rule::feature_specialization, ":> BaseFeature", "subsets")]
#[case(Rule::feature_specialization, "::> ReferencedFeature", "references")]
#[case(Rule::feature_specialization, ":>> RedefinedFeature", "redefines")]
fn test_parse_feature_specialization(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_specialization_part, ": BaseType", "single typing")]
#[case(
    Rule::feature_specialization_part,
    ": Type1 [1]",
    "typing with multiplicity"
)]
#[case(
    Rule::feature_specialization_part,
    "[0..*] ordered",
    "multiplicity with properties"
)]
#[case(
    Rule::feature_specialization_part,
    ": Type1 [1] :> Base",
    "typing, multiplicity, and subsetting"
)]
fn test_parse_feature_specialization_part(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_reference, "myFeature", "simple feature reference")]
#[case(
    Rule::feature_reference,
    "'Quoted Feature'",
    "quoted feature reference"
)]
fn test_parse_feature_reference(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_feature_chain, "a.b", "simple chain")]
#[case(Rule::owned_feature_chain, "a.b.c", "longer chain")]
#[case(
    Rule::owned_feature_chain,
    "vehicle.engine.cylinder",
    "descriptive chain"
)]
fn test_parse_owned_feature_chain(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_subsetting, "BaseFeature", "feature reference")]
#[case(Rule::owned_subsetting, "a.b.c", "feature chain")]
fn test_parse_owned_subsetting(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_reference_subsetting, "RefFeature", "feature reference")]
#[case(Rule::owned_reference_subsetting, "parent.child", "feature chain")]
fn test_parse_owned_reference_subsetting(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_redefinition, "RedefinedFeature", "feature reference")]
#[case(Rule::owned_redefinition, "base.feature", "feature chain")]
fn test_parse_owned_redefinition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Definition Structure Tests

#[rstest]
#[case(Rule::basic_definition_prefix, "abstract", "abstract marker")]
#[case(Rule::basic_definition_prefix, "variation", "variation marker")]
fn test_parse_basic_definition_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::definition_prefix, "", "empty prefix")]
#[case(Rule::definition_prefix, "abstract", "abstract only")]
#[case(Rule::definition_prefix, "variation", "variation only")]
#[case(Rule::definition_prefix, "#Meta", "with metadata")]
#[case(Rule::definition_prefix, "abstract #Meta", "abstract with metadata")]
fn test_parse_definition_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::definition_suffix, "MyDef;", "simple declaration")]
#[case(Rule::definition_suffix, "MyDef { }", "declaration with body")]
#[case(Rule::definition_suffix, "MyDef :> Base;", "with subclassification")]
#[case(
    Rule::definition_suffix,
    "MyDef :> Base { }",
    "subclassification with body"
)]
fn test_parse_definition_suffix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::definition_declaration, "MyDef", "simple identification")]
#[case(
    Rule::definition_declaration,
    "MyDef :> Base",
    "with subclassification"
)]
#[case(Rule::definition_declaration, ":> Base", "subclassification only")]
fn test_parse_definition_declaration(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::definition_body, ";", "semicolon")]
#[case(Rule::definition_body, "{ }", "empty body")]
fn test_parse_definition_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::definition_member, "part def MyPart;", "part definition")]
#[case(
    Rule::definition_member,
    "attribute def MyAttr;",
    "attribute definition"
)]
fn test_parse_definition_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Usage Structure Tests

#[rstest]
#[case(Rule::constant_token, "constant", "constant")]
#[case(Rule::derived_token, "derived", "derived")]
fn test_parse_usage_modifiers(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_direction_kind, "in", "in direction")]
#[case(Rule::feature_direction_kind, "out", "out direction")]
#[case(Rule::feature_direction_kind, "inout", "inout direction")]
fn test_parse_feature_direction_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::ref_prefix, "", "empty")]
#[case(Rule::ref_prefix, "in", "with direction")]
#[case(Rule::ref_prefix, "abstract", "with abstract")]
#[case(Rule::ref_prefix, "constant", "with constant")]
#[case(Rule::ref_prefix, "derived", "with derived")]
#[case(Rule::ref_prefix, "in abstract constant derived", "all modifiers")]
fn test_parse_ref_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::basic_usage_prefix, "", "without ref")]
#[case(Rule::basic_usage_prefix, "ref", "with ref")]
#[case(Rule::basic_usage_prefix, "in ref", "with direction and ref")]
#[case(Rule::basic_usage_prefix, "constant ref", "with constant and ref")]
fn test_parse_basic_usage_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_value, "= myValue", "assignment")]
#[case(Rule::feature_value, ":= myValue", "initial assignment")]
#[case(Rule::feature_value, "default myValue", "default without assignment")]
#[case(Rule::feature_value, "default = myValue", "default with assignment")]
#[case(
    Rule::feature_value,
    "default := myValue",
    "default with initial assignment"
)]
fn test_parse_feature_value(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::value_part, "= value", "simple value part")]
#[case(Rule::value_part, ":= initialValue", "initial value part")]
fn test_parse_value_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::usage_body, ";", "simple body")]
#[case(Rule::usage_body, "{ }", "empty body")]
fn test_parse_usage_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Reference Usage Tests

#[rstest]
#[case(Rule::reference_usage, "ref myRef;", "simple reference")]
#[case(Rule::reference_usage, "ref myRef { }", "reference with body")]
fn test_parse_reference_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::default_reference_usage,
    "myDefault;",
    "simple default reference"
)]
#[case(Rule::default_reference_usage, "end myEnd;", "end default reference")]
fn test_parse_default_reference_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Body Element Tests

#[rstest]
#[case(
    Rule::non_occurrence_usage_element,
    "attribute myAttr;",
    "attribute usage"
)]
#[case(Rule::non_occurrence_usage_element, "ref myRef;", "reference usage")]
#[case(
    Rule::non_occurrence_usage_element,
    "bind source = target;",
    "binding connector"
)]
fn test_parse_non_occurrence_usage_element(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::occurrence_usage_element, "part myPart;", "part usage")]
#[case(Rule::occurrence_usage_element, "item myItem;", "item usage")]
#[case(Rule::occurrence_usage_element, "action myAction;", "action usage")]
fn test_parse_occurrence_usage_element(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::structure_usage_element, "part myPart;", "part usage")]
#[case(Rule::structure_usage_element, "item myItem;", "item usage")]
#[case(Rule::structure_usage_element, "port myPort;", "port usage")]
#[case(
    Rule::structure_usage_element,
    "connection myConn;",
    "connection usage"
)]
fn test_parse_structure_usage_element(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::behavior_usage_element, "action myAction;", "action usage")]
#[case(Rule::behavior_usage_element, "calc myCalc;", "calculation usage")]
#[case(Rule::behavior_usage_element, "state myState;", "state usage")]
#[case(
    Rule::behavior_usage_element,
    "constraint myConstraint;",
    "constraint usage"
)]
fn test_parse_behavior_usage_element(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Extended Definition and Usage Tests

#[rstest]
#[case(
    Rule::extended_definition,
    "#meta def ExtendedDef;",
    "simple extended definition"
)]
#[case(
    Rule::extended_definition,
    "abstract #meta def ExtendedDef { }",
    "extended definition with prefix and body"
)]
#[case(
    Rule::extended_definition,
    "#meta #meta2 def ExtendedDef :> Base;",
    "multiple extension keywords"
)]
fn test_parse_extended_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::extended_usage, "#meta extendedUsage;", "simple extended usage")]
#[case(
    Rule::extended_usage,
    "ref #meta extendedUsage;",
    "extended usage with ref prefix"
)]
#[case(
    Rule::extended_usage,
    "#meta #meta2 extendedUsage : Type;",
    "multiple extension keywords"
)]
fn test_parse_extended_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Attribute Definition and Usage Tests

#[rstest]
#[case(
    Rule::attribute_definition,
    "attribute def Speed;",
    "simple attribute definition"
)]
#[case(
    Rule::attribute_definition,
    "attribute def Speed :> Real;",
    "attribute definition with subclassification"
)]
#[case(
    Rule::attribute_definition,
    "abstract attribute def Speed { }",
    "attribute definition with prefix and body"
)]
fn test_parse_attribute_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::attribute_usage, "attribute speed;", "simple attribute usage")]
#[case(
    Rule::attribute_usage,
    "attribute speed : Real;",
    "attribute usage with typing"
)]
#[case(
    Rule::attribute_usage,
    "ref attribute speed;",
    "attribute usage with ref prefix"
)]
fn test_parse_attribute_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Enumeration Definition and Usage Tests

#[rstest]
#[case(
    Rule::enumeration_definition,
    "enum def Color;",
    "simple enumeration definition"
)]
#[case(
    Rule::enumeration_definition,
    "enum def Color { }",
    "enumeration definition with empty body"
)]
#[case(
    Rule::enumeration_definition,
    "#meta enum def Status { }",
    "enumeration with prefix metadata"
)]
fn test_parse_enumeration_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::enumeration_body, ";", "simple body")]
#[case(Rule::enumeration_body, "{ }", "empty body with braces")]
fn test_parse_enumeration_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::enumeration_usage_member, "red;", "simple enumerated value")]
#[case(
    Rule::enumeration_usage_member,
    "public green;",
    "enumerated value with visibility"
)]
#[case(
    Rule::enumeration_usage_member,
    "private blue;",
    "enumerated value with private visibility"
)]
fn test_parse_enumeration_usage_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::enumerated_value, "red;", "simple enumerated value")]
#[case(
    Rule::enumerated_value,
    "enum green;",
    "enumerated value with enum keyword"
)]
#[case(
    Rule::enumerated_value,
    "#meta blue;",
    "enumerated value with metadata"
)]
fn test_parse_enumerated_value(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::enumeration_usage, "enum status;", "simple enumeration usage")]
#[case(
    Rule::enumeration_usage,
    "enum status : Status;",
    "enumeration usage with typing"
)]
#[case(
    Rule::enumeration_usage,
    "ref enum myEnum;",
    "enumeration usage with ref prefix"
)]
fn test_parse_enumeration_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Occurrence Definition and Individual Definition Tests

#[rstest]
#[case(
    Rule::occurrence_definition,
    "occurrence def Occurrence1;",
    "simple occurrence definition"
)]
#[case(
    Rule::occurrence_definition,
    "occurrence def Occurrence1 { }",
    "occurrence definition with body"
)]
#[case(
    Rule::occurrence_definition,
    "abstract occurrence def Occurrence1;",
    "occurrence definition with abstract prefix"
)]
#[case(
    Rule::occurrence_definition,
    "individual occurrence def UniqueOccurrence;",
    "occurrence definition with individual marker"
)]
fn test_parse_occurrence_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::individual_definition,
    "individual def Thing;",
    "simple individual definition"
)]
#[case(
    Rule::individual_definition,
    "individual def Thing { }",
    "individual definition with body"
)]
#[case(
    Rule::individual_definition,
    "abstract individual def UniqueThing;",
    "individual definition with abstract prefix"
)]
fn test_parse_individual_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::occurrence_keyword, "occurrence", "occurrence keyword")]
#[case(
    Rule::occurrence_def_keyword,
    "occurrence def",
    "occurrence def keyword"
)]
#[case(
    Rule::occurrence_usage_keyword,
    "occurrence",
    "occurrence usage keyword"
)]
fn test_parse_occurrence_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Occurrence Usage Tests

#[rstest]
#[case(Rule::occurrence_usage, "occurrence occ1;", "simple occurrence usage")]
#[case(
    Rule::occurrence_usage,
    "occurrence occ1 { }",
    "occurrence usage with body"
)]
#[case(
    Rule::occurrence_usage,
    "ref individual occurrence uniqueOcc;",
    "occurrence usage with ref and individual marker"
)]
#[case(
    Rule::occurrence_usage,
    "snapshot occurrence snap1;",
    "occurrence usage with portion kind"
)]
fn test_parse_occurrence_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::individual_usage,
    "ref individual thing;",
    "simple individual usage"
)]
#[case(
    Rule::individual_usage,
    "ref individual thing { }",
    "individual usage with body"
)]
#[case(
    Rule::individual_usage,
    "out individual thing : Type;",
    "individual usage with typing"
)]
fn test_parse_individual_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::portion_usage,
    "snapshot snap1;",
    "simple snapshot portion usage"
)]
#[case(
    Rule::portion_usage,
    "timeslice slice1;",
    "simple timeslice portion usage"
)]
#[case(
    Rule::portion_usage,
    "ref individual snapshot snap2;",
    "individual snapshot usage"
)]
#[case(Rule::portion_usage, "snapshot snap3 { }", "portion usage with body")]
fn test_parse_portion_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::event_occurrence_usage,
    "event myEvent;",
    "simple event occurrence usage"
)]
#[case(
    Rule::event_occurrence_usage,
    "event myEvent { }",
    "event occurrence usage with body"
)]
#[case(
    Rule::event_occurrence_usage,
    "event myRef;",
    "event with owned reference subsetting"
)]
fn test_parse_event_occurrence_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Empty Succession Tests

#[rstest]
#[case(Rule::empty_succession, "then", "simple empty succession")]
#[case(
    Rule::empty_succession,
    "then [1]",
    "empty succession with multiplicity"
)]
#[case(
    Rule::empty_succession,
    "then [0..*]",
    "empty succession with range multiplicity"
)]
fn test_parse_empty_succession(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::multiplicity_source_end, "", "empty multiplicity source end")]
#[case(
    Rule::multiplicity_source_end,
    "[1]",
    "multiplicity source end with multiplicity"
)]
#[case(
    Rule::multiplicity_source_end,
    "[0..*]",
    "multiplicity source end with range"
)]
fn test_parse_multiplicity_source_end(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::empty_target_end, "", "empty target end")]
fn test_parse_empty_target_end(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Item Definition and Usage Tests

#[rstest]
#[case(Rule::item_definition, "item def MyItem;", "simple item definition")]
#[case(
    Rule::item_definition,
    "item def MyItem { }",
    "item definition with body"
)]
#[case(
    Rule::item_definition,
    "abstract item def MyItem;",
    "item definition with abstract prefix"
)]
#[case(
    Rule::item_definition,
    "individual item def UniqueItem;",
    "item definition with individual marker"
)]
fn test_parse_item_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::item_usage, "item myItem;", "simple item usage")]
#[case(Rule::item_usage, "item myItem { }", "item usage with body")]
#[case(
    Rule::item_usage,
    "ref individual item uniqueItem;",
    "item usage with ref and individual marker"
)]
#[case(
    Rule::item_usage,
    "snapshot item snap1;",
    "item usage with portion kind"
)]
fn test_parse_item_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::item_keyword, "item", "item keyword")]
#[case(Rule::item_def_keyword, "item def", "item def keyword")]
#[case(Rule::item_usage_keyword, "item", "item usage keyword")]
fn test_parse_item_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Part Definition and Usage Tests

#[rstest]
#[case(Rule::part_definition, "part def MyPart;", "simple part definition")]
#[case(
    Rule::part_definition,
    "part def MyPart { }",
    "part definition with body"
)]
#[case(
    Rule::part_definition,
    "abstract part def MyPart;",
    "part definition with abstract prefix"
)]
#[case(
    Rule::part_definition,
    "individual part def UniquePart;",
    "part definition with individual marker"
)]
fn test_parse_part_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::part_usage, "part myPart;", "simple part usage")]
#[case(Rule::part_usage, "part myPart { }", "part usage with body")]
#[case(
    Rule::part_usage,
    "ref individual part uniquePart;",
    "part usage with ref and individual marker"
)]
#[case(
    Rule::part_usage,
    "snapshot part snap1;",
    "part usage with portion kind"
)]
fn test_parse_part_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::part_keyword, "part", "part keyword")]
#[case(Rule::part_def_keyword, "part def", "part def keyword")]
#[case(Rule::part_usage_keyword, "part", "part usage keyword")]
fn test_parse_part_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Port Usage Tests

#[rstest]
#[case(Rule::port_usage, "port myPort;", "simple port usage")]
#[case(Rule::port_usage, "port myPort { }", "port usage with body")]
#[case(
    Rule::port_usage,
    "ref individual port uniquePort;",
    "port usage with ref and individual marker"
)]
#[case(
    Rule::port_usage,
    "snapshot port snap1;",
    "port usage with portion kind"
)]
fn test_parse_port_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::port_keyword, "port", "port keyword")]
#[case(Rule::port_usage_keyword, "port", "port usage keyword")]
fn test_parse_port_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Connector Tests

#[rstest]
#[case(Rule::connector_end, "myRef", "simple connector end")]
#[case(
    Rule::connector_end,
    "[1] myRef",
    "connector end with cross multiplicity"
)]
#[case(
    Rule::connector_end,
    "endName references myRef",
    "connector end with name and references"
)]
#[case(
    Rule::connector_end,
    "[0..*] endName references myRef",
    "connector end with all elements"
)]
fn test_parse_connector_end(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_cross_multiplicity, "[1]", "owned cross multiplicity")]
#[case(
    Rule::owned_cross_multiplicity,
    "[0..*]",
    "owned cross multiplicity with range"
)]
fn test_parse_owned_cross_multiplicity(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Binding Connector Tests

#[rstest]
#[case(
    Rule::binding_connector_as_usage,
    "bind source = target;",
    "simple binding connector"
)]
#[case(
    Rule::binding_connector_as_usage,
    "bind source = target { }",
    "binding connector with body"
)]
#[case(
    Rule::binding_connector_as_usage,
    "binding myBinding bind source = target;",
    "binding connector with declaration"
)]
fn test_parse_binding_connector_as_usage(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::binding_keyword, "binding", "binding keyword")]
fn test_parse_binding_keyword(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Connection Definition Tests

#[rstest]
#[case(
    Rule::connection_definition,
    "connection def MyConnection;",
    "simple connection definition"
)]
#[case(
    Rule::connection_definition,
    "connection def MyConnection { }",
    "connection definition with body"
)]
#[case(
    Rule::connection_definition,
    "abstract connection def MyConnection;",
    "connection definition with abstract prefix"
)]
fn test_parse_connection_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::connection_keyword, "connection", "connection keyword")]
#[case(
    Rule::connection_def_keyword,
    "connection def",
    "connection def keyword"
)]
fn test_parse_connection_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Connection Usage Tests

#[rstest]
#[case(
    Rule::connection_usage,
    "connection myConn;",
    "simple connection usage"
)]
#[case(
    Rule::connection_usage,
    "connection myConn { }",
    "connection usage with body"
)]
#[case(
    Rule::connection_usage,
    "connect source to target;",
    "connection usage with connector"
)]
#[case(
    Rule::connection_usage,
    "connection myConn connect source to target;",
    "connection usage with declaration and connector"
)]
#[case(
    Rule::connection_usage,
    "connect (a, b, c);",
    "connection usage with nary connector"
)]
fn test_parse_connection_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::connector_keyword, "connect", "connector keyword")]
#[case(
    Rule::connection_usage_keyword,
    "connection",
    "connection usage keyword"
)]
fn test_parse_connector_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Connector Part Tests

#[rstest]
#[case(
    Rule::binary_connector_part,
    "source to target",
    "binary connector part"
)]
fn test_parse_binary_connector_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::nary_connector_part, "(a, b)", "nary connector with two ends")]
#[case(
    Rule::nary_connector_part,
    "(a, b, c)",
    "nary connector with three ends"
)]
#[case(
    Rule::nary_connector_part,
    "(x, y, z, w)",
    "nary connector with four ends"
)]
fn test_parse_nary_connector_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::empty_source_end, "", "empty source end")]
fn test_parse_empty_source_end(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::interface_definition,
    "interface def MyInterface;",
    "simple interface definition with semicolon"
)]
#[case(
    Rule::interface_definition,
    "interface def Vehicle { port driver; }",
    "interface definition with port"
)]
#[case(
    Rule::interface_definition,
    "abstract interface def DataInterface { ref data : DataType; }",
    "abstract interface with reference usage"
)]
fn test_parse_interface_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::interface_keyword, "interface", "interface keyword")]
#[case(Rule::interface_def_keyword, "interface def", "interface def keyword")]
#[case(Rule::interface_usage_keyword, "interface", "interface usage keyword")]
fn test_parse_interface_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::interface_body, ";", "semicolon body")]
#[case(Rule::interface_body, "{ port driver; }", "body with port")]
fn test_parse_interface_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::interface_non_occurrence_usage_element,
    "ref data : DataType;",
    "reference usage"
)]
#[case(
    Rule::interface_non_occurrence_usage_element,
    "attribute speed : Real;",
    "attribute usage"
)]
fn test_parse_interface_non_occurrence_usage_element(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::interface_occurrence_usage_element,
    "end driver;",
    "default interface end"
)]
#[case(Rule::interface_occurrence_usage_element, "port sensor;", "port usage")]
fn test_parse_interface_occurrence_usage_element(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::default_interface_end,
    "end driver;",
    "end with usage declaration"
)]
#[case(Rule::default_interface_end, "end;", "end without declaration")]
fn test_parse_default_interface_end(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::interface_usage_declaration,
    "portA to portB",
    "binary interface part only"
)]
fn test_parse_interface_usage_declaration(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::interface_part, "portA to portB", "binary interface part")]
#[case(
    Rule::interface_part,
    "(portA, portB)",
    "nary interface part with two ports"
)]
#[case(
    Rule::interface_part,
    "(portA, portB, portC)",
    "nary interface part with three ports"
)]
fn test_parse_interface_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::binary_interface_part, "portA to portB", "binary interface part")]
fn test_parse_binary_interface_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::nary_interface_part, "(portA, portB)", "nary with two ports")]
#[case(
    Rule::nary_interface_part,
    "(portA, portB, portC)",
    "nary with three ports"
)]
fn test_parse_nary_interface_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::interface_end_member, "portA", "simple interface end member")]
fn test_parse_interface_end_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::interface_end, "portA", "simple interface end")]
#[case(
    Rule::interface_end,
    "myPort references BasePort",
    "interface end with references"
)]
fn test_parse_interface_end(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::flow_definition,
    "flow def DataFlow;",
    "simple flow definition with semicolon"
)]
#[case(
    Rule::flow_definition,
    "flow def FluidFlow { }",
    "flow definition with body"
)]
#[case(
    Rule::flow_definition,
    "abstract flow def AbstractFlow;",
    "abstract flow definition"
)]
fn test_parse_flow_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::flow_keyword, "flow", "flow keyword")]
#[case(Rule::flow_def_keyword, "flow def", "flow def keyword")]
#[case(Rule::message_keyword, "message", "message keyword")]
fn test_parse_flow_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::message_declaration,
    "eventA to eventB",
    "message declaration with two events"
)]
fn test_parse_message_declaration(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::payload_feature_member, "dataRef", "payload feature member")]
fn test_parse_payload_feature_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::payload_feature, "dataRef", "payload feature")]
fn test_parse_payload_feature(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::message_event_member, "eventRef", "message event member")]
fn test_parse_message_event_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::message_event, "eventRef", "message event")]
fn test_parse_message_event(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::payload, "dataRef", "payload feature")]
fn test_parse_payload(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::payload_feature_specialization_part,
    ": DataType [1]",
    "payload feature specialization with multiplicity"
)]
fn test_parse_payload_feature_specialization_part(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::flow_end_member, "myFlow", "flow end member")]
fn test_parse_flow_end_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::flow_end, "flowRef", "flow end")]
fn test_parse_flow_end(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::flow_feature_member, "flowRef", "flow feature member")]
fn test_parse_flow_feature_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::flow_feature, "flowRef", "flow feature")]
fn test_parse_flow_feature(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::flow_redefinition, "BaseFlow", "flow redefinition")]
fn test_parse_flow_redefinition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::action_definition,
    "action def Move;",
    "simple action definition with semicolon"
)]
#[case(
    Rule::action_definition,
    "action def Calculate { }",
    "action definition with body"
)]
#[case(
    Rule::action_definition,
    "abstract action def AbstractAction;",
    "abstract action definition"
)]
fn test_parse_action_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::action_keyword, "action", "action keyword")]
#[case(Rule::action_def_keyword, "action def", "action def keyword")]
#[case(Rule::action_usage_keyword, "action", "action usage keyword")]
fn test_parse_action_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::action_body, ";", "semicolon body")]
#[case(Rule::action_body, "{ }", "empty brace body")]
fn test_parse_action_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::initial_node_member, "first startNode;", "initial node member")]
fn test_parse_initial_node_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::member_prefix, "", "empty member prefix")]
fn test_parse_member_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::action_node, "send myMsg;", "send action node")]
#[case(Rule::action_node, "accept myAccept;", "accept action node")]
#[case(Rule::action_node, "assign x := y;", "assignment action node")]
fn test_parse_action_node(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::action_node_member, "send msg;", "action node member with send")]
#[case(
    Rule::action_node_member,
    "accept acc;",
    "action node member with accept"
)]
fn test_parse_action_node_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::action_target_succession,
    "then nextNode;",
    "simple target succession"
)]
#[case(
    Rule::action_target_succession,
    "if true then nextNode;",
    "guarded target succession"
)]
#[case(
    Rule::action_target_succession,
    "else defaultNode;",
    "default target succession"
)]
fn test_parse_action_target_succession(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::target_succession_member,
    "then nextNode;",
    "target succession member with then"
)]
fn test_parse_target_succession_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::guarded_target_succession,
    "if x then y;",
    "guarded target succession"
)]
fn test_parse_guarded_target_succession(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(
    Rule::guarded_succession_member,
    "if x then y;",
    "guarded succession member"
)]
fn test_parse_guarded_succession_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::action_usage_declaration, "", "empty action usage declaration")]
fn test_parse_action_usage_declaration(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::perform_action_usage_declaration,
    "actionRef",
    "perform action usage declaration with reference"
)]
#[case(
    Rule::perform_action_usage_declaration,
    "action myAction;",
    "perform action usage declaration with action keyword"
)]
fn test_parse_perform_action_usage_declaration(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(
    Rule::action_node_usage_declaration,
    "action",
    "action node usage declaration"
)]
fn test_parse_action_node_usage_declaration(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::action_node_prefix, "", "empty action node prefix")]
fn test_parse_action_node_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::accept_node_declaration,
    "accept msg;",
    "accept node declaration"
)]
fn test_parse_accept_node_declaration(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::accept_parameter_part, "msg", "accept parameter part")]
#[case(
    Rule::accept_parameter_part,
    "msg via port",
    "accept parameter part with via"
)]
fn test_parse_accept_parameter_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::payload_parameter, "data", "payload parameter")]
fn test_parse_payload_parameter(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::trigger_expression, "at timeValue", "time trigger expression")]
#[case(
    Rule::trigger_expression,
    "when condition",
    "change trigger expression"
)]
fn test_parse_trigger_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::time_trigger_kind, "at", "at trigger")]
#[case(Rule::time_trigger_kind, "after", "after trigger")]
fn test_parse_time_trigger_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::change_trigger_kind, "when", "when trigger")]
fn test_parse_change_trigger_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::argument_member, "arg", "argument member")]
fn test_parse_argument_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::argument_expression, "expr", "argument expression")]
fn test_parse_argument_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::owned_expression_reference,
    "exprRef",
    "owned expression reference"
)]
fn test_parse_owned_expression_reference(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::node_parameter, "param", "node parameter")]
fn test_parse_node_parameter(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_binding, "expr", "feature binding")]
fn test_parse_feature_binding(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::owned_expression, "expr", "owned expression")]
fn test_parse_owned_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::send_node_declaration, "send msg;", "send node declaration")]
fn test_parse_send_node_declaration(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(
    Rule::sender_receiver_part,
    "via port1",
    "sender receiver part with via"
)]
#[case(
    Rule::sender_receiver_part,
    "via port1 to port2",
    "sender receiver part with via and to"
)]
fn test_parse_sender_receiver_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::assignment_node_declaration,
    "assign x:= y",
    "assignment node declaration"
)]
fn test_parse_assignment_node_declaration(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::assignment_target_member, "target", "assignment target member")]
fn test_parse_assignment_target_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::target_parameter, "feature", "target parameter without binding")]
#[case(
    Rule::target_parameter,
    "binding.feature",
    "target parameter with binding"
)]
fn test_parse_target_parameter(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::target_binding, "target", "target binding with identifier")]
#[case(
    Rule::target_binding,
    "source.property",
    "target binding with feature chain"
)]
fn test_parse_target_binding(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::feature_chain_member, "chain", "feature chain member")]
fn test_parse_feature_chain_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::base_expression, "expr", "base expression")]
fn test_parse_base_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::target_expression, "expr", "simple target expression")]
#[case(Rule::target_expression, "expr.member", "target expression with chain")]
#[case(Rule::target_expression, "expr[index]", "target expression with index")]
fn test_parse_target_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::sequence_expression, "seq", "sequence expression")]
fn test_parse_sequence_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::reference_typing, "TypeRef", "reference typing")]
fn test_parse_reference_typing(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::expression_body_member,
    "{;}",
    "expression body member with empty body"
)]
fn test_parse_expression_body_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::function_reference_member,
    "funcRef",
    "function reference member"
)]
fn test_parse_function_reference_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::argument_list, "()", "empty argument list")]
#[case(Rule::argument_list, "(arg)", "argument list with one positional arg")]
#[case(
    Rule::argument_list,
    "(arg1, arg2)",
    "argument list with multiple positional args"
)]
#[case(
    Rule::argument_list,
    "(arg1, arg2, arg3)",
    "argument list with three positional args"
)]
#[case(
    Rule::argument_list,
    "(param1 = value1)",
    "argument list with one named arg"
)]
#[case(
    Rule::argument_list,
    "(param1 = value1, param2 = value2)",
    "argument list with multiple named args"
)]
fn test_parse_argument_list(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::positional_argument_list,
    "arg1, arg2",
    "positional argument list"
)]
#[case(
    Rule::positional_argument_list,
    "x, y, z",
    "positional argument list with three args"
)]
fn test_parse_positional_argument_list(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::argument, "value", "argument")]
#[case(Rule::argument, "expression", "argument with expression")]
fn test_parse_argument(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::named_argument_list,
    "param1 = value1, param2 = value2",
    "named argument list"
)]
#[case(
    Rule::named_argument_list,
    "x = a, y = b",
    "named argument list with two args"
)]
fn test_parse_named_argument_list(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::named_argument_member, "param = value", "named argument member")]
#[case(
    Rule::named_argument_member,
    "x = y",
    "named argument member with simple assignment"
)]
fn test_parse_named_argument_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::named_argument, "param = value", "named argument")]
#[case(
    Rule::named_argument,
    "x = expression",
    "named argument with expression"
)]
fn test_parse_named_argument(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::parameter_redefinition, "param", "parameter redefinition")]
#[case(
    Rule::parameter_redefinition,
    "featureRef",
    "parameter redefinition with feature ref"
)]
fn test_parse_parameter_redefinition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::argument_value, "value", "argument value")]
#[case(Rule::argument_value, "123", "argument value with number")]
#[case(Rule::argument_value, "\"string\"", "argument value with string")]
fn test_parse_argument_value(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::invocation_expression,
    "MyType()",
    "invocation expression with empty args"
)]
#[case(
    Rule::invocation_expression,
    "MyType(arg)",
    "invocation expression with one arg"
)]
#[case(
    Rule::invocation_expression,
    "MyType(arg1, arg2)",
    "invocation expression with multiple args"
)]
#[case(
    Rule::invocation_expression,
    "MyType(param = value)",
    "invocation expression with named arg"
)]
fn test_parse_invocation_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::target_feature, "feature", "target feature")]
fn test_parse_target_feature(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(
    Rule::target_accessed_feature_member,
    "accessed",
    "target accessed feature member"
)]
fn test_parse_target_accessed_feature_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::empty_usage, "", "empty usage")]
fn test_parse_empty_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::expression_parameter_member,
    "expr",
    "expression parameter member"
)]
fn test_parse_expression_parameter_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::action_body_parameter, "{}", "empty action body parameter")]
fn test_parse_action_body_parameter(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::action_body_parameter_member,
    "{}",
    "action body parameter member"
)]
fn test_parse_action_body_parameter_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::for_variable_declaration, "varName", "for variable declaration")]
fn test_parse_for_variable_declaration(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::control_node_prefix, "", "control node prefix")]
fn test_parse_control_node_prefix(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::if_node, "if condition {}", "if with condition")]
#[case(Rule::if_node, "if x {}", "if with simple condition")]
#[case(Rule::if_node, "if x {} else {}", "if with else")]
fn test_parse_if_node(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::if_node_parameter_member,
    "if condition {}",
    "if node parameter member"
)]
fn test_parse_if_node_parameter_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::while_loop_node, "while condition {}", "while loop")]
#[case(Rule::while_loop_node, "loop {}", "unconditional loop")]
#[case(Rule::while_loop_node, "while x {}", "while with simple condition")]
#[case(Rule::while_loop_node, "loop {} until result;", "loop with until")]
fn test_parse_while_loop_node(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::empty_parameter_member, "", "empty parameter member")]
fn test_parse_empty_parameter_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::for_loop_node, "for x in items {}", "for with simple vars")]
fn test_parse_for_loop_node(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::terminate_node, "terminate;", "terminate node")]
fn test_parse_terminate_node(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::control_node, "merge;", "control node with merge")]
#[case(Rule::control_node, "decide;", "control node with decide")]
#[case(Rule::control_node, "join;", "control node with join")]
#[case(Rule::control_node, "fork;", "control node with fork")]
fn test_parse_control_node(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// State Definition Tests

#[rstest]
#[case(Rule::state_keyword, "state", "state keyword")]
#[case(Rule::state_def_keyword, "state def", "state def keyword")]
#[case(Rule::state_usage_keyword, "state", "state usage keyword")]
fn test_parse_state_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::state_definition,
    "state def MyState;",
    "simple state definition"
)]
#[case(
    Rule::state_definition,
    "state def MyState {}",
    "state definition with empty body"
)]
#[case(
    Rule::state_definition,
    "state def MyState parallel {}",
    "parallel state definition"
)]
fn test_parse_state_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::state_def_body, ";", "semicolon state def body")]
#[case(Rule::state_def_body, "{}", "empty braces state def body")]
#[case(Rule::state_def_body, "parallel {}", "parallel state def body")]
fn test_parse_state_def_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::entry_action_kind, "entry", "entry action kind")]
fn test_parse_entry_action_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::do_action_kind, "do", "do action kind")]
fn test_parse_do_action_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::exit_action_kind, "exit", "exit action kind")]
fn test_parse_exit_action_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::state_action_usage,
    "action entryAction;",
    "full form with action keyword"
)]
#[case(
    Rule::state_action_usage,
    "action doAction: Action;",
    "typed state action usage"
)]
#[case(
    Rule::state_action_usage,
    "action exercise : Exercise { }",
    "state action with body"
)]
#[case(Rule::state_action_usage, ";", "empty action shorthand")]
#[case(
    Rule::state_action_usage,
    "monitorTemperature;",
    "reference subsetting shorthand"
)]
fn test_parse_state_action_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::empty_action_usage, "", "empty action usage")]
fn test_parse_empty_action_usage_state(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::entry_action_member,
    "entry action entryAction;",
    "full form with action keyword"
)]
#[case(
    Rule::entry_action_member,
    "entry action warmup : WarmUp;",
    "typed entry action"
)]
#[case(
    Rule::entry_action_member,
    "entry action entryAction :>> 'entry';",
    "entry action with redefinition"
)]
#[case(Rule::entry_action_member, "entry;", "empty entry action shorthand")]
#[case(
    Rule::entry_action_member,
    "entry setupSensor;",
    "entry reference subsetting shorthand"
)]
fn test_parse_entry_action_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::do_action_member,
    "do action doAction;",
    "full form with action keyword"
)]
#[case(
    Rule::do_action_member,
    "do action exercise : Exercise;",
    "typed do action"
)]
#[case(
    Rule::do_action_member,
    "do action doAction: Action :>> 'do';",
    "do action with redefinition"
)]
#[case(
    Rule::do_action_member,
    "do action exercise : Exercise { }",
    "do action with body"
)]
#[case(Rule::do_action_member, "do;", "empty do action shorthand")]
#[case(
    Rule::do_action_member,
    "do monitorTemperature;",
    "do reference subsetting shorthand"
)]
fn test_parse_do_action_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::exit_action_member,
    "exit action exitAction;",
    "full form with action keyword"
)]
#[case(
    Rule::exit_action_member,
    "exit action cooldown : Cooldown;",
    "typed exit action"
)]
#[case(
    Rule::exit_action_member,
    "exit action exitAction: Action :>> 'exit';",
    "exit action with redefinition"
)]
#[case(Rule::exit_action_member, "exit;", "empty exit action shorthand")]
#[case(
    Rule::exit_action_member,
    "exit cleanup;",
    "exit reference subsetting shorthand"
)]
fn test_parse_exit_action_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// State Usage Tests

#[rstest]
#[case(Rule::state_usage, "state;", "simple state usage")]
#[case(Rule::state_usage, "state {}", "state usage with empty body")]
#[case(Rule::state_usage, "state parallel {}", "parallel state usage")]
fn test_parse_state_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::state_usage_body, ";", "semicolon state usage body")]
#[case(Rule::state_usage_body, "{}", "empty braces state usage body")]
#[case(Rule::state_usage_body, "parallel {}", "parallel state usage body")]
fn test_parse_state_usage_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::exhibit_state_usage,
    "exhibit state;",
    "simple exhibit state usage"
)]
#[case(Rule::exhibit_state_usage, "exhibit myRef;", "exhibit with reference")]
#[case(
    Rule::exhibit_state_usage,
    "exhibit state MyState;",
    "exhibit with state and identifier"
)]
fn test_parse_exhibit_state_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Transition Usage Tests

#[rstest]
#[case(
    Rule::transition_usage_keyword,
    "transition",
    "transition usage keyword"
)]
fn test_parse_transition_usage_keyword(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::trigger_feature_kind, "accept", "trigger feature kind")]
fn test_parse_trigger_feature_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::guard_feature_kind, "if", "guard feature kind")]
fn test_parse_guard_feature_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::effect_feature_kind, "do", "effect feature kind")]
fn test_parse_effect_feature_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::transition_source_member,
    "sourceRef",
    "transition source member with reference"
)]
fn test_parse_transition_source_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::trigger_action, "msg", "trigger action with type")]
#[case(Rule::trigger_action, "msg via msg2", "trigger action with other msg")]
fn test_parse_trigger_action(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::trigger_action_member, "accept msg", "trigger action member")]
fn test_parse_trigger_action_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::guard_expression_member,
    "if condition",
    "guard expression member"
)]
fn test_parse_guard_expression_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::effect_behavior_member,
    "do {}",
    "effect behavior member with empty action"
)]
fn test_parse_effect_behavior_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_parse_succeeds(rule, input, desc);
}

#[rstest]
#[case(Rule::effect_behavior_usage, "", "empty effect behavior usage")]
fn test_parse_effect_behavior_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::transition_succession, "target", "transition succession")]
fn test_parse_transition_succession(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::transition_succession_member,
    "target",
    "transition succession member"
)]
fn test_parse_transition_succession_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Calculation Definition Tests

#[rstest]
#[case(Rule::calculation_keyword, "calc", "calculation keyword")]
#[case(Rule::calculation_def_keyword, "calc def", "calculation def keyword")]
#[case(Rule::calculation_usage_keyword, "calc", "calculation usage keyword")]
fn test_parse_calculation_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::calculation_definition,
    "calc def MyCalc;",
    "simple calculation definition"
)]
#[case(
    Rule::calculation_definition,
    "calc def MyCalc {}",
    "calculation definition with empty body"
)]
fn test_parse_calculation_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::calculation_body, ";", "semicolon calculation body")]
#[case(Rule::calculation_body, "{}", "empty braces calculation body")]
fn test_parse_calculation_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::calculation_body_part, "", "empty calculation body part")]
fn test_parse_calculation_body_part(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::return_parameter_member,
    "return myValue;",
    "return parameter member"
)]
fn test_parse_return_parameter_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::result_expression_member, "result", "result expression member")]
fn test_parse_result_expression_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

// Calculation Usage Tests

#[rstest]
#[case(Rule::calculation_usage, "calc;", "simple calculation usage")]
#[case(Rule::calculation_usage, "calc {}", "calculation usage with body")]
fn test_parse_calculation_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Constraint Definition Tests

#[rstest]
#[case(Rule::constraint_keyword, "constraint", "constraint keyword")]
#[case(
    Rule::constraint_def_keyword,
    "constraint def",
    "constraint def keyword"
)]
#[case(
    Rule::constraint_usage_keyword,
    "constraint",
    "constraint usage keyword"
)]
fn test_parse_constraint_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::constraint_definition,
    "constraint def MyConstraint;",
    "simple constraint definition"
)]
#[case(
    Rule::constraint_definition,
    "constraint def MyConstraint {}",
    "constraint definition with body"
)]
fn test_parse_constraint_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Constraint Usage Tests

#[rstest]
#[case(Rule::constraint_usage, "constraint;", "simple constraint usage")]
#[case(Rule::constraint_usage, "constraint {}", "constraint usage with body")]
fn test_parse_constraint_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::constraint_usage_declaration,
    "",
    "empty constraint usage declaration"
)]
fn test_parse_constraint_usage_declaration(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::assert_constraint_usage,
    "assert myRef;",
    "assert constraint usage with reference"
)]
#[case(
    Rule::assert_constraint_usage,
    "assert not myRef;",
    "assert constraint usage with negation"
)]
#[case(
    Rule::assert_constraint_usage,
    "assert constraint;",
    "assert constraint usage with keyword"
)]
fn test_parse_assert_constraint_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Requirement Definition Tests

#[rstest]
#[case(Rule::requirement_keyword, "requirement", "requirement keyword")]
#[case(
    Rule::requirement_def_keyword,
    "requirement def",
    "requirement def keyword"
)]
fn test_parse_requirement_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::requirement_definition,
    "requirement def SafetyRequirement;",
    "requirement definition with semicolon"
)]
#[case(
    Rule::requirement_definition,
    "requirement def SafetyRequirement {}",
    "requirement definition with empty body"
)]
#[case(
    Rule::requirement_definition,
    "requirement def SafetyRequirement { /* requirement doc */ }",
    "requirement definition with doc comment"
)]
fn test_parse_requirement_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::requirement_body, ";", "semicolon body")]
#[case(Rule::requirement_body, "{}", "empty body")]
fn test_parse_requirement_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::subject_usage, "subject mySubject;", "subject usage")]
#[case(
    Rule::subject_usage,
    "subject subj default Case::result;",
    "subject usage with default qualified value"
)]
#[case(
    Rule::subject_usage,
    "subject subj default myValue;",
    "subject usage with default simple value"
)]
#[case(
    Rule::subject_usage,
    "subject subj : MyType;",
    "subject usage with typing"
)]
#[case(
    Rule::subject_usage,
    "subject subj : MyType default myValue;",
    "subject usage with typing and default"
)]
fn test_parse_subject_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::subject_member, "subject mySubject;", "subject member")]
#[case(
    Rule::subject_member,
    "subject subj default Case::result;",
    "subject member with default qualified value"
)]
fn test_parse_subject_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::requirement_constraint_usage,
    "myConstraint {}",
    "requirement constraint usage with reference"
)]
#[case(
    Rule::requirement_constraint_usage,
    "constraint {}",
    "requirement constraint usage with keyword"
)]
fn test_parse_requirement_constraint_usage(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::requirement_constraint_member,
    "assume myConstraint {}",
    "assume requirement constraint member"
)]
#[case(
    Rule::requirement_constraint_member,
    "require constraint {}",
    "require requirement constraint member"
)]
fn test_parse_requirement_constraint_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::framed_concern_kind, "frame", "frame keyword")]
fn test_parse_framed_concern_kind(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::framed_concern_usage,
    "myConcern;",
    "framed concern usage with reference"
)]
#[case(
    Rule::framed_concern_usage,
    "concern {}",
    "framed concern usage with keyword"
)]
fn test_parse_framed_concern_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::framed_concern_member,
    "frame myConcern;",
    "framed concern member"
)]
fn test_parse_framed_concern_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::actor_usage, "actor myActor;", "actor usage")]
fn test_parse_actor_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::actor_member, "actor myActor;", "actor member")]
fn test_parse_actor_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::stakeholder_usage,
    "stakeholder myStakeholder;",
    "stakeholder usage"
)]
fn test_parse_stakeholder_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::stakeholder_member,
    "stakeholder myStakeholder;",
    "stakeholder member"
)]
fn test_parse_stakeholder_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::requirement_verification_usage,
    "myVerification;",
    "requirement verification usage with reference"
)]
#[case(
    Rule::requirement_verification_usage,
    "requirement myReq {}",
    "requirement verification usage with keyword"
)]
fn test_parse_requirement_verification_usage(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::requirement_verification_member,
    "verify myVerification;",
    "requirement verification member"
)]
fn test_parse_requirement_verification_member(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::concern_usage, "concern myConcern {}", "concern usage")]
fn test_parse_concern_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Concern Definition Tests

#[rstest]
#[case(Rule::concern_keyword, "concern", "concern keyword")]
#[case(Rule::concern_def_keyword, "concern def", "concern def keyword")]
#[case(Rule::concern_usage_keyword, "concern", "concern usage keyword")]
fn test_parse_concern_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::concern_definition,
    "concern def PerformanceConcern;",
    "concern definition with semicolon"
)]
#[case(
    Rule::concern_definition,
    "concern def PerformanceConcern {}",
    "concern definition with empty body"
)]
#[case(
    Rule::concern_definition,
    "concern def BrakingConcern { require constraint { /**/ } }",
    "concern definition with requirement constraint and doc"
)]
#[case(
    Rule::concern_definition,
    "concern def SafetyConcern { subject vehicle; stakeholder driver; }",
    "concern definition with subject and stakeholder"
)]
fn test_parse_concern_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Case Definition Tests

#[rstest]
#[case(Rule::case_keyword, "case", "case keyword")]
#[case(Rule::case_def_keyword, "case def", "case def keyword")]
#[case(Rule::case_usage_keyword, "case", "case usage keyword")]
fn test_parse_case_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::case_definition,
    "case def TestCase;",
    "case definition with semicolon"
)]
#[case(
    Rule::case_definition,
    "case def TestCase {}",
    "case definition with empty body"
)]
#[case(
    Rule::case_definition,
    "case def TestCase { subject testSubject; }",
    "case definition with subject"
)]
fn test_parse_case_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::case_body, ";", "semicolon body")]
#[case(Rule::case_body, "{}", "empty body")]
fn test_parse_case_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::objective_requirement_usage,
    "myObjective {}",
    "objective requirement usage with declaration"
)]
#[case(
    Rule::objective_requirement_usage,
    "{}",
    "objective requirement usage with empty body"
)]
#[case(
    Rule::objective_requirement_usage,
    "obj : RequirementCheck[1] { subject subj default Case::result; }",
    "objective with subject and default value"
)]
fn test_parse_objective_requirement_usage(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::objective_member, "objective myObjective {}", "objective member")]
#[case(
    Rule::objective_member,
    "objective obj : RequirementCheck[1] { subject subj default Case::result; }",
    "objective member with subject and default value"
)]
fn test_parse_objective_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Case Usage Tests

#[rstest]
#[case(Rule::case_usage, "case testCase;", "case usage with semicolon")]
#[case(Rule::case_usage, "case testCase {}", "case usage with empty body")]
#[case(
    Rule::case_usage,
    "case testCase { subject testSubject; }",
    "case usage with subject"
)]
fn test_parse_case_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Analysis Case Tests

#[rstest]
#[case(Rule::analysis_keyword, "analysis", "analysis keyword")]
#[case(Rule::verification_keyword, "verification", "verification keyword")]
fn test_parse_analysis_verification_keywords(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::analysis_case_definition,
    "analysis case def AnalysisTest;",
    "analysis case definition with semicolon"
)]
#[case(
    Rule::analysis_case_definition,
    "analysis case def AnalysisTest {}",
    "analysis case definition with empty body"
)]
#[case(
    Rule::analysis_case_definition,
    "analysis case def AnalysisTest { subject testSubject; }",
    "analysis case definition with subject"
)]
fn test_parse_analysis_case_definition(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::analysis_case_usage,
    "analysis testAnalysis;",
    "analysis case usage with semicolon"
)]
#[case(
    Rule::analysis_case_usage,
    "analysis testAnalysis {}",
    "analysis case usage with empty body"
)]
#[case(
    Rule::analysis_case_usage,
    "analysis testAnalysis { actor analyst; }",
    "analysis case usage with actor"
)]
fn test_parse_analysis_case_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Verification Case Tests

#[rstest]
#[case(
    Rule::verification_case_definition,
    "verification case def VerifyTest;",
    "verification case definition with semicolon"
)]
#[case(
    Rule::verification_case_definition,
    "verification case def VerifyTest {}",
    "verification case definition with empty body"
)]
#[case(
    Rule::verification_case_definition,
    "verification case def VerifyTest { objective myObj {} }",
    "verification case definition with objective"
)]
fn test_parse_verification_case_definition(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::verification_case_usage,
    "verification testVerification;",
    "verification case usage with semicolon"
)]
#[case(
    Rule::verification_case_usage,
    "verification testVerification {}",
    "verification case usage with empty body"
)]
fn test_parse_verification_case_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Use Case Tests

#[rstest]
#[case(Rule::use_case_def_keyword, "use case def", "use case def keyword")]
fn test_parse_use_case_def_keyword(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::use_case_definition,
    "use case def TestUseCase;",
    "use case definition with semicolon"
)]
#[case(
    Rule::use_case_definition,
    "use case def TestUseCase {}",
    "use case definition with empty body"
)]
fn test_parse_use_case_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// View Definition Tests

#[rstest]
#[case(Rule::view_keyword, "view", "view keyword")]
#[case(Rule::view_def_keyword, "view def", "view def keyword")]
#[case(Rule::view_usage_keyword, "view", "view usage keyword")]
fn test_parse_view_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::view_definition,
    "view def TestView;",
    "view definition with semicolon"
)]
#[case(
    Rule::view_definition,
    "view def TestView {}",
    "view definition with empty body"
)]
fn test_parse_view_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::view_definition_body, ";", "semicolon body")]
#[case(Rule::view_definition_body, "{}", "empty body")]
fn test_parse_view_definition_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::view_rendering_usage,
    "myRendering;",
    "view rendering usage with reference"
)]
#[case(
    Rule::view_rendering_usage,
    "rendering myRender {}",
    "view rendering usage with keyword"
)]
fn test_parse_view_rendering_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::view_rendering_member,
    "render myRendering;",
    "view rendering member"
)]
fn test_parse_view_rendering_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// View Usage Tests

#[rstest]
#[case(Rule::view_usage, "view myView;", "view usage with semicolon")]
#[case(Rule::view_usage, "view myView {}", "view usage with empty body")]
fn test_parse_view_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::view_body, ";", "semicolon body")]
#[case(Rule::view_body, "{}", "empty body")]
fn test_parse_view_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Viewpoint Tests

#[rstest]
#[case(Rule::viewpoint_keyword, "viewpoint", "viewpoint keyword")]
#[case(Rule::viewpoint_def_keyword, "viewpoint def", "viewpoint def keyword")]
#[case(Rule::viewpoint_usage_keyword, "viewpoint", "viewpoint usage keyword")]
fn test_parse_viewpoint_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::viewpoint_definition,
    "viewpoint def TestViewpoint;",
    "viewpoint definition with semicolon"
)]
#[case(
    Rule::viewpoint_definition,
    "viewpoint def TestViewpoint {}",
    "viewpoint definition with empty body"
)]
#[case(
    Rule::viewpoint_definition,
    "viewpoint def TestViewpoint { stakeholder user; }",
    "viewpoint definition with stakeholder"
)]
fn test_parse_viewpoint_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::viewpoint_usage,
    "viewpoint myViewpoint;",
    "viewpoint usage with semicolon"
)]
#[case(
    Rule::viewpoint_usage,
    "viewpoint myViewpoint {}",
    "viewpoint usage with empty body"
)]
fn test_parse_viewpoint_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Rendering Definition Tests

#[rstest]
#[case(Rule::rendering_token, "rendering", "rendering keyword")]
#[case(Rule::rendering_def, "rendering def", "rendering def keyword")]
fn test_parse_rendering_keywords(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::rendering_definition,
    "rendering def TestRendering;",
    "rendering definition with semicolon"
)]
#[case(
    Rule::rendering_definition,
    "rendering def TestRendering {}",
    "rendering definition with empty body"
)]
fn test_parse_rendering_definition(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Rendering Usage Tests

#[rstest]
#[case(Rule::rendering_usage, "rendering myRendering;", "rendering usage")]
fn test_parse_rendering_usage(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// Expression Tests

#[rstest]
#[case(Rule::expression_body, ";", "expression body with semicolon")]
#[case(Rule::expression_body, "{}", "expression body with empty body")]
fn test_parse_expression_body(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::owned_expression_member,
    "myValue",
    "owned expression member with identifier"
)]
fn test_parse_owned_expression_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::conditional_expression,
    "myValue",
    "conditional expression with identifier"
)]
#[case(
    Rule::conditional_expression,
    "\"test\"",
    "conditional expression with string"
)]
#[case(
    Rule::conditional_expression,
    "123",
    "conditional expression with number"
)]
#[case(
    Rule::conditional_expression,
    "false",
    "conditional expression with boolean"
)]
#[case(
    Rule::conditional_expression,
    "if x ? a else b",
    "conditional expression with if-else"
)]
fn test_parse_conditional_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::null_coalescing_expression,
    "value",
    "null coalescing expression with single value"
)]
#[case(
    Rule::null_coalescing_expression,
    "a ?? b",
    "null coalescing expression with operator"
)]
fn test_parse_null_coalescing_expression(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::implies_expression,
    "value",
    "implies expression with single value"
)]
#[case(
    Rule::implies_expression,
    "a implies b",
    "implies expression with operator"
)]
fn test_parse_implies_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::or_expression, "value", "or expression with single value")]
#[case(Rule::or_expression, "a | b", "or expression with pipe operator")]
#[case(Rule::or_expression, "a or b", "or expression with or keyword")]
fn test_parse_or_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::xor_expression, "value", "xor expression with single value")]
#[case(Rule::xor_expression, "a xor b", "xor expression with operator")]
fn test_parse_xor_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::and_expression, "value", "and expression with single value")]
#[case(
    Rule::and_expression,
    "a & b",
    "and expression with ampersand operator"
)]
#[case(Rule::and_expression, "a and b", "and expression with and keyword")]
fn test_parse_and_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::equality_expression,
    "value",
    "equality expression with identifier"
)]
#[case(Rule::equality_expression, "42", "equality expression with number")]
#[case(
    Rule::equality_expression,
    "a == b",
    "equality expression with == operator"
)]
#[case(
    Rule::equality_expression,
    "a != b",
    "equality expression with != operator"
)]
#[case(
    Rule::equality_expression,
    "a === b",
    "equality expression with === operator"
)]
#[case(
    Rule::equality_expression,
    "a !== b",
    "equality expression with !== operator"
)]
fn test_parse_equality_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::equality_operator, "==", "equality operator ==")]
#[case(Rule::equality_operator, "!=", "equality operator !=")]
#[case(Rule::equality_operator, "===", "equality operator ===")]
#[case(Rule::equality_operator, "!==", "equality operator !==")]
fn test_parse_equality_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::classification_expression,
    "value",
    "classification expression with identifier"
)]
#[case(
    Rule::classification_expression,
    "value hastype Type",
    "classification expression with hastype"
)]
#[case(
    Rule::classification_expression,
    "value istype Type",
    "classification expression with istype"
)]
#[case(
    Rule::classification_expression,
    "value @ Type",
    "classification expression with @ operator"
)]
#[case(
    Rule::classification_expression,
    "value as Type",
    "classification expression with as"
)]
#[case(
    Rule::classification_expression,
    "metadata @@ Type",
    "classification expression with @@"
)]
#[case(
    Rule::classification_expression,
    "metadata meta Type",
    "classification expression with meta"
)]
fn test_parse_classification_expression(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::classification_test_operator,
    "hastype",
    "classification test operator hastype"
)]
#[case(
    Rule::classification_test_operator,
    "istype",
    "classification test operator istype"
)]
#[case(
    Rule::classification_test_operator,
    "@",
    "classification test operator @"
)]
fn test_parse_classification_test_operator(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::type_reference_member, "MyType", "type reference member")]
fn test_parse_type_reference_member(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::type_reference, "MyType", "type reference")]
fn test_parse_type_reference(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::relational_expression,
    "value",
    "relational expression with identifier"
)]
#[case(Rule::relational_expression, "42", "relational expression with number")]
#[case(
    Rule::relational_expression,
    "a < b",
    "relational expression with < operator"
)]
#[case(
    Rule::relational_expression,
    "a > b",
    "relational expression with > operator"
)]
#[case(
    Rule::relational_expression,
    "a <= b",
    "relational expression with <= operator"
)]
#[case(
    Rule::relational_expression,
    "a >= b",
    "relational expression with >= operator"
)]
fn test_parse_relational_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::relational_operator, "<", "relational operator <")]
#[case(Rule::relational_operator, ">", "relational operator >")]
#[case(Rule::relational_operator, "<=", "relational operator <=")]
#[case(Rule::relational_operator, ">=", "relational operator >=")]
fn test_parse_relational_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::range_expression, "value", "range expression with single value")]
#[case(
    Rule::range_expression,
    "1..10",
    "range expression with range operator"
)]
fn test_parse_range_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::additive_expression,
    "value",
    "additive expression with single value"
)]
#[case(
    Rule::additive_expression,
    "a + b",
    "additive expression with + operator"
)]
#[case(
    Rule::additive_expression,
    "a - b",
    "additive expression with - operator"
)]
#[case(
    Rule::additive_expression,
    "a + b - c",
    "additive expression with multiple operators"
)]
fn test_parse_additive_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::additive_operator, "+", "additive operator +")]
#[case(Rule::additive_operator, "-", "additive operator -")]
fn test_parse_additive_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::multiplicative_expression,
    "value",
    "multiplicative expression with single value"
)]
#[case(
    Rule::multiplicative_expression,
    "a * b",
    "multiplicative expression with * operator"
)]
#[case(
    Rule::multiplicative_expression,
    "a / b",
    "multiplicative expression with / operator"
)]
#[case(
    Rule::multiplicative_expression,
    "a % b",
    "multiplicative expression with % operator"
)]
fn test_parse_multiplicative_expression(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::multiplicative_operator, "*", "multiplicative operator *")]
#[case(Rule::multiplicative_operator, "/", "multiplicative operator /")]
#[case(Rule::multiplicative_operator, "%", "multiplicative operator %")]
fn test_parse_multiplicative_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(
    Rule::exponentiation_expression,
    "value",
    "exponentiation expression with single value"
)]
#[case(
    Rule::exponentiation_expression,
    "a ** b",
    "exponentiation expression with ** operator"
)]
#[case(
    Rule::exponentiation_expression,
    "a ^ b",
    "exponentiation expression with ^ operator"
)]
fn test_parse_exponentiation_expression(
    #[case] rule: Rule,
    #[case] input: &str,
    #[case] desc: &str,
) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::exponentiation_operator, "**", "exponentiation operator **")]
#[case(Rule::exponentiation_operator, "^", "exponentiation operator ^")]
fn test_parse_exponentiation_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::unary_expression, "value", "unary expression with identifier")]
#[case(Rule::unary_expression, "+value", "unary expression with + operator")]
#[case(Rule::unary_expression, "-value", "unary expression with - operator")]
#[case(Rule::unary_expression, "~value", "unary expression with ~ operator")]
#[case(
    Rule::unary_expression,
    "not value",
    "unary expression with not operator"
)]
fn test_parse_unary_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::unary_operator, "+", "unary operator +")]
#[case(Rule::unary_operator, "-", "unary operator -")]
#[case(Rule::unary_operator, "~", "unary operator ~")]
#[case(Rule::unary_operator, "not", "unary operator not")]
fn test_parse_unary_operator(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::extent_expression, "value", "extent expression with identifier")]
#[case(Rule::extent_expression, "42", "extent expression with number")]
fn test_parse_extent_expression(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::string_value, "\"hello world\"", "string literal")]
#[case(Rule::string_value, "\"\"", "empty string")]
fn test_parse_string_value(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::numeric_value, "42", "positive integer")]
#[case(Rule::numeric_value, "-42", "negative integer")]
#[case(Rule::numeric_value, "3.14", "decimal number")]
#[case(Rule::numeric_value, "-3.14", "negative decimal")]
fn test_parse_numeric_value(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::boolean_value, "true", "boolean true")]
#[case(Rule::boolean_value, "false", "boolean false")]
fn test_parse_boolean_value(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

#[rstest]
#[case(Rule::literal, "\"string\"", "literal with string")]
#[case(Rule::literal, "42", "literal with number")]
#[case(Rule::literal, "true", "literal with boolean")]
#[case(Rule::literal, "null", "literal with null")]
fn test_parse_literal(#[case] rule: Rule, #[case] input: &str, #[case] desc: &str) {
    assert_round_trip(rule, input, desc);
}

// =============================================================================
// Grammar Pattern Extraction Tests
// These tests validate extracted common patterns for better error messages
// =============================================================================

/// Tests transition_target pattern (then X | if guard then X | else X)
#[rstest]
#[case("then a", "simple then target")]
#[case("then a.b", "then with feature chain")]
#[case("if guard then x", "guarded target")]
#[case("else fallback", "default target")]
fn test_parse_transition_target(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::transition_target, input);
    assert!(
        result.is_ok(),
        "Failed to parse transition_target '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests typed_reference pattern (subsetting with optional specializations)
#[rstest]
#[case("MyType", "simple type reference")]
#[case("Package::Type", "qualified type reference")]
#[case("MyType :> Base", "reference with specialization")]
fn test_parse_typed_reference(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::typed_reference, input);
    assert!(
        result.is_ok(),
        "Failed to parse typed_reference '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests action_declaration_header pattern
#[rstest]
#[case("action", "just action keyword")]
#[case("action myAction", "action with name")]
#[case("action myAction : ActionType", "action with name and type")]
fn test_parse_action_declaration_header(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::action_declaration_header, input);
    assert!(
        result.is_ok(),
        "Failed to parse action_declaration_header '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests reference_chain pattern (subsetting with zero or more specializations)
#[rstest]
#[case("MyType", "simple type reference")]
#[case("Package::Type", "qualified type reference")]
#[case("MyType :> Base", "reference with one specialization")]
#[case("MyType :> Base, Other", "reference with multiple specializations")]
fn test_parse_reference_chain(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::reference_chain, input);
    assert!(
        result.is_ok(),
        "Failed to parse reference_chain '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Tests succession_header pattern
#[rstest]
#[case("succession", "just succession keyword")]
#[case("succession mySuccession", "succession with name")]
fn test_parse_succession_header(#[case] input: &str, #[case] desc: &str) {
    let result = SysMLParser::parse(Rule::succession_header, input);
    assert!(
        result.is_ok(),
        "Failed to parse succession_header '{}' ({}): {:?}",
        input,
        desc,
        result.err()
    );
}

/// Regression test for RequirementTest.sysml parsing
#[test]
fn test_parse_package_with_constraints_and_import() {
    let input = r#"package RequirementTest {
        constraint def C;
        constraint c : C;
        private import q::**;
    }"#;
    let result = SysMLParser::parse(Rule::model, input);
    assert!(
        result.is_ok(),
        "Failed to parse package with constraints and import: {:?}",
        result.err()
    );
}
