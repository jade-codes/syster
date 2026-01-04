//! Tests for SysML v2 Release examples
//!
//! This module tests parsing of official SysML v2 Release examples from:
//! https://github.com/Systems-Modeling/SysML-v2-Release
//!
//! The examples are stored in `tests/sysml-examples/` directory.
//!
//! # Setup
//! To populate the examples directory:
//! ```bash
//! git clone --depth 1 https://github.com/Systems-Modeling/SysML-v2-Release.git /tmp/sysml
//! cp -r /tmp/sysml/sysml/src/examples crates/syster-base/tests/sysml-examples
//! ```

#![allow(clippy::unwrap_used)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use syster::project::file_loader;

fn get_examples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/sysml-examples")
}

/// Recursively collect all .sysml files from a directory
fn collect_sysml_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_sysml_files(&path, files);
            } else if path.extension().is_some_and(|ext| ext == "sysml") {
                files.push(path);
            }
        }
    }
}

/// Test all SysML v2 Release examples and report results
#[test]
fn test_sysml_examples_parsing() {
    let examples_dir = get_examples_dir();

    if !examples_dir.exists() {
        eprintln!(
            "⏭️  Skipping: sysml-examples directory not found at {:?}",
            examples_dir
        );
        eprintln!("   To run these tests, execute:");
        eprintln!(
            "   git clone --depth 1 https://github.com/Systems-Modeling/SysML-v2-Release.git /tmp/sysml"
        );
        eprintln!("   cp -r /tmp/sysml/sysml/src/examples crates/syster-base/tests/sysml-examples");
        return;
    }

    let mut files = Vec::new();
    collect_sysml_files(&examples_dir, &mut files);
    files.sort();

    if files.is_empty() {
        eprintln!("⚠️  No .sysml files found in {:?}", examples_dir);
        return;
    }

    let mut passed = Vec::new();
    let mut failed: HashMap<String, Vec<String>> = HashMap::new();

    for file_path in &files {
        let relative = file_path
            .strip_prefix(&examples_dir)
            .unwrap_or(file_path)
            .display()
            .to_string();

        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                failed
                    .entry(format!("IO Error: {}", e))
                    .or_default()
                    .push(relative);
                continue;
            }
        };

        let parse_result = file_loader::parse_with_result(&content, file_path);

        if parse_result.content.is_some() && parse_result.errors.is_empty() {
            passed.push(relative);
        } else {
            let error_msg = parse_result
                .errors
                .first()
                .map(|e| {
                    // Extract just the "expected X" part for grouping
                    if let Some(pos) = e.message.find("expected ") {
                        let rest = &e.message[pos..];
                        if let Some(end) = rest.find('\n') {
                            rest[..end].to_string()
                        } else {
                            rest.to_string()
                        }
                    } else {
                        e.message.clone()
                    }
                })
                .unwrap_or_else(|| "Unknown error".to_string());

            failed.entry(error_msg).or_default().push(relative);
        }
    }

    let total = files.len();
    let pass_count = passed.len();
    let fail_count = total - pass_count;
    let pass_rate = (pass_count as f64 / total as f64) * 100.0;

    eprintln!("\n╔════════════════════════════════════════════════════════════════╗");
    eprintln!("║           SysML v2 Examples Parsing Summary                    ║");
    eprintln!("╠════════════════════════════════════════════════════════════════╣");
    eprintln!(
        "║ Total files: {:>4}                                              ║",
        total
    );
    eprintln!(
        "║ Passed:      {:>4} ({:>5.1}%)                                    ║",
        pass_count, pass_rate
    );
    eprintln!(
        "║ Failed:      {:>4} ({:>5.1}%)                                    ║",
        fail_count,
        100.0 - pass_rate
    );
    eprintln!("╚════════════════════════════════════════════════════════════════╝");

    if !failed.is_empty() {
        eprintln!("\n📋 Failures by error pattern:");

        // Sort by count descending
        let mut error_counts: Vec<_> = failed.iter().collect();
        error_counts.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (error, files) in error_counts {
            eprintln!("\n  ❌ {} ({} files)", error, files.len());
            for f in files.iter().take(3) {
                eprintln!("     - {}", f);
            }
            if files.len() > 3 {
                eprintln!("     ... and {} more", files.len() - 3);
            }
        }
    }

    if !passed.is_empty() {
        eprintln!("\n✅ Passing files ({}):", passed.len());
        for f in &passed {
            eprintln!("   - {}", f);
        }
    }

    eprintln!();

    // The test itself always passes - it's informational
    // Uncomment the assertion below to make it fail if any files don't parse:
    // assert_eq!(fail_count, 0, "Some example files failed to parse");
}

/// Regression test: ensure no previously-passing files start failing
///
/// This list should be kept in sync with the actual passing files.
/// Run test_sysml_examples_parsing to see the current list.
#[test]
fn test_no_regressions() {
    let examples_dir = get_examples_dir();

    if !examples_dir.exists() {
        return; // Skip if examples not present
    }

    // List of files that MUST continue to parse successfully
    // This prevents accidental grammar regressions
    let must_pass = [
        "Simple Tests/ImportTest.sysml",
        "Simple Tests/AliasTest.sysml",
        "Simple Tests/EnumerationTest.sysml",
        "Simple Tests/MultiplicityTest.sysml",
        "Simple Tests/DependencyTest.sysml",
        "Simple Tests/DefaultValueTest.sysml",
        "Simple Tests/ConstraintTest.sysml",
        "Import Tests/AliasImport.sysml",
        "Import Tests/CircularImport.sysml",
        "Import Tests/PrivateImportTest.sysml",
        "Import Tests/QualifiedNameImportTest.sysml",
        "Comment Examples/Comments.sysml",
    ];

    let mut regressions = Vec::new();

    for relative_path in must_pass {
        let file_path = examples_dir.join(relative_path);

        if !file_path.exists() {
            continue; // Skip if file doesn't exist
        }

        let content = match std::fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let parse_result = file_loader::parse_with_result(&content, &file_path);

        if parse_result.content.is_none() || !parse_result.errors.is_empty() {
            let error = parse_result
                .errors
                .first()
                .map(|e| e.message.clone())
                .unwrap_or_else(|| "Unknown error".to_string());
            regressions.push(format!("{}: {}", relative_path, error));
        }
    }

    if !regressions.is_empty() {
        panic!(
            "🚨 REGRESSION: {} previously-passing files now fail:\n  - {}",
            regressions.len(),
            regressions.join("\n  - ")
        );
    }
}
