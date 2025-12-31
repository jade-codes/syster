//! Helper functions for architecture tests
//!
//! This module contains shared utilities for checking layer dependencies
//! and architectural constraints.

use std::fs;
use std::path::Path;

// ============================================================================
// File System Utilities
// ============================================================================

/// Recursively visits all .rs files in a directory
pub fn visit_rust_files<F>(dir: &Path, callback: &mut F)
where
    F: FnMut(&Path),
{
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit_rust_files(&path, callback);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            callback(&path);
        }
    }
}

/// Reads a file and returns its content, panicking with a helpful message if it fails
pub fn read_required_file(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Required file not found: {}", path.display()))
}

// ============================================================================
// Import Checking Utilities
// ============================================================================

/// Finds files that contain any of the specified import patterns
pub fn find_files_with_imports(
    dir: &Path,
    patterns: &[&str],
    skip_predicate: impl Fn(&Path) -> bool + Copy,
) -> Vec<(std::path::PathBuf, usize)> {
    if !dir.exists() {
        return Vec::new();
    }

    let mut violations = Vec::new();
    visit_rust_files(dir, &mut |path| {
        if skip_predicate(path) {
            return;
        }

        if let Ok(content) = fs::read_to_string(path) {
            violations.extend(
                content
                    .lines()
                    .enumerate()
                    .filter(|(_, line)| patterns.iter().any(|p| line.contains(p)))
                    .map(|(idx, _)| (path.to_path_buf(), idx + 1)),
            );
        }
    });
    violations
}

// ============================================================================
// Layer Dependency Checking
// ============================================================================

/// Check if a file contains any forbidden import patterns
pub fn check_file_imports(
    path: &Path,
    allowed_modules: &[&str],
    layer_name: &str,
    skip_check: impl Fn(&Path, &str) -> bool,
) -> Vec<String> {
    let Ok(content) = fs::read_to_string(path) else {
        return Vec::new();
    };

    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with("use crate::") {
                return None;
            }

            // Handle both "use crate::module" and "use crate::{module::..."
            let after_crate = line.strip_prefix("use crate::")?;
            let after_crate = after_crate.trim_start_matches('{');
            let module = after_crate.split("::").next()?;

            // Skip empty module names (multi-line use statements)
            if module.is_empty() {
                return None;
            }

            // Allow modules to import from themselves (e.g., project can import from project)
            if module == layer_name || allowed_modules.contains(&module) || skip_check(path, module)
            {
                None
            } else {
                Some(format!(
                    "{} imports forbidden module '{}' in {}: {}",
                    layer_name,
                    module,
                    path.display(),
                    line
                ))
            }
        })
        .collect()
}

/// Recursively collects violations for all Rust files in a directory
pub fn collect_layer_violations(
    dir: &Path,
    allowed_modules: &[&str],
    layer_name: &str,
) -> Vec<String> {
    collect_layer_violations_with_skip(dir, allowed_modules, layer_name, |_, _| false)
}

/// Recursively collects violations with custom skip logic
pub fn collect_layer_violations_with_skip(
    dir: &Path,
    allowed_modules: &[&str],
    layer_name: &str,
    skip_check: impl Fn(&Path, &str) -> bool + Copy,
) -> Vec<String> {
    let mut all_violations = Vec::new();

    if !dir.exists() {
        return all_violations;
    }

    visit_rust_files(dir, &mut |path| {
        let violations = check_file_imports(path, allowed_modules, layer_name, skip_check);
        all_violations.extend(violations);
    });

    all_violations
}

// ============================================================================
// Formatting Utilities
// ============================================================================

/// Formats violations as a bulleted list
pub fn format_violation_list<T: std::fmt::Display>(items: &[T]) -> String {
    items
        .iter()
        .map(|item| format!("  - {item}"))
        .collect::<Vec<_>>()
        .join("\n")
}

// ============================================================================
// Reverse Dependency Checking
// ============================================================================

/// Checks if any files in a directory tree import from forbidden crates
pub fn check_no_reverse_dependency(
    dir: &Path,
    forbidden_crate: &str,
    layer_name: &str,
) -> Vec<String> {
    let patterns = [
        format!("use {forbidden_crate}"),
        format!("use crate::{forbidden_crate}"),
    ];
    let pattern_refs: Vec<&str> = patterns.iter().map(|s| s.as_str()).collect();

    find_files_with_imports(dir, &pattern_refs, |_| false)
        .into_iter()
        .map(|(path, _)| {
            format!(
                "  {} imports from {} layer (forbidden)",
                path.display(),
                layer_name
            )
        })
        .collect()
}
