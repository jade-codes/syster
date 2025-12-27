//! Shared inlay hint types used by both KerML and SysML adapters

use crate::core::Position;

/// An inlay hint to display in the editor
#[derive(Debug, Clone, PartialEq)]
pub struct InlayHint {
    /// Position where the hint should be displayed
    pub position: Position,
    /// The hint text to display
    pub label: String,
    /// The kind of hint
    pub kind: InlayHintKind,
    /// Whether padding should be added before/after the hint
    pub padding_left: bool,
    pub padding_right: bool,
}

/// The kind of inlay hint
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlayHintKind {
    /// Type annotation (e.g., `: Real` or `: Vehicle`)
    Type,
    /// Parameter name (e.g., `value:` or `width:`)
    Parameter,
}
