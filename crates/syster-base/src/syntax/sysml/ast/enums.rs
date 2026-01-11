use super::types::{Alias, Comment, Definition, Import, Package, Usage};

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Package(Package),
    Definition(Definition),
    Usage(Usage),
    Comment(Comment),
    Import(Import),
    Alias(Alias),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DefinitionKind {
    #[default]
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
    Allocation,
    Calculation,
    Connection,
    Constraint,
    Enumeration,
    Flow,
    Individual,
    Interface,
    Occurrence,
    Metadata,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum UsageKind {
    #[default]
    Part,
    Port,
    Action,
    Item,
    Attribute,
    Requirement,
    Concern,
    Case,
    View,
    Enumeration,
    // Occurrence-based usage types
    Occurrence,
    Individual,
    Snapshot,
    Timeslice,
    // Domain-specific usage types
    SatisfyRequirement,
    PerformAction,
    ExhibitState,
    IncludeUseCase,
    // Reference usages (parameters with direction like `in`, `out`, `inout`)
    Reference,
    // Additional usage types
    Constraint,
    Calculation,
    State,
    Connection,
    Interface,
    Allocation,
    Flow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionMember {
    Comment(Box<Comment>),
    Usage(Box<Usage>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsageMember {
    Comment(Comment),
    Usage(Box<Usage>),
}
