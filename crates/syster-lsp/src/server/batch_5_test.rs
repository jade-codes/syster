//! Comprehensive tests for batch-5 module
//!
//! This file contains tests for:
//! - Issue #131: syster::semantic::adapters::sysml::inlay_hints::collect_usage_hints
//! - Issue #128: syster_lsp::server::helpers::get_symbol_relationships  
//! - Issue #124: syster_lsp::server::helpers::span_to_lsp_range
//! - Issue #123: syster_lsp::server::helpers::format_rich_hover
//! - Issue #121: syster_lsp::server::helpers::position_to_byte_offset

use crate::server::LspServer;
use crate::server::helpers::{position_to_byte_offset, span_to_lsp_range};
use async_lsp::lsp_types::{Position, Url};
use syster::core::{Position as CorePosition, Span};

// ============================================================================
// Tests for position_to_byte_offset (Issue #121)
// ============================================================================

#[test]
fn test_position_to_byte_offset_start_of_document() {
    let text = "part def Vehicle;";
    let pos = Position::new(0, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 0);
}

#[test]
fn test_position_to_byte_offset_middle_of_line() {
    let text = "part def Vehicle;";
    let pos = Position::new(0, 5);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 5);
}

#[test]
fn test_position_to_byte_offset_end_of_line() {
    let text = "part def Vehicle;";
    let pos = Position::new(0, 17);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 17);
}

#[test]
fn test_position_to_byte_offset_second_line_start() {
    let text = "part def Car;\npart def Bike;";
    let pos = Position::new(1, 0);
    // "part def Car;\n" = 14 bytes
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 14);
}

#[test]
fn test_position_to_byte_offset_second_line_middle() {
    let text = "part def Car;\npart def Bike;";
    let pos = Position::new(1, 5);
    // "part def Car;\n" = 14 bytes + 5 = 19
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 19);
}

#[test]
fn test_position_to_byte_offset_multiline_complex() {
    let text = "line1\nline2\nline3";
    // Start of line 2 (after "line1\n" = 6 bytes)
    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 12);
}

#[test]
fn test_position_to_byte_offset_with_unicode() {
    let text = "café\nwörld";
    // "café" = 5 bytes (c=1, a=1, f=1, é=2)
    let pos = Position::new(0, 4);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 5);

    // Start of line 1 (after "café\n" = 6 bytes)
    let pos = Position::new(1, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 6);
}

#[test]
fn test_position_to_byte_offset_with_emoji() {
    let text = "Hi 😀\nWorld";
    // "Hi " = 3 bytes, "😀" = 4 bytes
    let pos = Position::new(0, 4);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 7);
}

#[test]
fn test_position_to_byte_offset_empty_document() {
    let text = "";
    let pos = Position::new(0, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 0);
}

#[test]
fn test_position_to_byte_offset_at_document_end() {
    let text = "line1\nline2";
    // Position at line count (end of document)
    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), text.len());
}

#[test]
fn test_position_to_byte_offset_beyond_document() {
    let text = "line1\nline2";
    // Line 3 doesn't exist (only lines 0 and 1)
    let pos = Position::new(3, 0);
    assert!(position_to_byte_offset(text, pos).is_err());
}

#[test]
fn test_position_to_byte_offset_beyond_line_length() {
    let text = "short";
    // Character 100 is beyond line length - should clamp
    let pos = Position::new(0, 100);
    // Should return length of the line
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 5);
}

#[test]
fn test_position_to_byte_offset_newline_handling() {
    let text = "a\nb\nc";
    // Each line is 1 char + newline
    // Line 0: "a\n" starts at 0
    // Line 1: "b\n" starts at 2
    // Line 2: "c" starts at 4
    let pos = Position::new(1, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 2);

    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 4);
}

// ============================================================================
// Tests for span_to_lsp_range (Issue #124)
// ============================================================================

#[test]
fn test_span_to_lsp_range_single_line() {
    let span = Span {
        start: CorePosition {
            line: 5,
            column: 10,
        },
        end: CorePosition {
            line: 5,
            column: 20,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 5);
    assert_eq!(range.start.character, 10);
    assert_eq!(range.end.line, 5);
    assert_eq!(range.end.character, 20);
}

#[test]
fn test_span_to_lsp_range_multi_line() {
    let span = Span {
        start: CorePosition { line: 1, column: 5 },
        end: CorePosition {
            line: 3,
            column: 10,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 1);
    assert_eq!(range.start.character, 5);
    assert_eq!(range.end.line, 3);
    assert_eq!(range.end.character, 10);
}

#[test]
fn test_span_to_lsp_range_zero_position() {
    let span = Span {
        start: CorePosition { line: 0, column: 0 },
        end: CorePosition { line: 0, column: 5 },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 0);
    assert_eq!(range.start.character, 0);
    assert_eq!(range.end.line, 0);
    assert_eq!(range.end.character, 5);
}

#[test]
fn test_span_to_lsp_range_large_numbers() {
    let span = Span {
        start: CorePosition {
            line: 1000,
            column: 500,
        },
        end: CorePosition {
            line: 2000,
            column: 1000,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 1000);
    assert_eq!(range.start.character, 500);
    assert_eq!(range.end.line, 2000);
    assert_eq!(range.end.character, 1000);
}

#[test]
fn test_span_to_lsp_range_single_character() {
    let span = Span {
        start: CorePosition {
            line: 10,
            column: 15,
        },
        end: CorePosition {
            line: 10,
            column: 16,
        },
    };

    let range = span_to_lsp_range(&span);
    assert_eq!(range.start.line, 10);
    assert_eq!(range.start.character, 15);
    assert_eq!(range.end.line, 10);
    assert_eq!(range.end.character, 16);
}

// ============================================================================
// Tests for format_rich_hover (Issue #123)
// These tests use format_rich_hover indirectly via get_hover on LspServer
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
fn test_format_rich_hover_with_relationships() {
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
        async_lsp::lsp_types::MarkedString::String(content),
    ) = hover_content.contents
    {
        // Should contain specialization relationship
        assert!(content.contains("Specializes") || content.contains("**Relationships:**"));
    }
}

#[test]
fn test_format_rich_hover_no_relationships() {
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
fn test_format_rich_hover_multiple_relationships() {
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
// Tests for collect_usage_hints via extract_inlay_hints (Issue #131)
// ============================================================================

#[test]
fn test_inlay_hints_usage_with_explicit_type() {
    // Test that collect_usage_hints doesn't add hints when type is explicit
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part myCar : Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Create InlayHintParams with proper structure
    let params = async_lsp::lsp_types::InlayHintParams {
        text_document: async_lsp::lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        range: async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        },
        work_done_progress_params: async_lsp::lsp_types::WorkDoneProgressParams::default(),
    };

    // Get inlay hints for the file
    let hints = server.get_inlay_hints(&params);

    // Since myCar has explicit type ": Vehicle", no hint should be added
    // (the function only adds hints for usages WITHOUT explicit types)
    let hints_for_mycar: Vec<_> = hints.iter().filter(|h| h.position.line == 3).collect();

    // Should not add redundant type hint
    assert!(
        hints_for_mycar.is_empty()
            || hints_for_mycar.iter().all(|h| {
                if let async_lsp::lsp_types::InlayHintLabel::String(s) = &h.label {
                    !s.contains("Vehicle")
                } else {
                    true
                }
            })
    );
}

#[test]
fn test_inlay_hints_usage_without_explicit_type() {
    // Test that collect_usage_hints adds hints when type can be inferred
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part myCar;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let params = async_lsp::lsp_types::InlayHintParams {
        text_document: async_lsp::lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        range: async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        },
        work_done_progress_params: async_lsp::lsp_types::WorkDoneProgressParams::default(),
    };

    // Get inlay hints
    let hints = server.get_inlay_hints(&params);

    // The function should try to infer type from symbol table
    // Note: In this case, myCar has no typing relationship, so it may not have a hint
    // This tests the logic path through collect_usage_hints

    // Verify the function doesn't crash and returns a valid result
    let _ = hints; // Use the result to ensure it was computed
}

#[test]
fn test_inlay_hints_nested_usage() {
    // Test that collect_usage_hints recurses into nested usages
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Engine;
    part def Vehicle {
        part engine : Engine;
    }
}
    "#;

    server.open_document(&uri, text).unwrap();

    let params = async_lsp::lsp_types::InlayHintParams {
        text_document: async_lsp::lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        range: async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        },
        work_done_progress_params: async_lsp::lsp_types::WorkDoneProgressParams::default(),
    };

    // Get inlay hints - should process nested usage (engine)
    let hints = server.get_inlay_hints(&params);

    // Verify it doesn't crash on nested structure
    let _ = hints; // Use the result to ensure it was computed
}

#[test]
fn test_inlay_hints_with_range_filter() {
    // Test that collect_usage_hints respects range filtering
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part car1;
    part car2;
    part car3;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Get hints for a specific range (e.g., just line 3)
    let params = async_lsp::lsp_types::InlayHintParams {
        text_document: async_lsp::lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        range: async_lsp::lsp_types::Range {
            start: Position::new(3, 0),
            end: Position::new(3, 100),
        },
        work_done_progress_params: async_lsp::lsp_types::WorkDoneProgressParams::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Should only return hints within the specified range
    // Verify it processes the range parameter correctly
    for hint in hints {
        assert_eq!(hint.position.line, 3);
    }
}

#[test]
fn test_inlay_hints_empty_file() {
    // Test collect_usage_hints with empty file
    let mut server = LspServer::new();
    let uri = Url::parse("file:///empty.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();

    let params = async_lsp::lsp_types::InlayHintParams {
        text_document: async_lsp::lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        range: async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        },
        work_done_progress_params: async_lsp::lsp_types::WorkDoneProgressParams::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Should return empty hints without crashing
    assert!(hints.is_empty());
}

#[test]
fn test_inlay_hints_usage_without_name() {
    // Test edge case: usage without a name
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle {
        part : Engine;
    }
}
    "#;

    server.open_document(&uri, text).unwrap();

    let params = async_lsp::lsp_types::InlayHintParams {
        text_document: async_lsp::lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        range: async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        },
        work_done_progress_params: async_lsp::lsp_types::WorkDoneProgressParams::default(),
    };

    // Should handle anonymous usages gracefully
    let hints = server.get_inlay_hints(&params);

    // Should not crash - verify function completes
    let _ = hints;
}

#[test]
fn test_inlay_hints_deeply_nested() {
    // Test deeply nested structure
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Level1 {
        part level2 {
            part level3 {
                part level4;
            }
        }
    }
}
    "#;

    server.open_document(&uri, text).unwrap();

    let params = async_lsp::lsp_types::InlayHintParams {
        text_document: async_lsp::lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        range: async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(20, 0),
        },
        work_done_progress_params: async_lsp::lsp_types::WorkDoneProgressParams::default(),
    };

    // Should recursively process all levels
    let hints = server.get_inlay_hints(&params);

    // Verify it handles deep nesting without crashing
    let _ = hints;
}

// ============================================================================
// Integration tests combining multiple functions
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
        assert!(content.contains("Specializes") || content.contains("Base"));
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
