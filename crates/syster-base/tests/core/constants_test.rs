#![allow(clippy::unwrap_used)]

use syster::core::constants::is_supported_extension;

// ============================================================================
// Tests for is_supported_extension (#362)
// ============================================================================

#[test]
fn test_is_supported_extension_sysml() {
    assert!(is_supported_extension("sysml"));
}

#[test]
fn test_is_supported_extension_kerml() {
    assert!(is_supported_extension("kerml"));
}

#[test]
fn test_is_supported_extension_unsupported() {
    assert!(!is_supported_extension("txt"));
    assert!(!is_supported_extension("rs"));
    assert!(!is_supported_extension("md"));
    assert!(!is_supported_extension("json"));
}

#[test]
fn test_is_supported_extension_empty() {
    assert!(!is_supported_extension(""));
}

#[test]
fn test_is_supported_extension_case_sensitive() {
    // The function is case-sensitive
    assert!(!is_supported_extension("SYSML"));
    assert!(!is_supported_extension("SysML"));
    assert!(!is_supported_extension("KERML"));
    assert!(!is_supported_extension("KerML"));
}

#[test]
fn test_is_supported_extension_with_dot() {
    // Extensions should be provided without the dot
    assert!(!is_supported_extension(".sysml"));
    assert!(!is_supported_extension(".kerml"));
}
