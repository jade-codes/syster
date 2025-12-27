use std::ops::ControlFlow;
use std::path::PathBuf;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::panic::CatchUnwindLayer;
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::tracing::TracingLayer;
use async_lsp::{ClientSocket, LanguageClient, LanguageServer, ResponseError};
use futures::future::BoxFuture;
use serde_json::Value;
use tower::ServiceBuilder;
use tracing::{Level, info};

use async_lsp::lsp_types::*;

mod server;
use server::LspServer;

/// Server state that owns the LspServer and client socket
struct ServerState {
    client: ClientSocket,
    server: LspServer,
}

impl LanguageServer for ServerState {
    type Error = ResponseError;
    type NotifyResult = ControlFlow<async_lsp::Result<()>>;

    fn initialize(
        &mut self,
        params: InitializeParams,
    ) -> BoxFuture<'static, Result<InitializeResult, Self::Error>> {
        let (stdlib_enabled, stdlib_path) =
            if let Some(Value::Object(opts)) = params.initialization_options {
                let enabled = opts
                    .get("stdlibEnabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                let path = opts
                    .get("stdlibPath")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(PathBuf::from);
                (enabled, path)
            } else {
                (true, None)
            };

        // Update server with config - this happens synchronously before returning the future
        self.server = LspServer::with_config(stdlib_enabled, stdlib_path);

        Box::pin(async move {
            Ok(InitializeResult {
                capabilities: ServerCapabilities {
                    text_document_sync: Some(TextDocumentSyncCapability::Options(
                        TextDocumentSyncOptions {
                            open_close: Some(true),
                            change: Some(TextDocumentSyncKind::INCREMENTAL),
                            will_save: None,
                            will_save_wait_until: None,
                            save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                                include_text: Some(false),
                            })),
                        },
                    )),
                    hover_provider: Some(HoverProviderCapability::Simple(true)),
                    definition_provider: Some(OneOf::Left(true)),
                    references_provider: Some(OneOf::Left(true)),
                    document_symbol_provider: Some(OneOf::Left(true)),
                    rename_provider: Some(OneOf::Right(RenameOptions {
                        prepare_provider: Some(true),
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                    })),
                    document_formatting_provider: Some(OneOf::Left(true)),
                    completion_provider: Some(CompletionOptions {
                        resolve_provider: Some(false),
                        trigger_characters: Some(vec![":".to_string(), " ".to_string()]),
                        ..Default::default()
                    }),
                    folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
                    selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
                    semantic_tokens_provider: Some(
                        SemanticTokensServerCapabilities::SemanticTokensOptions(
                            SemanticTokensOptions {
                                legend: LspServer::semantic_tokens_legend(),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                                range: None,
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                            },
                        ),
                    ),
                    workspace: Some(WorkspaceServerCapabilities {
                        workspace_folders: None,
                        file_operations: None,
                    }),
                    ..Default::default()
                },
                server_info: Some(ServerInfo {
                    name: "SysML v2 Language Server".to_string(),
                    version: Some(env!("CARGO_PKG_VERSION").to_string()),
                }),
            })
        })
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
        let path = std::path::Path::new(uri.path());
        let symbols = self.server.get_document_symbols(path);
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
        let result = self.server.get_semantic_tokens(uri.as_str());
        Box::pin(async move { Ok(result) })
    }

    fn completion(
        &mut self,
        params: CompletionParams,
    ) -> BoxFuture<'static, Result<Option<CompletionResponse>, Self::Error>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let path = std::path::Path::new(uri.path());
        let result = Some(self.server.get_completions(path, position));
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
        info!("formatting: snapshot for {}", uri);

        // Snapshot the text synchronously - this is fast
        let text_snapshot = self.server.get_document_text(&uri);

        // Get the current cancellation token for this document.
        // When didChange arrives, this token is cancelled and replaced.
        let cancel_token = uri
            .to_file_path()
            .ok()
            .and_then(|path| self.server.get_document_cancel_token(&path))
            .unwrap_or_default();
        let cancel_token_for_select = cancel_token.clone();

        Box::pin(async move {
            info!("formatting: start work for {}", uri);
            let result = match text_snapshot {
                Some(text) => {
                    // Run formatting on the blocking thread pool.
                    // Use select! to race the work against cancellation.
                    let format_task = tokio::task::spawn_blocking(move || {
                        server::formatting::format_text_async(&text, options, &cancel_token)
                    });

                    tokio::select! {
                        result = format_task => result.unwrap_or(None),
                        _ = cancel_token_for_select.cancelled() => {
                            info!("formatting: cancelled for {}", uri);
                            None
                        }
                    }
                }
                None => None,
            };
            info!("formatting: done for {}", uri);
            Ok(result)
        })
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
        let path = std::path::Path::new(uri.path());
        let ranges = self.server.get_folding_ranges(path);
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
        let path = std::path::Path::new(uri.path());
        let ranges = self.server.get_selection_ranges(path, positions);
        let result = if ranges.is_empty() {
            None
        } else {
            Some(ranges)
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

        for change in params.content_changes {
            if let Err(e) = self.server.apply_incremental_change(&uri, &change) {
                let _ = self.client.log_message(LogMessageParams {
                    typ: MessageType::ERROR,
                    message: format!("Failed to apply change to {uri}: {e}"),
                });
                return ControlFlow::Continue(());
            }
        }

        let diagnostics = self.server.get_diagnostics(&uri);
        info!("did_change: done, {} diagnostics", diagnostics.len());
        let _ = self.client.publish_diagnostics(PublishDiagnosticsParams {
            uri,
            diagnostics,
            version: None,
        });
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
}

impl ServerState {
    fn new_router(client: ClientSocket) -> Router<Self> {
        Router::from_language_server(Self {
            client,
            server: LspServer::new(),
        })
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
