use super::types::{
    Annotation, Classifier, Comment, Feature, Import, Package, Redefinition, Specialization,
    Subsetting, TypingRelationship,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Package(Package),
    Classifier(Classifier),
    Feature(Feature),
    Comment(Comment),
    Import(Import),
    Annotation(Annotation),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassifierKind {
    Type,
    Classifier,
    DataType,
    Class,
    Structure,
    Behavior,
    Function,
    Association,
    AssociationStructure,
    Metaclass,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassifierMember {
    Feature(Feature),
    Comment(Comment),
    Specialization(Specialization),
    Import(Import),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureMember {
    Comment(Comment),
    Subsetting(Subsetting),
    Redefinition(Redefinition),
    Typing(TypingRelationship),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureDirection {
    In,
    Out,
    InOut,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ImportKind {
    Normal,
    Recursive,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}
