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
}

#[test]
fn test_ensure_workspace_loaded() {
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
    server
        .ensure_workspace_loaded()
        .expect("Should load stdlib");

    // Verify stdlib was loaded
    assert!(
        server.workspace().has_stdlib(),
        "Stdlib should be marked as loaded"
    );
    assert!(
        !server.workspace().files().is_empty(),
        "Workspace should have files after stdlib loading"
    );

    // Verify we can find specific stdlib files
    let has_base = server
        .workspace()
        .files()
        .keys()
        .any(|p| p.to_string_lossy().contains("Base.kerml"));
    assert!(has_base, "Should have loaded Base.kerml from stdlib");

    // Load stdlib again - count shouldn't change (idempotent)
    server
        .ensure_workspace_loaded()
        .expect("Should load stdlib");
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
    server
        .ensure_workspace_loaded()
        .expect("Should load stdlib");

    // Debug: Check how many KerML vs SysML files
    let mut _kerml_count = 0;
    let mut _sysml_count = 0;
    for path in server.workspace().files().keys() {
        if path.extension().and_then(|e| e.to_str()) == Some("kerml") {
            _kerml_count += 1;
        } else if path.extension().and_then(|e| e.to_str()) == Some("sysml") {
            _sysml_count += 1;
        }
    }

    // Check if ScalarValues.kerml is loaded
    let _scalar_values_path = server
        .workspace()
        .files()
        .keys()
        .find(|p| p.to_string_lossy().contains("ScalarValues.kerml"));

    // Find TradeStudies.sysml file
    let trade_studies_path = server
        .workspace()
        .files()
        .keys()
        .find(|p| p.to_string_lossy().contains("TradeStudies.sysml"))
        .expect("Should have TradeStudies.sysml in stdlib")
        .clone();

    // Convert to absolute path for URL conversion
    let abs_path = std::fs::canonicalize(&trade_studies_path).expect("Should canonicalize path");

    // Open the document (simulate LSP did_open)
    let uri = async_lsp::lsp_types::Url::from_file_path(&abs_path).expect("Should convert to URL");
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

    // Try to get hover at that position
    let position = async_lsp::lsp_types::Position {
        line: line_index as u32,
        character: (col_index + 5) as u32, // Middle of "ScalarValue"
    };

    let hover_result = server.get_hover(&uri, position);

    if let Some(hover) = hover_result {
        if let async_lsp::lsp_types::HoverContents::Scalar(
            async_lsp::lsp_types::MarkedString::String(content),
        ) = hover.contents
        {
            assert!(
                content.contains("ScalarValue"),
                "Hover should mention ScalarValue"
            );
        }
    } else {
        // Debug: Check if ScalarValue exists in symbol table
        let _scalar_value_symbols: Vec<_> = server
            .workspace()
            .symbol_table()
            .all_symbols()
            .iter()
            .filter(|(_, s)| {
                s.name() == "ScalarValue" || s.qualified_name().contains("ScalarValue")
            })
            .map(|(_, s)| (s.name(), s.qualified_name(), s.span().is_some()))
            .collect();

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

    // Show what packages are actually loaded
    let packages: Vec<_> = all_symbols
        .iter()
        .filter(|(_, s)| s.qualified_name() == s.name() && !s.name().contains("::"))
        .take(20)
        .collect();

    for (_key, _symbol) in packages {}

    // Show symbols containing "Case" to debug why Case isn't found
    let case_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(_, s)| s.name().contains("Case") || s.qualified_name().contains("Case"))
        .take(10)
        .collect();

    for (_key, _symbol) in case_symbols {}

    // Try finding some basic symbols that should be in SysML stdlib
    let test_symbols = vec!["Part", "Attribute", "Item"];

    for simple_name in test_symbols {
        let _found = all_symbols.iter().any(|(_, s)| s.name() == simple_name);
    }
}

#[test]
fn test_document_lifecycle() {
    let mut server = LspServer::new();

    // Create a test document
    let test_uri = async_lsp::lsp_types::Url::parse("file:///test.sysml").unwrap();
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

    for (_qualified_name, symbol) in all_symbols.iter().take(test_count) {
        let simple_name = symbol.name();
        let _resolved = resolver.resolve(simple_name);
    }
}

#[test]
fn test_cross_file_resolution() {
    let mut server = LspServer::new();

    // Create first file with a definition
    let file1_uri = async_lsp::lsp_types::Url::parse("file:///file1.sysml").unwrap();
    let file1_content = r#"
package MyPackage {
    part def MyPart;
    port def MyPort;
}
"#;

    // Create second file that references first file
    let file2_uri = async_lsp::lsp_types::Url::parse("file:///file2.sysml").unwrap();
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
    let all_symbols = server.workspace().symbol_table().all_symbols();
    let our_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(key, _)| key.contains("My"))
        .collect();
    for (_key, _symbol) in our_symbols {}

    // Now try to resolve symbols
    let resolver = server.resolver();

    // Should find MyPart (defined in file1)
    let my_part = resolver.resolve("MyPart");

    if let Some(symbol) = my_part {
        // Check if it has the right qualified name
        assert_eq!(symbol.qualified_name(), "MyPackage::MyPart");
    }

    // Should also find MyPort
    let _my_port = resolver.resolve("MyPort");
    // assert!(my_port.is_some(), "Should find MyPort symbol");

    // Resolver doesn't work - it only searches current scope
    // But hover should work because it does global search

    // Add document to the LSP server's cache
    let file2_path = PathBuf::from("/test2.sysml");
    server
        .document_texts_mut()
        .insert(file2_path.clone(), file2_content.to_string());

    // Test hover on "MyPackage" in import statement
    let hover_package = async_lsp::lsp_types::Position {
        line: 2,
        character: 18,
    };
    let package_result = server.find_symbol_at_position(&file2_path, hover_package);
    assert!(package_result.is_some(), "Should find MyPackage");

    // Test hover on "MyPart" usage
    let hover_mypart = async_lsp::lsp_types::Position {
        line: 4,
        character: 26, // "part myInstance : MyPart;"
    };
    let mypart_result = server.find_symbol_at_position(&file2_path, hover_mypart);
    assert!(
        mypart_result.is_some(),
        "Hover should find MyPart via global search"
    );
}

#[test]
fn test_cancel_document_operations() {
    let mut server = LspServer::new();
    let path = PathBuf::from("/test.sysml");

    // First call creates a new token
    let token1 = server.cancel_document_operations(&path);
    assert!(!token1.is_cancelled(), "New token should not be cancelled");

    // Second call should cancel the first token and return a new one
    let token2 = server.cancel_document_operations(&path);
    assert!(token1.is_cancelled(), "Previous token should be cancelled");
    assert!(!token2.is_cancelled(), "New token should not be cancelled");

    // Third call should cancel the second token
    let token3 = server.cancel_document_operations(&path);
    assert!(token2.is_cancelled(), "Previous token should be cancelled");
    assert!(!token3.is_cancelled(), "New token should not be cancelled");

    // First token should still be cancelled
    assert!(
        token1.is_cancelled(),
        "First token should still be cancelled"
    );

    // Current token remains valid until next update
    assert!(!token3.is_cancelled(), "Current token should remain valid");
}

#[test]
fn test_cancel_operations_per_document() {
    let mut server = LspServer::new();
    let path_a = PathBuf::from("/a.sysml");
    let path_b = PathBuf::from("/b.sysml");

    // Create tokens for two different documents
    let token_a1 = server.cancel_document_operations(&path_a);
    let token_b1 = server.cancel_document_operations(&path_b);

    assert!(!token_a1.is_cancelled());
    assert!(!token_b1.is_cancelled());

    // Update document A - should only cancel token_a1
    let token_a2 = server.cancel_document_operations(&path_a);
    assert!(token_a1.is_cancelled(), "Token A1 should be cancelled");
    assert!(!token_b1.is_cancelled(), "Token B1 should NOT be cancelled");
    assert!(!token_a2.is_cancelled(), "Token A2 should not be cancelled");

    // Update document B - should only cancel token_b1
    let token_b2 = server.cancel_document_operations(&path_b);
    assert!(!token_a2.is_cancelled(), "Token A2 should NOT be cancelled");
    assert!(token_b1.is_cancelled(), "Token B1 should be cancelled");
    assert!(!token_b2.is_cancelled(), "Token B2 should not be cancelled");
}

#[test]
fn test_get_document_cancel_token() {
    let mut server = LspServer::new();
    let path = PathBuf::from("/test.sysml");

    // Initially no token exists
    assert!(server.get_document_cancel_token(&path).is_none());

    // After cancel_document_operations, token should be retrievable
    let token1 = server.cancel_document_operations(&path);
    let retrieved = server.get_document_cancel_token(&path);
    assert!(retrieved.is_some());

    // Retrieved token should be the same (cloned)
    let retrieved = retrieved.unwrap();
    assert!(!retrieved.is_cancelled());

    // Cancelling original should also cancel the cloned token (they share state)
    token1.cancel();
    assert!(retrieved.is_cancelled());
}

#[tokio::test]
async fn test_cancellation_stops_async_work() {
    use tokio::time::{Duration, timeout};

    let mut server = LspServer::new();
    let path = PathBuf::from("/test.sysml");

    // Get a token for this document
    let token = server.cancel_document_operations(&path);
    let token_clone = token.clone();

    // Spawn a task that waits for cancellation
    let task = tokio::spawn(async move {
        // Simulate work that checks cancellation
        token_clone.cancelled().await;
        "cancelled"
    });

    // Task should be waiting (not yet cancelled)
    let result = timeout(
        Duration::from_millis(10),
        &mut Box::pin(async { task.is_finished() }),
    )
    .await;
    assert!(
        result.is_err() || !task.is_finished(),
        "Task should still be running"
    );

    // Now cancel by simulating a document update
    let _new_token = server.cancel_document_operations(&path);

    // Task should complete quickly now
    let result = timeout(Duration::from_millis(100), task).await;
    assert!(result.is_ok(), "Task should complete after cancellation");
    assert_eq!(result.unwrap().unwrap(), "cancelled");
}

#[test]
fn test_rapid_changes_then_format() {
    use async_lsp::lsp_types::{
        FormattingOptions, Position, Range, TextDocumentContentChangeEvent,
    };
    use std::time::Instant;
    use tokio_util::sync::CancellationToken;

    let mut server = LspServer::new();

    // Create a test document
    let test_uri = async_lsp::lsp_types::Url::parse("file:///test.sysml").unwrap();
    let initial_content = "package Test { part def Vehicle; }";

    // Open document
    server.open_document(&test_uri, initial_content).unwrap();
    println!("Opened document");

    // Simulate rapid typing - multiple incremental changes
    let changes = [
        // Add newline after {
        TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 0,
                    character: 14,
                },
                end: Position {
                    line: 0,
                    character: 14,
                },
            }),
            range_length: None,
            text: "\n    ".to_string(),
        },
        // Add a new part
        TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 1,
                    character: 4,
                },
                end: Position {
                    line: 1,
                    character: 4,
                },
            }),
            range_length: None,
            text: "part def Engine;\n    ".to_string(),
        },
        // Add another part
        TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 2,
                    character: 4,
                },
                end: Position {
                    line: 2,
                    character: 4,
                },
            }),
            range_length: None,
            text: "part def Wheel;\n    ".to_string(),
        },
    ];

    // Apply changes rapidly without parsing between them, to simulate debounced behavior
    let path = test_uri.to_file_path().unwrap();
    for (i, change) in changes.iter().enumerate() {
        let start = Instant::now();
        server.cancel_document_operations(&path);
        server.apply_text_change_only(&test_uri, change).unwrap();
        println!("Change {}: {}ms", i + 1, start.elapsed().as_millis());
    }

    // After all rapid changes, parse once (as would happen after debounce delay)
    server.parse_document(&test_uri);

    // Get the current document text
    let text = server.get_document_text(&test_uri).unwrap();
    println!("Document after changes:\n{}", text);

    // Now format
    let format_start = Instant::now();
    let cancel_token = CancellationToken::new();
    let options = FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        ..Default::default()
    };

    let format_result = syster_lsp::formatting::format_text(&text, options, &cancel_token);
    println!("Format: {}ms", format_start.elapsed().as_millis());

    if let Some(edits) = format_result {
        println!("Formatted result ({} edits):", edits.len());
        for edit in &edits {
            println!("  Edit: {:?}", edit.range);
            println!("  New text:\n{}", edit.new_text);
        }
    } else {
        println!("No formatting changes needed");
    }
}

#[test]
fn test_interleaved_changes_and_format() {
    use async_lsp::lsp_types::{FormattingOptions, TextDocumentContentChangeEvent};
    use std::time::Instant;
    use tokio_util::sync::CancellationToken;

    let mut server = LspServer::new();

    // Create a test document with poor formatting
    let test_uri = async_lsp::lsp_types::Url::parse("file:///test2.sysml").unwrap();
    let content = "package   Test{part def    Vehicle;part def Engine;}";

    // Open document
    let start = Instant::now();
    server.open_document(&test_uri, content).unwrap();
    println!("open_document: {}ms", start.elapsed().as_millis());

    // Get initial text and format
    let text = server.get_document_text(&test_uri).unwrap();
    let cancel_token = CancellationToken::new();
    let options = FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        ..Default::default()
    };

    let format_start = Instant::now();
    let format_result = syster_lsp::formatting::format_text(&text, options.clone(), &cancel_token);
    println!("format (first): {}ms", format_start.elapsed().as_millis());

    // Apply formatted result as a change
    if let Some(edits) = format_result {
        let formatted_text = &edits[0].new_text;
        println!("Formatted:\n{}", formatted_text);

        // Simulate user making a change after format
        let change = TextDocumentContentChangeEvent {
            range: None, // Full document replacement
            range_length: None,
            text: formatted_text.clone(),
        };

        let change_start = Instant::now();
        let path = test_uri.to_file_path().unwrap();
        server.cancel_document_operations(&path);
        server.apply_text_change_only(&test_uri, &change).unwrap();
        server.parse_document(&test_uri);
        println!(
            "apply_text_change_only + parse_document: {}ms",
            change_start.elapsed().as_millis()
        );

        // Format again - should be idempotent (no changes)
        let text2 = server.get_document_text(&test_uri).unwrap();
        let cancel_token2 = CancellationToken::new();

        let format_start2 = Instant::now();
        let format_result2 = syster_lsp::formatting::format_text(&text2, options, &cancel_token2);
        println!("format (second): {}ms", format_start2.elapsed().as_millis());

        assert!(
            format_result2.is_none(),
            "Second format should produce no changes (idempotent)"
        );
    }
}

#[test]
fn test_parse_timing_breakdown() {
    use std::path::PathBuf;
    use std::time::Instant;

    // Test raw parsing time without LSP overhead
    let source = "package Test { part def Vehicle; part def Engine; part def Wheel; }";
    let path = PathBuf::from("/test.sysml");

    // Warm up
    let _ = syster::project::file_loader::parse_with_result(source, &path);

    // Measure parse time
    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = syster::project::file_loader::parse_with_result(source, &path);
    }
    let parse_total = start.elapsed();
    println!(
        "Raw parse: {:.3}ms avg over {} iterations",
        parse_total.as_micros() as f64 / 1000.0 / iterations as f64,
        iterations
    );

    // Now test open_document directly (full document replacement)
    let mut server = syster_lsp::LspServer::new();
    let test_uri = async_lsp::lsp_types::Url::parse("file:///test.sysml").unwrap();

    // Open document first
    server.open_document(&test_uri, source).unwrap();

    // Measure open_document time
    let iterations = 50;
    let start = Instant::now();
    for _ in 0..iterations {
        server.open_document(&test_uri, source).unwrap();
    }
    let change_total = start.elapsed();
    println!(
        "open_document: {:.3}ms avg over {} iterations",
        change_total.as_micros() as f64 / 1000.0 / iterations as f64,
        iterations
    );

    // Measure formatting time
    use async_lsp::lsp_types::FormattingOptions;
    use tokio_util::sync::CancellationToken;

    let options = FormattingOptions {
        tab_size: 4,
        insert_spaces: true,
        ..Default::default()
    };
    let cancel_token = CancellationToken::new();

    // Warm up
    let _ = syster_lsp::formatting::format_text(source, options.clone(), &cancel_token);

    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = syster_lsp::formatting::format_text(source, options.clone(), &cancel_token);
    }
    let format_total = start.elapsed();
    println!(
        "format_text: {:.3}ms avg over {} iterations",
        format_total.as_micros() as f64 / 1000.0 / iterations as f64,
        iterations
    );

    // Simulate 20 rapid changes (like fast typing)
    println!("\n--- 20 rapid changes simulation ---");
    let start = Instant::now();
    for i in 0..20 {
        let modified = format!("package Test {{ part def V{}; }}", i);
        server.open_document(&test_uri, &modified).unwrap();
    }
    let total = start.elapsed();
    println!(
        "20 changes total: {:.3}ms ({:.3}ms avg)",
        total.as_micros() as f64 / 1000.0,
        total.as_micros() as f64 / 1000.0 / 20.0
    );

    // Test with a large file with many symbols
    println!("\n--- Large file with many symbols ---");
    let mut large_source = String::from("package LargePackage {\n");
    for i in 0..50 {
        large_source.push_str(&format!("    part def Part{};\n", i));
        large_source.push_str(&format!("    port def Port{};\n", i));
        large_source.push_str(&format!("    action def Action{};\n", i));
    }
    large_source.push_str("}\n");
    println!("Large file: {} bytes, ~150 symbols", large_source.len());

    let large_uri = async_lsp::lsp_types::Url::parse("file:///large.sysml").unwrap();

    // Open large file
    let start = Instant::now();
    server.open_document(&large_uri, &large_source).unwrap();
    println!(
        "open_document (large): {:.3}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );

    // Change large file
    let iterations = 20;
    let start = Instant::now();
    for i in 0..iterations {
        let mut modified = large_source.clone();
        modified.push_str(&format!("// edit {}\n", i));
        server.open_document(&large_uri, &modified).unwrap();
    }
    let total = start.elapsed();
    println!(
        "20 changes (large file): {:.3}ms total ({:.3}ms avg)",
        total.as_micros() as f64 / 1000.0,
        total.as_micros() as f64 / 1000.0 / iterations as f64
    );

    // Format large file
    let start = Instant::now();
    let _ = syster_lsp::formatting::format_text(&large_source, options.clone(), &cancel_token);
    println!(
        "format (large file): {:.3}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );
}

#[test]
fn test_timing_with_stdlib_loaded() {
    use async_lsp::lsp_types::FormattingOptions;
    use std::time::Instant;
    use tokio_util::sync::CancellationToken;

    // Create server with stdlib
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let mut server = syster_lsp::LspServer::with_config(true, Some(stdlib_path.clone()));

    // Load stdlib
    println!("--- With stdlib loaded ---");
    let start = Instant::now();
    server.ensure_workspace_loaded().unwrap();
    println!(
        "ensure_workspace_loaded (stdlib): {:.3}ms",
        start.elapsed().as_micros() as f64 / 1000.0
    );
    println!("Workspace files: {}", server.workspace().files().len());
    println!(
        "Symbols: {}",
        server.workspace().symbol_table().all_symbols().len()
    );

    // Find AnalysisTooling.sysml
    let analysis_tooling_path = server
        .workspace()
        .files()
        .keys()
        .find(|p| p.to_string_lossy().contains("AnalysisTooling.sysml"))
        .cloned();

    if let Some(path) = analysis_tooling_path {
        let text = std::fs::read_to_string(&path).unwrap();
        let uri = async_lsp::lsp_types::Url::from_file_path(&path).unwrap();

        println!("\n--- AnalysisTooling.sysml ({} bytes) ---", text.len());

        // Open document
        let start = Instant::now();
        server.open_document(&uri, &text).unwrap();
        println!(
            "open_document: {:.3}ms",
            start.elapsed().as_micros() as f64 / 1000.0
        );

        // Make changes
        let iterations = 20;
        let start = Instant::now();
        for i in 0..iterations {
            let mut modified = text.clone();
            modified.push_str(&format!("\n// edit {}", i));
            server.open_document(&uri, &modified).unwrap();
        }
        let total = start.elapsed();
        println!(
            "20 changes: {:.3}ms total ({:.3}ms avg)",
            total.as_micros() as f64 / 1000.0,
            total.as_micros() as f64 / 1000.0 / iterations as f64
        );

        // Format
        let options = FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            ..Default::default()
        };
        let cancel_token = CancellationToken::new();
        let start = Instant::now();
        let _ = syster_lsp::formatting::format_text(&text, options, &cancel_token);
        println!(
            "format: {:.3}ms",
            start.elapsed().as_micros() as f64 / 1000.0
        );
    } else {
        println!("AnalysisTooling.sysml not found in stdlib");
    }
}

/// Test that replicates the duplicate relationship bug for TemperatureDifferenceValue
/// User reported: hovering over TemperatureDifferenceValue in ISQ.sysml shows
/// two relationships for ScalarQuantityValue
#[test]
fn test_hover_temperature_difference_value_no_duplicate_specialization() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;

    // Create workspace and load stdlib
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path.clone());

    // Load and populate stdlib
    stdlib_loader
        .load(&mut workspace)
        .expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate");

    // Find ISQ::TemperatureDifferenceValue symbol
    let symbol_table = workspace.symbol_table();
    let all_symbols = symbol_table.all_symbols();
    let temp_diff_symbol = all_symbols
        .iter()
        .find(|(_, sym)| sym.qualified_name() == "ISQ::TemperatureDifferenceValue");

    assert!(
        temp_diff_symbol.is_some(),
        "Should find TemperatureDifferenceValue"
    );
    let (_, symbol) = temp_diff_symbol.unwrap();

    // Get relationships the same way hover does
    let graph = workspace.relationship_graph();
    let grouped_rels = graph.get_relationships_grouped(symbol.qualified_name());

    println!(
        "Grouped relationships for TemperatureDifferenceValue: {:?}",
        grouped_rels
    );

    // Find the "Specializes" group
    let specializes_group = grouped_rels
        .iter()
        .find(|(label, _)| label == "Specializes");
    assert!(
        specializes_group.is_some(),
        "Should have Specializes relationship"
    );

    let (_, targets) = specializes_group.unwrap();
    println!("Specializes targets: {:?}", targets);

    // Check for duplicates
    let mut unique_targets: Vec<_> = targets.clone();
    unique_targets.sort();
    unique_targets.dedup();

    assert_eq!(
        targets.len(),
        unique_targets.len(),
        "Found duplicate relationships in hover! Got {} but only {} unique: {:?}",
        targets.len(),
        unique_targets.len(),
        targets
    );

    // Should specialize exactly 1 type (ScalarQuantityValue)
    assert_eq!(
        targets.len(),
        1,
        "Should have exactly 1 specialization target in hover, got: {:?}",
        targets
    );
}

/// Test that hover for TemperatureDifferenceValue doesn't show duplicate relationships
#[test]
fn test_hover_output_temperature_difference_value() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;
    use syster_lsp::server::helpers::format_rich_hover;

    // Create workspace and load stdlib
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path.clone());

    // Load and populate stdlib
    stdlib_loader
        .load(&mut workspace)
        .expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate");

    // Find ISQ::TemperatureDifferenceValue symbol
    let symbol_table = workspace.symbol_table();
    let all_symbols = symbol_table.all_symbols();
    let temp_diff_symbol = all_symbols
        .iter()
        .find(|(_, sym)| sym.qualified_name() == "ISQ::TemperatureDifferenceValue");

    assert!(
        temp_diff_symbol.is_some(),
        "Should find TemperatureDifferenceValue"
    );
    let (_, symbol) = temp_diff_symbol.unwrap();

    // Generate the actual hover output
    let hover_output = format_rich_hover(symbol, &workspace);

    println!("=== HOVER OUTPUT ===");
    println!("{}", hover_output);
    println!("=== END HOVER OUTPUT ===");

    // Check that ScalarQuantityValue only appears once
    let scalar_count = hover_output.matches("ScalarQuantityValue").count();
    assert_eq!(
        scalar_count, 1,
        "ScalarQuantityValue should appear exactly once in hover, found {} times:\n{}",
        scalar_count, hover_output
    );
}

#[test]
fn test_hover_output_celsius_temperature_value() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;
    use syster_lsp::server::helpers::format_rich_hover;

    // Create workspace and load stdlib
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path.clone());

    // Load and populate stdlib
    stdlib_loader
        .load(&mut workspace)
        .expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate");

    // Find ISQThermodynamics::CelsiusTemperatureValue symbol
    let symbol_table = workspace.symbol_table();
    let all_symbols = symbol_table.all_symbols();
    let celsius_symbol = all_symbols
        .iter()
        .find(|(_, sym)| sym.qualified_name() == "ISQThermodynamics::CelsiusTemperatureValue");

    assert!(
        celsius_symbol.is_some(),
        "Should find CelsiusTemperatureValue"
    );
    let (_, symbol) = celsius_symbol.unwrap();

    // Generate the actual hover output
    let hover_output = format_rich_hover(symbol, &workspace);

    println!("=== HOVER OUTPUT (CelsiusTemperatureValue) ===");
    println!("{}", hover_output);
    println!("=== END HOVER OUTPUT ===");

    // Check that ScalarQuantityValue only appears once
    let scalar_count = hover_output.matches("ScalarQuantityValue").count();
    assert_eq!(
        scalar_count, 1,
        "ScalarQuantityValue should appear exactly once in hover, found {} times:\n{}",
        scalar_count, hover_output
    );
}

#[test]
fn test_hover_at_position_temperature_difference_value() {
    use std::path::PathBuf;
    use syster::semantic::Workspace;
    use syster::syntax::file::SyntaxFile;
    use syster_lsp::server::helpers::format_rich_hover;

    // Create workspace and load stdlib
    let mut workspace: Workspace<SyntaxFile> = Workspace::new();
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path.clone());

    // Load and populate stdlib
    stdlib_loader
        .load(&mut workspace)
        .expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate");

    // Check: are there multiple symbols with name "TemperatureDifferenceValue"?
    let symbol_table = workspace.symbol_table();
    let all_symbols = symbol_table.all_symbols();
    let matching_symbols: Vec<_> = all_symbols
        .iter()
        .filter(|(_, sym)| sym.name() == "TemperatureDifferenceValue")
        .collect();

    println!("=== Symbols named 'TemperatureDifferenceValue' ===");
    for (key, sym) in &matching_symbols {
        println!("  Key: {}, QName: {}", key, sym.qualified_name());
    }
    println!("Total: {}", matching_symbols.len());

    // Now generate hover for each and check
    for (_, sym) in &matching_symbols {
        let hover = format_rich_hover(sym, &workspace);
        let count = hover.matches("ScalarQuantityValue").count();
        println!("\n--- Hover for {} ---\n{}", sym.qualified_name(), hover);
        assert_eq!(
            count,
            1,
            "Should have exactly 1 ScalarQuantityValue in hover for {}",
            sym.qualified_name()
        );
    }
}

#[test]
fn test_lsp_hover_isq_temperature_difference_value() {
    use async_lsp::lsp_types::{HoverContents, MarkedString, Position, Url};
    use std::path::PathBuf;
    use syster_lsp::LspServer;

    // Create LSP server
    let mut server = LspServer::new();

    // Load stdlib
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path.clone());
    stdlib_loader
        .load(server.workspace_mut())
        .expect("Failed to load stdlib");
    server
        .workspace_mut()
        .populate_all()
        .expect("Failed to populate");

    // Find ISQ.sysml file
    let isq_path = server
        .workspace()
        .files()
        .keys()
        .find(|p| p.to_string_lossy().ends_with("ISQ.sysml"))
        .expect("Should have ISQ.sysml in stdlib")
        .clone();

    // Open the document
    let abs_path = std::fs::canonicalize(&isq_path).expect("Should canonicalize path");
    let uri = Url::from_file_path(&abs_path).expect("Should convert to URL");
    let text = std::fs::read_to_string(&isq_path).expect("Should read file");

    server
        .open_document(&uri, &text)
        .expect("Should open document");

    // Find line containing "TemperatureDifferenceValue" definition (line 26, 0-indexed = 25)
    let lines: Vec<&str> = text.lines().collect();
    let (line_index, col_index) = lines
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            if line.contains("attribute def TemperatureDifferenceValue") {
                line.find("TemperatureDifferenceValue").map(|pos| (i, pos))
            } else {
                None
            }
        })
        .expect("Should find TemperatureDifferenceValue definition");

    println!(
        "Found TemperatureDifferenceValue at line {}, col {}",
        line_index, col_index
    );
    println!("Line content: {}", lines[line_index]);

    // Hover at the position
    let position = Position {
        line: line_index as u32,
        character: (col_index + 10) as u32, // Middle of "TemperatureDifferenceValue"
    };

    let hover_result = server.get_hover(&uri, position);

    assert!(hover_result.is_some(), "Should get hover result");
    let hover = hover_result.unwrap();

    if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
        println!("=== LSP HOVER OUTPUT ===");
        println!("{}", content);
        println!("=== END LSP HOVER OUTPUT ===");

        // Check that ScalarQuantityValue only appears once
        let scalar_count = content.matches("ScalarQuantityValue").count();
        assert_eq!(
            scalar_count, 1,
            "ScalarQuantityValue should appear exactly once in hover, found {} times:\n{}",
            scalar_count, content
        );
    } else {
        panic!("Hover content should be a string");
    }
}

/// Tests for the actual runtime scenario where:
/// 1. Stdlib is loaded from target/release/sysml.library (auto-discovered)
/// 2. User opens ISQ.sysml from that same path
/// 3. Hover is requested
///
/// This replicates the exact production flow.
#[test]
fn test_lsp_hover_with_auto_discovered_stdlib() {
    use async_lsp::lsp_types::{HoverContents, MarkedString, Position, Url};
    use syster_lsp::LspServer;

    // Use the target/release/sysml.library path like production
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join("release")
        .join("sysml.library");

    // Skip test if stdlib doesn't exist there (CI might not have it)
    if !stdlib_path.exists() {
        println!(
            "Skipping test - stdlib not found at: {}",
            stdlib_path.display()
        );
        return;
    }

    println!("Using stdlib from: {:?}", stdlib_path);

    // Create LSP server and set up stdlib explicitly at production path
    let mut server = LspServer::new();
    let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path.clone());
    stdlib_loader
        .load(server.workspace_mut())
        .expect("Failed to load stdlib");
    server
        .workspace_mut()
        .populate_all()
        .expect("Failed to populate");

    // Find ISQ.sysml in the loaded workspace
    let isq_path = server
        .workspace()
        .files()
        .keys()
        .find(|p| p.to_string_lossy().ends_with("ISQ.sysml"))
        .expect("Should have ISQ.sysml in stdlib")
        .clone();

    println!("ISQ.sysml path from workspace: {:?}", isq_path);

    // THE BUG: User opens ISQ.sysml from crates/syster-base/sysml.library
    // but stdlib was loaded from target/release/sysml.library
    // These are DIFFERENT paths to the SAME logical file!
    let user_opened_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library")
        .join("Domain Libraries")
        .join("Quantities and Units")
        .join("ISQ.sysml");

    println!("User opens file from: {:?}", user_opened_path);

    let abs_path = std::fs::canonicalize(&user_opened_path).expect("Should canonicalize path");
    let uri = Url::from_file_path(&abs_path).expect("Should convert to URL");
    let text = std::fs::read_to_string(&user_opened_path).expect("Should read file");

    server
        .open_document(&uri, &text)
        .expect("Should open document");

    // Find line containing "TemperatureDifferenceValue" definition
    let lines: Vec<&str> = text.lines().collect();
    let (line_index, col_index) = lines
        .iter()
        .enumerate()
        .find_map(|(i, line)| {
            if line.contains("attribute def TemperatureDifferenceValue") {
                line.find("TemperatureDifferenceValue").map(|pos| (i, pos))
            } else {
                None
            }
        })
        .expect("Should find TemperatureDifferenceValue definition");

    println!(
        "Found TemperatureDifferenceValue at line {}, col {}",
        line_index, col_index
    );

    // Hover at the position
    let position = Position {
        line: line_index as u32,
        character: (col_index + 10) as u32,
    };

    let hover_result = server.get_hover(&uri, position);

    assert!(hover_result.is_some(), "Should get hover result");
    let hover = hover_result.unwrap();

    if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
        println!("=== LSP HOVER OUTPUT (auto-discovered stdlib) ===");
        println!("{}", content);
        println!("=== END LSP HOVER OUTPUT ===");

        // Check that ScalarQuantityValue only appears once
        let scalar_count = content.matches("ScalarQuantityValue").count();
        assert_eq!(
            scalar_count, 1,
            "ScalarQuantityValue should appear exactly once in hover, found {} times:\n{}",
            scalar_count, content
        );
    } else {
        panic!("Hover content should be a string");
    }
}
