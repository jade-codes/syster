//! LSP server events
//!
//! Events are used to communicate between async tasks and the main LSP loop.
//! They are emitted via `ClientSocket::emit()` and handled by `Router::event()`.

use async_lsp::lsp_types::Url;

/// Trigger document parsing after debounce delay
pub struct ParseDocument {
    pub uri: Url,
}
