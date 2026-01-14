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

use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
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
///
/// This test is ignored by default because it parses ~95 files and takes ~8 seconds.
/// Run with: `cargo test test_sysml_examples_parsing -- --ignored`
#[test]
fn test_sysml_examples_parsing() {
    let examples_dir = get_examples_dir();

    if !examples_dir.exists() {
        eprintln!("⏭️  Skipping: sysml-examples directory not found at {examples_dir:?}");
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
        eprintln!("⚠️  No .sysml files found in {examples_dir:?}");
        return;
    }

    let passed = Mutex::new(Vec::new());
    let failed: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());

    files.par_iter().for_each(|file_path| {
        let relative = file_path
            .strip_prefix(&examples_dir)
            .unwrap_or(file_path)
            .display()
            .to_string();

        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                failed
                    .lock()
                    .unwrap()
                    .entry(format!("IO Error: {e}"))
                    .or_default()
                    .push(relative);
                return;
            }
        };

        let parse_result = file_loader::parse_with_result(&content, file_path);

        if parse_result.content.is_some() && parse_result.errors.is_empty() {
            passed.lock().unwrap().push(relative);
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

            failed
                .lock()
                .unwrap()
                .entry(error_msg)
                .or_default()
                .push(relative);
        }
    });

    let passed = passed.into_inner().unwrap();
    let failed = failed.into_inner().unwrap();
    let total = files.len();
    let pass_count = passed.len();
    let fail_count = total - pass_count;
    let pass_rate = (pass_count as f64 / total as f64) * 100.0;

    eprintln!("\n╔════════════════════════════════════════════════════════════════╗");
    eprintln!("║           SysML v2 Examples Parsing Summary                    ║");
    eprintln!("╠════════════════════════════════════════════════════════════════╣");
    eprintln!("║ Total files: {total:>4}                                              ║");
    eprintln!(
        "║ Passed:      {pass_count:>4} ({pass_rate:>5.1}%)                                    ║"
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
                eprintln!("     - {f}");
            }
            if files.len() > 3 {
                eprintln!("     ... and {} more", files.len() - 3);
            }
        }
    }

    if !passed.is_empty() {
        eprintln!("\n✅ Passing files ({}):", passed.len());
        for f in &passed {
            eprintln!("   - {f}");
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
            regressions.push(format!("{relative_path}: {error}"));
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

macro_rules! example_test {
    ($name:ident, $path:expr) => {
        #[test]
        fn $name() {
            let examples_dir = get_examples_dir();
            let file_path = examples_dir.join($path);

            if !file_path.exists() {
                eprintln!("Skipping: file not found at {:?}", file_path);
                return;
            }

            let content = std::fs::read_to_string(&file_path)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", $path, e));

            let parse_result = file_loader::parse_with_result(&content, &file_path);

            assert!(
                parse_result.content.is_some() && parse_result.errors.is_empty(),
                "Failed to parse {}:\n{}",
                $path,
                parse_result
                    .errors
                    .iter()
                    .map(|e| format!("  Line {}: {}", e.position.line, e.message))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    };
}

// Analysis Examples
example_test!(
    example_analysis_annotation,
    "Analysis Examples/AnalysisAnnotation.sysml"
);
example_test!(
    example_turbojet_stage_analysis,
    "Analysis Examples/Turbojet Stage Analysis.sysml"
);
example_test!(
    example_vehicle_analysis_demo,
    "Analysis Examples/Vehicle Analysis Demo.sysml"
);

// Arrowhead Framework Example
example_test!(
    example_ahf_sequences,
    "Arrowhead Framework Example/AHFSequences.sysml"
);
example_test!(
    example_ahf_norway_topics,
    "Arrowhead Framework Example/AHFNorwayTopics.sysml"
);

// Association Examples
example_test!(
    example_product_selection_unowned_ends,
    "Association Examples/ProductSelection_UnownedEnds.sysml"
);

// Camera Example
example_test!(example_picture_taking, "Camera Example/PictureTaking.sysml");

// Cause and Effect Examples
example_test!(
    example_cause_and_effect,
    "Cause and Effect Examples/CauseAndEffectExample.sysml"
);

// Flashlight Example
example_test!(
    example_flashlight,
    "Flashlight Example/Flashlight Example.sysml"
);

// Geometry Examples
example_test!(
    example_external_shape_ref,
    "Geometry Examples/ExternalShapeRefExample.sysml"
);
example_test!(
    example_vehicle_geometry_coords,
    "Geometry Examples/VehicleGeometryAndCoordinateFrames.sysml"
);

// Interaction Sequencing Examples
example_test!(
    example_server_sequence_outside_realization_3,
    "Interaction Sequencing Examples/ServerSequenceOutsideRealization-3.sysml"
);
example_test!(
    example_server_sequence_realization_3,
    "Interaction Sequencing Examples/ServerSequenceRealization-3.sysml"
);

// Mass Roll-up Example
example_test!(example_mass_rollup, "Mass Roll-up Example/MassRollup.sysml");

// Metadata Examples
example_test!(
    example_issue_metadata,
    "Metadata Examples/IssueMetadataExample.sysml"
);
example_test!(
    example_requirement_metadata,
    "Metadata Examples/RequirementMetadataExample.sysml"
);
example_test!(
    example_risk_metadata,
    "Metadata Examples/RiskMetadataExample.sysml"
);
example_test!(
    example_verification_metadata,
    "Metadata Examples/VerificationMetadataExample.sysml"
);

// Requirements Examples
example_test!(
    example_requirement_derivation,
    "Requirements Examples/RequirementDerivationExample.sysml"
);

// Room Model
example_test!(example_room_model, "Room Model/RoomModel.sysml");

// Simple Tests
example_test!(example_action_test, "Simple Tests/ActionTest.sysml");
example_test!(example_allocation_test, "Simple Tests/AllocationTest.sysml");
example_test!(example_analysis_test, "Simple Tests/AnalysisTest.sysml");
example_test!(example_assignment_test, "Simple Tests/AssignmentTest.sysml");
example_test!(example_comment_test, "Simple Tests/CommentTest.sysml");
example_test!(example_connection_test, "Simple Tests/ConnectionTest.sysml");
example_test!(
    example_control_node_test,
    "Simple Tests/ControlNodeTest.sysml"
);
example_test!(example_decision_test, "Simple Tests/DecisionTest.sysml");
example_test!(
    example_feature_path_test,
    "Simple Tests/FeaturePathTest.sysml"
);
example_test!(example_part_test, "Simple Tests/PartTest.sysml");
example_test!(
    example_requirement_test,
    "Simple Tests/RequirementTest.sysml"
);
example_test!(example_state_test, "Simple Tests/StateTest.sysml");
example_test!(
    example_textual_representation_test,
    "Simple Tests/TextualRepresentationTest.sysml"
);
example_test!(example_use_case_test, "Simple Tests/UseCaseTest.sysml");
example_test!(
    example_variability_test,
    "Simple Tests/VariabilityTest.sysml"
);
example_test!(
    example_verification_test,
    "Simple Tests/VerificationTest.sysml"
);
example_test!(example_view_test, "Simple Tests/ViewTest.sysml");

// State Space Representation Examples
example_test!(
    example_cart_sample,
    "State Space Representation Examples/CartSample.sysml"
);

// Timeslice and Snapshot Examples
example_test!(
    example_time_varying_attribute,
    "Timeslice and Snapshot Examples/TimeVaryingAttribute.sysml"
);

// Variability Examples
example_test!(
    example_vehicle_variability_model,
    "Variability Examples/VehicleVariabilityModel.sysml"
);

// Vehicle Example
example_test!(
    example_vehicle_individuals,
    "Vehicle Example/VehicleIndividuals.sysml"
);
example_test!(
    example_sysml_spec_annex_a,
    "Vehicle Example/SysML v2 Spec Annex A SimpleVehicleModel.sysml"
);
