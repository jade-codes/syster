//! Tests for parsing official KerML examples from the SysML-v2-Release repository.
//!
//! These examples are downloaded from:
//! https://github.com/Systems-Modeling/SysML-v2-Release/tree/master/kerml/src/examples
//!
//! ## Current Status
//! - **22/54 files parse successfully (41%)**
//! - Missing grammar features cause failures in the remaining files
//!
//! ## Missing Grammar Features (for future work)
//! 1. `meta` keyword - `baseType = Atom meta KerML::Classifier;`
//! 2. `#atom` notation - hash identifiers for sequence access
//! 3. `@Annotation` - metadata annotations
//! 4. `flow ... to` - flow connections with `to` keyword
//! 5. Multiple types with comma - `typed by A, B`
//! 6. `subclassifier` keyword
//! 7. `locale` keyword for internationalization
//! 8. `ordered` / `nonunique` multiplicity modifiers
//! 9. `$::` root scope reference
//! 10. `succession flow` with `of` and `from`
//! 11. Inline expressions with `.{}`

#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use pest::Parser;
use rayon::prelude::*;
use rstest::rstest;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use syster::parser::KerMLParser;
use syster::parser::kerml::Rule;

/// Get all .kerml files from the examples directory
fn get_kerml_example_files() -> Vec<String> {
    let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/kerml-examples");
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(&examples_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "kerml").unwrap_or(false) {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }

    files.sort();
    files
}

/// Test that all KerML example files can be parsed without errors.
/// This is a smoke test to ensure our grammar is compatible with the official examples.
#[test]
fn test_parse_all_kerml_examples() {
    let files = get_kerml_example_files();
    assert!(!files.is_empty(), "No KerML example files found!");

    let failures: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

    files.par_iter().for_each(|file_path| {
        let content = fs::read_to_string(file_path).unwrap();
        let filename = Path::new(file_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        match KerMLParser::parse(Rule::file, &content) {
            Ok(_) => {
                println!("✓ {}", filename);
            }
            Err(e) => {
                println!("✗ {}", filename);
                failures.lock().unwrap().push((filename, format!("{}", e)));
            }
        }
    });

    let failures = failures.into_inner().unwrap();

    if !failures.is_empty() {
        let failure_report: String = failures
            .iter()
            .map(|(name, err)| format!("\n--- {} ---\n{}", name, err))
            .collect();

        panic!(
            "Failed to parse {} of {} KerML example files:\n{}",
            failures.len(),
            files.len(),
            failure_report
        );
    }

    println!(
        "\n✓ Successfully parsed all {} KerML example files",
        files.len()
    );
}

/// Individual test cases for each KerML example file.
/// This allows seeing which specific files fail in test output.
#[rstest]
#[case("A-2-Atoms.kerml")]
#[case("A-2-ModelingInstances.kerml")]
#[case("A-3-2-WithoutConnectors.kerml")]
#[case("A-3-3-OneToOneConnectors.kerml")]
#[case("A-3-4-OneToUnrestrictedConnectors.kerml")]
#[case("A-3-5-TimingForStructures.kerml")]
#[case("A-3-6-Sequences.kerml")]
#[case("A-3-7-DecisionsAndMerges.kerml")]
#[case("A-3-8-ChangingFeatureValues.kerml")]
#[case("AddressBookModel.kerml")]
#[case("ArgumentResolution.kerml")]
#[case("Associations.kerml")]
#[case("Behaviors.kerml")]
#[case("Camera.kerml")]
#[case("Circular.kerml")]
#[case("Classes.kerml")]
#[case("Classifications.kerml")]
#[case("Classifiers.kerml")]
#[case("Comments.kerml")]
#[case("Conjugation.kerml")]
#[case("Connectors.kerml")]
#[case("Dependencies.kerml")]
#[case("Expansion.kerml")]
#[case("Expressions.kerml")]
#[case("FeatureChains.kerml")]
#[case("FeatureInheritance.kerml")]
#[case("Features.kerml")]
#[case("Filtering.kerml")]
#[case("Imports.kerml")]
#[case("Inheritance.kerml")]
#[case("Inverses.kerml")]
#[case("JohnIndividualExample.kerml")]
#[case("MassedThings.kerml")]
#[case("MassRollup_1.kerml")]
#[case("MassRollup_2.kerml")]
#[case("MetadataTest.kerml")]
#[case("PacketUsage.kerml")]
#[case("Packets.kerml")]
#[case("ProductSelection_N_ary.kerml")]
#[case("ProductSelection_OwnedEnds.kerml")]
#[case("ProductSelection_UnownedEnds.kerml")]
#[case("Redefinition.kerml")]
#[case("Scoping.kerml")]
#[case("TakePicture.kerml")]
#[case("TextualRepresentation.kerml")]
#[case("TimeVaryingCarDriver.kerml")]
#[case("TimeVaryingFeatures.kerml")]
#[case("Types.kerml")]
#[case("VehicleDefinitions.kerml")]
#[case("VehicleTanks.kerml")]
#[case("VehicleUsages.kerml")]
#[case("Vehicles_1.kerml")]
#[case("Vehicles_2.kerml")]
#[case("Vehicles_3.kerml")]
fn test_parse_kerml_example(#[case] filename: &str) {
    let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/kerml-examples");
    let file_path = examples_dir.join(filename);

    assert!(
        file_path.exists(),
        "Example file not found: {}",
        file_path.display()
    );

    let content = fs::read_to_string(&file_path).unwrap();

    let result = KerMLParser::parse(Rule::file, &content);
    assert!(
        result.is_ok(),
        "Failed to parse {}: {}",
        filename,
        result.err().unwrap()
    );
}
