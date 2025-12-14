use super::types::{Comment, Definition, Import, Package, Usage};

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Package(Package),
    Definition(Definition),
    Usage(Usage),
    Comment(Comment),
    Import(Import),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionKind {
    Part,
    Port,
    Action,
    State,
    Item,
    Attribute,
    Requirement,
    Concern,
    Case,
    AnalysisCase,
    VerificationCase,
    UseCase,
    View,
    Viewpoint,
    Rendering,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsageKind {
    Part,
    Port,
    Action,
    Item,
    Attribute,
    Requirement,
    Concern,
    Case,
    View,
    // Domain-specific usage types
    SatisfyRequirement,
    PerformAction,
    ExhibitState,
    IncludeUseCase,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionMember {
    Comment(Box<Comment>),
    Usage(Box<Usage>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsageMember {
    Comment(Comment),
}
