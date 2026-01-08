use crate::server::helpers::char_offset_to_utf16;

// ============================================================================
// ASCII String Tests
// ============================================================================

#[test]
fn test_empty_string() {
    let line = "";
    assert_eq!(char_offset_to_utf16(line, 0), 0);
}

#[test]
fn test_empty_string_with_offset() {
    // Offset beyond empty string should return 0 (no chars to take)
    let line = "";
    assert_eq!(char_offset_to_utf16(line, 5), 0);
}

#[test]
fn test_ascii_zero_offset() {
    let line = "hello";
    assert_eq!(char_offset_to_utf16(line, 0), 0);
}

#[test]
fn test_ascii_single_char() {
    let line = "hello";
    // First character 'h' is 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
}

#[test]
fn test_ascii_multiple_chars() {
    let line = "hello world";
    // "hello" = 5 UTF-16 code units (each ASCII char is 1 unit)
    assert_eq!(char_offset_to_utf16(line, 5), 5);
    // "hello worl" = 10 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 10), 10);
}

#[test]
fn test_ascii_full_string() {
    let line = "hello";
    // All 5 characters = 5 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 5), 5);
}

#[test]
fn test_ascii_offset_beyond_length() {
    let line = "hello";
    // Offset 10 is beyond length 5, should return total UTF-16 length (5)
    assert_eq!(char_offset_to_utf16(line, 10), 5);
}

// ============================================================================
// Basic Unicode Character Tests (BMP - 1 UTF-16 code unit each)
// ============================================================================

#[test]
fn test_single_accented_char() {
    // "é" is in BMP, 1 UTF-16 code unit (U+00E9)
    let line = "café";
    // "caf" = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
    // "café" = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 4);
}

#[test]
fn test_euro_sign() {
    // "€" is in BMP, 1 UTF-16 code unit (U+20AC)
    let line = "€100";
    // "€" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "€1" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // "€100" = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 4);
}

#[test]
fn test_checkmark_symbol() {
    // "✓" is in BMP, 1 UTF-16 code unit (U+2713)
    let line = "Test: ✓ Done";
    // "Test: " = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 6), 6);
    // "Test: ✓" = 7 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 7), 7);
    // "Test: ✓ " = 8 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 8), 8);
}

#[test]
fn test_chinese_characters() {
    // Chinese characters are in BMP, each 1 UTF-16 code unit
    let line = "你好世界"; // "Hello World" in Chinese
    // "你" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "你好" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // "你好世" = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
    // "你好世界" = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 4);
}

#[test]
fn test_cyrillic_characters() {
    // Cyrillic characters are in BMP, each 1 UTF-16 code unit
    let line = "Привет"; // "Hello" in Russian
    // "П" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "Пр" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // Full string = 6 chars = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 6), 6);
}

#[test]
fn test_arabic_characters() {
    // Arabic characters are in BMP, each 1 UTF-16 code unit
    let line = "مرحبا"; // "Hello" in Arabic
    // "م" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "مر" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
}

// ============================================================================
// Emoji and Surrogate Pair Tests (2 UTF-16 code units each)
// ============================================================================

#[test]
fn test_single_emoji() {
    // "😀" is outside BMP, 2 UTF-16 code units (U+1F600)
    let line = "Hi 😀";
    // "Hi " = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
    // "Hi 😀" = 3 + 2 = 5 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 5);
}

#[test]
fn test_emoji_at_start() {
    let line = "😀hi";
    // "😀" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 1), 2);
    // "😀h" = 2 + 1 = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 3);
    // "😀hi" = 2 + 1 + 1 = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 4);
}

#[test]
fn test_multiple_emojis() {
    // Each emoji is 1 char but 2 UTF-16 code units
    let line = "😀😁😂";
    // "😀" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 1), 2);
    // "😀😁" = 2 + 2 = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 4);
    // "😀😁😂" = 2 + 2 + 2 = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 6);
}

#[test]
fn test_party_emojis() {
    let line = "🎉🎊🎈";
    // Each party emoji is 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 0), 0);
    assert_eq!(char_offset_to_utf16(line, 1), 2); // After first emoji
    assert_eq!(char_offset_to_utf16(line, 2), 4); // After second emoji
    assert_eq!(char_offset_to_utf16(line, 3), 6); // After third emoji
}

#[test]
fn test_emoji_with_text() {
    let line = "Hello 😀 World";
    // "Hello " = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 6), 6);
    // "Hello 😀" = 6 + 2 = 8 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 7), 8);
    // "Hello 😀 " = 8 + 1 = 9 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 8), 9);
}

#[test]
fn test_rocket_emoji() {
    // "🚀" is outside BMP, 2 UTF-16 code units (U+1F680)
    let line = "Test: ✓ 🚀 Done";
    // "Test: " = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 6), 6);
    // "Test: ✓" = 7 UTF-16 code units (✓ is 1 unit)
    assert_eq!(char_offset_to_utf16(line, 7), 7);
    // "Test: ✓ " = 8 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 8), 8);
    // "Test: ✓ 🚀" = 8 + 2 = 10 UTF-16 code units (🚀 is 2 units)
    assert_eq!(char_offset_to_utf16(line, 9), 10);
}

// ============================================================================
// Mixed Content Tests
// ============================================================================

#[test]
fn test_mixed_ascii_and_emoji() {
    let line = "abc😀def";
    // "abc" = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
    // "abc😀" = 3 + 2 = 5 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 5);
    // "abc😀d" = 5 + 1 = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 5), 6);
    // "abc😀def" = 5 + 3 = 8 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 7), 8);
}

#[test]
fn test_mixed_unicode_and_emoji() {
    // Mix of BMP (1 unit) and non-BMP (2 unit) characters
    let line = "a€é😀b";
    // "a" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "a€" = 1 + 1 = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // "a€é" = 1 + 1 + 1 = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
    // "a€é😀" = 3 + 2 = 5 UTF-16 code units (emoji is 2)
    assert_eq!(char_offset_to_utf16(line, 4), 5);
    // "a€é😀b" = 5 + 1 = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 5), 6);
}

#[test]
fn test_alternating_ascii_emoji() {
    let line = "a😀b😁c😂d";
    // "a" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "a😀" = 1 + 2 = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 3);
    // "a😀b" = 3 + 1 = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 4);
    // "a😀b😁" = 4 + 2 = 6 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 6);
    // "a😀b😁c" = 6 + 1 = 7 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 5), 7);
    // "a😀b😁c😂" = 7 + 2 = 9 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 6), 9);
    // "a😀b😁c😂d" = 9 + 1 = 10 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 7), 10);
}

// ============================================================================
// Edge Cases and Special Unicode
// ============================================================================

#[test]
fn test_whitespace_only() {
    let line = "     "; // 5 spaces
    // 3 spaces = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
    // All 5 spaces = 5 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 5), 5);
}

#[test]
fn test_tabs_and_newlines() {
    let line = "\t\n\r";
    // Tab = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // Tab + newline = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // Tab + newline + carriage return = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
}

#[test]
fn test_zero_width_joiner() {
    // Zero-width joiner (U+200D) is in BMP, 1 UTF-16 code unit
    let line = "a\u{200D}b";
    // "a" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "a\u{200D}" = 1 + 1 = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // "a\u{200D}b" = 1 + 1 + 1 = 3 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 3);
}

#[test]
fn test_combining_diacritics() {
    // Combining acute accent is in BMP, 1 UTF-16 code unit
    let line = "e\u{0301}"; // e + combining acute accent
    // "e" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "e\u{0301}" = 1 + 1 = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
}

#[test]
fn test_grapheme_cluster_emoji() {
    // Family emoji with zero-width joiners
    // 👨‍👩‍👧‍👦 = man(2) + ZWJ(1) + woman(2) + ZWJ(1) + girl(2) + ZWJ(1) + boy(2) = 11 UTF-16 units
    let line = "hi👨‍👩‍👧‍👦!";
    // "h" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "hi" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // Man emoji (👨) = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 3), 4);
    // ZWJ = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 4), 5);
}

#[test]
fn test_special_unicode_arrows() {
    // Arrow symbols are in BMP, each 1 UTF-16 code unit
    let line = "→←↑↓";
    // "→" = 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 1), 1);
    // "→←" = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 2), 2);
    // "→←↑↓" = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 4);
}

#[test]
fn test_musical_symbols() {
    // Musical symbols outside BMP, 2 UTF-16 code units each
    let line = "Music: 𝄞𝄢";
    // "Music: " = 7 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 7), 7);
    // "Music: 𝄞" = 7 + 2 = 9 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 8), 9);
    // "Music: 𝄞𝄢" = 9 + 2 = 11 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 9), 11);
}

// ============================================================================
// Real-world SysML/KerML Code Tests
// ============================================================================

#[test]
fn test_sysml_code_ascii() {
    let line = "part def Vehicle;";
    // "part" = 4 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 4), 4);
    // "part def" = 8 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 8), 8);
    // "part def Vehicle" = 16 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 16), 16);
}

#[test]
fn test_sysml_code_with_unicode_identifier() {
    // Valid identifiers can contain Unicode
    let line = "part def Véhicule;";
    // "part def V" = 10 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 10), 10);
    // "part def Vé" = 11 UTF-16 code units (é is 1 UTF-16 unit)
    assert_eq!(char_offset_to_utf16(line, 11), 11);
    // "part def Véhicule" = 17 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 17), 17);
}

#[test]
fn test_sysml_comment_with_emoji() {
    // Comments might contain emoji
    let line = "// TODO: Fix this 🐛";
    // "// TODO: " = 9 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 9), 9);
    // "// TODO: Fix this " = 18 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 18), 18);
    // "// TODO: Fix this 🐛" = 18 + 2 = 20 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 19), 20);
}

#[test]
fn test_import_statement_with_qualified_name() {
    let line = "import Package::SubPackage::Element;";
    // "import Package::" = 16 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 16), 16);
    // Full line (36 chars, all ASCII)
    assert_eq!(char_offset_to_utf16(line, 36), 36);
}

#[test]
fn test_attribute_with_greek_symbol() {
    // Greek letters are in BMP, 1 UTF-16 code unit each
    let line = "attribute α: Real;";
    // "attribute " = 10 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 10), 10);
    // "attribute α" = 11 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 11), 11);
}

// ============================================================================
// Performance and Large Input Tests
// ============================================================================

#[test]
fn test_long_ascii_string() {
    // Test with a longer ASCII string
    let line = "a".repeat(1000);
    // 100 chars = 100 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 100), 100);
    // 500 chars = 500 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 500), 500);
    // All 1000 chars = 1000 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 1000), 1000);
}

#[test]
fn test_long_emoji_string() {
    // Test with repeated emojis (each 2 UTF-16 code units)
    let line = "😀".repeat(100);
    // 10 emojis = 20 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 10), 20);
    // 50 emojis = 100 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 50), 100);
    // All 100 emojis = 200 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 100), 200);
}

#[test]
fn test_long_bmp_unicode_string() {
    // Test with BMP Unicode characters (each 1 UTF-16 code unit)
    let line = "€".repeat(100);
    // 10 Euro signs = 10 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 10), 10);
    // 50 Euro signs = 50 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 50), 50);
    // All 100 Euro signs = 100 UTF-16 code units
    assert_eq!(char_offset_to_utf16(&line, 100), 100);
}

// ============================================================================
// Consistency and Property Tests
// ============================================================================

#[test]
fn test_monotonic_increase() {
    // UTF-16 offset should always increase (or stay same) as char offset increases
    let line = "hello 世界 😀!";
    let mut prev_utf16 = 0;
    for char_offset in 0..=line.chars().count() {
        let utf16 = char_offset_to_utf16(line, char_offset);
        assert!(
            utf16 >= prev_utf16,
            "UTF-16 offset should increase monotonically: offset {char_offset}, utf16 {utf16} < prev {prev_utf16}"
        );
        prev_utf16 = utf16;
    }
}

#[test]
fn test_idempotent_at_length() {
    // Calling with offset >= length should always return same result
    let line = "test 😀";
    let len = line.chars().count();
    let expected = char_offset_to_utf16(line, len);

    assert_eq!(char_offset_to_utf16(line, len + 1), expected);
    assert_eq!(char_offset_to_utf16(line, len + 10), expected);
    assert_eq!(char_offset_to_utf16(line, len + 100), expected);
}

#[test]
fn test_ascii_char_equals_utf16() {
    // For ASCII, char offset should equal UTF-16 offset
    let line = "hello world";
    for i in 0..=line.len() {
        assert_eq!(char_offset_to_utf16(line, i), i as u32);
    }
}

#[test]
fn test_all_bmp_chars_equal_utf16() {
    // For all BMP characters, char count should equal UTF-16 code unit count
    let line = "café ✓ €100 你好";
    let char_count = line.chars().count();
    let utf16_count = char_offset_to_utf16(line, char_count);
    assert_eq!(char_count as u32, utf16_count);
}

#[test]
fn test_emoji_doubles_utf16_count() {
    // Each emoji adds 2 to UTF-16 count but only 1 to char count
    let emojis = "😀😁😂😃😄";
    let char_count = emojis.chars().count();
    let utf16_count = char_offset_to_utf16(emojis, char_count);
    // 5 emojis = 5 chars but 10 UTF-16 code units
    assert_eq!(char_count, 5);
    assert_eq!(utf16_count, 10);
}

// ============================================================================
// Boundary and Corner Cases
// ============================================================================

#[test]
fn test_offset_zero_always_returns_zero() {
    let test_cases = vec!["", "a", "😀", "hello", "你好", "a😀b"];
    for line in test_cases {
        assert_eq!(
            char_offset_to_utf16(line, 0),
            0,
            "Offset 0 should always return 0 for line: {line}"
        );
    }
}

#[test]
fn test_very_large_offset() {
    let line = "test";
    // Very large offset should clamp to string length
    assert_eq!(char_offset_to_utf16(line, usize::MAX), 4);
}

#[test]
fn test_single_emoji_full_offset() {
    let line = "😀";
    // Single emoji = 1 char = 2 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 1), 2);
}

#[test]
fn test_only_emojis() {
    let line = "😀😁😂😃😄😅😆😇😈😉";
    // 10 emojis = 20 UTF-16 code units
    assert_eq!(char_offset_to_utf16(line, 10), 20);
}

#[test]
fn test_newline_characters() {
    let line = "line1\nline2\r\n";
    // Each control character is 1 UTF-16 code unit
    assert_eq!(char_offset_to_utf16(line, 5), 5); // "line1"
    assert_eq!(char_offset_to_utf16(line, 6), 6); // "line1\n"
    assert_eq!(char_offset_to_utf16(line, 11), 11); // "line1\nline2"
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_consistency_with_string_encode_utf16() {
    // Verify our function matches Rust's built-in UTF-16 encoding
    let test_cases = vec!["hello", "café", "你好", "😀😁", "test 😀 123", "a€é😀b"];

    for line in test_cases {
        let char_count = line.chars().count();
        let our_result = char_offset_to_utf16(line, char_count);
        let rust_result = line.encode_utf16().count();
        assert_eq!(
            our_result as usize, rust_result,
            "Mismatch for line: {line}"
        );
    }
}

#[test]
fn test_partial_string_utf16_encoding() {
    let line = "Hello 😀 World 🚀!";

    // Test various offsets match Rust's UTF-16 encoding
    for char_offset in 0..=line.chars().count() {
        let our_result = char_offset_to_utf16(line, char_offset);
        let partial: String = line.chars().take(char_offset).collect();
        let rust_result = partial.encode_utf16().count();
        assert_eq!(
            our_result as usize, rust_result,
            "Mismatch at offset {char_offset} for line: {line}"
        );
    }
}
