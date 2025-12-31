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
