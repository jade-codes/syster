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
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
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
                self.client
                    .log_message(MessageType::INFO, format!("Opened document: {}", uri))
                    .await;

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
                        format!("Failed to open document {}: {}", uri, e),
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
                    self.client
                        .log_message(MessageType::INFO, format!("Updated document: {}", uri))
                        .await;

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
                            format!("Failed to update document {}: {}", uri, e),
                        )
                        .await;
                }
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;

        let mut server = self.server.lock().await;
        match server.close_document(&uri) {
            Ok(_) => {
                self.client
                    .log_message(MessageType::INFO, format!("Closed document: {}", uri))
                    .await;
            }
            Err(e) => {
                self.client
                    .log_message(
                        MessageType::ERROR,
                        format!("Failed to close document {}: {}", uri, e),
                    )
                    .await;
            }
        }
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

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| SysterLanguageServer {
        client,
        server: std::sync::Arc::new(tokio::sync::Mutex::new(LspServer::new())),
    });
    TowerServer::new(stdin, stdout, socket).serve(service).await;
}
