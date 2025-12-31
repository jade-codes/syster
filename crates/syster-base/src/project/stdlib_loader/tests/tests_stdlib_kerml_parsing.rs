#![allow(clippy::unwrap_used)]

use crate::project::file_loader;
use rstest::rstest;
use std::path::PathBuf;

#[rstest]
#[case("Kernel Libraries/Kernel Data Type Library/Collections.kerml")]
#[case("Kernel Libraries/Kernel Data Type Library/ScalarValues.kerml")]
#[case("Kernel Libraries/Kernel Data Type Library/VectorValues.kerml")]
#[case("Kernel Libraries/Kernel Function Library/BaseFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/BooleanFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/CollectionFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/ComplexFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/ControlFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/DataFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/IntegerFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/NaturalFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/NumericalFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/OccurrenceFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/RationalFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/RealFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/ScalarFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/SequenceFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/StringFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/TrigFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/VectorFunctions.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Base.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Clocks.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/ControlPerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/FeatureReferencingPerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/KerML.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Links.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Metaobjects.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Objects.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Observation.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Occurrences.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Performances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/SpatialFrames.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/StatePerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Transfers.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/TransitionPerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Triggers.kerml")]
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
