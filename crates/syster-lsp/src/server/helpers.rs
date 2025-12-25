use syster::semantic::symbol_table::Symbol;
use syster::syntax::SyntaxFile;
use tower_lsp::lsp_types::{Position, Range};

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
