use crate::core::{ParseError, ParseResult, get_extension, load_file, validate_extension};
use crate::syntax::sysml::ast::{SysMLFile, parse_file};
use pest::Parser;
use std::path::{Path, PathBuf};

/// Loads and parses a SysML file from disk.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The file has an invalid extension
/// - The file fails to parse
/// - AST construction fails
pub fn load_and_parse(path: &PathBuf) -> Result<SysMLFile, String> {
    validate_extension(path)?;
    let content = load_file(path)?;
    parse_content(&content, path)
}

/// Parses SysML content from a string (for LSP in-memory documents).
///
/// # Errors
///
/// Returns an error if:
/// - The content fails to parse
/// - AST construction fails
pub fn parse_content(content: &str, path: &Path) -> Result<SysMLFile, String> {
    let mut pairs = crate::parser::SysMLParser::parse(crate::parser::sysml::Rule::model, content)
        .map_err(|e| format!("Parse error in {}: {}", path.display(), e))?;

    parse_file(&mut pairs).map_err(|e| format!("AST error in {}: {:?}", path.display(), e))
}

/// Parses content and returns a ParseResult with detailed error information.
/// This is the primary function for LSP usage - errors don't fail, they're captured.
///
/// If full parse fails, we parse valid definitions individually to enable LSP features
/// even with incomplete/invalid code present.
pub fn parse_with_result(content: &str, path: &Path) -> ParseResult<SysMLFile> {
    if let Err(e) = get_extension(path) {
        return ParseResult::with_errors(vec![e]);
    }

    // Try full parse first (fastest for valid files)
    match crate::parser::SysMLParser::parse(crate::parser::sysml::Rule::model, content) {
        Ok(mut pairs) => match parse_file(&mut pairs) {
            Ok(file) => ParseResult::success(file),
            Err(e) => {
                let error = ParseError::ast_error(format!("{e:?}"), 0, 0);
                ParseResult::with_errors(vec![error])
            }
        },
        Err(parse_error) => {
            // Extract position from pest error
            let (line, col) = match parse_error.line_col {
                pest::error::LineColLocation::Pos((l, c)) => (l - 1, c - 1),
                pest::error::LineColLocation::Span((l, c), _) => (l - 1, c - 1),
            };

            let error = ParseError::syntax_error(format!("{}", parse_error.variant), line, col);

            // Try to recover valid definitions
            match parse_valid_definitions(content) {
                Some(partial_file) => ParseResult {
                    content: Some(partial_file),
                    errors: vec![error],
                },
                None => ParseResult::with_errors(vec![error]),
            }
        }
    }
}

/// Parse individual valid definitions from content, skipping invalid lines.
/// This allows LSP to work with incomplete code.
fn parse_valid_definitions(content: &str) -> Option<SysMLFile> {
    use crate::syntax::sysml::ast::SysMLFile;

    let mut elements = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") || !trimmed.ends_with(';') {
            continue;
        }

        if let Some(element) = try_parse_as_element(trimmed) {
            elements.push(element);
        }
    }

    if elements.is_empty() {
        None
    } else {
        Some(SysMLFile {
            namespace: None,
            namespaces: Vec::new(),
            elements,
        })
    }
}

fn try_parse_as_element(line: &str) -> Option<crate::syntax::sysml::ast::Element> {
    use crate::parser::{SysMLParser, sysml::Rule};
    use crate::syntax::sysml::ast::{Element, parse_definition, parse_usage};

    // Try as definition (any kind)
    if line.contains(" def ") {
        // Try all *_definition rules
        for rule in [
            Rule::part_definition,
            Rule::attribute_definition,
            Rule::action_definition,
            Rule::item_definition,
            Rule::port_definition,
            Rule::connection_definition,
            Rule::interface_definition,
            Rule::allocation_definition,
            Rule::state_definition,
            Rule::requirement_definition,
            Rule::enumeration_definition,
            Rule::occurrence_definition,
            Rule::calculation_definition,
        ] {
            if let Ok(mut pairs) = SysMLParser::parse(rule, line)
                && let Some(pair) = pairs.next()
                && let Ok(def) = parse_definition(pair)
            {
                return Some(Element::Definition(def));
            }
        }
    }

    // Try as usage (any kind)
    if !line.contains(" def ") {
        for rule in [
            Rule::part_usage,
            Rule::attribute_usage,
            Rule::item_usage,
            Rule::port_usage,
            Rule::action_usage,
        ] {
            if let Ok(mut pairs) = SysMLParser::parse(rule, line)
                && let Some(pair) = pairs.next()
            {
                return Some(Element::Usage(parse_usage(pair)));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests;
