#![allow(clippy::unwrap_used)]

use super::file::SyntaxFile;
use crate::syntax::kerml::ast::{
    Element as KerMLElement, Import as KerMLImport, ImportKind, KerMLFile,
    NamespaceDeclaration as KerMLNamespace,
};
use crate::syntax::sysml::ast::{
    Element as SysMLElement, Import as SysMLImport, NamespaceDeclaration as SysMLNamespace,
    SysMLFile,
};

// ============================================================================
// Helper functions to create test data
// ============================================================================

/// Creates a minimal SysML file for testing
fn create_sysml_file(elements: Vec<SysMLElement>) -> SysMLFile {
    SysMLFile {
        namespace: None,
        namespaces: vec![],
        elements,
    }
}

/// Creates a minimal KerML file for testing
fn create_kerml_file(elements: Vec<KerMLElement>) -> KerMLFile {
    KerMLFile {
        namespace: None,
        elements,
    }
}

/// Creates a SysML import element
fn create_sysml_import(path: &str) -> SysMLElement {
    SysMLElement::Import(SysMLImport {
        path: path.to_string(),
        is_recursive: false,
        span: None,
    })
}

/// Creates a KerML import element
fn create_kerml_import(path: &str, kind: ImportKind) -> KerMLElement {
    KerMLElement::Import(KerMLImport {
        path: path.to_string(),
        is_recursive: false,
        kind,
        span: None,
    })
}

// ============================================================================
// Tests for as_sysml() - Issue #247
// ============================================================================

#[test]
fn test_as_sysml_with_sysml_variant_returns_some() {
    let sysml_file = create_sysml_file(vec![]);
    let syntax_file = SyntaxFile::SysML(sysml_file.clone());

    let result = syntax_file.as_sysml();

    assert!(
        result.is_some(),
        "as_sysml() should return Some for SysML variant"
    );
    assert_eq!(*result.unwrap(), sysml_file);
}

#[test]
fn test_as_sysml_with_kerml_variant_returns_none() {
    let kerml_file = create_kerml_file(vec![]);
    let syntax_file = SyntaxFile::KerML(kerml_file);

    let result = syntax_file.as_sysml();

    assert!(
        result.is_none(),
        "as_sysml() should return None for KerML variant"
    );
}

#[test]
fn test_as_sysml_with_sysml_containing_elements() {
    let import = create_sysml_import("Base::Package");
    let sysml_file = create_sysml_file(vec![import]);
    let syntax_file = SyntaxFile::SysML(sysml_file.clone());

    let result = syntax_file.as_sysml();

    assert!(result.is_some());
    assert_eq!(result.unwrap().elements.len(), 1);
}

#[test]
fn test_as_sysml_with_sysml_containing_namespace() {
    let mut sysml_file = create_sysml_file(vec![]);
    sysml_file.namespace = Some(SysMLNamespace {
        name: "TestNamespace".to_string(),
        span: None,
    });
    let syntax_file = SyntaxFile::SysML(sysml_file.clone());

    let result = syntax_file.as_sysml();

    assert!(result.is_some());
    assert!(result.unwrap().namespace.is_some());
    assert_eq!(
        result.unwrap().namespace.as_ref().unwrap().name,
        "TestNamespace"
    );
}

// ============================================================================
// Tests for as_kerml() - Issue #246
// ============================================================================

#[test]
fn test_as_kerml_with_kerml_variant_returns_some() {
    let kerml_file = create_kerml_file(vec![]);
    let syntax_file = SyntaxFile::KerML(kerml_file.clone());

    let result = syntax_file.as_kerml();

    assert!(
        result.is_some(),
        "as_kerml() should return Some for KerML variant"
    );
    assert_eq!(*result.unwrap(), kerml_file);
}

#[test]
fn test_as_kerml_with_sysml_variant_returns_none() {
    let sysml_file = create_sysml_file(vec![]);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let result = syntax_file.as_kerml();

    assert!(
        result.is_none(),
        "as_kerml() should return None for SysML variant"
    );
}

#[test]
fn test_as_kerml_with_kerml_containing_elements() {
    let import = create_kerml_import("Base::Package", ImportKind::Normal);
    let kerml_file = create_kerml_file(vec![import]);
    let syntax_file = SyntaxFile::KerML(kerml_file.clone());

    let result = syntax_file.as_kerml();

    assert!(result.is_some());
    assert_eq!(result.unwrap().elements.len(), 1);
}

#[test]
fn test_as_kerml_with_kerml_containing_namespace() {
    let mut kerml_file = create_kerml_file(vec![]);
    kerml_file.namespace = Some(KerMLNamespace {
        name: "TestNamespace".to_string(),
        span: None,
    });
    let syntax_file = SyntaxFile::KerML(kerml_file.clone());

    let result = syntax_file.as_kerml();

    assert!(result.is_some());
    assert!(result.unwrap().namespace.is_some());
    assert_eq!(
        result.unwrap().namespace.as_ref().unwrap().name,
        "TestNamespace"
    );
}

// ============================================================================
// Tests for extract_imports() - Issue #245
// ============================================================================

#[test]
fn test_extract_imports_from_sysml_with_single_import() {
    let import = create_sysml_import("Base::Package");
    let sysml_file = create_sysml_file(vec![import]);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 1, "Should extract 1 import");
    assert_eq!(imports[0], "Base::Package");
}

#[test]
fn test_extract_imports_from_sysml_with_multiple_imports() {
    let imports_elements = vec![
        create_sysml_import("Base::Package1"),
        create_sysml_import("Base::Package2"),
        create_sysml_import("Other::Module"),
    ];
    let sysml_file = create_sysml_file(imports_elements);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 3, "Should extract 3 imports");
    assert_eq!(imports[0], "Base::Package1");
    assert_eq!(imports[1], "Base::Package2");
    assert_eq!(imports[2], "Other::Module");
}

#[test]
fn test_extract_imports_from_empty_sysml_file() {
    let sysml_file = create_sysml_file(vec![]);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert!(imports.is_empty(), "Empty file should have no imports");
}

#[test]
fn test_extract_imports_from_kerml_with_single_import() {
    let import = create_kerml_import("Base::Package", ImportKind::Normal);
    let kerml_file = create_kerml_file(vec![import]);
    let syntax_file = SyntaxFile::KerML(kerml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 1, "Should extract 1 import");
    assert_eq!(imports[0], "Base::Package");
}

#[test]
fn test_extract_imports_from_kerml_with_multiple_imports() {
    let imports_elements = vec![
        create_kerml_import("Base::Package1", ImportKind::Normal),
        create_kerml_import("Base::Package2", ImportKind::All),
        create_kerml_import("Other::Module", ImportKind::Normal),
    ];
    let kerml_file = create_kerml_file(imports_elements);
    let syntax_file = SyntaxFile::KerML(kerml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 3, "Should extract 3 imports");
    assert_eq!(imports[0], "Base::Package1");
    assert_eq!(imports[1], "Base::Package2");
    assert_eq!(imports[2], "Other::Module");
}

#[test]
fn test_extract_imports_from_empty_kerml_file() {
    let kerml_file = create_kerml_file(vec![]);
    let syntax_file = SyntaxFile::KerML(kerml_file);

    let imports = syntax_file.extract_imports();

    assert!(imports.is_empty(), "Empty file should have no imports");
}

#[test]
fn test_extract_imports_with_wildcard_import_sysml() {
    let import = create_sysml_import("Base::*");
    let sysml_file = create_sysml_file(vec![import]);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "Base::*");
}

#[test]
fn test_extract_imports_with_wildcard_import_kerml() {
    let import = create_kerml_import("Base::*", ImportKind::All);
    let kerml_file = create_kerml_file(vec![import]);
    let syntax_file = SyntaxFile::KerML(kerml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "Base::*");
}

#[test]
fn test_extract_imports_with_recursive_import_sysml() {
    let import_element = SysMLImport {
        path: "Base::Package".to_string(),
        is_recursive: true,
        span: None,
    };
    let sysml_file = create_sysml_file(vec![SysMLElement::Import(import_element)]);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "Base::Package");
}

#[test]
fn test_extract_imports_with_recursive_import_kerml() {
    let import_element = KerMLImport {
        path: "Base::Package".to_string(),
        is_recursive: true,
        kind: ImportKind::Normal,
        span: None,
    };
    let kerml_file = create_kerml_file(vec![KerMLElement::Import(import_element)]);
    let syntax_file = SyntaxFile::KerML(kerml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0], "Base::Package");
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_extract_imports_ignores_non_import_elements_sysml() {
    use crate::syntax::sysml::ast::Comment;

    let import = create_sysml_import("Base::Package");
    let comment = SysMLElement::Comment(Comment {
        content: "This is a comment".to_string(),
        span: None,
    });
    let sysml_file = create_sysml_file(vec![import, comment]);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(
        imports.len(),
        1,
        "Should only extract imports, ignoring comments"
    );
    assert_eq!(imports[0], "Base::Package");
}

#[test]
fn test_extract_imports_ignores_non_import_elements_kerml() {
    use crate::syntax::kerml::ast::Comment;

    let import = create_kerml_import("Base::Package", ImportKind::Normal);
    let comment = KerMLElement::Comment(Comment {
        content: "This is a comment".to_string(),
        about: vec![],
        locale: None,
        span: None,
    });
    let kerml_file = create_kerml_file(vec![import, comment]);
    let syntax_file = SyntaxFile::KerML(kerml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(
        imports.len(),
        1,
        "Should only extract imports, ignoring comments"
    );
    assert_eq!(imports[0], "Base::Package");
}

#[test]
fn test_extract_imports_with_qualified_paths() {
    let imports_elements = vec![
        create_sysml_import("A::B::C"),
        create_sysml_import("X::Y::Z::W"),
    ];
    let sysml_file = create_sysml_file(imports_elements);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0], "A::B::C");
    assert_eq!(imports[1], "X::Y::Z::W");
}

#[test]
fn test_extract_imports_preserves_order() {
    let imports_elements = vec![
        create_sysml_import("First::Import"),
        create_sysml_import("Second::Import"),
        create_sysml_import("Third::Import"),
    ];
    let sysml_file = create_sysml_file(imports_elements);
    let syntax_file = SyntaxFile::SysML(sysml_file);

    let imports = syntax_file.extract_imports();

    assert_eq!(imports.len(), 3);
    assert_eq!(imports[0], "First::Import");
    assert_eq!(imports[1], "Second::Import");
    assert_eq!(imports[2], "Third::Import");
}

#[test]
fn test_both_as_methods_are_mutually_exclusive() {
    // Test SysML variant
    let sysml_file = create_sysml_file(vec![]);
    let sysml_syntax = SyntaxFile::SysML(sysml_file);

    assert!(sysml_syntax.as_sysml().is_some());
    assert!(sysml_syntax.as_kerml().is_none());

    // Test KerML variant
    let kerml_file = create_kerml_file(vec![]);
    let kerml_syntax = SyntaxFile::KerML(kerml_file);

    assert!(kerml_syntax.as_kerml().is_some());
    assert!(kerml_syntax.as_sysml().is_none());
}
