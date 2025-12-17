//! Architecture Layer Dependency Tests
//!
//! These tests enforce the layered architecture dependency rules:
//!
//! ```
//! CLI/LSP (Delivery)
//!       ↓
//! Project/Workspace
//!       ↓
//! Semantic
//!       ↓
//! Parser
//!       ↓
//! Core
//! ```
//!
//! Dependency Rules:
//! - core → no imports (only std)
//! - parser → only core
//! - semantic → core, parser
//! - project → core, parser, semantic, syntax
//! - syntax → core, parser (AST definitions only)
//! - CLI/LSP → everything
//! - No layer depends on CLI/LSP

use std::fs;
use std::path::Path;

/// Check if a file contains any forbidden import patterns
fn check_file_imports(path: &Path, allowed_modules: &[&str], layer_name: &str) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap_or_default();
    let mut violations = Vec::new();
    let mut in_use_block = false;
    let mut use_statement = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Track multi-line use statements
        if trimmed.starts_with("use ") {
            use_statement = trimmed.to_string();
            in_use_block = !trimmed.ends_with(';');

            if !in_use_block {
                check_use_statement(
                    &use_statement,
                    path,
                    allowed_modules,
                    layer_name,
                    &mut violations,
                );
                use_statement.clear();
            }
        } else if in_use_block {
            use_statement.push(' ');
            use_statement.push_str(trimmed);

            if trimmed.ends_with(';') || trimmed.ends_with("};") {
                in_use_block = false;
                check_use_statement(
                    &use_statement,
                    path,
                    allowed_modules,
                    layer_name,
                    &mut violations,
                );
                use_statement.clear();
            }
        }
    }

    violations
}

fn check_use_statement(
    use_stmt: &str,
    path: &Path,
    allowed_modules: &[&str],
    layer_name: &str,
    violations: &mut Vec<String>,
) {
    // Skip std imports
    if use_stmt.contains("use std::") || use_stmt.contains("use core::") {
        return;
    }

    // Extract the crate-relative import (e.g., "crate::semantic")
    if let Some(import) = use_stmt.strip_prefix("use crate::") {
        let module = import.split("::").next().unwrap_or("").trim();

        // Check if this module is in the allowed list
        if !allowed_modules.contains(&module) && !module.is_empty() {
            violations.push(format!(
                "  {} uses forbidden module 'crate::{}' (layer: {})",
                path.display(),
                module,
                layer_name
            ));
        }
    }
}
/// Recursively check all .rs files in a directory
fn check_directory(dir: &Path, allowed_modules: &[&str], layer_name: &str) -> Vec<String> {
    let mut violations = Vec::new();

    if !dir.exists() {
        return violations;
    }

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            violations.extend(check_directory(&path, allowed_modules, layer_name));
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            violations.extend(check_file_imports(&path, allowed_modules, layer_name));
        }
    }

    violations
}

#[test]
fn test_core_layer_has_no_dependencies() {
    // Core layer should only import from std, not from any other crate modules
    let violations = check_directory(
        Path::new("src/core"),
        &[], // No crate modules allowed
        "core",
    );
    if !violations.is_empty() {
        eprintln!("\n❌ Core layer dependency violations found:");
        for v in &violations {
            eprintln!("{}", v);
        }
        panic!(
            "\nCore layer should not depend on any other crate modules (only std).\nFound {} violations.",
            violations.len()
        );
    }
}

#[test]
fn test_parser_layer_only_depends_on_core() {
    // Parser layer can only import from core
    let violations = check_directory(Path::new("src/parser"), &["core"], "parser");

    if !violations.is_empty() {
        eprintln!("\n❌ Parser layer dependency violations found:");
        for v in &violations {
            eprintln!("{}", v);
        }
        panic!(
            "\nParser layer should only depend on core.\nFound {} violations.",
            violations.len()
        );
    }
}

#[test]
#[ignore = "Semantic layer has 57 violations - processors importing from language, semantic submodules need refactoring"]
fn test_semantic_layer_only_depends_on_core_and_parser() {
    // Semantic layer can import from core and parser
    let violations = check_directory(Path::new("src/semantic"), &["core", "parser"], "semantic");

    if !violations.is_empty() {
        eprintln!("\n❌ Semantic layer dependency violations found:");
        for v in &violations {
            eprintln!("{}", v);
        }
        panic!(
            "\nSemantic layer should only depend on core and parser.\nFound {} violations.",
            violations.len()
        );
    }
}

#[test]
#[ignore = "Syntax layer has 39 violations - needs refactoring to separate concerns"]
fn test_syntax_layer_has_minimal_dependencies() {
    // Syntax layer should only import from core and parser (no semantic, project, etc.)
    let violations = check_directory(Path::new("src/syntax"), &["core", "parser"], "syntax");

    if !violations.is_empty() {
        eprintln!("\n❌ Syntax layer dependency violations found:");
        for v in &violations {
            eprintln!("{}", v);
        }
        panic!(
            "\nSyntax layer should only depend on core and parser.\nFound {} violations.",
            violations.len()
        );
    }
}

#[test]
#[ignore = "Project layer has 6 violations - needs cleanup"]
fn test_project_layer_dependencies() {
    // Project layer can import from core, parser, semantic, and syntax
    let violations = check_directory(
        Path::new("src/project"),
        &["core", "parser", "semantic", "syntax"],
        "project",
    );

    if !violations.is_empty() {
        eprintln!("\n❌ Project layer dependency violations found:");
        for v in &violations {
            eprintln!("{}", v);
        }
        panic!(
            "\nProject layer should only depend on core, parser, semantic, and syntax.\nFound {} violations.",
            violations.len()
        );
    }
}

#[test]
fn test_no_layer_depends_on_lsp() {
    // No base library layer should import from lsp crate
    let base_src = Path::new("src");
    let mut violations = Vec::new();

    fn check_recursively(dir: &Path, violations: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    check_recursively(&path, violations);
                } else if path.extension().is_some_and(|ext| ext == "rs") {
                    let content = fs::read_to_string(&path).unwrap_or_default();

                    for line in content.lines() {
                        if line.contains("use syster_lsp") || line.contains("use crate::lsp") {
                            violations.push(format!(
                                "  {} imports from LSP layer (forbidden)",
                                path.display()
                            ));
                        }
                    }
                }
            }
        }
    }

    check_recursively(base_src, &mut violations);

    if !violations.is_empty() {
        eprintln!("\n❌ Reverse dependency violations found (base library importing from LSP):");
        for v in &violations {
            eprintln!("{}", v);
        }
        panic!(
            "\nNo layer in syster-base should depend on LSP.\nFound {} violations.",
            violations.len()
        );
    }
}

#[test]
fn test_no_layer_depends_on_cli() {
    // No base library layer should import from cli crate
    let base_src = Path::new("src");
    let mut violations = Vec::new();

    fn check_recursively(dir: &Path, violations: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    check_recursively(&path, violations);
                } else if path.extension().is_some_and(|ext| ext == "rs") {
                    let content = fs::read_to_string(&path).unwrap_or_default();

                    for line in content.lines() {
                        if line.contains("use syster_cli") || line.contains("use crate::cli") {
                            violations.push(format!(
                                "  {} imports from CLI layer (forbidden)",
                                path.display()
                            ));
                        }
                    }
                }
            }
        }
    }

    check_recursively(base_src, &mut violations);

    if !violations.is_empty() {
        eprintln!("\n❌ Reverse dependency violations found (base library importing from CLI):");
        for v in &violations {
            eprintln!("{}", v);
        }
        panic!(
            "\nNo layer in syster-base should depend on CLI.\nFound {} violations.",
            violations.len()
        );
    }
}

/// Helper test to show current architecture state
#[test]
fn test_show_architecture_violations_summary() {
    println!("\n📊 Architecture Layer Dependency Analysis\n");
    println!("==========================================\n");

    let layers = vec![
        ("core", vec![], "src/core"),
        ("parser", vec!["core"], "src/parser"),
        ("semantic", vec!["core", "parser"], "src/semantic"),
        ("syntax", vec!["core", "parser"], "src/syntax"),
        (
            "project",
            vec!["core", "parser", "semantic", "syntax"],
            "src/project",
        ),
    ];

    let mut total_violations = 0;

    for (layer_name, allowed, path) in layers {
        let violations = check_directory(Path::new(path), &allowed, layer_name);

        if violations.is_empty() {
            println!("✅ {}: No violations", layer_name);
        } else {
            println!("❌ {}: {} violation(s)", layer_name, violations.len());
            total_violations += violations.len();
        }
    }

    println!("\n==========================================");
    println!("Total violations: {}", total_violations);

    if total_violations > 0 {
        println!("\nRun individual tests with --ignored to see details:");
        println!("  cargo test --test architecture_tests -- --ignored --nocapture");
    }
}
