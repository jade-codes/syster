use async_lsp::lsp_types::Url;

use crate::server::LspServer;
use crate::server::type_references::collect_document_links;

#[test]
fn test_collect_specialization_links() {
    let mut server = LspServer::new();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part def Car :> Vehicle;
}
    "#;
    server.open_document(&uri, text).unwrap();

    let links = collect_document_links(server.workspace(), "/test.sysml");

    // Should have one link for the specialization (Car :> Vehicle)
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].tooltip, Some("Go to Vehicle".to_string()));
}

#[test]
fn test_collect_typing_links() {
    let mut server = LspServer::new();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Engine;
    part def Car {
        part engine : Engine;
    }
}
    "#;
    server.open_document(&uri, text).unwrap();

    let links = collect_document_links(server.workspace(), "/test.sysml");

    // Should have one link for the typing (engine : Engine)
    assert_eq!(links.len(), 1);
    // Target name may be qualified (Test::Engine) or simple (Engine)
    assert!(links[0].tooltip.as_ref().unwrap().contains("Engine"));
}

#[test]
fn test_collect_multiple_relationship_types() {
    let mut server = LspServer::new();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part def Engine;
    part def Car :> Vehicle {
        part engine : Engine;
    }
}
    "#;
    server.open_document(&uri, text).unwrap();

    let links = collect_document_links(server.workspace(), "/test.sysml");

    // Should have two links: specialization + typing
    assert_eq!(links.len(), 2);

    let tooltips: Vec<_> = links.iter().filter_map(|l| l.tooltip.as_ref()).collect();
    // Target names may be qualified or simple
    assert!(tooltips.iter().any(|t| t.contains("Vehicle")));
    assert!(tooltips.iter().any(|t| t.contains("Engine")));
}

#[test]
fn test_collect_cross_file_links() {
    let mut server = LspServer::new();

    // Base file with definitions
    let base_uri = Url::parse("file:///base.sysml").unwrap();
    let base_text = r#"
package Base {
    part def Vehicle;
}
    "#;
    server.open_document(&base_uri, base_text).unwrap();

    // Test file that imports and uses
    let test_uri = Url::parse("file:///test.sysml").unwrap();
    let test_text = r#"
package Test {
    import Base::*;
    part def Car :> Vehicle;
}
    "#;
    server.open_document(&test_uri, test_text).unwrap();

    let links = collect_document_links(server.workspace(), "/test.sysml");

    // Should have one link pointing to the base file
    assert_eq!(links.len(), 1);
    assert!(
        links[0]
            .target
            .as_ref()
            .unwrap()
            .path()
            .ends_with("base.sysml")
    );
}

#[test]
fn test_collect_no_links_for_empty_file() {
    let mut server = LspServer::new();

    let uri = Url::parse("file:///empty.sysml").unwrap();
    let text = "package Empty { }";
    server.open_document(&uri, text).unwrap();

    let links = collect_document_links(server.workspace(), "/empty.sysml");

    assert!(links.is_empty());
}
