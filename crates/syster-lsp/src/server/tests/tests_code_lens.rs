//! Tests for code lens functionality

use crate::server::LspServer;
use async_lsp::lsp_types::Url;

#[test]
fn test_code_lens_basic() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
    "#;

    server.open_document(&uri, text).unwrap();

    let lenses = server.get_code_lenses(&uri);

    // Should have one code lens for Vehicle showing 1 reference
    assert_eq!(lenses.len(), 1);
    assert!(lenses[0].command.is_some());
    let cmd = lenses[0].command.as_ref().unwrap();
    assert_eq!(cmd.title, "1 reference");
    assert_eq!(cmd.command, "syster.showReferences");
}

#[test]
fn test_code_lens_multiple_references() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
part truck : Vehicle;
part bike : Vehicle;
    "#;

    server.open_document(&uri, text).unwrap();

    let lenses = server.get_code_lenses(&uri);

    // Should have one code lens for Vehicle showing 3 references
    assert_eq!(lenses.len(), 1);
    let cmd = lenses[0].command.as_ref().unwrap();
    assert_eq!(cmd.title, "3 references");
}

#[test]
fn test_code_lens_no_references() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part def Bike;
    "#;

    server.open_document(&uri, text).unwrap();

    let lenses = server.get_code_lenses(&uri);

    // Should have no code lenses since there are no references
    assert_eq!(lenses.len(), 0);
}

#[test]
fn test_code_lens_classifier() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.kerml").unwrap();
    let text = r#"
classifier Vehicle;
classifier Car specializes Vehicle;
    "#;

    server.open_document(&uri, text).unwrap();

    let lenses = server.get_code_lenses(&uri);

    // Should have one code lens for Vehicle showing the specialization reference
    assert_eq!(lenses.len(), 1);
    let cmd = lenses[0].command.as_ref().unwrap();
    assert_eq!(cmd.title, "1 reference");
}

#[test]
fn test_code_lens_features_with_references() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle {
    attribute speed : Real;
}
part def Car specializes Vehicle {
    attribute speed redefines Vehicle::speed;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let lenses = server.get_code_lenses(&uri);

    // Should have code lenses for Vehicle (1 specialization) and speed (1 redefinition)
    let lens_titles: Vec<&str> = lenses
        .iter()
        .filter_map(|l| l.command.as_ref())
        .map(|c| c.title.as_str())
        .collect();

    assert!(
        lens_titles.contains(&"1 reference"),
        "Vehicle should have 1 reference"
    );
}

#[test]
fn test_code_lens_invalid_uri() {
    let server = LspServer::new();
    let uri = Url::parse("http://example.com/not-a-file").unwrap();

    let lenses = server.get_code_lenses(&uri);

    // Should return empty vec for invalid file URI
    assert_eq!(lenses.len(), 0);
}

#[test]
fn test_code_lens_usages_with_references() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part myCar : Vehicle;
part anotherRef : myCar;
    "#;

    server.open_document(&uri, text).unwrap();

    let lenses = server.get_code_lenses(&uri);

    // Should have code lenses for both Vehicle (typed by myCar) and myCar (typed by anotherRef)
    assert!(!lenses.is_empty(), "Should have at least one code lens");
}
