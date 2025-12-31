/// Test for supporting multiple packages per file (Issue #10)
///
/// This test verifies that SysML files can contain multiple package declarations
/// and that all packages are properly tracked in the AST.
use std::path::Path;
use syster::syntax::sysml::parser::parse_content;

#[test]
fn test_multiple_packages_in_single_file() {
    let input = r#"
        package Vehicle;
        package Engine;
        package Transmission;
    "#;

    let result = parse_content(input, Path::new("test.sysml"));
    assert!(result.is_ok(), "Parse should succeed");

    let sysml_file = result.unwrap();

    // Should track all namespace declarations, not just the first one
    assert_eq!(
        sysml_file.namespaces.len(),
        3,
        "Should find all 3 package declarations"
    );

    // Verify each package name
    assert_eq!(sysml_file.namespaces[0].name, "Vehicle");
    assert_eq!(sysml_file.namespaces[1].name, "Engine");
    assert_eq!(sysml_file.namespaces[2].name, "Transmission");

    // Backward compatibility: namespace field should still contain the first package
    assert!(sysml_file.namespace.is_some());
    assert_eq!(sysml_file.namespace.as_ref().unwrap().name, "Vehicle");
}

#[test]
fn test_single_package_backward_compatibility() {
    let input = "package SinglePackage;";

    let result = parse_content(input, Path::new("test.sysml"));
    assert!(result.is_ok());

    let sysml_file = result.unwrap();

    // Single package case
    assert_eq!(sysml_file.namespaces.len(), 1);
    assert_eq!(sysml_file.namespaces[0].name, "SinglePackage");

    // namespace field should still work
    assert!(sysml_file.namespace.is_some());
    assert_eq!(sysml_file.namespace.as_ref().unwrap().name, "SinglePackage");
}

#[test]
fn test_no_packages() {
    let input = "part myPart;";

    let result = parse_content(input, Path::new("test.sysml"));
    assert!(result.is_ok());

    let sysml_file = result.unwrap();

    // No packages
    assert_eq!(sysml_file.namespaces.len(), 0);
    assert!(sysml_file.namespace.is_none());
}
