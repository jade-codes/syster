use crate::server::helpers::char_offset_to_byte;

// ============================================================================
// ASCII String Tests
// ============================================================================

#[test]
fn test_empty_string() {
    let line = "";
    assert_eq!(char_offset_to_byte(line, 0), 0);
}

#[test]
fn test_empty_string_with_offset() {
    // Offset beyond empty string should return 0 (no chars to take)
    let line = "";
    assert_eq!(char_offset_to_byte(line, 5), 0);
}

#[test]
fn test_ascii_zero_offset() {
    let line = "hello";
    assert_eq!(char_offset_to_byte(line, 0), 0);
}

#[test]
fn test_ascii_single_char() {
    let line = "hello";
    // First character 'h' is 1 byte
    assert_eq!(char_offset_to_byte(line, 1), 1);
}

#[test]
fn test_ascii_multiple_chars() {
    let line = "hello world";
    // "hello" = 5 bytes
    assert_eq!(char_offset_to_byte(line, 5), 5);
    // "hello worl" = 10 bytes
    assert_eq!(char_offset_to_byte(line, 10), 10);
}

#[test]
fn test_ascii_full_string() {
    let line = "hello";
    // All 5 characters = 5 bytes
    assert_eq!(char_offset_to_byte(line, 5), 5);
}

#[test]
fn test_ascii_offset_beyond_length() {
    let line = "hello";
    // Offset 10 is beyond length 5, should return total byte length
    assert_eq!(char_offset_to_byte(line, 10), 5);
}

// ============================================================================
// Multi-byte UTF-8 Character Tests
// ============================================================================

#[test]
fn test_single_2byte_char() {
    // "é" is 2 bytes in UTF-8 (U+00E9)
    let line = "café";
    // "caf" = 3 ASCII bytes + "é" = 2 bytes = 5 total
    assert_eq!(char_offset_to_byte(line, 4), 5);
}

#[test]
fn test_single_3byte_char() {
    // "€" is 3 bytes in UTF-8 (U+20AC)
    let line = "€100";
    // "€" = 3 bytes
    assert_eq!(char_offset_to_byte(line, 1), 3);
    // "€1" = 3 + 1 = 4 bytes
    assert_eq!(char_offset_to_byte(line, 2), 4);
}

#[test]
fn test_single_4byte_emoji() {
    // "😀" is 4 bytes in UTF-8 (U+1F600)
    let line = "😀hi";
    // "😀" = 4 bytes
    assert_eq!(char_offset_to_byte(line, 1), 4);
    // "😀h" = 4 + 1 = 5 bytes
    assert_eq!(char_offset_to_byte(line, 2), 5);
    // "😀hi" = 4 + 1 + 1 = 6 bytes
    assert_eq!(char_offset_to_byte(line, 3), 6);
}

#[test]
fn test_mixed_ascii_and_multibyte() {
    // Mix of 1-byte, 2-byte, 3-byte, 4-byte characters
    let line = "a€é😀b";
    // "a" = 1 byte
    assert_eq!(char_offset_to_byte(line, 1), 1);
    // "a€" = 1 + 3 = 4 bytes
    assert_eq!(char_offset_to_byte(line, 2), 4);
    // "a€é" = 1 + 3 + 2 = 6 bytes
    assert_eq!(char_offset_to_byte(line, 3), 6);
    // "a€é😀" = 1 + 3 + 2 + 4 = 10 bytes
    assert_eq!(char_offset_to_byte(line, 4), 10);
    // "a€é😀b" = 1 + 3 + 2 + 4 + 1 = 11 bytes
    assert_eq!(char_offset_to_byte(line, 5), 11);
}

#[test]
fn test_multiple_emojis() {
    // Multiple 4-byte emoji characters
    let line = "😀😁😂";
    // "😀" = 4 bytes
    assert_eq!(char_offset_to_byte(line, 1), 4);
    // "😀😁" = 4 + 4 = 8 bytes
    assert_eq!(char_offset_to_byte(line, 2), 8);
    // "😀😁😂" = 4 + 4 + 4 = 12 bytes
    assert_eq!(char_offset_to_byte(line, 3), 12);
}

#[test]
fn test_chinese_characters() {
    // Chinese characters are typically 3 bytes in UTF-8
    let line = "你好世界"; // "Hello World" in Chinese
    // "你" = 3 bytes
    assert_eq!(char_offset_to_byte(line, 1), 3);
    // "你好" = 3 + 3 = 6 bytes
    assert_eq!(char_offset_to_byte(line, 2), 6);
    // "你好世" = 3 + 3 + 3 = 9 bytes
    assert_eq!(char_offset_to_byte(line, 3), 9);
    // "你好世界" = 3 + 3 + 3 + 3 = 12 bytes
    assert_eq!(char_offset_to_byte(line, 4), 12);
}

#[test]
fn test_cyrillic_characters() {
    // Cyrillic characters are typically 2 bytes in UTF-8
    let line = "Привет"; // "Hello" in Russian
    // "П" = 2 bytes
    assert_eq!(char_offset_to_byte(line, 1), 2);
    // "Пр" = 2 + 2 = 4 bytes
    assert_eq!(char_offset_to_byte(line, 2), 4);
    // Full string = 6 chars * 2 bytes = 12 bytes
    assert_eq!(char_offset_to_byte(line, 6), 12);
}

#[test]
fn test_arabic_characters() {
    // Arabic characters are typically 2 bytes in UTF-8
    let line = "مرحبا"; // "Hello" in Arabic
    // "م" = 2 bytes
    assert_eq!(char_offset_to_byte(line, 1), 2);
    // "مر" = 2 + 2 = 4 bytes
    assert_eq!(char_offset_to_byte(line, 2), 4);
}

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn test_whitespace_only() {
    let line = "     "; // 5 spaces
    // 3 spaces = 3 bytes
    assert_eq!(char_offset_to_byte(line, 3), 3);
    // All 5 spaces = 5 bytes
    assert_eq!(char_offset_to_byte(line, 5), 5);
}

#[test]
fn test_tabs_and_newlines() {
    let line = "\t\n\r";
    // Tab = 1 byte
    assert_eq!(char_offset_to_byte(line, 1), 1);
    // Tab + newline = 2 bytes
    assert_eq!(char_offset_to_byte(line, 2), 2);
    // Tab + newline + carriage return = 3 bytes
    assert_eq!(char_offset_to_byte(line, 3), 3);
}

#[test]
fn test_zero_width_joiner() {
    // Zero-width joiner (U+200D) is 3 bytes
    let line = "a\u{200D}b";
    // "a" = 1 byte
    assert_eq!(char_offset_to_byte(line, 1), 1);
    // "a\u{200D}" = 1 + 3 = 4 bytes
    assert_eq!(char_offset_to_byte(line, 2), 4);
    // "a\u{200D}b" = 1 + 3 + 1 = 5 bytes
    assert_eq!(char_offset_to_byte(line, 3), 5);
}

#[test]
fn test_combining_diacritics() {
    // "é" can be represented as "e" + combining acute accent
    let line = "e\u{0301}"; // e + combining acute accent
    // "e" = 1 byte
    assert_eq!(char_offset_to_byte(line, 1), 1);
    // "e\u{0301}" = 1 + 2 = 3 bytes (combining acute is 2 bytes)
    assert_eq!(char_offset_to_byte(line, 2), 3);
}

#[test]
fn test_grapheme_cluster_emoji() {
    // Family emoji created with zero-width joiners
    // 👨‍👩‍👧‍👦 = man + ZWJ + woman + ZWJ + girl + ZWJ + boy
    let line = "hi👨‍👩‍👧‍👦!";
    // "h" = 1 byte
    assert_eq!(char_offset_to_byte(line, 1), 1);
    // "hi" = 2 bytes
    assert_eq!(char_offset_to_byte(line, 2), 2);
    // The emoji cluster has 7 characters but appears as one grapheme
    // We're testing char count, not grapheme count
    // Man emoji (👨) = 4 bytes
    assert_eq!(char_offset_to_byte(line, 3), 6);
}

#[test]
fn test_special_unicode_symbols() {
    // Various special symbols with different byte sizes
    let line = "→←↑↓"; // Arrows, each 3 bytes
    // "→" = 3 bytes
    assert_eq!(char_offset_to_byte(line, 1), 3);
    // "→←" = 3 + 3 = 6 bytes
    assert_eq!(char_offset_to_byte(line, 2), 6);
    // "→←↑↓" = 3 + 3 + 3 + 3 = 12 bytes
    assert_eq!(char_offset_to_byte(line, 4), 12);
}

// ============================================================================
// Real-world SysML/KerML Code Tests
// ============================================================================

#[test]
fn test_sysml_code_ascii() {
    let line = "part def Vehicle;";
    // "part" = 4 bytes
    assert_eq!(char_offset_to_byte(line, 4), 4);
    // "part def" = 8 bytes
    assert_eq!(char_offset_to_byte(line, 8), 8);
    // "part def Vehicle" = 16 bytes
    assert_eq!(char_offset_to_byte(line, 16), 16);
}

#[test]
fn test_sysml_code_with_unicode_identifier() {
    // Valid identifiers can contain Unicode
    let line = "part def Véhicule;";
    // "part def V" = 10 bytes
    assert_eq!(char_offset_to_byte(line, 10), 10);
    // "part def Vé" = 10 + 2 = 12 bytes (é is 2 bytes)
    assert_eq!(char_offset_to_byte(line, 11), 12);
}

#[test]
fn test_sysml_comment_with_emoji() {
    // Comments might contain emoji
    let line = "// TODO: Fix this 🐛";
    // "// TODO: " = 9 bytes
    assert_eq!(char_offset_to_byte(line, 9), 9);
    // "// TODO: Fix this " = 18 bytes
    assert_eq!(char_offset_to_byte(line, 18), 18);
    // "// TODO: Fix this 🐛" = 18 + 4 = 22 bytes
    assert_eq!(char_offset_to_byte(line, 19), 22);
}

#[test]
fn test_import_statement_with_qualified_name() {
    let line = "import Package::SubPackage::Element;";
    // "import Package::" = 16 bytes
    assert_eq!(char_offset_to_byte(line, 16), 16);
    // Full line (36 chars)
    assert_eq!(char_offset_to_byte(line, 36), 36);
}

// ============================================================================
// Performance and Large Input Tests
// ============================================================================

#[test]
fn test_long_ascii_string() {
    // Test with a longer ASCII string
    let line = "a".repeat(1000);
    // 100 chars = 100 bytes
    assert_eq!(char_offset_to_byte(&line, 100), 100);
    // 500 chars = 500 bytes
    assert_eq!(char_offset_to_byte(&line, 500), 500);
    // All 1000 chars = 1000 bytes
    assert_eq!(char_offset_to_byte(&line, 1000), 1000);
}

#[test]
fn test_long_multibyte_string() {
    // Test with a longer multi-byte string
    let line = "€".repeat(100);
    // 10 Euro signs = 10 * 3 = 30 bytes
    assert_eq!(char_offset_to_byte(&line, 10), 30);
    // 50 Euro signs = 50 * 3 = 150 bytes
    assert_eq!(char_offset_to_byte(&line, 50), 150);
    // All 100 Euro signs = 100 * 3 = 300 bytes
    assert_eq!(char_offset_to_byte(&line, 100), 300);
}

// ============================================================================
// Consistency Tests
// ============================================================================

#[test]
fn test_monotonic_increase() {
    // Byte offset should always increase (or stay same) as char offset increases
    let line = "hello 世界 😀!";
    let mut prev_bytes = 0;
    for char_offset in 0..=line.chars().count() {
        let bytes = char_offset_to_byte(line, char_offset);
        assert!(
            bytes >= prev_bytes,
            "Byte offset should increase monotonically: offset {char_offset}, bytes {bytes} < prev {prev_bytes}"
        );
        prev_bytes = bytes;
    }
}

#[test]
fn test_idempotent_at_length() {
    // Calling with offset >= length should always return same result
    let line = "test €";
    let len = line.chars().count();
    let expected = char_offset_to_byte(line, len);

    assert_eq!(char_offset_to_byte(line, len + 1), expected);
    assert_eq!(char_offset_to_byte(line, len + 10), expected);
    assert_eq!(char_offset_to_byte(line, len + 100), expected);
}

#[test]
fn test_round_trip_consistency() {
    // For ASCII, char offset should equal byte offset
    let line = "hello world";
    for i in 0..=line.len() {
        assert_eq!(char_offset_to_byte(line, i), i);
    }
}

// ============================================================================
// Integration with position_to_byte_offset Tests
// ============================================================================

#[test]
fn test_integration_with_multiline_text() {
    // Test that char_offset_to_byte works correctly as part of position_to_byte_offset
    use crate::server::helpers::position_to_byte_offset;
    use async_lsp::lsp_types::Position;

    let text = "line1\nliné2\nlin€3";

    // Position at start of line 1
    let result = position_to_byte_offset(text, Position::new(1, 0)).unwrap();
    assert_eq!(result, 6); // "line1\n" = 6 bytes

    // Position at char offset 4 in line 1 (after "liné")
    let result = position_to_byte_offset(text, Position::new(1, 4)).unwrap();
    // "line1\n" = 6 bytes, "liné" = 1+1+1+2 = 5 bytes, total = 11 bytes
    assert_eq!(result, 11);

    // Position at start of line 2
    let result = position_to_byte_offset(text, Position::new(2, 0)).unwrap();
    assert_eq!(result, 13); // "line1\nliné2\n" = 13 bytes
}
