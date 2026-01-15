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

#[derive(Debug, Clone, PartialEq)]
pub struct MetaRel {
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
    /// Meta type references (meta Qualified::Name)
    pub meta: Vec<MetaRel>,
}

impl Relationships {
    /// Create an empty relationships struct (for tests)
    pub fn none() -> Self {
        Self::default()
    }

    /// Get the span for a relationship to a specific target.
    /// Searches all relationship types for a matching target name.
    pub fn get_span_for_target(&self, target: &str) -> Option<Span> {
        // Check typing first (most common for usages)
        if self.typed_by.as_deref() == Some(target) {
            return self.typed_by_span;
        }

        // Check all one-to-many relationship vectors
        for rel in &self.specializes {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.redefines {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.subsets {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.references {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.crosses {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.satisfies {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.performs {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.exhibits {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.includes {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.asserts {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.verifies {
            if rel.target == target {
                return rel.span;
            }
        }
        for rel in &self.meta {
            if rel.target == target {
                return rel.span;
            }
        }

        None
    }

    /// Get all relationship targets with their spans.
    /// Returns tuples of (relationship_kind, target_name, span).
    pub fn all_targets_with_spans(&self) -> Vec<(&'static str, &str, Option<Span>)> {
        let mut result = Vec::new();

        if let Some(ref target) = self.typed_by {
            result.push(("typing", target.as_str(), self.typed_by_span));
        }

        for rel in &self.specializes {
            result.push(("specialization", rel.target.as_str(), rel.span));
        }
        for rel in &self.redefines {
            result.push(("redefinition", rel.target.as_str(), rel.span));
        }
        for rel in &self.subsets {
            result.push(("subsetting", rel.target.as_str(), rel.span));
        }
        for rel in &self.references {
            result.push(("reference", rel.target.as_str(), rel.span));
        }
        for rel in &self.crosses {
            result.push(("cross", rel.target.as_str(), rel.span));
        }
        for rel in &self.satisfies {
            result.push(("satisfy", rel.target.as_str(), rel.span));
        }
        for rel in &self.performs {
            result.push(("perform", rel.target.as_str(), rel.span));
        }
        for rel in &self.exhibits {
            result.push(("exhibit", rel.target.as_str(), rel.span));
        }
        for rel in &self.includes {
            result.push(("include", rel.target.as_str(), rel.span));
        }
        for rel in &self.asserts {
            result.push(("assert", rel.target.as_str(), rel.span));
        }
        for rel in &self.verifies {
            result.push(("verify", rel.target.as_str(), rel.span));
        }
        for rel in &self.meta {
            result.push(("meta", rel.target.as_str(), rel.span));
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Definition {
    pub kind: DefinitionKind,
    pub name: Option<String>,
    /// Short name (e.g., "kg" from `<kg> kilogram`)
    pub short_name: Option<String>,
    /// Span of the short name identifier (inside the `<` and `>`)
    pub short_name_span: Option<Span>,
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
            short_name: None,
            short_name_span: None,
            relationships,
            body,
            span: None,
            is_abstract: false,
            is_variation: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Usage {
    pub kind: UsageKind,
    pub name: Option<String>,
    /// Short name (e.g., "kg" from `<kg> kilogram`)
    pub short_name: Option<String>,
    /// Span of the short name identifier (inside the `<` and `>`)
    pub short_name_span: Option<Span>,
    pub relationships: Relationships,
    pub body: Vec<UsageMember>,
    /// Span of the usage name identifier
    pub span: Option<Span>,
    // Property modifiers
    #[doc(hidden)]
    pub is_derived: bool,
    #[doc(hidden)]
    pub is_const: bool,
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
            short_name: None,
            short_name_span: None,
            relationships,
            body,
            span: None,
            is_derived: false,
            is_const: false,
        }
    }

    /// For domain-specific usages (satisfy, perform, exhibit, include), get the primary target.
    /// Returns the target and its span from whichever syntactic position it appears in.
    /// Priority: typed_by > first subset > name
    pub fn domain_target(&self) -> Option<(&str, Option<Span>)> {
        match self.kind {
            UsageKind::SatisfyRequirement
            | UsageKind::PerformAction
            | UsageKind::ExhibitState
            | UsageKind::IncludeUseCase => {
                if let Some(ref typed_by) = self.relationships.typed_by {
                    Some((typed_by.as_str(), self.relationships.typed_by_span))
                } else if let Some(first_subset) = self.relationships.subsets.first() {
                    Some((first_subset.target.as_str(), first_subset.span))
                } else if let Some(ref name) = self.name {
                    Some((name.as_str(), self.span))
                } else {
                    None
                }
            }
            _ => None,
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
    pub is_public: bool,
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
