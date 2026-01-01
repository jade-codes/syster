use super::enums::{
    ClassifierKind, ClassifierMember, Element, FeatureDirection, FeatureMember, ImportKind,
};
use crate::core::Span;
pub use crate::syntax::kerml::model::types::Comment;

#[derive(Debug, Clone, PartialEq)]
pub struct KerMLFile {
    pub namespace: Option<NamespaceDeclaration>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Classifier {
    pub kind: ClassifierKind,
    pub is_abstract: bool,
    pub name: Option<String>,
    pub body: Vec<ClassifierMember>,
    /// Span of the classifier name identifier
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    pub name: Option<String>,
    pub direction: Option<FeatureDirection>,
    pub is_readonly: bool,
    pub is_derived: bool,
    pub body: Vec<FeatureMember>,
    /// Span of the feature name identifier
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub path_span: Option<Span>,
    pub is_recursive: bool,
    pub kind: ImportKind,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub reference: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Specialization {
    pub general: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Redefinition {
    pub redefined: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subsetting {
    pub subset: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypingRelationship {
    pub typed: String,
    pub span: Option<Span>,
}
