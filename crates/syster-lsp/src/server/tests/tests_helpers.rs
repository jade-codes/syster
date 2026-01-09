use crate::server::helpers::*;
use async_lsp::lsp_types::{Position, Range, Url};

// ========================================================================
// Tests for char_offset_to_utf16
// ========================================================================

#[test]
fn test_char_offset_to_utf16_ascii() {
    let line = "hello world";
    assert_eq!(char_offset_to_utf16(line, 0), 0);
    assert_eq!(char_offset_to_utf16(line, 5), 5);
    assert_eq!(char_offset_to_utf16(line, 11), 11);
}

#[test]
fn test_char_offset_to_utf16_empty_string() {
    let line = "";
    assert_eq!(char_offset_to_utf16(line, 0), 0);
}

#[test]
fn test_char_offset_to_utf16_emoji() {
    // Emoji take 2 UTF-16 code units
    let line = "Hello 😀 World";
    // "Hello " = 6 chars = 6 UTF-16 units
    assert_eq!(char_offset_to_utf16(line, 6), 6);
    // "Hello 😀" = 7 chars = 8 UTF-16 units (emoji is 2 units)
    assert_eq!(char_offset_to_utf16(line, 7), 8);
    // "Hello 😀 " = 8 chars = 9 UTF-16 units
    assert_eq!(char_offset_to_utf16(line, 8), 9);
}

#[test]
fn test_char_offset_to_utf16_multiple_emoji() {
    let line = "🎉🎊🎈";
    // Each emoji is 1 char but 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 0), 0);
    assert_eq!(char_offset_to_utf16(line, 1), 2); // After first emoji
    assert_eq!(char_offset_to_utf16(line, 2), 4); // After second emoji
    assert_eq!(char_offset_to_utf16(line, 3), 6); // After third emoji
}

#[test]
fn test_char_offset_to_utf16_mixed_content() {
    let line = "Test: ✓ 🚀 Done";
    // "Test: " = 6 ASCII = 6 UTF-16 units
    assert_eq!(char_offset_to_utf16(line, 6), 6);
    // "Test: ✓" = 7 chars (✓ is 1 UTF-16 unit) = 7 UTF-16 units
    assert_eq!(char_offset_to_utf16(line, 7), 7);
    // "Test: ✓ " = 8 chars = 8 UTF-16 units
    assert_eq!(char_offset_to_utf16(line, 8), 8);
    // "Test: ✓ 🚀" = 9 chars (🚀 is 2 UTF-16 units) = 10 UTF-16 units
    assert_eq!(char_offset_to_utf16(line, 9), 10);
}

#[test]
fn test_char_offset_to_utf16_unicode_characters() {
    // Test with various Unicode characters
    let line = "café";
    assert_eq!(char_offset_to_utf16(line, 0), 0);
    assert_eq!(char_offset_to_utf16(line, 3), 3); // 'é' is 1 UTF-16 unit
    assert_eq!(char_offset_to_utf16(line, 4), 4);
}

// ========================================================================
// Tests for char_offset_to_byte
// ========================================================================

#[test]
fn test_char_offset_to_byte_ascii() {
    let line = "hello world";
    assert_eq!(char_offset_to_byte(line, 0), 0);
    assert_eq!(char_offset_to_byte(line, 5), 5);
    assert_eq!(char_offset_to_byte(line, 11), 11);
}

#[test]
fn test_char_offset_to_byte_empty_string() {
    let line = "";
    assert_eq!(char_offset_to_byte(line, 0), 0);
}

#[test]
fn test_char_offset_to_byte_multi_byte_utf8() {
    // 'é' is 2 bytes in UTF-8
    let line = "café";
    assert_eq!(char_offset_to_byte(line, 0), 0);
    assert_eq!(char_offset_to_byte(line, 3), 3); // "caf" = 3 bytes
    assert_eq!(char_offset_to_byte(line, 4), 5); // "café" = 5 bytes (é is 2 bytes)
}

#[test]
fn test_char_offset_to_byte_emoji() {
    // Emoji are 4 bytes in UTF-8
    let line = "Hi 😀";
    assert_eq!(char_offset_to_byte(line, 0), 0);
    assert_eq!(char_offset_to_byte(line, 3), 3); // "Hi " = 3 bytes
    assert_eq!(char_offset_to_byte(line, 4), 7); // "Hi 😀" = 7 bytes (emoji is 4)
}

#[test]
fn test_char_offset_to_byte_mixed_content() {
    let line = "Test: ✓ Done";
    assert_eq!(char_offset_to_byte(line, 6), 6); // "Test: " = 6 bytes
    assert_eq!(char_offset_to_byte(line, 7), 9); // "Test: ✓" = 9 bytes (✓ is 3)
    assert_eq!(char_offset_to_byte(line, 8), 10); // "Test: ✓ " = 10 bytes
}

// ========================================================================
// Tests for position_to_byte_offset
// ========================================================================

#[test]
fn test_position_to_byte_offset_single_line() {
    let text = "hello world";
    let pos = Position::new(0, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 0);

    let pos = Position::new(0, 5);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 5);

    let pos = Position::new(0, 11);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 11);
}

#[test]
fn test_position_to_byte_offset_multi_line() {
    let text = "line1\nline2\nline3";
    // Start of line 0
    let pos = Position::new(0, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 0);

    // Start of line 1 (after "line1\n" = 6 bytes)
    let pos = Position::new(1, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 6);

    // Start of line 2 (after "line1\nline2\n" = 12 bytes)
    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 12);

    // Middle of line 1
    let pos = Position::new(1, 3);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 9); // 6 + 3
}

#[test]
fn test_position_to_byte_offset_end_of_document() {
    let text = "line1\nline2";
    // Position at line count (end of document)
    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), text.len());
}

#[test]
fn test_position_to_byte_offset_out_of_bounds() {
    let text = "line1\nline2";
    // Line beyond document
    let pos = Position::new(3, 0);
    assert!(position_to_byte_offset(text, pos).is_err());
}

#[test]
fn test_position_to_byte_offset_empty_text() {
    let text = "";
    let pos = Position::new(0, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 0);
}

#[test]
fn test_position_to_byte_offset_with_unicode() {
    let text = "café\nwörld";
    // Start of line 0
    let pos = Position::new(0, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 0);

    // After "café" (5 bytes)
    let pos = Position::new(0, 4);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 5);

    // Start of line 1 (after "café\n" = 6 bytes)
    let pos = Position::new(1, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 6);
}

// ========================================================================
// Tests for apply_text_edit
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
fn test_apply_text_edit_multi_line() {
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
fn test_apply_text_edit_invalid_range() {
    let text = "hello world";
    // Start after end
    let range = Range::new(Position::new(0, 5), Position::new(0, 3));
    assert!(apply_text_edit(text, &range, "test").is_err());
}

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
fn test_apply_text_edit_out_of_bounds_line() {
    let text = "hello world";
    // Line beyond document bounds should error
    let range = Range::new(Position::new(0, 0), Position::new(5, 0));
    assert!(apply_text_edit(text, &range, "test").is_err());
}

#[test]
fn test_apply_text_edit_empty_text() {
    let text = "";
    let range = Range::new(Position::new(0, 0), Position::new(0, 0));
    let result = apply_text_edit(text, &range, "hello").unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_apply_text_edit_with_unicode() {
    let text = "café wörld";
    // Replace "café" (4 chars, 5 bytes)
    let range = Range::new(Position::new(0, 0), Position::new(0, 4));
    let result = apply_text_edit(text, &range, "tea").unwrap();
    assert_eq!(result, "tea wörld");
}

#[test]
fn test_apply_text_edit_complete_replacement() {
    let text = "hello world";
    let range = Range::new(Position::new(0, 0), Position::new(0, 11));
    let result = apply_text_edit(text, &range, "goodbye").unwrap();
    assert_eq!(result, "goodbye");
}

// ========================================================================
// Tests for URL path segment decoding behavior
// ========================================================================

#[test]
fn test_url_path_segments_needs_decoding() {
    // Test that path_segments() does NOT automatically decode URL encoding
    let url = Url::parse("file:///path/to/my%20test%20file.sysml").unwrap();
    let file_name = url
        .path_segments()
        .and_then(|mut s| s.next_back())
        .unwrap_or("unknown");

    // path_segments() returns encoded string
    assert_eq!(file_name, "my%20test%20file.sysml");

    // Use decode_uri_component to decode it
    let decoded = decode_uri_component(file_name);
    assert_eq!(decoded, "my test file.sysml");
}

#[test]
fn test_decode_uri_component_with_spaces() {
    let encoded = "my%20file%20name.txt";
    let decoded = decode_uri_component(encoded);
    assert_eq!(decoded, "my file name.txt");
}

#[test]
fn test_decode_uri_component_with_special_chars() {
    let encoded = "file%2Bname%28test%29.txt";
    let decoded = decode_uri_component(encoded);
    assert_eq!(decoded, "file+name(test).txt");
}

#[test]
fn test_decode_uri_component_no_encoding() {
    let plain = "simple.txt";
    let decoded = decode_uri_component(plain);
    assert_eq!(decoded, "simple.txt");
}

#[test]
fn test_decode_uri_component_mixed() {
    let encoded = "test%20file.txt";
    let decoded = decode_uri_component(encoded);
    assert_eq!(decoded, "test file.txt");
}
