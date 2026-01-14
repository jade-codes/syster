/**
 * SysML v2 Edge Types for Diagram Visualization
 * 
 * This module defines TypeScript types for SysML v2 relationships that can be
 * visualized as edges in a diagram. Each type corresponds to a relationship
 * from the language specification.
 * 
 * Edge Categories:
 * - Core Relationships: specialization, redefinition, subsetting, typing
 * - Domain-Specific: satisfy, perform, exhibit, include, assert, verify
 * - Structural: composition (part containment)
 */

// ========== Constants ==========

/** SysML edge type discriminators */
export const EDGE_TYPES = {
  // Core KerML/SysML relationships
  SPECIALIZATION: "specialization",
  REDEFINITION: "redefinition",
  SUBSETTING: "subsetting",
  TYPING: "typing",
  REFERENCE_SUBSETTING: "reference_subsetting",
  CROSS_SUBSETTING: "cross_subsetting",
  CONJUGATION: "conjugation",
  
  // Domain-specific SysML relationships
  SATISFY: "satisfy",
  PERFORM: "perform",
  EXHIBIT: "exhibit",
  INCLUDE: "include",
  ASSERT: "assert",
  VERIFY: "verify",
  DEPENDENCY: "dependency",
  
  // Structural relationships
  COMPOSITION: "composition",
  CONNECTION: "connection",
  BINDING: "binding",
  ALLOCATION: "allocation",
  FLOW: "flow",
  SUCCESSION: "succession",
  MEMBERSHIP: "membership",
} as const;

/** Base properties common to all SysML edges */
interface BaseSysMLEdge {
  /** Unique identifier for the edge */
  id: string;
  /** Source node qualified name */
  source: string;
  /** Target node qualified name */
  target: string;
  /** Optional label to display on edge */
  label?: string;
}

// ========== Core Relationship Edges ==========

/** Specialization edge - IS-A inheritance relationship */
export interface SpecializationEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.SPECIALIZATION;
}

/** Redefinition edge - OVERRIDES relationship for features */
export interface RedefinitionEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.REDEFINITION;
}

/** Subsetting edge - REFINES relationship for features */
export interface SubsettingEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.SUBSETTING;
}

/** Typing edge - INSTANCE-OF relationship */
export interface TypingEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.TYPING;
}

/** Reference subsetting edge - subsetting for reference features */
export interface ReferenceSubsettingEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.REFERENCE_SUBSETTING;
}

/** Cross subsetting edge - cross-namespace subsetting */
export interface CrossSubsettingEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.CROSS_SUBSETTING;
}

/** Conjugation edge - conjugated port relationship */
export interface ConjugationEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.CONJUGATION;
}

// ========== Domain-Specific Relationship Edges ==========

/** Satisfy edge - requirement satisfaction relationship */
export interface SatisfyEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.SATISFY;
}

/** Perform edge - action performance relationship */
export interface PerformEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.PERFORM;
}

/** Exhibit edge - state exhibition relationship */
export interface ExhibitEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.EXHIBIT;
}

/** Include edge - use case inclusion relationship */
export interface IncludeEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.INCLUDE;
}

/** Assert edge - constraint assertion relationship */
export interface AssertEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.ASSERT;
}

/** Verify edge - verification relationship */
export interface VerifyEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.VERIFY;
}

/** Dependency edge - dependency between elements */
export interface DependencyEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.DEPENDENCY;
}

// ========== Structural Edges ==========

/** Composition edge - part containment (HAS-A relationship) */
export interface CompositionEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.COMPOSITION;
  /** Multiplicity of the contained part (e.g., "4" for wheels, "1" for engine, "*" for any) */
  multiplicity?: string;
}

/** Connection edge - connection between ports or parts */
export interface ConnectionEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.CONNECTION;
  /** Qualified name of the connection definition (if any) */
  connectionDef?: string;
}

/** Binding edge - binding connector (equality constraint) */
export interface BindingEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.BINDING;
}

/** Allocation edge - allocation of one element to another */
export interface AllocationEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.ALLOCATION;
  /** Qualified name of the allocation definition (if any) */
  allocationDef?: string;
}

/** Flow edge - item flow between ports or parts */
export interface FlowEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.FLOW;
  /** Type of item flowing (if specified) */
  itemType?: string;
}

/** Succession edge - succession/transition between actions or states */
export interface SuccessionEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.SUCCESSION;
  /** Guard condition for the transition (if any) */
  guard?: string;
}

/** Membership edge - containment of feature in classifier (parent-child relationship) */
export interface MembershipEdge extends BaseSysMLEdge {
  type: typeof EDGE_TYPES.MEMBERSHIP;
}

// ========== Union Types ==========

/** All core and domain-specific relationship edges */
export type SysMLRelationshipEdge =
  | SpecializationEdge
  | RedefinitionEdge
  | SubsettingEdge
  | TypingEdge
  | ReferenceSubsettingEdge
  | CrossSubsettingEdge
  | ConjugationEdge
  | SatisfyEdge
  | PerformEdge
  | ExhibitEdge
  | IncludeEdge
  | AssertEdge
  | VerifyEdge
  | DependencyEdge;

/** All structural edges (composition, connections, flows, etc.) */
export type SysMLStructuralEdge =
  | CompositionEdge
  | ConnectionEdge
  | BindingEdge
  | AllocationEdge
  | FlowEdge
  | SuccessionEdge
  | MembershipEdge;

/** All SysML edge types */
export type SysMLEdge = SysMLRelationshipEdge | SysMLStructuralEdge;

// ========== Type Guards ==========

/** Structural edge types for type guard */
const STRUCTURAL_EDGE_TYPES = [
  EDGE_TYPES.COMPOSITION,
  EDGE_TYPES.CONNECTION,
  EDGE_TYPES.BINDING,
  EDGE_TYPES.ALLOCATION,
  EDGE_TYPES.FLOW,
  EDGE_TYPES.SUCCESSION,
  EDGE_TYPES.MEMBERSHIP,
] as const;

/** Check if an edge is a relationship edge (not structural) */
export function isRelationshipEdge(edge: SysMLEdge): edge is SysMLRelationshipEdge {
  return !STRUCTURAL_EDGE_TYPES.includes(edge.type as typeof STRUCTURAL_EDGE_TYPES[number]);
}

/** Check if an edge is a structural edge (composition, connection, etc.) */
export function isStructuralEdge(edge: SysMLEdge): edge is SysMLStructuralEdge {
  return STRUCTURAL_EDGE_TYPES.includes(edge.type as typeof STRUCTURAL_EDGE_TYPES[number]);
}
