use std::path::PathBuf;
use syster::project::StdLibLoader;
use syster::semantic::Workspace;
use syster::syntax::SyntaxFile;

#[test]
#[ignore = "Calculation depends on Items.sysml which has complex arrow expressions not yet supported"]
fn test_stdlib_calculation_symbol_loads() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    // Check if Calculation symbol exists
    let all_symbols = workspace.symbol_table().all_symbols();
    let calculation = all_symbols
        .iter()
        .find(|(_, sym)| sym.name() == "Calculation");

    assert!(
        calculation.is_some(),
        "Calculation symbol should be in symbol table. Found {} symbols total",
        workspace.symbol_table().all_symbols().len()
    );

    let (key, sym) = calculation.unwrap();
    println!("Found Calculation with key: {key}");
    println!("Qualified name: {}", sym.qualified_name());
}

#[test]
fn test_stdlib_case_symbol_loads() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    // Check if Case symbol exists
    let all_symbols = workspace.symbol_table().all_symbols();
    let case = all_symbols.iter().find(|(_, sym)| sym.name() == "Case");

    assert!(
        case.is_some(),
        "Case symbol should be in symbol table. Found {} symbols total",
        workspace.symbol_table().all_symbols().len()
    );

    let (key, sym) = case.unwrap();
    println!("Found Case with key: {key}");
    println!("Qualified name: {}", sym.qualified_name());
}

#[test]
fn test_stdlib_symbol_count() {
    let stdlib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    let mut workspace = Workspace::<SyntaxFile>::new();
    let loader = StdLibLoader::with_path(stdlib_path);
    loader.load(&mut workspace).expect("Failed to load stdlib");
    workspace.populate_all().expect("Failed to populate stdlib");

    let symbol_count = workspace.symbol_table().all_symbols().len();
    println!("Total symbols loaded: {symbol_count}");

    // We should have significantly more symbols now with Items, Cases, etc
    assert!(
        symbol_count >= 1451,
        "Expected at least 1451 symbols, found {symbol_count}"
    );

    // Print first 20 symbols for debugging
    println!("\nFirst 20 symbols:");
    for (i, (key, sym)) in workspace.symbol_table().all_symbols().iter().enumerate() {
        if i >= 20 {
            break;
        }
        println!("  {}: {} (qualified: {})", i + 1, sym.name(), key);
    }
}
