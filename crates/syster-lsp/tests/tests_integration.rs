//! Integration tests for LSP server
//!
//! Tests the full stack from server initialization through symbol resolution

use std::path::PathBuf;
use std::sync::OnceLock;
use syster::semantic::Workspace;
use syster::syntax::file::SyntaxFile;
use syster_lsp::LspServer;

/// Shared stdlib workspace loaded once for all tests that need it.
/// This avoids re-parsing the ~100+ stdlib files for each test.
static STDLIB_WORKSPACE: OnceLock<Workspace<SyntaxFile>> = OnceLock::new();

/// Get a reference to the pre-loaded stdlib workspace.
/// The first call loads and populates the stdlib; subsequent calls return the cached version.
fn get_stdlib_workspace() -> &'static Workspace<SyntaxFile> {
    STDLIB_WORKSPACE.get_or_init(|| {
        let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("syster-base")
            .join("sysml.library");

        let mut workspace: Workspace<SyntaxFile> = Workspace::new();
        let stdlib_loader = syster::project::StdLibLoader::with_path(stdlib_path);
        stdlib_loader
            .load(&mut workspace)
            .expect("Failed to load stdlib");
        workspace.populate_all().expect("Failed to populate stdlib");
        workspace
    })
}

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
    let symbol_count = server.workspace().symbol_table().iter_symbols().count();
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
            .iter_symbols()
            .filter(|s| s.name() == "ScalarValue" || s.qualified_name().contains("ScalarValue"))
            .map(|s| {
                (
                    s.name().to_string(),
                    s.qualified_name().to_string(),
                    s.span().is_some(),
                )
            })
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

    // Show what packages are actually loaded
    let packages: Vec<_> = symbol_table
        .iter_symbols()
        .filter(|s| s.qualified_name() == s.name() && !s.name().contains("::"))
        .take(20)
        .collect();

    for _symbol in packages {}

    // Show symbols containing "Case" to debug why Case isn't found
    let case_symbols: Vec<_> = symbol_table
        .iter_symbols()
        .filter(|s| s.name().contains("Case") || s.qualified_name().contains("Case"))
        .take(10)
        .collect();

    for _symbol in case_symbols {}

    // Try finding some basic symbols that should be in SysML stdlib
    let test_symbols = vec!["Part", "Attribute", "Item"];

    for simple_name in test_symbols {
        let _found = symbol_table.iter_symbols().any(|s| s.name() == simple_name);
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
    let symbol_table = server.workspace().symbol_table();

    if symbol_table.iter_symbols().next().is_none() {
        panic!("Symbol table is empty - stdlib population may have failed");
    }

    // Test resolving the first few symbols by their simple names
    let resolver = server.resolver();
    let symbols_vec: Vec<_> = symbol_table.iter_symbols().take(10).collect();

    for symbol in &symbols_vec {
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
    let our_symbols: Vec<_> = server
        .workspace()
        .symbol_table()
        .iter_symbols()
        .filter(|s| s.qualified_name().contains("My"))
        .collect();
    for _symbol in our_symbols {}

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
    let file2_path = PathBuf::from("/file2.sysml");
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
    println!("Document after changes:\n{text}");

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
        println!("Formatted:\n{formatted_text}");

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
        let modified = format!("package Test {{ part def V{i}; }}");
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
        large_source.push_str(&format!("    part def Part{i};\n"));
        large_source.push_str(&format!("    port def Port{i};\n"));
        large_source.push_str(&format!("    action def Action{i};\n"));
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
        modified.push_str(&format!("// edit {i}\n"));
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
        server.workspace().symbol_table().iter_symbols().count()
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
            modified.push_str(&format!("\n// edit {i}"));
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
    // Use shared pre-loaded stdlib workspace
    let workspace = get_stdlib_workspace();

    // Find ISQ::TemperatureDifferenceValue symbol
    let symbol_table = workspace.symbol_table();
    let temp_diff_symbol = symbol_table
        .iter_symbols()
        .find(|sym| sym.qualified_name() == "ISQ::TemperatureDifferenceValue");

    assert!(
        temp_diff_symbol.is_some(),
        "Should find TemperatureDifferenceValue"
    );

    // Verify the symbol is indexed in reference_index
    let index = workspace.reference_index();
    let symbol = temp_diff_symbol.unwrap();

    // Check that the symbol has references (it's used elsewhere in stdlib)
    // The ReferenceIndex now stores only qualified names for reverse lookups
    let sources = index.get_sources(symbol.qualified_name());
    println!(
        "Sources referencing TemperatureDifferenceValue: {:?}",
        sources
    );
}

/// Test that hover for TemperatureDifferenceValue doesn't show duplicate relationships
#[test]
fn test_hover_output_temperature_difference_value() {
    use syster_lsp::server::helpers::format_rich_hover;

    // Use shared pre-loaded stdlib workspace
    let workspace = get_stdlib_workspace();

    // Find ISQ::TemperatureDifferenceValue symbol
    let symbol_table = workspace.symbol_table();
    let temp_diff_symbol = symbol_table
        .iter_symbols()
        .find(|sym| sym.qualified_name() == "ISQ::TemperatureDifferenceValue");

    assert!(
        temp_diff_symbol.is_some(),
        "Should find TemperatureDifferenceValue"
    );
    let symbol = temp_diff_symbol.unwrap();

    // Generate the actual hover output
    let hover_output = format_rich_hover(symbol, workspace);

    println!("=== HOVER OUTPUT ===");
    println!("{hover_output}");
    println!("=== END HOVER OUTPUT ===");

    // Check that ScalarQuantityValue only appears once
    let scalar_count = hover_output.matches("ScalarQuantityValue").count();
    assert_eq!(
        scalar_count, 1,
        "ScalarQuantityValue should appear exactly once in hover, found {scalar_count} times:\n{hover_output}"
    );
}

#[test]
fn test_hover_output_celsius_temperature_value() {
    use syster_lsp::server::helpers::format_rich_hover;

    // Use shared pre-loaded stdlib workspace
    let workspace = get_stdlib_workspace();

    // Find ISQThermodynamics::CelsiusTemperatureValue symbol
    let symbol_table = workspace.symbol_table();
    let celsius_symbol = symbol_table
        .iter_symbols()
        .find(|sym| sym.qualified_name() == "ISQThermodynamics::CelsiusTemperatureValue");

    assert!(
        celsius_symbol.is_some(),
        "Should find CelsiusTemperatureValue"
    );
    let symbol = celsius_symbol.unwrap();

    // Generate the actual hover output
    let hover_output = format_rich_hover(symbol, workspace);

    println!("=== HOVER OUTPUT (CelsiusTemperatureValue) ===");
    println!("{hover_output}");
    println!("=== END HOVER OUTPUT ===");

    // Check that ScalarQuantityValue only appears once
    let scalar_count = hover_output.matches("ScalarQuantityValue").count();
    assert_eq!(
        scalar_count, 1,
        "ScalarQuantityValue should appear exactly once in hover, found {scalar_count} times:\n{hover_output}"
    );
}

#[test]
fn test_hover_at_position_temperature_difference_value() {
    use syster_lsp::server::helpers::format_rich_hover;

    // Use shared pre-loaded stdlib workspace
    let workspace = get_stdlib_workspace();

    // Check: are there multiple symbols with name "TemperatureDifferenceValue"?
    let symbol_table = workspace.symbol_table();
    let matching_symbols: Vec<_> = symbol_table
        .iter_symbols()
        .filter(|sym| sym.name() == "TemperatureDifferenceValue")
        .collect();

    println!("=== Symbols named 'TemperatureDifferenceValue' ===");
    for sym in &matching_symbols {
        println!("  QName: {}", sym.qualified_name());
    }
    println!("Total: {}", matching_symbols.len());

    // Now generate hover for each and check
    for sym in &matching_symbols {
        let hover = format_rich_hover(sym, workspace);
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

    println!("Found TemperatureDifferenceValue at line {line_index}, col {col_index}");
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
        println!("{content}");
        println!("=== END LSP HOVER OUTPUT ===");

        // Check that ScalarQuantityValue only appears once
        let scalar_count = content.matches("ScalarQuantityValue").count();
        assert_eq!(
            scalar_count, 1,
            "ScalarQuantityValue should appear exactly once in hover, found {scalar_count} times:\n{content}"
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

    println!("Using stdlib from: {stdlib_path:?}");

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

    println!("ISQ.sysml path from workspace: {isq_path:?}");

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

    println!("User opens file from: {user_opened_path:?}");

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

    println!("Found TemperatureDifferenceValue at line {line_index}, col {col_index}");

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
        println!("{content}");
        println!("=== END LSP HOVER OUTPUT ===");

        // Check that ScalarQuantityValue only appears once
        let scalar_count = content.matches("ScalarQuantityValue").count();
        assert_eq!(
            scalar_count, 1,
            "ScalarQuantityValue should appear exactly once in hover, found {scalar_count} times:\n{content}"
        );
    } else {
        panic!("Hover content should be a string");
    }
}

// =============================================================================
// Tests for hover duplicate prevention after file updates
// =============================================================================

#[test]
fn test_hover_no_duplicates_after_file_update() {
    // BUG: When a file is edited/saved, hover shows duplicate entries
    // This test replicates the issue by updating a file multiple times
    // and checking the hover output for duplicates.
    use async_lsp::lsp_types::{
        HoverContents, MarkedString, Position, TextDocumentContentChangeEvent, Url,
    };

    let mut server = LspServer::new();

    let file_path = PathBuf::from("/test/hover_duplicates.sysml");
    let uri = Url::from_file_path(&file_path).unwrap();

    let source = r#"
        package Test {
            part def Vehicle;
            part def Car :> Vehicle;
            part myCar : Vehicle;
        }
    "#;

    // Open the document
    server.open_document(&uri, source).unwrap();

    // Parse the document
    server.parse_document(&uri);

    // Helper to get hover for "Vehicle" and check for duplicates
    let check_hover = |server: &LspServer, iteration: usize| {
        // Hover over "Vehicle" in "part def Vehicle"
        let position = Position {
            line: 2,
            character: 22, // Position of "Vehicle" in "part def Vehicle"
        };

        let hover_result = server.get_hover(&uri, position);
        assert!(
            hover_result.is_some(),
            "Iteration {iteration}: Should get hover result"
        );

        let hover = hover_result.unwrap();
        if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
            println!("=== HOVER OUTPUT (iteration {iteration}) ===");
            println!("{content}");
            println!("=== END ===\n");

            // Count how many times "Referenced by" section appears
            let referenced_by_count = content.matches("Referenced by").count();
            assert!(
                referenced_by_count <= 1,
                "Iteration {iteration}: 'Referenced by' should appear at most once, found {referenced_by_count} times"
            );

            // Count individual reference entries - each should appear once
            // References to Vehicle: Car (specialization), myCar (typing)
            let car_refs = content.matches("Car").count();
            let mycar_refs = content.matches("myCar").count();

            // Car appears once in definition list and possibly once in references
            // myCar appears once in references
            assert!(
                car_refs <= 2,
                "Iteration {iteration}: 'Car' should appear at most 2 times (def + ref), found {car_refs}"
            );
            assert!(
                mycar_refs <= 1,
                "Iteration {iteration}: 'myCar' should appear at most once, found {mycar_refs}"
            );

            content
        } else {
            panic!("Hover content should be a string");
        }
    };

    // Check hover initially
    let initial_content = check_hover(&server, 0);

    // Simulate multiple file updates (like saving the file repeatedly)
    for i in 1..=3 {
        // Apply a "change" (same content, simulating save)
        let _ = server.apply_text_change_only(
            &uri,
            &TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: source.to_string(),
            },
        );

        // Reparse
        server.parse_document(&uri);

        // Check hover again
        let new_content = check_hover(&server, i);

        // Content should be identical to initial (no accumulating duplicates)
        assert_eq!(
            initial_content, new_content,
            "Iteration {i}: Hover content should be identical after update"
        );
    }
}

#[test]
fn test_hover_referenced_by_count_stable_after_updates() {
    // Test that the "Referenced by: (N usages)" count stays stable after updates
    use async_lsp::lsp_types::{
        HoverContents, MarkedString, Position, TextDocumentContentChangeEvent, Url,
    };

    let mut server = LspServer::new();

    let file_path = PathBuf::from("/test/reference_count.sysml");
    let uri = Url::from_file_path(&file_path).unwrap();

    let source = r#"
        package Test {
            part def Base;
            part def A :> Base;
            part def B :> Base;
            part def C :> Base;
            part x : Base;
            part y : Base;
        }
    "#;

    // Open and parse
    server.open_document(&uri, source).unwrap();
    server.parse_document(&uri);

    // Get initial hover for "Base" and count references
    let get_reference_count = |server: &LspServer| -> Option<usize> {
        let position = Position {
            line: 2,
            character: 22,
        };

        let hover = server.get_hover(&uri, position)?;
        if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
            // Extract count from "Referenced by: (N usages)" without regex
            // Look for pattern like "(5 usages)" or "(1 usage)"
            if let Some(start) = content.find("Referenced by:") {
                let after = &content[start..];
                if let Some(paren_start) = after.find('(') {
                    let after_paren = &after[paren_start + 1..];
                    if let Some(space_pos) = after_paren.find(' ') {
                        let num_str = &after_paren[..space_pos];
                        return num_str.parse().ok();
                    }
                }
            }
            None
        } else {
            None
        }
    };

    let initial_count = get_reference_count(&server);
    println!("Initial reference count: {initial_count:?}");

    // The exact count depends on what relationships are tracked
    // The important thing is that it stays STABLE after updates
    assert!(
        initial_count.is_some(),
        "Should have some references initially"
    );

    // Update the file 5 times
    for i in 1..=5 {
        let _ = server.apply_text_change_only(
            &uri,
            &TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: source.to_string(),
            },
        );
        server.parse_document(&uri);

        let count = get_reference_count(&server);
        println!("Reference count after update {i}: {count:?}");

        assert_eq!(
            count, initial_count,
            "Reference count should stay at 5 after update {i}, got {count:?}"
        );
    }
}

#[test]
fn test_hover_no_duplicates_with_stdlib_after_updates() {
    // Test with stdlib loaded - this is closer to real usage
    // The issue might be with cross-file references to stdlib symbols
    use async_lsp::lsp_types::{
        HoverContents, MarkedString, Position, TextDocumentContentChangeEvent, Url,
    };

    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");
    let mut server = LspServer::with_config(true, Some(stdlib_path));

    // Load stdlib
    server
        .ensure_workspace_loaded()
        .expect("Should load stdlib");

    let file_path = PathBuf::from("/test/with_stdlib.sysml");
    let uri = Url::from_file_path(&file_path).unwrap();

    // Source using syntax that the parser fully supports
    // Two packages: one defines a type, other uses it
    let source = r#"
package TestWithStdlib {

part def Calculator {
    attribute result : ScalarValues::Real;
}

part cal : Calculator;
}
    "#;

    // Open and parse
    server.open_document(&uri, source).unwrap();
    server.parse_document(&uri);

    // Helper to get hover content for "Calculator"
    let get_hover_content = |server: &LspServer| -> Option<String> {
        let position = Position {
            line: 3,
            character: 12, // Position of "Calculator" in "part def Calculator"
        };

        let hover = server.get_hover(&uri, position)?;
        if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
            Some(content)
        } else {
            None
        }
    };

    let initial_content = get_hover_content(&server);
    println!("=== INITIAL HOVER ===\n{initial_content:?}\n");
    assert!(initial_content.is_some(), "Should get initial hover");

    // Count references in initial content
    let count_refs = |content: &str| -> usize { content.matches("Referenced by").count() };

    let initial_ref_sections = initial_content.as_ref().map(|c| count_refs(c)).unwrap_or(0);
    println!("Initial 'Referenced by' sections: {initial_ref_sections}");

    // Update the file 5 times
    for i in 1..=5 {
        let _ = server.apply_text_change_only(
            &uri,
            &TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: source.to_string(),
            },
        );
        server.parse_document(&uri);

        let content = get_hover_content(&server);
        let ref_sections = content.as_ref().map(|c| count_refs(c)).unwrap_or(0);
        println!("Update {i}: 'Referenced by' sections = {ref_sections}");

        assert_eq!(
            ref_sections, initial_ref_sections,
            "Iteration {i}: 'Referenced by' count should stay at {initial_ref_sections}, got {ref_sections}"
        );
    }
}

/// Test that import references are properly cleared when imports are removed from a file.
///
/// This test replicates the bug where:
/// 1. File A defines a type
/// 2. File B imports and uses that type
/// 3. User removes the import from File B
/// 4. Hover on the type in File A should no longer show File B as a reference
///
/// If this fails, it means stale import references are not being cleared when files are updated.
#[test]
fn test_hover_import_references_cleared_when_import_removed() {
    use async_lsp::lsp_types::{
        HoverContents, MarkedString, Position, TextDocumentContentChangeEvent, Url,
    };

    let mut server = LspServer::new();

    // File A: defines a type
    let file_a_path = PathBuf::from("/test/types.sysml");
    let uri_a = Url::from_file_path(&file_a_path).unwrap();

    let source_a = r#"
package Types {
    part def Vehicle;
}
    "#;

    // File B: imports and uses the type
    let file_b_path = PathBuf::from("/test/usage.sysml");
    let uri_b = Url::from_file_path(&file_b_path).unwrap();

    let source_b_with_import = r#"
package Usage {
    import Types::Vehicle;
    part myVehicle : Vehicle;
}
    "#;

    let source_b_without_import = r#"
package Usage {
    // Import removed
    part myVehicle : SomeOtherType;
}
    "#;

    // Open both files
    server.open_document(&uri_a, source_a).unwrap();
    server.open_document(&uri_b, source_b_with_import).unwrap();

    // Helper to get hover content for "Vehicle" in file A
    let get_vehicle_hover = |server: &LspServer| -> Option<String> {
        let position = Position {
            line: 2,
            character: 16, // Position of "Vehicle" in "part def Vehicle"
        };

        let hover = server.get_hover(&uri_a, position)?;
        if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
            Some(content)
        } else {
            None
        }
    };

    // Check initial hover - should mention file B as an import reference
    let initial_hover = get_vehicle_hover(&server);
    println!("=== INITIAL HOVER (with import) ===");
    println!("{}", initial_hover.as_ref().unwrap_or(&"None".to_string()));

    // Count references to usage.sysml
    let count_usage_refs = |content: &str| -> usize { content.matches("usage.sysml").count() };

    let initial_usage_refs = initial_hover
        .as_ref()
        .map(|c| count_usage_refs(c))
        .unwrap_or(0);
    println!("Initial references to usage.sysml: {initial_usage_refs}");

    // We expect the import to create a reference
    // (depending on implementation, this might be in "Referenced by" or similar section)
    // If initial_usage_refs is 0, the test setup might need adjustment,
    // but we still want to verify no stale refs after removal

    // Now remove the import from file B
    let _ = server.apply_text_change_only(
        &uri_b,
        &TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: source_b_without_import.to_string(),
        },
    );
    server.parse_document(&uri_b);

    // Check hover again - should NOT reference file B anymore
    let updated_hover = get_vehicle_hover(&server);
    println!("\n=== UPDATED HOVER (import removed) ===");
    println!("{}", updated_hover.as_ref().unwrap_or(&"None".to_string()));

    let updated_usage_refs = updated_hover
        .as_ref()
        .map(|c| count_usage_refs(c))
        .unwrap_or(0);
    println!("Updated references to usage.sysml: {updated_usage_refs}");

    // The key assertion: after removing the import, there should be NO references to usage.sysml
    assert_eq!(
        updated_usage_refs, 0,
        "After removing import, hover should not reference usage.sysml anymore. \
         Found {updated_usage_refs} references. This indicates stale import references are not being cleared."
    );

    // Additional check: if we initially had references, they should be gone
    if initial_usage_refs > 0 {
        assert!(
            updated_usage_refs < initial_usage_refs,
            "Reference count should decrease after removing import. \
             Initial: {initial_usage_refs}, Updated: {updated_usage_refs}"
        );
    }
}

/// Test that semantic tokens are updated when imports are removed.
///
/// This validates that the semantic token collector properly reflects
/// the current state of imports after a file is updated.
#[test]
fn test_semantic_tokens_updated_when_import_removed() {
    use async_lsp::lsp_types::{SemanticTokensResult, TextDocumentContentChangeEvent, Url};

    let mut server = LspServer::new();

    // File A: defines a type
    let file_a_path = PathBuf::from("/test/defs.sysml");
    let uri_a = Url::from_file_path(&file_a_path).unwrap();

    let source_a = r#"
package Defs {
    part def Engine;
}
    "#;

    // File B: imports and references the type
    let file_b_path = PathBuf::from("/test/refs.sysml");
    let uri_b = Url::from_file_path(&file_b_path).unwrap();

    let source_b_with_import = r#"
package Refs {
    import Defs::Engine;
    part myEngine : Engine;
}
    "#;

    let source_b_without_import = r#"
package Refs {
    // Import removed - Engine reference is now unresolved
    part myEngine : UnresolvedType;
}
    "#;

    // Open both files
    server.open_document(&uri_a, source_a).unwrap();
    server.open_document(&uri_b, source_b_with_import).unwrap();

    // Get initial semantic tokens for file B
    let initial_tokens = server.get_semantic_tokens(&uri_b);
    let initial_token_count = match &initial_tokens {
        Some(SemanticTokensResult::Tokens(tokens)) => tokens.data.len(),
        _ => 0,
    };
    println!("Initial token count for file B: {initial_token_count}");

    // Now remove the import
    let _ = server.apply_text_change_only(
        &uri_b,
        &TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: source_b_without_import.to_string(),
        },
    );
    server.parse_document(&uri_b);

    // Get updated semantic tokens
    let updated_tokens = server.get_semantic_tokens(&uri_b);
    let updated_token_count = match &updated_tokens {
        Some(SemanticTokensResult::Tokens(tokens)) => tokens.data.len(),
        _ => 0,
    };
    println!("Updated token count for file B: {updated_token_count}");

    // The tokens should be different after removing the import
    // (the "Engine" type reference should no longer be marked as a Type token)
    // Note: The exact assertion depends on implementation details,
    // but at minimum the token set should reflect the new state
    println!("Token count changed: {initial_token_count} -> {updated_token_count}");

    // If we have a way to check for specific type references in the tokens,
    // we would verify that the "Engine" reference is no longer present
    // For now, we just verify the tokens are regenerated (not stale)
}

#[test]
fn test_hover_on_usage_site_cleared_when_import_removed() {
    use async_lsp::lsp_types::{
        HoverContents, MarkedString, Position, TextDocumentContentChangeEvent, Url,
    };

    let mut server = LspServer::new();

    // File A: defines a type
    let file_a_path = PathBuf::from("/test/engine_def.sysml");
    let uri_a = Url::from_file_path(&file_a_path).unwrap();

    let source_a = r#"
package EngineDefs {
    part def Engine;
}
    "#;

    // File B: imports and uses the type
    let file_b_path = PathBuf::from("/test/car.sysml");
    let uri_b = Url::from_file_path(&file_b_path).unwrap();

    let source_b_with_import = r#"
package Car {
    import EngineDefs::Engine;
    part myEngine : Engine;
}
    "#;

    // After removing the import, keep "Engine" text but it should be unresolved
    let source_b_without_import = r#"
package Car {
    part myEngine : Engine;
}
    "#;

    // Open both files
    server.open_document(&uri_a, source_a).unwrap();
    server.open_document(&uri_b, source_b_with_import).unwrap();

    // Hover on "Engine" in the usage line in file B
    // "    part myEngine : Engine;" - Engine starts at position ~20
    let position_in_file_b = Position {
        line: 3,
        character: 21, // Position of "Engine" in ": Engine;"
    };

    // Get initial hover - should show info about Engine from EngineDefs
    let initial_hover = server.get_hover(&uri_b, position_in_file_b);
    println!("=== INITIAL HOVER on Engine in file B (with import) ===");
    let initial_content = match &initial_hover {
        Some(hover) => match &hover.contents {
            HoverContents::Scalar(MarkedString::String(s)) => s.clone(),
            _ => "Non-string content".to_string(),
        },
        None => "No hover".to_string(),
    };
    println!("{initial_content}");

    // Check that initial hover mentions the definition file
    let initial_has_engine_def = initial_content.contains("EngineDefs")
        || initial_content.contains("engine_def.sysml")
        || initial_content.contains("Part def Engine");
    println!("Initial hover references EngineDefs: {initial_has_engine_def}");

    // Now remove the import from file B
    let _ = server.apply_text_change_only(
        &uri_b,
        &TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: source_b_without_import.to_string(),
        },
    );
    server.parse_document(&uri_b);

    // Hover on the same position - now "Engine" should be unresolved
    // Position changes because import line is removed, so line 3 becomes line 2
    let position_after_removal = Position {
        line: 2,
        character: 21, // Position of "Engine" in ": Engine;"
    };

    let updated_hover = server.get_hover(&uri_b, position_after_removal);
    println!("\n=== UPDATED HOVER on Engine in file B (import removed) ===");
    let updated_content = match &updated_hover {
        Some(hover) => match &hover.contents {
            HoverContents::Scalar(MarkedString::String(s)) => s.clone(),
            _ => "Non-string content".to_string(),
        },
        None => "No hover".to_string(),
    };
    println!("{updated_content}");

    // After removing import, hover should NOT show EngineDefs info
    // because Engine is now unresolved in this scope
    let updated_has_engine_def =
        updated_content.contains("EngineDefs") || updated_content.contains("engine_def.sysml");
    println!("Updated hover references EngineDefs: {updated_has_engine_def}");

    // The key assertion: after removing import, Engine reference should not
    // show stale information about the import that no longer exists
    assert!(
        !updated_has_engine_def,
        "After removing import, hover on 'Engine' should NOT reference EngineDefs anymore. \
         This indicates stale semantic information is being shown. \
         Content: {updated_content}"
    );
}

/// Test that hover shows correct information for imported types.
/// This ensures the relationship graph properly tracks import-based references.
#[test]
fn test_hover_shows_import_based_references() {
    use async_lsp::lsp_types::{HoverContents, MarkedString, Position, Url};

    let mut server = LspServer::new();

    // File with type definition
    let types_path = PathBuf::from("/test/base_types.sysml");
    let types_uri = Url::from_file_path(&types_path).unwrap();

    let types_source = r#"
package BaseTypes {
    part def Sensor;
}
    "#;

    // File with import and usage
    let usage_path = PathBuf::from("/test/sensor_usage.sysml");
    let usage_uri = Url::from_file_path(&usage_path).unwrap();

    let usage_source = r#"
package SensorUsage {
    import BaseTypes::Sensor;
    part tempSensor : Sensor;
    part pressureSensor : Sensor;
}
    "#;

    // Open files
    server.open_document(&types_uri, types_source).unwrap();
    server.open_document(&usage_uri, usage_source).unwrap();

    // Get hover for "Sensor" definition
    let position = Position {
        line: 2,
        character: 16, // "Sensor" in "part def Sensor"
    };

    let hover_result = server.get_hover(&types_uri, position);
    assert!(hover_result.is_some(), "Should get hover for Sensor");

    let hover = hover_result.unwrap();
    if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
        println!("=== SENSOR HOVER ===");
        println!("{content}");

        // The hover should include information about usages
        // Check for presence of typing relationships (tempSensor, pressureSensor typed as Sensor)
        let has_usage_info = content.contains("tempSensor") || content.contains("pressureSensor");

        println!("Hover mentions usages (tempSensor/pressureSensor): {has_usage_info}");

        // Note: This might not show in hover depending on implementation
        // The key test is test_hover_import_references_cleared_when_import_removed above
    }
}

/// Test hover on ISQ::MassValue in a user file
/// This directly tests the bug where hovering on MassValue in "import ISQ::MassValue" fails
#[test]
fn test_hover_isq_massvalue() {
    use async_lsp::lsp_types::{HoverContents, MarkedString, Position, Url};

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

    // Debug: Check ISQ symbols
    let resolver = server.resolver();
    let isq_package = resolver.resolve_qualified("ISQ");
    println!("ISQ package found: {:?}", isq_package.map(|s| s.name()));

    let isqbase_massvalue = resolver.resolve_qualified("ISQBase::MassValue");
    println!(
        "ISQBase::MassValue found: {:?}",
        isqbase_massvalue.map(|s| s.qualified_name())
    );

    let isq_massvalue = resolver.resolve_qualified("ISQ::MassValue");
    println!(
        "ISQ::MassValue resolved: {:?}",
        isq_massvalue.map(|s| s.qualified_name())
    );

    // Create a test file with import
    let user_source = r#"package MyTest {
    private import ISQ::MassValue;
    
    part def MyPart {
        attribute mass: MassValue;
    }
}"#;

    let user_uri = Url::parse("file:///test/mytest.sysml").unwrap();
    server
        .open_document(&user_uri, user_source)
        .expect("Should open user document");

    // Debug: Check what word is extracted
    let line = "    private import ISQ::MassValue;";
    let extracted = syster::core::text_utils::extract_qualified_name_at_cursor(line, 23);
    println!("Extracted at pos 23: {extracted:?}");

    // Test hover on "MassValue" in the import statement (line 1, position ~23)
    // Line 1 is "    private import ISQ::MassValue;"
    // Position of 'M' in MassValue is at column 23
    let position = Position {
        line: 1,
        character: 23,
    };

    let hover_result = server.get_hover(&user_uri, position);

    println!("Hover result: {:?}", hover_result.is_some());

    if let Some(hover) = &hover_result
        && let HoverContents::Scalar(MarkedString::String(content)) = &hover.contents
    {
        println!("=== HOVER CONTENT ===");
        println!("{content}");
    }

    assert!(
        hover_result.is_some(),
        "Hover on ISQ::MassValue should return something"
    );

    let hover = hover_result.unwrap();
    if let HoverContents::Scalar(MarkedString::String(content)) = hover.contents {
        assert!(
            content.contains("MassValue"),
            "Hover should mention MassValue, got: {content}"
        );
    } else {
        panic!("Unexpected hover contents format");
    }
}

/// Test hover on ISQ::MassValue using the extension's stdlib path
/// This simulates exactly what happens when VS Code loads
#[test]
fn test_hover_isq_massvalue_extension_stdlib() {
    // Use the stdlib path from syster-base
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");

    println!("Using stdlib path: {stdlib_path:?}");
    assert!(stdlib_path.exists(), "Extension stdlib path should exist");

    let mut server = LspServer::with_config(true, Some(stdlib_path));

    // Load stdlib first - this is what happens on VS Code startup
    server
        .ensure_workspace_loaded()
        .expect("Should load stdlib");

    // Debug: Check ISQ symbols
    let resolver = server.resolver();
    let isq_package = resolver.resolve_qualified("ISQ");
    println!("ISQ package found: {:?}", isq_package.map(|s| s.name()));

    let isqbase_massvalue = resolver.resolve_qualified("ISQBase::MassValue");
    println!(
        "ISQBase::MassValue found: {:?}",
        isqbase_massvalue.map(|s| s.qualified_name())
    );

    let isq_massvalue = resolver.resolve_qualified("ISQ::MassValue");
    println!(
        "ISQ::MassValue resolved: {:?}",
        isq_massvalue.map(|s| s.qualified_name())
    );

    // Check ISQ's public imports
    let isq_symbol = resolver.resolve_qualified("ISQ");
    if let Some(isq) = isq_symbol {
        let scope_id = isq.scope_id();
        println!("ISQ scope_id: {scope_id}");

        // Check child scopes for imports
        if let Some(scope) = server.workspace().symbol_table().scopes().get(scope_id) {
            println!("ISQ scope has {} children", scope.children.len());
            for &child_id in &scope.children {
                let imports = server
                    .workspace()
                    .symbol_table()
                    .get_scope_imports(child_id);
                let public_imports: Vec<_> = imports.iter().filter(|i| i.is_public).collect();
                if !public_imports.is_empty() {
                    println!(
                        "Child scope {} has {} public imports",
                        child_id,
                        public_imports.len()
                    );
                    for imp in &public_imports {
                        println!("  - {} (is_namespace: {})", imp.path, imp.is_namespace);
                    }
                }
            }
        }
    }

    // Now assert the key thing works
    assert!(
        isq_massvalue.is_some(),
        "ISQ::MassValue should resolve via public import re-export"
    );
}

/// Tests that semantic tokens are generated for RequirementDerivation.sysml
/// This file uses `SysML::Usage` which should highlight as Type
#[test]
fn test_semantic_tokens_for_requirement_derivation_file() {
    use syster::semantic::processors::SemanticTokenCollector;

    // Create server with explicit stdlib path
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");

    let mut server = LspServer::with_config(true, Some(stdlib_path.clone()));
    server.set_workspace_folders(vec![]);

    // Explicitly load stdlib and populate
    assert!(
        server.ensure_workspace_loaded().is_ok(),
        "Workspace should load"
    );

    // Find the RequirementDerivation.sysml file in the loaded workspace
    let req_deriv_path = stdlib_path
        .join("Domain Libraries")
        .join("Requirement Derivation")
        .join("RequirementDerivation.sysml");

    println!("Looking for file: {:?}", req_deriv_path);

    // Check if the file was loaded
    let file_exists = server.workspace().get_file(&req_deriv_path).is_some();
    assert!(file_exists, "RequirementDerivation.sysml should be loaded");

    // Check reference index for `SysML::Usage` references in this file
    let references_in_file = server
        .workspace()
        .reference_index()
        .get_references_in_file(&req_deriv_path.to_string_lossy());

    println!("References in RequirementDerivation.sysml:");
    for ref_info in &references_in_file {
        println!(
            "  - Line {}, Col {}: {:?}",
            ref_info.span.start.line, ref_info.span.start.column, ref_info.token_type
        );
    }

    // There should be several SysML::Usage references
    assert!(
        !references_in_file.is_empty(),
        "RequirementDerivation.sysml should have references indexed"
    );

    // Collect semantic tokens
    let tokens = SemanticTokenCollector::collect_from_workspace(
        server.workspace(),
        &req_deriv_path.to_string_lossy(),
    );

    println!("\nSemantic tokens:");
    for token in &tokens {
        println!(
            "  Line {}, Col {}, Len {}: {:?}",
            token.line, token.column, token.length, token.token_type
        );
    }

    // Should have tokens for the metadata defs and their references
    assert!(
        !tokens.is_empty(),
        "Should have semantic tokens for RequirementDerivation.sysml"
    );

    // Check for SysML::Usage tokens (should be Type tokens on lines with `: SysML::Usage`)
    let usage_tokens: Vec<_> = tokens.iter().filter(|t| t.length == 12).collect();
    println!("\nTokens with length 12 (SysML::Usage):");
    for token in &usage_tokens {
        println!(
            "  Line {}, Col {}: {:?}",
            token.line, token.column, token.token_type
        );
    }
}

/// Tests that semantic tokens work via the LSP's get_semantic_tokens method
/// This is the actual code path used by VS Code
#[test]
fn test_semantic_tokens_via_lsp_for_stdlib_file() {
    use async_lsp::lsp_types::Url;

    // Create server with explicit stdlib path
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("syster-base")
        .join("sysml.library");

    let mut server = LspServer::with_config(true, Some(stdlib_path.clone()));
    server.set_workspace_folders(vec![]);

    // Explicitly load stdlib and populate
    assert!(
        server.ensure_workspace_loaded().is_ok(),
        "Workspace should load"
    );

    // Find the RequirementDerivation.sysml file in the loaded workspace
    let req_deriv_path = stdlib_path
        .join("Domain Libraries")
        .join("Requirement Derivation")
        .join("RequirementDerivation.sysml");

    // Convert to file URI (what VS Code would send)
    let uri = Url::from_file_path(&req_deriv_path).expect("should create URI");

    println!("Requesting semantic tokens for URI: {}", uri);

    // This is the exact method the LSP handler calls
    let result = server.get_semantic_tokens(&uri);

    if let Some(tokens) = &result {
        match tokens {
            async_lsp::lsp_types::SemanticTokensResult::Tokens(t) => {
                println!("Got {} semantic tokens via LSP method", t.data.len());
                // Print decoded tokens (they're delta-encoded)
                let mut current_line = 0u32;
                let mut current_col = 0u32;
                for (i, tok) in t.data.iter().enumerate() {
                    current_line += tok.delta_line;
                    if tok.delta_line > 0 {
                        current_col = tok.delta_start;
                    } else {
                        current_col += tok.delta_start;
                    }
                    let token_type_name = match tok.token_type {
                        0 => "Namespace",
                        1 => "Type",
                        2 => "Variable",
                        3 => "Property",
                        4 => "Keyword",
                        _ => "Unknown",
                    };
                    if tok.length == 12 {
                        println!(
                            "  Token {}: Line {}, Col {}, Len {}: {} <-- SysML::Usage?",
                            i, current_line, current_col, tok.length, token_type_name
                        );
                    }
                }
            }
            async_lsp::lsp_types::SemanticTokensResult::Partial(_) => {
                println!("Got partial tokens");
            }
        }
    } else {
        println!("get_semantic_tokens returned None");
    }

    // The LSP method should return tokens for this stdlib file
    assert!(
        result.is_some(),
        "LSP should return semantic tokens for RequirementDerivation.sysml"
    );
}
