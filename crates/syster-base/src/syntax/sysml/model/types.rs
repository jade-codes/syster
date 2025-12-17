use crate::syntax::kerml::model::types::{
    ClassifierReference, Conjugation, FeatureMembership, FeatureTyping, Import,
    InvocationExpression, Membership, MembershipImport, NamespaceImport, OwningMembership,
    ParameterMembership,
};
use crate::syntax::kerml::model::{
    AssociationStructure, Behavior, BindingConnector, BooleanExpression, Class, Classifier,
    Connector, DataType, Expression, Feature, Interaction, Invariant, ItemFlow, Metaclass,
    MetadataFeature, Predicate, Step, Structure, Succession, SuccessionItemFlow, SysMLFunction,
};
use crate::syntax::sysml::model::{
    PortionKind, RequirementConstraintKind, StateSubactionKind, TransitionFeatureKind, TriggerKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub classifier: Classifier,
    pub is_variation: bool,
    pub is_individual: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Usage {
    pub feature: Feature,
    pub is_variation: bool,
    pub is_reference: bool,
    pub portion_kind: Option<PortionKind>,
    pub is_individual: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccurrenceDefinition {
    pub definition: Definition,
    pub class: Class,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccurrenceUsage {
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeDefinition {
    pub definition: Definition,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeUsage {
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub structure: Structure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemUsage {
    pub occurrence_usage: OccurrenceUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartDefinition {
    pub item_definition: ItemDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartUsage {
    pub item_usage: ItemUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub structure: Structure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortUsage {
    pub occurrence_usage: OccurrenceUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub behavior: Behavior,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionUsage {
    pub occurrence_usage: OccurrenceUsage,
    pub step: Step,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceUsage {
    pub usage: Usage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataDefinition {
    pub metaclass: Metaclass,
    pub item_definition: ItemDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataUsage {
    pub metadata_feature: MetadataFeature,
    pub item_usage: ItemUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfActionUsage {
    pub action_usage: ActionUsage,
    pub condition: ParameterMembership,
    pub then: ParameterMembership,
    pub else_: Option<ParameterMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateDefinition {
    pub action_definition: ActionDefinition,
    pub is_parallel: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateUsage {
    pub action_usage: ActionUsage,
    pub is_parallel: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExhibitStateUsage {
    pub state_usage: StateUsage,
    pub perform_action_usage: PerformActionUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub predicate: Predicate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintUsage {
    pub occurrence_usage: OccurrenceUsage,
    pub boolean_expression: BooleanExpression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertConstraintUsage {
    pub constraint_usage: ConstraintUsage,
    pub invariant: Invariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionUsage {
    pub action_usage: ActionUsage,
    pub source: Option<Membership>,
    pub transition_link_source: Option<ParameterMembership>,
    pub payload: Option<ParameterMembership>,
    pub accepter: Option<TransitionFeatureMembership>,
    pub guard: Option<TransitionFeatureMembership>,
    pub effect: Option<TransitionFeatureMembership>,
    pub then: Option<OwningMembership>,
    pub else_: Option<OwningMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptActionUsage {
    pub action_usage: ActionUsage,
    pub payload: ParameterMembership,
    pub receiver: Option<ParameterMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementDefinition {
    pub constraint_definition: ConstraintDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementUsage {
    pub constraint_usage: ConstraintUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfyRequirementUsage {
    pub requirement_usage: RequirementUsage,
    pub assert_constraint_usage: AssertConstraintUsage,
    pub satisfaction_subject: Option<SubjectMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcernDefinition {
    pub requirement_definition: RequirementDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcernUsage {
    pub requirement_usage: RequirementUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationDefinition {
    pub action_definition: ActionDefinition,
    pub function: SysMLFunction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationUsage {
    pub action_usage: ActionUsage,
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseDefinition {
    pub calculation_definition: CalculationDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseUsage {
    pub calculation_usage: CalculationUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisCaseDefinition {
    pub case_definition: CaseDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisCaseUsage {
    pub case_usage: CaseUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorAsUsage {
    pub usage: Usage,
    pub connector: Connector,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConnectorAsUsage {
    pub connector_as_usage: ConnectorAsUsage,
    pub binding_connector: BindingConnector,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionDefinition {
    pub part_definition: PartDefinition,
    pub association_structure: AssociationStructure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionUsage {
    pub part_usage: PartUsage,
    pub connector_as_usage: ConnectorAsUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDefinition {
    pub connection_definition: ConnectionDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceUsage {
    pub connection_usage: ConnectionUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewDefinition {
    pub part_definition: PartDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewUsage {
    pub part_usage: PartUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewpointDefinition {
    pub requirement_definition: RequirementDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewpointUsage {
    pub requirement_usage: RequirementUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderingDefinition {
    pub part_definition: PartDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderingUsage {
    pub part_usage: PartUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCaseDefinition {
    pub case_definition: CaseDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCaseUsage {
    pub case_usage: CaseUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationDefinition {
    pub attribute_definition: AttributeDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationUsage {
    pub attribute_usage: AttributeUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationDefinition {
    pub connection_definition: ConnectionDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationUsage {
    pub connection_usage: ConnectionUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseDefinition {
    pub case_definition: CaseDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseUsage {
    pub case_usage: CaseUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeUseCaseUsage {
    pub use_case_usage: UseCaseUsage,
    pub perform_action_usage: PerformActionUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowConnectionDefinition {
    pub action_definition: ActionDefinition,
    pub interaction: Interaction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowConnectionUsage {
    pub connector_as_usage: ConnectorAsUsage,
    pub action_usage: ActionUsage,
    pub item_flow: ItemFlow,
    pub messages: Vec<ParameterMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionFlowConnectionUsage {
    pub flow_connection_usage: FlowConnectionUsage,
    pub succession_item_flow: SuccessionItemFlow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentActionUsage {
    pub action_usage: ActionUsage,
    pub target_member: Membership,
    pub assigned_value: ParameterMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerInvocationExpression {
    pub invocation_expression: InvocationExpression,
    pub kind: TriggerKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventOccurrenceUsage {
    pub occurrence_usage: OccurrenceUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerformActionUsage {
    pub action_usage: ActionUsage,
    pub event_occurrence_usage: EventOccurrenceUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopActionUsage {
    pub action_usage: ActionUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileLoopActionUsage {
    pub loop_action_usage: LoopActionUsage,
    pub condition: Option<ParameterMembership>,
    pub body: ParameterMembership,
    pub until: Option<ParameterMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForLoopActionUsage {
    pub loop_action_usage: LoopActionUsage,
    pub variable: FeatureMembership,
    pub sequence: ParameterMembership,
    pub body: ParameterMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendActionUsage {
    pub action_usage: ActionUsage,
    pub payload: ParameterMembership,
    pub sender: Option<ParameterMembership>,
    pub receiver: Option<ParameterMembership>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlNode {
    pub action_usage: ActionUsage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForkNode {
    pub control_node: ControlNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeNode {
    pub control_node: ControlNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinNode {
    pub control_node: ControlNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionNode {
    pub control_node: ControlNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionAsUsage {
    pub connector_as_usage: ConnectorAsUsage,
    pub succession: Succession,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expose {
    pub import: Import,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjugatedPortReference {
    pub classifier_reference: ClassifierReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantMembership {
    pub owning_membership: OwningMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifeClass {
    pub class: Class,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjugatedPortDefinition {
    pub port_definition: PortDefinition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjugatedPortTyping {
    pub feature_typing: FeatureTyping,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortConjugation {
    pub conjugation: Conjugation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateSubactionMembership {
    pub feature_membership: FeatureMembership,
    pub kind: StateSubactionKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionFeatureMembership {
    pub feature_membership: FeatureMembership,
    pub kind: TransitionFeatureKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectMembership {
    pub parameter_membership: ParameterMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorMembership {
    pub parameter_membership: ParameterMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StakeholderMembership {
    pub parameter_membership: ParameterMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementConstraintMembership {
    pub feature_membership: FeatureMembership,
    pub kind: Option<RequirementConstraintKind>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedConcernMembership {
    pub requirement_constraint_membership: RequirementConstraintMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementVerificationMembership {
    pub requirement_constraint_membership: RequirementConstraintMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveMembership {
    pub feature_membership: FeatureMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewRenderingMembership {
    pub feature_membership: FeatureMembership,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipExpose {
    pub expose: Expose,
    pub membership_import: MembershipImport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceExpose {
    pub expose: Expose,
    pub namespace_import: NamespaceImport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminateActionUsage {
    pub action_usage: ActionUsage,
    pub terminated_occurrence: Option<ParameterMembership>,
}
