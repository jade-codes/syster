#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::super::*;
use crate::core::Span;
use crate::syntax::sysml::visitor::{AstVisitor, Visitable};

// ============================================================================
// Import struct tests
// ============================================================================

#[test]
fn test_import_creation() {
    let import = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
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
        span: None,
    };

    let import2 = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
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
        span: None,
    };

    let import2 = Import {
        path: "Package2::Element".to_string(),
        path_span: None,
        is_recursive: false,
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
        span: None,
    };

    let import2 = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
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
        span: Some(span1),
    };

    let import2 = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
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
        span: None,
    };

    let debug_str = format!("{:?}", import);
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
// Visitable trait tests (Issue #170)
// ============================================================================

struct GenericTestVisitor {
    import_visited: bool,
    import_path: Option<String>,
    import_is_recursive: Option<bool>,
}

impl AstVisitor for GenericTestVisitor {
    fn visit_import(&mut self, import: &Import) {
        self.import_visited = true;
        self.import_path = Some(import.path.clone());
        self.import_is_recursive = Some(import.is_recursive);
    }
}

#[test]
fn test_import_visitable_accept_generic() {
    let import = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        import_visited: false,
        import_path: None,
        import_is_recursive: None,
    };

    import.accept(&mut visitor);

    assert!(visitor.import_visited, "Import should be visited");
    assert_eq!(
        visitor.import_path,
        Some("Package::Element".to_string()),
        "Visitor should capture import path"
    );
    assert_eq!(
        visitor.import_is_recursive,
        Some(false),
        "Visitor should capture is_recursive flag"
    );
}

#[test]
fn test_import_visitable_with_multiple_visitors() {
    let import = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };

    let mut visitor1 = GenericTestVisitor {
        import_visited: false,
        import_path: None,
        import_is_recursive: None,
    };

    let mut visitor2 = GenericTestVisitor {
        import_visited: false,
        import_path: None,
        import_is_recursive: None,
    };

    import.accept(&mut visitor1);
    import.accept(&mut visitor2);

    assert!(visitor1.import_visited);
    assert!(visitor2.import_visited);
    assert_eq!(visitor1.import_path, visitor2.import_path);
    assert_eq!(visitor1.import_is_recursive, visitor2.import_is_recursive);
}

#[test]
fn test_import_visitable_with_span() {
    let span = Span {
        start: crate::core::span::Position {
            line: 5,
            column: 10,
        },
        end: crate::core::span::Position {
            line: 5,
            column: 30,
        },
    };

    let import = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        span: Some(span),
    };

    let mut visitor = GenericTestVisitor {
        import_visited: false,
        import_path: None,
        import_is_recursive: None,
    };

    import.accept(&mut visitor);

    assert!(visitor.import_visited);
    assert_eq!(visitor.import_path, Some("Package::Element".to_string()));
}

#[test]
fn test_import_visitable_recursive() {
    let import = Import {
        path: "Package::*::**".to_string(),
        path_span: None,
        is_recursive: true,
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        import_visited: false,
        import_path: None,
        import_is_recursive: None,
    };

    import.accept(&mut visitor);

    assert!(visitor.import_visited);
    assert_eq!(visitor.import_path, Some("Package::*::**".to_string()));
    assert_eq!(visitor.import_is_recursive, Some(true));
}

#[test]
fn test_import_visitable_non_recursive() {
    let import = Import {
        path: "Package::Member".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };

    let mut visitor = GenericTestVisitor {
        import_visited: false,
        import_path: None,
        import_is_recursive: None,
    };

    import.accept(&mut visitor);

    assert!(visitor.import_visited);
    assert_eq!(visitor.import_path, Some("Package::Member".to_string()));
    assert_eq!(visitor.import_is_recursive, Some(false));
}

// ============================================================================
// ImportCountingVisitor tests (Issue #169)
// ============================================================================

/// A visitor that counts import visits and total visit calls.
/// This is separate from the CountingVisitor in tests.rs which tracks all element types.
/// We use a focused visitor here to specifically test import visitor behavior.
struct ImportCountingVisitor {
    imports: usize,
    total_visits: usize,
}

impl AstVisitor for ImportCountingVisitor {
    fn visit_import(&mut self, _import: &Import) {
        self.imports += 1;
        self.total_visits += 1;
    }

    fn visit_element(&mut self, _element: &Element) {
        self.total_visits += 1;
    }
}

#[test]
fn test_import_visitable_accept_counting_visitor() {
    let import = Import {
        path: "Package::Element".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };

    let mut visitor = ImportCountingVisitor {
        imports: 0,
        total_visits: 0,
    };

    import.accept(&mut visitor);

    assert_eq!(visitor.imports, 1, "Should visit exactly one import");
    assert_eq!(
        visitor.total_visits, 1,
        "Total visits should match import visits"
    );
}

#[test]
fn test_import_visitable_counting_multiple_imports() {
    let import1 = Import {
        path: "Package1::*".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };
    let import2 = Import {
        path: "Package2::Element".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };
    let import3 = Import {
        path: "Package3::*::**".to_string(),
        path_span: None,
        is_recursive: true,
        span: None,
    };

    let mut visitor = ImportCountingVisitor {
        imports: 0,
        total_visits: 0,
    };

    import1.accept(&mut visitor);
    import2.accept(&mut visitor);
    import3.accept(&mut visitor);

    assert_eq!(visitor.imports, 3, "Should count all three imports");
    assert_eq!(visitor.total_visits, 3);
}

#[test]
fn test_import_element_with_counting_visitor() {
    let import = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };
    let element = Element::Import(import);

    let mut visitor = ImportCountingVisitor {
        imports: 0,
        total_visits: 0,
    };

    element.accept(&mut visitor);

    assert_eq!(visitor.imports, 1, "Should count import through element");
    // Element visitor calls visit_element then visit_import
    assert_eq!(
        visitor.total_visits, 2,
        "Should count both element and import visits"
    );
}

#[test]
fn test_import_counting_visitor_zero_initial() {
    let visitor = ImportCountingVisitor {
        imports: 0,
        total_visits: 0,
    };

    assert_eq!(visitor.imports, 0, "Initial import count should be zero");
    assert_eq!(
        visitor.total_visits, 0,
        "Initial total visits should be zero"
    );
}

#[test]
fn test_import_in_file_with_counting_visitor() {
    let import1 = Import {
        path: "Package1::*".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };
    let import2 = Import {
        path: "Package2::Element".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Import(import1), Element::Import(import2)],
    };

    let mut visitor = ImportCountingVisitor {
        imports: 0,
        total_visits: 0,
    };

    file.accept(&mut visitor);

    assert_eq!(visitor.imports, 2, "Should count both imports in file");
    assert_eq!(
        visitor.total_visits, 4,
        "Should count 2 element visits + 2 import visits"
    );
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
        span: Some(span),
    };

    assert_eq!(import.path, "Package::*::**");
    assert!(import.is_recursive);
    assert!(import.span.is_some());
}

#[test]
fn test_import_in_package_with_counting_visitor() {
    let import = Import {
        path: "Package::*".to_string(),
        path_span: None,
        is_recursive: false,
        span: None,
    };

    let package = Package {
        name: Some("TestPkg".to_string()),
        elements: vec![Element::Import(import)],
        span: None,
    };

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Package(package)],
    };

    let mut visitor = ImportCountingVisitor {
        imports: 0,
        total_visits: 0,
    };

    file.accept(&mut visitor);

    assert_eq!(visitor.imports, 1, "Should count import in package");
}
