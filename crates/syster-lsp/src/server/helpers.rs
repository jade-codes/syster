use async_lsp::lsp_types::{Location, Position, Range, Url};
use percent_encoding::percent_decode_str;
use std::path::PathBuf;
use syster::semantic::resolver::Resolver;
use syster::semantic::symbol_table::Symbol;
use syster::syntax::SyntaxFile;

use syster::semantic::Workspace;

/// Convert a URI to a PathBuf, returning None if the conversion fails
pub fn uri_to_path(uri: &Url) -> Option<PathBuf> {
    uri.to_file_path().ok()
}

/// Decode percent-encoded strings (e.g., "my%20file.txt" -> "my file.txt")
///
/// Used to display file names to users with proper formatting instead of URL encoding.
/// Handles invalid encoding gracefully by returning the original string.
pub fn decode_uri_component(s: &str) -> String {
    percent_decode_str(s)
        .decode_utf8()
        .map(|cow| cow.into_owned())
        .unwrap_or_else(|_| s.to_string())
}

/// Convert a character offset in a line to UTF-16 code units
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
    let line_idx = pos.line as usize;
    let char_offset = pos.character as usize;

    // Split by \n to handle both LF and CRLF (since \r\n split on \n leaves \r at line end)
    let lines: Vec<&str> = text.split('\n').collect();

    if line_idx > lines.len() {
        return Err(format!(
            "Line {} out of bounds (total lines: {})",
            line_idx,
            lines.len()
        ));
    }

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
pub fn apply_text_edit(text: &str, range: &Range, new_text: &str) -> Result<String, String> {
    let start_byte = position_to_byte_offset(text, range.start)?;
    let end_byte = position_to_byte_offset(text, range.end)?;

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
    use tracing::debug;

    let mut locations = Vec::new();

    // Query reference index by qualified name
    let refs = workspace.reference_index().get_references(qualified_name);

    debug!("[COLLECT_REFS] reference_index refs count={}", refs.len());

    for ref_info in refs {
        if let Ok(uri) = Url::from_file_path(&ref_info.file) {
            locations.push(Location {
                uri,
                range: span_to_lsp_range(&ref_info.span),
            });
        }
    }

    // Add import references by iterating all imports (computed on demand)
    let symbol_table = workspace.symbol_table();
    let mut import_count = 0;
    for scope_id in 0..symbol_table.scope_count() {
        for import in symbol_table.get_scope_imports(scope_id) {
            // Skip wildcard imports - they don't reference a specific symbol
            if import.path.ends_with("::*") || import.path.ends_with("::**") {
                continue;
            }

            // Check if this import references our target
            if import.path == qualified_name
                && let (Some(span), Some(file)) = (import.span, &import.file)
                && let Ok(uri) = Url::from_file_path(file)
            {
                locations.push(Location {
                    uri,
                    range: span_to_lsp_range(&span),
                });
                import_count += 1;
            }
        }
    }

    debug!("[COLLECT_REFS] import refs count={}", import_count);
    debug!("[COLLECT_REFS] total locations={}", locations.len());
    locations
}

/// Format rich hover information with relationships and documentation
pub fn format_rich_hover(
    symbol: &Symbol,
    workspace: &syster::semantic::Workspace<SyntaxFile>,
) -> String {
    use tracing::debug;

    debug!(
        "[FORMAT_HOVER] symbol={}, qname={}",
        symbol.name(),
        symbol.qualified_name()
    );

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

    // Source file with clickable link that jumps to definition
    if let Some(file) = symbol.source_file() {
        debug!("[FORMAT_HOVER] source_file={}", file);
        if let Ok(uri) = Url::from_file_path(file) {
            let file_name = std::path::Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(file);

            if let Some(span) = symbol.span() {
                let line = span.start.line + 1;
                let col = span.start.column + 1;
                result.push_str(&format!(
                    "\n**Defined in:** [{file_name}:{line}:{col}]({uri}#L{line})\n"
                ));
            } else {
                result.push_str(&format!("\n**Defined in:** [{file_name}]({uri})\n"));
            }
        } else {
            result.push_str(&format!("\n**Defined in:** `{file}`\n"));
        }
    }

    // Outgoing relationships (what this symbol references)
    let relationships = get_symbol_relationships(symbol, workspace);
    debug!(
        "[FORMAT_HOVER] outgoing relationships count={}",
        relationships.len()
    );
    for (rel_type, targets) in &relationships {
        debug!(
            "[FORMAT_HOVER]   rel_type={}, targets={:?}",
            rel_type, targets
        );
    }
    if !relationships.is_empty() {
        use syster::core::constants::relationship_label;
        let resolver = Resolver::new(workspace.symbol_table());
        for (rel_type, targets) in relationships {
            let label = relationship_label(&rel_type);
            result.push_str(&format!("\n**{label}:**\n"));
            for target in targets {
                // Try to make targets clickable too
                if let Some(target_symbol) = resolver.resolve(&target)
                    && let Some(target_file) = target_symbol.source_file()
                    && let Ok(target_uri) = Url::from_file_path(target_file)
                {
                    if let Some(target_span) = target_symbol.span() {
                        let line = target_span.start.line + 1;
                        result.push_str(&format!("- [{target}]({target_uri}#L{line})\n"));
                    } else {
                        result.push_str(&format!("- [{target}]({target_uri})\n"));
                    }
                } else {
                    result.push_str(&format!("- `{target}`\n"));
                }
            }
        }
    }

    // Note: Typing for Usage symbols is now handled in get_symbol_relationships()
    // and displayed via the relationships loop above, so no duplicate handling needed here.

    // Incoming references (use Shift+F12 to see all)
    // Reuse the shared collect_reference_locations to include both relationship and import refs
    let mut references: Vec<Location> =
        collect_reference_locations(workspace, symbol.qualified_name());
    // Sort for deterministic output (by file path, then line, then column)
    references.sort_by(|a, b| {
        a.uri
            .as_str()
            .cmp(b.uri.as_str())
            .then(a.range.start.line.cmp(&b.range.start.line))
            .then(a.range.start.character.cmp(&b.range.start.character))
    });
    if !references.is_empty() {
        let count = references.len();
        let plural = if count == 1 { "" } else { "s" };
        result.push_str(&format!("\n**Referenced by:** ({count} usage{plural})\n"));
        for loc in &references {
            let file_name = loc
                .uri
                .path_segments()
                .and_then(|mut s| s.next_back())
                .unwrap_or("unknown");
            let decoded_file_name = decode_uri_component(file_name);
            let line = loc.range.start.line + 1;
            let col = loc.range.start.character + 1;
            result.push_str(&format!(
                "- [{decoded_file_name}:{line}:{col}]({}#L{line})\n",
                loc.uri
            ));
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

/// Get relationships for a symbol from the workspace's reference index.
///
/// Uses the forward index to find what this symbol references (specializations)
/// and also extracts typing from Usage symbols.
fn get_symbol_relationships(
    symbol: &Symbol,
    workspace: &syster::semantic::Workspace<SyntaxFile>,
) -> Vec<(String, Vec<String>)> {
    let mut relationships = Vec::new();

    // For Usage symbols, extract typing relationship from usage_type field
    if let Symbol::Usage {
        usage_type: Some(type_name),
        ..
    } = symbol
    {
        relationships.push(("Typed by".to_string(), vec![type_name.clone()]));
    }

    // Get specializations from the reference index
    let qname = symbol.qualified_name();
    let index = workspace.reference_index();
    let targets: Vec<String> = index
        .get_targets(qname)
        .into_iter()
        .map(String::from)
        .collect();

    if !targets.is_empty() {
        relationships.push(("Specializes".to_string(), targets));
    }

    relationships
}
