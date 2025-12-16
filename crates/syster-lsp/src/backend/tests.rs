use super::*;

#[test]
fn test_backend_creation() {
    let backend = Backend::new();
    assert_eq!(backend.workspace().file_count(), 0);
}

#[test]
fn test_backend_provides_workspace_access() {
    let mut backend = Backend::new();

    // Should be able to access workspace mutably
    let workspace = backend.workspace_mut();
    assert_eq!(workspace.file_count(), 0);

    // Should be able to access workspace immutably
    let workspace = backend.workspace();
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_open_sysml_document() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    backend.open_document(&uri, text).unwrap();

    assert_eq!(backend.workspace().file_count(), 1);
    assert!(backend.workspace().symbol_table().all_symbols().len() > 0);
}

#[test]
fn test_open_invalid_sysml() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "invalid syntax !@#$%";

    // Should succeed (errors are captured, not returned)
    let result = backend.open_document(&uri, text);
    assert!(result.is_ok());

    // File should NOT be added to workspace (parse failed)
    assert_eq!(backend.workspace().file_count(), 0);

    // Should have diagnostics
    let diagnostics = backend.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty());
    assert!(diagnostics[0].message.len() > 0);
}

#[test]
fn test_open_unsupported_extension() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.txt").unwrap();
    let text = "some text";

    let result = backend.open_document(&uri, text);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unsupported file extension"));
}

#[test]
fn test_open_kerml_file() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.kerml").unwrap();
    let text = "classifier Vehicle;";

    let result = backend.open_document(&uri, text);
    // KerML not yet supported
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("KerML"));
}

#[test]
fn test_change_document() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open initial document
    backend.open_document(&uri, "part def Car;").unwrap();
    assert_eq!(backend.workspace().file_count(), 1);
    let initial_symbols = backend.workspace().symbol_table().all_symbols().len();

    // Change document content
    backend
        .change_document(&uri, "part def Vehicle; part def Bike;")
        .unwrap();

    assert_eq!(backend.workspace().file_count(), 1);
    let updated_symbols = backend.workspace().symbol_table().all_symbols().len();
    assert!(updated_symbols > initial_symbols);
}

#[test]
fn test_change_document_with_error() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open valid document
    backend.open_document(&uri, "part def Car;").unwrap();
    assert_eq!(backend.workspace().file_count(), 1);

    // Change to invalid content - should succeed but capture error
    let result = backend.change_document(&uri, "invalid syntax !@#");
    assert!(result.is_ok());

    // File should be removed from workspace (parse failed)
    assert_eq!(backend.workspace().file_count(), 0);

    // Should have diagnostics
    let diagnostics = backend.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty());
}

#[test]
fn test_change_nonexistent_document() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Try to change a document that was never opened
    let result = backend.change_document(&uri, "part def Car;");
    // Should succeed - change_document handles both open and update
    assert!(result.is_ok());
}

#[test]
fn test_close_document() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open and close document
    backend.open_document(&uri, "part def Car;").unwrap();
    backend.close_document(&uri).unwrap();

    // Document should still be in workspace (we keep it for cross-file refs)
    assert_eq!(backend.workspace().file_count(), 1);
}

#[test]
fn test_get_diagnostics_for_valid_file() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    backend.open_document(&uri, text).unwrap();

    let diagnostics = backend.get_diagnostics(&uri);
    assert!(
        diagnostics.is_empty(),
        "Valid file should have no diagnostics"
    );
}

#[test]
fn test_get_diagnostics_for_parse_error() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def invalid syntax";

    backend.open_document(&uri, text).unwrap();

    let diagnostics = backend.get_diagnostics(&uri);
    assert!(
        !diagnostics.is_empty(),
        "Should have parse error diagnostic"
    );
    assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    assert!(diagnostics[0].message.len() > 0);
}

#[test]
fn test_get_diagnostics_clears_on_fix() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open with error
    backend.open_document(&uri, "invalid syntax").unwrap();
    let diagnostics = backend.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty());

    // Fix the error
    backend.change_document(&uri, "part def Car;").unwrap();
    let diagnostics = backend.get_diagnostics(&uri);
    assert!(
        diagnostics.is_empty(),
        "Diagnostics should be cleared after fix"
    );
}

#[test]
fn test_get_diagnostics_for_nonexistent_file() {
    let backend = Backend::new();
    let uri = Url::parse("file:///nonexistent.sysml").unwrap();

    let diagnostics = backend.get_diagnostics(&uri);
    assert!(
        diagnostics.is_empty(),
        "Nonexistent file should have no diagnostics"
    );
}

#[test]
fn test_extract_word_at_position() {
    let text = "part def Vehicle;";

    // Position on "Vehicle"
    let word = extract_word_at_position(
        text,
        Position {
            line: 0,
            character: 9,
        },
    );
    assert_eq!(word, Some("Vehicle".to_string()));

    // Position on "part"
    let word = extract_word_at_position(
        text,
        Position {
            line: 0,
            character: 0,
        },
    );
    assert_eq!(word, Some("part".to_string()));

    // Position on "def"
    let word = extract_word_at_position(
        text,
        Position {
            line: 0,
            character: 5,
        },
    );
    assert_eq!(word, Some("def".to_string()));

    // Position on whitespace
    let word = extract_word_at_position(
        text,
        Position {
            line: 0,
            character: 4,
        },
    );
    assert_eq!(word, None);

    // Position on semicolon
    let word = extract_word_at_position(
        text,
        Position {
            line: 0,
            character: 16,
        },
    );
    assert_eq!(word, None);
}

#[test]
fn test_hover_on_symbol() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    backend.open_document(&uri, text).unwrap();

    // Hover on "Vehicle"
    let hover = backend.get_hover(
        &uri,
        Position {
            line: 0,
            character: 9,
        },
    );
    assert!(hover.is_some());

    let hover = hover.unwrap();
    if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
        assert!(content.contains("Vehicle"));
        // Symbol table stores "Part" (capitalized kind)
        assert!(content.contains("Part def"));
    } else {
        panic!("Expected scalar string content");
    }
}

#[test]
fn test_hover_on_whitespace() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    backend.open_document(&uri, text).unwrap();

    // Hover on whitespace
    let hover = backend.get_hover(
        &uri,
        Position {
            line: 0,
            character: 4,
        },
    );
    assert!(hover.is_none(), "Should not get hover on whitespace");
}

#[test]
fn test_hover_on_unknown_symbol() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    backend.open_document(&uri, text).unwrap();

    // Hover on "part" (keyword, not in symbol table)
    let hover = backend.get_hover(
        &uri,
        Position {
            line: 0,
            character: 0,
        },
    );
    assert!(hover.is_none(), "Keywords should not have hover");
}

#[test]
fn test_hover_multiline() {
    let mut backend = Backend::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;\npart def Car;";

    backend.open_document(&uri, text).unwrap();

    // Hover on "Car" on line 2
    let hover = backend.get_hover(
        &uri,
        Position {
            line: 1,
            character: 9,
        },
    );
    assert!(hover.is_some());

    let hover = hover.unwrap();
    if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
        assert!(content.contains("Car"));
    } else {
        panic!("Expected scalar string content");
    }
}
