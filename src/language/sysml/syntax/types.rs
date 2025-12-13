use super::enums::{DefinitionKind, DefinitionMember, Element, UsageKind, UsageMember};

#[derive(Debug, Clone, PartialEq)]
pub struct SysMLFile {
    pub namespace: Option<NamespaceDeclaration>,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDeclaration {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: Option<String>,
    pub elements: Vec<Element>,
}

/// Represents relationship information that can be attached to definitions and usages
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Relationships {
    /// Specializations (:> or "specializes")
    pub specializes: Vec<String>,
    /// Redefinitions (:>> or "redefines")
    pub redefines: Vec<String>,
    /// Subsetting (:> or "subsets")
    pub subsets: Vec<String>,
    /// Feature typing (: or "typed by")
    pub typed_by: Option<String>,
    /// References (::> or "references")
    pub references: Vec<String>,
    /// Crosses (=> or "crosses")  
    pub crosses: Vec<String>,
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Usage {
    pub kind: UsageKind,
    pub name: Option<String>,
    pub relationships: Relationships,
    pub body: Vec<UsageMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub is_recursive: bool,
}
