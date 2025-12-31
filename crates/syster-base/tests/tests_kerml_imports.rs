use syster::semantic::extract_kerml_imports;
use syster::syntax::kerml::ast::enums::ImportKind;
use syster::syntax::kerml::ast::{Element, Import, KerMLFile, NamespaceDeclaration};

#[test]
fn test_extract_kerml_imports_empty_file() {
    let file = KerMLFile {
        namespace: None,
        elements: vec![],
    };

    let imports = extract_kerml_imports(&file);
    assert_eq!(imports, Vec::<String>::new());
}

#[test]
fn test_extract_kerml_imports_single_import() {
    let file = KerMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "TestNamespace".to_string(),
            span: None,
        }),
        elements: vec![Element::Import(Import {
            path: "Base::DataValue".to_string(),
            is_recursive: false,
            kind: ImportKind::Normal,
            span: None,
        })],
    };

    let imports = extract_kerml_imports(&file);
    assert_eq!(imports, vec!["Base::DataValue"]);
}

#[test]
fn test_extract_kerml_imports_multiple_imports() {
    let file = KerMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "TestNamespace".to_string(),
            span: None,
        }),
        elements: vec![
            Element::Import(Import {
                path: "Base::DataValue".to_string(),
                is_recursive: false,
                kind: ImportKind::Normal,
                span: None,
            }),
            Element::Import(Import {
                path: "Standard::Functions".to_string(),
                is_recursive: true,
                kind: ImportKind::Recursive,
                span: None,
            }),
        ],
    };

    let imports = extract_kerml_imports(&file);
    assert_eq!(imports, vec!["Base::DataValue", "Standard::Functions"]);
}

#[test]
fn test_extract_kerml_imports_mixed_elements() {
    use syster::syntax::kerml::ast::{Comment, Package};

    let file = KerMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "TestNamespace".to_string(),
            span: None,
        }),
        elements: vec![
            Element::Import(Import {
                path: "Base::DataValue".to_string(),
                is_recursive: false,
                kind: ImportKind::Normal,
                span: None,
            }),
            Element::Package(Package {
                name: Some("MyPackage".to_string()),
                elements: vec![],
                span: None,
            }),
            Element::Comment(Comment {
                content: "Test comment".to_string(),
                about: vec![],
                locale: None,
                span: None,
            }),
            Element::Import(Import {
                path: "Standard::Functions".to_string(),
                is_recursive: false,
                kind: ImportKind::Normal,
                span: None,
            }),
        ],
    };

    let imports = extract_kerml_imports(&file);
    assert_eq!(imports, vec!["Base::DataValue", "Standard::Functions"]);
}
