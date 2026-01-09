#![allow(clippy::unwrap_used)]

//! Tests for position_to_byte_offset helper function (Issue #121)
//!
//! Tests the conversion of LSP Position (line, character) to byte offset in text.

use crate::server::helpers::position_to_byte_offset;
use async_lsp::lsp_types::Position;

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

// CRLF line ending tests (Windows-style line endings)

#[test]
fn test_position_to_byte_offset_crlf_second_line_start() {
    // CRLF line endings: \r\n is 2 bytes
    let text = "part def Car;\r\npart def Bike;";
    let pos = Position::new(1, 0);
    // "part def Car;\r\n" = 15 bytes (13 + 2 for \r\n)
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 15);
}

#[test]
fn test_position_to_byte_offset_crlf_second_line_middle() {
    let text = "part def Car;\r\npart def Bike;";
    let pos = Position::new(1, 5);
    // "part def Car;\r\n" = 15 bytes + 5 = 20
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 20);
}

#[test]
fn test_position_to_byte_offset_crlf_multiline() {
    let text = "line1\r\nline2\r\nline3";
    // Line 0: "line1\r\n" starts at 0, length 7
    // Line 1: "line2\r\n" starts at 7, length 7
    // Line 2: "line3" starts at 14
    let pos = Position::new(0, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 0);

    let pos = Position::new(1, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 7);

    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 14);
}

#[test]
fn test_position_to_byte_offset_crlf_single_char_lines() {
    let text = "a\r\nb\r\nc";
    // Line 0: "a\r\n" = 3 bytes
    // Line 1: "b\r\n" = 3 bytes (starts at byte 3)
    // Line 2: "c" starts at byte 6
    let pos = Position::new(1, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 3);

    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 6);
}

#[test]
fn test_position_to_byte_offset_crlf_edit_at_semicolon() {
    // This test simulates the exact scenario that caused the bug:
    // editing a semicolon in a CRLF file
    let text = "behavior Behavior1 {\r\n    step s1;\r\n}";
    // Line 0: "behavior Behavior1 {\r\n" = 22 bytes
    // Line 1: "    step s1;\r\n" starts at byte 22
    // Position of semicolon on line 1: column 11 (0-indexed)
    let pos = Position::new(1, 11);
    // byte 22 + 11 = 33
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), 33);

    // Verify the character at that position is the semicolon
    let offset = position_to_byte_offset(text, pos).unwrap();
    assert_eq!(&text[offset..offset + 1], ";");
}

#[test]
fn test_position_to_byte_offset_crlf_vs_lf_difference() {
    // Same logical content with LF vs CRLF should give different byte offsets
    let text_lf = "line1\nline2";
    let text_crlf = "line1\r\nline2";

    let pos = Position::new(1, 0);

    // LF: "line1\n" = 6 bytes
    assert_eq!(position_to_byte_offset(text_lf, pos).unwrap(), 6);

    // CRLF: "line1\r\n" = 7 bytes
    assert_eq!(position_to_byte_offset(text_crlf, pos).unwrap(), 7);
}

#[test]
fn test_position_to_byte_offset_crlf_end_of_document() {
    let text = "line1\r\nline2";
    // Position at end of document (line 2, which is past last line)
    let pos = Position::new(2, 0);
    assert_eq!(position_to_byte_offset(text, pos).unwrap(), text.len());
}
