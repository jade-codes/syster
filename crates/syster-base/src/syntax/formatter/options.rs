//! Formatting options

/// Formatting options for SysML/KerML code
#[derive(Debug, Clone)]
pub struct FormatOptions {
    /// Number of spaces per indentation level (or tab width if using tabs)
    pub tab_size: usize,
    /// Use spaces for indentation (false = use tabs)
    pub insert_spaces: bool,
    /// Maximum line width before breaking
    pub print_width: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            tab_size: 4,
            insert_spaces: true,
            print_width: 80,
        }
    }
}

impl FormatOptions {
    /// Generate indentation string for the given level
    pub fn indent(&self, level: usize) -> String {
        if self.insert_spaces {
            " ".repeat(self.tab_size * level)
        } else {
            "\t".repeat(level)
        }
    }
}
