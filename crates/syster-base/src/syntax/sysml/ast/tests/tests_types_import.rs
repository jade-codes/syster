#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use crate::core::Span;

// ============================================================================
// Import struct tests
// ============================================================================

#[test]
fn test_import_creation() {
    let import = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "Package::Element");
    assert!(!import.is_recursive);
    assert_eq!(import.span, None);
}

#[test]
fn test_import_with_span() {
    let span = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 20,
        },
    };

    let import = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: Some(span),
    };

    assert_eq!(import.path, "Package::*");
    assert!(!import.is_recursive);
    assert_eq!(import.span, Some(span));
}

#[test]
fn test_import_recursive() {
    let import = Import {
        path: "Package::*::**".to_string(),
        path_span: None,
        is_recursive: true,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "Package::*::**");
    assert!(import.is_recursive);
}

#[test]
fn test_import_non_recursive() {
    let import = Import {
        path: "Package::Member".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "Package::Member");
    assert!(!import.is_recursive);
}

#[test]
fn test_import_simple_path() {
    let import = Import {
        path: "SimplePackage".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "SimplePackage");
    assert!(!import.is_recursive);
}

#[test]
fn test_import_wildcard_path() {
    let import = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "Package::*");
    assert!(!import.is_recursive);
}

#[test]
fn test_import_recursive_wildcard_path() {
    let import = Import {
        path: "Package::*::**".to_string(),
        path_span: None,
        is_recursive: true,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "Package::*::**");
    assert!(import.is_recursive);
}

#[test]
fn test_import_empty_path() {
    let import = Import {
        path: String::new(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "");
    assert!(import.path.is_empty());
}

#[test]
fn test_import_clone() {
    let import1 = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: true,
        is_public: false,
        span: None,
    };

    let import2 = import1.clone();

    assert_eq!(import1.path, import2.path);
    assert_eq!(import1.is_recursive, import2.is_recursive);
    assert_eq!(import1.span, import2.span);
}

#[test]
fn test_import_partial_eq() {
    let import1 = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    let import2 = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import1, import2);
}

#[test]
fn test_import_not_eq_different_path() {
    let import1 = Import {
        path: "Package1::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    let import2 = Import {
        path: "Package2::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_ne!(import1, import2);
}

#[test]
fn test_import_not_eq_different_recursive() {
    let import1 = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: true,
        is_public: false,
        span: None,
    };

    let import2 = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_ne!(import1, import2);
}

#[test]
fn test_import_not_eq_different_span() {
    let span1 = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 10,
        },
    };

    let span2 = Span {
        start: crate::core::span::Position { line: 2, column: 0 },
        end: crate::core::span::Position {
            line: 2,
            column: 10,
        },
    };

    let import1 = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: Some(span1),
    };

    let import2 = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: Some(span2),
    };

    assert_ne!(import1, import2);
}

#[test]
fn test_import_debug_trait() {
    let import = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    let debug_str = format!("{import:?}");
    assert!(debug_str.contains("Import"));
    assert!(debug_str.contains("Package::Element"));
}

// ============================================================================
// Import as Element tests
// ============================================================================

#[test]
fn test_import_as_element() {
    let import = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    let element = Element::Import(import.clone());

    match element {
        Element::Import(i) => {
            assert_eq!(i.path, "Package::*");
            assert!(!i.is_recursive);
            assert_eq!(i, import);
        }
        _ => panic!("Expected Element::Import variant"),
    }
}

#[test]
fn test_import_element_pattern_matching() {
    let import = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: true,
        is_public: false,
        span: None,
    };

    let element = Element::Import(import);

    if let Element::Import(i) = element {
        assert_eq!(i.path, "Package::Element");
        assert!(i.is_recursive);
    } else {
        panic!("Failed to match Element::Import");
    }
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_import_very_long_path() {
    let long_path = format!("{}::Element", "Package".repeat(100));
    let import = Import {
        path: long_path.clone(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, long_path);
    assert_eq!(import.path.len(), 700 + "::Element".len());
}

#[test]
fn test_import_complex_qualified_path() {
    let import = Import {
        path: "RootPackage::SubPackage::NestedPackage::Element".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(
        import.path,
        "RootPackage::SubPackage::NestedPackage::Element"
    );
    assert!(import.path.contains("::"));
}

#[test]
fn test_import_with_special_characters() {
    let import = Import {
        path: "Package_123::Element_456".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "Package_123::Element_456");
    assert!(import.path.contains('_'));
}

#[test]
fn test_import_unicode_path() {
    // While SysML identifiers typically use ASCII, we test that the Import struct
    // can handle Unicode strings if they are provided
    let unicode_path = "Package::元素".to_string();
    let import = Import {
        path: unicode_path.clone(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, unicode_path);
}

#[test]
fn test_import_wildcard_only() {
    let import = Import {
        path: "*".to_string(),
        path_span: None,
        is_recursive: false,
        is_public: false,
        span: None,
    };

    assert_eq!(import.path, "*");
}

#[test]
fn test_import_both_flags_and_span() {
    let span = Span {
        start: crate::core::span::Position { line: 1, column: 0 },
        end: crate::core::span::Position {
            line: 1,
            column: 25,
        },
    };

    let import = Import {
        path: "Package::*::**".to_string(),
        path_span: None,
        is_recursive: true,
        is_public: false,
        span: Some(span),
    };

    assert_eq!(import.path, "Package::*::**");
    assert!(import.is_recursive);
    assert!(import.span.is_some());
}
