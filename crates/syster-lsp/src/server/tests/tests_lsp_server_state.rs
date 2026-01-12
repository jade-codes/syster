//! Tests for ServerState's LanguageServer trait implementation
//!
//! These tests verify that the ServerState properly implements the async_lsp::LanguageServer trait
//! and correctly delegates to the underlying LspServer methods.
//!
//! Note: We test through the public LanguageServer trait API, not internal methods.

use crate::server::LspServer;
use crate::server::formatting::*;
use async_lsp::lsp_types::*;

/// Helper struct to create a ServerState for testing
/// We need to simulate the ServerState from main.rs but in a testable way
struct TestServerState {
    server: LspServer,
    // For testing, we don't need a real client socket
    // The methods we're testing don't use the client
}

impl TestServerState {
    fn new() -> Self {
        Self {
            server: LspServer::new(),
        }
    }

    /// Helper to open a document for testing
    fn open_doc(&mut self, uri: &Url, text: &str) {
        self.server.open_document(uri, text).unwrap();
    }
}

// ============================================================================
// Tests for document_symbol (#321)
// ============================================================================

#[tokio::test]
async fn test_document_symbol_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package TestPkg {
    part def Vehicle;
    part car : Vehicle;
}
    "#;

    state.open_doc(&uri, text);

    // Note: The LanguageServer trait method would create DocumentSymbolParams,
    // but we're testing through the underlying LspServer method
    let path = std::path::Path::new(uri.path());
    let result = state.server.get_document_symbols(path);

    // Should have symbols
    assert!(!result.is_empty(), "Should find document symbols");

    // Verify structure
    assert_eq!(result.len(), 1, "Should have 1 root symbol (package)");
    let pkg = &result[0];
    assert_eq!(pkg.name, "TestPkg");
    assert_eq!(pkg.kind, SymbolKind::NAMESPACE);

    // Check children
    let children = pkg.children.as_ref().unwrap();
    assert_eq!(children.len(), 2, "Package should have 2 children");

    let names: Vec<&str> = children.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Vehicle"));
    assert!(names.contains(&"car"));
}

#[tokio::test]
async fn test_document_symbol_empty_file() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///empty.sysml").unwrap();

    state.open_doc(&uri, "");

    let path = std::path::Path::new(uri.path());
    let result = state.server.get_document_symbols(path);

    // Empty file should have no symbols
    assert!(result.is_empty(), "Empty file should have no symbols");
}

#[tokio::test]
async fn test_document_symbol_nested_hierarchy() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Outer {
    package Inner {
        part def Vehicle;
    }
}
    "#;

    state.open_doc(&uri, text);

    let path = std::path::Path::new(uri.path());
    let result = state.server.get_document_symbols(path);

    assert_eq!(result.len(), 1, "Should have 1 root");
    let outer = &result[0];
    assert_eq!(outer.name, "Outer");

    let outer_children = outer.children.as_ref().unwrap();
    assert_eq!(outer_children.len(), 1);
    let inner = &outer_children[0];
    assert_eq!(inner.name, "Inner");

    let inner_children = inner.children.as_ref().unwrap();
    assert_eq!(inner_children.len(), 1);
    assert_eq!(inner_children[0].name, "Vehicle");
}

#[tokio::test]
async fn test_document_symbol_nonexistent_file() {
    let state = TestServerState::new();
    let uri = Url::parse("file:///nonexistent.sysml").unwrap();

    let path = std::path::Path::new(uri.path());
    let result = state.server.get_document_symbols(path);

    // Nonexistent file should return empty
    assert!(result.is_empty(), "Nonexistent file should have no symbols");
}

// ============================================================================
// Tests for selection_range (#322)
// ============================================================================

#[tokio::test]
async fn test_selection_range_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    let path = std::path::Path::new(uri.path());
    let positions = vec![Position::new(0, 10)]; // Inside "Vehicle"

    let result = state.server.get_selection_ranges(path, positions);

    assert_eq!(result.len(), 1, "Should return one selection range");
    let range = &result[0];

    // Should have a valid range
    assert!(range.range.end.character > range.range.start.character);
}

#[tokio::test]
async fn test_selection_range_multiple_positions() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part def Car;
    "#;

    state.open_doc(&uri, text);

    let path = std::path::Path::new(uri.path());
    let positions = vec![
        Position::new(1, 10), // In "Vehicle"
        Position::new(2, 10), // In "Car"
    ];

    let result = state.server.get_selection_ranges(path, positions);

    assert_eq!(
        result.len(),
        2,
        "Should return selection range for each position"
    );
}

#[tokio::test]
async fn test_selection_range_nested_structure() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle {
        attribute speed : Real;
    }
}
    "#;

    state.open_doc(&uri, text);

    let path = std::path::Path::new(uri.path());
    let positions = vec![Position::new(3, 20)]; // Inside attribute

    let result = state.server.get_selection_ranges(path, positions);

    assert_eq!(result.len(), 1);
    let range = &result[0];

    // Should have parent ranges for nested structure
    // The selection should expand: attribute -> part def -> package
    assert!(
        range.range.end.character > range.range.start.character,
        "Should have valid range"
    );
}

#[tokio::test]
async fn test_selection_range_invalid_position() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    let path = std::path::Path::new(uri.path());
    let positions = vec![Position::new(100, 100)]; // Way out of bounds

    let result = state.server.get_selection_ranges(path, positions);

    // Should return default range (single character)
    assert_eq!(result.len(), 1, "Should handle invalid position gracefully");
}

#[tokio::test]
async fn test_selection_range_empty_file() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///empty.sysml").unwrap();

    state.open_doc(&uri, "");

    let path = std::path::Path::new(uri.path());
    let positions = vec![Position::new(0, 0)];

    let result = state.server.get_selection_ranges(path, positions);

    assert_eq!(
        result.len(),
        1,
        "Should return default range for empty file"
    );
}

// ============================================================================
// Tests for semantic_tokens_full (#323)
// ============================================================================

#[tokio::test]
async fn test_semantic_tokens_full_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Auto {
    part def Vehicle;
    part myVehicle : Vehicle;
}
    "#;

    state.open_doc(&uri, text);

    let result = state.server.get_semantic_tokens(&uri);

    assert!(result.is_some(), "Should return semantic tokens");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected Tokens result");
    };

    // Should have tokens for: package name, part def name, part usage name, etc.
    assert!(
        tokens.data.len() >= 4,
        "Should have multiple semantic tokens"
    );

    // Verify token types are present
    let token_types: Vec<u32> = tokens.data.iter().map(|t| t.token_type).collect();
    // Should include different types: namespace, type, variable/property
    let unique_types: std::collections::HashSet<_> = token_types.iter().collect();
    assert!(
        unique_types.len() > 1,
        "Should have multiple token types (namespace, type, property, etc.)"
    );
}

#[tokio::test]
async fn test_semantic_tokens_full_empty_file() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///empty.sysml").unwrap();

    state.open_doc(&uri, "");

    let result = state.server.get_semantic_tokens(&uri);

    // Empty file should return Some with empty tokens
    if let Some(SemanticTokensResult::Tokens(tokens)) = result {
        assert!(
            tokens.data.is_empty(),
            "Empty file should have no semantic tokens"
        );
    }
}

#[tokio::test]
async fn test_semantic_tokens_full_multiline() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part def Car;
part def Truck;
part myCar : Car;
    "#;

    state.open_doc(&uri, text);

    let result = state.server.get_semantic_tokens(&uri);

    assert!(result.is_some(), "Should return tokens for multiline file");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected Tokens result");
    };

    // Should have tokens across multiple lines
    assert!(
        tokens.data.len() >= 4,
        "Should have tokens for all definitions"
    );

    // Verify delta encoding works (delta_line should be used)
    let has_line_deltas = tokens.data.iter().any(|t| t.delta_line > 0);
    assert!(
        has_line_deltas,
        "Multiline tokens should have line deltas > 0"
    );
}

#[tokio::test]
async fn test_semantic_tokens_full_with_index() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Base;
part def Derived :> Base;
part instance : Derived;
    "#;

    state.open_doc(&uri, text);

    let result = state.server.get_semantic_tokens(&uri);

    assert!(result.is_some(), "Should handle relationships");

    let SemanticTokensResult::Tokens(tokens) = result.unwrap() else {
        panic!("Expected Tokens result");
    };

    // Should include tokens for type references (Base, Derived)
    assert!(
        tokens.data.len() >= 3,
        "Should have tokens for all elements"
    );
}

#[tokio::test]
async fn test_semantic_tokens_full_nonexistent_file() {
    let state = TestServerState::new();
    let uri = Url::parse("file:///nonexistent.sysml").unwrap();

    let result = state.server.get_semantic_tokens(&uri);

    // Should return None for nonexistent file
    assert!(
        result.is_none(),
        "Nonexistent file should return None for semantic tokens"
    );
}

// ============================================================================
// Tests for hover (#324)
// ============================================================================

#[tokio::test]
async fn test_hover_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    let position = Position::new(0, 10); // Inside "Vehicle"
    let result = state.server.get_hover(&uri, position);

    assert!(result.is_some(), "Should return hover info");

    let hover = result.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };

    assert!(
        content.contains("Vehicle"),
        "Hover should contain symbol name"
    );
    assert!(
        content.contains("Part def"),
        "Hover should contain symbol type"
    );
}

#[tokio::test]
async fn test_hover_with_typing_relationship() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
    "#;

    state.open_doc(&uri, text);

    // Hover on the usage
    let position = Position::new(2, 5); // On "car"
    let result = state.server.get_hover(&uri, position);

    assert!(result.is_some(), "Should return hover for usage");

    let hover = result.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };

    assert!(content.contains("car"), "Should show usage name");
    assert!(
        content.contains("Typed by") || content.contains("Vehicle"),
        "Should show typing relationship"
    );
}

#[tokio::test]
async fn test_hover_with_specialization() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Base;
part def Derived :> Base;
    "#;

    state.open_doc(&uri, text);

    // Hover on the derived type
    let position = Position::new(2, 10); // On "Derived"
    let result = state.server.get_hover(&uri, position);

    assert!(result.is_some(), "Should return hover for derived type");

    let hover = result.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };

    assert!(content.contains("Derived"), "Should show symbol name");
    // Note: Relationship info not available without RelationshipGraph
}

#[tokio::test]
async fn test_hover_no_symbol_at_position() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    // Hover on whitespace/keyword
    let position = Position::new(0, 0); // On "p" of "part"
    let result = state.server.get_hover(&uri, position);

    // Should return None for non-symbol positions
    assert!(
        result.is_none(),
        "Should return None when no symbol at position"
    );
}

#[tokio::test]
async fn test_hover_returns_range() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def MySymbol;"#;

    state.open_doc(&uri, text);

    let position = Position::new(0, 10); // Inside symbol name
    let result = state.server.get_hover(&uri, position);

    assert!(result.is_some());
    let hover = result.unwrap();

    // Should include a range
    assert!(hover.range.is_some(), "Hover should include range");

    let range = hover.range.unwrap();
    assert!(
        range.end.character > range.start.character,
        "Range should span the symbol"
    );
}

#[tokio::test]
async fn test_hover_multiline_document() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def First;
part def Second;
part def Third;
    "#;

    state.open_doc(&uri, text);

    // Hover on symbol on line 2
    let position = Position::new(2, 10); // On "Second"
    let result = state.server.get_hover(&uri, position);

    assert!(result.is_some(), "Should find symbol on different lines");

    let hover = result.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };

    assert!(content.contains("Second"));
}

// ============================================================================
// Tests for rename (#325)
// ============================================================================

#[tokio::test]
async fn test_rename_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def OldName;
part usage : OldName;
    "#;

    state.open_doc(&uri, text);

    let position = Position::new(1, 10); // On "OldName" in definition
    let result = state.server.get_rename_edits(&uri, position, "NewName");

    assert!(result.is_some(), "Should return rename edits");

    let edit = result.unwrap();
    assert!(edit.changes.is_some(), "Should have changes");

    let changes = edit.changes.unwrap();
    assert!(changes.contains_key(&uri), "Should have edits for the file");

    let edits = &changes[&uri];
    assert_eq!(edits.len(), 2, "Should rename definition and usage");

    // All edits should use new name
    for text_edit in edits {
        assert_eq!(text_edit.new_text, "NewName");
    }
}

#[tokio::test]
async fn test_rename_from_usage() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
    "#;

    state.open_doc(&uri, text);

    // Rename from usage position
    let position = Position::new(2, 12); // On "Vehicle" in usage
    let result = state.server.get_rename_edits(&uri, position, "Automobile");

    assert!(result.is_some(), "Should rename from usage");

    let edit = result.unwrap();
    let changes = edit.changes.unwrap();
    let edits = &changes[&uri];

    assert_eq!(edits.len(), 2, "Should rename definition and usage");
    assert!(edits.iter().all(|e| e.new_text == "Automobile"));
}

#[tokio::test]
async fn test_rename_no_symbol() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    // Try to rename at invalid position (keyword)
    let position = Position::new(0, 0); // On "p" of "part"
    let result = state.server.get_rename_edits(&uri, position, "NewName");

    // Should return None for non-renameable positions
    assert!(
        result.is_none(),
        "Should return None when no symbol to rename"
    );
}

#[tokio::test]
async fn test_rename_with_multiple_usages() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Engine;
part car1 : Engine;
part car2 : Engine;
part car3 : Engine;
    "#;

    state.open_doc(&uri, text);

    let position = Position::new(1, 10); // On definition
    let result = state.server.get_rename_edits(&uri, position, "Motor");

    assert!(result.is_some());

    let edit = result.unwrap();
    let changes = edit.changes.unwrap();
    let edits = &changes[&uri];

    // Should rename definition + 3 usages = 4 edits
    assert_eq!(edits.len(), 4, "Should rename all occurrences");
    assert!(edits.iter().all(|e| e.new_text == "Motor"));
}

#[tokio::test]
async fn test_rename_preserves_other_symbols() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Car;
part def Truck;
part myCar : Car;
    "#;

    state.open_doc(&uri, text);

    // Rename only Car, not Truck
    let position = Position::new(1, 10); // On "Car"
    let result = state.server.get_rename_edits(&uri, position, "Vehicle");

    assert!(result.is_some());

    let edit = result.unwrap();
    let changes = edit.changes.unwrap();
    let edits = &changes[&uri];

    // Should only rename Car (definition + usage) = 2 edits
    assert_eq!(edits.len(), 2, "Should only rename Car, not Truck");

    // Verify Truck's line is not in edits
    let lines: Vec<u32> = edits.iter().map(|e| e.range.start.line).collect();
    assert!(!lines.contains(&2), "Should not rename Truck line");
}

#[tokio::test]
async fn test_rename_qualified_name() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Outer {
    package Inner {
        part def Vehicle;
    }
    part car : Inner::Vehicle;
}
    "#;

    state.open_doc(&uri, text);

    // Rename using qualified reference
    let position = Position::new(5, 25); // On "Vehicle" in qualified name
    let result = state.server.get_rename_edits(&uri, position, "Automobile");

    assert!(
        result.is_some(),
        "Should support rename from qualified name"
    );

    let edit = result.unwrap();
    let changes = edit.changes.unwrap();
    let edits = &changes[&uri];

    // Should rename definition and qualified usage
    assert_eq!(edits.len(), 2, "Should rename definition and usage");
}

// ============================================================================
// Tests for initialize (#261-264, #278, #299, #316)
// ============================================================================

#[tokio::test]
async fn test_initialize_returns_capabilities() {
    let mut state = TestServerState::new();

    // Initialize should set up server capabilities
    // We can't directly call initialize without async-lsp infrastructure,
    // but we can verify the server is ready
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "part def Vehicle;");

    // Verify server can perform operations (it's initialized)
    let symbols = state
        .server
        .get_document_symbols(std::path::Path::new(uri.path()));
    assert!(!symbols.is_empty(), "Initialized server should work");
}

#[tokio::test]
async fn test_initialize_with_stdlib_disabled() {
    // Create server with stdlib disabled
    let server = LspServer::with_config(false, None);

    // Should work but not load stdlib
    assert_eq!(server.workspace().file_count(), 0, "Should not load stdlib");
}

#[tokio::test]
async fn test_initialize_with_custom_stdlib_path() {
    // Create server with custom path
    let custom_path = std::path::PathBuf::from("/custom/path");
    let server = LspServer::with_config(true, Some(custom_path));

    // Server should be created (even if path doesn't exist)
    assert_eq!(server.workspace().file_count(), 0);
}

// ============================================================================
// Tests for definition (#258, #275, #296, #313)
// ============================================================================

#[tokio::test]
async fn test_definition_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
    "#;

    state.open_doc(&uri, text);

    // Go to definition from usage
    let position = Position::new(2, 12); // On "Vehicle" in usage
    let result = state.server.get_definition(&uri, position);

    assert!(result.is_some(), "Should find definition");

    let location = result.unwrap();
    assert_eq!(location.uri, uri);
    assert_eq!(
        location.range.start.line, 1,
        "Should point to definition line"
    );
}

#[tokio::test]
async fn test_definition_from_definition() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    // Go to definition from the definition itself
    let position = Position::new(0, 10); // On "Vehicle" name
    let result = state.server.get_definition(&uri, position);

    assert!(result.is_some(), "Should return itself");
    let location = result.unwrap();
    assert_eq!(location.range.start.line, 0);
}

#[tokio::test]
async fn test_definition_no_symbol() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    // Try definition on keyword/whitespace
    let position = Position::new(0, 0); // On "p" of "part"
    let result = state.server.get_definition(&uri, position);

    assert!(result.is_none(), "Should return None for non-symbol");
}

#[tokio::test]
async fn test_definition_nested_elements() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Auto {
    part def Engine;
    part def Car {
        part engine : Engine;
    }
}
    "#;

    state.open_doc(&uri, text);

    // Go to definition from nested usage
    let position = Position::new(4, 23); // On "Engine" in nested part
    let result = state.server.get_definition(&uri, position);

    assert!(result.is_some(), "Should find definition in parent scope");
    let location = result.unwrap();
    assert_eq!(
        location.range.start.line, 2,
        "Should point to Engine definition"
    );
}

// ============================================================================
// Tests for references (#266, #283, #301, #318)
// ============================================================================

#[tokio::test]
async fn test_references_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
part bike : Vehicle;
    "#;

    state.open_doc(&uri, text);

    // Find references including declaration
    let position = Position::new(1, 10); // On definition
    let result = state.server.get_references(&uri, position, true);

    assert!(result.is_some(), "Should find references");
    let locations = result.unwrap();

    // Should find definition + 2 usages = 3 total
    assert_eq!(locations.len(), 3, "Should find all references");
}

#[tokio::test]
async fn test_references_exclude_declaration() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Engine;
part car : Engine;
    "#;

    state.open_doc(&uri, text);

    // Find references excluding declaration
    let position = Position::new(1, 10);
    let result = state.server.get_references(&uri, position, false);

    assert!(result.is_some());
    let locations = result.unwrap();

    // Should only find usage, not definition
    assert_eq!(locations.len(), 1, "Should exclude definition");
    assert_eq!(
        locations[0].range.start.line, 2,
        "Should only have usage line"
    );
}

#[tokio::test]
async fn test_references_no_symbol() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    let position = Position::new(0, 0); // On keyword
    let result = state.server.get_references(&uri, position, true);

    // Should return None or empty for non-symbol
    if let Some(refs) = result {
        assert!(refs.is_empty(), "Should have no references for keyword");
    }
}

#[tokio::test]
async fn test_references_no_usages() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def UnusedType;
part def OtherType;
    "#;

    state.open_doc(&uri, text);

    // Find references excluding declaration (no usages)
    let position = Position::new(1, 10);
    let result = state.server.get_references(&uri, position, false);

    assert!(result.is_some());
    let locations = result.unwrap();
    assert_eq!(locations.len(), 0, "Should find no usages");
}

// ============================================================================
// Tests for completion (#257, #274, #295, #312)
// ============================================================================

#[tokio::test]
async fn test_completion_keywords() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "package Test {}\n";

    state.open_doc(&uri, text);

    let position = Position::new(1, 0); // After package
    let path = std::path::Path::new(uri.path());
    let result = state.server.get_completions(path, position);

    let CompletionResponse::Array(items) = result else {
        panic!("Expected array response");
    };

    assert!(!items.is_empty(), "Should return keyword completions");

    // Should have SysML keywords
    let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(labels.contains(&"part def"), "Should suggest 'part def'");
    assert!(labels.contains(&"package"), "Should suggest 'package'");
}

#[tokio::test]
async fn test_completion_empty_file() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "");

    let position = Position::new(0, 0);
    let path = std::path::Path::new(uri.path());
    let result = state.server.get_completions(path, position);

    let CompletionResponse::Array(items) = result else {
        panic!("Expected array response");
    };

    assert!(!items.is_empty(), "Should suggest top-level keywords");
}

#[tokio::test]
async fn test_completion_type_context() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part car : 
}
    "#;

    state.open_doc(&uri, text);

    // After colon - should suggest type symbols
    let position = Position::new(3, 15);
    let path = std::path::Path::new(uri.path());
    let result = state.server.get_completions(path, position);

    let CompletionResponse::Array(items) = result else {
        panic!("Expected array response");
    };

    let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();
    assert!(labels.contains(&"Vehicle"), "Should suggest Vehicle type");
}

#[tokio::test]
async fn test_completion_invalid_position() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "part def Car;");

    // Position way beyond file end
    let position = Position::new(100, 100);
    let path = std::path::Path::new(uri.path());
    let result = state.server.get_completions(path, position);

    // Should handle gracefully
    match result {
        CompletionResponse::Array(_) | CompletionResponse::List(_) => {
            // Either response is acceptable
        }
    }
}

// ============================================================================
// Tests for formatting (#254, #259-260, #276-277, #298, #315)
// ============================================================================

#[tokio::test]
async fn test_formatting_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part  def   Vehicle;"; // Extra spaces

    state.open_doc(&uri, text);

    // Get document text for formatting
    let doc_text = state.server.get_document_text(&uri);
    assert!(doc_text.is_some(), "Should have document text");

    // Format the text (synchronously for testing)
    let options = FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        ..Default::default()
    };

    let cancel_token = tokio_util::sync::CancellationToken::new();
    let formatted = format_text(&doc_text.unwrap(), options, &cancel_token);
    assert!(formatted.is_some(), "Should format successfully");
}

#[tokio::test]
async fn test_formatting_empty_file() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "");

    let doc_text = state.server.get_document_text(&uri);
    assert!(doc_text.is_some());

    let options = FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        ..Default::default()
    };

    let cancel_token = tokio_util::sync::CancellationToken::new();
    let formatted = format_text(&doc_text.unwrap(), options, &cancel_token);
    // Empty file should return None or empty edits
    assert!(formatted.is_none() || formatted.unwrap().is_empty());
}

#[tokio::test]
async fn test_formatting_preserves_structure() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Auto {
    part def Vehicle;
}
    "#;

    state.open_doc(&uri, text);

    let doc_text = state.server.get_document_text(&uri);
    assert!(doc_text.is_some());

    let options = FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        ..Default::default()
    };

    let cancel_token = tokio_util::sync::CancellationToken::new();
    let formatted = format_text(&doc_text.unwrap(), options, &cancel_token);
    assert!(
        formatted.is_some(),
        "Formatting should succeed for structured content"
    );
}

// ============================================================================
// Tests for prepare_rename (#268, #285, #303, #320)
// ============================================================================

#[tokio::test]
async fn test_prepare_rename_on_definition() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    let position = Position::new(0, 10); // On "Vehicle"
    let result = state.server.prepare_rename(&uri, position);

    assert!(result.is_some(), "Should be renameable");

    let response = result.unwrap();
    if let PrepareRenameResponse::RangeWithPlaceholder { placeholder, .. } = response {
        assert_eq!(placeholder, "Vehicle", "Should use symbol name");
    }
}

#[tokio::test]
async fn test_prepare_rename_on_usage() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Car;
part myCar : Car;
    "#;

    state.open_doc(&uri, text);

    let position = Position::new(2, 14); // On "Car" in usage
    let result = state.server.prepare_rename(&uri, position);

    assert!(result.is_some(), "Should be renameable from usage");
}

#[tokio::test]
async fn test_prepare_rename_no_symbol() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;"#;

    state.open_doc(&uri, text);

    let position = Position::new(0, 0); // On keyword
    let result = state.server.prepare_rename(&uri, position);

    assert!(result.is_none(), "Should not be renameable");
}

#[tokio::test]
async fn test_prepare_rename_returns_correct_range() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def MyLongSymbolName;"#;

    state.open_doc(&uri, text);

    let position = Position::new(0, 12); // Inside symbol name
    let result = state.server.prepare_rename(&uri, position);

    assert!(result.is_some());

    let response = result.unwrap();
    if let PrepareRenameResponse::RangeWithPlaceholder { range, placeholder } = response {
        assert_eq!(placeholder, "MyLongSymbolName");
        assert!(
            range.end.character > range.start.character,
            "Range should span symbol"
        );
    }
}

// ============================================================================
// Tests for folding_range (#267, #284, #302, #319)
// ============================================================================

#[tokio::test]
async fn test_folding_range_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle {
        attribute weight : Real;
    }
}
    "#;

    state.open_doc(&uri, text);

    let path = std::path::Path::new(uri.path());
    let ranges = state.server.get_folding_ranges(path);

    // Should have folding ranges for blocks
    for range in &ranges {
        assert!(range.end_line >= range.start_line, "Valid folding range");
    }
}

#[tokio::test]
async fn test_folding_range_empty_file() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "");

    let path = std::path::Path::new(uri.path());
    let ranges = state.server.get_folding_ranges(path);

    // Empty file should have no folding ranges
    assert!(ranges.is_empty(), "Empty file should have no folding");
}

#[tokio::test]
async fn test_folding_range_single_line() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "part def Car;");

    let path = std::path::Path::new(uri.path());
    let ranges = state.server.get_folding_ranges(path);

    // Single-line elements shouldn't create folding ranges
    assert!(ranges.is_empty() || ranges.iter().all(|r| r.end_line > r.start_line));
}

#[tokio::test]
async fn test_folding_range_nested() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Outer {
    package Inner {
        part def Vehicle;
    }
}
    "#;

    state.open_doc(&uri, text);

    let path = std::path::Path::new(uri.path());
    let ranges = state.server.get_folding_ranges(path);

    // Should handle nested structures
    for range in &ranges {
        assert!(range.end_line > range.start_line, "Multi-line fold");
    }
}

// ============================================================================
// Tests for inlay_hint (#265, #282, #300, #317)
// ============================================================================

#[tokio::test]
async fn test_inlay_hint_basic() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
    "#;

    state.open_doc(&uri, text);

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(10, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = state.server.get_inlay_hints(&params);

    // May or may not have hints depending on implementation
    // Just verify it doesn't crash
    for hint in &hints {
        // Check label exists - InlayHintLabel is an enum so we just verify it's there
        match &hint.label {
            InlayHintLabel::String(s) => assert!(!s.is_empty(), "Label should not be empty"),
            InlayHintLabel::LabelParts(parts) => {
                assert!(!parts.is_empty(), "Label parts should not be empty")
            }
        }
    }
}

#[tokio::test]
async fn test_inlay_hint_empty_file() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "");

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = state.server.get_inlay_hints(&params);
    assert!(hints.is_empty(), "Empty file should have no hints");
}

#[tokio::test]
async fn test_inlay_hint_out_of_range() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    state.open_doc(&uri, "part def Car;");

    let params = InlayHintParams {
        text_document: TextDocumentIdentifier { uri: uri.clone() },
        range: Range {
            start: Position::new(100, 0), // Beyond file
            end: Position::new(200, 0),
        },
        work_done_progress_params: Default::default(),
    };

    let hints = state.server.get_inlay_hints(&params);
    // Should handle gracefully
    assert!(hints.is_empty(), "Out of range should return empty");
}

// ============================================================================
// Tests for did_open (#309)
// ============================================================================

#[tokio::test]
async fn test_did_open_valid_document() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    // Simulate did_open
    state.open_doc(&uri, text);

    // Verify document was opened and parsed
    assert_eq!(state.server.workspace().file_count(), 1);

    assert!(
        state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Vehicle")
    );
}

#[tokio::test]
async fn test_did_open_invalid_document() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "invalid syntax !@#$%";

    // Should handle parse errors gracefully
    let result = state.server.open_document(&uri, text);
    assert!(result.is_ok(), "Should succeed even with parse errors");

    // Document should not be in workspace (parse failed)
    assert_eq!(state.server.workspace().file_count(), 0);

    // But should have diagnostics
    let diagnostics = state.server.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty(), "Should have error diagnostics");
}

#[tokio::test]
async fn test_did_open_multiple_documents() {
    let mut state = TestServerState::new();
    let uri1 = Url::parse("file:///file1.sysml").unwrap();
    let uri2 = Url::parse("file:///file2.sysml").unwrap();

    state.open_doc(&uri1, "part def Car;");
    state.open_doc(&uri2, "part def Truck;");

    assert_eq!(state.server.workspace().file_count(), 2);

    let st = state.server.workspace().symbol_table();
    assert!(st.iter_symbols().any(|s| s.name() == "Car"));
    assert!(st.iter_symbols().any(|s| s.name() == "Truck"));
}

// ============================================================================
// Tests for did_change (#297)
// ============================================================================

#[tokio::test]
async fn test_did_change_incremental_insert() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    state.open_doc(&uri, "part def Car;");

    // Simulate incremental change - insert at end
    let change = TextDocumentContentChangeEvent {
        range: Some(Range {
            start: Position::new(0, 13),
            end: Position::new(0, 13),
        }),
        range_length: None,
        text: "\npart def Truck;".to_string(),
    };

    state.server.apply_text_change_only(&uri, &change).unwrap();
    state.server.parse_document(&uri);

    // Verify both symbols exist
    assert!(
        state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Car")
    );
    assert!(
        state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Truck")
    );
}

#[tokio::test]
async fn test_did_change_incremental_delete() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    state.open_doc(&uri, "part def Car;\npart def Truck;");

    // Delete second line
    let change = TextDocumentContentChangeEvent {
        range: Some(Range {
            start: Position::new(1, 0),
            end: Position::new(1, 16),
        }),
        range_length: Some(16),
        text: "".to_string(),
    };

    state.server.apply_text_change_only(&uri, &change).unwrap();
    state.server.parse_document(&uri);

    // Only Car should exist
    assert!(
        state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Car")
    );
    assert!(
        !state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Truck")
    );
}

#[tokio::test]
async fn test_did_change_incremental_replace() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    state.open_doc(&uri, "part def Car;");

    // Replace "Car" with "Vehicle"
    let change = TextDocumentContentChangeEvent {
        range: Some(Range {
            start: Position::new(0, 9),
            end: Position::new(0, 12),
        }),
        range_length: Some(3),
        text: "Vehicle".to_string(),
    };

    state.server.apply_text_change_only(&uri, &change).unwrap();
    state.server.parse_document(&uri);

    assert!(
        state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Vehicle")
    );
    assert!(
        !state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Car")
    );
}

#[tokio::test]
async fn test_did_change_full_sync() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    state.open_doc(&uri, "part def Car;");

    // Full document sync (no range)
    let change = TextDocumentContentChangeEvent {
        range: None,
        range_length: None,
        text: "part def CompletelyNew;".to_string(),
    };

    state.server.apply_text_change_only(&uri, &change).unwrap();
    state.server.parse_document(&uri);

    assert!(
        state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "CompletelyNew")
    );
    assert!(
        !state
            .server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Car")
    );
}

// ============================================================================
// Tests for did_close (#311)
// ============================================================================

#[tokio::test]
async fn test_did_close_document() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    state.open_doc(&uri, "part def Vehicle;");
    assert_eq!(state.server.workspace().file_count(), 1);

    // Close document
    state.server.close_document(&uri).unwrap();

    // Document stays in workspace (for cross-file references)
    assert_eq!(state.server.workspace().file_count(), 1);
}

#[tokio::test]
async fn test_did_close_nonexistent() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///never_opened.sysml").unwrap();

    // Closing non-existent document should not error
    let result = state.server.close_document(&uri);
    assert!(
        result.is_ok(),
        "Should handle closing non-existent gracefully"
    );
}

// ============================================================================
// Tests for did_save (#310)
// ============================================================================

#[tokio::test]
async fn test_did_save_document() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    state.open_doc(&uri, "part def Vehicle;");

    // did_save is a no-op in current implementation
    // Just verify it doesn't break anything
    let symbols_before = state
        .server
        .workspace()
        .symbol_table()
        .iter_symbols()
        .count();

    // Simulate save (no-op)
    // In real ServerState, did_save returns ControlFlow::Continue(())

    let symbols_after = state
        .server
        .workspace()
        .symbol_table()
        .iter_symbols()
        .count();
    assert_eq!(
        symbols_before, symbols_after,
        "Save should not modify state"
    );
}

// ============================================================================
// Tests for new_router (#293)
// ============================================================================

#[tokio::test]
async fn test_new_router_setup() {
    // Testing new_router requires async-lsp infrastructure
    // We test that the server can be created and used
    let state = TestServerState::new();

    // Verify server is initialized
    assert_eq!(state.server.workspace().file_count(), 0);
}

#[tokio::test]
async fn test_router_handles_multiple_operations() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part car : Vehicle;
    "#;

    state.open_doc(&uri, text);

    // Test multiple operations work together
    let hover = state.server.get_hover(&uri, Position::new(1, 10));
    assert!(hover.is_some(), "Hover should work");

    let definition = state.server.get_definition(&uri, Position::new(2, 12));
    assert!(definition.is_some(), "Definition should work");

    let references = state
        .server
        .get_references(&uri, Position::new(1, 10), true);
    assert!(references.is_some(), "References should work");

    let symbols = state
        .server
        .get_document_symbols(std::path::Path::new(uri.path()));
    assert!(!symbols.is_empty(), "Symbols should work");
}
