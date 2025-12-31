//! Integration tests for call hierarchy LSP feature

use async_lsp::lsp_types::*;
use std::path::PathBuf;
use syster_lsp::LspServer;

fn create_test_server() -> LspServer {
    LspServer::with_config(false, None)
}

fn create_temp_file(content: &str, name: &str) -> (PathBuf, Url, String) {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(name);
    std::fs::write(&file_path, content).expect("Failed to write temp file");
    let abs_path = std::fs::canonicalize(&file_path).expect("Failed to canonicalize path");
    let uri = Url::from_file_path(&abs_path).expect("Failed to convert to URL");
    (abs_path, uri, content.to_string())
}

#[test]
fn test_prepare_call_hierarchy_on_action_definition() {
    let mut server = create_test_server();

    let content = r#"action def ProcessData; action def ExecuteTask;"#;

    let (_path, uri, text) = create_temp_file(content, "test_prepare.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Find "ProcessData" position
    let params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 15,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = server.prepare_call_hierarchy(&params);
    assert!(result.is_some(), "Should prepare call hierarchy");

    let items = result.unwrap();
    assert_eq!(items.len(), 1, "Should return one item");
    assert_eq!(items[0].name, "ProcessData", "Should find ProcessData");
    assert_eq!(items[0].kind, SymbolKind::FUNCTION);
}

#[test]
fn test_prepare_call_hierarchy_on_non_callable() {
    let mut server = create_test_server();

    let content = r#"part def Vehicle;"#;

    let (_path, uri, text) = create_temp_file(content, "test_non_callable.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Try to prepare call hierarchy on "Vehicle" (not callable)
    let params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 14,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let result = server.prepare_call_hierarchy(&params);
    assert!(result.is_none(), "Should not prepare for non-callable");
}

#[test]
fn test_incoming_calls_single_caller() {
    let mut server = create_test_server();

    let content = r#"action def PerformAction; action def Caller { perform PerformAction; }"#;

    let (_path, uri, text) = create_temp_file(content, "test_incoming.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // First prepare call hierarchy on PerformAction
    let prepare_params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 15,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let prepare_result = server.prepare_call_hierarchy(&prepare_params);
    assert!(prepare_result.is_some(), "Should prepare call hierarchy");

    let items = prepare_result.unwrap();
    let item = &items[0];

    // Now get incoming calls
    let incoming_params = CallHierarchyIncomingCallsParams {
        item: item.clone(),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let incoming_result = server.incoming_calls(&incoming_params);
    assert!(incoming_result.is_some(), "Should get incoming calls");

    let incoming = incoming_result.unwrap();
    assert_eq!(incoming.len(), 1, "Should have one caller");
    assert_eq!(incoming[0].from.name, "Caller");
}

#[test]
fn test_incoming_calls_no_callers() {
    let mut server = create_test_server();

    let content = r#"action def UnusedAction;"#;

    let (_path, uri, text) = create_temp_file(content, "test_no_incoming.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Prepare call hierarchy
    let prepare_params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 15,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let prepare_result = server.prepare_call_hierarchy(&prepare_params);
    let items = prepare_result.unwrap();
    let item = &items[0];

    // Get incoming calls
    let incoming_params = CallHierarchyIncomingCallsParams {
        item: item.clone(),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let incoming_result = server.incoming_calls(&incoming_params);
    assert!(incoming_result.is_some(), "Should return empty result");

    let incoming = incoming_result.unwrap();
    assert_eq!(incoming.len(), 0, "Should have no callers");
}

#[test]
fn test_outgoing_calls_single_callee() {
    let mut server = create_test_server();

    let content = r#"action def SubAction; action def MainAction { perform SubAction; }"#;

    let (_path, uri, text) = create_temp_file(content, "test_outgoing.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Prepare call hierarchy on MainAction
    let prepare_params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 35,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let prepare_result = server.prepare_call_hierarchy(&prepare_params);
    assert!(prepare_result.is_some(), "Should prepare call hierarchy");

    let items = prepare_result.unwrap();
    let item = &items[0];

    // Get outgoing calls
    let outgoing_params = CallHierarchyOutgoingCallsParams {
        item: item.clone(),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let outgoing_result = server.outgoing_calls(&outgoing_params);
    assert!(outgoing_result.is_some(), "Should get outgoing calls");

    let outgoing = outgoing_result.unwrap();
    assert_eq!(outgoing.len(), 1, "Should have one callee");
    assert_eq!(outgoing[0].to.name, "SubAction");
}

#[test]
fn test_outgoing_calls_no_callees() {
    let mut server = create_test_server();

    let content = r#"action def LeafAction;"#;

    let (_path, uri, text) = create_temp_file(content, "test_no_outgoing.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Prepare call hierarchy
    let prepare_params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 15,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let prepare_result = server.prepare_call_hierarchy(&prepare_params);
    let items = prepare_result.unwrap();
    let item = &items[0];

    // Get outgoing calls
    let outgoing_params = CallHierarchyOutgoingCallsParams {
        item: item.clone(),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let outgoing_result = server.outgoing_calls(&outgoing_params);
    assert!(outgoing_result.is_some(), "Should return empty result");

    let outgoing = outgoing_result.unwrap();
    assert_eq!(outgoing.len(), 0, "Should have no callees");
}

#[test]
fn test_call_hierarchy_multiple_callers() {
    let mut server = create_test_server();

    let content = r#"action def SharedAction; action def Caller1 { perform SharedAction; } action def Caller2 { perform SharedAction; }"#;

    let (_path, uri, text) = create_temp_file(content, "test_multiple_callers.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Prepare call hierarchy on SharedAction
    let prepare_params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 15,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let prepare_result = server.prepare_call_hierarchy(&prepare_params);
    let items = prepare_result.unwrap();
    let item = &items[0];

    // Get incoming calls
    let incoming_params = CallHierarchyIncomingCallsParams {
        item: item.clone(),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let incoming_result = server.incoming_calls(&incoming_params);
    assert!(incoming_result.is_some(), "Should get incoming calls");

    let incoming = incoming_result.unwrap();
    assert_eq!(incoming.len(), 2, "Should have two callers");

    let caller_names: Vec<_> = incoming.iter().map(|c| c.from.name.as_str()).collect();
    assert!(caller_names.contains(&"Caller1"));
    assert!(caller_names.contains(&"Caller2"));
}

#[test]
fn test_call_hierarchy_multiple_callees() {
    let mut server = create_test_server();

    let content = r#"action def SubAction1; action def SubAction2; action def MainAction { perform SubAction1; perform SubAction2; }"#;

    let (_path, uri, text) = create_temp_file(content, "test_multiple_callees.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Prepare call hierarchy on MainAction
    let prepare_params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 62,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let prepare_result = server.prepare_call_hierarchy(&prepare_params);
    let items = prepare_result.unwrap();
    let item = &items[0];

    // Get outgoing calls
    let outgoing_params = CallHierarchyOutgoingCallsParams {
        item: item.clone(),
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
    };

    let outgoing_result = server.outgoing_calls(&outgoing_params);
    assert!(outgoing_result.is_some(), "Should get outgoing calls");

    let outgoing = outgoing_result.unwrap();
    assert_eq!(outgoing.len(), 2, "Should have two callees");

    let callee_names: Vec<_> = outgoing.iter().map(|c| c.to.name.as_str()).collect();
    assert!(callee_names.contains(&"SubAction1"));
    assert!(callee_names.contains(&"SubAction2"));
}

#[test]
fn test_call_hierarchy_action_usage() {
    let mut server = create_test_server();

    let content = r#"action def ActionDef; action myAction { perform ActionDef; }"#;

    let (_path, uri, text) = create_temp_file(content, "test_action_usage.sysml");
    server
        .open_document(&uri, &text)
        .expect("Failed to open document");

    // Prepare call hierarchy on action usage (myAction)
    let prepare_params = CallHierarchyPrepareParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
            position: Position {
                line: 0,
                character: 30,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let prepare_result = server.prepare_call_hierarchy(&prepare_params);
    assert!(prepare_result.is_some(), "Should prepare for action usage");

    let items = prepare_result.unwrap();
    assert_eq!(items[0].name, "myAction");
}
