#![allow(clippy::unwrap_used)]

use std::path::Path;
use syster::syntax::kerml::parser::parse_content;

#[test]
fn test_parse_scalar_values_file() {
    let content = r#"standard library package ScalarValues {
    private import Base::DataValue;
    abstract datatype ScalarValue specializes DataValue;
    datatype Boolean specializes ScalarValue;
    datatype String specializes ScalarValue;
}"#;

    let path = Path::new("ScalarValues.kerml");
    let result = parse_content(content, path);

    match result {
        Ok(file) => {
            println!("Namespace: {:?}", file.namespace);
            println!("Elements count: {}", file.elements.len());
            for (i, elem) in file.elements.iter().enumerate() {
                println!("  Element {i}: {elem:?}");

                // Check if it's a package with body elements
                if let syster::syntax::kerml::ast::Element::Package(pkg) = elem {
                    println!(
                        "    Package '{}' has {} body elements",
                        pkg.name.as_ref().unwrap_or(&"unnamed".to_string()),
                        pkg.elements.len()
                    );
                    for (j, body_elem) in pkg.elements.iter().enumerate() {
                        println!("      Body element {j}: {body_elem:?}");
                    }
                }
            }

            assert!(!file.elements.is_empty(), "File should have elements");

            // Check that the package has body elements (the datatypes)
            if let syster::syntax::kerml::ast::Element::Package(pkg) = &file.elements[0] {
                assert!(
                    !pkg.elements.is_empty(),
                    "Package should have body elements (datatypes, imports, etc.)"
                );
            }
        }
        Err(e) => {
            panic!("Failed to parse: {e}");
        }
    }
}
