#![allow(clippy::unwrap_used)]

use crate::core::constants::is_supported_extension;

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

// ============================================================================
// Additional comprehensive tests for is_supported_extension (#534)
// ============================================================================

#[test]
fn test_is_supported_extension_whitespace() {
    // Leading whitespace
    assert!(!is_supported_extension(" sysml"));
    assert!(!is_supported_extension("\tsysml"));
    assert!(!is_supported_extension("\nkerml"));

    // Trailing whitespace
    assert!(!is_supported_extension("sysml "));
    assert!(!is_supported_extension("kerml\t"));
    assert!(!is_supported_extension("kerml\n"));

    // Both leading and trailing
    assert!(!is_supported_extension(" sysml "));
    assert!(!is_supported_extension("\tkerml\t"));

    // Internal whitespace
    assert!(!is_supported_extension("sys ml"));
    assert!(!is_supported_extension("ker ml"));
}

#[test]
fn test_is_supported_extension_special_characters() {
    // Extensions with special characters should not match
    assert!(!is_supported_extension("sysml!"));
    assert!(!is_supported_extension("@kerml"));
    assert!(!is_supported_extension("sys#ml"));
    assert!(!is_supported_extension("kerml$"));
    assert!(!is_supported_extension("sys%ml"));
    assert!(!is_supported_extension("ker^ml"));
    assert!(!is_supported_extension("sys&ml"));
    assert!(!is_supported_extension("ker*ml"));
    assert!(!is_supported_extension("(sysml)"));
    assert!(!is_supported_extension("[kerml]"));
    assert!(!is_supported_extension("{sysml}"));
}

#[test]
fn test_is_supported_extension_numeric_strings() {
    // Pure numbers
    assert!(!is_supported_extension("123"));
    assert!(!is_supported_extension("0"));

    // Extensions with numbers
    assert!(!is_supported_extension("sysml2"));
    assert!(!is_supported_extension("sysml1"));
    assert!(!is_supported_extension("kerml3"));
    assert!(!is_supported_extension("1sysml"));
    assert!(!is_supported_extension("2kerml"));
}

#[test]
fn test_is_supported_extension_unicode() {
    // Unicode characters should not match
    assert!(!is_supported_extension("sysml™"));
    assert!(!is_supported_extension("sysml©"));
    assert!(!is_supported_extension("sysml®"));
    assert!(!is_supported_extension("sysml€"));
    assert!(!is_supported_extension("sys🔥ml"));
    assert!(!is_supported_extension("кerml")); // Cyrillic 'к'
    assert!(!is_supported_extension("sysмl")); // Cyrillic 'м'
}

#[test]
fn test_is_supported_extension_path_like_strings() {
    // Paths with forward slashes
    assert!(!is_supported_extension("path/to/sysml"));
    assert!(!is_supported_extension("sysml/file"));
    assert!(!is_supported_extension("/sysml"));
    assert!(!is_supported_extension("kerml/"));

    // Paths with backslashes
    assert!(!is_supported_extension("path\\to\\kerml"));
    assert!(!is_supported_extension("kerml\\file"));
    assert!(!is_supported_extension("\\kerml"));
    assert!(!is_supported_extension("sysml\\"));
}

#[test]
fn test_is_supported_extension_multiple_dots() {
    // Multiple dots should not match
    assert!(!is_supported_extension("..sysml"));
    assert!(!is_supported_extension("sysml.."));
    assert!(!is_supported_extension("sys.ml"));
    assert!(!is_supported_extension("ker.ml"));
    assert!(!is_supported_extension("file.sysml"));
    assert!(!is_supported_extension("sysml.backup"));
}

#[test]
fn test_is_supported_extension_mixed_case_variations() {
    // Various mixed case combinations
    assert!(!is_supported_extension("Sysml"));
    assert!(!is_supported_extension("sYsml"));
    assert!(!is_supported_extension("sysMl"));
    assert!(!is_supported_extension("sysML"));
    assert!(!is_supported_extension("Kerml"));
    assert!(!is_supported_extension("kErml"));
    assert!(!is_supported_extension("keRml"));
    assert!(!is_supported_extension("kerMl"));
    assert!(!is_supported_extension("kerML"));
}

#[test]
fn test_is_supported_extension_very_long_strings() {
    // Very long strings should not match
    let long_string = "a".repeat(1000);
    assert!(!is_supported_extension(&long_string));

    let long_with_sysml = format!("{}sysml", "x".repeat(500));
    assert!(!is_supported_extension(&long_with_sysml));

    let sysml_with_long = format!("sysml{}", "y".repeat(500));
    assert!(!is_supported_extension(&sysml_with_long));
}

#[test]
fn test_is_supported_extension_similar_strings() {
    // Strings similar to valid extensions but not exact matches
    assert!(!is_supported_extension("sysm")); // Missing 'l'
    assert!(!is_supported_extension("sysmll")); // Extra 'l'
    assert!(!is_supported_extension("ssysml")); // Extra 's' at start
    assert!(!is_supported_extension("sysmlx")); // Extra character at end
    assert!(!is_supported_extension("ysml")); // Missing first 's'
    assert!(!is_supported_extension("sysm1")); // '1' instead of 'l'

    assert!(!is_supported_extension("kerm")); // Missing 'l'
    assert!(!is_supported_extension("kermll")); // Extra 'l'
    assert!(!is_supported_extension("kkerml")); // Extra 'k' at start
    assert!(!is_supported_extension("kermlx")); // Extra character at end
    assert!(!is_supported_extension("erml")); // Missing first 'k'
    assert!(!is_supported_extension("kerm1")); // '1' instead of 'l'
}

#[test]
fn test_is_supported_extension_null_byte() {
    // Null byte should not match
    assert!(!is_supported_extension("sysml\0"));
    assert!(!is_supported_extension("\0sysml"));
    assert!(!is_supported_extension("sys\0ml"));
    assert!(!is_supported_extension("kerml\0"));
    assert!(!is_supported_extension("\0kerml"));
}

#[test]
fn test_is_supported_extension_only_supported() {
    // Verify only the two supported extensions return true
    let all_test_cases = vec![
        // Valid extensions
        ("sysml", true),
        ("kerml", true),
        // Everything else
        ("", false),
        ("txt", false),
        ("rs", false),
        ("md", false),
        ("json", false),
        ("SYSML", false),
        ("KERML", false),
        (".sysml", false),
        (".kerml", false),
        (" sysml", false),
        ("sysml ", false),
        ("sys ml", false),
        ("sysml2", false),
        ("xml", false),
        ("html", false),
        ("py", false),
    ];

    for (ext, expected) in all_test_cases {
        assert_eq!(
            is_supported_extension(ext),
            expected,
            "Expected is_supported_extension(\"{}\") to be {}",
            ext,
            expected
        );
    }
}
