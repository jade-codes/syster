use crate::server::LspServer;
use crate::server::helpers::uri_to_path;
use async_lsp::lsp_types::*;
use syster::syntax::formatter;
use tokio_util::sync::CancellationToken;

impl LspServer {
    /// Get a snapshot of the document text for async formatting
    pub fn get_document_text(&self, uri: &Url) -> Option<String> {
        let path = uri_to_path(uri)?;
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
