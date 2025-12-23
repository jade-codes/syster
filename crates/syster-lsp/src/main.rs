use serde_json::Value;
use std::path::PathBuf;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server as TowerServer};

mod server;
use server::LspServer;

struct SysterLanguageServer {
    client: Client,
    server: std::sync::Arc<tokio::sync::Mutex<LspServer>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for SysterLanguageServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Parse initialization options for stdlib configuration
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

        // Configure server with initialization options
        let mut server = self.server.lock().await;
        *server = LspServer::with_config(stdlib_enabled, stdlib_path);

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
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
                rename_provider: Some(OneOf::Left(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![":".to_string(), " ".to_string()]),
                    ..Default::default()
                }),
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
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "SysML v2 language server initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;

        let mut server = self.server.lock().await;
        match server.open_document(&uri, &text) {
            Ok(_) => {
                // Publish diagnostics
                let diagnostics = server.get_diagnostics(&uri);
                self.client
                    .publish_diagnostics(uri, diagnostics, None)
                    .await;
            }
            Err(e) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Failed to open document {uri}: {e}"),
                    )
                    .await;
            }
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        // We're using FULL sync, so there should be exactly one change with full content
        if let Some(change) = params.content_changes.into_iter().next() {
            let mut server = self.server.lock().await;
            match server.change_document(&uri, &change.text) {
                Ok(_) => {
                    // Publish diagnostics
                    let diagnostics = server.get_diagnostics(&uri);
                    self.client
                        .publish_diagnostics(uri, diagnostics, None)
                        .await;
                }
                Err(e) => {
                    self.client
                        .log_message(
                            MessageType::ERROR,
                            format!("Failed to update document {uri}: {e}"),
                        )
                        .await;
                }
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        let mut server = self.server.lock().await;
        if let Err(e) = server.close_document(&uri) {
            self.client
                .log_message(
                    MessageType::ERROR,
                    format!("Failed to close document {uri}: {e}"),
                )
                .await;
        }
    }

    async fn did_save(&self, _params: DidSaveTextDocumentParams) {
        // Note: The document content is already up-to-date from did_change events
        // No need to reload the file here since we track changes incrementally
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let server = self.server.lock().await;
        Ok(server.get_hover(&uri, position))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let server = self.server.lock().await;
        let location = server.get_definition(&uri, position);

        Ok(location.map(GotoDefinitionResponse::Scalar))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let include_declaration = params.context.include_declaration;

        let server = self.server.lock().await;
        Ok(server.get_references(&uri, position, include_declaration))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;

        let server = self.server.lock().await;
        let path = std::path::Path::new(uri.path());
        let symbols = server.get_document_symbols(path);

        if symbols.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DocumentSymbolResponse::Nested(symbols)))
        }
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;

        let server = self.server.lock().await;
        let result = server.get_semantic_tokens(uri.as_str());
        Ok(result)
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        let server = self.server.lock().await;
        let path = std::path::Path::new(uri.path());
        let response = server.get_completions(path, position);

        Ok(Some(response))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;
        let new_name = params.new_name;

        let server = self.server.lock().await;
        Ok(server.get_rename_edits(&uri, position, &new_name))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // Don't initialize tracing subscriber - it writes to stdout which corrupts LSP protocol
    // LSP uses stdin/stdout for JSON-RPC communication
    // If logging is needed, use client.log_message() instead

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| SysterLanguageServer {
        client,
        server: std::sync::Arc::new(tokio::sync::Mutex::new(LspServer::new())),
    });
    TowerServer::new(stdin, stdout, socket).serve(service).await;
}
