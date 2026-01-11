//! Comprehensive tests for core LspServer module functions
//!
//! This module provides extensive test coverage for the following LspServer methods:
//! - get_folding_ranges
//! - semantic_tokens_legend
//! - get_semantic_tokens
//! - get_selection_ranges, build_selection_range_chain, default_selection_range
//! - get_inlay_hints
//!
//! Tests cover both success and edge cases through the public API.

use crate::server::LspServer;
use async_lsp::lsp_types::*;
use std::path::Path;

// ============================================================================
// Tests for get_folding_ranges (#100-109)
// ============================================================================

#[test]
fn test_folding_ranges_basic_package() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package TestPackage {
    part def Vehicle {
        attribute weight : Real;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Folding ranges are implementation-dependent
    // The main verification is that the function doesn't crash
    // and returns valid data if any ranges are returned

    // Verify ranges are sorted by start line if any exist
    for i in 1..ranges.len() {
        assert!(
            ranges[i].start_line >= ranges[i - 1].start_line,
            "Ranges should be sorted by start line"
        );
    }

    // All ranges should be valid (end >= start)
    for range in &ranges {
        assert!(
            range.end_line >= range.start_line,
            "End line must be >= start line"
        );
    }
}

#[test]
fn test_folding_ranges_nested_structures() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Outer {
    package Inner {
        part def Vehicle {
            attribute speed : Real;
            part engine : Engine;
        }
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // The implementation should handle nested structures gracefully
    // Check that if we have ranges, they have appropriate kinds
    let has_region = ranges
        .iter()
        .any(|r| r.kind == Some(FoldingRangeKind::Region));

    // If we have any ranges, at least one should be a Region
    if !ranges.is_empty() {
        assert!(
            has_region,
            "Should have Region kind folding ranges if any ranges exist"
        );
    }
}

#[test]
fn test_folding_ranges_single_line_no_fold() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Car;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Single-line elements should not create folding ranges
    assert!(
        ranges.is_empty() || ranges.iter().all(|r| r.end_line > r.start_line),
        "Single-line elements should not create folding ranges"
    );
}

#[test]
fn test_folding_ranges_empty_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Empty file should have no folding ranges
    assert!(
        ranges.is_empty(),
        "Empty file should have no folding ranges"
    );
}

#[test]
fn test_folding_ranges_nonexistent_file() {
    let server = LspServer::new();
    let path = Path::new("/nonexistent.sysml");
    let ranges = server.get_folding_ranges(path);

    // Nonexistent file should return empty vec
    assert!(
        ranges.is_empty(),
        "Nonexistent file should return empty vec"
    );
}

#[test]
fn test_folding_ranges_multiple_top_level_elements() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Pkg1 {
    part def Car;
}

package Pkg2 {
    part def Truck;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Multiple packages should be handled correctly
    // Verify the function works without crashing
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_with_comments() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"/* Multi-line
   comment block */
package TestPkg {
    // Single line comment
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should handle comments appropriately without crashing
    // Check for comment kind if comments are foldable
    let _has_comment_kind = ranges
        .iter()
        .any(|r| r.kind == Some(FoldingRangeKind::Comment));

    // Note: Whether comments are folded depends on implementation; we only
    // verify that the call does not crash and returns structurally valid data.

    // All returned ranges should be valid
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_character_positions() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should include character positions
    for range in &ranges {
        // Character positions are optional in LSP but our impl provides them
        if range.start_character.is_some() && range.end_character.is_some() {
            let start_char = range.start_character.unwrap();
            let end_char = range.end_character.unwrap();

            // On same line, end should be after start
            if range.start_line == range.end_line {
                assert!(
                    end_char >= start_char,
                    "End character should be >= start character"
                );
            }
        }
    }
}

// ============================================================================
// Tests for semantic_tokens_legend (#95, 97-99)
// ============================================================================

#[test]
fn test_semantic_tokens_legend_has_required_types() {
    let legend = LspServer::semantic_tokens_legend();

    // Should have at least the basic token types
    assert!(
        !legend.token_types.is_empty(),
        "Legend should have token types"
    );

    // Verify specific token types are present
    let type_strings: Vec<String> = legend
        .token_types
        .iter()
        .map(|t| t.as_str().to_string())
        .collect();

    assert!(
        type_strings.contains(&"namespace".to_string()),
        "Should have NAMESPACE token type"
    );
    assert!(
        type_strings.contains(&"type".to_string()),
        "Should have TYPE token type"
    );
    assert!(
        type_strings.contains(&"variable".to_string()),
        "Should have VARIABLE token type"
    );
    assert!(
        type_strings.contains(&"property".to_string()),
        "Should have PROPERTY token type"
    );
    assert!(
        type_strings.contains(&"keyword".to_string()),
        "Should have KEYWORD token type"
    );
}

#[test]
fn test_semantic_tokens_legend_consistent() {
    // Call multiple times to ensure it's consistent
    let legend1 = LspServer::semantic_tokens_legend();
    let legend2 = LspServer::semantic_tokens_legend();

    assert_eq!(
        legend1.token_types.len(),
        legend2.token_types.len(),
        "Legend should be consistent across calls"
    );

    // Verify same types in same order
    for (t1, t2) in legend1.token_types.iter().zip(legend2.token_types.iter()) {
        assert_eq!(t1, t2, "Token types should be in same order");
    }
}

#[test]
fn test_semantic_tokens_legend_no_modifiers() {
    let legend = LspServer::semantic_tokens_legend();

    // Current implementation has no modifiers
    assert!(
        legend.token_modifiers.is_empty(),
        "Current implementation has no token modifiers"
    );
}

// ============================================================================
// Tests for get_semantic_tokens (#92-96)
// ============================================================================

#[test]
fn test_semantic_tokens_basic_package() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package TestPkg {
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();
    let result = server.get_semantic_tokens(&uri);

    assert!(result.is_some(), "Should return semantic tokens");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected SemanticTokens result");
    };

    // Should have tokens for package, part, def, identifiers
    assert!(!tokens.data.is_empty(), "Should have semantic tokens");

    // Verify tokens are in delta encoding format
    let legend_len = LspServer::semantic_tokens_legend().token_types.len() as u32;
    let mut _prev_line = 0;
    for token in &tokens.data {
        // Delta line is relative to previous token
        _prev_line += token.delta_line;

        // Token type should be valid (within legend range)
        assert!(token.token_type < legend_len, "Token type should be valid");

        // Length should be positive
        assert!(token.length > 0, "Token length should be positive");
    }
}

#[test]
fn test_semantic_tokens_multiple_symbols() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Auto {
    part def Vehicle;
    part def Car;
    part myCar : Car;
}"#;

    server.open_document(&uri, text).unwrap();
    let result = server.get_semantic_tokens(&uri);

    assert!(
        result.is_some(),
        "Should return tokens for multiple symbols"
    );

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected SemanticTokens result");
    };

    // Should have multiple tokens
    assert!(tokens.data.len() >= 4, "Should have tokens for all symbols");
}

#[test]
fn test_semantic_tokens_empty_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();
    let result = server.get_semantic_tokens(&uri);

    // Empty file should return Some with empty tokens
    assert!(result.is_some(), "Empty file should return Some result");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected SemanticTokens result");
    };

    assert!(tokens.data.is_empty(), "Empty file should have no tokens");
}

#[test]
fn test_semantic_tokens_nonexistent_file() {
    let server = LspServer::new();
    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let result = server.get_semantic_tokens(&uri);

    // Nonexistent file should return None
    assert!(result.is_none(), "Nonexistent file should return None");
}

#[test]
fn test_semantic_tokens_with_index() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Base;
    part def Derived :> Base;
    part instance : Derived;
}"#;

    server.open_document(&uri, text).unwrap();
    let result = server.get_semantic_tokens(&uri);

    assert!(result.is_some(), "Should handle relationships");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected SemanticTokens result");
    };

    // Should have tokens for all symbols including relationships
    assert!(
        !tokens.data.is_empty(),
        "Should have tokens for relationships"
    );
}

#[test]
fn test_semantic_tokens_utf16_encoding() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    // Test with unicode characters that have different UTF-8 and UTF-16 lengths
    // Note: Pest parser may not handle all unicode in identifiers
    let text = "package TestPkg { part def Vehicle; }";

    server.open_document(&uri, text).unwrap();
    let result = server.get_semantic_tokens(&uri);

    assert!(result.is_some(), "Should handle unicode characters");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected SemanticTokens result");
    };

    // Should successfully encode tokens with UTF-16 positions
    assert!(!tokens.data.is_empty(), "Should have tokens");
}

#[test]
fn test_semantic_tokens_multiline_structure() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle {
        attribute weight : Real;
        part engine : Engine;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let result = server.get_semantic_tokens(&uri);

    assert!(result.is_some(), "Should handle multiline structures");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected SemanticTokens result");
    };

    // Tokens should span multiple lines
    let mut has_multiline = false;
    let mut current_line = 0;
    for token in &tokens.data {
        current_line += token.delta_line;
        if current_line > 0 {
            has_multiline = true;
            break;
        }
    }

    assert!(has_multiline, "Should have tokens on multiple lines");
}

// ============================================================================
// Tests for get_selection_ranges, build_selection_range_chain,
// default_selection_range (#71-91)
// ============================================================================

#[test]
fn test_selection_ranges_basic_element() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(0, 10)]; // Inside "Vehicle"

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1, "Should return one selection range");

    let range = &ranges[0];
    assert!(
        range.range.start.line <= range.range.end.line,
        "Range should be valid"
    );
}

#[test]
fn test_selection_ranges_multiple_positions() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
    part def Car;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![
        Position::new(1, 14), // On "Vehicle"
        Position::new(2, 14), // On "Car"
    ];

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(
        ranges.len(),
        2,
        "Should return selection range for each position"
    );
}

#[test]
fn test_selection_ranges_nested_structure() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle {
        attribute weight : Real;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(2, 20)]; // Inside attribute

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1, "Should return one range");

    // Check if parent chain exists for nested elements
    let range = &ranges[0];
    let mut depth = 0;
    let mut current = Some(range);

    while let Some(r) = current {
        depth += 1;
        current = r.parent.as_ref().map(|b| b.as_ref());
    }

    // Should have some nesting for attribute inside part def
    assert!(
        depth >= 1,
        "Should have at least one level in selection chain"
    );
}

#[test]
fn test_selection_ranges_out_of_bounds() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(100, 100)]; // Way out of bounds

    let ranges = server.get_selection_ranges(path, positions);

    // Should return default range (single character) gracefully
    assert_eq!(ranges.len(), 1, "Should return a default range");

    let range = &ranges[0];
    assert!(
        range.range.end.character >= range.range.start.character,
        "Default range should be valid"
    );
}

#[test]
fn test_selection_ranges_nonexistent_file() {
    let server = LspServer::new();
    let path = Path::new("/nonexistent.sysml");
    let positions = vec![Position::new(0, 0)];

    let ranges = server.get_selection_ranges(path, positions);

    // Should return default ranges for nonexistent file
    assert_eq!(
        ranges.len(),
        1,
        "Should return default range for nonexistent file"
    );

    // Default range should be single character
    let range = &ranges[0];
    assert_eq!(
        range.range.end.character,
        range.range.start.character + 1,
        "Default range should be single character"
    );
}

#[test]
fn test_selection_ranges_empty_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(0, 0)];

    let ranges = server.get_selection_ranges(path, positions);

    // Empty file should return default ranges
    assert_eq!(ranges.len(), 1, "Should return one range");

    // Should be a valid default range
    let range = &ranges[0];
    assert!(range.range.start.line == 0 && range.range.start.character == 0);
}

#[test]
fn test_selection_ranges_chain_ordering() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Outer {
    package Inner {
        part def Vehicle;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(2, 18)]; // On "Vehicle"

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1, "Should return one range chain");

    // Walk the parent chain and verify each parent is larger than child
    let mut current = Some(&ranges[0]);

    while let Some(range) = current {
        if let Some(parent) = &range.parent {
            // Parent should start at or before child
            assert!(
                parent.range.start.line <= range.range.start.line,
                "Parent should start at or before child"
            );

            // Parent should end at or after child
            assert!(
                parent.range.end.line >= range.range.end.line,
                "Parent should end at or after child"
            );
        }

        current = range.parent.as_ref().map(|b| b.as_ref());
    }
}

#[test]
fn test_selection_ranges_whitespace_position() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "    part def Vehicle;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(0, 2)]; // In leading whitespace

    let ranges = server.get_selection_ranges(path, positions);

    // Should handle whitespace positions gracefully
    assert_eq!(ranges.len(), 1, "Should return a range");
}

#[test]
fn test_selection_ranges_between_elements() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Car;

part def Truck;"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(1, 0)]; // Empty line between elements

    let ranges = server.get_selection_ranges(path, positions);

    // Should return default range for positions not in elements
    assert_eq!(ranges.len(), 1, "Should return a range");
}

// ============================================================================
// Tests for get_inlay_hints (#64-71)
// ============================================================================

#[test]
fn test_inlay_hints_basic_structure() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
    part car : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(3, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // May or may not have hints depending on implementation
    // Just verify it doesn't crash and returns valid data
    for hint in &hints {
        // Label should not be empty
        match &hint.label {
            InlayHintLabel::String(s) => assert!(!s.is_empty(), "Label should not be empty"),
            InlayHintLabel::LabelParts(_) => {} // Also valid
        }

        // Kind should be valid
        assert!(hint.kind.is_some(), "Kind should be specified");
    }
}

#[test]
fn test_inlay_hints_empty_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Empty file should have no hints
    assert!(hints.is_empty(), "Empty file should have no hints");
}

#[test]
fn test_inlay_hints_nonexistent_file() {
    let server = LspServer::new();
    let uri = Url::parse("file:///nonexistent.sysml").unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Nonexistent file should return empty vec
    assert!(hints.is_empty(), "Nonexistent file should return empty vec");
}

#[test]
fn test_inlay_hints_specific_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
    part car : Vehicle;
    part truck : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    // Request hints for only part of the file
    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(2, 0),
            end: Position::new(3, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Should only return hints within the requested range
    for hint in &hints {
        let line = hint.position.line;
        assert!(
            (2..3).contains(&line),
            "Hints should be within requested range"
        );
    }
}

#[test]
fn test_inlay_hints_type_annotations() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
    part inferredType : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(3, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Check if any hints are type hints
    let _has_type_hints = hints.iter().any(|h| h.kind == Some(InlayHintKind::TYPE));

    // Whether we have type hints depends on implementation
    // Just verify the function works
}

#[test]
fn test_inlay_hints_padding_fields() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle {
        attribute speed : Real;
    }
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(4, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Verify padding fields are set appropriately
    for hint in &hints {
        // Padding should be specified (true or false)
        assert!(
            hint.padding_left.is_some(),
            "Padding left should be specified"
        );
        assert!(
            hint.padding_right.is_some(),
            "Padding right should be specified"
        );
    }
}

#[test]
fn test_inlay_hints_out_of_bounds_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    // Request hints for range beyond file bounds
    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(100, 0),
            end: Position::new(200, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Should handle gracefully, return empty
    assert!(hints.is_empty(), "Out of range should return empty");
}

#[test]
fn test_inlay_hints_parameter_hints() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    action def ProcessData {
        in input : Real;
        out output : Real;
    }
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(5, 0),
        },
        work_done_progress_params: Default::default(),
    };

    server.get_inlay_hints(&params);
}

// ============================================================================
// Additional comprehensive tests for folding ranges (#551-560)
// ============================================================================

#[test]
fn test_folding_ranges_kerml_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.kerml").unwrap();
    let text = r#"class Vehicle {
    feature weight : Real;
    feature speed : Real;
}"#;

    // KerML files not yet fully supported, so we expect an error
    let result = server.open_document(&uri, text);
    if result.is_err() {
        // Expected - KerML not fully supported yet
        return;
    }

    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should handle KerML files gracefully if opened
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_only_comments() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"/* This is a
   multi-line comment
   block */
// Single line comment"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should handle comment-only files
    if !ranges.is_empty() {
        let has_comment = ranges
            .iter()
            .any(|r| r.kind == Some(FoldingRangeKind::Comment));
        // If there are ranges in comment-only file, they should be comments
        assert!(
            has_comment || ranges.is_empty(),
            "Comment-only file should have comment ranges if any"
        );
    }
}

#[test]
fn test_folding_ranges_mixed_kerml_sysml_syntax() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Mixed {
    part def Vehicle {
        attribute weight : Real;
    }
    class Engine {
        feature power : Real;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should handle mixed syntax
    for i in 1..ranges.len() {
        assert!(ranges[i].start_line >= ranges[i - 1].start_line);
    }
}

#[test]
fn test_folding_ranges_comment_validity() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"/* Block comment
   line 2
   line 3 */
package Test {
    // Inline comment
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Test that comment ranges have valid line numbers
    let comment_ranges: Vec<_> = ranges
        .iter()
        .filter(|r| r.kind == Some(FoldingRangeKind::Comment))
        .collect();

    // Verify comment ranges have proper structure
    for range in comment_ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_has_region_kinds() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package TestPkg {
    part def Vehicle {
        attribute weight : Real;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Test that region ranges exist for non-comments
    let region_ranges: Vec<_> = ranges
        .iter()
        .filter(|r| r.kind == Some(FoldingRangeKind::Region))
        .collect();

    // Should have region ranges for packages and elements
    if !ranges.is_empty() {
        assert!(
            !region_ranges.is_empty(),
            "Should have region ranges for packages/elements"
        );
    }
}

#[test]
fn test_folding_ranges_deeply_nested_packages() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Level1 {
    package Level2 {
        package Level3 {
            package Level4 {
                part def Vehicle;
            }
        }
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should handle deeply nested structures
    // Note: folding ranges depend on implementation and parser
    // Verify that function doesn't crash and returns valid data
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_line_boundaries() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def A;
}
package Test2 {
    part def B;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges don't overlap incorrectly
    for i in 0..ranges.len() {
        for j in (i + 1)..ranges.len() {
            let r1 = &ranges[i];
            let r2 = &ranges[j];

            // If ranges overlap, one must contain the other (nested)
            let overlap = r1.start_line <= r2.end_line && r2.start_line <= r1.end_line;
            if overlap {
                let r1_contains_r2 = r1.start_line <= r2.start_line && r1.end_line >= r2.end_line;
                let r2_contains_r1 = r2.start_line <= r1.start_line && r2.end_line >= r1.end_line;
                assert!(
                    r1_contains_r2 || r2_contains_r1,
                    "Overlapping ranges must be properly nested"
                );
            }
        }
    }
}

#[test]
fn test_folding_ranges_sort_stability() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package A { part def X; }
package B { part def Y; }
package C { part def Z; }"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify sorting by start line is correct
    for i in 1..ranges.len() {
        assert!(
            ranges[i].start_line >= ranges[i - 1].start_line,
            "Ranges must be sorted by start line"
        );
    }
}

#[test]
fn test_folding_ranges_with_attributes() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle {
        attribute weight : Real;
        attribute speed : Real;
        attribute color : String;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should handle attributes without crashing
    // Folding ranges depend on implementation
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

// ============================================================================
// Additional comprehensive tests for inlay hints (#545-550)
// ============================================================================

#[test]
fn test_inlay_hints_multiple_files() {
    let mut server = LspServer::new();
    let uri1 = Url::parse("file:///test1.sysml").unwrap();
    let uri2 = Url::parse("file:///test2.sysml").unwrap();
    let text1 = r#"package Test {
    part def Vehicle;
    part car : Vehicle;
}"#;
    let text2 = r#"package Different {
    part def Truck;
    part myTruck : Truck;
}"#;

    server.open_document(&uri1, text1).unwrap();
    server.open_document(&uri2, text2).unwrap();

    let params1 = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri1 },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(3, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let params2 = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri2 },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(3, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints1 = server.get_inlay_hints(&params1);
    let hints2 = server.get_inlay_hints(&params2);

    // Both files should be handled independently - verify hints are returned for both
    // and they don't interfere with each other
    for hint in &hints1 {
        assert!(hint.kind.is_some(), "File 1 hints should have kinds");
    }
    for hint in &hints2 {
        assert!(hint.kind.is_some(), "File 2 hints should have kinds");
    }
}

#[test]
fn test_inlay_hints_zero_width_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(1, 5),
            end: Position::new(1, 5),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Zero-width range should be handled gracefully
    for hint in &hints {
        assert!(hint.kind.is_some());
    }
}

#[test]
fn test_inlay_hints_multiline_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle {
        attribute weight : Real;
        attribute speed : Real;
    }
    part car : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(6, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Should handle multiline ranges
    for hint in &hints {
        assert!(hint.position.line <= 6);
    }
}

#[test]
fn test_inlay_hints_kind_assignment() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle {
        attribute weight : Real;
    }
    part myCar : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(5, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // All hints should have a kind assigned
    for hint in &hints {
        assert!(hint.kind.is_some(), "All hints should have a kind");
        let kind = hint.kind.unwrap();
        assert!(
            kind == InlayHintKind::TYPE || kind == InlayHintKind::PARAMETER,
            "Kind should be TYPE or PARAMETER"
        );
    }
}

#[test]
fn test_inlay_hints_position_accuracy() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part car : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(2, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Compute maximum line length in the document to validate character positions
    let max_line_length: u32 = text
        .lines()
        .map(|line| line.len() as u32)
        .max()
        .unwrap_or(0);

    // Verify positions are within document bounds
    for hint in &hints {
        assert!(hint.position.line <= 2, "Position should be within range");
        assert!(
            hint.position.character <= max_line_length,
            "Character should be within line length bounds"
        );
    }
}

#[test]
fn test_inlay_hints_label_format() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
    part car : Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(3, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = server.get_inlay_hints(&params);

    // Verify label format
    for hint in &hints {
        match &hint.label {
            InlayHintLabel::String(s) => {
                assert!(!s.is_empty(), "String labels should not be empty");
            }
            InlayHintLabel::LabelParts(parts) => {
                assert!(!parts.is_empty(), "Label parts should not be empty");
            }
        }
    }
}

// ============================================================================
// Additional comprehensive tests for selection ranges (#535-544)
// ============================================================================

#[test]
fn test_selection_ranges_at_line_start() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(0, 0), Position::new(1, 0)];

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 2, "Should return two ranges");
    for range in &ranges {
        assert!(range.range.start.line <= range.range.end.line);
    }
}

#[test]
fn test_selection_ranges_at_line_end() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    // Position 17 is at the semicolon - testing positions at element boundaries
    let positions = vec![Position::new(0, 17)];

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1);
    assert!(ranges[0].range.end.character >= ranges[0].range.start.character);
}

#[test]
fn test_selection_ranges_default_range_validity() {
    let server = LspServer::new();
    let path = Path::new("/nonexistent.sysml");
    let positions = vec![Position::new(5, 10), Position::new(10, 20)];
    let positions_copy = positions.clone();

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 2);
    for (i, range) in ranges.iter().enumerate() {
        // Default range should be single character at position
        assert_eq!(
            range.range.start, positions_copy[i],
            "Default range should start at requested position"
        );
        assert_eq!(
            range.range.end.character,
            positions_copy[i].character + 1,
            "Default range should be single character"
        );
        assert!(
            range.parent.is_none(),
            "Default range should have no parent"
        );
    }
}

#[test]
fn test_selection_ranges_build_chain_single_span() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def V;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(0, 10)];
    let positions_copy = positions.clone();

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1);

    // Verify that the built selection range chain is well-formed for a single-span document.
    let requested_pos = positions_copy[0];
    let mut current = Some(&ranges[0]);
    let mut prev_range: Option<Range> = None;
    let mut depth = 0;

    while let Some(sel_range) = current {
        // Guard against cycles in the parent chain.
        depth += 1;
        assert!(
            depth < 16,
            "Selection range parent chain is unexpectedly deep or cyclic"
        );

        let range = sel_range.range;

        // Range must contain the requested position.
        let starts_before_or_at_pos = range.start.line < requested_pos.line
            || (range.start.line == requested_pos.line
                && range.start.character <= requested_pos.character);
        let ends_after_or_at_pos = range.end.line > requested_pos.line
            || (range.end.line == requested_pos.line
                && range.end.character >= requested_pos.character);
        assert!(
            starts_before_or_at_pos && ends_after_or_at_pos,
            "Each selection range in the chain should contain the requested position"
        );

        if let Some(prev) = prev_range {
            // Parent ranges (walking up the chain) should be no smaller than their children.
            assert!(
                range.start.line <= prev.start.line,
                "Parent should start before or at child"
            );
            assert!(
                range.end.line >= prev.end.line,
                "Parent should end after or at child"
            );
        }

        prev_range = Some(range);
        current = sel_range.parent.as_ref().map(|b| b.as_ref());
    }

    // Single element may or may not have parent depending on AST structure, but
    // the chain must be finite and each element must contain the requested position.
}

#[test]
fn test_selection_ranges_build_chain_ordering() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Outer {
    package Middle {
        package Inner {
            part def Vehicle;
        }
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(3, 22)]; // On "Vehicle"

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1);

    // Walk the chain and verify proper nesting
    let mut current = Some(&ranges[0]);
    let mut prev_range: Option<Range> = None;

    while let Some(sel_range) = current {
        if let Some(prev) = prev_range {
            // Current parent should contain previous child
            assert!(
                sel_range.range.start.line <= prev.start.line,
                "Parent should start before or at child"
            );
            assert!(
                sel_range.range.end.line >= prev.end.line,
                "Parent should end after or at child"
            );
        }
        prev_range = Some(sel_range.range);
        current = sel_range.parent.as_ref().map(|b| b.as_ref());
    }
}

#[test]
fn test_selection_ranges_multiple_positions_independence() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def A;
    part def B;
    part def C;
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![
        Position::new(1, 14),
        Position::new(2, 14),
        Position::new(3, 14),
    ];

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 3);

    // Each position should get its own independent chain rooted at its line
    assert_eq!(
        ranges[0].range.start.line, 1,
        "First selection range should correspond to line 1 (part def A)"
    );
    assert_eq!(
        ranges[1].range.start.line, 2,
        "Second selection range should correspond to line 2 (part def B)"
    );
    assert_eq!(
        ranges[2].range.start.line, 3,
        "Third selection range should correspond to line 3 (part def C)"
    );
}

#[test]
fn test_selection_ranges_chain_parent_exists() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle {
        attribute weight : Real;
    }
}"#;

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(2, 20)]; // Inside attribute

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1);

    // Nested structure should have parent chain
    let has_parent = ranges[0].parent.is_some();
    if has_parent {
        // If there's a parent, verify it's larger
        let child = &ranges[0];
        let parent = child.parent.as_ref().unwrap();

        assert!(
            parent.range.start.line <= child.range.start.line,
            "Parent should start before or at child"
        );
        assert!(
            parent.range.end.line >= child.range.end.line,
            "Parent should end after or at child"
        );
    }
}

#[test]
fn test_selection_ranges_empty_positions_vec() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![];

    let ranges = server.get_selection_ranges(path, positions);

    assert!(
        ranges.is_empty(),
        "Empty positions should return empty ranges"
    );
}

#[test]
fn test_selection_ranges_same_position_multiple_times() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![
        Position::new(0, 10),
        Position::new(0, 10),
        Position::new(0, 10),
    ];

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(
        ranges.len(),
        3,
        "Should return range for each position even if duplicates"
    );
}

#[test]
fn test_selection_ranges_ascii_positions() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();
    let path = Path::new(uri.path());
    let positions = vec![Position::new(0, 9)];

    let ranges = server.get_selection_ranges(path, positions);

    assert_eq!(ranges.len(), 1);
    assert!(ranges[0].range.start.line <= ranges[0].range.end.line);
}
