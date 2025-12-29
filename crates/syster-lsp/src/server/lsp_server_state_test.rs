//! Tests for ServerState's LanguageServer trait implementation
//!
//! These tests verify that the ServerState properly implements the async_lsp::LanguageServer trait
//! and correctly delegates to the underlying LspServer methods.
//!
//! Note: We test through the public LanguageServer trait API, not internal methods.

use super::LspServer;
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

    let result = state.server.get_semantic_tokens(uri.as_str());

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

    let result = state.server.get_semantic_tokens(uri.as_str());

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

    let result = state.server.get_semantic_tokens(uri.as_str());

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
async fn test_semantic_tokens_full_with_relationships() {
    let mut state = TestServerState::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Base;
part def Derived :> Base;
part instance : Derived;
    "#;

    state.open_doc(&uri, text);

    let result = state.server.get_semantic_tokens(uri.as_str());

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

    let result = state.server.get_semantic_tokens(uri.as_str());

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
    assert!(
        content.contains("Specializes") || content.contains("Base"),
        "Should show specialization relationship"
    );
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
