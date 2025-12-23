use crate::core::{ParseError, ParseResult, get_extension, load_file, validate_extension};
use crate::syntax::kerml::ast::KerMLFile;
use ::from_pest::FromPest;
use pest::Parser;
use std::path::{Path, PathBuf};

/// Loads and parses a KerML file from disk.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The file has an invalid extension
/// - The file fails to parse
/// - AST construction fails
pub fn load_and_parse(path: &PathBuf) -> Result<KerMLFile, String> {
    validate_extension(path)?;
    let content = load_file(path)?;
    parse_content(&content, path)
}

/// Parses KerML content from a string (for LSP in-memory documents).
///
/// # Errors
///
/// Returns an error if:
/// - The content fails to parse
/// - AST construction fails
pub fn parse_content(content: &str, path: &Path) -> Result<KerMLFile, String> {
    let mut pairs = crate::parser::KerMLParser::parse(crate::parser::kerml::Rule::file, content)
        .map_err(|e| format!("Parse error in {}: {}", path.display(), e))?;

    KerMLFile::from_pest(&mut pairs)
        .map_err(|e| format!("AST error in {}: {:?}", path.display(), e))
}

/// Parses content and returns a ParseResult with detailed error information.
/// This is the primary function for LSP usage - errors don't fail, they're captured.
pub fn parse_with_result(content: &str, path: &Path) -> ParseResult<KerMLFile> {
    if let Err(e) = get_extension(path) {
        return ParseResult::with_errors(vec![e]);
    }

    match crate::parser::KerMLParser::parse(crate::parser::kerml::Rule::file, content) {
        Ok(mut pairs) => match KerMLFile::from_pest(&mut pairs) {
            Ok(file) => ParseResult::success(file),
            Err(e) => {
                let error = ParseError::ast_error(format!("{e:?}"), 0, 0);
                ParseResult::with_errors(vec![error])
            }
        },
        Err(parse_error) => {
            // Extract position from pest error
            let (line, col) = match parse_error.line_col {
                pest::error::LineColLocation::Pos((l, c)) => (l - 1, c - 1), // Convert to 0-indexed
                pest::error::LineColLocation::Span((l, c), _) => (l - 1, c - 1),
            };

            let error = ParseError::syntax_error(format!("{}", parse_error.variant), line, col);
            ParseResult::with_errors(vec![error])
        }
    }
}
