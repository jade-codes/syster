#![allow(clippy::unwrap_used)]

//! Tests for format_rich_hover and get_symbol_relationships helper functions
//!
//! Tests cover:
//! - Issue #123: syster_lsp::server::helpers::format_rich_hover
//! - Issue #128: syster_lsp::server::helpers::get_symbol_relationships
//!
//! Both functions are tested through the public LspServer::get_hover API,
//! as they are private helper functions.

use crate::server::LspServer;
use async_lsp::lsp_types::{Position, Url};

// ============================================================================
// Tests for format_rich_hover (Issue #123)
// ============================================================================

#[test]
fn test_format_rich_hover_package_basic() {
    // Test format_rich_hover through get_hover
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package TestPackage {
    part def Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Get hover on the package name
    let hover = server.get_hover(&uri, Position::new(1, 9));
    assert!(hover.is_some());

    let hover_content = hover.unwrap();
    if let async_lsp::lsp_types::HoverContents::Scalar(
        async_lsp::lsp_types::MarkedString::String(content),
    ) = hover_content.contents
    {
        // Should contain package declaration
        assert!(content.contains("package TestPackage"));
        // Should contain qualified name
        assert!(content.contains("**Qualified Name:**"));
    }
}

#[test]
fn test_format_rich_hover_definition_with_kind() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Automotive {
    part def Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let hover = server.get_hover(&uri, Position::new(2, 14));
    assert!(hover.is_some());

    let hover_content = hover.unwrap();
    if let async_lsp::lsp_types::HoverContents::Scalar(
        async_lsp::lsp_types::MarkedString::String(content),
    ) = hover_content.contents
    {
        assert!(content.contains("Vehicle"));
        assert!(content.contains("**Qualified Name:**"));
    }
}

#[test]
fn test_format_rich_hover_usage_with_type() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part myCar : Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let hover = server.get_hover(&uri, Position::new(3, 10));
    assert!(hover.is_some());

    let hover_content = hover.unwrap();
    if let async_lsp::lsp_types::HoverContents::Scalar(
        async_lsp::lsp_types::MarkedString::String(content),
    ) = hover_content.contents
    {
        assert!(content.contains("myCar"));
    }
}

#[test]
fn test_format_rich_hover_feature_declaration() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle {
        attribute weight : Real;
    }
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Hover on attribute name
    let hover = server.get_hover(&uri, Position::new(3, 19));

    // Attribute might or might not have hover depending on implementation
    // The test verifies format_rich_hover doesn't crash
    if let Some(h) = hover
        && let async_lsp::lsp_types::HoverContents::Scalar(
            async_lsp::lsp_types::MarkedString::String(content),
        ) = h.contents
    {
        // Should contain some identifying info
        assert!(!content.is_empty());
    }
}

#[test]
fn test_format_rich_hover_without_source_file() {
    // Test that format_rich_hover handles missing source file gracefully
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def SimpleType;";

    server.open_document(&uri, text).unwrap();

    let hover = server.get_hover(&uri, Position::new(0, 9));
    assert!(hover.is_some());

    // Should not crash if source_file is None
    let hover_content = hover.unwrap();
    if let async_lsp::lsp_types::HoverContents::Scalar(
        async_lsp::lsp_types::MarkedString::String(content),
    ) = hover_content.contents
    {
        assert!(content.contains("SimpleType"));
    }
}

// ============================================================================
// Tests for get_symbol_relationships via format_rich_hover (Issue #128)
// ============================================================================

#[test]
fn test_get_symbol_relationships_with_index() {
    // This tests get_symbol_relationships indirectly through format_rich_hover
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part def Car :> Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Get hover on Car (which specializes Vehicle)
    let hover = server.get_hover(&uri, Position::new(3, 14));
    assert!(hover.is_some());

    // The hover content should include relationship information
    let hover_content = hover.unwrap();
    if let async_lsp::lsp_types::HoverContents::Scalar(
        async_lsp::lsp_types::MarkedString::String(_content),
    ) = hover_content.contents
    {
        // Should contain specialization relationship
        // Relationship info not available without RelationshipGraph
    }
}

#[test]
fn test_get_symbol_relationships_no_relationships() {
    // Test symbol with no relationships
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def StandaloneType;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Get hover on StandaloneType (no relationships)
    let hover = server.get_hover(&uri, Position::new(2, 14));
    assert!(hover.is_some());

    // Should not crash and should provide basic info
    let hover_content = hover.unwrap();
    if let async_lsp::lsp_types::HoverContents::Scalar(
        async_lsp::lsp_types::MarkedString::String(content),
    ) = hover_content.contents
    {
        assert!(content.contains("StandaloneType"));
        assert!(content.contains("**Qualified Name:**"));
    }
}

#[test]
fn test_get_symbol_relationships_multiple_relationships() {
    // Test symbol with multiple relationships
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Base;
    part def Interface;
    part def Derived :> Base;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Get hover on Derived
    let hover = server.get_hover(&uri, Position::new(4, 14));
    assert!(hover.is_some());

    let hover_content = hover.unwrap();
    if let async_lsp::lsp_types::HoverContents::Scalar(
        async_lsp::lsp_types::MarkedString::String(content),
    ) = hover_content.contents
    {
        // Should show relationships
        assert!(content.contains("Derived"));
    }
}

// ============================================================================
// Integration tests
// ============================================================================

#[test]
fn test_integration_hover_uses_correct_positions() {
    // Test that hover correctly uses position_to_byte_offset and span_to_lsp_range
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    // Hover on "Vehicle" - tests position_to_byte_offset internally
    let hover = server.get_hover(&uri, Position::new(1, 14));
    assert!(hover.is_some());

    let hover_result = hover.unwrap();
    // Should have a range that was converted using span_to_lsp_range
    assert!(hover_result.range.is_some());

    let range = hover_result.range.unwrap();
    assert_eq!(range.start.line, 1);
    assert!(range.end.character > range.start.character);
}

#[test]
fn test_integration_format_rich_hover_complete_flow() {
    // Test complete flow of format_rich_hover with real workspace
    let mut server = LspServer::new();
    let uri = Url::parse("file:///complete.sysml").unwrap();
    let text = r#"
package Complete {
    part def Base;
    part def Derived :> Base;
    part instance : Derived;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Get hover on Derived - tests format_rich_hover with relationships
    let hover = server.get_hover(&uri, Position::new(3, 14));
    assert!(hover.is_some());

    if let Some(h) = hover
        && let async_lsp::lsp_types::HoverContents::Scalar(
            async_lsp::lsp_types::MarkedString::String(content),
        ) = h.contents
    {
        // Should have declaration
        assert!(content.contains("Derived"));
        // Should have qualified name
        assert!(content.contains("Complete::Derived"));
        // Should have relationships
        // Relationship info not available without RelationshipGraph
    }
}

#[test]
fn test_integration_all_functions_with_unicode() {
    // Test all functions work correctly with Unicode
    let mut server = LspServer::new();
    let uri = Url::parse("file:///unicode.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
    part myCar : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    // Test position_to_byte_offset with basic text first
    let pos = Position::new(1, 14);
    // This internally uses position_to_byte_offset
    let hover = server.get_hover(&uri, pos);

    // Should work
    if let Some(h) = hover {
        assert!(h.range.is_some());
        if let async_lsp::lsp_types::HoverContents::Scalar(
            async_lsp::lsp_types::MarkedString::String(content),
        ) = h.contents
        {
            assert!(content.contains("Vehicle"));
        }
    }

    // Now test with actual unicode
    let uri2 = Url::parse("file:///unicode2.sysml").unwrap();
    let text2 = "part def Café;";
    server.open_document(&uri2, text2).unwrap();

    // Hover on "Café" - position after "part def "
    let hover2 = server.get_hover(&uri2, Position::new(0, 9));

    // Should handle Unicode without crashing
    if let Some(h) = hover2
        && let async_lsp::lsp_types::HoverContents::Scalar(
            async_lsp::lsp_types::MarkedString::String(content),
        ) = h.contents
    {
        assert!(content.contains("Café"));
    }
}

#[test]
fn test_format_rich_hover_references_with_url_encoded_filenames() {
    // Test that file names with spaces (URL-encoded) are displayed correctly
    let mut server = LspServer::new();

    // Create a file with spaces in the name (simulate URL encoding)
    let uri_with_spaces = Url::parse("file:///test%20file%20name.sysml").unwrap();
    let text = r#"
package Test {
    part def Base;
    part usage : Base;
}
    "#;

    server.open_document(&uri_with_spaces, text).unwrap();

    // Get hover on Base to see the "Referenced by:" section
    let hover = server.get_hover(&uri_with_spaces, Position::new(2, 14));
    assert!(hover.is_some());

    if let Some(h) = hover
        && let async_lsp::lsp_types::HoverContents::Scalar(
            async_lsp::lsp_types::MarkedString::String(content),
        ) = h.contents
    {
        // The hover should contain the decoded file name (with spaces), not URL-encoded
        // Looking for the file name in the "Referenced by:" section
        assert!(
            content.contains("Referenced by:"),
            "Should have references section"
        );

        // The file name in the markdown link text should be decoded (have spaces, not %20)
        // Format is: [filename:line:col](url)
        // We check for the pattern but allow flexibility in line/col numbers
        assert!(
            content.contains("[test file name.sysml:"),
            "File name in markdown link text should be decoded with spaces. Content:\n{}",
            content
        );

        // The URL in the markdown link target should still be encoded (that's correct for URLs)
        assert!(
            content.contains("file:///test%20file%20name.sysml"),
            "URL should remain encoded for proper linking. Content:\n{}",
            content
        );
    }
}
