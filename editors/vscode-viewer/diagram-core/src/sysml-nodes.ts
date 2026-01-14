/**
 * SysML v2 Node Types for Diagram Visualization
 * 
 * This module defines TypeScript types for SysML v2 elements that can be
 * visualized as nodes in a diagram. Each type corresponds to a SysML concept
 * from the language specification.
 * 
 * Type Hierarchy:
 * - Definitions (templates/types): PartDef, PortDef, ActionDef, etc.
 * - Usages (instances): PartUsage, PortUsage, ActionUsage, etc.
 * 
 * All node types include:
 * - id: Unique identifier for rendering
 * - type: Discriminator for type checking
 * - name: Simple name of the element
 * - qualifiedName: Fully qualified name (Package::Element)
 */

// ========== Constants ==========

/** SysML node type discriminators */
export const NODE_TYPES = {
  // Definitions
  PART_DEF: "PartDef",
  PORT_DEF: "PortDef",
  ACTION_DEF: "ActionDef",
  STATE_DEF: "StateDef",
  ITEM_DEF: "ItemDef",
  ATTRIBUTE_DEF: "AttributeDef",
  REQUIREMENT_DEF: "RequirementDef",
  CONCERN_DEF: "ConcernDef",
  CASE_DEF: "CaseDef",
  ANALYSIS_CASE_DEF: "AnalysisCaseDef",
  VERIFICATION_CASE_DEF: "VerificationCaseDef",
  USE_CASE_DEF: "UseCaseDef",
  VIEW_DEF: "ViewDef",
  VIEWPOINT_DEF: "ViewpointDef",
  RENDERING_DEF: "RenderingDef",
  ALLOCATION_DEF: "AllocationDef",
  CALCULATION_DEF: "CalculationDef",
  CONNECTION_DEF: "ConnectionDef",
  CONSTRAINT_DEF: "ConstraintDef",
  ENUMERATION_DEF: "EnumerationDef",
  FLOW_DEF: "FlowDef",
  INDIVIDUAL_DEF: "IndividualDef",
  INTERFACE_DEF: "InterfaceDef",
  OCCURRENCE_DEF: "OccurrenceDef",
  METADATA_DEF: "MetadataDef",
  
  // Usages
  PART_USAGE: "PartUsage",
  PORT_USAGE: "PortUsage",
  ACTION_USAGE: "ActionUsage",
  ITEM_USAGE: "ItemUsage",
  ATTRIBUTE_USAGE: "AttributeUsage",
  REQUIREMENT_USAGE: "RequirementUsage",
  CONCERN_USAGE: "ConcernUsage",
  CASE_USAGE: "CaseUsage",
  VIEW_USAGE: "ViewUsage",
  ENUMERATION_USAGE: "EnumerationUsage",
  SATISFY_REQUIREMENT_USAGE: "SatisfyRequirementUsage",
  PERFORM_ACTION_USAGE: "PerformActionUsage",
  EXHIBIT_STATE_USAGE: "ExhibitStateUsage",
  INCLUDE_USE_CASE_USAGE: "IncludeUseCaseUsage",
  STATE_USAGE: "StateUsage",
  OCCURRENCE_USAGE: "OccurrenceUsage",
  INDIVIDUAL_USAGE: "IndividualUsage",
  SNAPSHOT_USAGE: "SnapshotUsage",
  TIMESLICE_USAGE: "TimesliceUsage",
  REFERENCE_USAGE: "ReferenceUsage",
  CONSTRAINT_USAGE: "ConstraintUsage",
  CALCULATION_USAGE: "CalculationUsage",
  CONNECTION_USAGE: "ConnectionUsage",
  INTERFACE_USAGE: "InterfaceUsage",
  ALLOCATION_USAGE: "AllocationUsage",
  FLOW_USAGE: "FlowUsage",
} as const;

/** Feature directions */
export const FEATURE_DIRECTIONS = {
  IN: "in",
  OUT: "out",
  INOUT: "inout",
} as const;

/** Base properties common to all SysML nodes */
interface BaseSysMLNode {
  /** Unique identifier for the node in the diagram */
  id: string;
  /** Simple name of the element */
  name: string;
  /** Fully qualified name like "Package::Subpackage::Element" */
  qualifiedName: string;
}

/** Feature direction for ports and features */
export type FeatureDirection = typeof FEATURE_DIRECTIONS[keyof typeof FEATURE_DIRECTIONS];

// ========== Definition Nodes (Types) ==========

/** Part definition - physical or logical component type */
export interface PartDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.PART_DEF;
  /** Names of features owned by this part definition */
  features: string[];
}

/** Port definition - interface/connection point type */
export interface PortDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.PORT_DEF;
  /** Direction of data flow through the port */
  direction: FeatureDirection;
}

/** Action definition - behavior type */
export interface ActionDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ACTION_DEF;
}

/** State definition - state machine type */
export interface StateDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.STATE_DEF;
}

/** Requirement definition - requirement type */
export interface RequirementDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.REQUIREMENT_DEF;
  /** Requirement text/documentation */
  text?: string;
}

/** Concern definition - stakeholder concern type */
export interface ConcernDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CONCERN_DEF;
}

/** Case definition - analysis/test case type */
export interface CaseDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CASE_DEF;
}

/** Analysis case definition - computational analysis type */
export interface AnalysisCaseDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ANALYSIS_CASE_DEF;
}

/** Verification case definition - verification test type */
export interface VerificationCaseDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.VERIFICATION_CASE_DEF;
}

/** Use case definition - scenario type */
export interface UseCaseDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.USE_CASE_DEF;
}

/** View definition - viewpoint type */
export interface ViewDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.VIEW_DEF;
}

/** Viewpoint definition - perspective type */
export interface ViewpointDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.VIEWPOINT_DEF;
}

/** Rendering definition - visualization type */
export interface RenderingDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.RENDERING_DEF;
}

/** Allocation definition - allocation relationship type */
export interface AllocationDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ALLOCATION_DEF;
}

/** Calculation definition - calculation type */
export interface CalculationDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CALCULATION_DEF;
}

/** Connection definition - connection type */
export interface ConnectionDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CONNECTION_DEF;
}

/** Constraint definition - constraint type */
export interface ConstraintDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CONSTRAINT_DEF;
}

/** Enumeration definition - enumeration type */
export interface EnumerationDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ENUMERATION_DEF;
}

/** Flow definition - flow type */
export interface FlowDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.FLOW_DEF;
}

/** Individual definition - individual entity type */
export interface IndividualDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.INDIVIDUAL_DEF;
}

/** Interface definition - interface type */
export interface InterfaceDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.INTERFACE_DEF;
}

/** Occurrence definition - occurrence type */
export interface OccurrenceDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.OCCURRENCE_DEF;
}

/** Metadata definition - metadata type */
export interface MetadataDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.METADATA_DEF;
}

/** Item definition - resource/flow type */
export interface ItemDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ITEM_DEF;
}

/** Attribute definition - data attribute type */
export interface AttributeDefNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ATTRIBUTE_DEF;
  /** Data type of the attribute (e.g., "Real", "Integer", "String") */
  dataType?: string;
}

// ========== Usage Nodes (Instances) ==========

/** Part usage - physical or logical component instance */
export interface PartUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.PART_USAGE;
  /** Qualified name of the definition this part is typed by */
  typedBy?: string;
}

/** Port usage - interface/connection point instance */
export interface PortUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.PORT_USAGE;
  /** Direction of data flow through the port */
  direction: FeatureDirection;
  /** Qualified name of the port definition this is typed by */
  typedBy?: string;
}

/** Action usage - behavior instance */
export interface ActionUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ACTION_USAGE;
  /** Qualified name of the action definition this is typed by */
  typedBy?: string;
}

/** Item usage - resource instance */
export interface ItemUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ITEM_USAGE;
  /** Qualified name of the item definition this is typed by */
  typedBy?: string;
}

/** Attribute usage - data attribute instance */
export interface AttributeUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ATTRIBUTE_USAGE;
  /** Data type of the attribute */
  dataType?: string;
  /** Qualified name of the attribute definition this is typed by */
  typedBy?: string;
}

/** Requirement usage - requirement instance */
export interface RequirementUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.REQUIREMENT_USAGE;
  /** Requirement text */
  text?: string;
  /** Qualified name of the requirement definition this is typed by */
  typedBy?: string;
}

/** Concern usage - stakeholder concern instance */
export interface ConcernUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CONCERN_USAGE;
  /** Qualified name of the concern definition this is typed by */
  typedBy?: string;
}

/** Case usage - analysis/test case instance */
export interface CaseUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CASE_USAGE;
  /** Qualified name of the case definition this is typed by */
  typedBy?: string;
}

/** View usage - viewpoint instance */
export interface ViewUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.VIEW_USAGE;
  /** Qualified name of the view definition this is typed by */
  typedBy?: string;
}

/** Enumeration usage - enumeration instance */
export interface EnumerationUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ENUMERATION_USAGE;
  /** Qualified name of the enumeration definition this is typed by */
  typedBy?: string;
}

/** Satisfy requirement usage - satisfaction relationship instance */
export interface SatisfyRequirementUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.SATISFY_REQUIREMENT_USAGE;
  /** Requirement being satisfied */
  satisfiedRequirement?: string;
}

/** Perform action usage - action performance instance */
export interface PerformActionUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.PERFORM_ACTION_USAGE;
  /** Action being performed */
  performedAction?: string;
}

/** Exhibit state usage - state exhibition instance */
export interface ExhibitStateUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.EXHIBIT_STATE_USAGE;
  /** State being exhibited */
  exhibitedState?: string;
}

/** Include use case usage - use case inclusion instance */
export interface IncludeUseCaseUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.INCLUDE_USE_CASE_USAGE;
  /** Use case being included */
  includedUseCase?: string;
}

/** State usage - state/mode instance */
export interface StateUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.STATE_USAGE;
  /** Qualified name of the state definition this is typed by */
  typedBy?: string;
}

/** Occurrence usage - occurrence instance */
export interface OccurrenceUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.OCCURRENCE_USAGE;
  /** Qualified name of the occurrence definition this is typed by */
  typedBy?: string;
}

/** Individual usage - individual entity instance */
export interface IndividualUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.INDIVIDUAL_USAGE;
  /** Qualified name of the individual definition this is typed by */
  typedBy?: string;
}

/** Snapshot usage - snapshot of an individual at a point in time */
export interface SnapshotUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.SNAPSHOT_USAGE;
  /** Qualified name of the individual this is a snapshot of */
  snapshotOf?: string;
}

/** Timeslice usage - portion of an individual's lifetime */
export interface TimesliceUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.TIMESLICE_USAGE;
  /** Qualified name of the individual this is a timeslice of */
  timesliceOf?: string;
}

/** Reference usage - reference to another element (parameter with direction) */
export interface ReferenceUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.REFERENCE_USAGE;
  /** Direction of the reference */
  direction?: FeatureDirection;
  /** Qualified name of the definition this is typed by */
  typedBy?: string;
}

/** Constraint usage - constraint instance */
export interface ConstraintUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CONSTRAINT_USAGE;
  /** Qualified name of the constraint definition this is typed by */
  typedBy?: string;
}

/** Calculation usage - calculation instance */
export interface CalculationUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CALCULATION_USAGE;
  /** Qualified name of the calculation definition this is typed by */
  typedBy?: string;
}

/** Connection usage - connection instance */
export interface ConnectionUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.CONNECTION_USAGE;
  /** Qualified name of the connection definition this is typed by */
  typedBy?: string;
}

/** Interface usage - interface instance */
export interface InterfaceUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.INTERFACE_USAGE;
  /** Qualified name of the interface definition this is typed by */
  typedBy?: string;
}

/** Allocation usage - allocation instance */
export interface AllocationUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.ALLOCATION_USAGE;
  /** Qualified name of the allocation definition this is typed by */
  typedBy?: string;
}

/** Flow usage - flow connection instance */
export interface FlowUsageNode extends BaseSysMLNode {
  type: typeof NODE_TYPES.FLOW_USAGE;
  /** Item type flowing through this connection */
  itemType?: string;
  /** Qualified name of the flow definition this is typed by */
  typedBy?: string;
}

// ========== Union Types ==========

/** All SysML definition node types */
export type SysMLDefinitionNode =
  | PartDefNode
  | PortDefNode
  | ActionDefNode
  | StateDefNode
  | ItemDefNode
  | AttributeDefNode
  | RequirementDefNode
  | ConcernDefNode
  | CaseDefNode
  | AnalysisCaseDefNode
  | VerificationCaseDefNode
  | UseCaseDefNode
  | ViewDefNode
  | ViewpointDefNode
  | RenderingDefNode
  | AllocationDefNode
  | CalculationDefNode
  | ConnectionDefNode
  | ConstraintDefNode
  | EnumerationDefNode
  | FlowDefNode
  | IndividualDefNode
  | InterfaceDefNode
  | OccurrenceDefNode
  | MetadataDefNode;

/** All SysML usage node types */
export type SysMLUsageNode =
  | PartUsageNode
  | PortUsageNode
  | ActionUsageNode
  | ItemUsageNode
  | AttributeUsageNode
  | RequirementUsageNode
  | ConcernUsageNode
  | CaseUsageNode
  | ViewUsageNode
  | EnumerationUsageNode
  | SatisfyRequirementUsageNode
  | PerformActionUsageNode
  | ExhibitStateUsageNode
  | IncludeUseCaseUsageNode
  | StateUsageNode
  | OccurrenceUsageNode
  | IndividualUsageNode
  | SnapshotUsageNode
  | TimesliceUsageNode
  | ReferenceUsageNode
  | ConstraintUsageNode
  | CalculationUsageNode
  | ConnectionUsageNode
  | InterfaceUsageNode
  | AllocationUsageNode
  | FlowUsageNode;

/** All SysML node types */
export type SysMLNode = SysMLDefinitionNode | SysMLUsageNode;

// ========== Type Guards ==========

/** Check if a node is a definition node */
export function isDefinitionNode(node: SysMLNode): node is SysMLDefinitionNode {
  return node.type.endsWith("Def");
}

/** Check if a node is a usage node */
export function isUsageNode(node: SysMLNode): node is SysMLUsageNode {
  return node.type.endsWith("Usage");
}
