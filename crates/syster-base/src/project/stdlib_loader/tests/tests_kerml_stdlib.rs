#![allow(clippy::unwrap_used)]

use crate::syntax::kerml::ast::*;
use crate::syntax::kerml::parser::parse_content;
use std::path::Path;

#[test]
fn test_parse_scalar_values_file() {
    let content = r#"standard library package ScalarValues {
    private import Base::DataValue;
    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
    datatype String specializes ScalarValue;
}"#;

    let path = Path::new("ScalarValues.kerml");
    let file = parse_content(content, path).expect("Should parse ScalarValues.kerml successfully");
    for elem in file.elements.iter() {
        // Check if it's a package with body elements
        if let Element::Package(pkg) = elem {
            for _body_elem in pkg.elements.iter() {}
        }
    }

    assert!(!file.elements.is_empty(), "File should have elements");

    // Check that the package has body elements (the datatypes)
    let Element::Package(pkg) = &file.elements[0] else {
        panic!(
            "First element should be a Package, got {:?}",
            file.elements[0]
        );
    };
    assert!(
        !pkg.elements.is_empty(),
        "Package should have body elements (datatypes, imports, etc.)"
    );
}
