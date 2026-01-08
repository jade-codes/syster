//! Text manipulation utilities for working with source code.

/// Check if a character is considered part of a word (identifier).
///
/// Uses Unicode Standard Annex #31 rules for identifier characters.
/// This matches the behavior expected by most programming languages including SysML.
#[inline]
pub fn is_word_character(c: char) -> bool {
    unicode_ident::is_xid_continue(c)
}

/// Find the boundaries of a word at the given position.
///
/// Returns `Some((start, end))` where `start` is the character index of the word start
/// and `end` is the character index after the last word character.
/// Returns `None` if there is no word at the position.
pub fn find_word_boundaries(chars: &[char], position: usize) -> Option<(usize, usize)> {
    if position >= chars.len() {
        return None;
    }

    // Check if we're on a word character
    if !is_word_character(chars[position]) {
        return None;
    }

    // Find start of word
    let mut start = position;
    while start > 0 && is_word_character(chars[start - 1]) {
        start -= 1;
    }

    // Find end of word
    let mut end = position;
    while end < chars.len() && is_word_character(chars[end]) {
        end += 1;
    }

    Some((start, end))
}

/// Extract the word (identifier) at the cursor position in a line of text.
///
/// Returns the word as a `String`, or `None` if there is no word at the position.
///
/// # Example
/// ```
/// use syster::core::text_utils::extract_word_at_cursor;
///
/// let line = "let foo = bar";
/// assert_eq!(extract_word_at_cursor(line, 4), Some("foo".to_string()));
/// assert_eq!(extract_word_at_cursor(line, 10), Some("bar".to_string()));
/// assert_eq!(extract_word_at_cursor(line, 8), None); // space
/// ```
pub fn extract_word_at_cursor(line: &str, position: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();

    if position >= chars.len() {
        return None;
    }

    let (start, end) = find_word_boundaries(&chars, position)?;

    Some(chars[start..end].iter().collect())
}

/// Check if a character is part of a qualified name (identifier or `:`).
#[inline]
fn is_qualified_name_character(c: char) -> bool {
    is_word_character(c) || c == ':'
}

/// Extract the qualified name at the cursor position in a line of text.
///
/// This extracts names that may contain `::` separators, like `Package::Type`.
/// Returns `None` if there is no qualified name at the position.
///
/// # Example
/// ```
/// use syster::core::text_utils::extract_qualified_name_at_cursor;
///
/// let line = "import ISQ::MassValue;";
/// assert_eq!(extract_qualified_name_at_cursor(line, 7), Some("ISQ::MassValue".to_string()));
/// assert_eq!(extract_qualified_name_at_cursor(line, 12), Some("ISQ::MassValue".to_string()));
/// ```
pub fn extract_qualified_name_at_cursor(line: &str, position: usize) -> Option<String> {
    let chars: Vec<char> = line.chars().collect();

    if position >= chars.len() {
        return None;
    }

    // Check if we're on a qualified name character
    if !is_qualified_name_character(chars[position]) {
        return None;
    }

    // Find start of qualified name
    let mut start = position;
    while start > 0 && is_qualified_name_character(chars[start - 1]) {
        start -= 1;
    }

    // Find end of qualified name
    let mut end = position;
    while end < chars.len() && is_qualified_name_character(chars[end]) {
        end += 1;
    }

    let result: String = chars[start..end].iter().collect();

    // Clean up: trim leading/trailing colons and ensure valid structure
    let trimmed = result.trim_matches(':');
    if trimmed.is_empty() {
        return None;
    }

    Some(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_word_character() {
        assert!(is_word_character('a'));
        assert!(is_word_character('Z'));
        assert!(is_word_character('0'));
        assert!(is_word_character('_'));
        assert!(!is_word_character(' '));
        assert!(!is_word_character('.'));
        assert!(!is_word_character(':'));
    }

    #[test]
    fn test_find_word_boundaries() {
        let text = "foo bar_baz";
        let chars: Vec<char> = text.chars().collect();

        // Position in "foo"
        assert_eq!(find_word_boundaries(&chars, 0), Some((0, 3)));
        assert_eq!(find_word_boundaries(&chars, 1), Some((0, 3)));
        assert_eq!(find_word_boundaries(&chars, 2), Some((0, 3)));

        // Position in space
        assert_eq!(find_word_boundaries(&chars, 3), None);

        // Position in "bar_baz"
        assert_eq!(find_word_boundaries(&chars, 4), Some((4, 11)));
        assert_eq!(find_word_boundaries(&chars, 7), Some((4, 11)));
        assert_eq!(find_word_boundaries(&chars, 10), Some((4, 11)));
    }

    #[test]
    fn test_extract_word_at_cursor() {
        let line = "let foo = bar";

        assert_eq!(extract_word_at_cursor(line, 0), Some("let".to_string()));
        assert_eq!(extract_word_at_cursor(line, 4), Some("foo".to_string()));
        assert_eq!(extract_word_at_cursor(line, 5), Some("foo".to_string()));
        assert_eq!(extract_word_at_cursor(line, 10), Some("bar".to_string()));

        // Spaces and special chars
        assert_eq!(extract_word_at_cursor(line, 3), None);
        assert_eq!(extract_word_at_cursor(line, 8), None);
    }

    #[test]
    fn test_extract_word_out_of_bounds() {
        let line = "foo";
        assert_eq!(extract_word_at_cursor(line, 100), None);
    }

    #[test]
    fn test_extract_word_empty_line() {
        assert_eq!(extract_word_at_cursor("", 0), None);
    }

    #[test]
    fn test_unicode_identifiers() {
        // Unicode identifiers should work
        let line = "let café = αβγ";
        assert_eq!(extract_word_at_cursor(line, 4), Some("café".to_string()));
        assert_eq!(extract_word_at_cursor(line, 11), Some("αβγ".to_string()));

        // Mixed ASCII and Unicode
        let line2 = "foo_bar café_shop";
        assert_eq!(
            extract_word_at_cursor(line2, 0),
            Some("foo_bar".to_string())
        );
        assert_eq!(
            extract_word_at_cursor(line2, 8),
            Some("café_shop".to_string())
        );
    }

    #[test]
    fn test_extract_qualified_name_at_cursor() {
        let line = "import ISQ::MassValue;";
        // Hovering over "ISQ"
        assert_eq!(
            extract_qualified_name_at_cursor(line, 7),
            Some("ISQ::MassValue".to_string())
        );
        // Hovering over "MassValue"
        assert_eq!(
            extract_qualified_name_at_cursor(line, 12),
            Some("ISQ::MassValue".to_string())
        );
        // Hovering over "::"
        assert_eq!(
            extract_qualified_name_at_cursor(line, 10),
            Some("ISQ::MassValue".to_string())
        );
    }

    #[test]
    fn test_extract_qualified_name_nested() {
        let line = "import A::B::C;";
        assert_eq!(
            extract_qualified_name_at_cursor(line, 7),
            Some("A::B::C".to_string())
        );
        assert_eq!(
            extract_qualified_name_at_cursor(line, 10),
            Some("A::B::C".to_string())
        );
        assert_eq!(
            extract_qualified_name_at_cursor(line, 12),
            Some("A::B::C".to_string())
        );
    }

    #[test]
    fn test_extract_qualified_name_simple() {
        // Simple name without ::
        let line = "part def Vehicle;";
        assert_eq!(
            extract_qualified_name_at_cursor(line, 9),
            Some("Vehicle".to_string())
        );
    }

    #[test]
    fn test_extract_qualified_name_with_tab() {
        // Line with tab at start (like in ConstraintTest.sysml)
        let line = "\tprivate import ISQ::MassValue;";
        // Position 17 is on "ISQ" (after tab + "private import ")
        assert_eq!(
            extract_qualified_name_at_cursor(line, 17),
            Some("ISQ::MassValue".to_string())
        );
        // Position 22 is on "MassValue"
        assert_eq!(
            extract_qualified_name_at_cursor(line, 22),
            Some("ISQ::MassValue".to_string())
        );
    }
}
