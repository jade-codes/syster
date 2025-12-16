use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod backend;
use backend::Backend;

struct SysterLanguageServer {
    client: Client,
    backend: std::sync::Arc<tokio::sync::Mutex<Backend>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for SysterLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
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
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        let mut backend = self.backend.lock().await;
        match backend.open_document(&uri, &text) {
            Ok(_) => {
                self.client
                    .log_message(MessageType::INFO, format!("Opened document: {}", uri))
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
        let uri = params.text_document.uri;

        // We're using FULL sync, so there should be exactly one change with full content
        if let Some(change) = params.content_changes.into_iter().next() {
            let mut backend = self.backend.lock().await;
            match backend.change_document(&uri, &change.text) {
                Ok(_) => {
                    self.client
                        .log_message(MessageType::INFO, format!("Updated document: {}", uri))
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

        let mut backend = self.backend.lock().await;
        match backend.close_document(&uri) {
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
        backend: std::sync::Arc::new(tokio::sync::Mutex::new(Backend::new())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
