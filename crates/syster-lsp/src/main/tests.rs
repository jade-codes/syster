//! Tests for the main LSP server entry point
//!
//! These tests verify the ServerState and its LanguageServer trait implementation,
//! which are the core components used by the main() function.

use super::*;
use serde_json::Value;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Helper to create a test ServerState
/// Returns the state and the parse receiver to keep the channel open
fn create_test_server_state() -> (ServerState, mpsc::UnboundedReceiver<Url>) {
    let client = ClientSocket::new_closed();
    let (parse_tx, parse_rx) = mpsc::unbounded_channel::<Url>();

    let state = ServerState {
        client,
        server: LspServer::new(),
        parse_tx,
    };

    (state, parse_rx)
}

#[tokio::test]
async fn test_initialize_with_default_options() {
    let (mut state, _parse_rx) = create_test_server_state();

    let params = InitializeParams {
        initialization_options: None,
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(
        result.is_ok(),
        "Initialize should succeed with default options"
    );

    let init_result = result.unwrap();
    assert_eq!(
        init_result.server_info.as_ref().unwrap().name,
        "SysML v2 Language Server"
    );
    assert!(init_result.server_info.as_ref().unwrap().version.is_some());
}

#[tokio::test]
async fn test_initialize_with_stdlib_disabled() {
    let (mut state, _parse_rx) = create_test_server_state();

    let mut opts = serde_json::Map::new();
    opts.insert("stdlibEnabled".to_string(), Value::Bool(false));

    let params = InitializeParams {
        initialization_options: Some(Value::Object(opts)),
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(
        result.is_ok(),
        "Initialize should succeed with stdlib disabled"
    );
}

#[tokio::test]
async fn test_initialize_with_custom_stdlib_path() {
    let (mut state, _parse_rx) = create_test_server_state();

    let mut opts = serde_json::Map::new();
    opts.insert("stdlibEnabled".to_string(), Value::Bool(true));
    opts.insert(
        "stdlibPath".to_string(),
        Value::String("/custom/path".to_string()),
    );

    let params = InitializeParams {
        initialization_options: Some(Value::Object(opts)),
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(
        result.is_ok(),
        "Initialize should succeed with custom stdlib path"
    );
}

#[tokio::test]
async fn test_initialize_with_empty_stdlib_path() {
    let (mut state, _parse_rx) = create_test_server_state();

    let mut opts = serde_json::Map::new();
    opts.insert("stdlibEnabled".to_string(), Value::Bool(true));
    opts.insert("stdlibPath".to_string(), Value::String("".to_string())); // Empty string should be filtered

    let params = InitializeParams {
        initialization_options: Some(Value::Object(opts)),
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(
        result.is_ok(),
        "Initialize should succeed, filtering empty stdlib path"
    );
}

#[tokio::test]
async fn test_initialize_capabilities() {
    let (mut state, _parse_rx) = create_test_server_state();

    let params = InitializeParams::default();
    let result = state.initialize(params).await.unwrap();

    let caps = result.capabilities;

    // Verify text document sync
    assert!(caps.text_document_sync.is_some());
    if let Some(TextDocumentSyncCapability::Options(opts)) = caps.text_document_sync {
        assert_eq!(opts.open_close, Some(true));
        assert_eq!(opts.change, Some(TextDocumentSyncKind::INCREMENTAL));
    }

    // Verify feature capabilities
    assert!(caps.hover_provider.is_some());
    assert!(caps.definition_provider.is_some());
    assert!(caps.references_provider.is_some());
    assert!(caps.document_symbol_provider.is_some());
    assert!(caps.rename_provider.is_some());
    assert!(caps.document_formatting_provider.is_some());
    assert!(caps.completion_provider.is_some());
    assert!(caps.folding_range_provider.is_some());
    assert!(caps.selection_range_provider.is_some());
    assert!(caps.inlay_hint_provider.is_some());
    assert!(caps.semantic_tokens_provider.is_some());
    assert!(caps.workspace_symbol_provider.is_some());
}

#[tokio::test]
async fn test_initialize_completion_trigger_characters() {
    let (mut state, _parse_rx) = create_test_server_state();

    let params = InitializeParams::default();
    let result = state.initialize(params).await.unwrap();

    if let Some(completion) = result.capabilities.completion_provider {
        assert_eq!(
            completion.trigger_characters,
            Some(vec![":".to_string(), " ".to_string()])
        );
    } else {
        panic!("Completion provider should be configured");
    }
}

#[test]
fn test_did_open_valid_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    };

    let result = state.did_open(params);
    assert!(matches!(result, ControlFlow::Continue(())));

    // Verify document was added to workspace
    assert_eq!(state.server.workspace().file_count(), 1);
}

#[test]
fn test_did_open_invalid_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "invalid syntax !@#$%".to_string(),
        },
    };

    let result = state.did_open(params);
    assert!(matches!(result, ControlFlow::Continue(())));

    // Document should NOT be in workspace (parse failed)
    assert_eq!(state.server.workspace().file_count(), 0);
}

#[test]
fn test_did_open_unsupported_extension() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.txt").unwrap();
    let params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "text".to_string(),
            version: 1,
            text: "some text".to_string(),
        },
    };

    let result = state.did_open(params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_change_incremental() {
    let (mut state, _parse_rx) = create_test_server_state();

    // First open a document
    let uri = Url::parse("file:///test.sysml").unwrap();
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    };
    let _ = state.did_open(open_params);

    // Now make an incremental change
    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 0,
                    character: 16,
                },
                end: Position {
                    line: 0,
                    character: 16,
                },
            }),
            range_length: None,
            text: "\npart def Engine;".to_string(),
        }],
    };

    let result = state.did_change(change_params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_change_multiple_changes() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    };
    let _ = state.did_open(open_params);

    // Multiple changes in one notification
    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![
            TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 0,
                        character: 16,
                    },
                    end: Position {
                        line: 0,
                        character: 16,
                    },
                }),
                range_length: None,
                text: "\npart def Engine;".to_string(),
            },
            TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 1,
                        character: 16,
                    },
                    end: Position {
                        line: 1,
                        character: 16,
                    },
                }),
                range_length: None,
                text: "\npart def Wheel;".to_string(),
            },
        ],
    };

    let result = state.did_change(change_params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_change_sends_parse_request() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    };
    let _ = state.did_open(open_params);

    // The parse_tx channel should not be closed
    assert!(!state.parse_tx.is_closed());

    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None, // Full document change
            range_length: None,
            text: "part def Engine;".to_string(),
        }],
    };

    let result = state.did_change(change_params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_close() {
    let (mut state, _parse_rx) = create_test_server_state();

    // First open a document
    let uri = Url::parse("file:///test.sysml").unwrap();
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    };
    let _ = state.did_open(open_params);
    assert_eq!(state.server.workspace().file_count(), 1);

    // Now close it
    let close_params = DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
    };

    let result = state.did_close(close_params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_save() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let params = DidSaveTextDocumentParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        text: None,
    };

    let result = state.did_save(params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[tokio::test]
async fn test_hover_on_valid_symbol() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 12,
            }, // On "Vehicle"
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.hover(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_hover_on_empty_position() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 0,
            }, // Before any symbol
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.hover(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_definition_request() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 12,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.definition(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_references_with_declaration() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;\npart v : Vehicle;")
        .unwrap();

    let params = ReferenceParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 12,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: ReferenceContext {
            include_declaration: true,
        },
    };

    let result = state.references(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_references_without_declaration() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;\npart v : Vehicle;")
        .unwrap();

    let params = ReferenceParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 12,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: ReferenceContext {
            include_declaration: false,
        },
    };

    let result = state.references(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_document_symbol() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.document_symbol(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_document_symbol_empty_file() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///empty.sysml").unwrap();
    state.server.open_document(&uri, "").unwrap();

    let params = DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.document_symbol(params).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none(), "Empty file should return None");
}

#[tokio::test]
async fn test_semantic_tokens_full() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = SemanticTokensParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.semantic_tokens_full(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_completion() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 5,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    let result = state.completion(params).await;
    assert!(result.is_ok());
    assert!(
        result.unwrap().is_some(),
        "Completion should return results"
    );
}

#[tokio::test]
async fn test_rename() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = RenameParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 12,
            },
        },
        new_name: "Car".to_string(),
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.rename(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_prepare_rename() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = TextDocumentPositionParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        position: Position {
            line: 0,
            character: 12,
        },
    };

    let result = state.prepare_rename(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_folding_range() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "package Test {\n  part def Vehicle;\n}")
        .unwrap();

    let params = FoldingRangeParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.folding_range(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_folding_range_empty_file() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///empty.sysml").unwrap();
    state.server.open_document(&uri, "").unwrap();

    let params = FoldingRangeParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.folding_range(params).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none(), "Empty file should return None");
}

#[tokio::test]
async fn test_selection_range() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = SelectionRangeParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        positions: vec![Position {
            line: 0,
            character: 12,
        }],
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.selection_range(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_selection_range_empty() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = SelectionRangeParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        positions: vec![], // Empty positions
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.selection_range(params).await;
    assert!(result.is_ok());
    assert!(
        result.unwrap().is_none(),
        "Empty positions should return None"
    );
}

#[tokio::test]
async fn test_inlay_hint() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 17,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.inlay_hint(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_formatting_with_valid_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part   def    Vehicle;")
        .unwrap();

    let params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        options: FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.formatting(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_formatting_with_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();

    let params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        options: FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.formatting(params).await;
    assert!(result.is_ok());
    assert!(
        result.unwrap().is_none(),
        "Formatting unopened document should return None"
    );
}

#[tokio::test]
async fn test_new_router_creates_channels() {
    let client = ClientSocket::new_closed();

    let router = ServerState::new_router(client);

    // Router should be created successfully
    // We can't easily inspect the router internals, but we can verify it exists
    std::mem::drop(router);
}

#[test]
fn test_server_state_has_valid_server() {
    let (state, _parse_rx) = create_test_server_state();

    // Verify the server is initialized
    assert_eq!(state.server.workspace().file_count(), 0);
}

#[test]
fn test_did_change_cancels_previous_operations() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    };
    let _ = state.did_open(open_params);

    // Get a cancel token before the change
    let path = PathBuf::from("/test.sysml");
    let token_before = state.server.cancel_document_operations(&path);
    assert!(!token_before.is_cancelled());

    // Make a change - this should cancel previous operations
    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "part def Engine;".to_string(),
        }],
    };

    let result = state.did_change(change_params);
    assert!(matches!(result, ControlFlow::Continue(())));

    // The token from before should be cancelled
    assert!(token_before.is_cancelled());
}

#[tokio::test]
async fn test_formatting_respects_cancellation() {
    use tokio::time::{Duration, timeout};

    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        options: FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    // Start formatting in background
    let format_future = state.formatting(params);

    // Cancel the operation by triggering a document change
    let path = uri.to_file_path().unwrap();
    state.server.cancel_document_operations(&path);

    // Wait for formatting to complete (it should be cancelled)
    let result = timeout(Duration::from_secs(5), format_future).await;
    assert!(
        result.is_ok(),
        "Formatting should complete (possibly cancelled)"
    );
}

#[test]
fn test_multiple_documents_lifecycle() {
    let (mut state, _parse_rx) = create_test_server_state();

    // Open multiple documents
    let uri1 = Url::parse("file:///test1.sysml").unwrap();
    let uri2 = Url::parse("file:///test2.sysml").unwrap();

    let _ = state.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri1.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    });

    let _ = state.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri2.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Engine;".to_string(),
        },
    });

    assert_eq!(state.server.workspace().file_count(), 2);

    // Close one document - this doesn't remove from workspace to keep cross-file references working
    let result = state.did_close(DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri: uri1.clone() },
    });

    assert!(matches!(result, ControlFlow::Continue(())));
    // Files remain in workspace even after close for cross-file references
    assert_eq!(state.server.workspace().file_count(), 2);
}

#[tokio::test]
async fn test_hover_after_change() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    // Make a change
    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "part def Engine;".to_string(),
        }],
    };
    let _ = state.did_change(change_params);

    // Parse the change
    state.server.parse_document(&uri);

    // Hover should work on the new content
    let params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 12,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.hover(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_initialize_updates_server_config() {
    let (mut state, _parse_rx) = create_test_server_state();

    // Verify initial state
    assert_eq!(state.server.workspace().file_count(), 0);

    // Initialize with stdlib disabled
    let mut opts = serde_json::Map::new();
    opts.insert("stdlibEnabled".to_string(), Value::Bool(false));

    let params = InitializeParams {
        initialization_options: Some(Value::Object(opts)),
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(result.is_ok());

    // Server should be updated with new config
    // (We can't directly test the internal config, but we can verify the server still works)
    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();
    assert_eq!(state.server.workspace().file_count(), 1);
}

// ========================================
// Additional comprehensive tests
// ========================================

#[tokio::test]
async fn test_initialize_with_mixed_options() {
    let (mut state, _parse_rx) = create_test_server_state();

    let mut opts = serde_json::Map::new();
    opts.insert("stdlibEnabled".to_string(), Value::Bool(true));
    opts.insert(
        "stdlibPath".to_string(),
        Value::String("/custom".to_string()),
    );
    opts.insert(
        "unknownOption".to_string(),
        Value::String("ignored".to_string()),
    );

    let params = InitializeParams {
        initialization_options: Some(Value::Object(opts)),
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(
        result.is_ok(),
        "Initialize should handle unknown options gracefully"
    );
}

#[tokio::test]
async fn test_initialize_with_invalid_stdlib_path_type() {
    let (mut state, _parse_rx) = create_test_server_state();

    let mut opts = serde_json::Map::new();
    opts.insert("stdlibEnabled".to_string(), Value::Bool(true));
    opts.insert("stdlibPath".to_string(), Value::Number(123.into())); // Wrong type

    let params = InitializeParams {
        initialization_options: Some(Value::Object(opts)),
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(
        result.is_ok(),
        "Initialize should handle invalid option types"
    );
}

#[tokio::test]
async fn test_initialize_with_non_object_options() {
    let (mut state, _parse_rx) = create_test_server_state();

    let params = InitializeParams {
        initialization_options: Some(Value::String("invalid".to_string())),
        ..Default::default()
    };

    let result = state.initialize(params).await;
    assert!(
        result.is_ok(),
        "Initialize should handle non-object options"
    );
}

#[test]
fn test_did_open_multiple_files_same_name_different_paths() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri1 = Url::parse("file:///dir1/test.sysml").unwrap();
    let uri2 = Url::parse("file:///dir2/test.sysml").unwrap();

    let params1 = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri1.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    };

    let params2 = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri2.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Engine;".to_string(),
        },
    };

    let _ = state.did_open(params1);
    let _ = state.did_open(params2);

    assert_eq!(state.server.workspace().file_count(), 2);
}

#[test]
fn test_did_change_with_empty_changes() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let _ = state.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    });

    // Capture initial state
    let initial_text = state.server.get_document_text(&uri);

    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![], // Empty changes
    };

    let result = state.did_change(change_params);
    assert!(matches!(result, ControlFlow::Continue(())));

    // Verify document text remains unchanged
    let final_text = state.server.get_document_text(&uri);
    assert_eq!(
        initial_text, final_text,
        "Document text should remain unchanged with empty changes"
    );
}

#[test]
fn test_did_change_full_document_replacement() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let _ = state.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    });

    // Full document replacement (no range)
    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "package NewPackage {\n  part def Engine;\n}".to_string(),
        }],
    };

    let result = state.did_change(change_params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_close_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri },
    };

    let result = state.did_close(params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_close_already_closed_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let _ = state.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    });

    // Close once
    let _ = state.did_close(DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
    });

    // Close again
    let result = state.did_close(DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier { uri },
    });

    assert!(matches!(result, ControlFlow::Continue(())));
}

#[tokio::test]
async fn test_hover_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 0,
                character: 0,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.hover(params).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_definition_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 0,
                character: 0,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.definition(params).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_references_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = ReferenceParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 0,
                character: 0,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: ReferenceContext {
            include_declaration: true,
        },
    };

    let result = state.references(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_completion_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 0,
                character: 0,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    let result = state.completion(params).await;
    assert!(result.is_ok());
    // Completion always returns Some result
    assert!(result.unwrap().is_some());
}

#[tokio::test]
async fn test_rename_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = RenameParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position {
                line: 0,
                character: 0,
            },
        },
        new_name: "NewName".to_string(),
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.rename(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_prepare_rename_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = TextDocumentPositionParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position {
            line: 0,
            character: 0,
        },
    };

    let result = state.prepare_rename(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_semantic_tokens_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = SemanticTokensParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.semantic_tokens_full(params).await;
    assert!(result.is_ok());
}

#[test]
fn test_did_change_on_unopened_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///nonexistent.sysml").unwrap();
    let params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri, version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "part def Vehicle;".to_string(),
        }],
    };

    let result = state.did_change(params);
    // Should not crash, should return Continue
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[tokio::test]
async fn test_sequential_operations_on_same_document() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    // Perform multiple operations sequentially - hover and definition both work synchronously
    let hover_result = state
        .hover(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 0,
                    character: 12,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await;

    let definition_result = state
        .definition(GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position {
                    line: 0,
                    character: 12,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
        })
        .await;

    assert!(hover_result.is_ok());
    assert!(definition_result.is_ok());
}

#[tokio::test]
async fn test_operations_after_document_error() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    // Open with invalid syntax
    state.server.open_document(&uri, "invalid !@#$ syntax").ok();

    // Operations should still work even if document had errors
    let hover_result = state
        .hover(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: Position {
                    line: 0,
                    character: 0,
                },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        })
        .await;

    assert!(hover_result.is_ok());
}

#[tokio::test]
async fn test_initialize_with_all_options_false() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///empty.sysml").unwrap();
    state.server.open_document(&uri, "").unwrap();

    let params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        options: FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.formatting(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_formatting_with_large_tab_size() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        options: FormattingOptions {
            tab_size: 100,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.formatting(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_formatting_with_tabs() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = DocumentFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        options: FormattingOptions {
            tab_size: 4,
            insert_spaces: false, // Use tabs
            ..Default::default()
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = state.formatting(params).await;
    assert!(result.is_ok());
}

#[test]
fn test_did_save_with_text() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let params = DidSaveTextDocumentParams {
        text_document: TextDocumentIdentifier { uri },
        text: Some("part def Vehicle;".to_string()),
    };

    let result = state.did_save(params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[tokio::test]
async fn test_selection_range_multiple_positions() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();

    let params = SelectionRangeParams {
        text_document: TextDocumentIdentifier { uri },
        positions: vec![
            Position {
                line: 0,
                character: 0,
            },
            Position {
                line: 0,
                character: 5,
            },
            Position {
                line: 0,
                character: 12,
            },
        ],
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.selection_range(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_document_symbol_after_error_recovery() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    // Start with invalid content
    state.server.open_document(&uri, "invalid syntax").ok();

    // Update to valid content
    let _ = state.did_change(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "part def Vehicle;".to_string(),
        }],
    });

    // Parse the change
    state.server.parse_document(&uri);

    let params = DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let result = state.document_symbol(params).await;
    assert!(result.is_ok());
}

#[test]
fn test_rapid_document_changes() {
    let (mut state, _parse_rx) = create_test_server_state();

    let uri = Url::parse("file:///test.sysml").unwrap();
    let _ = state.did_open(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "sysml".to_string(),
            version: 1,
            text: "part def Vehicle;".to_string(),
        },
    });

    // Simulate rapid typing
    for i in 2..10 {
        let change_params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: i,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 0,
                        character: 16,
                    },
                    end: Position {
                        line: 0,
                        character: 16,
                    },
                }),
                range_length: None,
                text: format!("\npart def Part{i};"),
            }],
        };

        let result = state.did_change(change_params);
        assert!(matches!(result, ControlFlow::Continue(())));
    }
}

#[tokio::test]
async fn test_initialize_replaces_server_instance() {
    let (mut state, _parse_rx) = create_test_server_state();

    // Open a document
    let uri = Url::parse("file:///test.sysml").unwrap();
    state
        .server
        .open_document(&uri, "part def Vehicle;")
        .unwrap();
    assert_eq!(state.server.workspace().file_count(), 1);

    // Initialize - this replaces the server instance
    let params = InitializeParams::default();
    let _ = state.initialize(params).await;

    // After initialization, the server is replaced with a new instance
    assert_eq!(state.server.workspace().file_count(), 0);
}

// =============================================================================
// Tests for workspace/didChangeWatchedFiles notification (Issue: server crash)
// =============================================================================

#[test]
fn test_did_change_watched_files_does_not_crash() {
    let (mut state, _parse_rx) = create_test_server_state();

    // Simulate VS Code sending didChangeWatchedFiles when a new file is created
    let params = DidChangeWatchedFilesParams {
        changes: vec![FileEvent {
            uri: Url::parse("file:///workspaces/test/NewFile.sysml").unwrap(),
            typ: FileChangeType::CREATED,
        }],
    };

    // This should not panic - previously it crashed with "Unhandled notification"
    let result = state.did_change_watched_files(params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_change_watched_files_file_deleted() {
    let (mut state, _parse_rx) = create_test_server_state();

    let params = DidChangeWatchedFilesParams {
        changes: vec![FileEvent {
            uri: Url::parse("file:///workspaces/test/DeletedFile.sysml").unwrap(),
            typ: FileChangeType::DELETED,
        }],
    };

    let result = state.did_change_watched_files(params);
    assert!(matches!(result, ControlFlow::Continue(())));
}

#[test]
fn test_did_change_watched_files_multiple_changes() {
    let (mut state, _parse_rx) = create_test_server_state();

    let params = DidChangeWatchedFilesParams {
        changes: vec![
            FileEvent {
                uri: Url::parse("file:///workspaces/test/File1.sysml").unwrap(),
                typ: FileChangeType::CREATED,
            },
            FileEvent {
                uri: Url::parse("file:///workspaces/test/File2.sysml").unwrap(),
                typ: FileChangeType::CHANGED,
            },
            FileEvent {
                uri: Url::parse("file:///workspaces/test/File3.kerml").unwrap(),
                typ: FileChangeType::DELETED,
            },
        ],
    };

    let result = state.did_change_watched_files(params);
    assert!(matches!(result, ControlFlow::Continue(())));
}
