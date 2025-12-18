#![allow(clippy::unwrap_used)]

use rstest::rstest;
use std::path::PathBuf;
use syster::project::file_loader;

/// Test that each KerML stdlib file can be parsed successfully
///
/// This test suite creates individual test cases for each KerML file in the standard library.
/// When a file fails to parse, the test name clearly indicates which file has the issue.
///
/// NOTE: Many KerML stdlib files currently fail to parse due to incomplete grammar support.
/// These tests are included to track progress on KerML parser implementation.

#[rstest]
// Kernel Data Type Library - Partially supported
#[case("Kernel Libraries/Kernel Data Type Library/Collections.kerml")]
#[case("Kernel Libraries/Kernel Data Type Library/ScalarValues.kerml")]
#[ignore = "Abstract datatype syntax not fully supported"]
#[case("Kernel Libraries/Kernel Data Type Library/VectorValues.kerml")]
#[ignore = "Vector datatype syntax not fully supported"]
// Kernel Function Library - Most tests ignored due to parser limitations
#[case("Kernel Libraries/Kernel Function Library/BaseFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/BooleanFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/CollectionFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/ComplexFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/ControlFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/DataFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/IntegerFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/NaturalFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/NumericalFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/OccurrenceFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/RationalFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/RealFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/ScalarFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/SequenceFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/StringFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/TrigFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
#[case("Kernel Libraries/Kernel Function Library/VectorFunctions.kerml")]
#[ignore = "KerML function syntax not fully supported"]
// Kernel Semantic Library - Partially supported
#[case("Kernel Libraries/Kernel Semantic Library/Base.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Clocks.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/ControlPerformances.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/FeatureReferencingPerformances.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/KerML.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Links.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/Metaobjects.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/Objects.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/Observation.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/Occurrences.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/Performances.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/SpatialFrames.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/StatePerformances.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/Transfers.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/TransitionPerformances.kerml")]
#[ignore = "KerML advanced features not fully supported"]
#[case("Kernel Libraries/Kernel Semantic Library/Triggers.kerml")]
#[ignore = "KerML advanced features not fully supported"]
fn test_parse_stdlib_kerml_file(#[case] relative_path: &str) {
    let mut path = PathBuf::from("sysml.library");
    path.push(relative_path);

    let result = file_loader::load_and_parse(&path);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {}",
        relative_path,
        result.err().unwrap()
    );
}
