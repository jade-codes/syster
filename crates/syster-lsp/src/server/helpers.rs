use syster::semantic::symbol_table::Symbol;
use tower_lsp::lsp_types::{Position, Range};

/// Extract the word at the cursor position from the document text
pub fn extract_word_at_cursor(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;

    syster::core::text_utils::extract_word_at_cursor(line, position.character as usize)
}

/// Find an element at the given position in the AST
pub fn find_element_at_position(
    element: &syster::syntax::sysml::ast::Element,
    position: syster::core::Position,
) -> Option<(String, syster::core::Span)> {
    use syster::syntax::sysml::ast::Element;

    match element {
        Element::Package(pkg) => {
            // First, check nested elements (most specific match)
            for child in &pkg.elements {
                if let Some(result) = find_element_at_position(child, position) {
                    return Some(result);
                }
            }
            // If no nested element matched, check if position is in package itself
            check_element_match(&pkg.name, pkg.span, position)
        }
        Element::Definition(def) => check_element_match(&def.name, def.span, position),
        Element::Usage(usage) => check_element_match(&usage.name, usage.span, position),
        Element::Alias(alias) => check_element_match(&alias.name, alias.span, position),
        _ => None,
    }
}

/// Helper to check if a position matches an element's name and span
fn check_element_match(
    name: &Option<String>,
    span: Option<syster::core::Span>,
    position: syster::core::Position,
) -> Option<(String, syster::core::Span)> {
    if let (Some(name), Some(span)) = (name, span)
        && span.contains(position)
    {
        Some((name.clone(), span))
    } else {
        None
    }
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
pub fn format_rich_hover(symbol: &Symbol, workspace: &syster::semantic::Workspace) -> String {
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
        result.push_str(&format!("\n**Defined in:** `{}`\n", file));
    }

    // Relationships (using relationship graph)
    if let Some(relationships) = get_symbol_relationships(symbol, workspace)
        && !relationships.is_empty()
    {
        result.push_str("\n**Relationships:**\n");
        for rel in relationships {
            result.push_str(&format!("- {}\n", rel));
        }
    }

    result
}

/// Format the symbol declaration
fn format_symbol_declaration(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Alias { name, target, .. } => format!("alias {} for {}", name, target),
        Symbol::Package { name, .. } => format!("package {}", name),
        Symbol::Classifier { name, .. } => format!("classifier {}", name),
        Symbol::Definition { name, kind, .. } => format!("{} def {}", kind, name),
        Symbol::Usage { name, kind, .. } => format!("{} {}", kind, name),
        Symbol::Feature {
            name, feature_type, ..
        } => {
            let type_str = feature_type
                .as_ref()
                .map(|t| format!(": {}", t))
                .unwrap_or_default();
            format!("feature {}{}", name, type_str)
        }
    }
}

/// Get relationships for a symbol from the workspace
fn get_symbol_relationships(
    symbol: &Symbol,
    workspace: &syster::semantic::Workspace,
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
