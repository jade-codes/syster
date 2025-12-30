#![allow(clippy::unwrap_used)]

//! Tests for apply_text_edit helper function (Issue #118)
//!
//! Tests the application of LSP text edits to a document.
//! The function converts LSP Range (line, character) to byte offsets and applies the edit.

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
fn test_apply_text_edit_replacement_middle() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 6), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "earth").unwrap();
    assert_eq!(result, "hello earth");
}

#[test]
fn test_apply_text_edit_insertion_at_start() {
    let text = "world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "hello ").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_insertion_in_middle() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, " beautiful").unwrap();
    assert_eq!(result, "hello beautiful world");
}

#[test]
fn test_apply_text_edit_insertion_at_end() {
    let text = "hello";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, " world").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_deletion_start() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 6));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "world");
}

#[test]
fn test_apply_text_edit_deletion_middle() {
    let text = "hello beautiful world";
    let range = Range::new(Position::new(0, 6), Position::new(0, 16));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_deletion_end() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_apply_text_edit_complete_replacement() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "goodbye earth").unwrap();
    assert_eq!(result, "goodbye earth");
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
    let result = apply_text_edit(text, &range, "single line").unwrap();
    assert_eq!(result, "single line");
}

#[test]
fn test_apply_text_edit_insert_newline() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, "\n").unwrap();
    assert_eq!(result, "hello\n world");
}

#[test]
fn test_apply_text_edit_replace_with_multiline() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 6), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "beautiful\nworld").unwrap();
    assert_eq!(result, "hello beautiful\nworld");
}

#[test]
fn test_apply_text_edit_start_of_second_line() {
    let text = "line1\nline2";
    let range = Range::new(Position::new(1, 0), Position::new(1, 0));
    let result = apply_text_edit(text, &range, "prefix_").unwrap();
    assert_eq!(result, "line1\nprefix_line2");
}

#[test]
fn test_apply_text_edit_end_of_first_line() {
    let text = "line1\nline2";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, "_suffix").unwrap();
    assert_eq!(result, "line1_suffix\nline2");
}

// ========================================================================
// Unicode and Multi-byte Characters
// ========================================================================

#[test]
fn test_apply_text_edit_with_unicode_characters() {
    let text = "café wörld";
    // Replace "café" (4 chars, 5 bytes)
    let range = Range::new(Position::new(0, 0), Position::new(0, 4));
    let result = apply_text_edit(text, &range, "tea").unwrap();
    assert_eq!(result, "tea wörld");
}

#[test]
fn test_apply_text_edit_unicode_middle() {
    let text = "hello café world";
    let range = Range::new(Position::new(0, 6), Position::new(0, 10));
    let result = apply_text_edit(text, &range, "tea").unwrap();
    assert_eq!(result, "hello tea world");
}

#[test]
fn test_apply_text_edit_with_emoji() {
    let text = "Hello 😀 World";
    // Replace emoji
    let range = Range::new(Position::new(0, 6), Position::new(0, 7));
    let result = apply_text_edit(text, &range, "🎉").unwrap();
    assert_eq!(result, "Hello 🎉 World");
}

#[test]
fn test_apply_text_edit_multiple_emoji() {
    let text = "🎉🎊🎈";
    // Replace second emoji
    let range = Range::new(Position::new(0, 1), Position::new(0, 2));
    let result = apply_text_edit(text, &range, "✓").unwrap();
    assert_eq!(result, "🎉✓🎈");
}

#[test]
fn test_apply_text_edit_mixed_unicode_content() {
    let text = "Test: ✓ 🚀 Done";
    // Replace from checkmark to rocket
    let range = Range::new(Position::new(0, 6), Position::new(0, 9));
    let result = apply_text_edit(text, &range, "OK").unwrap();
    assert_eq!(result, "Test: OK Done");
}

// ========================================================================
// Edge Cases
// ========================================================================

#[test]
fn test_apply_text_edit_empty_text_insert() {
    let text = "";
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "hello").unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_apply_text_edit_empty_replacement() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_replace_all_with_empty() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "").unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_apply_text_edit_single_character() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 1));
    let result = apply_text_edit(text, &range, "H").unwrap();
    assert_eq!(result, "Hello world");
}

#[test]
fn test_apply_text_edit_whitespace() {
    let text = "hello world";
    // Replace single space with double space
    let range = Range::new(Position::new(0, 5), Position::new(0, 6));
    let result = apply_text_edit(text, &range, "  ").unwrap();
    assert_eq!(result, "hello  world");
}

#[test]
fn test_apply_text_edit_tabs_and_spaces() {
    let text = "hello\tworld";
    let range = Range::new(Position::new(0, 5), Position::new(0, 6));
    let result = apply_text_edit(text, &range, "    ").unwrap();
    assert_eq!(result, "hello    world");
}

#[test]
fn test_apply_text_edit_newlines_only() {
    let text = "\n\n\n";
    let range = Range::new(Position::new(1, 0), Position::new(1, 0));
    let result = apply_text_edit(text, &range, "text").unwrap();
    assert_eq!(result, "\ntext\n\n");
}

// ========================================================================
// Error Cases
// ========================================================================

#[test]
fn test_apply_text_edit_invalid_range_reversed() {
    let text = "hello world";
    // Start after end
    let range = Range::new(Position::new(0, 10), Position::new(0, 5));
    let result = apply_text_edit(text, &range, "test");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid range"));
}

#[test]
fn test_apply_text_edit_line_out_of_bounds() {
    let text = "hello world";
    // Line 5 doesn't exist
    let range = Range::new(Position::new(0, 0), Position::new(5, 0));
    let result = apply_text_edit(text, &range, "test");
    assert!(result.is_err());
}

#[test]
fn test_apply_text_edit_start_line_out_of_bounds() {
    let text = "hello world";
    // Start line doesn't exist
    let range = Range::new(Position::new(5, 0), Position::new(5, 5));
    let result = apply_text_edit(text, &range, "test");
    assert!(result.is_err());
}

#[test]
fn test_apply_text_edit_character_beyond_line() {
    let text = "hello world";
    // Character offset beyond line length should work (clamps to line end)
    let range = Range::new(Position::new(0, 0), Position::new(0, 100));
    let result = apply_text_edit(text, &range, "test").unwrap();
    assert_eq!(result, "test");
}

#[test]
fn test_apply_text_edit_multiline_out_of_bounds() {
    let text = "line1\nline2";
    // End line beyond document
    let range = Range::new(Position::new(0, 0), Position::new(10, 0));
    let result = apply_text_edit(text, &range, "test");
    assert!(result.is_err());
}

// ========================================================================
// Realistic SysML Examples
// ========================================================================

#[test]
fn test_apply_text_edit_sysml_rename_element() {
    let text = "part def Vehicle;";
    let range = Range::new(Position::new(0, 9), Position::new(0, 16));
    let result = apply_text_edit(text, &range, "Car").unwrap();
    assert_eq!(result, "part def Car;");
}

#[test]
fn test_apply_text_edit_sysml_add_attribute() {
    let text = "part def Vehicle {\n}";
    let range = Range::new(Position::new(1, 0), Position::new(1, 0));
    let result = apply_text_edit(text, &range, "    attribute mass : Real;\n").unwrap();
    assert_eq!(result, "part def Vehicle {\n    attribute mass : Real;\n}");
}

#[test]
fn test_apply_text_edit_sysml_change_keyword() {
    let text = "part def Vehicle;";
    let range = Range::new(Position::new(0, 0), Position::new(0, 8));
    let result = apply_text_edit(text, &range, "item def").unwrap();
    assert_eq!(result, "item def Vehicle;");
}

#[test]
fn test_apply_text_edit_sysml_multiline_block() {
    let text = "part def Vehicle {\n    attribute mass : Real;\n    attribute length : Real;\n}";
    // Replace entire attribute block
    let range = Range::new(Position::new(1, 4), Position::new(2, 32));
    let result = apply_text_edit(text, &range, "// Attributes removed").unwrap();
    assert_eq!(
        result,
        "part def Vehicle {\n    // Attributes removed\n}"
    );
}

// ========================================================================
// Zero-length Ranges (Insertions)
// ========================================================================

#[test]
fn test_apply_text_edit_zero_length_start() {
    let text = "world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "hello ").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_zero_length_middle() {
    let text = "helloworld";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, " ").unwrap();
    assert_eq!(result, "hello world");
}

#[test]
fn test_apply_text_edit_zero_length_end() {
    let text = "hello";
    let range = Range::new(Position::new(0, 5), Position::new(0, 5));
    let result = apply_text_edit(text, &range, " world").unwrap();
    assert_eq!(result, "hello world");
}

// ========================================================================
// Consecutive Edits Simulation
// ========================================================================

#[test]
fn test_apply_text_edit_consecutive_replacements() {
    let text = "hello world";
    // First edit
    let range1 = Range::new(Position::new(0, 0), Position::new(0, 5));
    let text1 = apply_text_edit(text, &range1, "goodbye").unwrap();
    assert_eq!(text1, "goodbye world");

    // Second edit on result (note: positions need recalculation in real LSP)
    let range2 = Range::new(Position::new(0, 8), Position::new(0, 13));
    let text2 = apply_text_edit(&text1, &range2, "earth").unwrap();
    assert_eq!(text2, "goodbye earth");
}

// ========================================================================
// Special Characters
// ========================================================================

#[test]
fn test_apply_text_edit_with_quotes() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 6), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "\"world\"").unwrap();
    assert_eq!(result, "hello \"world\"");
}

#[test]
fn test_apply_text_edit_with_backslashes() {
    let text = "path/to/file";
    let range = Range::new(Position::new(0, 0), Position::new(0, 12));
    let result = apply_text_edit(text, &range, "path\\to\\file").unwrap();
    assert_eq!(result, "path\\to\\file");
}

#[test]
fn test_apply_text_edit_with_special_symbols() {
    let text = "value = 10";
    let range = Range::new(Position::new(0, 8), Position::new(0, 10));
    let result = apply_text_edit(text, &range, "20 + 30").unwrap();
    assert_eq!(result, "value = 20 + 30");
}
