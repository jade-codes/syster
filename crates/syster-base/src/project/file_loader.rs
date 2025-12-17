mod collection;
mod parsing;

// Re-export core file loading functions (generic, no language dependencies)
pub use collection::collect_file_paths;
pub use parsing::{get_extension, load_file, validate_extension};

// Re-export language-agnostic parsing that dispatches to correct language parser
pub use crate::syntax::parser::{load_and_parse, parse_content, parse_with_result};

#[cfg(test)]
mod tests;
