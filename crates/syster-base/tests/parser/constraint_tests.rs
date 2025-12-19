use pest::Parser;
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_assert_constraint_with_expression() {
    let source = r#"
        package Test {
            item def Satellite {
                assert constraint { mass < 1000.0 }
            }
        }
    "#;
    let result = SysMLParser::parse(Rule::file, source);
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
}

#[test]
fn test_assert_constraint_empty() {
    let source = r#"
        package Test {
            item def Satellite {
                assert constraint { true }
            }
        }
    "#;
    assert!(SysMLParser::parse(Rule::file, source).is_ok());
}
