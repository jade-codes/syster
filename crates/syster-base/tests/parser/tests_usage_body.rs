use pest::Parser;
use syster::parser::{SysMLParser, sysml::Rule};

/// Test that usage bodies parse correctly (smoke test)
#[test]
fn test_usage_body_parsing() {
    let source = r#"
        package Test {
            requirement def MyReq {
                doc /* comment */
            }
        }
    "#;
    assert!(SysMLParser::parse(Rule::file, source).is_ok());
}

#[test]
fn test_empty_usage_body() {
    let source = r#"package Test { requirement req {} }"#;
    assert!(SysMLParser::parse(Rule::file, source).is_ok());
}
