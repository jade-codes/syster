use super::enums::{FeatureDirectionKind, VisibilityKind};
use crate::core::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub declared_name: Option<String>,
    pub declared_short_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace {
    pub element: Element,
    pub prefixes: Vec<Box<OwningMembership>>,
    pub children: Vec<NamespaceChild>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceChild {
    Import(Box<Import>),
    Membership(Box<Membership>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub namespace: Namespace,
    pub is_sufficient: bool,
    pub is_abstract: Option<AbstractMarker>,
    pub heritage: Vec<Inheritance>,
    pub type_relationships: Vec<TypeOrFeatureRelationship>,
    pub multiplicity: Option<Box<OwningMembership>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractMarker {
    Abstract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeOrFeatureRelationship {
    Type(Box<TypeRelationship>),
    Feature(Box<FeatureRelationship>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeRelationship {
    Unioning(Box<Unioning>),
    Differencing(Box<Differencing>),
    Disjoining(Box<Disjoining>),
    Intersecting(Box<Intersecting>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureRelationship {
    TypeRelationship(Box<TypeRelationship>),
    FeatureChaining(Box<FeatureChaining>),
    FeatureInverting(Box<FeatureInverting>),
    TypeFeaturing(Box<TypeFeaturing>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classifier {
    pub type_: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataType {
    pub classifier: Classifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub classifier: Classifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Structure {
    pub class: Class,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Behavior {
    pub class: Class,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Association {
    pub classifier: Classifier,
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationStructure {
    pub association: Association,
    pub structure: Structure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metaclass {
    pub structure: Structure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysMLFunction {
    pub behavior: Behavior,
    pub result: Option<ResultExpressionMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub function: SysMLFunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interaction {
    pub association: Association,
    pub behavior: Behavior,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    pub type_: Type,
    pub is_nonunique: bool,
    pub is_ordered: bool,
    pub direction: Option<FeatureDirectionKind>,
    pub is_composite: Option<CompositeMarker>,
    pub is_derived: Option<DerivedMarker>,
    pub is_end: Option<EndMarker>,
    pub is_portion: Option<PortionMarker>,
    pub is_readonly: Option<ReadonlyMarker>,
    pub value: Option<Box<FeatureValue>>,
    pub write: Option<Box<Membership>>,
    pub crossing_feature: Option<Box<OwningMembership>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositeMarker {
    Composite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedMarker {
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndMarker {
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortionMarker {
    Portion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadonlyMarker {
    Readonly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataFeature {
    pub feature: Feature,
    pub annotating_element: AnnotatingElement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multiplicity {
    pub feature: Feature,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiplicityRange {
    pub multiplicity: Multiplicity,
    pub range: Option<Box<OwningMembership>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemFeature {
    pub feature: Feature,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub feature: Feature,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connector {
    pub feature: Feature,
    pub relationship: Relationship,
    pub ends: Vec<EndFeatureMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemFlowEnd {
    pub feature: Feature,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succession {
    pub connector: Connector,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConnector {
    pub connector: Connector,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub step: Step,
    pub result: Option<ResultExpressionMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemFlow {
    pub connector: Connector,
    pub step: Step,
    pub item: Option<Box<FeatureMembership>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionItemFlow {
    pub item_flow: ItemFlow,
    pub succession: Succession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanExpression {
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invariant {
    pub boolean_expression: BooleanExpression,
    pub is_negated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub namespace: Namespace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryPackage {
    pub package: Package,
    pub is_standard: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    pub reference: String,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotatingElement {
    pub about: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextualAnnotatingElement {
    pub annotating_element: AnnotatingElement,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub content: String,
    pub about: Vec<Annotation>,
    pub locale: Option<String>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Documentation {
    pub comment: Comment,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextualRepresentation {
    pub textual_annotating_element: TextualAnnotatingElement,
    pub language: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralExpression {
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralBoolean {
    pub literal_expression: LiteralExpression,
    pub literal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralString {
    pub literal_expression: LiteralExpression,
    pub literal: String,
}

#[derive(Debug, Clone)]
pub struct LiteralNumber {
    pub literal_expression: LiteralExpression,
    pub literal: f64,
}

impl PartialEq for LiteralNumber {
    fn eq(&self, other: &Self) -> bool {
        self.literal_expression == other.literal_expression && self.literal == other.literal
    }
}

impl Eq for LiteralNumber {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralInfinity {
    pub literal_expression: LiteralExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullExpression {
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationExpression {
    pub expression: Expression,
    pub operands: Option<Vec<Box<Expression>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorExpression {
    pub invocation_expression: InvocationExpression,
    pub operator: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexExpression {
    pub operator_expression: OperatorExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureChainExpression {
    pub operator_expression: OperatorExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectExpression {
    pub operator_expression: OperatorExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectExpression {
    pub operator_expression: OperatorExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureReferenceExpression {
    pub expression: Expression,
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataAccessExpression {
    pub expression: Expression,
    pub reference: ElementReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementReference {
    pub parts: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceReference {
    pub element_reference: ElementReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReference {
    pub namespace_reference: NamespaceReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifierReference {
    pub type_reference: TypeReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureReference {
    pub type_reference: TypeReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaclassReference {
    pub classifier_reference: ClassifierReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipReference {
    pub element_reference: ElementReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    pub element: Element,
    pub visibility: Option<VisibilityKind>,
    pub elements: Vec<RelationshipElement>,
    // Edge source
    pub source: Option<Element>,
    pub source_ref: Option<ElementReference>,
    pub source_chain: Option<Feature>,
    // Edge targets
    pub target: Option<Element>,
    pub target_ref: Option<ElementReference>,
    pub target_chain: Option<Feature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipElement {
    Namespace(Box<Namespace>),
    Relationship(Box<Relationship>),
    Feature(Box<Feature>),
    Annotation(Box<Annotation>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inheritance {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unioning {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Differencing {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intersecting {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureChaining {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specialization {
    pub inheritance: Inheritance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disjoining {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureInverting {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFeaturing {
    pub featuring: Featuring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureTyping {
    pub specialization: Specialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subclassification {
    pub specialization: Specialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subsetting {
    pub specialization: Specialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjugation {
    pub inheritance: Inheritance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redefinition {
    pub subsetting: Subsetting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSubsetting {
    pub subsetting: Subsetting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossSubsetting {
    pub subsetting: Subsetting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub relationship: Relationship,
    pub prefixes: Vec<Annotation>,
    pub client: Vec<ElementReference>,
    pub supplier: Vec<ElementReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub relationship: Relationship,
    pub imports_all: bool,
    pub is_recursive: bool,
    pub is_namespace: Option<NamespaceMarker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceMarker {
    Namespace, // Represents "::*"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipImport {
    pub import: Import,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceImport {
    pub import: Import,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Membership {
    pub relationship: Relationship,
    pub is_alias: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwningMembership {
    pub membership: Membership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureValue {
    pub owning_membership: OwningMembership,
    pub is_default: bool,
    pub is_initial: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementFilterMembership {
    pub owning_membership: OwningMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Featuring {
    pub relationship: Relationship,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureMembership {
    pub featuring: Featuring,
    pub owning_membership: OwningMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndFeatureMembership {
    pub feature_membership: FeatureMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterMembership {
    pub feature_membership: FeatureMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultExpressionMembership {
    pub feature_membership: FeatureMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnParameterMembership {
    pub parameter_membership: ParameterMembership,
}
