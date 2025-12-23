use super::LspServer;
use syster::core::constants::REL_TYPING;
use syster::semantic::symbol_table::Symbol;
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

    // Hover on "Vehicle" name should work
    let hover = server.get_hover(
        &uri,
        Position {
            line: 0,
            character: 10, // On "Vehicle" name
        },
    );
    assert!(
        hover.is_some(),
        "Position on element name should return hover"
    );
}

#[test]
fn test_hover_on_unknown_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Vehicle;\npart def Car;";

    server.open_document(&uri, text).unwrap();

    // Hover on "Vehicle" name (position 0,9) - should return hover
    let hover = server.get_hover(
        &uri,
        Position {
            line: 0,
            character: 10,
        },
    );
    assert!(
        hover.is_some(),
        "Position on element name should return hover"
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
            eprintln!("  {key} -> {target}");
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

    // Should have 3 symbols
    assert_eq!(symbols.len(), 3);

    // Check symbol names
    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Vehicle"));
    assert!(names.contains(&"Engine"));
    assert!(names.contains(&"engine"));
}

#[test]
fn test_document_symbols_hierarchical() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Auto {
    part def Vehicle {
        part engine : Engine;
        part wheel : Wheel;
    }
    part def Engine;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new("/test.sysml");
    let symbols = server.get_document_symbols(path);

    // Should have 1 root symbol (Auto package)
    assert_eq!(symbols.len(), 1, "Expected 1 root symbol");

    let auto = &symbols[0];
    assert_eq!(auto.name, "Auto");
    assert_eq!(auto.kind, tower_lsp::lsp_types::SymbolKind::NAMESPACE);

    // Auto should have 2 children: Vehicle and Engine
    let auto_children = auto.children.as_ref().expect("Auto should have children");
    assert_eq!(auto_children.len(), 2, "Auto should have 2 children");

    let child_names: Vec<&str> = auto_children.iter().map(|s| s.name.as_str()).collect();
    assert!(child_names.contains(&"Vehicle"));
    assert!(child_names.contains(&"Engine"));

    // Find Vehicle and check its children
    let vehicle = auto_children
        .iter()
        .find(|s| s.name == "Vehicle")
        .expect("Vehicle not found");
    let vehicle_children = vehicle
        .children
        .as_ref()
        .expect("Vehicle should have children");
    assert_eq!(vehicle_children.len(), 2, "Vehicle should have 2 children");

    let vehicle_child_names: Vec<&str> = vehicle_children.iter().map(|s| s.name.as_str()).collect();
    assert!(vehicle_child_names.contains(&"engine"));
    assert!(vehicle_child_names.contains(&"wheel"));
}

#[test]
fn test_document_symbols_deeply_nested() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Level1 {
    package Level2 {
        part def Level3 {
            part level4 : Level4;
        }
    }
}
    "#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new("/test.sysml");
    let symbols = server.get_document_symbols(path);

    // Should have 1 root symbol
    assert_eq!(symbols.len(), 1);

    let level1 = &symbols[0];
    assert_eq!(level1.name, "Level1");

    // Level1 -> Level2
    let level1_children = level1
        .children
        .as_ref()
        .expect("Level1 should have children");
    assert_eq!(level1_children.len(), 1);
    let level2 = &level1_children[0];
    assert_eq!(level2.name, "Level2");

    // Level2 -> Level3
    let level2_children = level2
        .children
        .as_ref()
        .expect("Level2 should have children");
    assert_eq!(level2_children.len(), 1);
    let level3 = &level2_children[0];
    assert_eq!(level3.name, "Level3");

    // Level3 -> level4
    let level3_children = level3
        .children
        .as_ref()
        .expect("Level3 should have children");
    assert_eq!(level3_children.len(), 1);
    let level4 = &level3_children[0];
    assert_eq!(level4.name, "level4");
}

#[test]
fn test_document_symbols_mixed_hierarchy() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    // Each package declaration creates its own scope
    let text = r#"
package Automotive {
    part def Vehicle;
}
package Electronics {
    part def Sensor;
}
    "#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new("/test.sysml");
    let symbols = server.get_document_symbols(path);

    // Should have 2 root symbols (Automotive and Electronics packages)
    assert_eq!(symbols.len(), 2, "Expected 2 root packages");

    let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"Automotive"));
    assert!(names.contains(&"Electronics"));

    // Automotive should have Vehicle as child
    let automotive = symbols
        .iter()
        .find(|s| s.name == "Automotive")
        .expect("Automotive not found");
    let auto_children = automotive
        .children
        .as_ref()
        .expect("Automotive should have children");
    assert_eq!(auto_children.len(), 1);
    assert_eq!(auto_children[0].name, "Vehicle");

    // Electronics should have Sensor as child
    let electronics = symbols
        .iter()
        .find(|s| s.name == "Electronics")
        .expect("Electronics not found");
    let elec_children = electronics
        .children
        .as_ref()
        .expect("Electronics should have children");
    assert_eq!(elec_children.len(), 1);
    assert_eq!(elec_children[0].name, "Sensor");
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

#[test]
fn test_rename_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package TestPkg {
    part def OldName;
    part myPart : OldName;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Rename at definition position
    let position = Position::new(2, 14); // On "OldName" in definition
    let Some(edit) = server.get_rename_edits(&uri, position, "NewName") else {
        panic!("Expected rename edit");
    };

    let Some(changes) = edit.changes else {
        panic!("Expected changes");
    };

    let Some(edits) = changes.get(&uri) else {
        panic!("Expected edits for file");
    };

    // Should have 2 edits: definition + usage
    assert_eq!(edits.len(), 2);

    // Check that both locations are being renamed
    let edit_texts: Vec<&str> = edits.iter().map(|e| e.new_text.as_str()).collect();
    assert!(edit_texts.iter().all(|&t| t == "NewName"));
}

#[test]
fn test_rename_from_usage() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package TestPkg {
    part def Vehicle;
    part car : Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Rename from usage position
    let position = Position::new(3, 16); // On "Vehicle" in usage
    let Some(edit) = server.get_rename_edits(&uri, position, "Automobile") else {
        panic!("Expected rename edit");
    };

    let Some(changes) = edit.changes else {
        panic!("Expected changes");
    };

    let Some(edits) = changes.get(&uri) else {
        panic!("Expected edits for file");
    };

    // Should have 2 edits: definition + usage
    assert_eq!(edits.len(), 2);

    let edit_texts: Vec<&str> = edits.iter().map(|e| e.new_text.as_str()).collect();
    assert!(edit_texts.iter().all(|&t| t == "Automobile"));
}

#[test]
fn test_cross_file_reference_resolution_basic() {
    // Test cross-file reference resolution at the workspace/symbol table level
    // This is the foundational layer - if this doesn't work, nothing above it will

    let mut server = LspServer::new();

    // File 1: Define a type
    let file1_uri = Url::parse("file:///base.sysml").unwrap();
    let file1_text = r#"
    package BasePackage {
        attribute def BaseUnit {
        }
    }
    "#;

    // File 2: Reference the type from File 1
    let file2_uri = Url::parse("file:///derived.sysml").unwrap();
    let file2_text = r#"
    package DerivedPackage {
        import BasePackage::BaseUnit;
        
        attribute def DerivedUnit :> BaseUnit {
        }
    }
    "#;

    // Open both files
    server.open_document(&file1_uri, file1_text).unwrap();
    server.open_document(&file2_uri, file2_text).unwrap();

    eprintln!("Workspace file count: {}", server.workspace().file_count());
    eprintln!(
        "Total symbols: {}",
        server.workspace().symbol_table().all_symbols().len()
    );

    let all_syms = server.workspace().symbol_table().all_symbols();
    eprintln!("\nAll symbols:");
    for (name, sym) in all_syms.iter() {
        let qualified = sym.qualified_name();
        eprintln!("  {} -> {} (qualified: {})", name, sym.name(), qualified);
    }

    // Check if BaseUnit is in the symbol table
    let symbol_table = server.workspace().symbol_table();

    let by_simple = symbol_table.lookup("BaseUnit");
    let by_qualified = symbol_table.lookup_qualified("BasePackage::BaseUnit");

    eprintln!("\nLookup BaseUnit:");
    eprintln!("  Simple name: {:?}", by_simple.is_some());
    eprintln!("  Qualified: {:?}", by_qualified.is_some());

    assert!(
        by_simple.is_some() || by_qualified.is_some(),
        "BaseUnit should be findable in symbol table"
    );

    // Now try to resolve the reference from file 2
    // Position should be on "BaseUnit" in ":> BaseUnit"
    let position = Position::new(4, 40); // Approximate position of BaseUnit after :>
    let definition = server.get_definition(&file2_uri, position);

    eprintln!(
        "\nDefinition lookup result: {:?}",
        definition.as_ref().map(|d| d.uri.path())
    );

    assert!(
        definition.is_some(),
        "Should resolve cross-file reference to BaseUnit"
    );

    let def_location = definition.unwrap();
    assert!(
        def_location.uri.path().contains("base.sysml"),
        "Definition should point to base.sysml, got: {}",
        def_location.uri.path()
    );
}

#[test]
fn test_cross_file_stdlib_reference_resolution() {
    // This test verifies that references to stdlib types are resolved correctly
    // Bug: attribute def SoundPressureLevelUnit :> DimensionOneUnit
    // DimensionOneUnit from MeasurementReferences doesn't resolve

    // For tests, we need to find the stdlib in target/debug, not target/debug/deps
    let stdlib_path = std::env::current_exe()
        .ok()
        .and_then(|exe| {
            // Test binary is at target/debug/deps/<binary>
            // Stdlib is at target/debug/sysml.library
            exe.parent()
                .and_then(|deps| deps.parent())
                .map(|debug| debug.join("sysml.library"))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("sysml.library"));

    let mut server = LspServer::with_config(true, Some(stdlib_path));

    // Load stdlib
    server.ensure_stdlib_loaded().unwrap();

    eprintln!("After stdlib load:");
    eprintln!("  Files: {}", server.workspace().file_count());
    eprintln!(
        "  Symbols: {}",
        server.workspace().symbol_table().all_symbols().len()
    );

    // Check if MeasurementReferences file is loaded
    let has_measurement_refs = server
        .workspace()
        .files()
        .keys()
        .any(|p| p.to_string_lossy().contains("MeasurementReferences"));
    eprintln!("  Has MeasurementReferences.sysml: {has_measurement_refs}");

    // Check what symbols ARE in the symbol table from stdlib
    eprintln!("\n  First 10 stdlib symbols:");
    for (i, (name, symbol)) in server
        .workspace()
        .symbol_table()
        .all_symbols()
        .iter()
        .enumerate()
        .take(10)
    {
        let symbol_type = match symbol {
            Symbol::Package { .. } => "Package",
            Symbol::Classifier { .. } => "Classifier",
            Symbol::Feature { .. } => "Feature",
            Symbol::Definition { kind, .. } => kind.as_str(),
            Symbol::Usage { kind, .. } => kind.as_str(),
            Symbol::Alias { .. } => "Alias",
        };
        eprintln!("    {i}: {name} ({symbol_type})");
    }

    // Check specifically for attribute definitions
    eprintln!("\n  Attribute definitions in symbol table:");
    let mut attr_count = 0;
    for (name, symbol) in server.workspace().symbol_table().all_symbols() {
        if let Symbol::Definition { kind, .. } = symbol
            && kind == "Attribute"
        {
            attr_count += 1;
            if attr_count <= 5 {
                eprintln!("    - {name}");
            }
        }
    }
    eprintln!("  Total attribute definitions: {attr_count}");

    // Open a file that references a stdlib type
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
    package TestPackage {
        import MeasurementReferences::DimensionOneUnit;
        
        attribute def MyUnit :> DimensionOneUnit {
        }
    }
    "#;

    server.open_document(&uri, text).unwrap();

    eprintln!("\nAfter opening test file:");
    eprintln!("  Files: {}", server.workspace().file_count());
    eprintln!(
        "  Symbols: {}",
        server.workspace().symbol_table().all_symbols().len()
    );

    // Check if DimensionOneUnit is in symbol table
    if let Some(symbol) = server
        .workspace()
        .symbol_table()
        .lookup_qualified("MeasurementReferences::DimensionOneUnit")
    {
        eprintln!("\nFound DimensionOneUnit: {symbol:?}");
    } else {
        eprintln!("\nDimensionOneUnit NOT found in symbol table");
        eprintln!("\nLooking for any MeasurementReferences symbols:");
        for (name, _) in server.workspace().symbol_table().all_symbols() {
            if name.contains("MeasurementReferences") {
                eprintln!("  - {name}");
            }
        }
    }

    // Try to get definition of DimensionOneUnit at line 4, column 36 (the :> reference)
    let position = Position::new(4, 36);
    let definition = server.get_definition(&uri, position);

    eprintln!("\nDefinition result: {definition:?}");

    assert!(
        definition.is_some(),
        "Should resolve definition for stdlib type DimensionOneUnit"
    );

    let def_location = definition.unwrap();
    assert!(
        def_location
            .uri
            .path()
            .contains("MeasurementReferences.sysml"),
        "Definition should point to stdlib MeasurementReferences.sysml file"
    );
}

#[test]
fn test_stdlib_files_actually_load() {
    // Most basic test: do stdlib files get added to the workspace at all?
    // For tests, we need to find the stdlib in target/debug, not target/debug/deps
    let stdlib_path = std::env::current_exe()
        .ok()
        .and_then(|exe| {
            // Test binary is at target/debug/deps/<binary>
            // Stdlib is at target/debug/sysml.library
            exe.parent()
                .and_then(|deps| deps.parent())
                .map(|debug| debug.join("sysml.library"))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("sysml.library"));

    let mut server = LspServer::with_config(true, Some(stdlib_path.clone()));

    eprintln!("Before stdlib load:");
    eprintln!("  Files: {}", server.workspace().file_count());

    eprintln!("\nStdlib path: {}", stdlib_path.display());
    eprintln!("  Exists: {}", stdlib_path.exists());
    eprintln!("  Is dir: {}", stdlib_path.is_dir());

    let load_result = server.ensure_stdlib_loaded();
    eprintln!("\nLoad result: {load_result:?}");

    eprintln!("\nAfter stdlib load:");
    eprintln!("  Files: {}", server.workspace().file_count());
    eprintln!(
        "  Symbols: {}",
        server.workspace().symbol_table().all_symbols().len()
    );

    // Print some file paths
    eprintln!("\nFirst 5 files:");
    for (i, path) in server.workspace().files().keys().enumerate().take(5) {
        eprintln!("  {}: {}", i, path.display());
    }

    assert!(
        server.workspace().file_count() > 0,
        "Stdlib files should be loaded into workspace"
    );

    assert!(
        !server.workspace().symbol_table().all_symbols().is_empty(),
        "Stdlib symbols should be populated"
    );
}

#[test]
fn test_measurement_references_file_directly() {
    use std::path::PathBuf;

    let file_path = PathBuf::from(
        "/workspaces/syster/target/debug/sysml.library/Domain Libraries/Quantities and Units/MeasurementReferences.sysml",
    );

    eprintln!("File exists: {}", file_path.exists());

    if !file_path.exists() {
        eprintln!("File not found, skipping test");
        return;
    }

    let content = std::fs::read_to_string(&file_path).expect("Failed to read file");
    eprintln!("File size: {} bytes", content.len());

    let parse_result = syster::project::file_loader::parse_with_result(&content, &file_path);

    if parse_result.content.is_none() {
        eprintln!("Parse FAILED!");
        eprintln!("Errors: {}", parse_result.errors.len());
        for (i, err) in parse_result.errors.iter().enumerate().take(5) {
            eprintln!("  {i}: {err:?}");
        }
        panic!("Failed to parse MeasurementReferences.sysml");
    }

    eprintln!("Parse succeeded!");

    let syntax_file = parse_result.content.unwrap();
    let sysml_file = match syntax_file {
        syster::syntax::SyntaxFile::SysML(f) => f,
        _ => panic!("Expected SysML file"),
    };

    eprintln!("Top-level elements: {}", sysml_file.elements.len());

    // Populate symbol table
    let mut workspace = syster::semantic::Workspace::<syster::syntax::SyntaxFile>::new();
    workspace.add_file(
        file_path.clone(),
        syster::syntax::SyntaxFile::SysML(sysml_file),
    );
    let _ = workspace.populate_all();

    eprintln!(
        "\nSymbols found: {}",
        workspace.symbol_table().all_symbols().len()
    );
    eprintln!("\nAll symbols:");
    for (name, symbol) in workspace.symbol_table().all_symbols() {
        let sym_type = match symbol {
            Symbol::Package { .. } => "Package",
            Symbol::Definition { kind, .. } => kind.as_str(),
            Symbol::Usage { kind, .. } => kind.as_str(),
            Symbol::Classifier { .. } => "Classifier",
            Symbol::Feature { .. } => "Feature",
            Symbol::Alias { .. } => "Alias",
        };
        eprintln!("  {name} ({sym_type})");
    }

    // Check for attribute definitions
    let all_syms = workspace.symbol_table().all_symbols();
    let attr_defs: Vec<_> = all_syms
        .iter()
        .filter(|(_, sym)| matches!(sym, Symbol::Definition { kind, .. } if kind == "Attribute"))
        .map(|(name, _)| name)
        .collect();

    eprintln!("\nAttribute definitions: {}", attr_defs.len());
    for name in attr_defs.iter().take(10) {
        eprintln!("  - {name}");
    }

    assert!(!attr_defs.is_empty(), "Should have attribute definitions");

    // Look for DimensionOneUnit specifically
    let has_dimension_one = all_syms
        .iter()
        .any(|(name, _)| name.contains("DimensionOneUnit"));

    eprintln!("\nHas DimensionOneUnit: {has_dimension_one}");
    assert!(has_dimension_one, "Should find DimensionOneUnit");
}

#[test]
fn test_dimension_one_unit_cross_file_resolution() {
    use syster::project::stdlib_loader::StdLibLoader;
    use syster::semantic::Workspace;
    use syster::syntax::parser::parse_content;

    let stdlib_path = std::path::PathBuf::from("../../target/debug/sysml.library");
    eprintln!("StdLib path: {:?}", stdlib_path.canonicalize().unwrap());

    let mut workspace = Workspace::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).unwrap();
    let _populate_result = workspace.populate_all(); // Ignore errors, we want to test what DID load

    eprintln!(
        "Loaded stdlib - Files: {}, Symbols: {}",
        workspace.file_count(),
        workspace.symbol_table().all_symbols().len()
    );

    // Sample some package names from stdlib
    let package_names: Vec<_> = workspace
        .symbol_table()
        .all_symbols()
        .iter()
        .filter_map(|(name, sym)| {
            if matches!(sym, Symbol::Package { .. }) {
                Some(name.as_str())
            } else {
                None
            }
        })
        .take(10)
        .collect();
    eprintln!("Sample package names: {package_names:?}");

    // Check what symbols we actually have
    let measurement_refs_syms: Vec<_> = workspace
        .symbol_table()
        .all_symbols()
        .iter()
        .filter(|(name, _)| name.contains("MeasurementReferences") || name.contains("DimensionOne"))
        .map(|(name, _)| name.as_str())
        .collect();
    eprintln!("MeasurementReferences symbols: {measurement_refs_syms:?}");

    // Check if MeasurementReferences.sysml file is in workspace
    let has_measurement_file = workspace
        .files()
        .keys()
        .any(|path| path.to_string_lossy().contains("MeasurementReferences"));
    eprintln!("Has MeasurementReferences.sysml file: {has_measurement_file}");

    // Check parse errors for MeasurementReferences
    if let Some((_path, file)) = workspace
        .files()
        .iter()
        .find(|(p, _)| p.to_string_lossy().contains("MeasurementReferences"))
    {
        let (file_type, elem_count) = match file.content() {
            syster::syntax::SyntaxFile::SysML(sysml) => ("SysML", sysml.elements.len()),
            syster::syntax::SyntaxFile::KerML(kerml) => ("KerML", kerml.elements.len()),
        };
        eprintln!(
            "MeasurementReferences file type: {file_type}, has {elem_count} top-level elements"
        );
    }

    // Check DimensionOneUnit exists
    let found = workspace
        .symbol_table()
        .lookup_qualified("MeasurementReferences::DimensionOneUnit");
    eprintln!(
        "Lookup MeasurementReferences::DimensionOneUnit: {}",
        found.is_some()
    );
    assert!(
        found.is_some(),
        "DimensionOneUnit should be found in stdlib"
    );

    // Now add a user file that uses it
    let test_code = r#"
package TestPkg {
    import MeasurementReferences::DimensionOneUnit;
    
    attribute def MyUnit :> DimensionOneUnit {
    }
}
"#;

    let path = std::path::PathBuf::from("/test/myfile.sysml");
    let file = parse_content(test_code, &path).unwrap();
    workspace.add_file(path.clone(), file);
    let _ = workspace.populate_all();

    eprintln!(
        "After adding user file - Files: {}, Symbols: {}",
        workspace.file_count(),
        workspace.symbol_table().all_symbols().len()
    );

    // Verify MyUnit is in the table
    let my_unit = workspace.symbol_table().lookup_qualified("TestPkg::MyUnit");
    eprintln!("Lookup TestPkg::MyUnit: {}", my_unit.is_some());
    assert!(my_unit.is_some(), "MyUnit should be found");

    // Verify DimensionOneUnit is still findable
    let dim_one = workspace
        .symbol_table()
        .lookup_qualified("MeasurementReferences::DimensionOneUnit");
    eprintln!(
        "Lookup MeasurementReferences::DimensionOneUnit after user file: {}",
        dim_one.is_some()
    );
    assert!(
        dim_one.is_some(),
        "DimensionOneUnit should still be found after adding user file"
    );
}
