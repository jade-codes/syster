//! Foldable range types for code folding support

use crate::core::Span;

/// Kind of foldable region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldingKind {
    /// A region of code (package, definition, usage body)
    Region,
    /// A comment block
    Comment,
}

/// A range in a document that can be folded
#[derive(Debug, Clone, PartialEq)]
pub struct FoldableRange {
    /// The span of the foldable region
    pub span: Span,
    /// The kind of folding
    pub kind: FoldingKind,
    /// Optional text to show when collapsed
    pub collapsed_text: Option<String>,
}

impl FoldableRange {
    pub fn new(span: Span, kind: FoldingKind) -> Self {
        Self {
            span,
            kind,
            collapsed_text: None,
        }
    }

    pub fn with_collapsed_text(mut self, text: impl Into<String>) -> Self {
        self.collapsed_text = Some(text.into());
        self
    }

    /// Returns true if this range spans multiple lines (and thus is foldable)
    pub fn is_multiline(&self) -> bool {
        self.span.end.line > self.span.start.line
    }
}
