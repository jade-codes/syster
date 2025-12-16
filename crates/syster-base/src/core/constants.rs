/// Supported file extensions for SysML and KerML files
pub const SUPPORTED_EXTENSIONS: &[&str] = &["sysml", "kerml"];

/// SysML file extension
pub const SYSML_EXT: &str = "sysml";

/// KerML file extension
pub const KERML_EXT: &str = "kerml";

/// Checks if a file extension is supported
pub fn is_supported_extension(ext: &str) -> bool {
    SUPPORTED_EXTENSIONS.contains(&ext)
}

// SysML relationship type constants
pub const REL_SPECIALIZATION: &str = "specialization";
pub const REL_REDEFINITION: &str = "redefinition";
pub const REL_SUBSETTING: &str = "subsetting";
pub const REL_TYPING: &str = "typing";
pub const REL_REFERENCE_SUBSETTING: &str = "reference_subsetting";
pub const REL_CROSS_SUBSETTING: &str = "cross_subsetting";

// Domain-specific SysML relationships
pub const REL_SATISFY: &str = "satisfy";
pub const REL_PERFORM: &str = "perform";
pub const REL_EXHIBIT: &str = "exhibit";
pub const REL_INCLUDE: &str = "include";
pub const REL_ASSERT: &str = "assert";
pub const REL_VERIFY: &str = "verify";

/// Maps relationship type constants to human-readable labels
pub fn relationship_label(rel_type: &str) -> &str {
    match rel_type {
        REL_SPECIALIZATION => "Specializes",
        REL_REDEFINITION => "Redefines",
        REL_SUBSETTING => "Subsets",
        REL_TYPING => "Typed by",
        REL_REFERENCE_SUBSETTING => "Reference subsets",
        REL_CROSS_SUBSETTING => "Cross subsets",
        REL_SATISFY => "Satisfies",
        REL_PERFORM => "Performs",
        REL_EXHIBIT => "Exhibits",
        REL_INCLUDE => "Includes",
        REL_ASSERT => "Asserts",
        REL_VERIFY => "Verifies",
        _ => rel_type,
    }
}
