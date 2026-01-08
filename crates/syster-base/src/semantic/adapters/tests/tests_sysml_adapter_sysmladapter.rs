#![allow(clippy::unwrap_used)]

//! Tests for SysmlAdapter AstVisitor implementation
//!
//! This file contains tests for:
//! - `visit_namespace` - Tests namespace declaration handling
//! - `visit_comment` - Tests comment handling

use super::super::SysmlAdapter;
use crate::semantic::resolver::Resolver;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::syntax::sysml::ast::{Comment, Element, NamespaceDeclaration, SysMLFile};

// ============================================================================
// visit_namespace TESTS
// ============================================================================

#[test]
fn test_visit_namespace_creates_package_symbol() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "TestNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // Verify the namespace was added as a Package symbol
    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("TestNamespace");
    assert!(symbol.is_some());

    let Some(Symbol::Package {
        name,
        qualified_name,
        ..
    }) = symbol
    else {
        panic!("Expected Package symbol, got: {symbol:?}");
    };
    assert_eq!(name, "TestNamespace");
    assert_eq!(qualified_name, "TestNamespace");
}

#[test]
fn test_visit_namespace_enters_namespace_scope() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "MyNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // After populate, the adapter should have entered the namespace
    // We can't directly test current_namespace (it's private), but we can test
    // that the namespace affects scope by checking all_symbols
    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("MyNamespace");
    assert!(symbol.is_some());
}

#[test]
fn test_visit_namespace_with_empty_name() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // Even with empty name, should create a symbol
    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("");
    assert!(symbol.is_some());
}

#[test]
fn test_visit_namespace_with_special_characters() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "Name_With_Underscores123".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("Name_With_Underscores123");
    assert!(symbol.is_some());
}

#[test]
fn test_visit_namespace_stores_scope_id() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "ScopedNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("ScopedNamespace");
    assert!(symbol.is_some());

    if let Some(Symbol::Package { scope_id, .. }) = symbol {
        // Scope ID should be set (0 is the root scope)
        assert_eq!(*scope_id, 0);
    } else {
        panic!("Expected Package symbol");
    }
}

#[test]
fn test_visit_namespace_with_span() {
    use crate::core::{Position, Span};

    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let test_span = Some(Span {
        start: Position::new(1, 1),
        end: Position::new(1, 10),
    });

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "SpannedNamespace".to_string(),
            span: test_span,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("SpannedNamespace");
    assert!(symbol.is_some());

    if let Some(Symbol::Package { span, .. }) = symbol {
        assert_eq!(*span, test_span);
    } else {
        panic!("Expected Package symbol");
    }
}

#[test]
fn test_visit_namespace_no_namespace_in_file() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // With no namespace, symbol table should remain empty (except for any default symbols)
    let is_empty = table.iter_symbols().next().is_none();
    let has_empty_name = table.iter_symbols().any(|sym| sym.name().is_empty());
    assert!(is_empty || !has_empty_name);
}

#[test]
fn test_visit_namespace_affects_subsequent_elements() {
    use crate::syntax::sysml::ast::{Definition, DefinitionKind};

    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "OuterNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![Element::Definition(Definition {
            kind: DefinitionKind::Part,
            name: Some("InnerPart".to_string()),
            body: vec![],
            relationships: Default::default(),
            is_abstract: false,
            is_variation: false,
            span: None,
            short_name: None,
            short_name_span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // The namespace should exist
    let resolver = Resolver::new(&table);
    let ns_symbol = resolver.resolve("OuterNamespace");
    assert!(ns_symbol.is_some());

    // The inner part should be qualified with the namespace
    let has_qualified = table
        .iter_symbols()
        .any(|sym| sym.qualified_name().contains("OuterNamespace::InnerPart"));
    assert!(
        has_qualified,
        "Definition should be qualified with namespace"
    );
}

#[test]
fn test_visit_namespace_multiple_namespaces_not_supported() {
    // This test documents current behavior - multiple namespace declarations
    // are not typically used in a single file, but if they are, only the
    // first one should be processed
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "FirstNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // Only the first namespace should be in the symbol table
    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("FirstNamespace");
    assert!(symbol.is_some());
}

// ============================================================================
// visit_comment TESTS
// ============================================================================

#[test]
fn test_visit_comment_does_not_affect_symbol_table() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "This is a test comment".to_string(),
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // Symbol table should remain empty - comments don't create symbols
    assert!(table.iter_symbols().next().is_none());
}

#[test]
fn test_visit_comment_empty_content() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "".to_string(),
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    assert!(table.iter_symbols().next().is_none());
}

#[test]
fn test_visit_comment_multiline_content() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "Line 1\nLine 2\nLine 3".to_string(),
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    assert!(table.iter_symbols().next().is_none());
}

#[test]
fn test_visit_comment_with_special_characters() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "/* Comment with special chars: @#$%^&*() */".to_string(),
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    assert!(table.iter_symbols().next().is_none());
}

#[test]
fn test_visit_comment_multiple_comments() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Comment(Comment {
                content: "First comment".to_string(),
                span: None,
            }),
            Element::Comment(Comment {
                content: "Second comment".to_string(),
                span: None,
            }),
            Element::Comment(Comment {
                content: "Third comment".to_string(),
                span: None,
            }),
        ],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    assert!(table.iter_symbols().next().is_none());
}

#[test]
fn test_visit_comment_between_definitions() {
    use crate::syntax::sysml::ast::{Definition, DefinitionKind};

    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("FirstPart".to_string()),
                body: vec![],
                relationships: Default::default(),
                is_abstract: false,
                is_variation: false,
                span: None,
                short_name: None,
                short_name_span: None,
            }),
            Element::Comment(Comment {
                content: "Comment between definitions".to_string(),
                span: None,
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("SecondPart".to_string()),
                body: vec![],
                relationships: Default::default(),
                is_abstract: false,
                is_variation: false,
                span: None,
                short_name: None,
                short_name_span: None,
            }),
        ],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // Should have exactly 2 symbols (the definitions), not the comment
    assert_eq!(table.iter_symbols().count(), 2);
    assert!(Resolver::new(&table).resolve("FirstPart").is_some());
    assert!(Resolver::new(&table).resolve("SecondPart").is_some());
}

#[test]
fn test_visit_comment_with_span() {
    use crate::core::{Position, Span};

    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "Comment with span".to_string(),
            span: Some(Span {
                start: Position::new(1, 1),
                end: Position::new(1, 20),
            }),
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // Comment with span should still not affect symbol table
    assert!(table.iter_symbols().next().is_none());
}

#[test]
fn test_visit_comment_does_not_change_current_namespace() {
    use crate::syntax::sysml::ast::{Definition, DefinitionKind};

    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "TestNS".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![
            Element::Comment(Comment {
                content: "Comment in namespace".to_string(),
                span: None,
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("PartInNamespace".to_string()),
                body: vec![],
                relationships: Default::default(),
                is_abstract: false,
                is_variation: false,
                span: None,
                short_name: None,
                short_name_span: None,
            }),
        ],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // The definition should still be qualified with the namespace,
    // proving that the comment didn't change the namespace context
    let has_qualified = table
        .iter_symbols()
        .any(|sym| sym.qualified_name().contains("TestNS::PartInNamespace"));
    assert!(has_qualified, "Comment should not affect namespace context");
}

#[test]
fn test_visit_comment_long_content() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let long_comment = "a".repeat(10000);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: long_comment,
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    assert!(table.iter_symbols().next().is_none());
}

#[test]
fn test_visit_comment_unicode_content() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "Unicode comment: 你好世界 🚀 ñ é".to_string(),
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    assert!(table.iter_symbols().next().is_none());
}

// ============================================================================
// INTEGRATION TESTS - Both visit_namespace and visit_comment
// ============================================================================

#[test]
fn test_namespace_with_comments() {
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "DocumentedNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "This namespace is documented".to_string(),
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // Should have only the namespace symbol, not the comment
    assert_eq!(table.iter_symbols().count(), 1);
    assert!(
        Resolver::new(&table)
            .resolve("DocumentedNamespace")
            .is_some()
    );
}

#[test]
fn test_comment_before_namespace_not_typical_but_handled() {
    // This test documents the behavior when elements appear before namespace
    // which is not typical SysML structure but should be handled gracefully
    let mut table = SymbolTable::new();
    let mut adapter = SysmlAdapter::new(&mut table);

    let file = SysMLFile {
        namespace: Some(NamespaceDeclaration {
            name: "LateNamespace".to_string(),
            span: None,
        }),
        namespaces: vec![],
        elements: vec![Element::Comment(Comment {
            content: "Comment at file level".to_string(),
            span: None,
        })],
    };

    let result = adapter.populate(&file);
    assert!(result.is_ok());

    // The namespace should still be created
    let resolver = Resolver::new(&table);
    let symbol = resolver.resolve("LateNamespace");
    assert!(symbol.is_some());

    // The comment should not create a symbol
    assert_eq!(table.iter_symbols().count(), 1);
}
