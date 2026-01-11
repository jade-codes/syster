use crate::project::StdLibLoader;
use crate::semantic::Workspace;
use crate::syntax::SyntaxFile;
use std::path::PathBuf;

#[test]
fn test_stdlib_calculation_symbol_loads() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    // Check if Calculation symbol exists
    let calculation = workspace
        .symbol_table()
        .iter_symbols()
        .find(|sym| sym.name() == "Calculation");

    assert!(
        calculation.is_some(),
        "Calculation symbol should be in symbol table. Found {} symbols total",
        workspace.symbol_table().iter_symbols().count()
    );

    let _sym = calculation.unwrap();
}

#[test]
fn test_stdlib_case_symbol_loads() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    // Check if Case symbol exists
    let case = workspace
        .symbol_table()
        .iter_symbols()
        .find(|sym| sym.name() == "Case");

    assert!(
        case.is_some(),
        "Case symbol should be in symbol table. Found {} symbols total",
        workspace.symbol_table().iter_symbols().count()
    );

    let _sym = case.unwrap();
}

#[test]
fn test_stdlib_symbol_count() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    let symbol_count = workspace.symbol_table().iter_symbols().count();

    // We should have significantly more symbols now with Items, Cases, etc
    assert!(
        symbol_count >= 1451,
        "Expected at least 1451 symbols, found {symbol_count}"
    );

    // Print first 20 symbols for debugging
    for (i, _sym) in workspace.symbol_table().iter_symbols().enumerate() {
        if i >= 20 {
            break;
        }
    }
}

#[test]
fn test_stdlib_si_symbols() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    // Check if SI package exists
    let si_package = workspace.symbol_table().find_by_qualified_name("SI");
    assert!(
        si_package.is_some(),
        "SI package should exist in symbol table"
    );

    // Get all SI symbols
    let si_symbols: Vec<_> = workspace
        .symbol_table()
        .iter_symbols()
        .filter(|sym| sym.qualified_name().starts_with("SI::"))
        .take(30)
        .map(|sym| sym.qualified_name())
        .collect();

    // Check for gram (without short name)
    let gram = workspace.symbol_table().find_by_qualified_name("SI::gram");
    assert!(
        gram.is_some(),
        "SI::gram should exist. Found SI symbols: {si_symbols:?}"
    );

    // Check for kilogram (without short name)
    let kilogram = workspace
        .symbol_table()
        .find_by_qualified_name("SI::kilogram");
    assert!(
        kilogram.is_some(),
        "SI::kilogram should exist. Found SI symbols: {si_symbols:?}"
    );
}

#[test]
fn test_stdlib_isq_massvalue() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    // Debug: list all top-level packages
    let packages: Vec<_> = workspace
        .symbol_table()
        .iter_symbols()
        .filter(|sym| matches!(sym, crate::semantic::symbol_table::Symbol::Package { .. }))
        .filter(|sym| !sym.qualified_name().contains("::"))
        .map(|sym| sym.qualified_name())
        .collect();
    println!("Top-level packages: {packages:?}");

    // Check if ISQ package exists
    let isq_package = workspace.symbol_table().find_by_qualified_name("ISQ");
    println!("ISQ package: {:?}", isq_package.map(|s| s.name()));

    // Get all ISQ symbols
    let isq_symbols: Vec<_> = workspace
        .symbol_table()
        .iter_symbols()
        .filter(|sym| {
            sym.qualified_name().starts_with("ISQ::")
                || sym.qualified_name().starts_with("ISQBase::")
        })
        .take(30)
        .map(|sym| sym.qualified_name())
        .collect();
    println!("ISQ/ISQBase symbols: {isq_symbols:?}");

    // Check ISQBase::MassValue directly
    let isqbase_mass_value = workspace
        .symbol_table()
        .find_by_qualified_name("ISQBase::MassValue");
    assert!(
        isqbase_mass_value.is_some(),
        "ISQBase::MassValue should exist. Symbols: {isq_symbols:?}"
    );

    // Test that ISQ::MassValue resolves via public re-export
    let resolver = crate::semantic::resolver::Resolver::new(workspace.symbol_table());
    let isq_mass_value = resolver.resolve_qualified("ISQ::MassValue");
    assert!(
        isq_mass_value.is_some(),
        "ISQ::MassValue should resolve via public import re-export from ISQBase"
    );
}

/// Test that simulates what the LSP hover does when cursor is on "MassValue" in "ISQ::MassValue"
#[test]
fn test_stdlib_hover_simulation() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    let resolver = crate::semantic::resolver::Resolver::new(workspace.symbol_table());

    // Simulate: user hovers on "MassValue" in "import ISQ::MassValue"
    // The extract_qualified_name_at_cursor should return "ISQ::MassValue"
    let line = "    private import ISQ::MassValue;";

    // Position 23 is on "MassValue"
    let extracted = crate::core::text_utils::extract_qualified_name_at_cursor(line, 23);
    println!("Extracted from line: {extracted:?}");
    assert_eq!(extracted, Some("ISQ::MassValue".to_string()));

    // Now resolve it - this is what hover does
    let symbol = resolver.resolve_qualified("ISQ::MassValue");
    println!(
        "Resolved ISQ::MassValue: {:?}",
        symbol.map(|s| s.qualified_name())
    );
    assert!(
        symbol.is_some(),
        "Hover on ISQ::MassValue should resolve the symbol"
    );

    // Also test resolver.resolve() which is what hover actually calls
    let symbol2 = resolver.resolve("ISQ::MassValue");
    println!(
        "resolver.resolve(ISQ::MassValue): {:?}",
        symbol2.map(|s| s.qualified_name())
    );
    assert!(
        symbol2.is_some(),
        "resolver.resolve(ISQ::MassValue) should work"
    );
}

/// Test that simulates what the LSP does when a user creates a new file
/// and imports ISQ::MassValue, then hovers on it
#[test]
fn test_stdlib_hover_with_user_file() {
    use crate::parser::{SysMLParser, sysml::Rule};
    use crate::syntax::sysml::ast::parse_file;

    use pest::Parser;

    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    // Now simulate adding a user file (like the LSP does when you open a new file)
    let user_source = r#"package MyTest {
    private import ISQ::MassValue;
    
    part def MyPart {
        attribute mass: MassValue;
    }
}"#;

    // Parse and add user file
    let mut pairs = SysMLParser::parse(Rule::model, user_source).expect("parse user source");
    let sysml_file = parse_file(&mut pairs).expect("from_pest");
    let user_path = PathBuf::from("/test/mytest.sysml");
    workspace.add_file(user_path.clone(), SyntaxFile::SysML(sysml_file));
    workspace
        .populate_file(&user_path)
        .expect("populate user file");

    // Now test hover on "ISQ::MassValue" in line 2
    let line = "    private import ISQ::MassValue;";
    let extracted = crate::core::text_utils::extract_qualified_name_at_cursor(line, 23);
    println!("Extracted from user file: {extracted:?}");
    assert_eq!(extracted, Some("ISQ::MassValue".to_string()));

    // Resolve using the resolver (this is what hover does)
    let resolver = crate::semantic::resolver::Resolver::new(workspace.symbol_table());
    let symbol = resolver.resolve_qualified("ISQ::MassValue");
    println!(
        "After adding user file - ISQ::MassValue: {:?}",
        symbol.map(|s| s.qualified_name())
    );
    assert!(
        symbol.is_some(),
        "ISQ::MassValue should still resolve after adding user file"
    );

    // Also check that MassValue can be resolved in the user file's scope
    let user_scope = workspace
        .symbol_table()
        .get_scope_for_file(&user_path.to_string_lossy());
    println!("User file scope: {user_scope:?}");

    if let Some(scope_id) = user_scope {
        let mass_value = resolver.resolve_in_scope("MassValue", scope_id);
        println!(
            "MassValue in user scope: {:?}",
            mass_value.map(|s| s.qualified_name())
        );
        // This should work because we imported ISQ::MassValue
        assert!(
            mass_value.is_some(),
            "MassValue should be resolvable in user file scope after import"
        );
    }
}
