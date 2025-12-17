use crate::core::constants::{KERML_EXT, SYSML_EXT};
use crate::core::{ParseError, ParseResult, get_extension, load_file, validate_extension};
use crate::syntax::SyntaxFile;
use crate::syntax::kerml::KerMLFile;
use std::path::{Path, PathBuf};

/// Loads and parses a language file (SysML or KerML) based on file extension.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The file has an invalid extension
/// - The file fails to parse
pub fn load_and_parse(path: &PathBuf) -> Result<SyntaxFile, String> {
    let ext = validate_extension(path)?;
    let content = load_file(path)?;

    match ext {
        SYSML_EXT => {
            let file = crate::syntax::sysml::parser::parse_content(&content, path)?;
            Ok(SyntaxFile::SysML(file))
        }
        KERML_EXT => {
            // TODO: Implement KerML parser - currently returns empty placeholder
            Ok(SyntaxFile::KerML(KerMLFile::empty()))
        }
        _ => unreachable!("validate_extension should have caught this"),
    }
}

/// Parses language content from a string based on file extension.
///
/// # Errors
///
/// Returns an error if:
/// - The file has an invalid extension
/// - The content fails to parse
pub fn parse_content(content: &str, path: &Path) -> Result<SyntaxFile, String> {
    let ext = validate_extension(path)?;

    match ext {
        SYSML_EXT => {
            let file = crate::syntax::sysml::parser::parse_content(content, path)?;
            Ok(SyntaxFile::SysML(file))
        }
        KERML_EXT => {
            // TODO: Implement KerML parser - currently returns empty placeholder
            Ok(SyntaxFile::KerML(KerMLFile::empty()))
        }
        _ => unreachable!("validate_extension should have caught this"),
    }
}

/// Parses content and returns a ParseResult with detailed error information.
/// This is the primary function for LSP usage - errors don't fail, they're captured.
///
/// Dispatches to the appropriate language parser based on file extension.
pub fn parse_with_result(content: &str, path: &Path) -> ParseResult<SyntaxFile> {
    let ext = match get_extension(path) {
        Ok(e) => e,
        Err(e) => return ParseResult::with_errors(vec![e]),
    };

    match ext {
        SYSML_EXT => {
            let result = crate::syntax::sysml::parser::parse_with_result(content, path);
            match result.content {
                Some(file) => ParseResult::success(SyntaxFile::SysML(file)),
                None => ParseResult::with_errors(result.errors),
            }
        }
        KERML_EXT => {
            // TODO: Implement KerML parser - currently returns empty placeholder
            ParseResult::success(SyntaxFile::KerML(KerMLFile::empty()))
        }
        _ => {
            let error = ParseError::syntax_error("Unsupported file extension", 0, 0);
            ParseResult::with_errors(vec![error])
        }
    }
}
