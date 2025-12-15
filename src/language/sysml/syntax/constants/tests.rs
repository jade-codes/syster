use super::*;

#[test]
fn test_kind_constants_are_valid_strings() {
    assert_eq!(SYSML_KIND_PART, "Part");
    assert_eq!(SYSML_KIND_PORT, "Port");
    assert_eq!(SYSML_KIND_ITEM, "Item");
    assert_eq!(SYSML_KIND_ATTRIBUTE, "Attribute");
    assert_eq!(SYSML_KIND_ACTION, "Action");
    assert_eq!(SYSML_KIND_STATE, "State");
    assert_eq!(SYSML_KIND_REQUIREMENT, "Requirement");
    assert_eq!(SYSML_KIND_CONCERN, "UseCase");
    assert_eq!(SYSML_KIND_CASE, "UseCase");
    assert_eq!(SYSML_KIND_ANALYSIS_CASE, "UseCase");
    assert_eq!(SYSML_KIND_VERIFICATION_CASE, "UseCase");
    assert_eq!(SYSML_KIND_USE_CASE, "UseCase");
    assert_eq!(SYSML_KIND_VIEW, "View");
    assert_eq!(SYSML_KIND_VIEWPOINT, "Viewpoint");
    assert_eq!(SYSML_KIND_RENDERING, "Rendering");
}

#[test]
fn test_primary_kinds_unique() {
    let primary_kinds = vec![
        SYSML_KIND_PART,
        SYSML_KIND_PORT,
        SYSML_KIND_ITEM,
        SYSML_KIND_ATTRIBUTE,
        SYSML_KIND_ACTION,
        SYSML_KIND_STATE,
        SYSML_KIND_REQUIREMENT,
        SYSML_KIND_VIEW,
        SYSML_KIND_VIEWPOINT,
        SYSML_KIND_RENDERING,
        "UseCase",
    ];

    let mut unique = primary_kinds.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(
        unique.len(),
        primary_kinds.len(),
        "Primary kind constants should be unique"
    );
}

#[test]
fn test_case_kinds_all_map_to_use_case() {
    assert_eq!(SYSML_KIND_CONCERN, "UseCase");
    assert_eq!(SYSML_KIND_CASE, "UseCase");
    assert_eq!(SYSML_KIND_ANALYSIS_CASE, "UseCase");
    assert_eq!(SYSML_KIND_VERIFICATION_CASE, "UseCase");
    assert_eq!(SYSML_KIND_USE_CASE, "UseCase");
}
