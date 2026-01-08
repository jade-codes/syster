use std::path::PathBuf;
use syster::project::WorkspaceLoader;
use syster::semantic::Workspace;

#[derive(Debug)]
pub struct AnalysisResult {
    pub file_count: usize,
    pub symbol_count: usize,
}

pub fn run_analysis(
    input: &PathBuf,
    verbose: bool,
    load_stdlib: bool,
    stdlib_path: Option<&PathBuf>,
) -> Result<AnalysisResult, String> {
    if verbose {
        println!("Analyzing: {}", input.display());
    }

    // Create workspace
    let mut workspace = Workspace::new();
    let loader = WorkspaceLoader::new();

    // Load standard library
    if load_stdlib {
        if verbose {
            println!("Loading standard library...");
        }
        let stdlib_loader = if let Some(path) = stdlib_path {
            syster::project::StdLibLoader::with_path(path.clone())
        } else {
            syster::project::StdLibLoader::new()
        };
        stdlib_loader.load(&mut workspace)?;
    }

    // Load input file(s)
    if input.is_file() {
        loader.load_file(input, &mut workspace)?;
    } else if input.is_dir() {
        loader.load_directory(input, &mut workspace)?;
    } else {
        return Err(format!("Input path does not exist: {}", input.display()));
    }

    if verbose {
        println!("Populating symbol tables...");
    }

    // Populate workspace
    workspace.populate_all()?;

    let symbol_count = workspace.symbol_table().iter_symbols().count();
    let file_count = workspace.file_count();

    Ok(AnalysisResult {
        file_count,
        symbol_count,
    })
}
