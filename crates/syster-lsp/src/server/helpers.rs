use async_lsp::lsp_types::{Position, Range};
use syster::semantic::symbol_table::Symbol;
use syster::syntax::SyntaxFile;

/// Convert a character offset in a line to UTF-16 code units
///
/// LSP uses UTF-16 code units for positions, so we need to convert from character offsets
pub fn char_offset_to_utf16(line: &str, char_offset: usize) -> u32 {
    line.chars()
        .take(char_offset)
        .map(|c| c.len_utf16())
        .sum::<usize>() as u32
}

/// Convert character offset to byte offset within a line
pub fn char_offset_to_byte(line: &str, char_offset: usize) -> usize {
    line.chars().take(char_offset).map(|c| c.len_utf8()).sum()
}

/// Convert LSP Position to byte offset in text
///
/// Handles multi-line documents by calculating line offsets and character positions
/// Note: Treats position.character as character count (not strict UTF-16 code units)
pub fn position_to_byte_offset(text: &str, pos: Position) -> Result<usize, String> {
    let lines: Vec<&str> = text.lines().collect();
    let line_idx = pos.line as usize;
    let char_offset = pos.character as usize;

    // Allow line == lines.len() for end-of-document positions
    if line_idx > lines.len() {
        return Err(format!(
            "Line {} out of bounds (total lines: {})",
            line_idx,
            lines.len()
        ));
    }

    // If at end of document (past last line), return total byte length
    if line_idx == lines.len() {
        return Ok(text.len());
    }

    // Calculate byte offset up to the start of the target line
    let mut byte_offset = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == line_idx {
            break;
        }
        byte_offset += line.len() + 1; // +1 for newline
    }

    // Add character offset within the line converted to bytes
    let line = lines[line_idx];
    let line_byte_offset = char_offset_to_byte(line, char_offset);

    Ok(byte_offset + line_byte_offset)
}

/// Apply a text edit to a string based on LSP range
///
/// Converts LSP Position (line, character) to byte offset and performs the edit
pub fn apply_text_edit(text: &str, range: &Range, new_text: &str) -> Result<String, String> {
    // Convert start and end positions to byte offsets
    let start_byte = position_to_byte_offset(text, range.start)?;
    let end_byte = position_to_byte_offset(text, range.end)?;

    // Validate range
    if start_byte > end_byte {
        return Err(format!(
            "Invalid range: start ({start_byte}) > end ({end_byte})"
        ));
    }

    if end_byte > text.len() {
        return Err(format!(
            "Range end ({}) exceeds text length ({})",
            end_byte,
            text.len()
        ));
    }

    // Build new text: prefix + new_text + suffix
    let mut result = String::with_capacity(text.len() + new_text.len());
    result.push_str(&text[..start_byte]);
    result.push_str(new_text);
    result.push_str(&text[end_byte..]);

    Ok(result)
}

/// Convert our Span to LSP Range
pub fn span_to_lsp_range(span: &syster::core::Span) -> Range {
    Range {
        start: Position {
            line: span.start.line as u32,
            character: span.start.column as u32,
        },
        end: Position {
            line: span.end.line as u32,
            character: span.end.column as u32,
        },
    }
}

/// Convert our Position to LSP Position
pub fn position_to_lsp_position(pos: &syster::core::Position) -> Position {
    Position {
        line: pos.line as u32,
        character: pos.column as u32,
    }
}

/// Convert our Span to LSP FoldingRange
pub fn span_to_folding_range(
    span: &syster::core::Span,
    kind: async_lsp::lsp_types::FoldingRangeKind,
) -> async_lsp::lsp_types::FoldingRange {
    async_lsp::lsp_types::FoldingRange {
        start_line: span.start.line as u32,
        start_character: Some(span.start.column as u32),
        end_line: span.end.line as u32,
        end_character: Some(span.end.column as u32),
        kind: Some(kind),
        collapsed_text: None,
    }
}

/// Format rich hover information with relationships and documentation
pub fn format_rich_hover(
    symbol: &Symbol,
    workspace: &syster::semantic::Workspace<SyntaxFile>,
) -> String {
    let mut result = String::new();

    // Main declaration
    result.push_str("```sysml\n");
    result.push_str(&format_symbol_declaration(symbol));
    result.push_str("\n```\n");

    // Qualified name
    result.push_str(&format!(
        "\n**Qualified Name:** `{}`\n",
        symbol.qualified_name()
    ));

    // Source file
    if let Some(file) = symbol.source_file() {
        result.push_str(&format!("\n**Defined in:** `{file}`\n"));
    }

    // Relationships (using relationship graph)
    if let Some(relationships) = get_symbol_relationships(symbol, workspace)
        && !relationships.is_empty()
    {
        result.push_str("\n**Relationships:**\n");
        for rel in relationships {
            result.push_str(&format!("- {rel}\n"));
        }
    }

    result
}

/// Format the symbol declaration
fn format_symbol_declaration(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Alias { name, target, .. } => format!("alias {name} for {target}"),
        Symbol::Package { name, .. } => format!("package {name}"),
        Symbol::Classifier { name, .. } => format!("classifier {name}"),
        Symbol::Definition { name, kind, .. } => format!("{kind} def {name}"),
        Symbol::Usage { name, kind, .. } => format!("{kind} {name}"),
        Symbol::Feature {
            name, feature_type, ..
        } => {
            let type_str = feature_type
                .as_ref()
                .map(|t| format!(": {t}"))
                .unwrap_or_default();
            format!("feature {name}{type_str}")
        }
    }
}

/// Get relationships for a symbol from the workspace
fn get_symbol_relationships(
    symbol: &Symbol,
    workspace: &syster::semantic::Workspace<SyntaxFile>,
) -> Option<Vec<String>> {
    let qname = symbol.qualified_name();
    let graph = workspace.relationship_graph();

    let relationships = graph.get_formatted_relationships(qname);

    if relationships.is_empty() {
        None
    } else {
        Some(relationships)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
