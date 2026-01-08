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

mod tests_architecture_helpers;

use std::path::Path;
use tests_architecture_helpers::*;

#[test]
fn test_core_layer_has_no_dependencies() {
    let violations = collect_layer_violations(Path::new("src/core"), &[], "core");
    assert!(
        violations.is_empty(),
        "\n❌ Core layer should not depend on any other crate modules (only std).\nViolations:\n{}\n",
        violations.join("\n")
    );
}

#[test]
fn test_parser_layer_only_depends_on_core() {
    let violations = collect_layer_violations(Path::new("src/parser"), &["core"], "parser");
    assert!(
        violations.is_empty(),
        "\n❌ Parser layer should only depend on core.\nViolations:\n{}\n",
        violations.join("\n")
    );
}

#[test]
fn test_semantic_layer_only_depends_on_core_and_parser() {
    let violations = collect_layer_violations_with_skip(
        Path::new("src/semantic"),
        &["core", "parser", "syntax"],
        "semantic",
        |path, _module| {
            // Allow tests to import anything
            path.file_name().is_some_and(|n| n == "tests.rs")
        },
    );
    assert!(
        violations.is_empty(),
        "\n❌ Semantic layer should only depend on core, parser, and syntax (the file type enum, not sysml/kerml ASTs).\nViolations:\n{}\n",
        violations.join("\n")
    );
}

#[test]
fn test_syntax_layer_has_minimal_dependencies() {
    let violations =
        collect_layer_violations(Path::new("src/syntax"), &["core", "parser"], "syntax");
    assert!(
        violations.is_empty(),
        "\n❌ Syntax layer should only depend on core and parser.\nViolations:\n{}\n",
        violations.join("\n")
    );
}

#[test]
fn test_project_layer_dependencies() {
    let violations = collect_layer_violations(
        Path::new("src/project"),
        &["core", "parser", "semantic", "syntax"],
        "project",
    );
    assert!(
        violations.is_empty(),
        "\n❌ Project layer should only depend on core, parser, semantic, and syntax.\nViolations:\n{}\n",
        violations.join("\n")
    );
}

#[test]
fn test_no_layer_depends_on_lsp() {
    let violations = check_no_reverse_dependency(Path::new("src"), "syster_lsp", "LSP");
    assert!(
        violations.is_empty(),
        "\n❌ No layer in syster-base should depend on LSP.\nViolations:\n{}\n",
        violations.join("\n")
    );
}

#[test]
fn test_no_layer_depends_on_cli() {
    let violations = check_no_reverse_dependency(Path::new("src"), "syster_cli", "CLI");
    assert!(
        violations.is_empty(),
        "\n❌ No layer in syster-base should depend on CLI.\nViolations:\n{}\n",
        violations.join("\n")
    );
}

/// Helper test to show current architecture state
#[test]
fn test_show_architecture_violations_summary() {
    let layers = vec![
        ("core", vec![], "src/core"),
        ("parser", vec!["core"], "src/parser"),
        ("semantic", vec!["core", "parser", "syntax"], "src/semantic"),
        ("syntax", vec!["core", "parser"], "src/syntax"),
        (
            "project",
            vec!["core", "parser", "semantic", "syntax"],
            "src/project",
        ),
    ];

    let mut total_violations = 0;

    for (layer_name, allowed, path) in layers {
        let violations = collect_layer_violations(Path::new(path), &allowed, layer_name);

        if violations.is_empty() {
        } else {
            total_violations += violations.len();
        }
    }

    assert_eq!(
        total_violations, 0,
        "Found {total_violations} architecture violations. Run with --nocapture to see details."
    );
}

// ============================================================================
// PHASE 6: Semantic Adapter Separation Tests
// ============================================================================

/// Checks that only files in `semantic/adapters/` and `semantic/processors/` import from syntax
#[test]
fn test_semantic_layer_only_adapters_import_syntax() {
    let syntax_patterns = [
        "use crate::syntax::sysml",
        "use crate::syntax::kerml",
        "from syntax::sysml",
        "from syntax::kerml",
    ];

    let violations = find_files_with_imports(Path::new("src/semantic"), &syntax_patterns, |path| {
        // Skip adapters, processors, and test files/directories
        path.components().any(|c| {
            matches!(
                c.as_os_str().to_str(),
                Some("adapters" | "processors" | "tests")
            )
        }) || path.file_name().is_some_and(|n| {
            n == "tests.rs"
                || n == "workspace_file_test.rs"
                || n.to_string_lossy().starts_with("tests_")
        })
    });

    assert!(
        violations.is_empty(),
        "\n❌ Architecture violation: {} file(s) in semantic/ import from syntax layer:\n{}\n\n\
        Only adapters/ and processors/ may import from syntax::sysml or syntax::kerml.\n",
        violations.len(),
        format_violation_list(
            &violations
                .iter()
                .map(|(file, line)| format!("{}:{}", file.display(), line))
                .collect::<Vec<_>>()
        )
    );
}

/// Verifies that all required constants are defined in core/constants.rs
#[test]
fn test_core_constants_defined() {
    let content = read_required_file(Path::new("src/core/constants.rs"));

    let required_constants = [
        "pub const REL_SATISFY",
        "pub const REL_PERFORM",
        "pub const REL_EXHIBIT",
        "pub const REL_INCLUDE",
        "pub const ROLE_REQUIREMENT",
        "pub const ROLE_ACTION",
        "pub const ROLE_STATE",
        "pub const ROLE_USE_CASE",
    ];

    let missing: Vec<_> = required_constants
        .iter()
        .filter(|constant| !content.contains(*constant))
        .collect();

    assert!(
        missing.is_empty(),
        "\n❌ Missing required constants in core/constants.rs:\n{}\n",
        format_violation_list(&missing)
    );
}
