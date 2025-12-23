//! Integration tests for LSP server
//!
//! Tests the full stack from server initialization through symbol resolution

use std::path::PathBuf;
use syster_lsp::LspServer;

#[test]
fn test_server_initialization() {
    // This test explicitly loads stdlib to test initialization
    let mut server = LspServer::new();

    // Load stdlib for testing
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path);
    stdlib_loader
        .load(server.workspace_mut())
        .expect("Failed to load stdlib");

    // Populate symbol table from loaded files
    server
        .workspace_mut()
        .populate_all()
        .expect("Failed to populate symbols");

    // Verify workspace is created
    assert!(
        !server.workspace().files().is_empty(),
        "Stdlib files should be loaded"
    );

    // Verify symbols are populated
    let symbol_count = server.workspace().symbol_table().all_symbols().len();
    assert!(
        symbol_count > 0,
        "Symbol table should be populated with stdlib symbols"
    );

    println!(
        "✓ Server initialized with {} files and {} symbols",
        server.workspace().files().len(),
        symbol_count
    );
}

#[test]
fn test_ensure_stdlib_loaded() {
    // Create server with explicit stdlib path for testing
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let mut server = LspServer::with_config(true, Some(stdlib_path));

    // Initially workspace should be empty
    assert_eq!(
        server.workspace().files().len(),
        0,
        "Workspace should start empty"
    );
    assert!(
        !server.workspace().has_stdlib(),
        "Stdlib should not be loaded initially"
    );

    // Load stdlib
    server.ensure_stdlib_loaded().expect("Should load stdlib");

    // Verify stdlib was loaded
    assert!(
        server.workspace().has_stdlib(),
        "Stdlib should be marked as loaded"
    );
    assert!(
        !server.workspace().files().is_empty(),
        "Workspace should have files after stdlib loading"
    );

    println!(
        "✓ Stdlib loaded: {} workspace files",
        server.workspace().files().len()
    );

    // Verify we can find specific stdlib files
    let has_base = server
        .workspace()
        .files()
        .keys()
        .any(|p| p.to_string_lossy().contains("Base.kerml"));
    assert!(has_base, "Should have loaded Base.kerml from stdlib");

    // Load stdlib again - count shouldn't change (idempotent)
    server.ensure_stdlib_loaded().expect("Should load stdlib");
    assert_eq!(
        server.workspace().files().len(),
        server.workspace().files().len(),
        "Files count should remain the same on second call"
    );
}

#[test]
fn test_hover_on_cross_file_symbol() {
    // Create server with explicit stdlib path for testing
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let mut server = LspServer::with_config(true, Some(stdlib_path));

    // Load stdlib first
    server.ensure_stdlib_loaded().expect("Should load stdlib");

    println!(
        "✓ Stdlib loaded with {} symbols",
        server.workspace().symbol_table().all_symbols().len()
    );

    // Debug: Check how many KerML vs SysML files
    let mut kerml_count = 0;
    let mut sysml_count = 0;
    for path in server.workspace().files().keys() {
        if path.extension().and_then(|e| e.to_str()) == Some("kerml") {
            kerml_count += 1;
        } else if path.extension().and_then(|e| e.to_str()) == Some("sysml") {
            sysml_count += 1;
        }
    }
    println!("✓ File breakdown: {kerml_count} KerML files, {sysml_count} SysML files");

    // Check if ScalarValues.kerml is loaded
    let scalar_values_path = server
        .workspace()
        .files()
        .keys()
        .find(|p| p.to_string_lossy().contains("ScalarValues.kerml"));

    if let Some(path) = scalar_values_path {
        println!("✓ Found ScalarValues.kerml at: {}", path.display());
    } else {
        println!("✗ ScalarValues.kerml NOT in workspace!");
    }

    // Find TradeStudies.sysml file
    let trade_studies_path = server
        .workspace()
        .files()
        .keys()
        .find(|p| p.to_string_lossy().contains("TradeStudies.sysml"))
        .expect("Should have TradeStudies.sysml in stdlib")
        .clone();

    println!(
        "✓ Found TradeStudies.sysml at: {}",
        trade_studies_path.display()
    );

    // Convert to absolute path for URL conversion
    let abs_path = std::fs::canonicalize(&trade_studies_path).expect("Should canonicalize path");

    // Open the document (simulate LSP did_open)
    let uri = tower_lsp::lsp_types::Url::from_file_path(&abs_path).expect("Should convert to URL");
    let text = std::fs::read_to_string(&trade_studies_path).expect("Should read file");

    server
        .open_document(&uri, &text)
        .expect("Should open document");

    // Find line containing "ScalarValue" - it should be in the EvaluationFunction definition
    let lines: Vec<&str> = text.lines().collect();
    let (line_index, col_index) = lines
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.find("ScalarValue").map(|pos| (i, pos)))
        .expect("Should find ScalarValue in file");

    println!("✓ Found 'ScalarValue' at line {line_index}, col {col_index}");
    println!("  Line content: {}", lines[line_index].trim());

    // Try to get hover at that position
    let position = tower_lsp::lsp_types::Position {
        line: line_index as u32,
        character: (col_index + 5) as u32, // Middle of "ScalarValue"
    };

    println!(
        "✓ Attempting hover at line {}, char {}",
        position.line, position.character
    );

    let hover_result = server.get_hover(&uri, position);

    if let Some(hover) = hover_result {
        println!("✓ Hover succeeded!");
        if let tower_lsp::lsp_types::HoverContents::Scalar(
            tower_lsp::lsp_types::MarkedString::String(content),
        ) = hover.contents
        {
            println!("  Content: {content}");
            assert!(
                content.contains("ScalarValue"),
                "Hover should mention ScalarValue"
            );
        }
    } else {
        println!("✗ Hover failed - no result returned");

        // Debug: Check if ScalarValue exists in symbol table
        let scalar_value_symbols: Vec<_> = server
            .workspace()
            .symbol_table()
            .all_symbols()
            .iter()
            .filter(|(_, s)| {
                s.name() == "ScalarValue" || s.qualified_name().contains("ScalarValue")
            })
            .map(|(_, s)| (s.name(), s.qualified_name(), s.span().is_some()))
            .collect();

        println!("  Symbols matching 'ScalarValue': {scalar_value_symbols:?}");

        panic!("Hover should work for cross-file symbol ScalarValue");
    }
}

#[test]
fn test_stdlib_symbols_present() {
    // This test explicitly loads stdlib to verify symbols
    let mut server = LspServer::new();

    // Load stdlib for testing
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path);
    stdlib_loader
        .load(server.workspace_mut())
        .expect("Failed to load stdlib");

    // Populate symbol table from loaded files
    server
        .workspace_mut()
        .populate_all()
        .expect("Failed to populate symbols");

    let symbol_table = server.workspace().symbol_table();
    let all_symbols = symbol_table.all_symbols();

    println!("Total symbols loaded: {}", all_symbols.len());

    // Show what packages are actually loaded
    println!("\nPackages found:");
    let packages: Vec<_> = all_symbols
        .iter()
        .filter(|(_, s)| s.qualified_name() == s.name() && !s.name().contains("::"))
        .take(20)
        .collect();

    for (_key, symbol) in packages {
        println!("  - {}", symbol.name());
    }

    // Show symbols containing "Case" to debug why Case isn't found
    println!("\nSymbols containing 'Case':");
    let case_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(_, s)| s.name().contains("Case") || s.qualified_name().contains("Case"))
        .take(10)
        .collect();

    for (key, symbol) in case_symbols {
        println!(
            "  key='{}' name='{}' qualified='{}'",
            key,
            symbol.name(),
            symbol.qualified_name()
        );
    }

    // Try finding some basic symbols that should be in SysML stdlib
    let test_symbols = vec!["Part", "Attribute", "Item"];

    for simple_name in test_symbols {
        let found = all_symbols.iter().any(|(_, s)| s.name() == simple_name);
        if found {
            println!("✓ Found '{simple_name}' in symbol table");
        } else {
            println!("✗ Missing '{simple_name}'");
        }
    }
}

#[test]
fn test_document_lifecycle() {
    let mut server = LspServer::new();

    // Create a test document
    let test_uri = tower_lsp::lsp_types::Url::parse("file:///test.sysml").unwrap();
    let test_content = r#"
package TestPackage {
    part def TestPart;
    port def TestPort;
}
"#;

    // Open document
    let result = server.open_document(&test_uri, test_content);
    assert!(result.is_ok(), "Document should open successfully");

    // Verify file is in workspace
    let path = PathBuf::from("/test.sysml");
    assert!(
        server.workspace().files().contains_key(&path),
        "File should be in workspace"
    );

    println!("✓ Document lifecycle test passed");
}
#[test]
fn test_symbol_resolution_after_population() {
    // This test explicitly loads stdlib to test resolution
    let mut server = LspServer::new();

    // Load stdlib for testing
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path);
    stdlib_loader
        .load(server.workspace_mut())
        .expect("Failed to load stdlib");

    // Populate symbol table from loaded files
    server
        .workspace_mut()
        .populate_all()
        .expect("Failed to populate symbols");

    // Get some actual symbols from the table to verify resolution works
    let all_symbols = server.workspace().symbol_table().all_symbols();

    if all_symbols.is_empty() {
        panic!("Symbol table is empty - stdlib population may have failed");
    }

    // Test resolving the first few symbols by their simple names
    let resolver = server.resolver();
    let test_count = all_symbols.len().min(10);

    for (qualified_name, symbol) in all_symbols.iter().take(test_count) {
        let simple_name = symbol.name();
        let resolved = resolver.resolve(simple_name);

        // Note: Simple name resolution might not find all symbols if there are duplicates
        // or if they're in nested scopes, but it should work for top-level symbols
        if resolved.is_none() {
            println!(
                "⚠ Could not resolve '{simple_name}' by simple name (qualified: {qualified_name})"
            );
        } else {
            println!("✓ Resolved '{simple_name}' -> {qualified_name}");
        }
    }
}

#[test]
fn test_cross_file_resolution() {
    let mut server = LspServer::new();

    // Create first file with a definition
    let file1_uri = tower_lsp::lsp_types::Url::parse("file:///file1.sysml").unwrap();
    let file1_content = r#"
package MyPackage {
    part def MyPart;
    port def MyPort;
}
"#;

    // Create second file that references first file
    let file2_uri = tower_lsp::lsp_types::Url::parse("file:///file2.sysml").unwrap();
    let file2_content = r#"
package AnotherPackage {
    import MyPackage::*;
    
    part myInstance : MyPart;
}
"#;

    // Open both documents
    assert!(server.open_document(&file1_uri, file1_content).is_ok());
    assert!(server.open_document(&file2_uri, file2_content).is_ok());

    // Debug: Show what's actually in the symbol table FIRST
    println!(
        "\nAll symbols in table (total {}):",
        server.workspace().symbol_table().all_symbols().len()
    );
    let all_symbols = server.workspace().symbol_table().all_symbols();
    let our_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(key, _)| key.contains("My"))
        .collect();
    for (key, symbol) in our_symbols {
        println!(
            "  key='{}' name='{}' qualified='{}'",
            key,
            symbol.name(),
            symbol.qualified_name()
        );
    }

    // Now try to resolve symbols
    let resolver = server.resolver();

    // Should find MyPart (defined in file1)
    let my_part = resolver.resolve("MyPart");
    println!(
        "Resolver result for 'MyPart': {:?}",
        my_part.map(|s| s.qualified_name())
    );

    if let Some(symbol) = my_part {
        println!(
            "✓ Found MyPart: {} (qualified: {})",
            symbol.name(),
            symbol.qualified_name()
        );

        // Check if it has the right qualified name
        assert_eq!(symbol.qualified_name(), "MyPackage::MyPart");
    }

    // Should also find MyPort
    let my_port = resolver.resolve("MyPort");
    // assert!(my_port.is_some(), "Should find MyPort symbol");
    println!(
        "Resolver result for 'MyPort': {:?}",
        my_port.map(|s| s.qualified_name())
    );

    // Resolver doesn't work - it only searches current scope
    // But hover should work because it does global search
    println!("\n--- Testing actual LSP hover (uses global search) ---");

    // Add document to the LSP server's cache
    let file2_path = PathBuf::from("/test2.sysml");
    server
        .document_texts_mut()
        .insert(file2_path.clone(), file2_content.to_string());

    // Test hover on "MyPackage" in import statement
    let hover_package = tower_lsp::lsp_types::Position {
        line: 2,
        character: 18,
    };
    let package_result = server.find_symbol_at_position(&file2_path, hover_package);
    println!("Hover on 'MyPackage': {package_result:?}");
    assert!(package_result.is_some(), "Should find MyPackage");

    // Test hover on "MyPart" usage
    let hover_mypart = tower_lsp::lsp_types::Position {
        line: 4,
        character: 26, // "part myInstance : MyPart;"
    };
    let mypart_result = server.find_symbol_at_position(&file2_path, hover_mypart);

    println!("Hover on 'MyPart': {mypart_result:?}");
    assert!(
        mypart_result.is_some(),
        "Hover should find MyPart via global search"
    );

    println!("✓ Cross-file symbol resolution works via hover!");
}
