#![allow(clippy::unwrap_used)]

use super::*;
use crate::language::sysml::syntax::{Element, Import, SysMLFile};

#[test]
fn test_extract_no_imports() {
    // TDD: File with no imports returns empty vec
    let file = SysMLFile {
        namespace: None,
        elements: vec![],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 0);
}

#[test]
fn test_extract_single_import() {
    // TDD: File with one import statement
    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Import(Import {
            path: "Base::Vehicle".to_string(),
            is_recursive: false,
            span: None,
        })],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "Base::Vehicle");
}

#[test]
fn test_extract_multiple_imports() {
    // TDD: File with multiple import statements
    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Import(Import {
                path: "Base::Vehicle".to_string(),
                is_recursive: false,
                span: None,
            }),
            Element::Import(Import {
                path: "Systems::Engine".to_string(),
                is_recursive: false,
                span: None,
            }),
            Element::Import(Import {
                path: "Utils::*".to_string(),
                is_recursive: true,
                span: None,
            }),
        ],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 3);
    assert!(imports.contains(&"Base::Vehicle".to_string()));
    assert!(imports.contains(&"Systems::Engine".to_string()));
    assert!(imports.contains(&"Utils::*".to_string()));
}

#[test]
fn test_extract_recursive_imports() {
    // TDD: Wildcard imports should be captured
    let file = SysMLFile {
        namespace: None,
        elements: vec![Element::Import(Import {
            path: "SysML::*".to_string(),
            is_recursive: true,
            span: None,
        })],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "SysML::*");
}

#[test]
fn test_extract_imports_mixed_elements() {
    // TDD: Should extract imports even with other elements present
    use crate::language::sysml::syntax::types::NamespaceDeclaration;

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "MyPackage".to_string(),
            span: None,
        }),
        elements: vec![
            Element::Import(Import {
                path: "Base::Vehicle".to_string(),
                is_recursive: false,
                span: None,
            }),
            Element::Comment(crate::language::sysml::syntax::Comment {
                content: "Some comment".to_string(),
                span: None,
            }),
            Element::Import(Import {
                path: "Systems::Engine".to_string(),
                is_recursive: false,
                span: None,
            }),
        ],
    };

    let imports = extract_imports(&file);
    assert_eq!(imports.len(), 2);
}

#[test]
fn test_parse_namespace_path() {
    // TDD: Parse import path into components
    let path = "Base::Components::Vehicle";
    let parts = parse_import_path(path);

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "Base");
    assert_eq!(parts[1], "Components");
    assert_eq!(parts[2], "Vehicle");
}

#[test]
fn test_parse_wildcard_import() {
    // TDD: Wildcard imports parse correctly
    let path = "SysML::*";
    let parts = parse_import_path(path);

    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "SysML");
    assert_eq!(parts[1], "*");
}

#[test]
fn test_is_wildcard_import() {
    // TDD: Detect wildcard imports
    assert!(is_wildcard_import("SysML::*"));
    assert!(is_wildcard_import("Base::Components::*"));
    assert!(!is_wildcard_import("Base::Vehicle"));
    assert!(!is_wildcard_import("SysML::Items"));
}
