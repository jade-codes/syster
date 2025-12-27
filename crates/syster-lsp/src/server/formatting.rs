use crate::server::LspServer;
use async_lsp::lsp_types::*;
use syster::syntax::formatter;
use tokio_util::sync::CancellationToken;

impl LspServer {
    /// Get a snapshot of the document text for async formatting
    pub fn get_document_text(&self, uri: &Url) -> Option<String> {
        let path = uri.to_file_path().ok()?;
        self.document_texts.get(&path).cloned()
    }
}

/// Format text with cancellation support
/// Returns None if cancelled or if no changes needed
pub fn format_text_async(
    text: &str,
    options: FormattingOptions,
    cancel: &CancellationToken,
) -> Option<Vec<TextEdit>> {
    // Check cancellation before starting
    if cancel.is_cancelled() {
        return None;
    }

    // Convert LSP options to formatter options
    let format_options = formatter::FormatOptions {
        tab_size: options.tab_size as usize,
        insert_spaces: options.insert_spaces,
        print_width: 80, // Default print width
    };

    // Use the Rowan-based formatter that preserves comments
    // The formatter checks the cancellation token periodically
    let formatted = formatter::format_async(text, &format_options, cancel)?;

    // Check cancellation before building result
    if cancel.is_cancelled() {
        return None;
    }

    if formatted == text {
        return None;
    }

    Some(vec![TextEdit {
        range: full_document_range(text),
        new_text: formatted,
    }])
}

/// Calculate the range that covers the entire document
fn full_document_range(text: &str) -> Range {
    let line_count = text.lines().count().saturating_sub(1) as u32;
    let last_char = text.lines().last().map_or(0, |line| line.len() as u32);

    Range {
        start: Position::new(0, 0),
        end: Position::new(line_count, last_char),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_basic_kerml() {
        let input = "package Test {
feature x;
}";
        let format_options = syster::syntax::formatter::FormatOptions {
            tab_size: 4,
            insert_spaces: true,
            print_width: 80,
        };

        let result = syster::syntax::formatter::format_async(
            input,
            &format_options,
            &CancellationToken::new(),
        )
        .unwrap();

        // Rowan formatter preserves structure with proper indentation
        assert!(result.contains("package Test"));
        assert!(result.contains("feature x"));
    }

    #[test]
    fn test_format_nested_kerml() {
        let input = "package Test {
struct Vehicle {
feature wheels;
}
}";
        let format_options = syster::syntax::formatter::FormatOptions {
            tab_size: 2,
            insert_spaces: true,
            print_width: 80,
        };

        let result = syster::syntax::formatter::format_async(
            input,
            &format_options,
            &CancellationToken::new(),
        )
        .unwrap();

        // Verify structure is preserved
        assert!(result.contains("package Test"));
        assert!(result.contains("struct Vehicle"));
        assert!(result.contains("feature wheels"));
    }

    #[test]
    fn test_format_with_tabs() {
        let input = "package Test {
feature x;
}";
        let format_options = syster::syntax::formatter::FormatOptions {
            tab_size: 1,
            insert_spaces: false,
            print_width: 80,
        };

        let result = syster::syntax::formatter::format_async(
            input,
            &format_options,
            &CancellationToken::new(),
        )
        .unwrap();

        // Verify tabs are used for indentation
        assert!(result.contains("package Test"));
        assert!(result.contains("\t")); // Should have tab indentation
    }

    #[test]
    fn test_format_preserves_comments() {
        let input = "// This is a comment
package Test {
    /* block comment */
    feature x;
}";
        let format_options = syster::syntax::formatter::FormatOptions {
            tab_size: 4,
            insert_spaces: true,
            print_width: 80,
        };

        let result = syster::syntax::formatter::format_async(
            input,
            &format_options,
            &CancellationToken::new(),
        )
        .unwrap();

        // Rowan formatter preserves comments
        assert!(result.contains("// This is a comment"));
        assert!(result.contains("/* block comment */"));
    }

    #[test]
    fn test_format_normalizes_excessive_whitespace() {
        let input = "metadata def              ToolVariable";
        let format_options = syster::syntax::formatter::FormatOptions {
            tab_size: 4,
            insert_spaces: true,
            print_width: 80,
        };

        let result = syster::syntax::formatter::format_async(
            input,
            &format_options,
            &CancellationToken::new(),
        )
        .unwrap();

        // Multiple spaces should be normalized to single space
        assert_eq!(
            result.trim(),
            "metadata def ToolVariable",
            "Should normalize multiple spaces. Got: |{}|",
            result
        );
    }

    #[test]
    fn test_lsp_format_normalizes_whitespace() {
        let source = "metadata def              ToolVariable  ";
        let options = FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        };

        let result = format_text_async(source, options, &CancellationToken::new());

        assert!(result.is_some(), "format should return Some edits");
        let edits = result.unwrap();
        assert_eq!(edits.len(), 1, "Should have one edit");

        let new_text = &edits[0].new_text;
        assert!(
            new_text.contains("metadata def ToolVariable"),
            "Formatted text should normalize whitespace. Got: |{}|",
            new_text
        );
        assert!(
            !new_text.contains("def              "),
            "Should not have multiple spaces. Got: |{}|",
            new_text
        );
    }
}
