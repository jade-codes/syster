use crate::project::file_loader;
use crate::semantic::Workspace;
use crate::syntax::SyntaxFile;
use rayon::prelude::*;
use std::path::PathBuf;

/// Loads the SysML standard library into the workspace.
///
/// # Errors
///
/// Returns an error if:
/// - The stdlib directory cannot be read
/// - File collection fails
///
/// Note: Individual file parse failures are logged but do not cause the load to fail.
pub fn load(stdlib_path: &PathBuf, workspace: &mut Workspace<SyntaxFile>) -> Result<(), String> {
    if !stdlib_path.exists() || !stdlib_path.is_dir() {
        return Ok(());
    }

    // Collect all file paths first
    let file_paths = file_loader::collect_file_paths(stdlib_path)?;

    // Parse files in parallel
    let results: Vec<_> = file_paths
        .par_iter()
        .map(|path| (path, parse_file(path)))
        .collect();

    // Add successfully parsed files and track failures
    let mut failed_files = Vec::new();
    for (path, result) in results {
        match result {
            Ok((path, file)) => {
                workspace.add_file(path, file);
            }
            Err(e) => {
                failed_files.push((path.clone(), e));
            }
        }
    }

    workspace.mark_stdlib_loaded();

    Ok(())
}

fn parse_file(path: &PathBuf) -> Result<(PathBuf, SyntaxFile), String> {
    file_loader::load_and_parse(path).map(|file| (path.clone(), file))
}
