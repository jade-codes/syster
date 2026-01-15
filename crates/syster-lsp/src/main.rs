use std::ops::ControlFlow;
use std::time::Duration;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::lsp_types::*;
use async_lsp::panic::CatchUnwindLayer;
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::tracing::TracingLayer;
use async_lsp::{ClientSocket, LanguageClient, LanguageServer, ResponseError};
use futures::future::BoxFuture;
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tracing::{Level, info};

mod server;
use server::LspServer;
use server::background_tasks::{debounce, events::ParseDocument};
use server::diagram::GetDiagramRequest;
use server::helpers::uri_to_path;

/// Server state that owns the LspServer and client socket
struct ServerState {
    client: ClientSocket,
    server: LspServer,
    /// Channel to send parse requests to the debounce task
    parse_tx: mpsc::UnboundedSender<Url>,
}

impl LanguageServer for ServerState {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        params: InitializeParams,
    ) -> BoxFuture<'static, Result<InitializeResult, Self::Error>> {
        let (stdlib_enabled, stdlib_path) =
            LspServer::parse_init_options(params.initialization_options);

        self.server = LspServer::with_config(stdlib_enabled, stdlib_path);

        // Extract workspace folders from initialization params
        let mut folders = Vec::new();

        // Try workspace_folders first (multi-root workspaces)
        if let Some(workspace_folders) = params.workspace_folders {
            for folder in workspace_folders {
                if let Ok(path) = folder.uri.to_file_path() {
                    folders.push(path);
                }
            }
        }

        // Fall back to root_uri (single folder workspace, deprecated but still used)
        #[allow(deprecated)]
        if folders.is_empty()
            && let Some(root_uri) = params.root_uri
            && let Ok(path) = root_uri.to_file_path()
        {
            folders.push(path);
        }

        self.server.set_workspace_folders(folders);

        let result = LspServer::initialize_result();
        Box::pin(async move { Ok(result) })
    }

    fn hover(
        &mut self,
        params: HoverParams,
    ) -> BoxFuture<'static, Result<Option<Hover>, Self::Error>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let result = self.server.get_hover(&uri, position);
        Box::pin(async move { Ok(result) })
    }

    fn definition(
        &mut self,
        params: GotoDefinitionParams,
    ) -> BoxFuture<'static, Result<Option<GotoDefinitionResponse>, Self::Error>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        let result = self
            .server
            .get_definition(&uri, position)
            .map(GotoDefinitionResponse::Scalar);
        Box::pin(async move { Ok(result) })
    }

    fn references(
        &mut self,
        params: ReferenceParams,
    ) -> BoxFuture<'static, Result<Option<Vec<Location>>, Self::Error>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let include_declaration = params.context.include_declaration;
        let result = self
            .server
            .get_references(&uri, position, include_declaration);
        Box::pin(async move { Ok(result) })
    }

    fn document_symbol(
        &mut self,
        params: DocumentSymbolParams,
    ) -> BoxFuture<'static, Result<Option<DocumentSymbolResponse>, Self::Error>> {
        let uri = params.text_document.uri;
        let Some(path) = uri_to_path(&uri) else {
            return Box::pin(async { Ok(None) });
        };
        let symbols = self.server.get_document_symbols(&path);
        let result = if symbols.is_empty() {
            None
        } else {
            Some(DocumentSymbolResponse::Nested(symbols))
        };
        Box::pin(async move { Ok(result) })
    }

    fn semantic_tokens_full(
        &mut self,
        params: SemanticTokensParams,
    ) -> BoxFuture<'static, Result<Option<SemanticTokensResult>, Self::Error>> {
        let uri = params.text_document.uri;
        let result = self.server.get_semantic_tokens(&uri);
        Box::pin(async move { Ok(result) })
    }

    fn completion(
        &mut self,
        params: CompletionParams,
    ) -> BoxFuture<'static, Result<Option<CompletionResponse>, Self::Error>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let Some(path) = uri_to_path(&uri) else {
            return Box::pin(async { Ok(None) });
        };
        let result = Some(self.server.get_completions(&path, position));
        Box::pin(async move { Ok(result) })
    }

    fn rename(
        &mut self,
        params: RenameParams,
    ) -> BoxFuture<'static, Result<Option<WorkspaceEdit>, Self::Error>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;
        let result = self.server.get_rename_edits(&uri, position, &new_name);
        Box::pin(async move { Ok(result) })
    }

    fn formatting(
        &mut self,
        params: DocumentFormattingParams,
    ) -> BoxFuture<'static, Result<Option<Vec<TextEdit>>, Self::Error>> {
        let uri = params.text_document.uri;
        let options = params.options;

        // Snapshot the text synchronously - this is fast
        let text_snapshot = self.server.get_document_text(&uri);

        // Get the current cancellation token for this document.
        let cancel_token = uri
            .to_file_path()
            .ok()
            .and_then(|path| self.server.get_document_cancel_token(&path))
            .unwrap_or_default();

        Box::pin(server::formatting::format_document(
            text_snapshot,
            options,
            cancel_token,
        ))
    }

    fn prepare_rename(
        &mut self,
        params: TextDocumentPositionParams,
    ) -> BoxFuture<'static, Result<Option<PrepareRenameResponse>, Self::Error>> {
        let uri = params.text_document.uri;
        let position = params.position;
        let result = self.server.prepare_rename(&uri, position);
        Box::pin(async move { Ok(result) })
    }

    fn folding_range(
        &mut self,
        params: FoldingRangeParams,
    ) -> BoxFuture<'static, Result<Option<Vec<FoldingRange>>, Self::Error>> {
        let uri = params.text_document.uri;
        let Some(path) = uri_to_path(&uri) else {
            return Box::pin(async { Ok(None) });
        };
        let ranges = self.server.get_folding_ranges(&path);
        let result = if ranges.is_empty() {
            None
        } else {
            Some(ranges)
        };
        Box::pin(async move { Ok(result) })
    }

    fn selection_range(
        &mut self,
        params: SelectionRangeParams,
    ) -> BoxFuture<'static, Result<Option<Vec<SelectionRange>>, Self::Error>> {
        let uri = params.text_document.uri;
        let positions = params.positions;
        let Some(path) = uri_to_path(&uri) else {
            return Box::pin(async { Ok(None) });
        };
        let ranges = self.server.get_selection_ranges(&path, positions);
        let result = if ranges.is_empty() {
            None
        } else {
            Some(ranges)
        };
        Box::pin(async move { Ok(result) })
    }

    fn inlay_hint(
        &mut self,
        params: InlayHintParams,
    ) -> BoxFuture<'static, Result<Option<Vec<InlayHint>>, Self::Error>> {
        let hints = self.server.get_inlay_hints(&params);
        let result = if hints.is_empty() { None } else { Some(hints) };
        Box::pin(async move { Ok(result) })
    }

    fn code_lens(
        &mut self,
        params: CodeLensParams,
    ) -> BoxFuture<'static, Result<Option<Vec<CodeLens>>, Self::Error>> {
        let lenses = self.server.get_code_lenses(&params.text_document.uri);
        let result = if lenses.is_empty() {
            None
        } else {
            Some(lenses)
        };
        Box::pin(async move { Ok(result) })
    }

    fn document_link(
        &mut self,
        params: DocumentLinkParams,
    ) -> BoxFuture<'static, Result<Option<Vec<DocumentLink>>, Self::Error>> {
        let links = self.server.get_document_links(&params.text_document.uri);
        let result = if links.is_empty() { None } else { Some(links) };
        Box::pin(async move { Ok(result) })
    }

    fn symbol(
        &mut self,
        params: WorkspaceSymbolParams,
    ) -> BoxFuture<'static, Result<Option<WorkspaceSymbolResponse>, Self::Error>> {
        let symbols = self.server.get_workspace_symbols(&params.query);
        let result = if symbols.is_empty() {
            None
        } else {
            Some(WorkspaceSymbolResponse::Nested(symbols))
        };
        Box::pin(async move { Ok(result) })
    }

    // Notification handlers - these are called synchronously in async-lsp!
    // This is the key difference from tower-lsp that fixes our ordering issues.

    fn did_open(&mut self, params: DidOpenTextDocumentParams) -> Self::NotifyResult {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;
        info!("did_open: {}", uri);

        match self.server.open_document(&uri, &text) {
            Ok(_) => {
                let diagnostics = self.server.get_diagnostics(&uri);
                let _ = self.client.publish_diagnostics(PublishDiagnosticsParams {
                    uri,
                    diagnostics,
                    version: None,
                });
            }
            Err(e) => {
                let _ = self.client.log_message(LogMessageParams {
                    typ: MessageType::ERROR,
                    message: format!("Failed to open document {uri}: {e}"),
                });
            }
        }
        ControlFlow::Continue(())
    }

    fn did_change(&mut self, params: DidChangeTextDocumentParams) -> Self::NotifyResult {
        let uri = params.text_document.uri.clone();
        info!(
            "did_change: {} ({} changes)",
            uri,
            params.content_changes.len()
        );

        // Cancel any in-flight operations for this document (formatting, hover, etc.)
        // This ensures old operations don't waste CPU on stale data
        if let Ok(path) = uri.to_file_path() {
            self.server.cancel_document_operations(&path);
        }

        // Apply text changes only (fast - just string manipulation)
        for change in params.content_changes {
            if let Err(e) = self.server.apply_text_change_only(&uri, &change) {
                let _ = self.client.log_message(LogMessageParams {
                    typ: MessageType::ERROR,
                    message: format!("Failed to apply change to {uri}: {e}"),
                });
                return ControlFlow::Continue(());
            }
        }

        // Send parse request to debounce task (non-blocking)
        let _ = self.parse_tx.send(uri);

        ControlFlow::Continue(())
    }

    fn did_close(&mut self, params: DidCloseTextDocumentParams) -> Self::NotifyResult {
        let uri = params.text_document.uri;
        if let Err(e) = self.server.close_document(&uri) {
            let _ = self.client.log_message(LogMessageParams {
                typ: MessageType::ERROR,
                message: format!("Failed to close document {uri}: {e}"),
            });
        }
        ControlFlow::Continue(())
    }

    fn did_save(&mut self, _params: DidSaveTextDocumentParams) -> Self::NotifyResult {
        ControlFlow::Continue(())
    }

    fn did_change_watched_files(
        &mut self,
        _params: DidChangeWatchedFilesParams,
    ) -> Self::NotifyResult {
        // Currently we don't need to react to file system changes
        // The workspace is updated when files are opened/changed via the editor
        ControlFlow::Continue(())
    }
}

impl ServerState {
    fn new_router(client: ClientSocket) -> Router<Self> {
        let (parse_tx, parse_rx) = mpsc::unbounded_channel::<Url>();

        // Spawn debounced parse task: waits for typing to pause before parsing
        let emit_client = client.clone();
        debounce::spawn(
            Duration::from_millis(debounce::DEFAULT_DELAY_MS),
            parse_rx,
            move |uri| emit_client.emit(ParseDocument { uri }).is_ok(),
        );

        let mut router = Router::from_language_server(Self {
            client,
            server: LspServer::new(),
            parse_tx,
        });

        // Handle ParseDocument events
        router.event(|state: &mut ServerState, event: ParseDocument| {
            state.server.parse_document(&event.uri);

            let diagnostics = state.server.get_diagnostics(&event.uri);
            let _ = state.client.publish_diagnostics(PublishDiagnosticsParams {
                uri: event.uri,
                diagnostics,
                version: None,
            });
            ControlFlow::Continue(())
        });

        // Custom request: syster/getDiagram
        // Returns diagram data (symbols + relationships) for visualization
        router.request::<GetDiagramRequest, _>(|state, params| {
            let file_path = params
                .uri
                .as_ref()
                .and_then(|uri| Url::parse(uri).ok())
                .and_then(|url| url.to_file_path().ok());
            let result = state.server.get_diagram(file_path.as_deref());
            Box::pin(async move { Ok(result) })
        });

        router
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing to stderr (stdout is used for LSP communication)
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_writer(std::io::stderr)
        .init();

    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client.clone()))
            .service(ServerState::new_router(client))
    });

    // Use tokio compat for stdin/stdout
    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
        async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
    );

    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    server.run_buffered(stdin, stdout).await.unwrap();
}

#[cfg(test)]
#[path = "main/tests.rs"]
mod tests;
