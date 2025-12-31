//! Shared folding range types
//!
//! Common types for code folding across KerML and SysML.

use crate::core::Span;

/// A foldable region in source code
#[derive(Debug, Clone)]
pub struct FoldingRangeInfo {
    /// The span of the foldable region
    pub span: Span,
    /// Whether this is a comment region
    pub is_comment: bool,
}

impl FoldingRangeInfo {
    /// Create a new folding range for code (not a comment)
    pub fn code(span: Span) -> Self {
        Self {
            span,
            is_comment: false,
        }
    }

    /// Create a new folding range for a comment
    pub fn comment(span: Span) -> Self {
        Self {
            span,
            is_comment: true,
        }
    }
}
