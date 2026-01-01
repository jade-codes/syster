use super::enums::{DefinitionKind, DefinitionMember, Element, UsageKind, UsageMember};
use crate::core::Span;

// Relationship types with span information
#[derive(Debug, Clone, PartialEq)]
pub struct SpecializationRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RedefinitionRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubsettingRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrossRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SatisfyRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PerformRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExhibitRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IncludeRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssertRel {
    pub target: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerifyRel {
    pub target: String,
    pub span: Option<Span>,
}

/// Represents a parsed SysML file with support for multiple package declarations
#[derive(Debug, Clone, PartialEq)]
pub struct SysMLFile {
    /// Primary namespace (first package) - maintained for backward compatibility
    pub namespace: Option<NamespaceDeclaration>,
    /// All namespace declarations in the file (Issue #10)
    pub namespaces: Vec<NamespaceDeclaration>,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDeclaration {
    pub name: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: Option<String>,
    pub elements: Vec<Element>,
    /// Span of the package name identifier
    pub span: Option<Span>,
}

/// Represents relationship information that can be attached to definitions and usages
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Relationships {
    /// Specializations (:> or "specializes")
    pub specializes: Vec<SpecializationRel>,
    /// Redefinitions (:>> or "redefines")
    pub redefines: Vec<RedefinitionRel>,
    /// Subsetting (:> or "subsets")
    pub subsets: Vec<SubsettingRel>,
    /// Feature typing (: or "typed by")
    pub typed_by: Option<String>,
    /// Span of the type reference (if typed_by is set)
    pub typed_by_span: Option<crate::core::Span>,
    /// References (::> or "references")
    pub references: Vec<ReferenceRel>,
    /// Crosses (=> or "crosses")  
    pub crosses: Vec<CrossRel>,

    // Domain-specific SysML relationships
    /// Satisfy (satisfy) - satisfaction of requirements
    pub satisfies: Vec<SatisfyRel>,
    /// Perform (perform) - performance relationships
    pub performs: Vec<PerformRel>,
    /// Exhibit (exhibit) - exhibition of states
    pub exhibits: Vec<ExhibitRel>,
    /// Include (include) - use case inclusion
    pub includes: Vec<IncludeRel>,
    /// Assert (assert) - constraint assertion
    pub asserts: Vec<AssertRel>,
    /// Verify (verify) - requirement verification
    pub verifies: Vec<VerifyRel>,
}

impl Relationships {
    /// Create an empty relationships struct (for tests)
    pub fn none() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub kind: DefinitionKind,
    pub name: Option<String>,
    pub relationships: Relationships,
    pub body: Vec<DefinitionMember>,
    /// Span of the definition name identifier
    pub span: Option<Span>,
    // Property modifiers
    #[doc(hidden)]
    pub is_abstract: bool,
    #[doc(hidden)]
    pub is_variation: bool,
}

impl Definition {
    pub fn new(
        kind: DefinitionKind,
        name: Option<String>,
        relationships: Relationships,
        body: Vec<DefinitionMember>,
    ) -> Self {
        Self {
            kind,
            name,
            relationships,
            body,
            span: None,
            is_abstract: false,
            is_variation: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Usage {
    pub kind: UsageKind,
    pub name: Option<String>,
    pub relationships: Relationships,
    pub body: Vec<UsageMember>,
    /// Span of the usage name identifier
    pub span: Option<Span>,
    // Property modifiers
    #[doc(hidden)]
    pub is_derived: bool,
    #[doc(hidden)]
    pub is_readonly: bool,
}

impl Usage {
    /// Create a new Usage with default property flags
    pub fn new(
        kind: UsageKind,
        name: Option<String>,
        relationships: Relationships,
        body: Vec<UsageMember>,
    ) -> Self {
        Self {
            kind,
            name,
            relationships,
            body,
            span: None,
            is_derived: false,
            is_readonly: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub path_span: Option<Span>,
    pub is_recursive: bool,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Documentation {
    pub comment: Comment,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    pub name: Option<String>,
    pub target: String,
    pub target_span: Option<Span>,
    pub span: Option<Span>,
}
