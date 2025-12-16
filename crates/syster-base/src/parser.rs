#[path = "parser/kerml.rs"]
pub mod kerml;
#[path = "parser/keywords.rs"]
pub mod keywords;
#[path = "parser/sysml.rs"]
pub mod sysml;

// Re-export for convenience
pub use kerml::KerMLParser;
pub use sysml::SysMLParser;
