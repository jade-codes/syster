use crate::server::LspServer;
use crate::server::helpers::uri_to_path;
use async_lsp::ResponseError;
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

/// Handle document formatting request asynchronously
///
/// This function takes snapshots of the required data and returns a future
/// that can be awaited. The formatting work runs on a blocking thread pool
/// and respects cancellation.
pub async fn format_document(
    text_snapshot: Option<String>,
    options: FormattingOptions,
    cancel_token: CancellationToken,
) -> Result<Option<Vec<TextEdit>>, ResponseError> {
    let result = match text_snapshot {
        Some(text) => {
            let cancel_for_select = cancel_token.clone();

            // Run formatting on the blocking thread pool.
            // Use select! to race the work against cancellation.
            let format_task =
                tokio::task::spawn_blocking(move || format_text(&text, options, &cancel_token));

            tokio::select! {
                result = format_task => result.unwrap_or(None),
                _ = cancel_for_select.cancelled() => None,
            }
        }
        None => None,
    };

    Ok(result)
}

/// Format text with cancellation support
/// Returns None if cancelled or if no changes needed
pub fn format_text(
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
