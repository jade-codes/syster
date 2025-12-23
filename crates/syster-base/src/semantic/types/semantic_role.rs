//! Semantic roles represent the conceptual purpose of a symbol in the model,
//! independent of the source language (SysML, KerML, etc.).
//!
//! These roles are assigned by language adapters during AST-to-semantic translation.

use std::fmt;

/// Semantic role of a symbol in the model.
///
/// This enum captures the high-level purpose or category of a symbol,
/// abstracted from language-specific details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticRole {
    /// Specifies a capability or condition that must be satisfied
    Requirement,

    /// Represents an action or behavior that can be performed
    Action,

    /// Represents a state or mode of operation
    State,

    /// Describes a use case or scenario
    UseCase,

    /// Structural component or part
    Component,

    /// Interface or connection point
    Interface,

    /// Port for communication
    Port,

    /// Attribute or property
    Attribute,

    /// Connection between elements
    Connection,

    /// Constraint or rule
    Constraint,

    /// Analysis case or calculation
    AnalysisCase,

    /// Verification case
    VerificationCase,

    /// View or viewpoint
    View,

    /// Metadata annotation
    Metadata,

    /// Item or resource
    Item,

    /// Flow of data or material
    Flow,

    /// Allocation relationship
    Allocation,

    /// Generic classifier (when no specific role applies)
    Classifier,

    /// Generic feature (when no specific role applies)
    Feature,

    /// Package or namespace
    Package,

    /// Unknown or unmapped role (preserves original kind string)
    Unknown(String),
}

impl SemanticRole {
    /// Returns true if this role can be the target of a "satisfy" relationship
    pub fn is_requirement(&self) -> bool {
        matches!(self, SemanticRole::Requirement)
    }

    /// Returns true if this role can be the target of a "perform" relationship
    pub fn is_action(&self) -> bool {
        matches!(self, SemanticRole::Action)
    }

    /// Returns true if this role can be the target of an "exhibit" relationship
    pub fn is_state(&self) -> bool {
        matches!(self, SemanticRole::State)
    }

    /// Returns true if this role can be the target of an "include" relationship
    pub fn is_use_case(&self) -> bool {
        matches!(self, SemanticRole::UseCase)
    }
}

impl fmt::Display for SemanticRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticRole::Requirement => write!(f, "requirement"),
            SemanticRole::Action => write!(f, "action"),
            SemanticRole::State => write!(f, "state"),
            SemanticRole::UseCase => write!(f, "use case"),
            SemanticRole::Component => write!(f, "component"),
            SemanticRole::Interface => write!(f, "interface"),
            SemanticRole::Port => write!(f, "port"),
            SemanticRole::Attribute => write!(f, "attribute"),
            SemanticRole::Connection => write!(f, "connection"),
            SemanticRole::Constraint => write!(f, "constraint"),
            SemanticRole::AnalysisCase => write!(f, "analysis case"),
            SemanticRole::VerificationCase => write!(f, "verification case"),
            SemanticRole::View => write!(f, "view"),
            SemanticRole::Metadata => write!(f, "metadata"),
            SemanticRole::Item => write!(f, "item"),
            SemanticRole::Flow => write!(f, "flow"),
            SemanticRole::Allocation => write!(f, "allocation"),
            SemanticRole::Classifier => write!(f, "classifier"),
            SemanticRole::Feature => write!(f, "feature"),
            SemanticRole::Package => write!(f, "package"),
            SemanticRole::Unknown(s) => write!(f, "{s}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requirement_check() {
        assert!(SemanticRole::Requirement.is_requirement());
        assert!(!SemanticRole::Action.is_requirement());
    }

    #[test]
    fn test_display() {
        assert_eq!(SemanticRole::Requirement.to_string(), "requirement");
        assert_eq!(
            SemanticRole::Unknown("custom".to_string()).to_string(),
            "custom"
        );
    }
}
