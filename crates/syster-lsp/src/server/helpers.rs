use async_lsp::lsp_types::{Location, Position, Range, Url};
use std::path::PathBuf;
use syster::semantic::resolver::Resolver;
use syster::semantic::symbol_table::Symbol;
use syster::syntax::SyntaxFile;

use syster::semantic::Workspace;

/// Convert a URI to a PathBuf, returning None if the conversion fails
pub fn uri_to_path(uri: &Url) -> Option<PathBuf> {
    uri.to_file_path().ok()
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
pub fn position_to_byte_offset(text: &str, pos: Position) -> Result<usize, String> {
    let lines: Vec<&str> = text.lines().collect();
    let line_idx = pos.line as usize;
    let char_offset = pos.character as usize;

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

    let mut byte_offset = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == line_idx {
            break;
        }
        byte_offset += line.len() + 1;
    }

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

/// Collect all reference locations for a symbol (from reference index + imports)
///
/// This is the shared implementation used by both get_references and get_rename_edits.
pub fn collect_reference_locations(
    workspace: &Workspace<SyntaxFile>,
    qualified_name: &str,
) -> Vec<Location> {
    use tracing::debug;

    let mut locations = Vec::new();

    // Query reference index for references with their actual spans
    // References are stored with fully resolved qualified names during population,
    // so we just need to look up by the qualified name directly
    let mut refs = workspace.reference_index().get_references(qualified_name);

    // Also try simple name if different from qualified name
    // This handles cases where the source uses just the simple name
    if let Some(simple_name) = qualified_name.rsplit("::").next()
        && simple_name != qualified_name
    {
        let simple_refs = workspace.reference_index().get_references(simple_name);
        for r in simple_refs {
            if !refs
                .iter()
                .any(|existing| existing.file == r.file && existing.span == r.span)
            {
                refs.push(r);
            }
        }
    }

    debug!("[COLLECT_REFS] refs count={}", refs.len());

    // Convert references to LSP locations using the stored spans
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

    // Outgoing relationships - get from forward index
    // Show specializations for definitions
    if matches!(
        symbol,
        Symbol::Definition { .. } | Symbol::Classifier { .. }
    ) {
        let targets = workspace
            .reference_index()
            .get_targets(symbol.qualified_name());
        if !targets.is_empty() {
            let resolver = Resolver::new(workspace.symbol_table());
            result.push_str("\n**Specializes:**\n");
            for target in targets {
                if let Some(target_symbol) = resolver.resolve(target)
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

    // Show usage_type for usages
    if let Symbol::Usage {
        usage_type: Some(typed_by),
        ..
    } = symbol
    {
        let resolver = Resolver::new(workspace.symbol_table());
        result.push_str("\n**Typed by:**\n");
        if let Some(target_symbol) = resolver.resolve(typed_by)
            && let Some(target_file) = target_symbol.source_file()
            && let Ok(target_uri) = Url::from_file_path(target_file)
        {
            if let Some(target_span) = target_symbol.span() {
                let line = target_span.start.line + 1;
                result.push_str(&format!("- [{typed_by}]({target_uri}#L{line})\n"));
            } else {
                result.push_str(&format!("- [{typed_by}]({target_uri})\n"));
            }
        } else {
            result.push_str(&format!("- `{typed_by}`\n"));
        }
    }

    // Incoming references (use Shift+F12 to see all)
    let mut references: Vec<Location> =
        collect_reference_locations(workspace, symbol.qualified_name());
    // Sort by file path, then line, then column for deterministic ordering
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
            let line = loc.range.start.line + 1;
            let col = loc.range.start.character + 1;
            result.push_str(&format!(
                "- [{file_name}:{line}:{col}]({}#L{line})\n",
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
