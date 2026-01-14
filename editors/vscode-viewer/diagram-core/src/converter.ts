/**
 * SysML Workspace to Diagram Converter
 * 
 * Converts data from the Rust LSP (symbols and relationships from the semantic analyzer)
 * into our diagram-core types (SysMLNode and SysMLEdge).
 * 
 * This provides the bridge between the parsed SysML model and the visualization layer.
 */

import { NODE_TYPES, type SysMLNode } from "./sysml-nodes";
import { EDGE_TYPES, type SysMLEdge } from "./sysml-edges";
import type { Diagram } from "./index";

// ========== Input Data Types (from Rust LSP) ==========

/** Symbol data from Rust semantic analyzer */
export interface SymbolData {
  name: string;
  qualifiedName: string;
  kind: "Definition" | "Usage" | "Package" | "Classifier" | "Feature" | "Alias";
  
  // Definition-specific fields
  definitionKind?: "Part" | "Port" | "Action" | "State" | "Item" | "Attribute" | "Requirement" 
    | "Concern" | "Case" | "AnalysisCase" | "VerificationCase" | "UseCase" | "View" | "Viewpoint"
    | "Rendering" | "Allocation" | "Calculation" | "Connection" | "Constraint" | "Enumeration"
    | "Flow" | "Individual" | "Interface" | "Occurrence" | "Metadata";
  
  // Usage-specific fields
  usageKind?: "Part" | "Port" | "Action" | "Item" | "Attribute" | "Requirement" | "Concern" 
    | "Case" | "View" | "Enumeration" | "SatisfyRequirement" | "PerformAction" | "ExhibitState" 
    | "IncludeUseCase";
  
  // Common optional fields
  features?: string[];
  typedBy?: string;
  direction?: "in" | "out" | "inout";
  text?: string;
  dataType?: string;
  
  // Domain-specific usage fields
  satisfiedRequirement?: string;
  performedAction?: string;
  exhibitedState?: string;
  includedUseCase?: string;
}

/** Relationship data from Rust semantic analyzer */
export interface RelationshipData {
  type: "specialization" | "redefinition" | "subsetting" | "typing" | "reference_subsetting" 
    | "cross_subsetting" | "satisfy" | "perform" | "exhibit" | "include" | "assert" | "verify";
  source: string;  // Qualified name
  target: string;  // Qualified name
}

/** Complete workspace data from Rust LSP */
export interface WorkspaceData {
  symbols: SymbolData[];
  relationships: RelationshipData[];
}

// ========== Conversion Functions ==========

let edgeIdCounter = 0;

/** Generate unique edge ID */
function generateEdgeId(): string {
  return `edge_${edgeIdCounter++}`;
}

/** Convert a symbol from Rust LSP to a SysML node */
export function convertSymbolToNode(symbol: SymbolData): SysMLNode | null {
  const baseId = symbol.qualifiedName.replace(/::/g, "_");
  
  // Handle Definition symbols
  if (symbol.kind === "Definition" && symbol.definitionKind) {
    const base = {
      id: baseId,
      name: symbol.name,
      qualifiedName: symbol.qualifiedName,
    };

    switch (symbol.definitionKind) {
      case "Part":
        return { ...base, type: NODE_TYPES.PART_DEF, features: symbol.features || [] };
      case "Port":
        return { ...base, type: NODE_TYPES.PORT_DEF, direction: symbol.direction || "in" };
      case "Action":
        return { ...base, type: NODE_TYPES.ACTION_DEF };
      case "State":
        return { ...base, type: NODE_TYPES.STATE_DEF };
      case "Item":
        return { ...base, type: NODE_TYPES.ITEM_DEF };
      case "Attribute":
        return { ...base, type: NODE_TYPES.ATTRIBUTE_DEF, dataType: symbol.dataType };
      case "Requirement":
        return { ...base, type: NODE_TYPES.REQUIREMENT_DEF, text: symbol.text };
      case "Concern":
        return { ...base, type: NODE_TYPES.CONCERN_DEF };
      case "Case":
        return { ...base, type: NODE_TYPES.CASE_DEF };
      case "AnalysisCase":
        return { ...base, type: NODE_TYPES.ANALYSIS_CASE_DEF };
      case "VerificationCase":
        return { ...base, type: NODE_TYPES.VERIFICATION_CASE_DEF };
      case "UseCase":
        return { ...base, type: NODE_TYPES.USE_CASE_DEF };
      case "View":
        return { ...base, type: NODE_TYPES.VIEW_DEF };
      case "Viewpoint":
        return { ...base, type: NODE_TYPES.VIEWPOINT_DEF };
      case "Rendering":
        return { ...base, type: NODE_TYPES.RENDERING_DEF };
      case "Allocation":
        return { ...base, type: NODE_TYPES.ALLOCATION_DEF };
      case "Calculation":
        return { ...base, type: NODE_TYPES.CALCULATION_DEF };
      case "Connection":
        return { ...base, type: NODE_TYPES.CONNECTION_DEF };
      case "Constraint":
        return { ...base, type: NODE_TYPES.CONSTRAINT_DEF };
      case "Enumeration":
        return { ...base, type: NODE_TYPES.ENUMERATION_DEF };
      case "Flow":
        return { ...base, type: NODE_TYPES.FLOW_DEF };
      case "Individual":
        return { ...base, type: NODE_TYPES.INDIVIDUAL_DEF };
      case "Interface":
        return { ...base, type: NODE_TYPES.INTERFACE_DEF };
      case "Occurrence":
        return { ...base, type: NODE_TYPES.OCCURRENCE_DEF };
      case "Metadata":
        return { ...base, type: NODE_TYPES.METADATA_DEF };
      default:
        return null;  // Unknown definition kind
    }
  }

  // Handle Usage symbols
  if (symbol.kind === "Usage" && symbol.usageKind) {
    const base = {
      id: baseId,
      name: symbol.name,
      qualifiedName: symbol.qualifiedName,
    };

    switch (symbol.usageKind) {
      case "Part":
        return { ...base, type: NODE_TYPES.PART_USAGE, typedBy: symbol.typedBy };
      case "Port":
        return { ...base, type: NODE_TYPES.PORT_USAGE, direction: symbol.direction || "in", typedBy: symbol.typedBy };
      case "Action":
        return { ...base, type: NODE_TYPES.ACTION_USAGE, typedBy: symbol.typedBy };
      case "Item":
        return { ...base, type: NODE_TYPES.ITEM_USAGE, typedBy: symbol.typedBy };
      case "Attribute":
        return { ...base, type: NODE_TYPES.ATTRIBUTE_USAGE, dataType: symbol.dataType, typedBy: symbol.typedBy };
      case "Requirement":
        return { ...base, type: NODE_TYPES.REQUIREMENT_USAGE, text: symbol.text, typedBy: symbol.typedBy };
      case "Concern":
        return { ...base, type: NODE_TYPES.CONCERN_USAGE, typedBy: symbol.typedBy };
      case "Case":
        return { ...base, type: NODE_TYPES.CASE_USAGE, typedBy: symbol.typedBy };
      case "View":
        return { ...base, type: NODE_TYPES.VIEW_USAGE, typedBy: symbol.typedBy };
      case "Enumeration":
        return { ...base, type: NODE_TYPES.ENUMERATION_USAGE, typedBy: symbol.typedBy };
      case "SatisfyRequirement":
        return { ...base, type: NODE_TYPES.SATISFY_REQUIREMENT_USAGE, satisfiedRequirement: symbol.satisfiedRequirement };
      case "PerformAction":
        return { ...base, type: NODE_TYPES.PERFORM_ACTION_USAGE, performedAction: symbol.performedAction };
      case "ExhibitState":
        return { ...base, type: NODE_TYPES.EXHIBIT_STATE_USAGE, exhibitedState: symbol.exhibitedState };
      case "IncludeUseCase":
        return { ...base, type: NODE_TYPES.INCLUDE_USE_CASE_USAGE, includedUseCase: symbol.includedUseCase };
      default:
        return null;  // Unknown usage kind
    }
  }

  // Unsupported symbol kind (e.g., Package, Classifier, Feature, Alias)
  return null;
}

/** Convert a relationship from Rust LSP to a SysML edge */
export function convertRelationshipToEdge(relationship: RelationshipData): SysMLEdge {
  const base = {
    id: generateEdgeId(),
    source: relationship.source,
    target: relationship.target,
  };

  switch (relationship.type) {
    case "specialization":
      return { ...base, type: EDGE_TYPES.SPECIALIZATION };
    case "redefinition":
      return { ...base, type: EDGE_TYPES.REDEFINITION };
    case "subsetting":
      return { ...base, type: EDGE_TYPES.SUBSETTING };
    case "typing":
      return { ...base, type: EDGE_TYPES.TYPING };
    case "reference_subsetting":
      return { ...base, type: EDGE_TYPES.REFERENCE_SUBSETTING };
    case "cross_subsetting":
      return { ...base, type: EDGE_TYPES.CROSS_SUBSETTING };
    case "satisfy":
      return { ...base, type: EDGE_TYPES.SATISFY };
    case "perform":
      return { ...base, type: EDGE_TYPES.PERFORM };
    case "exhibit":
      return { ...base, type: EDGE_TYPES.EXHIBIT };
    case "include":
      return { ...base, type: EDGE_TYPES.INCLUDE };
    case "assert":
      return { ...base, type: EDGE_TYPES.ASSERT };
    case "verify":
      return { ...base, type: EDGE_TYPES.VERIFY };
    default:
      // This should never happen with proper Rust LSP data, but TypeScript requires exhaustiveness
      throw new Error(`Unknown relationship type: ${(relationship as any).type}`);
  }
}

/** Convert complete workspace data to a diagram */
export function createDiagramFromWorkspace(workspace: WorkspaceData): Diagram {
  const nodes: SysMLNode[] = [];
  const edges: SysMLEdge[] = [];

  // Convert symbols to nodes
  for (const symbol of workspace.symbols) {
    const node = convertSymbolToNode(symbol);
    if (node) {
      nodes.push(node);
    }
  }

  // Convert relationships to edges
  for (const relationship of workspace.relationships) {
    const edge = convertRelationshipToEdge(relationship);
    edges.push(edge);
  }

  // Transform SysMLNodes to DiagramNodes by adding position and storing node data
  const diagramNodes = nodes.map((node, index) => ({
    id: node.id,
    type: node.type,
    position: { x: index * 200, y: 0 }, // Default layout: horizontal spacing
    data: node as unknown as Record<string, unknown> // Store the full SysML node data
  }));

  return { nodes: diagramNodes, edges };
}
