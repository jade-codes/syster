use super::super::types::Annotation;
use super::super::types::Membership;
use super::super::types::{
    CollectExpression, FeatureChainExpression, FeatureReferenceExpression, IndexExpression,
    InvocationExpression, LiteralExpression, MetadataAccessExpression, NullExpression,
    OperatorExpression, SelectExpression,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisibilityKind {
    Private,
    Protected,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureDirectionKind {
    In,
    InOut,
    Out,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitwiseNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassificationTestOperator {
    At,
    HasType,
    IsType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EqualityOperator {
    NotEqual,
    NotIdentical,
    Equal,
    Identical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportKind {
    /// ::*
    Members,
    /// ::**
    MembersRecursive,
    /// ::*::**
    AllRecursive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationalOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineExpression {
    FeatureChain(Box<FeatureChainExpression>),
    Index(Box<IndexExpression>),
    Invocation(Box<InvocationExpression>),
    Literal(Box<LiteralExpression>),
    MetadataAccess(Box<MetadataAccessExpression>),
    Null(Box<NullExpression>),
    Operator(Box<OperatorExpression>),
    Collect(Box<CollectExpression>),
    Select(Box<SelectExpression>),
    FeatureReference(Box<FeatureReferenceExpression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonOwnerType {
    Membership(Box<Membership>),
    Annotation(Box<Annotation>),
}
