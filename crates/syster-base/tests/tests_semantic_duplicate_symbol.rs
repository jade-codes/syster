use std::path::PathBuf;
use syster::semantic::Workspace;
use syster::syntax::SyntaxFile;
use syster::syntax::sysml::parser::parse_content;

#[test]
fn test_duplicate_symbols_in_different_scopes_allowed() {
    // Test that symbols with the same name in different scopes are allowed
    // This happens in VerificationCases.sysml where 'requirementVerifications'
    // appears in two different nested scopes (lines 27 and 35)

    let test_sysml = r#"
        package TestPackage {
            part def OuterPart {
                requirement requirementVerifications;
            }
            
            part def InnerPart {
                requirement requirementVerifications;
            }
        }
    "#;

    let mut workspace = Workspace::<SyntaxFile>::new();

    // Parse the test file
    let parsed = parse_content(test_sysml, &PathBuf::from("test.sysml"));
    assert!(
        parsed.is_ok(),
        "Test file should parse successfully: {:?}",
        parsed.err()
    );

    workspace.add_file("test.sysml".into(), SyntaxFile::SysML(parsed.unwrap()));

    // This should NOT fail - same names in different scopes should be allowed
    let result = workspace.populate_all();

    assert!(
        result.is_ok(),
        "Should allow symbols with same name in different scopes. Error: {:?}",
        result.err()
    );

    // Both symbols should exist with their qualified names
    let symbols: Vec<_> = workspace.symbol_table().iter_symbols().collect();
    for _sym in symbols.iter().take(10) {}

    // Check by qualified name in the symbol itself (not the key)
    let outer_req = symbols.iter().find(|sym| {
        sym.qualified_name().contains("OuterPart") && sym.name() == "requirementVerifications"
    });
    let inner_req = symbols.iter().find(|sym| {
        sym.qualified_name().contains("InnerPart") && sym.name() == "requirementVerifications"
    });

    assert!(
        outer_req.is_some(),
        "Should find TestPackage::OuterPart::requirementVerifications"
    );
    assert!(
        inner_req.is_some(),
        "Should find TestPackage::InnerPart::requirementVerifications"
    );
}
