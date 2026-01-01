use async_lsp::lsp_types::{Location, Position, Range, Url};
use std::path::PathBuf;
use syster::semantic::symbol_table::Symbol;
use syster::syntax::SyntaxFile;

use syster::semantic::Workspace;

/// Convert a URI to a PathBuf, returning None if the conversion fails
///
/// This is the standard pattern for handling file URIs in LSP handlers.
/// Use this when the handler should return None/empty on invalid URIs.
pub fn uri_to_path(uri: &Url) -> Option<PathBuf> {
    uri.to_file_path().ok()
}

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

/// Collect all reference locations for a symbol (relationship refs + import refs)
///
/// This is the shared implementation used by both get_references and get_rename_edits.
pub fn collect_reference_locations(
    workspace: &Workspace<SyntaxFile>,
    qualified_name: &str,
) -> Vec<Location> {
    let mut locations = Vec::new();

    // Add relationship references (typing, specialization, etc.)
    let refs = workspace
        .relationship_graph()
        .get_references_to(qualified_name);

    for (source_qname, span) in refs {
        let Some(reference_span) = span else {
            continue; // Skip references without precise spans
        };

        if let Some(source_symbol) = workspace.symbol_table().lookup_qualified(source_qname)
            && let Some(file) = source_symbol.source_file()
            && let Ok(uri) = Url::from_file_path(file)
        {
            locations.push(Location {
                uri,
                range: span_to_lsp_range(reference_span),
            });
        }
    }

    // Add import references
    let import_refs = workspace
        .symbol_table()
        .get_import_references(qualified_name);

    for (file, span) in import_refs {
        if let Ok(uri) = Url::from_file_path(file) {
            locations.push(Location {
                uri,
                range: span_to_lsp_range(span),
            });
        }
    }

    locations
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

    // Relationships (using relationship graph) - grouped by type
    let relationships = get_symbol_relationships(symbol, workspace);
    if !relationships.is_empty() {
        for (rel_type, targets) in relationships {
            result.push_str(&format!("\n**{rel_type}:**\n"));
            for target in targets {
                result.push_str(&format!("- `{target}`\n"));
            }
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
        Symbol::Import { path, .. } => format!("import {path}"),
    }
}

/// Get relationships for a symbol from the workspace, grouped by relationship type
fn get_symbol_relationships(
    symbol: &Symbol,
    workspace: &syster::semantic::Workspace<SyntaxFile>,
) -> Vec<(String, Vec<String>)> {
    let qname = symbol.qualified_name();
    let graph = workspace.relationship_graph();

    graph.get_relationships_grouped(qname)
}
