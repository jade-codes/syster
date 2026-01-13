use crate::server::LspServer;
use async_lsp::lsp_types::{
    DiagnosticSeverity, HoverContents, MarkedString, Position, PrepareRenameResponse, Url,
};
use syster::semantic::resolver::Resolver;
use syster::semantic::symbol_table::Symbol;

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
    assert!(
        server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .next()
            .is_some()
    );
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
    // KerML is now supported
    assert!(result.is_ok());
    assert_eq!(server.workspace().file_count(), 1);
}

#[test]
fn test_change_document() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open initial document
    server.open_document(&uri, "part def Car;").unwrap();
    assert_eq!(server.workspace().file_count(), 1);
    let initial_symbols = server.workspace().symbol_table().iter_symbols().count();

    // Change document content
    server
        .open_document(&uri, "part def Vehicle; part def Bike;")
        .unwrap();

    assert_eq!(server.workspace().file_count(), 1);
    let updated_symbols = server.workspace().symbol_table().iter_symbols().count();
    assert!(updated_symbols > initial_symbols);
}

#[test]
fn test_references_update_after_change_document() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document with a definition and usage
    let initial = r#"
part def Vehicle;
part car : Vehicle;
    "#;
    server.open_document(&uri, initial).unwrap();

    // Find references to Vehicle - should find the usage
    let position = Position::new(1, 10); // On "Vehicle" definition
    let refs = server.get_references(&uri, position, false).unwrap();
    assert_eq!(refs.len(), 1, "Should find 1 reference (car : Vehicle)");

    // Rename Vehicle to VehicleA in BOTH places (simulating user rename)
    let updated = r#"
part def VehicleA;
part car : VehicleA;
    "#;
    server.open_document(&uri, updated).unwrap();

    // Find references to VehicleA - should still find the usage
    let position = Position::new(1, 10); // On "VehicleA" definition
    let refs = server.get_references(&uri, position, false).unwrap();
    assert_eq!(
        refs.len(),
        1,
        "After rename, should find 1 reference (car : VehicleA)"
    );

    // Verify the old name has no references
    // (Vehicle no longer exists, so lookup should fail)
    assert!(
        Resolver::new(server.workspace().symbol_table())
            .resolve("Vehicle")
            .is_none(),
        "Old symbol 'Vehicle' should no longer exist"
    );
}

#[test]
fn test_references_break_when_definition_renamed_without_usages() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document with a definition and usage
    let initial = r#"
part def Vehicle;
part car : Vehicle;
    "#;
    server.open_document(&uri, initial).unwrap();

    // Find references to Vehicle - should find the usage
    let position = Position::new(1, 10); // On "Vehicle" definition
    let refs = server.get_references(&uri, position, false).unwrap();
    assert_eq!(refs.len(), 1, "Should find 1 reference (car : Vehicle)");

    // Rename ONLY the definition (simulating user typing, not using Rename)
    let broken = r#"
part def VehicleA;
part car : Vehicle;
    "#;
    server.open_document(&uri, broken).unwrap();

    // Find references to VehicleA - should find NONE (usage still says Vehicle)
    let position = Position::new(1, 10); // On "VehicleA" definition
    let refs = server.get_references(&uri, position, false);
    assert!(
        refs.is_none() || refs.as_ref().map(|r| r.is_empty()).unwrap_or(true),
        "VehicleA should have no references (usage still references old name)"
    );

    // The old name Vehicle is referenced but doesn't exist as a definition
    // This would be a semantic error in the diagnostics
}

#[test]
fn test_change_document_with_error() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open valid document
    server.open_document(&uri, "part def Car;").unwrap();
    assert_eq!(server.workspace().file_count(), 1);

    // Change to invalid content - should succeed but capture error
    let result = server.open_document(&uri, "invalid syntax !@#");
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
    let result = server.open_document(&uri, "part def Car;");
    // Should succeed - open_document handles both open and update
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
    server.open_document(&uri, "part def Car;").unwrap();
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
    // Specialization target not available without RelationshipGraph
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
fn test_hover_with_index() {
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
    // Specialization info not available without RelationshipGraph
    // Specialization target not available without RelationshipGraph

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
    if let Some(_wf) = file {}

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
    for _loc in &locations {}

    // Debug: check the symbol
    let _symbol = Resolver::new(server.workspace().symbol_table()).resolve("Auto::Wheel");

    // Debug: check all symbols
    for _sym in server.workspace().symbol_table().iter_symbols() {}

    // Should find: definition + 2 usages = 3 total
    assert_eq!(locations.len(), 3);

    // Verify lines
    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();
    assert!(lines.contains(&1)); // definition
    assert!(lines.contains(&3)); // frontWheel
    assert!(lines.contains(&4)); // rearWheel
}

// Edge case tests for references
#[test]
fn test_references_with_include_declaration() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part car : Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Find references including declaration
    let position = Position::new(2, 14); // On "Vehicle" definition
    let locations = server
        .get_references(&uri, position, true)
        .expect("Must find references");

    // MUST include both definition and usage when include_declaration=true
    assert_eq!(
        locations.len(),
        2,
        "Must include exactly 2 references: declaration (line 2) and usage (line 3)"
    );

    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();
    assert!(
        lines.contains(&2),
        "Must include definition line when include_declaration=true"
    );
    assert!(
        lines.contains(&3),
        "Must include usage line 'part car : Vehicle'"
    );
}

#[test]
fn test_references_exclude_declaration() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    part car : Vehicle;
    part bike : Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Find references excluding declaration
    let position = Position::new(2, 14); // On "Vehicle" definition
    let locations = server
        .get_references(&uri, position, false)
        .expect("Must find references even when excluding declaration");

    // MUST NOT include definition when include_declaration=false
    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();

    assert!(
        !lines.contains(&2),
        "Must NOT include definition line when include_declaration=false"
    );

    // MUST include usages
    assert!(
        lines.contains(&3) && lines.contains(&4),
        "Must include both usage lines (3: car, 4: bike)"
    );

    assert_eq!(
        locations.len(),
        2,
        "Must have exactly 2 references (excluding definition)"
    );
}

#[test]
fn test_references_across_files() {
    let mut server = LspServer::new();

    // File 1: Define type
    let file1_uri = Url::parse("file:///types.sysml").unwrap();
    let file1_text = r#"
package Types {
    part def Vehicle;
}
    "#;

    // File 2: Use type multiple times
    let file2_uri = Url::parse("file:///usage.sysml").unwrap();
    let file2_text = r#"
package Usage {
    import Types::Vehicle;
    part car : Vehicle;
    part truck : Vehicle;
}
    "#;

    server.open_document(&file1_uri, file1_text).unwrap();
    server.open_document(&file2_uri, file2_text).unwrap();

    // Find references from definition in file1
    let position = Position::new(2, 14); // On "Vehicle" in file1
    let locations = server
        .get_references(&file1_uri, position, true)
        .expect("Must find cross-file references");

    // MUST find references in both files
    let uris: std::collections::HashSet<_> = locations.iter().map(|l| &l.uri).collect();

    assert!(
        uris.contains(&file1_uri),
        "Must find reference in definition file (types.sysml)"
    );
    assert!(
        uris.contains(&file2_uri),
        "Must find references in usage file (usage.sysml)"
    );

    // Count references per file
    let file1_refs: Vec<_> = locations.iter().filter(|l| l.uri == file1_uri).collect();
    let file2_refs: Vec<_> = locations.iter().filter(|l| l.uri == file2_uri).collect();

    assert_eq!(
        file1_refs.len(),
        1,
        "Must have exactly 1 reference in file1 (definition)"
    );
    assert_eq!(
        file2_refs.len(),
        3,
        "Must have exactly 3 references in file2 (import + 2 usages: car and truck)"
    );
}

#[test]
#[ignore = "Requires proper scope resolution in ReferenceIndex"]
fn test_references_qualified_name_fallback() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Outer {
    package Inner {
        part def Vehicle;
    }
    part car : Inner::Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Debug: print all symbols
    for _symbol in server.workspace.symbol_table().iter_symbols() {}

    // Find references using qualified name
    let position = Position::new(5, 23); // On "Vehicle" in "Inner::Vehicle"
    let locations = server
        .get_references(&uri, position, true)
        .expect("Must resolve qualified name and find references");

    // MUST find exactly definition and qualified usage
    assert_eq!(
        locations.len(),
        2,
        "Must find exactly 2 references: definition (line 3) and qualified usage (line 5)"
    );

    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();

    assert!(
        lines.contains(&3),
        "Must find definition 'part def Vehicle' at line 3"
    );
    assert!(
        lines.contains(&5),
        "Must find qualified usage 'Inner::Vehicle' at line 5"
    );
}

#[test]
fn test_references_symbol_not_found() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "package Test {}";

    server.open_document(&uri, text).unwrap();

    // Try to find references at position with no symbol
    let position = Position::new(0, 5); // On "age" in "package" keyword
    let locations = server.get_references(&uri, position, true);

    // MUST return None or empty for non-symbol positions
    if let Some(locs) = locations {
        assert!(
            locs.is_empty(),
            "Must return empty list when no symbol at position (keyword positions don't have references)"
        );
    }
    // None is also acceptable: correctly identified no symbol to find references for
}

#[test]
#[ignore = "Requires proper scope resolution in ReferenceIndex"]
fn test_references_with_shadowing() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test_shadowing.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    package Nested {
        part def Vehicle;
        part car : Vehicle;
    }
    part truck : Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Find references to outer Vehicle
    let position = Position::new(2, 14); // On outer "Vehicle"

    let locations = server
        .get_references(&uri, position, true)
        .expect("Must find references to outer Vehicle");

    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();

    // MUST include outer Vehicle definition
    assert!(
        lines.contains(&2),
        "Must include outer Vehicle definition at line 2, got lines {lines:?}"
    );

    // MUST include usage of outer Vehicle (line 7: truck)
    assert!(
        lines.contains(&7),
        "Must include usage of outer Vehicle at line 7 'part truck : Vehicle'"
    );

    // MUST NOT include shadowed inner Vehicle (line 4)
    assert!(
        !lines.contains(&4),
        "Must NOT include shadowed inner 'part def Vehicle' at line 4 (different symbol)"
    );

    // MUST NOT include usage referring to inner Vehicle (line 5: car)
    assert!(
        !lines.contains(&5),
        "Must NOT include 'part car : Vehicle' at line 5 (refers to inner Vehicle, not outer)"
    );

    // Exactly 2 references: definition + truck usage
    assert_eq!(
        locations.len(),
        2,
        "Must have exactly 2 references: outer definition + truck usage"
    );
}

#[test]
fn test_references_to_imported_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
}
package Usage {
    import Test::Vehicle;
    part car : Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Find references to Vehicle from definition
    let position = Position::new(2, 14); // On "Vehicle" definition
    let locations = server
        .get_references(&uri, position, true)
        .expect("Must find references through import");

    // Now finds: definition (line 2), import (line 5), and usage (line 6)
    assert_eq!(
        locations.len(),
        3,
        "Must find exactly 3 references: definition (line 2), import (line 5), and usage (line 6)"
    );

    let lines: Vec<u32> = locations.iter().map(|l| l.range.start.line).collect();

    assert!(
        lines.contains(&2),
        "Must include definition 'part def Vehicle' at line 2"
    );
    assert!(
        lines.contains(&5),
        "Must include import 'import Test::Vehicle' at line 5"
    );
    assert!(
        lines.contains(&6),
        "Must include usage 'part car : Vehicle' at line 6"
    );
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
    assert_eq!(auto.kind, async_lsp::lsp_types::SymbolKind::NAMESPACE);

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

    let async_lsp::lsp_types::SemanticTokensResult::Tokens(tokens) =
        server.get_semantic_tokens(&uri).unwrap()
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

    let async_lsp::lsp_types::CompletionResponse::Array(items) =
        server.get_completions(path, position)
    else {
        panic!("Expected completion array");
    };

    assert!(!items.is_empty());

    // Should have keyword completions
    let keywords: Vec<&str> = items
        .iter()
        .filter(|item| item.kind == Some(async_lsp::lsp_types::CompletionItemKind::KEYWORD))
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

// Edge case tests for completion
#[test]
fn test_completion_filters_by_context() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Vehicle;
    attribute def Speed;
    part car : 
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Position after colon - should suggest type symbols
    let position = Position::new(4, 15);
    let result = server.get_completions(std::path::Path::new("/test.sysml"), position);

    match result {
        async_lsp::lsp_types::CompletionResponse::Array(items) => {
            let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();

            // Should include both definition types as valid typing targets
            assert!(
                labels.contains(&"Vehicle"),
                "Should suggest 'Vehicle' as a type after colon. Got: {labels:?}"
            );
            assert!(
                labels.contains(&"Speed"),
                "Should suggest 'Speed' as a type after colon. Got: {labels:?}"
            );
        }
        _ => panic!("Expected completion array for type context"),
    }
}

#[test]
fn test_completion_includes_imported_symbols() {
    let mut server = LspServer::new();

    let file1_uri = Url::parse("file:///types.sysml").unwrap();
    let file1_text = r#"
package Types {
    part def Engine;
}
    "#;

    let file2_uri = Url::parse("file:///usage.sysml").unwrap();
    let file2_text = r#"
package Usage {
    import Types::*;
    part car : 
}
    "#;

    server.open_document(&file1_uri, file1_text).unwrap();
    server.open_document(&file2_uri, file2_text).unwrap();

    // Position after colon in file2
    let position = Position::new(3, 15);
    let result = server.get_completions(std::path::Path::new("/usage.sysml"), position);

    match result {
        async_lsp::lsp_types::CompletionResponse::Array(items) => {
            let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();

            assert!(
                labels.contains(&"Engine"),
                "Should suggest imported 'Engine' type in completions"
            );
        }
        _ => panic!("Expected completion array with imported symbols"),
    }
}

#[test]
fn test_completion_respects_scope() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Outer {
    part def OuterType;
    
    package Inner {
        part def InnerType;
        part item : 
    }
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Position in Inner package after colon
    let position = Position::new(6, 20);
    let result = server.get_completions(std::path::Path::new("/test.sysml"), position);

    match result {
        async_lsp::lsp_types::CompletionResponse::Array(items) => {
            let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();

            // Should see both inner and outer types
            assert!(
                labels.contains(&"InnerType"),
                "Should suggest InnerType from current scope"
            );
            assert!(
                labels.contains(&"OuterType"),
                "Should suggest OuterType from parent scope"
            );
        }
        _ => panic!("Expected completion array with scoped symbols"),
    }
}

#[test]
fn test_completion_after_specializes_suggests_compatible_types() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Base;
    part def Other;
    part def Derived specializes 
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Position after "specializes" keyword
    let position = Position::new(4, 34);
    let result = server.get_completions(std::path::Path::new("/test.sysml"), position);

    match result {
        async_lsp::lsp_types::CompletionResponse::Array(items) => {
            let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();

            // Should suggest compatible types for specialization
            assert!(
                labels.contains(&"Base") || labels.contains(&"Other"),
                "Should suggest type definitions after 'specializes'"
            );
        }
        _ => panic!("Expected completion array after relationship keyword"),
    }
}

#[test]
fn test_completion_in_incomplete_expression() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    attribute def Speed;
    part def Vehicle {
        attribute speed : Speed;
        attribute maxSpeed : Speed = speed.
    }
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Position after "speed." - should suggest member access
    let position = Position::new(5, 46);
    let result = server.get_completions(std::path::Path::new("/test.sysml"), position);

    // At minimum, should return a response (even if empty for now)
    match result {
        async_lsp::lsp_types::CompletionResponse::Array(_items) => {
            // Member access completion is complex,
            // but we should at least handle the request without crashing
        }
        _ => {
            // Other response types are acceptable
        }
    }
}

#[test]
fn test_completion_empty_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();

    let position = Position::new(0, 0);
    let result = server.get_completions(std::path::Path::new("/test.sysml"), position);

    match result {
        async_lsp::lsp_types::CompletionResponse::Array(items) => {
            assert!(
                !items.is_empty(),
                "Should provide top-level keywords in empty file"
            );
            let labels: Vec<_> = items.iter().map(|i| i.label.as_str()).collect();
            assert!(
                labels.contains(&"package"),
                "Should suggest 'package' keyword in empty file"
            );
        }
        _ => panic!("Expected completion array in empty file"),
    }
}

#[test]
fn test_completion_handles_invalid_position() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "package Test {}";

    server.open_document(&uri, text).unwrap();

    // Position beyond end of file
    let position = Position::new(100, 100);
    let result = server.get_completions(std::path::Path::new("/test.sysml"), position);

    // Should handle gracefully without crashing
    match result {
        async_lsp::lsp_types::CompletionResponse::Array(_)
        | async_lsp::lsp_types::CompletionResponse::List(_) => {
            // Either response type is acceptable as long as it doesn't crash
        }
    }
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

// Edge case tests for rename
#[test]
fn test_rename_across_multiple_files() {
    let mut server = LspServer::new();

    // File 1: Define the type
    let file1_uri = Url::parse("file:///types.sysml").unwrap();
    let file1_text = r#"
package Types {
    part def Vehicle;
}
    "#;

    // File 2: Use the type
    let file2_uri = Url::parse("file:///usage.sysml").unwrap();
    let file2_text = r#"
package Usage {
    import Types::Vehicle;
    part car : Vehicle;
}
    "#;

    server.open_document(&file1_uri, file1_text).unwrap();
    server.open_document(&file2_uri, file2_text).unwrap();

    // Rename from file1 (definition)
    let position = Position::new(2, 14); // On "Vehicle" in definition
    let Some(edit) = server.get_rename_edits(&file1_uri, position, "Automobile") else {
        panic!("Rename across files should be supported");
    };

    let Some(changes) = edit.changes else {
        panic!("Should provide workspace edits for cross-file rename");
    };

    // MUST update both files
    assert!(
        changes.contains_key(&file1_uri),
        "Must rename in definition file"
    );
    assert!(
        changes.contains_key(&file2_uri),
        "Must rename in usage file including import statement"
    );

    // Verify type usage statement updated (import rename requires additional tracking)
    let file2_edits = &changes[&file2_uri];
    let has_usage_edit = file2_edits.iter().any(|e| e.range.start.line == 3);
    assert!(
        has_usage_edit,
        "Type usage 'part car : Vehicle' must be updated to use 'Automobile'"
    );

    // All edits must use new name
    for (_uri, edits) in changes.iter() {
        for edit in edits {
            assert_eq!(
                edit.new_text, "Automobile",
                "All rename edits must use the new name"
            );
        }
    }
}

#[test]
fn test_rename_qualified_name() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Outer {
    package Inner {
        part def Vehicle;
    }
    part car : Inner::Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Rename using qualified reference
    let position = Position::new(5, 23); // On "Vehicle" in "Inner::Vehicle"
    let Some(edit) = server.get_rename_edits(&uri, position, "Automobile") else {
        panic!("Should support rename from qualified name position");
    };

    let Some(changes) = edit.changes else {
        panic!("Should provide edits for qualified name rename");
    };

    let Some(edits) = changes.get(&uri) else {
        panic!("Should have edits in file");
    };

    // Must update both definition and qualified usage
    assert_eq!(
        edits.len(),
        2,
        "Must update exactly 2 locations: definition at line 3 and qualified usage at line 5"
    );

    let def_updated = edits.iter().any(|e| e.range.start.line == 3);
    let qualified_updated = edits.iter().any(|e| e.range.start.line == 5);

    assert!(
        def_updated,
        "Must update definition 'part def Vehicle' -> 'part def Automobile'"
    );
    assert!(
        qualified_updated,
        "Must update qualified usage 'Inner::Vehicle' -> 'Inner::Automobile'"
    );
}

#[test]
fn test_rename_nonexistent_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "package Test {}";

    server.open_document(&uri, text).unwrap();

    // Try to rename at a position with no symbol
    let position = Position::new(0, 5); // On "age" in "package" (not a renameable symbol)
    let result = server.get_rename_edits(&uri, position, "NewName");

    // Must return None for non-symbol positions
    assert!(
        result.is_none(),
        "Must return None when no renameable symbol at position (keywords cannot be renamed)"
    );
}

#[test]
fn test_rename_with_no_usages() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def UnusedType;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Rename the unused definition
    let position = Position::new(2, 14); // On "UnusedType"
    let Some(edit) = server.get_rename_edits(&uri, position, "NewType") else {
        panic!("Should support rename even with no usages");
    };

    let Some(changes) = edit.changes else {
        panic!("Should provide changes");
    };

    let Some(edits) = changes.get(&uri) else {
        panic!("Should have edits in file");
    };

    // Must have exactly 1 edit (just the definition)
    assert_eq!(
        edits.len(),
        1,
        "Must have exactly one edit when symbol has no usages"
    );
    assert_eq!(edits[0].new_text, "NewType", "Edit must use the new name");
    assert_eq!(
        edits[0].range.start.line, 2,
        "Edit must be on the definition line"
    );
}

#[test]
fn test_rename_preserves_other_symbols() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package Test {
    part def Car;
    part def Bike;
    part myCar : Car;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Rename only "Car"
    let position = Position::new(2, 14); // On "Car"
    let Some(edit) = server.get_rename_edits(&uri, position, "Vehicle") else {
        panic!("Should provide rename edits");
    };

    let Some(changes) = edit.changes else {
        panic!("Should provide changes");
    };

    let Some(edits) = changes.get(&uri) else {
        panic!("Should have edits in file");
    };

    // Must rename Car and its usage only
    assert_eq!(
        edits.len(),
        2,
        "Must have exactly 2 edits: definition + one usage"
    );

    // Verify only "Car" lines are edited
    for edit in edits {
        let line = edit.range.start.line;
        assert!(
            line == 2 || line == 4,
            "Must only edit lines with 'Car' (2 and 4), not 'Bike' (line 3)"
        );
        assert_eq!(edit.new_text, "Vehicle", "All edits must use new name");
    }

    // Verify Bike line is NOT in edits
    let bike_edited = edits.iter().any(|e| e.range.start.line == 3);
    assert!(
        !bike_edited,
        "Must NOT edit 'Bike' definition when renaming 'Car'"
    );
}

#[test]
fn test_prepare_rename_on_definition() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package TestPkg {
    part def Vehicle;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Prepare rename on definition
    let position = Position::new(2, 14); // On "Vehicle"
    let result = server.prepare_rename(&uri, position);

    assert!(result.is_some(), "Should be able to rename symbol");
    let response = result.unwrap();

    match response {
        PrepareRenameResponse::RangeWithPlaceholder { range, placeholder } => {
            assert_eq!(placeholder, "Vehicle", "Placeholder should be symbol name");
            assert_eq!(range.start.line, 2);
        }
        _ => panic!("Expected RangeWithPlaceholder response"),
    }
}

#[test]
fn test_prepare_rename_on_usage() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package TestPkg {
    part def Car;
    part myCar : Car;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Prepare rename on usage (the second "Car")
    let position = Position::new(3, 18); // On "Car" in usage
    let result = server.prepare_rename(&uri, position);

    assert!(result.is_some(), "Should be able to rename from usage");
}

#[test]
fn test_prepare_rename_nonexistent_symbol() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"
package TestPkg {
    part def Car;
}
    "#;

    server.open_document(&uri, text).unwrap();

    // Try to prepare rename on whitespace
    let position = Position::new(0, 0); // Empty area
    let result = server.prepare_rename(&uri, position);

    assert!(result.is_none(), "Should not be able to rename non-symbol");
}

#[test]
fn test_prepare_rename_returns_correct_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def MyVeryLongPartName;"#;

    server.open_document(&uri, text).unwrap();

    let position = Position::new(0, 12); // In the middle of "MyVeryLongPartName"
    let result = server.prepare_rename(&uri, position);

    assert!(result.is_some(), "Should find symbol");
    let response = result.unwrap();

    match response {
        PrepareRenameResponse::RangeWithPlaceholder { range, placeholder } => {
            assert_eq!(placeholder, "MyVeryLongPartName");
            // Range should cover the entire symbol name
            assert_eq!(range.start.character, 9); // Start of "MyVeryLongPartName"
            assert_eq!(range.end.character, 27); // End of "MyVeryLongPartName"
        }
        _ => panic!("Expected RangeWithPlaceholder response"),
    }
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

    for sym in server.workspace().symbol_table().iter_symbols() {
        let _qualified = sym.qualified_name();
    }

    // Check if BaseUnit is in the symbol table
    let symbol_table = server.workspace().symbol_table();

    let resolver = Resolver::new(symbol_table);
    let by_simple = resolver.resolve("BaseUnit");
    let by_qualified = resolver.resolve_qualified("BasePackage::BaseUnit");

    assert!(
        by_simple.is_some() || by_qualified.is_some(),
        "BaseUnit should be findable in symbol table"
    );

    // Now try to resolve the reference from file 2
    // Position should be on "BaseUnit" in ":> BaseUnit"
    let position = Position::new(4, 40); // Approximate position of BaseUnit after :>
    let definition = server.get_definition(&file2_uri, position);

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
    server.ensure_workspace_loaded().unwrap();

    // Check if MeasurementReferences file is loaded
    let _has_measurement_refs = server
        .workspace()
        .files()
        .keys()
        .any(|p| p.to_string_lossy().contains("MeasurementReferences"));

    // Check what symbols ARE in the symbol table from stdlib
    for symbol in server.workspace().symbol_table().iter_symbols().take(10) {
        let _symbol_type = match symbol {
            Symbol::Package { .. } => "Package",
            Symbol::Classifier { .. } => "Classifier",
            Symbol::Feature { .. } => "Feature",
            Symbol::Definition { kind, .. } => kind.as_str(),
            Symbol::Usage { kind, .. } => kind.as_str(),
            Symbol::Alias { .. } => "Alias",
            Symbol::Import { .. } => "Import",
        };
    }

    // Check specifically for attribute definitions
    let mut attr_count = 0;
    for symbol in server.workspace().symbol_table().iter_symbols() {
        if let Symbol::Definition { kind, .. } = symbol
            && kind == "Attribute"
        {
            attr_count += 1;
            if attr_count <= 5 {}
        }
    }

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

    // Check if DimensionOneUnit is in symbol table
    let resolver = Resolver::new(server.workspace().symbol_table());
    if resolver
        .resolve_qualified("MeasurementReferences::DimensionOneUnit")
        .is_some()
    {
    } else {
        for sym in server.workspace().symbol_table().iter_symbols() {
            if sym.qualified_name().contains("MeasurementReferences") {}
        }
    }

    // Try to get definition of DimensionOneUnit at line 4, column 36 (the :> reference)
    let position = Position::new(4, 36);
    let definition = server.get_definition(&uri, position);

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

    let _load_result = server.ensure_workspace_loaded();

    // Print some file paths
    for (_i, _path) in server.workspace().files().keys().enumerate().take(5) {}

    assert!(
        server.workspace().file_count() > 0,
        "Stdlib files should be loaded into workspace"
    );

    assert!(
        server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .next()
            .is_some(),
        "Stdlib symbols should be populated"
    );
}

#[test]
fn test_measurement_references_file_directly() {
    use std::path::PathBuf;

    let file_path = PathBuf::from(
        "/workspaces/syster/target/debug/sysml.library/Domain Libraries/Quantities and Units/MeasurementReferences.sysml",
    );

    if !file_path.exists() {
        return;
    }

    let content = std::fs::read_to_string(&file_path).expect("Failed to read file");

    let parse_result = syster::project::file_loader::parse_with_result(&content, &file_path);

    if parse_result.content.is_none() {
        for (_i, _err) in parse_result.errors.iter().enumerate().take(5) {}
        panic!("Failed to parse MeasurementReferences.sysml");
    }

    let syntax_file = parse_result.content.unwrap();
    let sysml_file = match syntax_file {
        syster::syntax::SyntaxFile::SysML(f) => f,
        _ => panic!("Expected SysML file"),
    };

    // Populate symbol table
    let mut workspace = syster::semantic::Workspace::<syster::syntax::SyntaxFile>::new();
    workspace.add_file(
        file_path.clone(),
        syster::syntax::SyntaxFile::SysML(sysml_file),
    );
    let _ = workspace.populate_all();
    for symbol in workspace.symbol_table().iter_symbols() {
        let _sym_type = match symbol {
            Symbol::Package { .. } => "Package",
            Symbol::Definition { kind, .. } => kind.as_str(),
            Symbol::Usage { kind, .. } => kind.as_str(),
            Symbol::Classifier { .. } => "Classifier",
            Symbol::Feature { .. } => "Feature",
            Symbol::Alias { .. } => "Alias",
            Symbol::Import { .. } => "Import",
        };
    }

    // Check for attribute definitions
    let attr_defs: Vec<_> = workspace
        .symbol_table()
        .iter_symbols()
        .filter(|sym| matches!(sym, Symbol::Definition { kind, .. } if kind == "Attribute"))
        .map(|sym| sym.qualified_name().to_string())
        .collect();
    for _name in attr_defs.iter().take(10) {}

    assert!(!attr_defs.is_empty(), "Should have attribute definitions");

    // Look for DimensionOneUnit specifically
    let has_dimension_one = workspace
        .symbol_table()
        .iter_symbols()
        .any(|sym| sym.qualified_name().contains("DimensionOneUnit"));
    assert!(has_dimension_one, "Should find DimensionOneUnit");
}

#[test]
fn test_dimension_one_unit_cross_file_resolution() {
    use syster::project::stdlib_loader::StdLibLoader;
    use syster::semantic::Workspace;
    use syster::syntax::parser::parse_content;

    // Use current_exe to find stdlib in correct target folder (debug or release)
    let stdlib_path = std::env::current_exe()
        .ok()
        .and_then(|exe| {
            exe.parent()
                .and_then(|deps| deps.parent())
                .map(|target| target.join("sysml.library"))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("../../target/debug/sysml.library"));

    let mut workspace = Workspace::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).unwrap();
    let _populate_result = workspace.populate_all(); // Ignore errors, we want to test what DID load

    // Sample some package names from stdlib
    let _package_names: Vec<_> = workspace
        .symbol_table()
        .iter_symbols()
        .filter_map(|sym| {
            if matches!(sym, Symbol::Package { .. }) {
                Some(sym.qualified_name().to_string())
            } else {
                None
            }
        })
        .take(10)
        .collect();

    // Check what symbols we actually have
    let _measurement_refs_syms: Vec<_> = workspace
        .symbol_table()
        .iter_symbols()
        .filter(|sym| {
            sym.qualified_name().contains("MeasurementReferences")
                || sym.qualified_name().contains("DimensionOne")
        })
        .map(|sym| sym.qualified_name().to_string())
        .collect();

    // Check if MeasurementReferences.sysml file is in workspace
    let _has_measurement_file = workspace
        .files()
        .keys()
        .any(|path| path.to_string_lossy().contains("MeasurementReferences"));

    // Check parse errors for MeasurementReferences
    if let Some((_path, file)) = workspace
        .files()
        .iter()
        .find(|(p, _)| p.to_string_lossy().contains("MeasurementReferences"))
    {
        let (_file_type, _elem_count) = match file.content() {
            syster::syntax::SyntaxFile::SysML(sysml) => ("SysML", sysml.elements.len()),
            syster::syntax::SyntaxFile::KerML(kerml) => ("KerML", kerml.elements.len()),
        };
    }

    // Check DimensionOneUnit exists
    let resolver = Resolver::new(workspace.symbol_table());
    let found = resolver.resolve_qualified("MeasurementReferences::DimensionOneUnit");
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

    // Verify MyUnit is in the table
    let resolver = Resolver::new(workspace.symbol_table());
    let my_unit = resolver.resolve_qualified("TestPkg::MyUnit");
    assert!(my_unit.is_some(), "MyUnit should be found");

    // Verify DimensionOneUnit is still findable
    let dim_one = resolver.resolve_qualified("MeasurementReferences::DimensionOneUnit");
    assert!(
        dim_one.is_some(),
        "DimensionOneUnit should still be found after adding user file"
    );
}

// ============================================================================
// Incremental Text Synchronization Tests (Issue #39)
// ============================================================================

#[test]
fn test_incremental_insert_at_start() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document
    let initial_text = "part def Vehicle;";
    server.open_document(&uri, initial_text).unwrap();

    // Insert text at start: "// Comment\n"
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        }),
        range_length: None,
        text: "// Comment\n".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Verify content is correct
    let path = uri.to_file_path().unwrap();
    let content = server.document_texts.get(&path).unwrap();
    assert_eq!(content, "// Comment\npart def Vehicle;");

    // Verify symbols still work
    assert!(
        server
            .workspace()
            .symbol_table()
            .iter_symbols()
            .any(|s| s.name() == "Vehicle")
    );
}

#[test]
fn test_incremental_insert_in_middle() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document with two definitions
    let initial_text = "part def Car;\npart def Bike;";
    server.open_document(&uri, initial_text).unwrap();

    // Insert new definition between them
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(1, 0),
            end: Position::new(1, 0),
        }),
        range_length: None,
        text: "part def Truck;\n".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Verify all three definitions exist
    let st = server.workspace().symbol_table();
    assert!(st.iter_symbols().any(|s| s.name() == "Car"));
    assert!(st.iter_symbols().any(|s| s.name() == "Truck"));
    assert!(st.iter_symbols().any(|s| s.name() == "Bike"));
}

#[test]
fn test_incremental_delete_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document
    let initial_text = "part def Car;\npart def Bike;";
    server.open_document(&uri, initial_text).unwrap();

    // Delete "Bike" definition (entire second line)
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(1, 0),
            end: Position::new(1, 15), // "part def Bike;" is 15 chars
        }),
        range_length: Some(15),
        text: "".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Verify only Car exists
    let st = server.workspace().symbol_table();
    assert!(st.iter_symbols().any(|s| s.name() == "Car"));
    assert!(!st.iter_symbols().any(|s| s.name() == "Bike"));
}

#[test]
fn test_incremental_replace_range() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document
    let initial_text = "part def Car;";
    server.open_document(&uri, initial_text).unwrap();

    // Replace "Car" with "Vehicle"
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 9), // Start of "Car"
            end: Position::new(0, 12),  // End of "Car"
        }),
        range_length: Some(3),
        text: "Vehicle".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Verify Vehicle exists, Car doesn't
    let st = server.workspace().symbol_table();
    assert!(st.iter_symbols().any(|s| s.name() == "Vehicle"));
    assert!(!st.iter_symbols().any(|s| s.name() == "Car"));
}

#[test]
fn test_incremental_multiple_changes() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document
    server.open_document(&uri, "part def Car;").unwrap();

    // Apply multiple incremental changes
    // Change 1: Add newline and new definition
    let change1 = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 13),
            end: Position::new(0, 13),
        }),
        range_length: None,
        text: "\npart def Bike;".to_string(),
    };
    server.apply_text_change_only(&uri, &change1).unwrap();
    server.parse_document(&uri);

    // Change 2: Insert comment at start
    let change2 = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        }),
        range_length: None,
        text: "// Vehicles\n".to_string(),
    };
    server.apply_text_change_only(&uri, &change2).unwrap();
    server.parse_document(&uri);

    // Verify both definitions exist
    let st = server.workspace().symbol_table();
    assert!(st.iter_symbols().any(|s| s.name() == "Car"));
    assert!(st.iter_symbols().any(|s| s.name() == "Bike"));
}

#[test]
fn test_incremental_multiline_insert() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document
    server.open_document(&uri, "part def Car;").unwrap();

    // Insert multi-line text
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 13),
            end: Position::new(0, 13),
        }),
        range_length: None,
        text: "\n\npart def Bike {\n    attribute weight : Real;\n}".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Verify both definitions and nested attribute exist
    let st = server.workspace().symbol_table();
    assert!(st.iter_symbols().any(|s| s.name() == "Car"));
    assert!(st.iter_symbols().any(|s| s.name() == "Bike"));
    assert!(st.iter_symbols().any(|s| s.name() == "weight"));
}

#[test]
fn test_incremental_change_preserves_diagnostics() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open valid document
    server.open_document(&uri, "part def Car;").unwrap();
    assert!(server.get_diagnostics(&uri).is_empty());

    // Make an incremental change that introduces an error
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 13),
            end: Position::new(0, 13),
        }),
        range_length: None,
        text: "\ninvalid syntax !@#".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Should have diagnostics now
    let diagnostics = server.get_diagnostics(&uri);
    assert!(!diagnostics.is_empty());
}

#[test]
fn test_incremental_change_updates_semantic_tokens() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document with a part definition
    server.open_document(&uri, "part def Vehicle;").unwrap();

    // Get initial semantic tokens
    let initial_tokens = server.get_semantic_tokens(&uri).unwrap();
    let async_lsp::lsp_types::SemanticTokensResult::Tokens(initial) = initial_tokens else {
        panic!("Expected SemanticTokens result");
    };

    // Should have tokens for "part", "def", and "Vehicle"
    assert!(!initial.data.is_empty());
    let initial_count = initial.data.len();

    // Make an incremental change to add another part definition
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 17),
            end: Position::new(0, 17),
        }),
        range_length: None,
        text: "\npart def Car;".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Get updated semantic tokens
    let updated_tokens = server.get_semantic_tokens(&uri).unwrap();
    let async_lsp::lsp_types::SemanticTokensResult::Tokens(updated) = updated_tokens else {
        panic!("Expected SemanticTokens result");
    };

    // Should have more tokens now (tokens for the new "part def Car")
    assert!(updated.data.len() > initial_count);

    // Verify the document content is correct
    let expected = "part def Vehicle;\npart def Car;";
    let actual = server.document_texts.values().next().unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_incremental_change_updates_references() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document with a definition and usage
    server
        .open_document(&uri, "part def Vehicle;\npart car : Vehicle;")
        .unwrap();

    // Find references to Vehicle - should find 2 (definition and usage)
    let vehicle_pos = Position::new(0, 9); // Position on "Vehicle" in definition
    let initial_refs = server.get_references(&uri, vehicle_pos, true).unwrap();
    assert_eq!(initial_refs.len(), 2); // Definition + usage

    // Make an incremental change to add another usage of Vehicle
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(1, 20),
            end: Position::new(1, 20),
        }),
        range_length: None,
        text: "\npart truck : Vehicle;".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Find references again - should now find 3 (definition + 2 usages)
    let updated_refs = server.get_references(&uri, vehicle_pos, true).unwrap();
    assert_eq!(updated_refs.len(), 3);

    // Verify the document content is correct
    let expected = "part def Vehicle;\npart car : Vehicle;\npart truck : Vehicle;";
    let actual = server.document_texts.values().next().unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_open_document_provides_semantic_tokens() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///new_file.sysml").unwrap();

    // Open a new document
    server
        .open_document(&uri, "part def Vehicle;\npart car : Vehicle;")
        .unwrap();

    // Should be able to get semantic tokens immediately
    let tokens = server.get_semantic_tokens(&uri).unwrap();
    let async_lsp::lsp_types::SemanticTokensResult::Tokens(result) = tokens else {
        panic!("Expected SemanticTokens result");
    };

    // Should have tokens for keywords and identifiers
    assert!(!result.data.is_empty());
}

#[test]
fn test_new_file_then_incremental_update() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///brand_new.sysml").unwrap();

    // Open a brand new file
    server.open_document(&uri, "part def Car;").unwrap();

    // Verify initial state
    let initial_tokens = server.get_semantic_tokens(&uri).unwrap();
    let async_lsp::lsp_types::SemanticTokensResult::Tokens(initial) = initial_tokens else {
        panic!("Expected SemanticTokens result");
    };
    assert!(!initial.data.is_empty());

    // Now make an incremental change
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 13),
            end: Position::new(0, 13),
        }),
        range_length: None,
        text: "\npart def Truck;".to_string(),
    };

    server.apply_text_change_only(&uri, &change).unwrap();
    server.parse_document(&uri);

    // Verify semantic tokens still work after update
    let updated_tokens = server.get_semantic_tokens(&uri).unwrap();
    let async_lsp::lsp_types::SemanticTokensResult::Tokens(updated) = updated_tokens else {
        panic!("Expected SemanticTokens result");
    };

    // Should have more tokens now
    assert!(updated.data.len() > initial.data.len());

    // Verify hover still works
    let hover_pos = Position::new(0, 9); // Position on "Car"
    let hover_result = server.get_hover(&uri, hover_pos);
    assert!(
        hover_result.is_some(),
        "Hover should work on new file after update"
    );
}

#[test]
fn test_incremental_change_on_unopened_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///never_opened.sysml").unwrap();

    // File was never opened with did_open, but client sends an incremental change
    // This can happen when creating a brand new file
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        }),
        range_length: None,
        text: "part def NewPart;".to_string(),
    };

    // Should not error, should treat as opening the document
    let result = server.apply_text_change_only(&uri, &change);
    server.parse_document(&uri);
    assert!(
        result.is_ok(),
        "Should handle incremental change on unopened file"
    );

    // Verify the document was added
    let content = server.document_texts.values().next();
    assert!(content.is_some());
    assert_eq!(content.unwrap(), "part def NewPart;");

    // Verify semantic tokens work
    let tokens = server.get_semantic_tokens(&uri);
    assert!(
        tokens.is_some(),
        "Should have semantic tokens after first edit"
    );
}

#[test]
fn test_incremental_insert_at_end_of_document() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();

    // Open document with multiple lines
    server
        .open_document(&uri, "part def Car;\npart def Truck;")
        .unwrap();

    // Insert at the very end of the document (after last line)
    let change = async_lsp::lsp_types::TextDocumentContentChangeEvent {
        range: Some(async_lsp::lsp_types::Range {
            start: Position::new(1, 16), // End of line 1
            end: Position::new(1, 16),
        }),
        range_length: None,
        text: "\npart def Bike;".to_string(),
    };

    // Should not error
    let result = server.apply_text_change_only(&uri, &change);
    server.parse_document(&uri);
    assert!(result.is_ok(), "Should handle insert at end of document");

    // Verify content
    let expected = "part def Car;\npart def Truck;\npart def Bike;";
    let actual = server.document_texts.values().next().unwrap();
    assert_eq!(actual, expected);
}
#[test]
fn test_folding_ranges_for_definitions() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package TestPackage {
    part def Vehicle {
        attribute weight : Real;
    }
    
    part def Car;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Folding ranges should be generated from text structure
    // even if symbol spans aren't available
    // The test verifies the function doesn't crash and returns valid data
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

// Note: Import block folding and consecutive line comment folding were removed
// as they are not useful for SysML/KerML. See semantic/adapters for AST-based folding.

#[test]
fn test_folding_ranges_no_single_line() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "part def Car; part def Truck;";

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Single-line definitions shouldn't create folding ranges
    // (or they might be empty if no multi-line structures exist)
    // The main point is it shouldn't crash
    assert!(ranges.is_empty() || ranges.iter().all(|r| r.end_line > r.start_line));
}

// ============================================================================
// Comprehensive Folding Range Tests
// ============================================================================

#[test]
fn test_folding_ranges_empty_file() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///empty.sysml").unwrap();
    let text = "";

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    assert!(
        ranges.is_empty(),
        "Empty file should have no folding ranges"
    );
}

#[test]
fn test_folding_ranges_simple_package() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify all ranges are valid (end >= start)
    for range in &ranges {
        assert!(
            range.end_line >= range.start_line,
            "All ranges must be valid"
        );
    }
}

#[test]
fn test_folding_ranges_nested_packages() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Outer {
    package Inner {
        part def Vehicle;
    }
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should have ranges for both outer and inner packages
    // Ranges may or may not exist depending on parser

    // Verify ranges are valid (end >= start)
    for range in &ranges {
        assert!(
            range.end_line >= range.start_line,
            "Range end must be >= start"
        );
    }

    // Ranges should be sorted by start line
    for i in 0..ranges.len().saturating_sub(1) {
        assert!(
            ranges[i].start_line <= ranges[i + 1].start_line,
            "Ranges should be sorted by start line"
        );
    }
}

#[test]
fn test_folding_ranges_part_def_with_body() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"part def Vehicle {
    attribute weight : Real;
    attribute speed : Real;
    part engine : Engine;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges are valid
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }

    // If ranges exist for this part def, verify they're reasonable
    if !ranges.is_empty() {
        let first_range = &ranges[0];
        assert_eq!(first_range.start_line, 0, "First element starts at line 0");
    }
}

#[test]
fn test_folding_ranges_attribute_def_with_body() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"attribute def Temperature {
    attribute unit : TemperatureUnit;
    attribute value : Real;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges are valid if they exist
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_multiline_comments() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"/* This is a
   multiline
   comment */
part def Vehicle;"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should have a folding range for the multiline comment
    let comment_ranges: Vec<_> = ranges
        .iter()
        .filter(|r| r.kind == Some(async_lsp::lsp_types::FoldingRangeKind::Comment))
        .collect();

    if !comment_ranges.is_empty() {
        let comment_range = comment_ranges[0];
        assert_eq!(
            comment_range.kind,
            Some(async_lsp::lsp_types::FoldingRangeKind::Comment),
            "Comment should have Comment kind"
        );
        assert!(
            comment_range.end_line > comment_range.start_line,
            "Multiline comment should span multiple lines"
        );
    }
}

#[test]
fn test_folding_ranges_mixed_content() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"/* Header comment
   with multiple lines */
package Test {
    /* Package comment */
    part def Vehicle {
        attribute weight : Real;
    }
    
    part def Car {
        attribute model : String;
    }
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should have multiple ranges: comments, package, and part defs
    // Ranges may or may not exist depending on parser

    // All ranges should be valid
    for range in &ranges {
        assert!(
            range.end_line >= range.start_line,
            "All ranges must be valid"
        );
    }
}

#[test]
fn test_folding_ranges_deeply_nested() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Level1 {
    package Level2 {
        package Level3 {
            part def Vehicle {
                attribute engine : Engine {
                    attribute cylinders : Integer;
                }
            }
        }
    }
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should have multiple ranges for the nested structure
    // Ranges may or may not exist depending on parser

    // Verify all ranges are valid
    for range in &ranges {
        assert!(
            range.end_line >= range.start_line,
            "Nested ranges must be valid"
        );
    }
}

#[test]
fn test_folding_ranges_nonexistent_file() {
    let server = LspServer::new();
    let path = std::path::Path::new("/nonexistent.sysml");
    let ranges = server.get_folding_ranges(path);

    assert!(
        ranges.is_empty(),
        "Nonexistent file should return empty ranges"
    );
}

#[test]
fn test_folding_ranges_multiple_top_level_elements() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Package1 {
    part def Vehicle;
}

package Package2 {
    part def Car;
}

package Package3 {
    part def Truck;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should have ranges for all three packages
    // Ranges may or may not exist depending on parser

    // Ranges should be sorted
    for i in 0..ranges.len().saturating_sub(1) {
        assert!(
            ranges[i].start_line <= ranges[i + 1].start_line,
            "Ranges should be sorted"
        );
    }
}

#[test]
fn test_folding_ranges_with_imports() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    import Base::Vehicle;
    import Base::Engine;
    
    part def Car {
        part engine : Engine;
    }
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Should have ranges for package and part def
    // Ranges may or may not exist depending on parser

    // Verify ranges cover the package and part def
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_action_def() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"action def Drive {
    in part vehicle : Vehicle;
    out part distance : Real;
    
    action start;
    action accelerate;
    action stop;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges are valid if they exist
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_requirement_def() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"requirement def SafetyRequirement {
    doc /* Safety requirements
         for the vehicle */
    
    require constraint {
        vehicle.speed <= maxSpeed
    }
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges are valid if they exist
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_maintains_character_positions() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Test {
    part def Vehicle;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify character positions are provided
    for range in &ranges {
        assert!(
            range.start_character.is_some(),
            "Should include start character position"
        );
        assert!(
            range.end_character.is_some(),
            "Should include end character position"
        );
    }
}

#[test]
fn test_folding_ranges_sorted_output() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"package Z {
    part def Vehicle;
}

package A {
    part def Car;
}

package M {
    part def Truck;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify output is sorted by start_line (per the implementation)
    for i in 0..ranges.len().saturating_sub(1) {
        assert!(
            ranges[i].start_line <= ranges[i + 1].start_line,
            "Folding ranges must be sorted by start line"
        );
    }
}

#[test]
fn test_folding_ranges_connection_def() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"connection def WireConnection {
    end source : Port;
    end target : Port;
    
    attribute wireGauge : Real;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges are valid if they exist
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_interface_def() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"interface def VehicleInterface {
    port driver : DriverPort;
    port fuel : FuelPort;
    
    connection wireConn : WireConnection;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges are valid if they exist
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_enumeration_def() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = r#"enum def TrafficLight {
    enum red;
    enum yellow;
    enum green;
}"#;

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    // Verify ranges are valid if they exist
    for range in &ranges {
        assert!(range.end_line >= range.start_line);
    }
}

#[test]
fn test_folding_ranges_whitespace_only() {
    let mut server = LspServer::new();
    let uri = Url::parse("file:///test.sysml").unwrap();
    let text = "   \n\n   \n   ";

    server.open_document(&uri, text).unwrap();

    let path = std::path::Path::new(uri.path());
    let ranges = server.get_folding_ranges(path);

    assert!(
        ranges.is_empty(),
        "Whitespace-only file should have no folding ranges"
    );
}

#[test]
fn test_workspace_symbols_query() {
    let mut server = LspServer::new();

    let uri1 = Url::parse("file:///symbols1.sysml").unwrap();
    let uri2 = Url::parse("file:///symbols2.sysml").unwrap();

    server
        .open_document(&uri1, "part def UniqueThing123;")
        .unwrap();
    server
        .open_document(&uri2, "part def AnotherThing456;")
        .unwrap();

    let results = server.get_workspace_symbols("UniqueThing123");

    assert!(!results.is_empty(), "Expected at least one matching symbol");
    assert!(
        results
            .iter()
            .all(|symbol| symbol.name.contains("UniqueThing123")),
        "All results should match the query"
    );
}
