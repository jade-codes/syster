#![allow(clippy::unwrap_used)]

use rstest::rstest;
use std::path::PathBuf;
use syster::project::file_loader;

#[rstest]
#[case("Domain Libraries/Analysis/AnalysisTooling.sysml")]
#[case("Domain Libraries/Analysis/SampledFunctions.sysml")]
#[case("Domain Libraries/Analysis/StateSpaceRepresentation.sysml")]
#[case("Domain Libraries/Analysis/TradeStudies.sysml")]
#[case("Domain Libraries/Cause and Effect/CausationConnections.sysml")]
#[case("Domain Libraries/Cause and Effect/CauseAndEffect.sysml")]
#[case("Domain Libraries/Geometry/ShapeItems.sysml")]
#[case("Domain Libraries/Geometry/SpatialItems.sysml")]
#[case("Domain Libraries/Metadata/ImageMetadata.sysml")]
#[case("Domain Libraries/Metadata/ModelingMetadata.sysml")]
#[case("Domain Libraries/Metadata/ParametersOfInterestMetadata.sysml")]
#[case("Domain Libraries/Metadata/RiskMetadata.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQAcoustics.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQAtomicNuclear.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQBase.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQCharacteristicNumbers.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQChemistryMolecular.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQCondensedMatter.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQElectromagnetism.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQInformation.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQLight.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQMechanics.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQSpaceTime.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQ.sysml")]
#[case("Domain Libraries/Quantities and Units/ISQThermodynamics.sysml")]
#[case("Domain Libraries/Quantities and Units/MeasurementRefCalculations.sysml")]
#[case("Domain Libraries/Quantities and Units/MeasurementReferences.sysml")]
#[case("Domain Libraries/Quantities and Units/Quantities.sysml")]
#[case("Domain Libraries/Quantities and Units/QuantityCalculations.sysml")]
#[case("Domain Libraries/Quantities and Units/SIPrefixes.sysml")]
#[case("Domain Libraries/Quantities and Units/SI.sysml")]
#[case("Domain Libraries/Quantities and Units/TensorCalculations.sysml")]
#[case("Domain Libraries/Quantities and Units/Time.sysml")]
#[case("Domain Libraries/Quantities and Units/USCustomaryUnits.sysml")]
#[case("Domain Libraries/Quantities and Units/VectorCalculations.sysml")]
#[case("Domain Libraries/Requirement Derivation/DerivationConnections.sysml")]
#[case("Domain Libraries/Requirement Derivation/RequirementDerivation.sysml")]
#[case("Systems Library/Actions.sysml")]
#[case("Systems Library/Allocations.sysml")]
#[case("Systems Library/AnalysisCases.sysml")]
#[case("Systems Library/Attributes.sysml")]
#[case("Systems Library/Calculations.sysml")]
#[case("Systems Library/Cases.sysml")]
#[case("Systems Library/Connections.sysml")]
#[case("Systems Library/Constraints.sysml")]
#[case("Systems Library/Flows.sysml")]
#[case("Systems Library/Interfaces.sysml")]
#[case("Systems Library/Items.sysml")]
#[case("Systems Library/Metadata.sysml")]
#[case("Systems Library/Parts.sysml")]
#[case("Systems Library/Ports.sysml")]
#[case("Systems Library/Requirements.sysml")]
#[case("Systems Library/StandardViewDefinitions.sysml")]
#[case("Systems Library/States.sysml")]
#[case("Systems Library/SysML.sysml")]
#[case("Systems Library/UseCases.sysml")]
#[case("Systems Library/VerificationCases.sysml")]
#[case("Systems Library/Views.sysml")]
fn test_parse_stdlib_sysml_file(#[case] relative_path: &str) {
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
