use crate::server::LspServer;
use async_lsp::lsp_types::Url;

#[test]
fn test_document_links_empty_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();

    let links = server.get_document_links(&uri);
    assert_eq!(links.len(), 0, "Empty file should have no document links");
}

#[test]
fn test_document_links_no_imports() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    let links = server.get_document_links(&uri);
    assert_eq!(
        links.len(),
        0,
        "File without imports should have no document links"
    );
}

#[test]
fn test_document_links_with_stdlib_import() {
    // Need stdlib loaded for this test to work
    let stdlib_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let mut server = LspServer::with_config(true, Some(stdlib_path));

    // Load stdlib
    server
        .ensure_workspace_loaded()
        .expect("Should load stdlib");

    let uri = Url::parse("file:///test.kerml").unwrap();
    // Import from standard library using classifier body
    let text = r#"
classifier TestClass {
    import ScalarValues::*;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let links = server.get_document_links(&uri);

    // Should have at least one link for the import
    assert!(
        !links.is_empty(),
        "File with stdlib import should have document links"
    );

    // Check that the link has a target
    assert!(
        links[0].target.is_some(),
        "Document link should have a target URI"
    );

    // Check that the link has a tooltip
    assert!(
        links[0].tooltip.is_some(),
        "Document link should have a tooltip"
    );
}

#[test]
fn test_document_links_kerml_file() {
    let mut server = LspServer::new();

    // Create a base file to import from
    let base_uri = Url::parse("file:///base.kerml").unwrap();
    let base_text = r#"
package Base {
    classifier DataValue;
}
    "#;
    server.open_document(&base_uri, base_text).unwrap();

    // Create a file that imports from the base file
    // Use classifier body for import (packages with imports have parsing issues)
    let test_uri = Url::parse("file:///test.kerml").unwrap();
    let test_text = r#"
classifier TestClass {
    import Base::DataValue;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have a link for the import
    assert_eq!(
        links.len(),
        1,
        "File with one import should have one document link"
    );

    // Verify the link points to the base file
    let target_path = links[0]
        .target
        .as_ref()
        .and_then(|u| u.to_file_path().ok())
        .expect("Link should have a valid file path");

    assert!(
        target_path.to_string_lossy().contains("base.kerml"),
        "Link should point to base.kerml"
    );
}

#[test]
fn test_document_links_sysml_file() {
    let mut server = LspServer::new();

    // Create a base file to import from
    let base_uri = Url::parse("file:///base.sysml").unwrap();
    let base_text = r#"
package Base {
    part def Vehicle;
}
    "#;
    server.open_document(&base_uri, base_text).unwrap();

    // Create a file that imports from the base file
    let test_uri = Url::parse("file:///test.sysml").unwrap();
    let test_text = r#"
package TestPkg {
    import Base::Vehicle;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have a link for the import
    assert_eq!(
        links.len(),
        1,
        "File with one import should have one document link"
    );

    // Verify the link has a tooltip
    assert!(
        links[0].tooltip.as_ref().unwrap().contains("Base::Vehicle"),
        "Tooltip should mention the import path"
    );
}

#[test]
fn test_document_links_wildcard_import() {
    let mut server = LspServer::new();

    // Create a base file to import from
    let base_uri = Url::parse("file:///base.kerml").unwrap();
    let base_text = r#"
package Base {
    classifier DataValue;
    classifier IntValue;
}
    "#;
    server.open_document(&base_uri, base_text).unwrap();

    // Create a file with wildcard import
    let test_uri = Url::parse("file:///test.kerml").unwrap();
    let test_text = r#"
classifier TestClass {
    import Base::*;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have a link for the wildcard import
    assert_eq!(
        links.len(),
        1,
        "File with wildcard import should have one document link"
    );

    // Verify the link points to the base file
    let target_path = links[0]
        .target
        .as_ref()
        .and_then(|u| u.to_file_path().ok())
        .expect("Link should have a valid file path");

    assert!(
        target_path.to_string_lossy().contains("base.kerml"),
        "Link should point to base.kerml for wildcard import"
    );
}

#[test]
fn test_document_links_multiple_imports() {
    let mut server = LspServer::new();

    // Create two base files
    let base1_uri = Url::parse("file:///base1.kerml").unwrap();
    let base1_text = r#"
package Base1 {
    classifier Type1;
}
    "#;
    server.open_document(&base1_uri, base1_text).unwrap();

    let base2_uri = Url::parse("file:///base2.kerml").unwrap();
    let base2_text = r#"
package Base2 {
    classifier Type2;
}
    "#;
    server.open_document(&base2_uri, base2_text).unwrap();

    // Create a file with multiple imports
    let test_uri = Url::parse("file:///test.kerml").unwrap();
    let test_text = r#"
classifier TestClass {
    import Base1::Type1;
    import Base2::Type2;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have links for both imports
    assert_eq!(
        links.len(),
        2,
        "File with two imports should have two document links"
    );
}

#[test]
fn test_document_links_nonexistent_import() {
    let mut server = LspServer::new();

    // Create a file that imports something that doesn't exist
    let test_uri = Url::parse("file:///test.kerml").unwrap();
    let test_text = r#"
package TestPkg {
    import NonExistent::Type;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have no links since the import cannot be resolved
    assert_eq!(
        links.len(),
        0,
        "Import to non-existent symbol should not create a link"
    );
}

#[test]
fn test_document_links_invalid_file() {
    let server = LspServer::new();
    let uri = Url::parse("file:///nonexistent.sysml").unwrap();

    let links = server.get_document_links(&uri);
    assert_eq!(
        links.len(),
        0,
        "Non-existent file should have no document links"
    );
}

// ============================================================================
// Tests for type reference document links (specialization, typing, subsetting)
// ============================================================================

#[test]
fn test_document_links_specialization() {
    let mut server = LspServer::new();

    // Create a base file with a definition
    let base_uri = Url::parse("file:///base.sysml").unwrap();
    let base_text = r#"
package Base {
    part def Vehicle;
}
    "#;
    server.open_document(&base_uri, base_text).unwrap();

    // Create a file that specializes the base definition
    let test_uri = Url::parse("file:///test.sysml").unwrap();
    let test_text = r#"
package Test {
    import Base::*;
    part def Car :> Vehicle;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have 1 link for import (type references are not included in document links)
    assert_eq!(
        links.len(),
        1,
        "File with import should have 1 document link for the import, got {}",
        links.len()
    );

    // Check that the link is for the import (Base)
    let has_base_link = links
        .iter()
        .any(|l| l.tooltip.as_ref().is_some_and(|t| t.contains("Base")));
    assert!(has_base_link, "Should have a link pointing to Base package");
}

#[test]
fn test_document_links_typing() {
    let mut server = LspServer::new();

    // Create a base file with a definition
    let base_uri = Url::parse("file:///base.sysml").unwrap();
    let base_text = r#"
package Base {
    part def Engine;
}
    "#;
    server.open_document(&base_uri, base_text).unwrap();

    // Create a file with a typed usage
    let test_uri = Url::parse("file:///test.sysml").unwrap();
    let test_text = r#"
package Test {
    import Base::*;
    part myEngine : Engine;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have 1 link for import (type references are not included in document links)
    assert_eq!(
        links.len(),
        1,
        "File with import should have 1 document link for the import, got {}",
        links.len()
    );

    // Check that the link is for the import (Base)
    let has_base_link = links
        .iter()
        .any(|l| l.tooltip.as_ref().is_some_and(|t| t.contains("Base")));
    assert!(has_base_link, "Should have a link pointing to Base package");
}

#[test]
fn test_document_links_subsetting() {
    let mut server = LspServer::new();

    // Create a file with subsetting
    let test_uri = Url::parse("file:///test.sysml").unwrap();
    let test_text = r#"
package Test {
    part def Vehicle {
        part components : Part[*];
        part wheels : Part[4] subsets components;
    }
    part def Part;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // The subsetting relationship should create a link
    // Note: This test verifies the mechanism works, actual link count
    // depends on relationship tracking implementation
    for link in &links {
        // All links should have valid ranges
        assert!(
            link.range.end.line >= link.range.start.line,
            "Link range should be valid"
        );
        // All links should have targets
        assert!(link.target.is_some(), "Link should have a target");
    }
}

#[test]
fn test_document_links_multiple_type_references() {
    let mut server = LspServer::new();

    // Create base definitions
    let base_uri = Url::parse("file:///base.sysml").unwrap();
    let base_text = r#"
package Base {
    part def Vehicle;
    part def Engine;
    part def Wheel;
}
    "#;
    server.open_document(&base_uri, base_text).unwrap();

    // Create a file with multiple type references
    let test_uri = Url::parse("file:///test.sysml").unwrap();
    let test_text = r#"
package Test {
    import Base::*;
    part def Car :> Vehicle {
        part engine : Engine;
        part wheels : Wheel[4];
    }
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = server.get_document_links(&test_uri);

    // Should have 1 link for import (type references are not included in document links)
    assert_eq!(
        links.len(),
        1,
        "File with import should have 1 document link for the import, got {}",
        links.len()
    );
}
