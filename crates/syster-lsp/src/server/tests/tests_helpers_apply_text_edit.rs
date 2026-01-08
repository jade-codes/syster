#![allow(clippy::unwrap_used)]

//! Tests for apply_text_edit helper function (Issue #118)
//!
//! Tests the application of text edits based on LSP Range.
//! The function converts LSP Position (line, character) to byte offset and performs the edit.

use crate::server::helpers::apply_text_edit;
use async_lsp::lsp_types::{Position, Range};

// ========================================================================
// Basic Operations
// ========================================================================

#[test]
fn test_apply_text_edit_simple_replacement() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    let result = apply_text_edit(text, &range, "goodbye").unwrap();
    assert_eq!(result, "goodbye world");
}

#[test]
fn test_apply_text_edit_insertion() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, " beautiful").unwrap();
    assert_eq!(result, "hello beautiful world");
}

#[test]
fn test_apply_text_edit_deletion() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_apply_text_edit_complete_replacement() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "goodbye").unwrap();
    assert_eq!(result, "goodbye");
}

#[test]
fn test_apply_text_edit_no_change() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "hello world");
}

// ========================================================================
// Multi-line Operations
// ========================================================================

#[test]
fn test_apply_text_edit_single_line_in_multiline() {
    let text = "line1\nline2\nline3";
    let range = Range::new(Position::new(1, 0), Position::new(1, 5));
    let result = apply_text_edit(text, &range, "REPLACED").unwrap();
    assert_eq!(result, "line1\nREPLACED\nline3");
}

#[test]
fn test_apply_text_edit_across_lines() {
    let text = "line1\nline2\nline3";
    let range = Range::new(Position::new(0, 3), Position::new(1, 3));
    let result = apply_text_edit(text, &range, "X").unwrap();
    assert_eq!(result, "linXe2\nline3");
}

#[test]
fn test_apply_text_edit_multiple_lines() {
    let text = "line1\nline2\nline3";
    let range = Range::new(Position::new(0, 0), Position::new(2, 5));
    let result = apply_text_edit(text, &range, "NEWTEXT").unwrap();
    assert_eq!(result, "NEWTEXT");
}

#[test]
fn test_apply_text_edit_insert_at_line_start() {
    let text = "line1\nline2\nline3";
    let range = Range::new(Position::new(1, 0), Position::new(1, 0));
    let result = apply_text_edit(text, &range, "PREFIX").unwrap();
    assert_eq!(result, "line1\nPREFIXline2\nline3");
}

#[test]
fn test_apply_text_edit_insert_at_line_end() {
    let text = "line1\nline2\nline3";
    let range = Range::new(Position::new(1, 5), Position::new(1, 5));
    let result = apply_text_edit(text, &range, "SUFFIX").unwrap();
    assert_eq!(result, "line1\nline2SUFFIX\nline3");
}

// ========================================================================
// Edge Cases
// ========================================================================

#[test]
fn test_apply_text_edit_empty_text() {
    let text = "";
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "hello").unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_apply_text_edit_empty_replacement() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_at_document_start() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "START ").unwrap();
    assert_eq!(result, "START hello world");
}

#[test]
fn test_apply_text_edit_at_document_end() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 11), Position::new(0, 11));
    let result = apply_text_edit(text, &range, " END").unwrap();
    assert_eq!(result, "hello world END");
}

#[test]
fn test_apply_text_edit_single_character() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 1));
    let result = apply_text_edit(text, &range, "H").unwrap();
    assert_eq!(result, "Hello world");
}

#[test]
fn test_apply_text_edit_replace_middle_word() {
    let text = "hello beautiful world";
    let range = Range::new(Position::new(0, 6), Position::new(0, 15));
    let result = apply_text_edit(text, &range, "amazing").unwrap();
    assert_eq!(result, "hello amazing world");
}

// ========================================================================
// Error Conditions
// ========================================================================

#[test]
fn test_apply_text_edit_invalid_range_start_after_end() {
    let text = "hello world";
    // Start after end
    let range = Range::new(Position::new(0, 5), Position::new(0, 3));
    let result = apply_text_edit(text, &range, "test");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid range"));
}

#[test]
fn test_apply_text_edit_out_of_bounds_line() {
    let text = "hello world";
    // Line beyond document bounds should error
    let range = Range::new(Position::new(0, 0), Position::new(5, 0));
    let result = apply_text_edit(text, &range, "test");
    assert!(result.is_err());
}

#[test]
fn test_apply_text_edit_start_line_out_of_bounds() {
    let text = "line1\nline2";
    // Start line beyond document
    let range = Range::new(Position::new(10, 0), Position::new(10, 5));
    let result = apply_text_edit(text, &range, "test");
    assert!(result.is_err());
}

// ========================================================================
// Boundary Values
// ========================================================================

#[test]
fn test_apply_text_edit_beyond_char_offset() {
    let text = "hello world";
    // Character offset beyond line length clamps to line end
    let range = Range::new(Position::new(0, 0), Position::new(0, 100));
    let result = apply_text_edit(text, &range, "test").unwrap();
    // Should replace entire line since offset clamps to end
    assert_eq!(result, "test");
}

#[test]
fn test_apply_text_edit_at_line_boundary() {
    let text = "line1\nline2";
    // Edit at the newline boundary
    let range = Range::new(Position::new(0, 5), Position::new(1, 0));
    let result = apply_text_edit(text, &range, "X").unwrap();
    assert_eq!(result, "line1Xline2");
}

#[test]
fn test_apply_text_edit_entire_document() {
    let text = "line1\nline2\nline3";
    let range = Range::new(Position::new(0, 0), Position::new(2, 5));
    let result = apply_text_edit(text, &range, "NEW").unwrap();
    assert_eq!(result, "NEW");
}

#[test]
fn test_apply_text_edit_zero_length_range() {
    let text = "hello world";
    // Zero-length range (insertion point)
    let range = Range::new(Position::new(0, 6), Position::new(0, 6));
    let result = apply_text_edit(text, &range, "beautiful ").unwrap();
    assert_eq!(result, "hello beautiful world");
}

// ========================================================================
// Unicode Handling
// ========================================================================

#[test]
fn test_apply_text_edit_with_unicode() {
    let text = "café wörld";
    // Replace "café" (4 chars, 5 bytes)
    let range = Range::new(Position::new(0, 0), Position::new(0, 4));
    let result = apply_text_edit(text, &range, "tea").unwrap();
    assert_eq!(result, "tea wörld");
}

#[test]
fn test_apply_text_edit_unicode_multiline() {
    let text = "café\nwörld";
    let range = Range::new(Position::new(0, 2), Position::new(1, 2));
    let result = apply_text_edit(text, &range, "X").unwrap();
    assert_eq!(result, "caXrld");
}

#[test]
fn test_apply_text_edit_with_emoji() {
    let text = "Hello 😀 World";
    // Replace the emoji (1 Rust char / Unicode scalar value, 4 bytes in UTF-8, 2 UTF-16 code units;
    // note: this implementation uses character-based positions, not UTF-16 code units as in strict LSP)
    let range = Range::new(Position::new(0, 6), Position::new(0, 7));
    let result = apply_text_edit(text, &range, "🎉").unwrap();
    assert_eq!(result, "Hello 🎉 World");
}

#[test]
fn test_apply_text_edit_emoji_at_boundaries() {
    let text = "😀😀😀";
    // Replace middle emoji
    let range = Range::new(Position::new(0, 1), Position::new(0, 2));
    let result = apply_text_edit(text, &range, "X").unwrap();
    assert_eq!(result, "😀X😀");
}

#[test]
fn test_apply_text_edit_mixed_unicode() {
    let text = "Test: ✓ 🚀 Done";
    // Replace "✓ 🚀" (checkmark, space, rocket)
    let range = Range::new(Position::new(0, 6), Position::new(0, 9));
    let result = apply_text_edit(text, &range, "PASS").unwrap();
    assert_eq!(result, "Test: PASS Done");
}

// ========================================================================
// SysML-specific Cases
// ========================================================================

#[test]
fn test_apply_text_edit_sysml_keyword() {
    let text = "part def Vehicle;";
    // Replace "part" with "package"
    let range = Range::new(Position::new(0, 0), Position::new(0, 4));
    let result = apply_text_edit(text, &range, "package").unwrap();
    assert_eq!(result, "package def Vehicle;");
}

#[test]
fn test_apply_text_edit_sysml_identifier() {
    let text = "part def Vehicle;";
    // Replace "Vehicle" with "Car"
    let range = Range::new(Position::new(0, 9), Position::new(0, 16));
    let result = apply_text_edit(text, &range, "Car").unwrap();
    assert_eq!(result, "part def Car;");
}

#[test]
fn test_apply_text_edit_sysml_block() {
    let text = "part def Vehicle {\n  part engine;\n}";
    // Replace the content inside braces
    let range = Range::new(Position::new(1, 2), Position::new(1, 15));
    let result = apply_text_edit(text, &range, "attribute mass;").unwrap();
    assert_eq!(result, "part def Vehicle {\n  attribute mass;\n}");
}

#[test]
fn test_apply_text_edit_add_sysml_annotation() {
    let text = "part def Vehicle;";
    // Insert documentation before definition
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "doc /* A vehicle */\n").unwrap();
    assert_eq!(result, "doc /* A vehicle */\npart def Vehicle;");
}

// ========================================================================
// Whitespace Handling
// ========================================================================

#[test]
fn test_apply_text_edit_preserve_trailing_whitespace() {
    let text = "hello   world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 8));
    let result = apply_text_edit(text, &range, " ").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_preserve_leading_whitespace() {
    let text = "   hello world";
    let range = Range::new(Position::new(0, 3), Position::new(0, 8));
    let result = apply_text_edit(text, &range, "hi").unwrap();
    assert_eq!(result, "   hi world");
}

#[test]
fn test_apply_text_edit_tabs_and_spaces() {
    let text = "\thello\t world";
    let range = Range::new(Position::new(0, 1), Position::new(0, 6));
    let result = apply_text_edit(text, &range, "hi").unwrap();
    assert_eq!(result, "\thi\t world");
}

// ========================================================================
// Large Text Operations
// ========================================================================

#[test]
fn test_apply_text_edit_large_text() {
    let text = "a".repeat(1000);
    let range = Range::new(Position::new(0, 0), Position::new(0, 1000));
    let result = apply_text_edit(&text, &range, "b").unwrap();
    assert_eq!(result, "b");
}

#[test]
fn test_apply_text_edit_large_replacement() {
    let text = "short";
    let range = Range::new(Position::new(0, 0), Position::new(0, 5));
    let large_replacement = "x".repeat(10000);
    let result = apply_text_edit(text, &range, &large_replacement).unwrap();
    assert_eq!(result, large_replacement);
}

#[test]
fn test_apply_text_edit_many_lines() {
    let text = (0..100)
        .map(|i| format!("line{i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let range = Range::new(Position::new(50, 0), Position::new(50, 6));
    let result = apply_text_edit(&text, &range, "REPLACED").unwrap();
    assert!(result.contains("REPLACED"));
}
