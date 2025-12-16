use super::LspServer;
use syster::core::constants::REL_TYPING;
use tower_lsp::lsp_types::{DiagnosticSeverity, HoverContents, MarkedString, Position, Url};

#[test]
fn test_server_creation() {
    let server = LspServer::new();
    assert_eq!(server.workspace().file_count(), 0);
}

#[test]
fn test_open_sysml_document() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    assert_eq!(server.workspace().file_count(), 1);
    assert!(!server.workspace().symbol_table().all_symbols().is_empty());
}

#[test]
fn test_open_invalid_sysml() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "invalid syntax !@#$%";

    // Should succeed (errors are captured, not returned)
    let result = server.open_document(&uri, text);
    assert!(result.is_ok());

    // File should NOT be added to workspace (parse failed)
    assert_eq!(server.workspace().file_count(), 0);

    // Should have diagnostics
    let diagnostics = server.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty());
    assert!(!diagnostics[0].message.is_empty());
}

#[test]
fn test_open_unsupported_extension() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.txt").unwrap();
    let text = "some text";

    let result = server.open_document(&uri, text);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unsupported file extension"));
}

#[test]
fn test_open_kerml_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.kerml").unwrap();
    let text = "classifier Vehicle;";

    let result = server.open_document(&uri, text);
    // KerML not yet supported
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("KerML"));
}

#[test]
fn test_change_document() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open initial document
    server.open_document(&uri, "part def Car;").unwrap();
    assert_eq!(server.workspace().file_count(), 1);
    let initial_symbols = server.workspace().symbol_table().all_symbols().len();

    // Change document content
    server
        .change_document(&uri, "part def Vehicle; part def Bike;")
        .unwrap();

    assert_eq!(server.workspace().file_count(), 1);
    let updated_symbols = server.workspace().symbol_table().all_symbols().len();
    assert!(updated_symbols > initial_symbols);
}

#[test]
fn test_change_document_with_error() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open valid document
    server.open_document(&uri, "part def Car;").unwrap();
    assert_eq!(server.workspace().file_count(), 1);

    // Change to invalid content - should succeed but capture error
    let result = server.change_document(&uri, "invalid syntax !@#");
    assert!(result.is_ok());

    // File should be removed from workspace (parse failed)
    assert_eq!(server.workspace().file_count(), 0);

    // Should have diagnostics
    let diagnostics = server.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty());
}

#[test]
fn test_change_nonexistent_document() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Try to change a document that was never opened
    let result = server.change_document(&uri, "part def Car;");
    // Should succeed - change_document handles both open and update
    assert!(result.is_ok());
}

#[test]
fn test_close_document() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open and close document
    server.open_document(&uri, "part def Car;").unwrap();
    server.close_document(&uri).unwrap();

    // Document should still be in workspace (we keep it for cross-file refs)
    assert_eq!(server.workspace().file_count(), 1);
}

#[test]
fn test_get_diagnostics_for_valid_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    let diagnostics = server.get_diagnostics(&uri);
    assert!(
        diagnostics.is_empty(),
        "Valid file should have no diagnostics"
    );
}

#[test]
fn test_get_diagnostics_for_parse_error() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def invalid syntax";

    server.open_document(&uri, text).unwrap();

    let diagnostics = server.get_diagnostics(&uri);
    assert!(
        !diagnostics.is_empty(),
        "Should have parse error diagnostic"
    );
    assert_eq!(diagnostics[0].severity, Some(DiagnosticSeverity::ERROR));
    assert!(!diagnostics[0].message.is_empty());
}

#[test]
fn test_get_diagnostics_clears_on_fix() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open with error
    server.open_document(&uri, "invalid syntax").unwrap();
    let diagnostics = server.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty());

    // Fix the error
    server.change_document(&uri, "part def Car;").unwrap();
    let diagnostics = server.get_diagnostics(&uri);
    assert!(
        diagnostics.is_empty(),
        "Diagnostics should be cleared after fix"
    );
}

#[test]
fn test_get_diagnostics_for_nonexistent_file() {
    let server = LspServer::new();
    let uri = Url::parse("file:///nonexistent.sysml").unwrap();

    let diagnostics = server.get_diagnostics(&uri);
    assert!(
        diagnostics.is_empty(),
        "Nonexistent file should have no diagnostics"
    );
}

#[test]
fn test_hover_on_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    // Hover on "Vehicle"
    let hover = server.get_hover(
        &uri,
        Position {
            line: 0,
            character: 9,
        },
    );
    assert!(hover.is_some());

    let hover = hover.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };
    assert!(content.contains("Vehicle"));
    // Symbol table stores "Part" (capitalized kind)
    assert!(content.contains("Part def"));
}

#[test]
fn test_hover_on_whitespace() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    let hover = server.get_hover(
        &uri,
        Position {
            line: 0,
            character: 4, // In whitespace between "part" and "def"
        },
    );
    // The hover returns Vehicle because the position is within the Definition's span
    assert!(
        hover.is_some(),
        "Position within element span should return hover"
    );
}

#[test]
fn test_hover_on_unknown_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;\npart def Car;";

    server.open_document(&uri, text).unwrap();

    // Hover on "part" keyword (position 0,0) - this is within Vehicle's span
    // so it returns Vehicle hover, not an error
    let hover = server.get_hover(
        &uri,
        Position {
            line: 0,
            character: 0,
        },
    );
    // AST-based hover returns the element at this position (Vehicle)
    assert!(
        hover.is_some(),
        "Position within element span should return hover"
    );

    // Test hover outside any element span (after semicolon with spaces)
    let text_with_space = "part def Vehicle;     \n";
    let uri2 = Url::parse("file:///test2.sysml").unwrap();
    server.open_document(&uri2, text_with_space).unwrap();

    let hover = server.get_hover(
        &uri2,
        Position {
            line: 0,
            character: 22, // Far after semicolon
        },
    );
    assert!(
        hover.is_none(),
        "Position outside element spans should have no hover"
    );
}

#[test]
fn test_hover_multiline() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;\npart def Car;";

    server.open_document(&uri, text).unwrap();

    // Hover on "Car" on line 2
    let hover = server.get_hover(
        &uri,
        Position {
            line: 1,
            character: 9,
        },
    );
    assert!(hover.is_some());

    let hover = hover.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };
    assert!(content.contains("Car"));
}

#[test]
fn test_hover_with_relationships() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;
part def Car :> Vehicle;
part myCar: Car;"#;

    server.open_document(&uri, text).unwrap();

    // Hover on "Car" definition (line 1)
    let hover = server.get_hover(
        &uri,
        Position {
            line: 1,
            character: 9,
        },
    );
    assert!(hover.is_some());

    let hover = hover.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };
    // Should show the definition
    assert!(content.contains("Part def Car"));
    // Should show qualified name
    assert!(content.contains("Qualified Name"));
    // Should show specialization relationship
    assert!(content.contains("Specializes"));
    assert!(content.contains("Vehicle"));

    // Hover on "myCar" usage (line 2)
    let hover = server.get_hover(
        &uri,
        Position {
            line: 2,
            character: 5,
        },
    );
    assert!(hover.is_some());

    let hover = hover.unwrap();
    let HoverContents::Scalar(MarkedString::String(content)) = hover.contents else {
        panic!("Expected scalar string content");
    };
    // Should show the usage - format is "Part myCar" (capitalized kind)
    assert!(content.contains("Part myCar") || content.contains("myCar"));
    // Should show typing relationship
    assert!(content.contains("Typed by"));
    assert!(content.contains("Car"));
}

#[test]
fn test_hover_shows_precise_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    let hover = server.get_hover(
        &uri,
        Position {
            line: 0,
            character: 9,
        },
    );
    assert!(hover.is_some());

    let hover = hover.unwrap();
    // Should return a range for the element
    assert!(hover.range.is_some(), "Hover should include element range");

    let range = hover.range.unwrap();
    assert_eq!(range.start.line, 0);
    assert_eq!(range.end.line, 0);
    // Range should cover the entire definition
    assert!(range.end.character > range.start.character);
}

#[test]
fn test_goto_definition_same_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Car;
part myCar : Car;"#;

    server.open_document(&uri, text).unwrap();

    // Position on "Car" in the usage line (line 1, after the colon)
    let location = server.get_definition(
        &uri,
        Position {
            line: 1,
            character: 14, // On "Car" type reference
        },
    );

    assert!(location.is_some(), "Should find definition");
    let location = location.unwrap();

    // Should point to the definition on line 0
    assert_eq!(location.uri, uri);
    assert_eq!(location.range.start.line, 0);
    // Range should cover the definition
    assert!(location.range.end.character > location.range.start.character);
}

#[test]
fn test_goto_definition_on_definition() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;";

    server.open_document(&uri, text).unwrap();

    // Position on "Vehicle" in its own definition
    let location = server.get_definition(
        &uri,
        Position {
            line: 0,
            character: 10,
        },
    );

    assert!(location.is_some(), "Should find itself");
    let location = location.unwrap();

    // Should point to itself
    assert_eq!(location.uri, uri);
    assert_eq!(location.range.start.line, 0);
}

#[test]
fn test_goto_definition_unknown_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Car;";

    server.open_document(&uri, text).unwrap();

    // Position in whitespace
    let location = server.get_definition(
        &uri,
        Position {
            line: 0,
            character: 0,
        },
    );

    assert!(location.is_none(), "No symbol at position");
}

#[test]
fn test_goto_definition_nested_elements() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Automotive {
    part def Engine;
    part myEngine : Engine;
}"#;

    server.open_document(&uri, text).unwrap();

    // Position on "Engine" type reference in the usage (line 2)
    let location = server.get_definition(
        &uri,
        Position {
            line: 2,
            character: 21, // On "Engine" type reference
        },
    );

    assert!(location.is_some(), "Should find Engine definition");
    let location = location.unwrap();

    // Should point to the definition on line 1
    assert_eq!(location.uri, uri);
    assert_eq!(location.range.start.line, 1);
}

#[test]
fn test_find_references_same_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Car;
part myCar : Car;
part yourCar : Car;"#;

    server.open_document(&uri, text).unwrap();

    // Find references to "Car" from the definition (line 0)
    let locations = server.get_references(
        &uri,
        Position {
            line: 0,
            character: 10, // On "Car" in definition
        },
        true, // include declaration
    );

    assert!(locations.is_some(), "Should find references");
    let locations = locations.unwrap();

    // Should find: definition + 2 usages = 3 total
    assert_eq!(locations.len(), 3, "Should find 3 references");

    // All should be in the same file
    for loc in &locations {
        assert_eq!(loc.uri, uri);
    }

    // Check lines: 0 (definition), 1 (first usage), 2 (second usage)
    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();
    assert!(lines.contains(&0));
    assert!(lines.contains(&1));
    assert!(lines.contains(&2));
}

#[test]
fn test_find_references_from_usage() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle;
part myVehicle : Vehicle;"#;

    server.open_document(&uri, text).unwrap();

    // Find references from a usage (line 1, on "Vehicle" type reference)
    let locations = server.get_references(
        &uri,
        Position {
            line: 1,
            character: 18, // On "Vehicle" type reference
        },
        true,
    );

    assert!(locations.is_some(), "Should find references from usage");
    let locations = locations.unwrap();

    // Should find: definition + usage = 2 total
    assert_eq!(locations.len(), 2);
}

#[test]
fn test_find_references_exclude_declaration() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Engine;
part myEngine : Engine;"#;

    server.open_document(&uri, text).unwrap();

    // Find references excluding declaration
    let locations = server.get_references(
        &uri,
        Position {
            line: 0,
            character: 10,
        },
        false, // exclude declaration
    );

    assert!(locations.is_some());
    let locations = locations.unwrap();

    // Should only find usages, not the definition
    assert_eq!(locations.len(), 1);
    assert_eq!(locations[0].range.start.line, 1); // Only the usage on line 1
}

#[test]
fn test_find_references_no_references() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def UnusedType;
part myPart;"#;

    server.open_document(&uri, text).unwrap();

    // Find references to UnusedType
    let locations = server.get_references(
        &uri,
        Position {
            line: 0,
            character: 10,
        },
        false, // exclude declaration
    );

    // Should return empty list (no usages)
    assert!(locations.is_some());
    let locations = locations.unwrap();
    assert_eq!(locations.len(), 0);
}

#[test]
fn test_find_references_nested_elements() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Auto {
    part def Wheel;
    part def Car {
        part frontWheel : Wheel;
        part rearWheel : Wheel;
    }
}"#;

    server.open_document(&uri, text).unwrap();

    // Debug: check parsed AST
    let file = server
        .workspace()
        .files()
        .get(&std::path::PathBuf::from("/test.sysml"));
    if let Some(wf) = file {
        eprintln!("Parsed AST:");
        eprintln!("{:#?}", wf.content());
    }

    // Find references to "Wheel" (line 1)
    let locations = server.get_references(
        &uri,
        Position {
            line: 1,
            character: 14, // On "Wheel" in definition
        },
        true,
    );

    assert!(locations.is_some());
    let locations = locations.unwrap();

    // Debug: print what we found
    eprintln!("Found {} locations:", locations.len());
    for loc in &locations {
        eprintln!("  Line {}: {:?}", loc.range.start.line, loc.uri);
    }

    // Debug: check the symbol
    let symbol = server.workspace().symbol_table().lookup("Auto::Wheel");
    eprintln!(
        "Symbol lookup result: {:?}",
        symbol.map(|s| (s.qualified_name(), s.references().len()))
    );

    // Debug: check all symbols
    eprintln!("All symbols in table:");
    for (key, sym) in server.workspace().symbol_table().all_symbols() {
        eprintln!(
            "  {} -> {} (refs: {})",
            key,
            sym.qualified_name(),
            sym.references().len()
        );
    }

    // Debug: check relationship graph
    eprintln!("Typing relationships:");
    for (key, _) in server.workspace().symbol_table().all_symbols() {
        if let Some(target) = server
            .workspace()
            .relationship_graph()
            .get_one_to_one(REL_TYPING, key)
        {
            eprintln!("  {} -> {}", key, target);
        }
    }

    // Should find: definition + 2 usages = 3 total
    assert_eq!(locations.len(), 3);

    // Verify lines
    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();
    assert!(lines.contains(&1)); // definition
    assert!(lines.contains(&3)); // frontWheel
    assert!(lines.contains(&4)); // rearWheel
}

#[test]
fn test_document_symbols() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
part def Vehicle;
part def Engine;
part engine : Engine;
    "#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new("/test.sysml");
    let symbols = server.get_document_symbols(path);

    // Debug: print what symbols we got
    for sym in &symbols {
        eprintln!("Symbol: {} (kind: {:?})", sym.name, sym.kind);
    }

    // Should have 3 symbols
    assert_eq!(symbols.len(), 3);

    // Check symbol names
    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Vehicle"));
    assert!(names.contains(&"Engine"));
    assert!(names.contains(&"engine"));
}

#[test]
fn test_semantic_tokens() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Auto {
    part def Vehicle;
    part myVehicle : Vehicle;
    alias MyAlias for Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let tower_lsp::lsp_types::SemanticTokensResult::Tokens(tokens) =
        server.get_semantic_tokens(uri.as_str()).unwrap()
    else {
        panic!("Expected SemanticTokens result");
    };

    // Should have tokens for: Auto (package), Vehicle (def), myVehicle (usage), MyAlias (alias)
    assert!(tokens.data.len() >= 4);

    // Verify we got different token types
    let token_types: Vec<u32> = tokens.data.iter().map(|t| t.token_type).collect();

    // TokenType enum values: Namespace=0, Type=1, Variable=2, Property=3, Keyword=4
    assert!(token_types.contains(&0)); // NAMESPACE
    assert!(token_types.contains(&1)); // TYPE
    assert!(token_types.contains(&2)); // VARIABLE
    assert!(token_types.contains(&3)); // PROPERTY
}

#[test]
fn test_code_completion_keywords() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "package Test {}\n";

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new("/test.sysml");
    let position = Position::new(1, 0); // After the package

    let tower_lsp::lsp_types::CompletionResponse::Array(items) =
        server.get_completions(path, position)
    else {
        panic!("Expected completion array");
    };

    assert!(!items.is_empty());

    // Should have keyword completions
    let keywords: Vec<&str> = items
        .iter()
        .filter(|item| item.kind == Some(tower_lsp::lsp_types::CompletionItemKind::KEYWORD))
        .map(|item| item.label.as_str())
        .collect();

    assert!(keywords.contains(&"part def"));
    assert!(keywords.contains(&"part"));
    assert!(keywords.contains(&"package"));
}

#[test]
fn test_code_completion_file_types() {
    // Test that keyword selection works for different file types
    let sysml_keywords =
        syster::keywords::get_keywords_for_file(std::path::Path::new("test.sysml"));
    let kerml_keywords =
        syster::keywords::get_keywords_for_file(std::path::Path::new("test.kerml"));

    // SysML has domain-specific keywords
    assert!(sysml_keywords.contains(&"part def"));
    assert!(sysml_keywords.contains(&"requirement"));
    assert!(sysml_keywords.contains(&"action"));

    // KerML has foundation keywords
    assert!(kerml_keywords.contains(&"classifier"));
    assert!(kerml_keywords.contains(&"feature"));
    assert!(kerml_keywords.contains(&"type"));

    // They should be different
    assert!(!kerml_keywords.contains(&"part def"));
    assert!(!sysml_keywords.contains(&"classifier"));
}
